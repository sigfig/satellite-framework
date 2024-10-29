////////////////////////////////////////////////////////////////////////////////
// temporary executable entry point for testing
////////////////////////////////////////////////////////////////////////////////

use rtems::*;
use crate::io::*;
use crate::examples::ticker::*;

use core::primitive::char;
use core::result::Result;
use core::fmt::Error;
use core::fmt::Write;

////////////////////////////////////////////////////////////////////////////////
// rtems application entry point

#[no_mangle]
pub extern "C" fn Init() {
    if let Err(e) = ticker_main_unsafe() {
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
