////////////////////////////////////////////////////////////////////////////////
// scheduler
////////////////////////////////////////////////////////////////////////////////

use crate::rtems::types::*;
use crate::rtems::task::*;

extern "C" {
  pub fn rtems_scheduler_ident(
    name: rtems_name,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_scheduler_ident_by_processor(
    cpu_index: uint32_t,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_scheduler_ident_by_processor_set(
    cpusetsize: size_t,
    cpuset: *const cpu_set_t,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_scheduler_get_maximum_priority(
    scheduler_id: rtems_id,
    priority: *mut rtems_task_priority
  ) -> rtems_status_code;

  pub fn rtems_scheduler_map_priority_to_posix(
    scheduler_id: rtems_id,
    priority: *mut rtems_task_priority,
    posix_priority: *mut int32_t
  ) -> rtems_status_code;


  pub fn rtems_scheduler_map_priority_from_posix(
    scheduler_id: rtems_id,
    posix_priority: *mut int32_t,
    priority: *mut rtems_task_priority
  ) -> rtems_status_code;

  pub fn rtems_scheduler_get_processor() -> uint32_t;

  pub fn rtems_scheduler_get_processor_maximum() -> uint32_t;

  pub fn rtems_scheduler_get_processor_set(
    scheduler_id: rtems_id,
    cpusetsize: size_t,
    cpuset: *mut cpu_set_t
  ) -> rtems_status_code;

  pub fn rtems_scheduler_add_processor(
    scheduler_id: rtems_id,
    cpu_index: uint32_t
  ) -> rtems_status_code;

  pub fn rtems_scheduler_remove_processor(
    scheduler_id: rtems_id,
    cpu_index: uint32_t
  ) -> rtems_status_code;
}
