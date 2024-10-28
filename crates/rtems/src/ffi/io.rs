////////////////////////////////////////////////////////////////////////////////
// io driver management and character device access
////////////////////////////////////////////////////////////////////////////////

use crate::types::*;
use core::ffi::*;

pub type rtems_device_major_number = uint32_t;
pub type rtems_device_minor_number = uint32_t;

pub type rtems_device_driver_entry = unsafe extern "C" fn(
  rtems_device_major_number,
  rtems_device_minor_number,
  *mut c_void
) -> rtems_status_code;

#[repr(C)]
pub struct rtems_driver_address_table {
  initialization_entry: rtems_device_driver_entry,
  open_entry: rtems_device_driver_entry,
  close_entry: rtems_device_driver_entry,
  read_entry: rtems_device_driver_entry,
  write_entry: rtems_device_driver_entry,
  control_entry: rtems_device_driver_entry,
}

extern "C" {
  pub fn rtems_io_register_driver(
    major: rtems_device_major_number,
    driver_table: *const rtems_driver_address_table,
    registered_major: *mut rtems_device_major_number
  ) -> rtems_status_code;

  pub fn rtems_io_unregister_driver(
    major: rtems_device_major_number
  ) -> rtems_status_code;

  pub fn rtems_io_initialize(
    major: rtems_device_major_number,
    minor: rtems_device_minor_number,
    argument: *mut c_void
  ) -> rtems_status_code;

  pub fn rtems_io_register_name(
    device_name: *const c_char,
    major: rtems_device_major_number,
    minor: rtems_device_minor_number
  ) -> rtems_status_code;

  pub fn rtems_io_open(
    major: rtems_device_major_number,
    minor: rtems_device_minor_number,
    argument: *mut c_void
  ) -> rtems_status_code;

  pub fn rtems_io_close(
    major: rtems_device_major_number,
    minor: rtems_device_minor_number,
    argument: *mut c_void
  ) -> rtems_status_code;

  pub fn rtems_io_read(
    major: rtems_device_major_number,
    minor: rtems_device_minor_number,
    argument: *mut c_void
  ) -> rtems_status_code;

  pub fn rtems_io_write(
    major: rtems_device_major_number,
    minor: rtems_device_minor_number,
    argument: *mut c_void
  ) -> rtems_status_code;

  pub fn rtems_io_control(
    major: rtems_device_major_number,
    minor: rtems_device_minor_number,
    argument: *mut c_void
  ) -> rtems_status_code;
}

////////////////////////////////////////////////////////////////////////////////
// character device io

extern "C" {
  pub fn rtems_putc(c: c_char) -> ();
  pub fn putk(s: *const c_char) -> c_int;
  pub fn printk(fmt: *const c_char, ...) -> c_int;
  pub fn getchark() -> c_int;
}
