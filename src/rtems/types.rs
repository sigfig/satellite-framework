////////////////////////////////////////////////////////////////////////////////
// common types for the rtems api
////////////////////////////////////////////////////////////////////////////////

use core::ffi::*;

////////////////////////////////////////////////////////////////////////////////
// c standard typedefs

pub type int16_t = c_short;
pub type uint16_t = c_ushort;
pub type int32_t = c_long;
pub type uint32_t = c_ulong;
pub type int64_t = c_longlong;
pub type uint64_t = c_ulonglong;
pub type size_t = c_uint;
pub type time_t = c_int;
pub type uintptr_t = c_uint;

// cpu_set_t turns out to have a very complicated definition in libc
// someone else has probably figured out how to express it properly in rust
pub type cpu_set_t = *mut c_void;

#[repr(C)]
pub struct timeval {
  tv_sec: time_t,
  tv_usec: time_t
}

#[repr(C)]
pub struct timespec {
  tv_sec: time_t,
  tv_usec: time_t
}

#[repr(C)]
pub struct bintime {
  sec: time_t,
  frac: uint64_t
}

////////////////////////////////////////////////////////////////////////////////
// rtems shared typedefs

pub type rtems_status_code = c_int;

pub type rtems_name = uint32_t;
pub type rtems_id = uint32_t;

pub type rtems_interval = uint32_t;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct rtems_time_of_day {
  pub year: uint32_t,
  pub month: uint32_t,
  pub day: uint32_t,
  pub hour: uint32_t,
  pub minute: uint32_t,
  pub second: uint32_t,
  pub ticks: uint32_t
}

pub type rtems_mode = uint32_t;
pub type rtems_attribute = uint32_t;
pub type rtems_option = uint32_t;

pub type rtems_print_printer = unsafe extern "C" fn(
  *mut c_void,
  *const c_char,
  ...
) -> c_int;

#[repr(C)]
pub struct rtems_printer {
  context: *mut c_void,
  printer: rtems_print_printer
}
