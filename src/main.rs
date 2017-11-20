extern crate ioctl_rs as ioctl;
extern crate byteorder;

use byteorder::{BigEndian, ByteOrder, ReadBytesExt};
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;
use std::path::Path;
use std::str;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;


fn setup_serial_port(fd: RawFd) -> bool {
    let r = ioctl::ap_set_preselect_mode(fd);
    println!("preselect result={:?}", r);

    let current_limit: u32 = ioctl::ap_get_qlimit(fd).unwrap();
    println!("done, current_limit={}", current_limit);

    let max_limit: u32 = ioctl::ap_get_qlimit_max(fd).unwrap();
    println!("done, max_limit={}", max_limit);

    let r = ioctl::ap_set_qlimit(fd, max_limit).unwrap();
    println!("set qlimit result={:?}", r);

    if current_limit < max_limit {
        let updated_limit: u32 = ioctl::ap_get_qlimit(fd).unwrap();
        println!("done, updated_limit={}", updated_limit);
    }

    let cur_size: u32 = ioctl::ap_get_len(fd).unwrap();
    println!("done, cur_size={}", cur_size);

    // consider returning a wrapped result of the max buffer size...
    let max_buflen: u32 = ioctl::ap_get_maxauditdata_size(fd).unwrap();
    println!("done, max_buflen={}", max_buflen);

    let writes_only = 0x00000002;
    //let writes_only = ioctl::os::freebsd::ACLASS_FILE_WRITE;
    let r = ioctl::ap_set_preselect_flags(fd, writes_only);
    println!("set preselect result={:?}", r);
    let r = ioctl::ap_set_preselect_flags_na(fd, writes_only);
    println!("set preselect_na result={:?}", r);

    let r = ioctl::ap_flush(fd).unwrap();
    println!("flush result={:?}", r);

    let stat: u64 = ioctl::ap_get_drops(fd).unwrap();
    println!("done, drops={}", stat);

    let stat2: u64 = ioctl::ap_get_truncates(fd).unwrap();
    println!("done, truncates={}", stat2);


    true
}

const AUT_HEADER32: u8 = 0x14;
const AUT_HEADER32_EX: u8 = 0x15;
const AUT_HEADER64: u8 = 0x74;
const AUT_HEADER64_EX: u8 = 0x79;
const AUT_OTHER_FILE32: u8 = 0x11;


fn main() {
    println!("Hello, world!");
    let path = Path::new("/dev/auditpipe");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    setup_serial_port(file.as_raw_fd());

    //JRH: TODO - need to convert this
    let mut record_buffer: [u8; 32767] = [0; 32767];
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let &(ref lock, ref cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        // We notify the condvar that the value has changed.
        cvar.notify_one();
    });

    // Wait for the thread to start up.
    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    //
    // 1.) each thread holds its own record_buffer
    //    ** 4 threads / 4 buffers
    //    ** worker thread responsible for zeroing it out
    //    ** use a condition variable to indicate it's populated
    //
    //    ** each worker gets a cloned channel to produce..
    //

    // Make a vector to hold the workers which are spawned.
    let mut workers = vec![];

    for i in 0..1 {
        // Spin up another thread
        workers.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    }



    for worker in workers {
        // Wait for the thread to finish. Returns a result.
        let _ = worker.join();
    }


    for _ in 0..20000 {
        let mut record_type: [u8; 1] = [0; 1];
        let result = file.read(&mut record_type);
        //println!("Record Received type={}", record_type[0]);

        if record_type[0] == AUT_HEADER32 {
            // ??
            let record_size = file.read_u32::<BigEndian>().unwrap();
            //println!("Record Received size={}", record_size);
            let record_remaining = (record_size - 5) as usize;

            let result = file.read(&mut record_buffer[0..record_remaining]);
            //println!("Read record numBytes={:?}", result);

            if record_buffer[0] == 11 {
                // ??
                let token_sec = BigEndian::read_u64(&record_buffer[1..9]);
                //println!("Read record sec={:?}", token_sec);

                let token_ms = BigEndian::read_u64(&record_buffer[9..17]);
                //println!("Read record ms={:?}", token_ms);

                let token_flen = BigEndian::read_u16(&record_buffer[41..43]);
                //println!("Read record flen={:?}", token_flen);


                let pathend = (43 + token_flen) as usize;

                //for i in 43..pathend {
                //println!("  x[{}] = {}", i, record_buffer[i]);
                //}

                let p = match str::from_utf8(&record_buffer[43..pathend]) {
                    Ok(x) => x,
                    Err(e) => {
                        println!("  err={}", e);
                        //unsafe {
                        //str::from_utf8_unchecked(&record_buffer[43..pathend])
                        //}
                        ""
                    }
                };
                println!("Read path={:?}", p);
            } else {
                println!("Unknown record type");
            }
        } else {
            println!("Unknown header={}", record_type[0]);
        }

    }


    setup_serial_port(file.as_raw_fd());

}
