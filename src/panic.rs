use crate::rtems::*;

use core::ffi::c_char;

#[panic_handler]
fn panic(panic: &core::panic::PanicInfo) -> ! {
    if let Some(location) = panic.location() {
        const FORMAT_STR: *const c_char = {
            const BYTES: &[u8] = b"Panic occurred at %.*s:%d:%d\n\0";
            BYTES.as_ptr().cast()
        };
        if location.file().len() != 0 {
            unsafe {
                rtems_panic(FORMAT_STR,
                    location.file().len() as core::ffi::c_int,
                    location.file().as_ptr(),
                    location.line() as core::ffi::c_int,
                    location.column() as core::ffi::c_int,
                );
            }
        }
    }

    let message = "Panic occured!";
    const FORMAT_PTR: *const c_char = {
        const BYTES: &[u8] = b"%.*s\n\0";
        BYTES.as_ptr().cast()
    };
    unsafe {
       rtems_panic(FORMAT_PTR,
           message.len() as core::ffi::c_int,
           message.as_ptr());
    }
}
