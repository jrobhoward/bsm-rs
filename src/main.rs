extern crate ioctl_rs as ioctl;

use std::fs::File;
use std::io::prelude::*;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;
use std::path::Path;


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


fn main() {
    println!("Hello, world!");
    let path = Path::new("/dev/auditpipe");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    setup_serial_port(file.as_raw_fd());

    let mut buffer = [0; 32767];
    let result = file.read(&mut buffer);
    for i in 0..result.unwrap() {
        println!("Goodbye, world! {}", buffer[i]);
    }
}
