#![no_std]
#![feature(fmt_as_str)]
#![allow(dead_code, unused_imports)]
// use core::ptr::null;
// use core::lazy::Lazy;
extern crate libc;
extern crate alloc;

use core::future::Future;
// struct String;

extern {
    /// from libc crate
    pub fn printf(format: *const u8, ...) -> i32;
}

#[cfg(test)]
mod tests {
    use printf;
    use alloc::boxed::Box;

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
            printf(a_ptr);
        }
    }
}
