////////////////////////////////////////////////////////////////////////////////
// rtems api wrappers
////////////////////////////////////////////////////////////////////////////////

#![allow(non_camel_case_types)]

pub mod types;
pub mod initialize;
pub mod scheduler;
pub mod task;
pub mod interrupt;
pub mod clock;
pub mod timer;
pub mod ratemon;
pub mod semaphor;
pub mod barrier;
pub mod message;
pub mod event;
pub mod signal;
pub mod partition;
pub mod region;
pub mod port;
pub mod io;
pub mod cache;
pub mod error;

pub use crate::rtems::types::*;
pub use crate::rtems::initialize::*;
pub use crate::rtems::scheduler::*;
pub use crate::rtems::task::*;
pub use crate::rtems::interrupt::*;
pub use crate::rtems::clock::*;
pub use crate::rtems::timer::*;
pub use crate::rtems::ratemon::*;
pub use crate::rtems::semaphor::*;
pub use crate::rtems::barrier::*;
pub use crate::rtems::message::*;
pub use crate::rtems::event::*;
pub use crate::rtems::signal::*;
pub use crate::rtems::partition::*;
pub use crate::rtems::region::*;
pub use crate::rtems::port::*;
pub use crate::rtems::io::*;
pub use crate::rtems::cache::*;
pub use crate::rtems::error::*;
