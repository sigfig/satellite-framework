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
// rtems status codes

pub type rtems_status_code = c_int;

extern "C" {
  pub fn rtems_status_code_to_errno(status_code: rtems_status_code) -> c_int;
  pub fn rtems_status_text(status_code: rtems_status_code) -> *const c_char;
}

pub const RTEMS_SUCCESSFUL: c_int = 0;
pub const RTEMS_TASK_EXITTED: c_int = 1;
pub const RTEMS_MP_NOT_CONFIGURED: c_int = 2;
pub const RTEMS_INVALID_NAME: c_int = 3;
pub const RTEMS_INVALID_ID: c_int = 4;
pub const RTEMS_TOO_MANY: c_int = 5;
pub const RTEMS_TIMEOUT: c_int = 6;
pub const RTEMS_OBJECT_WAS_DELETED: c_int = 7;
pub const RTEMS_INVALID_SIZE: c_int = 8;
pub const RTEMS_INVALID_ADDRESS: c_int = 9;
pub const RTEMS_INVALID_NUMBER: c_int = 10;
pub const RTEMS_NOT_DEFINED: c_int = 11;
pub const RTEMS_RESOURCE_IN_USE: c_int = 12;
pub const RTEMS_UNSATISFIED: c_int = 13;
pub const RTEMS_INCORRECT_STATE: c_int = 14;
pub const RTEMS_ALREAD_SUSPENDED: c_int = 15;
pub const RTEMS_ILLEGAL_ON_SELF: c_int = 16;
pub const RTEMS_ILLEGAL_ON_REMOTE_OBJECT: c_int = 17;
pub const RTEMS_CALLED_FROM_ISR: c_int = 18;
pub const RTEMS_INVALID_PRIORITY: c_int = 19;
pub const RTEMS_INVALID_CLOCK: c_int = 20;
pub const RTEMS_INVALID_NODE: c_int = 21;
pub const RTEMS_NOT_CONFIGURED: c_int = 22;
pub const RTEMS_NOT_OWNER_OF_RESOURCE: c_int = 23;
pub const RTEMS_NOT_IMPLEMENTED: c_int = 24;
pub const RTEMS_INTERNAL_ERROR: c_int = 25;
pub const RTEMS_NO_MEMORY: c_int = 26;
pub const RTEMS_IO_ERROR: c_int = 27;
pub const RTEMS_INTERRUPTED: c_int = 28;
pub const RTEMS_PROXY_BLOCKING: c_int = 29;

////////////////////////////////////////////////////////////////////////////////
// rtems mode bitflags

pub type rtems_mode = uint32_t;

extern "C" {
  // TODO: not sure if this works
  static rtems_interrupt_mask: uint32_t;
}

pub const RTEMS_ALL_MODE_MASKS: uint32_t = 0x0000ffff;
pub const RTEMS_ASR: uint32_t = 0x0;
pub const RTEMS_ASR_MASK: uint32_t = 0x00000400;
pub const RTEMS_CURRENT_MODE: uint32_t = 0x0;
pub const RTEMS_DEFAULT_MODES: uint32_t = 0x0;
pub const RTEMS_INTERRUPT_MASK: uint32_t = rtems_interupt_mask;
// TODO: make this a macro
pub fn RTEMS_INTERRUPT_LEVEL(level: uint32_t) -> uint32_t { level & RTEMS_INTERRUPT_MASK }
pub const RTEMS_NO_ASR: uint32_t = 0x00000400;
pub const RTEMS_NO_PREEMPT: uint32_t = 0x00000100;
pub const RTEMS_NO_TIMESLICE: uint32_t = 0x00000000;
pub const RTEMS_NO_PREEMPT: uint32_t = 0x00000000;
pub const RTEMS_NO_PREEMPT_MASK: uint32_t = 0x00000100;
pub const RTEMS_NO_TIMESLICE: uint32_t = 0x00000200;
pub const RTEMS_NO_TIMESLICE_MASK: uint32_t = 0x00000200;

////////////////////////////////////////////////////////////////////////////////
// rtems attribute bitflags

pub type rtems_attribute = uint32_t;

pub const RTEMS_APPLICATION_TASK: uint32_t = 0x0;
pub const RTEMS_BARRIER_AUTOMATIC_RELEASE: uint32_t = 0x00000200;
pub const RTEMS_BARRIER_MANUAL_RELEASE: uint32_t = 0x00000000;
pub const RTEMS_BINARY_SEMAPHORE: uint32_t = 0x00000010;
pub const RTEMS_COUNTING_SEMAPHORE: uint32_t = 0x00000000;
pub const RTEMS_DEFAULT_ATTRIBUTES: uint32_t = 0x00000000;
pub const RTEMS_FIFO: uint32_t = 0x00000000;
pub const RTEMS_FLOATING_POINT: uint32_t = 0x00000001;
pub const RTEMS_GLOBAL: uint32_t = 0x00000002;
pub const RTEMS_INHERIT_PRIORITY: uint32_t = 0x00000040;
pub const RTEMS_LOCAL: uint32_t = 0x00000000;
pub const RTEMS_MULTIPROCESSOR_RESOURCE_SHARING: uint32_t = 0x00000100;
pub const RTEMS_NO_FLOATING_POINT: uint32_t = 0x00000000;
pub const RTEMS_NO_INHERIT_PRIORITY: uint32_t = 0x00000000;
pub const RTEMS_NO_MULTIPROCESSOR_RESOURCE_SHARING: uint32_t = 0x00000000;
pub const RTEMS_NO_PRIORITY_CEILING: uint32_t = 0x00000000;
pub const RTEMS_PRIORITY: uint32_t = 0x00000004;
pub const RTEMS_PRIORITY_CEILING: uint32_t = 0x00000080;
pub const RTEMS_SEMAPHORE_CLASS: uint32_t = 0x00000030;
pub const RTEMS_SIMPLE_BINARY_SEMAPHORE: uint32_t = 0x00000020;
pub const RTEMS_SYSTEM_TASK: uint32_t = 0x00008000;
pub const RTEMS_: uint32_t = 0x00000200;

////////////////////////////////////////////////////////////////////////////////
// rtems attribute bitflags

pub type rtems_option = uint32_t;

pub const RTEMS_DEFAULT_OPTIONS: uint32_t = 0x00000000;
pub const RTEMS_EVENT_ALL: uint32_t = 0x00000000;
pub const RTEMS_EVENT_ANY: uint32_t = 0x00000002;
pub const RTEMS_NO_WAIT: uint32_t = 0x00000001;
pub const RTEMS_WAIT: uint32_t = 0x00000000;
