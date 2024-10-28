////////////////////////////////////////////////////////////////////////////////
// anchored doubly-linked list
////////////////////////////////////////////////////////////////////////////////

use crate::rtems::types::*;
use core::ffi::*;
use core::mem::ManuallyDrop;

// this is a doubly-linked list structure used in rtems
// it is sneakily generic
// it is cast to a larger struct type that has chain_node as its first element
// not sure how much it gets used in application code
// rtems/cpukit/include/rtems/score/chain.h#78
#[repr(C)]
pub struct rtems_chain_node {
  next: *mut rtems_chain_node,
  previous: *mut rtems_chain_node
}

#[repr(C)]
pub struct chain_head {
  node: rtems_chain_node,
  fill: *mut rtems_chain_node
}

#[repr(C)]
pub struct chain_tail {
  fill: *mut rtems_chain_node,
  node: rtems_chain_node
}

#[repr(C)]
pub union rtems_chain_control {
  head: ManuallyDrop<chain_head>,
  tail: ManuallyDrop<chain_tail>
}

extern "C" {
  pub fn rtems_chain_initialize(
    the_chain: *mut rtems_chain_control,
    starting_address: *mut c_void,
    number_nodes: size_t,
    node_size: size_t
  ) -> ();

  pub fn rtems_chain_initialize_empty(
    the_chain: *mut rtems_chain_control
  ) -> ();

  pub fn rtems_chain_is_null_node(
    the_node: *const rtems_chain_node
  ) -> bool;

  pub fn rtems_chain_head(
    the_chain: *mut rtems_chain_control
  ) -> *mut rtems_chain_node;

  pub fn rtems_chain_tail(
    the_chain: *mut rtems_chain_control
  ) -> *mut rtems_chain_node;

  pub fn rtems_chain_equal(
    left: *const rtems_chain_node,
    right: *const rtems_chain_node
  ) -> bool;

  pub fn rtems_chain_is_empty(
    the_chain: *mut rtems_chain_control
  ) -> bool;

  pub fn rtems_chain_is_first(
    the_node: *const rtems_chain_node
  ) -> bool;

  pub fn rtems_chain_is_last(
    the_node: *const rtems_chain_node
  ) -> bool;

  pub fn rtems_chain_has_only_one_node(
    the_chain: *const rtems_chain_control
  ) -> bool;

  pub fn rtems_chain_count_unprotected(
    the_chain: *const rtems_chain_control
  ) -> size_t;

  pub fn rtems_chain_is_head(
    the_chain: *const rtems_chain_control,
    the_node: *const rtems_chain_node
  ) -> bool;

  pub fn rtems_chain_is_tail(
    the_chain: *const rtems_chain_control,
    the_node: *const rtems_chain_node
  ) -> bool;

  pub fn rtems_chain_extract(
    the_node: *mut rtems_chain_node
  ) -> ();

  pub fn rtems_chain_extract_unprotected(
    the_node: *mut rtems_chain_node
  ) -> ();

  pub fn rtems_chain_get(
    the_chain: *mut rtems_chain_control
  ) -> *mut rtems_chain_node;

  pub fn rtems_chain_get_unprotected(
    the_chain: *mut rtems_chain_control
  ) -> *mut rtems_chain_node;

  pub fn rtems_chain_insert(
    after_node: *mut rtems_chain_node,
    the_node: *mut rtems_chain_node
  ) -> ();

  pub fn rtems_chain_insert_unprotected(
    after_node: *mut rtems_chain_node,
    the_node: *mut rtems_chain_node
  ) -> ();

  pub fn rtems_chain_append(
    the_chain: *const rtems_chain_control,
    the_node: *const rtems_chain_node
  ) -> ();

  pub fn rtems_chain_append_unprotected(
    the_chain: *const rtems_chain_control,
    the_node: *const rtems_chain_node
  ) -> ();

  pub fn rtems_chain_prepend(
    the_chain: *const rtems_chain_control,
    the_node: *const rtems_chain_node
  ) -> ();

  pub fn rtems_chain_prepend_unprotected(
    the_chain: *const rtems_chain_control,
    the_node: *const rtems_chain_node
  ) -> ();
}
