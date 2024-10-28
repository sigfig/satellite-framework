////////////////////////////////////////////////////////////////////////////////
// task scheduling and management
////////////////////////////////////////////////////////////////////////////////

use crate::types::*;
use core::ffi::*;

// TODO: this struct is fairly complicated
// and it's not clear that we actually need the full definition
// rtems/cpukit/include/rtems/rtems/tasks.h#400
// rtems/cpukit/include/rtems/score/thread.h#814
#[repr(C)]
pub struct rtems_tcb {
  placeholder: uint32_t
}

#[repr(C)]
pub struct rtems_task_config {
  name: rtems_name,
  initial_priority: rtems_task_priority,
  // NOTE: we probably want some abstraction for memory regions like these
  storage_area: *mut c_void,
  storage_size: size_t,
  maximum_thread_local_storage_size: size_t,
  storage_free: unsafe extern "C" fn(*mut c_void) -> (),
  initial_modes: rtems_mode,
  attributes: rtems_attribute
}

// supposed to be a numeric type that can hold both a uin32 and a native pointer
// rtems/cpukit/include/rtems/rtems/tasks.h#100
pub type rtems_task_argument = *mut c_void;
pub type rtems_task_entry = unsafe extern "C" fn(rtems_task_argument) -> ();
pub type rtems_task_priority = uint32_t;
pub type rtems_task_visitor = unsafe extern "C" fn(*mut rtems_tcb, *mut c_void) -> bool;

extern "C" {
  pub fn rtems_task_create(
    name: rtems_name,
    initial_priority: rtems_task_priority,
    stack_size: size_t,
    initial_modes: rtems_mode,
    attribute_set: rtems_attribute,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_task_construct(
    config: *const rtems_task_config,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_task_ident(
    name: rtems_name,
    node: uint32_t,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_task_self() -> rtems_id;

  pub fn rtems_task_start(
    id: rtems_id,
    entry_point: rtems_task_entry,
    argument: rtems_task_argument
  ) -> rtems_status_code;

  pub fn rtems_task_restart(
    id: rtems_id,
    argument: rtems_task_argument
  ) -> rtems_status_code;

  pub fn rtems_task_delete(
    id: rtems_id
  ) -> rtems_status_code;

  pub fn rtems_task_exit() -> ();

  pub fn rtems_task_suspend(
    id: rtems_id
  ) -> rtems_status_code;

  pub fn rtems_task_resume(
    id: rtems_id
  ) -> rtems_status_code;

  pub fn rtems_task_is_suspended(
    id: rtems_id
  ) -> rtems_status_code;

  pub fn rtems_task_set_priority(
    id: rtems_id,
    new_priority: rtems_task_priority,
    old_priority: *mut rtems_task_priority
  ) -> rtems_status_code;

  pub fn rtems_task_get_priority(
    id: rtems_id,
    scheduler_id: rtems_id,
    priority: *mut rtems_task_priority
  ) -> rtems_status_code;

  pub fn rtems_task_mode(
    mode_set: rtems_mode,
    mask: rtems_mode,
    previous_mode_set: *mut rtems_mode
  ) -> rtems_status_code;

  pub fn rtems_task_wake_after(
    ticks: rtems_interval
  ) -> rtems_status_code;

  pub fn rtems_task_wake_when(
    time_buffer: *const rtems_time_of_day
  ) -> rtems_status_code;

  pub fn rtems_task_get_scheduler(
    task_id: rtems_id,
    scheduler_id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_task_set_scheduler(
    task_id: rtems_id,
    scheduler_id: rtems_id,
    priority: rtems_task_priority
  ) -> rtems_status_code;

  pub fn rtems_task_get_affinity(
    id: rtems_id,
    cpusetsize: size_t,
    cpuset: *mut cpu_set_t
  ) -> rtems_status_code;

  pub fn rtems_task_set_affinity(
    id: rtems_id,
    cpusetsize: size_t,
    cpuset: *const cpu_set_t
  ) -> rtems_status_code;

  pub fn rtems_task_iterate(
    visitor: rtems_task_visitor,
    arg: *mut c_void
  ) -> ();
}
