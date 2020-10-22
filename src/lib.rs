#![no_std]
#![feature(fmt_as_str)]
#![allow(dead_code, unused_imports, unused_variables)]
// use core::ptr::null;
// use core::lazy::Lazy;
extern crate alloc;
use alloc::boxed::Box;


use core::future::Future;
// struct String;

//
// extern {
//     /// from libc crate
//     pub fn printf(format: *const u8, ...) -> i32;
// }


pub struct FileDescriptor {
    fd: libc::c_int
}

impl FileDescriptor {
    const STDIN: libc::c_int = 0i32;
    const STDOUT: libc::c_int = 1i32;
    const STDERR: libc::c_int = 2i32;

    pub fn new_stdout() -> Self {
        Self {
            fd: Self::STDOUT
        }
    }

    pub fn print_hello_world(&self) {
        let hello_world = b"Hellow World!\n\0";
        let hello_world_void_ptr = hello_world.as_ptr() as *const core::ffi::c_void;
        unsafe {
            libc::write(self.fd, hello_world_void_ptr, hello_world.len());
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let a = "1\n\0";
        let b = core::str::from_utf8(&[49u8, 0u8]).unwrap();
        let argument_obj = format_args!("a={}, b={}\0", a, b);
        // let d = c.as_str().unwrap();

        // https://stevedonovan.github.io/rustifications/2018/08/13/using-rust-like-c.html
        let a_ptr: *const u8 = a.as_ptr();
        // bad cast [u8] to *const u8, may cause memory error(signal: 11, SIGSEGV: invalid memory reference)
        // let b_ptr: *const u8 = &b.as_bytes()[0];
        unsafe {
            // libc::fork();
            // libc::printf();
            libc::printf(a_ptr as *const libc::c_char);
        }
    }

    #[test]
    fn test_libc_printf() {
        let a = libc::STDOUT_FILENO;
        let hello_world = b"Hellow World!\n\0";
        unsafe {
            libc::printf(hello_world.as_ptr() as *const libc::c_char);
        }
    }

    #[test]
    fn test_my_fd_and_stdout() {
        let fd_stdout = super::FileDescriptor::new_stdout();
        fd_stdout.print_hello_world();
    }
}
