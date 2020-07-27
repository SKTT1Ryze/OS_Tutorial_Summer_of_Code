#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::syscall::*;
use user_lib::alloc::string::String;
#[no_mangle]
pub fn main() -> usize {
    println!("Hello world from user mode program!");
    println!("tid: {}",sys_get_tid());
    println!("fork: {}",sys_fork());
    println!("tid: {}",sys_get_tid());
    let file = sys_open("test.rs");
    let mut buffer = [0u8; 1024];
    let size = sys_read(file as usize, &mut buffer);
    match String::from_utf8(buffer.iter().copied().take(size as usize).collect()) {
        Ok(string) => {
            print!("{}", string);
        }
        _ => {},
    }
    0
}
