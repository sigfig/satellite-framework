////////////////////////////////////////////////////////////////////////////////
// temporary executable entry point for testing
////////////////////////////////////////////////////////////////////////////////

use rtems::*;
use crate::io::*;

use core::primitive::char;
use core::result::Result;
use core::fmt::Error;
use core::fmt::Write;

////////////////////////////////////////////////////////////////////////////////
// rtems application entry point

#[no_mangle]
pub extern "C" fn Init() {
    if let Err(e) = ticker_main() {
        panic!("Main returned {:?}", e);
    }
    unsafe { rtems_shutdown_executive(0); }
}

////////////////////////////////////////////////////////////////////////////////
// hello world example

fn rust_main() -> Result<(), Error> {
    let mut console = Console;
    writeln!(console, "Hello from Rust")?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////
// ported ticker example using unsafe api

use core::ffi::*;

// TODO: can immediately remove the global state to make this work
static mut task_ids: [rtems_id; 4] = [0,0,0,0];
static mut task_names: [rtems_name; 4] = [0,0,0,0];

static RTEMS_MINIMUM_STACK_SIZE : uint32_t = 1024 * 4;
static RTEMS_DEFAULT_MODES : rtems_mode = 0;
static RTEMS_DEFAULT_ATTRIBUTES : rtems_attribute = 0;
static RTEMS_SELF : rtems_id = 0;

fn empty_tod() -> rtems_time_of_day {
  rtems_time_of_day {
    year: 0,
    month: 0,
    day: 0,
    hour: 0,
    minute: 0,
    second: 0,
    ticks: 0
  }
}

fn print_name(name: rtems_name) -> () {
  // TODO: this imaginary io scoping is bad
  let mut console = Console;
  let c1 = char::from_u32(name & 255).unwrap();
  let c2 = char::from_u32(name & (255 << 8)).unwrap();
  let c3 = char::from_u32(name & (255 << 16)).unwrap();
  let c4 = char::from_u32(name & (255 << 24)).unwrap();
  writeln!(console, "{}{}{}{}", c1, c2, c3, c4);
}

fn print_time(time: rtems_time_of_day) -> () {
  let mut console = Console;
  writeln!(
    console,
    "{}/{}/{} {}:{}:{}.{}",
    time.month,
    time.day,
    time.year,
    time.hour,
    time.minute,
    time.second,
    time.ticks
  );
}

pub unsafe extern "C" fn ticker_task(arg: *mut c_void) -> () {
  let task_index = *(arg as *mut uint32_t) as usize;
  let mut time : rtems_time_of_day = empty_tod();
  let ticks = task_index * 5 * (rtems_clock_get_ticks_per_second() as usize);
  let mut console = Console;

  loop {
    let _ = rtems_clock_get_tod(&mut time);

    if time.second >= 35 {
      writeln!(console, "*** end of clock test ***");
      return;
    }

    print_name(task_names[task_index]);
    print_time(time);
    let _  = rtems_task_wake_after(ticks.try_into().unwrap());
  }
}

fn ticker_main() -> Result<(), Error> {
  unsafe {
    let mut time = empty_tod();

    time.year = 1988;
    time.month = 12;
    time.day = 31;
    time.hour = 9;
    time.minute = 0;
    time.second = 0;
    time.ticks = 0;

    let _ = rtems_clock_set(&time);

    task_names[1] = rtems_build_name(
      't' as c_char,
      '1' as c_char,
      ' ' as c_char,
      ' ' as c_char
    );

    task_names[2] = rtems_build_name(
      't' as c_char,
      '2' as c_char,
      ' ' as c_char,
      ' ' as c_char
    );

    task_names[3] = rtems_build_name(
      't' as c_char,
      '3' as c_char,
      ' ' as c_char,
      ' ' as c_char
    );

    let mut _arg1 = 1;
    let mut _arg2 = 2;
    let mut _arg3 = 3;

    let _ = rtems_task_create(
      task_names[1],
      1,
      RTEMS_MINIMUM_STACK_SIZE * 2,
      RTEMS_DEFAULT_MODES,
      RTEMS_DEFAULT_ATTRIBUTES,
      &mut task_ids[1]
    );

    let _ = rtems_task_create(
      task_names[2],
      1,
      RTEMS_MINIMUM_STACK_SIZE * 2,
      RTEMS_DEFAULT_MODES,
      RTEMS_DEFAULT_ATTRIBUTES,
      &mut task_ids[2]
    );

    let _ = rtems_task_create(
      task_names[3],
      1,
      RTEMS_MINIMUM_STACK_SIZE * 2,
      RTEMS_DEFAULT_MODES,
      RTEMS_DEFAULT_ATTRIBUTES,
      &mut task_ids[3]
    );

    // NOTE:
    // task closures are fn(*mut c_void) -> ()
    let _ = rtems_task_start(
      task_ids[1],
      ticker_task,
      &mut _arg1 as *mut _ as *mut c_void
    );

    let _ = rtems_task_start(
      task_ids[2],
      ticker_task,
      &mut _arg2 as *mut _ as *mut c_void
    );

    let _ = rtems_task_start(
      task_ids[3],
      ticker_task,
      &mut _arg2 as *mut _ as *mut c_void
    );

    let _ = rtems_task_delete(RTEMS_SELF);
  }
  Ok(())
}
