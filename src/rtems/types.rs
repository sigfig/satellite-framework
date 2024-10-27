////////////////////////////////////////////////////////////////////////////////
// common types for the rtems api
////////////////////////////////////////////////////////////////////////////////

use core::ffi::*;
use core::mem::ManuallyDrop;

////////////////////////////////////////////////////////////////////////////////
// c standard typedefs

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
pub struct rtems_time_of_day {
  year: uint32_t,
  month: uint32_t,
  day: uint32_t,
  hour: uint32_t,
  minute: uint32_t,
  second: uint32_t,
  ticks: uint32_t
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

////////////////////////////////////////////////////////////////////////////////
// rtems datastructures

// this is a doubly-linked list structure used in rtems
// it is sneakily generic
// it is cast to a larger struct type that has chain_node as its first element
// not sure how much it gets used in application code
// rtems/cpukit/include/rtems/score/chain.h#78
#[repr(C)]
pub struct chain_node {
  next: *mut chain_node,
  previous: *mut chain_node
}

#[repr(C)]
pub struct chain_head {
  node: chain_node,
  fill: *mut chain_node
}

#[repr(C)]
pub struct chain_tail {
  fill: *mut chain_node,
  node: chain_node
}

#[repr(C)]
pub union chain_control {
  head: ManuallyDrop<chain_head>,
  tail: ManuallyDrop<chain_tail>
}
