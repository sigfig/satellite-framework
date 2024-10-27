////////////////////////////////////////////////////////////////////////////////
// semaphor synchronization
////////////////////////////////////////////////////////////////////////////////

use crate::rtems::types::*;
use crate::rtems::task::*;

extern "C" {
  pub fn rtems_semaphor_create(
    name: rtems_name,
    count: uint32_t,
    attribute_set: rtems_attribute,
    priority_ceiling: rtems_task_priority,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_semaphor_ident(
    name: rtems_name,
    node: uint32_t,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_semaphor_delete(
    id: rtems_id
  ) -> rtems_status_code;

  pub fn rtems_semaphor_obtain(
    id: rtems_id,
    option_set: rtems_option,
    timeout: rtems_interval
  ) -> rtems_status_code;

  pub fn rtems_semaphor_release(
    id: rtems_id
  ) -> rtems_status_code;

  pub fn rtems_semaphor_flush(
    id: rtems_id
  ) -> rtems_status_code;

  pub fn rtems_semaphor_set_priority(
    semaphore_id: rtems_id,
    scheduler_id: rtems_id,
    new_priority: rtems_task_priority,
    old_priority: *mut rtems_task_priority
  ) -> rtems_status_code;
}
