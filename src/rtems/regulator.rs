////////////////////////////////////////////////////////////////////////////////
// stream burst regulation support
////////////////////////////////////////////////////////////////////////////////

use crate::rtems::types::*;
use core::ffi::*;

pub type rtems_regulator_attributes;
pub type rtems_regulator_instance;
pub type rtems_regulator_statistics;

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
