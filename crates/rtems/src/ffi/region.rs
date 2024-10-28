////////////////////////////////////////////////////////////////////////////////
// memory regions
////////////////////////////////////////////////////////////////////////////////

use crate::types::*;
use core::ffi::*;

#[repr(C)]
pub struct heap_information {
  number: uintptr_t,
  largest: uintptr_t,
  total: uintptr_t
}

#[repr(C)]
pub struct heap_information_statistics {
  lifetime_allocated: uint64_t,
  lifetime_free: uint64_t,
  size: uintptr_t,
  free_size: uintptr_t,
  min_free_size: uintptr_t,
  free_blocks: uint32_t,
  max_free_blocks: uint32_t,
  used_blocks: uint32_t,
  max_search: uint32_t,
  searches: uint32_t,
  allocs: uint32_t,
  failed_allocs: uint32_t,
  frees: uint32_t,
  resizes: uint32_t
}

#[repr(C)]
pub struct heap_information_block {
  free: heap_information,
  used: heap_information,
  stats: heap_information_statistics
}

extern "C" {
  pub fn rtems_region_create(
    name: rtems_name,
    starting_address: *mut c_void,
    length: uintptr_t,
    page_size: uintptr_t,
    attribute_set: rtems_attribute,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_region_ident(
    name: rtems_name,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_region_delete(
    id: rtems_id
  ) -> rtems_status_code;

  pub fn rtems_region_extend(
    id: rtems_id,
    starting_address: *mut c_void,
    length: uintptr_t
  ) -> rtems_status_code;

  pub fn rtems_region_get_segment(
    id: rtems_id,
    size: uintptr_t,
    option_set: rtems_option,
    timeout: rtems_interval,
    segment: *mut *mut c_void
  ) -> rtems_status_code;

  pub fn rtems_region_return_segment(
    id: rtems_id,
    segment: *mut c_void
  ) -> rtems_status_code;

  pub fn rtems_region_resize_segment(
    id: rtems_id,
    segment: *mut c_void,
    size: uintptr_t,
    old_size: *mut uintptr_t
  ) -> rtems_status_code;

  pub fn rtems_region_get_information(
    id: rtems_id,
    the_info: *mut heap_information_block
  ) -> rtems_status_code;

  pub fn rtems_region_get_free_information(
    id: rtems_id,
    the_info: *mut heap_information_block
  ) -> rtems_status_code;

  pub fn rtems_region_get_segment_size(
    id: rtems_id,
    segment: *mut c_void,
    size: *mut uintptr_t
  ) -> rtems_status_code;
}
