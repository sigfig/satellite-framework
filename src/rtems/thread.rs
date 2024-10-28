////////////////////////////////////////////////////////////////////////////////
// thread api plus synchronization primitives
////////////////////////////////////////////////////////////////////////////////

use crate::rtems::types::*;
use core::ffi::*;

// NOTE:
// this file follows the organization of the rtems user manual
// the synchronization objects should probably be placed elsewhere in the future

pub type rtems_mutex;
pub type rtems_recursive_mutex;
pub type rtems_condition_variable;
pub type rtems_counting_semaphore;
pub type rtems_binary_semaphore;
pub type rtems_thread;

extern "C" {
  pub fn rtems_mutex_init(
    mutex: *mut rtems_mutex,
    name: *const c_char
  ) -> ();

  pub fn rtems_recursive_mutex_init(
    mutex: *mut rtems_recursive_mutex,
    name: *const c_char
  ) -> ();

  pub fn rtems_mutex_lock(
    mutex: *mut rtems_mutex
  ) -> ();

  pub fn rtems_recursive_mutex_lock(
    mutex: *mut rtems_recursive_mutex
  ) -> ();

  pub fn rtems_mutex_try_lock(
    mutex: *mut rtems_mutex
  ) -> c_int;

  pub fn rtems_recursive_mutex_try_lock(
    mutex: *mut rtems_recursive_mutex
  ) -> c_int;

  pub fn rtems_mutex_unlock(
    mutex: *mut rtems_mutex
  ) -> ();

  pub fn rtems_recursive_mutex_unlock(
    mutex: *mut rtems_recursive_mutex
  ) -> ();

  pub fn rtems_mutex_set_name(
    mutex: *mut rtems_mutex,
    name: *const c_char
  ) -> ();

  pub fn rtems_recursive_mutex_set_name(
    mutex: *mut rtems_recursive_mutex,
    name: *const c_char
  ) -> ();

  pub fn rtems_mutex_get_name(
    mutex: *mut rtems_mutex
  ) -> *const c_char;

  pub fn rtems_recursive_mutex_get_name(
    mutex: *const rtems_recursive_mutex
  ) -> *const c_char;

  pub fn rtems_mutex_destroy(
    mutex: *mut rtems_mutex
  ) -> ();

  pub fn rtems_recursive_mutex_destroy(
    mutex: *mut rtems_recursive_mutex
  ) -> ();

  pub fn rtems_condition_variable_init(
    condition_variable: *mut rtems_condition_variable,
    name: *const c_char
  ) -> ();

  pub fn rtems_condition_variable_wait(
    condition_variable: *mut rtems_condition_variable,
    mutex: *mut rtems_mutex
  ) -> ();

  pub fn rtems_condition_variable_signal(
    condition_variable: *mut rtems_condition_variable
  ) -> ();

  pub fn rtems_condition_variable_broadcast(
    condition_variable: *mut rtems_condition_variable
  ) -> ();

  pub fn rtems_condition_variable_set_name(
    condition_variable: *mut rtems_condition_variable,
    name: *const c_char
  ) -> ();

  pub fn rtems_condition_variable_get_name(
    condition_variable: *const rtems_condition_variable
  ) -> *const c_char;

  pub fn rtems_condition_variable_destroy(
    condition_variable: *mut rtems_condition_variable
  ) -> ();

  pub fn rtems_counting_semaphore_init(
    counting_semaphore: *mut rtems_counting_semaphore,
    name: *const c_char,
    value: c_uint
  ) -> ();

  pub fn rtems_counting_semaphore_wait(
    counting_semaphore: *mut rtems_counting_semaphore
  ) -> ();

  pub fn rtems_counting_semaphore_wait_timed_ticks(
    counting_semaphore: *mut rtems_counting_semaphore,
    ticks: uint32_t
  ) -> ();

  pub fn rtems_counting_semaphore_try_wait(
    counting_semaphore: *mut rtems_counting_semaphore
  ) -> ();

  pub fn rtems_counting_semaphore_post(
    counting_semaphore: *mut rtems_counting_semaphore
  ) -> ();

  pub fn rtems_counting_semaphore_set_name(
    counting_semaphore: *mut rtems_counting_semaphore,
    name: *const c_char,
  ) -> ();

  pub fn rtems_counting_semaphore_get_name(
    counting_semaphore: *const rtems_counting_semaphore
  ) -> *const c_char;

  pub fn rtems_counting_semaphore_destroy(
    counting_semaphore: *mut rtems_counting_semaphore
  ) -> ();

  pub fn rtems_binary_semaphore_init(
    binary_semaphore: *mut rtems_binary_semaphore,
    name: *const c_char,
    value: c_uint
  ) -> ();

  pub fn rtems_binary_semaphore_wait(
    binary_semaphore: *mut rtems_binary_semaphore
  ) -> ();

  pub fn rtems_binary_semaphore_wait_timed_ticks(
    binary_semaphore: *mut rtems_binary_semaphore,
    ticks: uint32_t
  ) -> ();

  pub fn rtems_binary_semaphore_try_wait(
    binary_semaphore: *mut rtems_binary_semaphore
  ) -> ();

  pub fn rtems_binary_semaphore_post(
    binary_semaphore: *mut rtems_binary_semaphore
  ) -> ();

  pub fn rtems_binary_semaphore_set_name(
    binary_semaphore: *mut rtems_binary_semaphore,
    name: *const c_char,
  ) -> ();

  pub fn rtems_binary_semaphore_get_name(
    binary_semaphore: *const rtems_binary_semaphore
  ) -> *const c_char;

  pub fn rtems_binary_semaphore_destroy(
    binary_semaphore: *mut rtems_binary_semaphore
  ) -> ();

  // NOTE:
  // the rtems docs declare that this thread api is still in testing
  // it may not even be in rtems 6, i haven't checked

  pub fn rtems_thread_start(
    thread: *mut rtems_thread,
    name: *const c_char,
    thread_size: size_t,
    priority: uint32_t,
    flags: uint32_t,
    entry: unsafe extern "C" fn(*mut c_void) -> (),
    arg: *mut c_void
  ) -> ();

  pub fn rtems_thread_restart(
    thread: *mut rtems_thread,
    arg: *mut c_void
  ) -> ();

  pub fn rtems_thread_event_send(
    thread: *mut rtems_thread,
    events: uint32_t
  ) -> ();

  pub fn rtems_thread_event_poll(
    thread: *mut rtems_thread,
    events_of_interest: uint32_t
  ) -> ();

  pub fn rtems_thread_event_wait_all(
    thread: *mut rtems_thread,
    events_of_interest: uint32_t
  ) -> ();

  pub fn rtems_thread_event_wait_any(
    thread: *mut rtems_thread,
    events_of_interest: uint32_t
  ) -> ();

  pub fn rtems_thread_destroy(
    thread: *mut rtems_thread
  ) -> ();

  pub fn rtems_thread_destroy_self() -> !;
}
