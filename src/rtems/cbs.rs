////////////////////////////////////////////////////////////////////////////////
// constant-bandwidth scheduler interface
////////////////////////////////////////////////////////////////////////////////

use crate::rtems::types::*;
use core::ffi::*;

pub type rtems_cbs_parameters;
pub type rtems_cbs_budget_overrun;
pub type rtems_cbs_server_id;

extern "C" {
  pub fn rtems_cbs_initialize() -> c_int;

  pub fn rtems_cbs_cleanup() -> c_int;

  pub fn rtems_cbs_create_server(
    params: *mut rtems_cbs_parameters,
    budget_overrun_callback: rtems_cbs_budget_overrun,
    server_id: *mut rtems_cbs_server_id
  ) -> c_int;

  pub fn rtems_cbs_attach_thread(
    server_id: rtems_cbs_server_id,
    task_id: rtems_id
  ) -> c_int;

  pub fn rtems_cbs_detach_thread(
    server_id: rtems_cbs_server_id,
    task_id: rtems_id
  ) -> c_int;

  pub fn rtems_cbs_destroy_server(
    server_id: rtems_cbs_server_id
  ) -> c_int;

  pub fn rtems_cbs_get_server_id(
    task_id: rtems_id,
    server_id: *mut rtems_cbs_server_id
  ) -> c_int;

  pub fn rtems_cbs_get_parameters(
    server_id: rtems_cbs_server_id,
    params: *mut rtems_cbs_parameters
  ) -> c_int;

  pub fn rtems_cbs_set_parameters(
    server_id: rtems_cbs_server_id,
    params: *mut rtems_cbs_parameters
  ) -> c_int;

  pub fn rtems_cbs_get_execution_time(
    server_id: rtems_cbs_server_id,
    exec_time: *mut time_t,
    abs_time: *mut time_t
  ) -> c_int;

  pub fn rtems_cbs_get_remaining_budget(
    server_id: rtems_cbs_server_id,
    remaining_budget: *mut time_t
  ) -> c_int;

  pub fn rtems_cbs_get_approved_budget(
    server_id: rtems_cbs_server_id,
    appr_budget: *mut time_t
  ) -> c_int;
}
