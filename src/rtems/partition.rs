////////////////////////////////////////////////////////////////////////////////
// memory partitions
////////////////////////////////////////////////////////////////////////////////

use crate::rtems::types::*;
use core::ffi::*;

extern "C" {
  pub fn rtems_partition_create(
    name: rtems_name,
    starting_address: *mut c_void,
    length: uintptr_t,
    buffer_size: size_t,
    attribute_set: rtems_attribute,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_partition_ident(
    name: rtems_name,
    node: uint32_t,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_partition_delete(
    id: rtems_id
  ) -> rtems_status_code;

  pub fn rtems_partition_get_buffer(
    id: rtems_id,
    buffer: *mut *mut c_void
  ) -> rtems_status_code;

  pub fn rtems_partition_return_buffer(
    id: rtems_id,
    buffer: *mut c_void
  ) -> rtems_status_code;
}
