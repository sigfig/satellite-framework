////////////////////////////////////////////////////////////////////////////////
// message queue interface
////////////////////////////////////////////////////////////////////////////////

use crate::types::*;
use core::ffi::*;

#[repr(C)]
pub struct rtems_message_queue_config {
  name: rtems_name,
  maximum_pending_messages: uint32_t,
  maximum_message_size: size_t,
  storage_area: *mut c_void,
  storage_size: size_t,
  storage_free: unsafe extern "C" fn(*mut c_void) -> (),
  attributes: rtems_attribute
}

extern "C" {
  pub fn rtems_message_queue_create(
    name: rtems_name,
    count: uint32_t,
    max_message_size: size_t,
    attribute_set: rtems_attribute,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_message_queue_construct(
    config: *const rtems_message_queue_config,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_message_queue_ident(
    name: rtems_name,
    node: uint32_t,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_message_queue_delete(
    id: rtems_id
  ) -> rtems_status_code;

  pub fn rtems_message_queue_send(
    id: rtems_id,
    buffer: *const c_void,
    size: size_t
  ) -> rtems_status_code;

  pub fn rtems_message_queue_urgent(
    id: rtems_id,
    buffer: *const c_void,
    size: size_t
  ) -> rtems_status_code;

  pub fn rtems_message_queue_broadcast(
    id: rtems_id,
    buffer: *const c_void,
    size: size_t,
    count: *mut uint32_t
  ) -> rtems_status_code;

  pub fn rtems_message_queue_receive(
    id: rtems_id,
    buffer: *mut c_void,
    size: *mut size_t,
    option_set: rtems_option,
    timeout: rtems_interval
  ) -> rtems_status_code;

  pub fn rtems_message_queue_get_number_pending(
    id: rtems_id,
    count: *mut uint32_t
  ) -> rtems_status_code;

  pub fn rtems_message_queue_flush(
    id: rtems_id,
    count: *mut uint32_t
  ) -> rtems_status_code;
}
