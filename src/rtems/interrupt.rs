////////////////////////////////////////////////////////////////////////////////
// interrupt manager

use crate::rtems::types::*;
use crate::rtems::task::*;
use core::ffi::*;

pub type rtems_isr_entry = unsafe extern "C" fn(uint32_t) -> ();
pub type rtems_vector_number = uint32_t;
pub type rtems_interrupt_level = uint32_t;

// this is either a char or a struct depending on rtems config
// rtems/cpukit/include/rtems/rtems/intr.h#529
// rtems/cpukit/include/rtems/score/isrlock.h#86
//
#[repr(C)]
pub struct rtems_interrupt_lock {
  placeholder: uint32_t
}

// this also varies by configuration
// rtems/cpukit/include/rtems/score/isrlock.h#94
#[repr(C)]
pub struct rtems_interrupt_lock_context {
  placeholder: uint32_t
}

pub type rtems_interrupt_handler = unsafe extern "C" fn(*mut c_void) -> ();

#[repr(C)]
pub struct rtems_interrupt_entry {
  handler: rtems_interrupt_handler,
  arg: *mut c_void,
  next: *mut rtems_interrupt_entry,
  info: *const c_char
}

// this is an enum
// rtems/cpukit/include/rtems/rtems/intr.h#2028
pub type rtems_interrupt_signal_variant = uint32_t;

#[repr(C)]
pub struct rtems_interrupt_attributes {
  is_maskable: bool,
  can_enable: bool,
  maybe_enable: bool,
  can_disable: bool,
  maybe_disable: bool,
  can_raise: bool,
  can_raise_on: bool,
  can_clear: bool,
  cleared_by_acknowledge: bool,
  can_get_affinity: bool,
  can_be_triggered_by_message: bool,
  trigger_signal: rtems_interrupt_signal_variant,
  can_get_priority: bool,
  can_set_priority: bool,
  maximum_priority: uint32_t,
}

pub type rtems_interrupt_per_handler_routine = unsafe extern "C" fn(
  *mut c_void,
  *const c_char,
  rtems_option,
  rtems_interrupt_handler,
  *mut c_void
) -> ();

#[repr(C)]
pub struct rtems_interrupt_server_config {
  name: rtems_name,
  priority: rtems_task_priority,
  storage_area: *mut c_void,
  storage_size: size_t,
  modes: rtems_mode,
  attributes: rtems_attribute,
  destroy: unsafe extern "C" fn(*mut rtems_interrupt_server_control) -> ()
}

// varies depending on rtems configuration
// rtems/cpukit/include/rtems/rtems/intr.h#2337
#[repr(C)]
pub struct rtems_interrupt_server_control {
  lock: rtems_interrupt_lock,
  entries: chain_control,
  server: rtems_id,
  errors: uint32_t,
  index: uint32_t,
  node: chain_node,
  destroy: unsafe extern "C" fn(*mut rtems_interrupt_server_control) -> ()
}

#[repr(C)]
pub struct rtems_interrupt_server_action {
  next: *mut rtems_interrupt_server_action,
  handler: rtems_interrupt_handler,
  arg: *mut c_void
}

#[repr(C)]
pub struct rtems_interrupt_server_entry {
  node: chain_node,
  server: *mut rtems_interrupt_server_control,
  vector: rtems_vector_number,
  actions: *mut rtems_interrupt_server_action
}

#[repr(C)]
pub struct rtems_interrupt_server_request {
  entry: rtems_interrupt_server_entry,
  action: rtems_interrupt_server_action
}

extern "C" {
  pub fn rtems_interrupt_catch(
    new_isr_handler: rtems_isr_entry,
    vector: rtems_vector_number,
    old_isr_handler: *mut rtems_isr_entry
  ) -> rtems_status_code;

  pub fn rtems_interrupt_disable(
    isr_cookie: rtems_interrupt_level
  ) -> ();

  pub fn rtems_interrupt_enable(
    isr_cookie: rtems_interrupt_level
  ) -> ();

  pub fn rtems_interrupt_flash(
    isr_cookie: rtems_interrupt_level
  ) -> ();

  pub fn rtems_interrupt_local_disable(
    isr_cookie: rtems_interrupt_level
  ) -> ();

  pub fn rtems_interrupt_local_enable(
    isr_cookie: rtems_interrupt_level
  ) -> ();

  pub fn rtems_interrupt_is_in_progress() -> bool;

  pub fn rtems_interrupt_lock_initialize(
    lock: *mut rtems_interrupt_lock,
    name: *const c_char
  ) -> rtems_status_code;

  pub fn rtems_interrupt_lock_destroy(
    lock: *mut rtems_interrupt_lock
  ) -> ();

  pub fn rtems_interrupt_lock_acquire(
    lock: *mut rtems_interrupt_lock,
    lock_context: *mut rtems_interrupt_lock_context
  ) -> ();

  pub fn rtems_interrupt_lock_release(
    lock: *mut rtems_interrupt_lock_context
  ) -> ();

  pub fn rtems_interrupt_lock_acquire_isr(
    lock: *mut rtems_interrupt_lock,
    lock_context: *mut rtems_interrupt_lock_context
  ) -> ();

  pub fn rtems_interrupt_lock_release_isr(
    lock: *mut rtems_interrupt_lock,
    lock_context: *mut rtems_interrupt_lock_context
  ) -> ();

  pub fn rtems_interrupt_lock_interrupt_disable(
    lock_context: *mut rtems_interrupt_lock_context
  ) -> ();

  pub fn rtems_interrupt_entry_initialize(
    entry: *mut rtems_interrupt_entry,
    routine: rtems_interrupt_handler,
    arg: *mut c_void,
    info: *const c_char
  ) -> ();

  pub fn rtems_interrupt_entry_install(
    vector: rtems_vector_number,
    options: rtems_option,
    entry: *mut rtems_interrupt_entry
  ) -> rtems_status_code;

  pub fn rtems_interrupt_entry_remote(
    vector: rtems_vector_number,
    entry: *mut rtems_interrupt_entry
  ) -> rtems_status_code;

  pub fn rtems_interrupt_handler_install(
    vector: rtems_vector_number,
    info: *const c_char,
    options: rtems_option,
    routine: rtems_interrupt_handler,
    arg: *mut c_void
  ) -> rtems_status_code;

  pub fn rtems_interrupt_handler_remove(
    vector: rtems_vector_number,
    routine: rtems_interrupt_handler,
    arg: *mut c_void
  ) -> rtems_status_code;

  pub fn rtems_interrupt_vector_is_enabled(
    vector: rtems_vector_number,
    enabled: *mut bool
  ) -> rtems_status_code;

  pub fn rtems_interrupt_vector_enable(
    vector: rtems_vector_number
  ) -> rtems_status_code;

  pub fn rtems_interrupt_vector_disable(
    vector: rtems_vector_number
  ) -> rtems_status_code;

  pub fn rtems_interrupt_is_pending(
    vector: rtems_vector_number,
    pending: *mut bool
  ) -> rtems_status_code;

  pub fn rtems_interrupt_raise(
    vector: rtems_vector_number
  ) -> rtems_status_code;

  pub fn rtems_interrupt_raise_on(
    vector: rtems_vector_number,
    cpu_index: uint32_t
  ) -> rtems_status_code;

  pub fn rtems_interrupt_clear(
    vector: rtems_vector_number
  ) -> rtems_status_code;

  pub fn rtems_interrupt_get_priority(
    vector: rtems_vector_number,
    priority: *mut uint32_t
  ) -> rtems_status_code;

  pub fn rtems_interrupt_set_priority(
    vector: rtems_vector_number,
    priority: uint32_t
  ) -> rtems_status_code;

  pub fn rtems_interrupt_get_affinity(
    vector: rtems_vector_number,
    affinity_size: size_t,
    affinity: *mut cpu_set_t
  ) -> rtems_status_code;

  pub fn rtems_interrupt_set_affinity(
    vector: rtems_vector_number,
    affinity_size: size_t,
    affinity: *const cpu_set_t
  ) -> rtems_status_code;

  pub fn rtems_interrupt_get_attributes(
    vector: rtems_vector_number,
    attributes: *mut rtems_interrupt_attributes
  ) -> rtems_status_code;


  pub fn rtems_interrupt_handler_iterate(
    vector: rtems_vector_number,
    routine: rtems_interrupt_per_handler_routine,
    arg: *mut c_void
  ) -> rtems_status_code;

  pub fn rtems_interrupt_server_initialize(
    priority: rtems_task_priority,
    stack_size: size_t,
    modes: rtems_mode,
    attributes: rtems_attribute,
    server_count: *mut uint32_t
  ) -> rtems_status_code;

  pub fn rtems_interrupt_server_create(
    control: *mut rtems_interrupt_server_control,
    config: *const rtems_interrupt_server_config,
    server_index: *mut uint32_t
  ) -> rtems_status_code;

  pub fn rtems_interrupt_server_handler_install(
    server_index: uint32_t,
    vector: rtems_vector_number,
    info: *const c_char,
    options: rtems_option,
    routine: rtems_interrupt_handler,
    arg: *mut c_void
  ) -> rtems_status_code;

  pub fn rtems_interrupt_server_handler_remove(
    server_index: uint32_t,
    vector: rtems_vector_number,
    routine: rtems_interrupt_handler,
    arg: *mut c_void
  ) -> rtems_status_code;

  pub fn rtems_interrupt_server_set_affinity(
    server_index: uint32_t,
    affinity_size: size_t,
    affinity: *const cpu_set_t,
    priority: rtems_task_priority
  ) -> rtems_status_code;

  pub fn rtems_interrupt_server_delete(
    server_index: uint32_t
  ) -> rtems_status_code;

  pub fn rtems_interrupt_server_suspend(
    server_index: uint32_t
  ) -> rtems_status_code;

  pub fn rtems_interrupt_server_resume(
    server_index: uint32_t
  ) -> rtems_status_code;

  pub fn rtems_interrupt_server_move(
    source_server_index: uint32_t,
    vector: rtems_vector_number,
    destination_server_index: uint32_t
  ) -> rtems_status_code;

  pub fn rtems_interrupt_server_handler_iterate(
    server_index: uint32_t,
    vector: rtems_vector_number,
    routine: rtems_interrupt_per_handler_routine,
    arg: *mut c_void
  ) -> rtems_status_code;

  pub fn rtems_interrupt_server_entry_initialize(
    server_index: uint32_t,
    entry: *mut rtems_interrupt_server_entry
  ) -> rtems_status_code;

  pub fn rtems_interrupt_server_action_prepend(
    entry: *mut rtems_interrupt_server_entry,
    action: *mut rtems_interrupt_server_action,
    routine: rtems_interrupt_handler,
    arg: *mut c_void
  ) -> rtems_status_code;

  pub fn rtems_interrupt_server_entry_destroy(
    entry: *mut rtems_interrupt_server_entry
  ) -> rtems_status_code;

  pub fn rtems_interrupt_server_entry_submit(
    entry: *mut rtems_interrupt_server_entry
  ) -> rtems_status_code;

  pub fn rtems_interrupt_server_entry_move(
    entry: *mut rtems_interrupt_server_entry,
    server_index: uint32_t
  ) -> rtems_status_code;

  pub fn rtems_interrupt_server_request_initialize(
    server_index: uint32_t,
    request: *mut rtems_interrupt_server_request,
    routine: rtems_interrupt_handler,
    arg: *mut c_void
  ) -> rtems_status_code;

  pub fn rtems_interrupt_server_request_set_vector(
    request: *mut rtems_interrupt_server_request,
    vector: rtems_vector_number
  ) -> rtems_status_code;

  pub fn rtems_interrupt_server_request_destroy(
    request: *mut rtems_interrupt_server_request
  ) -> rtems_status_code;

  pub fn rtems_interrupt_server_request_submit(
    request: *mut rtems_interrupt_server_request
  ) -> rtems_status_code;
}
