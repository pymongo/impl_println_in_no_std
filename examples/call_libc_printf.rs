fn main() {
    let hello_world = b"Hellow World!\n\0";
    unsafe {
        libc::printf(hello_world.as_ptr() as *const libc::c_char);
    }
}