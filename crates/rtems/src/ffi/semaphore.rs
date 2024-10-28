////////////////////////////////////////////////////////////////////////////////
// semaphore synchronization
////////////////////////////////////////////////////////////////////////////////

use crate::types::*;
use crate::task::*;

extern "C" {
  pub fn rtems_semaphore_create(
    name: rtems_name,
    count: uint32_t,
    attribute_set: rtems_attribute,
    priority_ceiling: rtems_task_priority,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_semaphore_ident(
    name: rtems_name,
    node: uint32_t,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_semaphore_delete(
    id: rtems_id
  ) -> rtems_status_code;

  pub fn rtems_semaphore_obtain(
    id: rtems_id,
    option_set: rtems_option,
    timeout: rtems_interval
  ) -> rtems_status_code;

  pub fn rtems_semaphore_release(
    id: rtems_id
  ) -> rtems_status_code;

  pub fn rtems_semaphore_flush(
    id: rtems_id
  ) -> rtems_status_code;

  pub fn rtems_semaphore_set_priority(
    semaphore_id: rtems_id,
    scheduler_id: rtems_id,
    new_priority: rtems_task_priority,
    old_priority: *mut rtems_task_priority
  ) -> rtems_status_code;
}
