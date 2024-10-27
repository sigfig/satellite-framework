////////////////////////////////////////////////////////////////////////////////
// barrier synchronization
////////////////////////////////////////////////////////////////////////////////

use crate::rtems::types::*;

extern "C" {
  pub fn rtems_barrier_create(
    name: rtems_name,
    attribute_set: rtems_attribute,
    maximum_waiters: uint32_t,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_barrier_ident(
    name: rtems_name,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_barrier_delete(
    id: rtems_id
  ) -> rtems_status_code;

  pub fn rtems_barrier_wait(
    id: rtems_id,
    timeout: interval
  ) -> rtems_status_code;

  pub fn rtems_barrier_release(
    id: rtems_id,
    released: *mut uint32_t
  ) -> rtems_status_code;
}
