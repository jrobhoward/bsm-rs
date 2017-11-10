extern crate ioctl_rs as ioctl;

use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;
use std::path::Path;


fn setup_serial_port(fd: RawFd) -> bool {
    let current_limit: u32 = ioctl::ap_get_qlimit(fd).unwrap();
    println!("done, current_limit={}", current_limit);

    let max_limit: u32 = ioctl::ap_get_qlimit_max(fd).unwrap();
    println!("done, max_limit={}", max_limit);

    ioctl::ap_set_qlimit(fd, max_limit).unwrap();

    if current_limit < max_limit {
        let updated_limit: u32 = ioctl::ap_get_qlimit(fd).unwrap();
        println!("done, updated_limit={}", updated_limit);
    }

    let cur_size: u32 = ioctl::ap_get_len(fd).unwrap();
    println!("done, cur_size={}", cur_size);

    // consider returning a wrapped result of the max buffer size...
    let max_buflen: u32 = ioctl::ap_get_maxauditdata_size(fd).unwrap();
    println!("done, max_buflen={}", max_buflen);

    ioctl::ap_flush(fd).unwrap();

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

    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    setup_serial_port(file.as_raw_fd());
    println!("Goodbye, world!");
}
