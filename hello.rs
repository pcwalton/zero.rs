#[allow(ctypes)];
#[no_std];

mod zero;

extern {
    #[fast_ffi]
    pub fn write(fd: i32, buf: *u8, nbyte: uint) -> uint;
}

fn main() {
    unsafe {
        let _ = write(0, &"Hello world!\n"[0], 13);
    }
}

