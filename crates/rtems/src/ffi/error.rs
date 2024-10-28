////////////////////////////////////////////////////////////////////////////////
// error management
////////////////////////////////////////////////////////////////////////////////

use crate::types::*;
use core::ffi::*;

// TODO:
// this exception frame struct is arch-specific
// i've documented the one for arm here but not implemented it
// this is likely to require some thought to get working

// equivalent to CPU_Exception_frame for arm
// rtems/cpukit/score/cpu/arm/include/rtems/score/cpu.h#640
#[repr(C)]
pub struct rtems_exception_frame {
  placeholder: uint32_t
}

// enum
// rtems/cpukit/include/rtems/score/interr.h#63
pub type rtems_fatal_source = uint32_t;

// another usage of uint32ptr
// rtems/cpukit/include/rtems/score/interr.h#238
pub type rtems_fatal_code = uint32_t;

extern "C" {
  pub fn rtems_fatal(
    fatal_source: rtems_fatal_source,
    fatal_code: rtems_fatal_code
  ) -> !;

  pub fn rtems_panic(
    fmt: *const c_char,
    ...
  ) -> !;

  pub fn rtems_shutdown_executive(
    fatal_code: uint32_t
  ) -> !;

  pub fn rtems_exception_frame_print(
    frame: *const rtems_exception_frame
  ) -> ();

  pub fn rtems_fatal_source_text(
    fatal_source: rtems_fatal_source
  ) -> *const c_char;

  pub fn rtems_internal_error_text(
    internal_error_code: rtems_fatal_code
  ) -> *const c_char;

  pub fn rtems_fatal_error_occurred(
    fatal_code: uint32_t
  ) -> ();
}
