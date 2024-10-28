////////////////////////////////////////////////////////////////////////////////
// cpu cache management
////////////////////////////////////////////////////////////////////////////////

use crate::types::*;
use core::ffi::*;

extern "C" {
  pub fn rtems_cache_flush_multiple_data_lines(
    begin: *const c_void,
    size: size_t
  ) -> ();

  pub fn rtems_cache_invalidate_multiple_data_lines(
    begin: *const c_void,
    size: size_t
  ) -> ();

  pub fn rtems_cache_invalidate_multiple_instruction_lines(
    begin: *const c_void,
    size: size_t
  ) -> ();

  pub fn rtems_cache_instruction_sync_after_code_change(
    begin: *const c_void,
    size: size_t
  ) -> ();

  pub fn rtems_cache_get_maximal_line_size() -> size_t;

  pub fn rtems_cache_get_data_line_size() -> size_t;

  pub fn rtems_cache_get_instruction_line_size() -> size_t;

  pub fn rtems_cache_get_data_cache_size(
    level: uint32_t
  ) -> size_t;

  pub fn rtems_cache_get_instrution_cache_size(
    level: uint32_t
  ) -> size_t;

  pub fn rtems_cache_flush_entire_data() -> ();

  pub fn rtems_cache_invalidate_entire_data() -> ();

  pub fn rtems_cache_invalidate_entire_instruction() -> ();

  pub fn rtems_cache_enable_date() -> ();

  pub fn rtems_cache_disable_data() -> ();

  pub fn rtems_cache_enable_instruction() -> ();

  pub fn rtems_cache_disable_instruction() -> ();

  pub fn rtems_cache_aligned_malloc(
    size: size_t
  ) -> *mut c_void;
}
