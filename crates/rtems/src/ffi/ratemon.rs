////////////////////////////////////////////////////////////////////////////////
// monotonic timer management
////////////////////////////////////////////////////////////////////////////////

use crate::types::*;

// enum
// rtems/cpukit/include/rtems/rtems/ratemon.h#91
pub type rtems_rate_monotonic_period_states = uint32_t;

#[repr(C)]
pub struct rtems_rate_monotonic_period_status {
  owner: rtems_id,
  state: rtems_rate_monotonic_period_states,
  since_last_period: timespec,
  executed_since_last_period: timespec,
  postponed_jobs_count: uint32_t
}

#[repr(C)]
pub struct rtems_rate_monotonic_period_statistics {
  count: uint32_t,
  missed_count: uint32_t,
  min_cpu_time: timespec,
  max_cpu_time: timespec,
  total_cpu_time: timespec,
  min_wall_time: timespec,
  max_wall_time: timespec,
  total_wall_time: timespec
}

extern "C" {
  pub fn rtems_rate_monotonic_create(
    name: rtems_name,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_rate_monotonic_ident(
    name: rtems_name,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_rate_monotonic_cancel(
    id: rtems_id
  ) -> rtems_status_code;

  pub fn rtems_rate_monotonic_delete(
    id: rtems_id
  ) -> rtems_status_code;

  pub fn rtems_rate_monotonic_period(
    id: rtems_id,
    length: rtems_interval
  ) -> rtems_status_code;

  pub fn rtems_rate_monotonic_get_status(
    id: rtems_id,
    status: *mut rtems_rate_monotonic_period_status
  ) -> rtems_status_code;

  pub fn rtems_rate_monotonic_get_statistics(
    id: rtems_id,
    status: *mut rtems_rate_monotonic_period_statistics
  ) -> rtems_status_code;

  pub fn rtems_rate_monotonic_reset_statistics(
    id: rtems_id
  ) -> rtems_status_code;

  pub fn rtems_rate_monotonic_reset_all_statistics() -> ();

  pub fn rtems_rate_monotonic_report_statistics() -> ();

  pub fn rtems_rate_monotonic_report_statistics_with_plugin(
    printer: *const rtems_printer
  ) -> rtems_status_code;
}
