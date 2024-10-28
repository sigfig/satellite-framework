////////////////////////////////////////////////////////////////////////////////
// timer management
////////////////////////////////////////////////////////////////////////////////

use crate::types::*;
use crate::task::*;
use core::ffi::*;

pub type rtems_timer_service_routine_entry = unsafe extern "C" fn(
  rtems_id,
  *mut c_void
) -> ();

// enum
// rtems/cpukit/include/rtems/rtems/timer.h#114
pub type timer_classes = uint32_t;

pub type watchdog_interval = uint32_t;

#[repr(C)]
pub struct rtems_timer_information {
  the_class: timer_classes,
  initial: watchdog_interval,
  start_time: watchdog_interval,
  stop_time: watchdog_interval
}

extern "C" {
  pub fn rtems_timer_create(
    name: rtems_name,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_timer_ident(
    name: rtems_name,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_timer_cancel(
    id: rtems_id
  ) -> rtems_status_code;

  pub fn rtems_timer_delete(
    id: rtems_id
  ) -> rtems_status_code;

  pub fn rtems_timer_fire_after(
    id: rtems_id,
    ticks: rtems_interval,
    routine: rtems_timer_service_routine_entry,
    user_data: *mut c_void
  ) -> rtems_status_code;

  pub fn rtems_timer_fire_when(
    id: rtems_id,
    wall_time: *const rtems_time_of_day,
    routine: rtems_timer_service_routine_entry,
    user_data: *mut c_void
  ) -> rtems_status_code;

  pub fn rtems_timer_initiate_server(
    priority: rtems_task_priority,
    stack_size: size_t,
    attribute_set: rtems_attribute
  ) -> rtems_status_code;

  pub fn rtems_timer_server_fire_after(
    id: rtems_id,
    ticks: rtems_interval,
    routine: rtems_timer_service_routine_entry,
    user_data: *mut c_void
  ) -> rtems_status_code;

  pub fn rtems_timer_server_fire_when(
    id: rtems_id,
    wall_time: *const rtems_time_of_day,
    routine: rtems_timer_service_routine_entry,
    user_data: *mut c_void
  ) -> rtems_status_code;

  pub fn rtems_timer_reset(
    id: rtems_id
  ) -> rtems_status_code;

  pub fn rtems_timer_get_information(
    id: rtems_id,
    the_info: rtems_timer_information
  ) -> rtems_status_code;
}
