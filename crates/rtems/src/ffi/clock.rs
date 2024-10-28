////////////////////////////////////////////////////////////////////////////////
// clock management
////////////////////////////////////////////////////////////////////////////////

use crate::types::*;

extern "C" {
  pub fn rtems_clock_set(
    time_of_day: *const rtems_time_of_day
  ) -> rtems_status_code;

  pub fn rtems_clock_get_tod(
    time_of_day: *mut rtems_time_of_day
  ) -> rtems_status_code;

  pub fn rtems_clock_get_tod_timeval(
    time_of_day: *mut timeval
  ) -> rtems_status_code;

  pub fn rtems_clock_get_realtime(
    time_snapshot: *mut timespec
  ) -> ();

  pub fn rtems_clock_get_realtime_bintime(
    time_snapshot: *mut bintime
  ) -> ();

  pub fn rtems_clock_get_realtime_timeval(
    time_snapshot: *mut timeval
  ) -> ();

  pub fn rtems_clock_get_realtime_coarse(
    time_snapshot: *mut timespec
  ) -> ();

  pub fn rtems_clock_get_realtime_coarse_bintime(
    time_snapshot: *mut bintime
  ) -> ();

  pub fn rtems_clock_get_realtime_coarse_timeval(
    time_snapshot: *mut timeval
  ) -> ();

  pub fn rtems_clock_get_monotonic(
    time_snapshot: *mut timespec
  ) -> ();

  pub fn rtems_clock_get_monotonic_bintime(
    time_snapshot: *mut bintime
  ) -> ();

  pub fn rtems_clock_get_monotonic_sbintime() -> int64_t;

  pub fn rtems_clock_get_monotonic_timeval(
    time_snapshot: *mut timeval
  ) -> ();

  pub fn rtems_clock_get_monotonic_coarse(
    time_snapshot: *mut timespec
  ) -> ();

  pub fn rtems_clock_get_monotonic_coarse_bintime(
    time_snapshot: *mut bintime
  ) -> ();

  pub fn rtems_clock_get_monotonic_coarse_timeval(
    time_snapshot: *mut timeval
  ) -> ();

  pub fn rtems_clock_get_boot_time(
    boot_time: *mut timespec
  ) -> ();

  pub fn rtems_clock_get_boot_time_bintime(
    boot_time: *mut bintime
  ) -> ();

  pub fn rtems_clock_get_boot_time_timeval(
    boot_time: *mut timeval
  ) -> ();

  pub fn rtems_clock_get_seconds_since_epoch(
    seconds_since_rtems_epoch: rtems_interval
  ) -> rtems_status_code;

  pub fn rtems_clock_get_ticks_per_second() -> rtems_interval;

  pub fn rtems_clock_get_ticks_since_boot() -> rtems_interval;

  pub fn rtems_clock_get_uptime(
    uptime: timespec
  ) -> rtems_status_code;

  pub fn rtems_clock_get_uptime_timeval(
    uptime: *mut timeval
  ) -> ();

  pub fn rtems_clock_get_uptime_seconds() -> time_t;

  pub fn rtems_clock_get_uptime_nanoseconds() -> uint64_t;

  pub fn rtems_clock_tick_later(
    delta: rtems_interval
  ) -> rtems_interval;

  pub fn rtems_clock_tick_later_usec(
    delta_in_usec: rtems_interval
  ) -> rtems_interval;

  pub fn rtems_clock_tick_before(
    ticks: rtems_interval
  ) -> bool;
}
