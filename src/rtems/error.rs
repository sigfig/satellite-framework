////////////////////////////////////////////////////////////////////////////////
// error management
////////////////////////////////////////////////////////////////////////////////

use crate::rtems::types::*;
use core::ffi::*;

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
