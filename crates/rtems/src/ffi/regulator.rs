////////////////////////////////////////////////////////////////////////////////
// stream burst regulation support
////////////////////////////////////////////////////////////////////////////////

use crate::types::*;
use crate::task::*;
use crate::ratemon::*;
use core::ffi::*;

pub type rtems_regulator_deliverer = unsafe extern "C" fn(
  *mut c_void,
  *mut c_void,
  size_t
) -> bool;

#[repr(C)]
pub struct rtems_regulator_attributes {
  deliverer: rtems_regulator_deliverer,
  deliverer_context: *mut c_void,
  maximum_message_size: size_t,
  maximum_messages: size_t,
  delivery_thread_priority: rtems_task_priority,
  delivery_thread_stack_size: size_t,
  delivery_thread_period: rtems_interval,
  maximum_to_deque_per_period: size_t
}

#[repr(C)]
pub struct rtems_regulator_statistics {
  obtained: size_t,
  released: size_t,
  delivered: size_t,
  period_statistics: rtems_rate_monotonic_period_statistics
}

#[repr(C)]
pub struct rtems_regulator_instance {
  initialized: uint32_t,
  attributes: rtems_regulator_attributes,
  message_memory: *mut c_void,
  message_queue_storage: *mut c_void,
  queue_id: rtems_id,
  messages_partition_id: rtems_id,
  delivery_thread_id: rtems_id,
  delivery_thread_period_id: rtems_id,
  delivery_thread_is_running: bool,
  delivery_thread_request_exit: bool,
  delivery_thread_has_exited: bool,
  statistics: rtems_regulator_statistics
}

extern "C" {
  pub fn rtems_regulator_create(
    attributes: *mut rtems_regulator_attributes,
    regulator: *mut *mut rtems_regulator_instance
  ) -> rtems_status_code;

  pub fn rtems_regulator_delete(
    regulator: *mut rtems_regulator_instance,
    ticks: rtems_interval
  ) -> rtems_status_code;

  pub fn rtems_regulator_obtain_buffer(
    regulator: *mut rtems_regulator_instance,
    buffer: *mut *mut c_void
  ) -> rtems_status_code;

  pub fn rtems_regulator_release_buffer(
    regulator: *mut rtems_regulator_instance,
    buffer: *mut c_void
  ) -> rtems_status_code;

  pub fn rtems_regulator_send(
    regulator: *mut rtems_regulator_instance,
    message: *mut c_void,
    length: size_t
  ) -> rtems_status_code;

  pub fn rtems_regulator_get_statistics(
    regulator: *mut rtems_regulator_instance,
    statistics: *mut rtems_regulator_statistics
  ) -> rtems_status_code;
}
