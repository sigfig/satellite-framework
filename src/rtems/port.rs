////////////////////////////////////////////////////////////////////////////////
// dual-port memory address translation
////////////////////////////////////////////////////////////////////////////////

use crate::rtems::types::*;
use core::ffi::*;

extern "C" {
  pub fn rtems_port_create(
    name: rtems_name,
    internal_start: *mut c_void,
    external_start: *mut c_void,
    length: uint32_t,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_port_ident(
    name: rtems_name,
    id: *mut rtems_id
  ) -> rtems_status_code;

  pub fn rtems_port_delete(
    id: rtems_id
  ) -> rtems_status_code;

  pub fn rtems_port_external_to_internal(
    id: rtems_id,
    external: *mut c_void,
    internal: *mut *mut c_void
  ) -> rtems_status_code;

  pub fn rtems_port_internal_to_external(
    id: rtems_id,
    external: *mut c_void,
    internal: *mut *mut c_void
  ) -> rtems_status_code;
}
