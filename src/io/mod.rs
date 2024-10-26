use crate::rtems::*;

pub struct Console;

impl core::fmt::Write for Console {
    fn write_str(&mut self, message: &str) -> core::fmt::Result {
        const FORMAT_STR: &core::ffi::CStr = {
            let Ok(s) = core::ffi::CStr::from_bytes_with_nul(b"%.*s\0") else {
                panic!()
            };
            s
        };
        if message.len() != 0 {
            unsafe {
                printk(FORMAT_STR.as_ptr(), message.len() as core::ffi::c_int, message.as_ptr());
            }
        }
        Ok(())
    }
}
