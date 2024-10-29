////////////////////////////////////////////////////////////////////////////////
// class rtems ticker example
////////////////////////////////////////////////////////////////////////////////

use rtems::*;
use crate::io::*;

use core::primitive::char;
use core::result::Result;
use core::fmt::Error;
use core::fmt::Write;
use core::mem::size_of;
use core::ffi::*;

////////////////////////////////////////////////////////////////////////////////
// adapted ticker example using the unsafe ffi

// TODO: can immediately remove the global state to make this work
// NOTE: i'm using ids[0] for the queue
static mut task_ids: [rtems_id; 4] = [0,0,0,0];
static mut task_names: [rtems_name; 4] = [0,0,0,0];
static mut task_running_count: u32 = 0;

static RTEMS_MINIMUM_STACK_SIZE : uint32_t = 1024 * 4;
static RTEMS_DEFAULT_MODES : rtems_mode = 0;
static RTEMS_DEFAULT_ATTRIBUTES : rtems_attribute = 0;

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

fn print_tick(task: u32, time: rtems_time_of_day) -> () {
  let mut console = Console;
  writeln!(
    console,
    "[{}] tick at {}:{}:{}",
    task,
    time.hour, time.minute, time.second
  );
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TaskArg {
  index: u32,
  parent: rtems_id
}

pub unsafe extern "C" fn ticker_task(arg_buf: *mut c_void) -> () {
  let args: TaskArg = *(arg_buf as *mut TaskArg);

  let mut time : rtems_time_of_day = empty_tod();

  let mut console = Console;

  loop {
    let _ = rtems_clock_get_tod(&mut time);

    if time.hour >= 10 {
      let mut v: u32 = args.index;
      let mut r: *mut c_void = &mut v as *mut _ as *mut c_void;

      rtems_message_queue_send(task_ids[0], r, 4);
      rtems_task_exit();
    }

    print_tick(args.index, time);
    let _  = rtems_task_wake_after(50000);
  }
}

pub fn ticker_main_unsafe() -> Result<(), Error> {
  let mut console = Console;

  unsafe {

    writeln!(console, "[ ] starting ticker example");

    let mut time = rtems_time_of_day {
      year: 1988,
      month: 12,
      day: 31,
      hour: 9,
      minute: 0,
      second: 0,
      ticks: 0
    };

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

    let mut _arg1 = TaskArg { index: 1, parent: rtems_task_self() };
    let mut _arg2 = TaskArg { index: 2, parent: rtems_task_self() };
    let mut _arg3 = TaskArg { index: 3, parent: rtems_task_self() };

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

    writeln!(console, "[ ] task 1 started");
    task_running_count += 1;

    let _ = rtems_task_start(
      task_ids[2],
      ticker_task,
      &mut _arg2 as *mut _ as *mut c_void
    );

    writeln!(console, "[ ] task 2 started");
    task_running_count += 1;

    let _ = rtems_task_start(
      task_ids[3],
      ticker_task,
      &mut _arg3 as *mut _ as *mut c_void
    );

    writeln!(console, "[ ] task 3 started");
    task_running_count += 1;

    let _ = rtems_message_queue_create(
      rtems_build_name(44, 44, 44, 44),
      16,
      4,
      0,
      &mut task_ids[0]
    );

    loop {
      let mut msg: u32 = 0;
      let mut msg_size: u32 = 0;

      let _ = rtems_message_queue_receive(
        task_ids[0],
        &mut msg as *mut _ as *mut c_void,
        &mut msg_size as *mut size_t,
        0,
        0
      );

      writeln!(console, "[ ] received shutdown message from task {}", msg);

      task_running_count -= 1;

      if task_running_count == 0 {
        break;
      }

      writeln!(console, "[ ] {} tasks remaining", task_running_count);
    }
  }

  writeln!(console, "[ ] all tasks complete");

  Ok(())
}
