////////////////////////////////////////////////////////////////////////////////
// task event set management
////////////////////////////////////////////////////////////////////////////////

use crate::rtems::types::*;

pub type rtems_event_set = uint32_t;

extern "C" {
  pub fn rtems_event_send(
    id: rtems_id,
    event_in: rtems_event_set
  ) -> rtems_status_code;

  pub fn rtems_event_receive(
    event_in: rtems_event_set,
    option_set: rtems_option,
    ticks: rtems_interval,
    event_out: *mut rtems_event_set
  ) -> rtems_status_code;
}
