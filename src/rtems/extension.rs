////////////////////////////////////////////////////////////////////////////////
// kernel extension support
////////////////////////////////////////////////////////////////////////////////

use crate::rtems::types::*;
use crate::rtems::error::*;
use core::ffi::*;

// TODO:
// i expect there are going to be some problems with handling driver function
// pointers like this. this interface in particular should be tested thoroughly

pub type rtems_extensions_thread_create_extension = unsafe extern "C" fn(
  *mut thread_control,
  *mut thread_control
) -> bool;

pub type rtems_extensions_thread_delete_extension = unsafe extern "C" fn(
  *mut thread_control,
  *mut thread_control
) -> ();

pub type rtems_extensions_thread_start_extension = unsafe extern "C" fn(
  *mut thread_control,
  *mut thread_control
) -> ();

pub type rtems_extensions_thread_restart_extension = unsafe extern "C" fn(
  *mut thread_control,
  *mut thread_control
) -> ();

pub type rtems_extensions_thread_switch_extension = unsafe extern "C" fn(
  *mut thread_control,
  *mut thread_control
) -> ();

pub type rtems_extensions_thread_begin_extension = unsafe extern "C" fn(
  *mut thread_control
) -> ();

pub type rtems_extensions_thread_exitted_extension = unsafe extern "C" fn(
  *mut thread_control
) -> ();

pub type rtems_extensions_fatal_extension = unsafe extern "C" fn(
  rtems_fatal_source,
  bool,
  rtems_fatal_code
) -> ();

pub type rtems_extensions_thread_terminate_extension = unsafe extern "C" fn(
  *mut thread_control
) -> ();

#[repr(C)]
pub struct rtems_extensions_table {
  thread_create: rtems_extensions_thread_create_extension,
  thread_start: rtems_extensions_thread_start_extension,
  thread_restart: rtems_extensions_thread_restart_extension,
  thread_delete: rtems_extensions_thread_delete_extension,
  thread_switch: rtems_extensions_thread_switch_extension,
  thread_begin: rtems_extensions_thread_begin_extension,
  thread_exitted: rtems_extensions_thread_exitted_extension,
  fatal: rtems_extensions_fatal_extension,
  thread_terminate: rtems_extensions_thread_terminate_extension,
}

extern "C" {
  pub fn rtems_extension_create(
    name: rtems_name,
    extension_table: *const rtems_extensions_table,
    id: rtems_id
  ) -> rtems_status_code;

  pub fn rtems_extension_delete(
    id: rtems_id
  ) -> rtems_status_code;

  pub fn rtems_extension_ident(
    name: rtems_name,
    id: *mut rtems_id
  ) -> rtems_status_code;
}
