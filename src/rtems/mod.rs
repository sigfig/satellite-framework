////////////////////////////////////////////////////////////////////////////////
// rtems api wrappers
////////////////////////////////////////////////////////////////////////////////

#![allow(non_camel_case_types)]

use core::ffi::*;

// TODO: rtems values and structs
// these need to be given semantic mappings to their enum values and structs
// however, we might want to abstract a little bit for the sake of rust apis
// we should wait to see how the api shakes out before doing anything here
pub mod types;
pub use crate::rtems::types::*;

////////////////////////////////////////////////////////////////////////////////
// iniitialization manager

extern "C" {
  pub fn rtems_initialize_executive() -> ();
}

pub mod scheduler;
pub use crate::rtems::scheduler::*;

pub mod task;
pub use crate::rtems::task::*;

pub mod interrupt;
pub use crate::rtems::interrupt::*;

pub mod clock;
pub use crate::rtems::clock::*;

pub mod timer;
pub use crate::rtems::timer::*;

pub mod ratemon;
pub use crate::rtems::ratemon::*;

pub mod io;
pub use crate::rtems::io::*;

////////////////////////////////////////////////////////////////////////////////
// error manager

extern "C" {
  pub fn rtems_panic(fmt: *const c_char, ...) -> !;
  pub fn rtems_shutdown_executive(fatal_code: uint32_t);
}
