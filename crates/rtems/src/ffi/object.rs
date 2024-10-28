////////////////////////////////////////////////////////////////////////////////
// object and id bookkeeping
////////////////////////////////////////////////////////////////////////////////

use crate::types::*;
use crate::chain::*;
use core::ffi::*;

#[repr(C)]
pub struct rtems_object_control {
  node: rtems_chain_node,
  id: rtems_id,
  name: rtems_name
}

#[repr(C)]
pub struct rtems_object_api_class_information {
  minimum_id: rtems_id,
  maximum_id: rtems_id,
  maximum: uint32_t,
  auto_extend: bool,
  unallocated: uint32_t
}

extern "C" {
  pub fn rtems_build_id(
    api: uint32_t,
    the_class: uint32_t,
    node: uint32_t,
    index: uint32_t
  ) -> rtems_id;

  pub fn rtems_build_name(
    c1: c_char,
    c2: c_char,
    c3: c_char,
    c4: c_char
  ) -> rtems_name;

  pub fn rtems_object_get_classic_name(
    id: rtems_id,
    name: *mut rtems_name
  ) -> rtems_status_code;

  pub fn rtems_object_get_name(
    id: rtems_id,
    length: size_t,
    name: *mut c_char
  ) -> *mut c_char;

  pub fn rtems_object_set_name(
    id: rtems_id,
    name: *const c_char
  ) -> rtems_status_code;

  pub fn rtems_object_id_get_api(
    id: rtems_id
  ) -> c_int;

  pub fn rtems_object_id_get_class(
    id: rtems_id
  ) -> c_int;

  pub fn rtems_object_id_get_node(
    id: rtems_id
  ) -> c_int;

  pub fn rtems_object_id_get_index(
    id: rtems_id
  ) -> c_int;

  pub fn rtems_object_id_api_minimum() -> c_int;

  pub fn rtems_object_id_api_maximum() -> c_int;

  pub fn rtems_object_api_minimum_class(
    api: c_int
  ) -> c_int;

  pub fn rtems_object_api_maximum_class(
    api: c_int
  ) -> c_int;

  pub fn rtems_object_get_api_name(
    api: c_int
  ) -> *const c_char;

  pub fn rtems_object_get_api_class_name(
    the_api: c_int,
    the_class: c_int
  ) -> *const c_char;

  pub fn rtems_object_get_class_information(
    the_api: c_int,
    the_class: c_int,
    info: *mut rtems_object_api_class_information
  ) -> ();

  pub fn rtems_object_get_local_node() -> uint16_t;
}
