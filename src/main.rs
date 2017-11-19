extern crate ioctl_rs as ioctl;
extern crate byteorder;

use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;
use std::path::Path;
use std::str;


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

    let mut record_buffer: [u8; 32767] = [0; 32767];

    for _ in 0..200000 {
        let mut record_type: [u8; 1] = [0; 1];
        let result = file.read(&mut record_type);
        //println!("Record Received type={}", record_type[0]);

        if record_type[0] == AUT_HEADER32 { // ??
            let record_size = file.read_u32::<BigEndian>().unwrap();
            //println!("Record Received size={}", record_size);
            let record_remaining = (record_size - 5) as usize;

            let result = file.read(&mut record_buffer[0..record_remaining]);
            //println!("Read record numBytes={:?}", result);

            if record_buffer[0] == 11 { // ??
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
                    },
                };
                println!("Read path={:?}", p);
            } else {
                println!("Unknown record type");
            }
        } else {
            println!("Unknown header={}", record_type[0]);
        }


        //let p = String::from_utf8_lossy((&record_buffer[43..pathend]));
        //println!("Read path={:?}", p);

        //JRH: index 42 has the length..

        //println!("Four: {} {} {} {}", record_buffer[0], record_buffer[1], record_buffer[2], record_buffer[3]);

        //let token_size = BigEndian::read_u32(&record_buffer[0..5]);
        //println!("Token Received size={}", token_size);

        //for i in 0..record_remaining {
        //println!("  x[{}] = {}",i, record_buffer[i]);
        //}

    }


    setup_serial_port(file.as_raw_fd());

}
