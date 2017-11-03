extern crate ioctl_rs as ioctl;

use std::io;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::os::unix::io::RawFd;
use std::os::unix::io::AsRawFd;


fn setup_serial_port(fd: RawFd) -> bool {
    let mut qlen = 0 as i32;
    let mut stat = 0 as u64;

    unsafe {
      //ioctl::ioctl(fd, qlen);
      ioctl::ap_get_qlimit_max(fd, &mut qlen);
      println!("done, qlen={}", qlen);

      ioctl::ap_get_drops(fd, &mut stat);
      println!("done, drops={}", stat);

      ioctl::ap_get_truncates(fd, &mut stat);
      println!("done, truncates={}", stat);
    }

    true
}


fn main() {
    println!("Hello, world!");
    let path = Path::new("/dev/auditpipe");
    let display = path.display();

    let mut file = match File::open(&path) {
    Err(why) => panic!("couldn't open {}", display),
    Ok(file) => file,
};
    setup_serial_port(file.as_raw_fd());
    println!("Goodbye, world!");
}
