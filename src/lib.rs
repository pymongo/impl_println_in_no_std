#![no_std]
use core::fmt::Write;
// use core::ffi::c_void;

// pub unsafe extern "C" fn write(
//     fd: i32,
//     ptr: *const cor,
//     size: size_t
// ) -> ssize_t
// pub struct StdoutWriter;


impl core::fmt::Write for StdoutWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        const STDOUT_FD: libc::c_int = 1;
        unsafe {
            libc::write(STDOUT_FD, s.as_ptr() as *const core::ffi::c_void, s.len());
        }
        Ok(())
    }
}

impl StdoutWriter {
    #[inline]
    pub fn write_fmt_helper(&mut self, args: core::fmt::Arguments) {
        core::fmt::Write::write_fmt(self, args).unwrap()
    }

    #[inline]
    pub fn write_str_helper(&mut self, s: &str) {
        self.write_str(s).unwrap()
    }
}

#[macro_export]
macro_rules! my_println {
    () => { $crate::my_println!("") };
    ($($arg:tt)*) => {
        {
            let mut writer = $crate::StdoutWriter;
            writer.write_fmt_helper(format_args!($($arg)*));
            writer.write_str_helper("\n");
        }
    };
}

#[test]
fn test_my_println() {
    let name = "Rust";
    my_println!("Hello {name}", name=name);
}

#[test]
fn test_libc_printf() {
    let hello_world = b"Hellow World!\n\0";
    unsafe {
        libc::printf(hello_world.as_ptr() as *const libc::c_char);
    }
}

#[test]
fn test_libc_write() {
    // https://stevedonovan.github.io/rustifications/2018/08/13/using-rust-like-c.html
    let s = b"libc::write\n\0";
    unsafe {
        libc::write(1, s.as_ptr() as *const core::ffi::c_void, s.len());
    }
}
