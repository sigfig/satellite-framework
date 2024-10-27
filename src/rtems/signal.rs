////////////////////////////////////////////////////////////////////////////////
// asynchronous task signaling
////////////////////////////////////////////////////////////////////////////////

use crate::rtems::types::*;

pub type rtems_signal_set = uint32_t;

pub type rtems_asr_entry = unsafe extern "C" fn(
  rtems_signal_set
) -> ();

extern "C" {
  pub fn rtems_signal_catch(
    asr_handler: rtems_asr_entry,
    mode_set: rtems_mode
  ) -> rtems_status_code;

  pub fn rtems_signal_send(
    id: rtems_id,
    signal_set: rtems_signal_set
  ) -> rtems_status_code;
}
