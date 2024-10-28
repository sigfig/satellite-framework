////////////////////////////////////////////////////////////////////////////////
// autobalancing red-black tree datastructure
////////////////////////////////////////////////////////////////////////////////

use crate::rtems::types::*;
use core::ffi::*;

#[repr(C)]
pub struct rtems_rbtree_node {
  left: *mut rtems_rbtree_node,
  right: *mut rtems_rbtree_node,
  parent: *mut rtems_rbtree_node
}

#[repr(C)]
pub struct rtems_rbtree_control {
  root: *mut rtems_rbtree_node
}

pub type rtems_rbtree_compare_result = c_long;

pub type rtems_rbtree_compare = unsafe extern "C" fn(
  *const rtems_rbtree_node,
  *const rtems_rbtree_node
) -> rtems_rbtree_compare_result;

extern "C" {
  pub fn rtems_rbtree_initialize(
    the_rbtree: *mut rtems_rbtree_control,
    compare: rtems_rbtree_compare,
    starting_address: *mut c_void,
    number_nodes: size_t,
    node_size: size_t,
    is_unique: bool
  ) -> ();

  pub fn rtems_rbtree_initialize_empty(
    the_rbtree: *mut rtems_rbtree_control
  ) -> ();

  pub fn rtems_rbtree_set_off_tree(
    node: *mut rtems_rbtree_node
  ) -> ();

  pub fn rtems_rbtree_is_node_off_tree(
    node: *const rtems_rbtree_node
  ) -> bool;

  pub fn rtems_rbtree_root(
    the_rbtree: *const rtems_rbtree_control
  ) -> *mut rtems_rbtree_node;

  pub fn rtems_rbtree_min(
    the_rbtree: *const rtems_rbtree_control
  ) -> *mut rtems_rbtree_node;

  pub fn rtems_rbtree_max(
    the_rbtree: *const rtems_rbtree_control
  ) -> *mut rtems_rbtree_node;

  pub fn rtems_rbtree_left(
    the_node: *const rtems_rbtree_node
  ) -> *mut rtems_rbtree_node;

  pub fn rtems_rbtree_right(
    the_node: *const rtems_rbtree_node
  ) -> *mut rtems_rbtree_node;

  pub fn rtems_rbtree_parent(
    the_node: *const rtems_rbtree_node
  ) -> *mut rtems_rbtree_node;

  pub fn rtems_rbtree_is_empty(
    the_rbtree: *const rtems_rbtree_control
  ) -> bool;

  pub fn rtems_rbtree_is_min(
    the_rbtree: *const rtems_rbtree_control,
    the_node: *const rtems_rbtree_node
  ) -> bool;

  pub fn rtems_rbtree_is_max(
    the_rbtree: *const rtems_rbtree_control,
    the_node: *const rtems_rbtree_node
  ) -> bool;

  pub fn rtems_rbtree_is_root(
    the_node: *const rtems_rbtree_node
  ) -> bool;

  pub fn rtems_rbtree_is_equal(
    compare_result: rtems_rbtree_compare_result
  ) -> bool;

  pub fn rtems_rbtree_is_greater(
    compare_result: rtems_rbtree_compare_result
  ) -> bool;

  pub fn rtems_rbtree_is_lesser(
    compare_result: rtems_rbtree_compare_result
  ) -> bool;

  pub fn rtems_rbtree_find(
    the_rbtree: *const rtems_rbtree_control,
    the_node: *const rtems_rbtree_node,
    compare: rtems_rbtree_compare,
    is_unique: bool
  ) -> *mut rtems_rbtree_node;

  pub fn rtems_rbtree_predecessor(
    the_node: *const rtems_rbtree_node
  ) -> *mut rtems_rbtree_node;

  pub fn rtems_rbtree_successor(
    the_node: *const rtems_rbtree_node
  ) -> *mut rtems_rbtree_node;

  pub fn rtems_rbtree_extract(
    the_rbtree: *mut rtems_rbtree_control,
    the_node: *mut rtems_rbtree_node
  ) -> ();

  pub fn rtems_rbtree_get_min(
    the_rbtree: *mut rtems_rbtree_control
  ) -> *mut rtems_rbtree_node;

  pub fn rtems_rbtree_get_max(
    the_rbtree: *mut rtems_rbtree_control
  ) -> *mut rtems_rbtree_node;

  pub fn rtems_rbtree_peek_min(
    the_rbtree: *const rtems_rbtree_control
  ) -> *mut rtems_rbtree_node;

  pub fn rtems_rbtree_peek_max(
    the_rbtree: *const rtems_rbtree_control
  ) -> *mut rtems_rbtree_node;

  pub fn rtems_rbtree_insert(
    the_rbtree: *mut rtems_rbtree_control,
    the_node: *mut rtems_rbtree_node,
    compare: rtems_rbtree_compare,
    is_unique: bool
  ) -> ();
}
