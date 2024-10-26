use crate::rtems::*;
use crate::io::*;

use core::fmt::Write;

#[no_mangle]
pub extern "C" fn Init() {
    if let Err(e) = rust_main() {
        panic!("Main returned {:?}", e);
    }
    unsafe {
        rtems_shutdown_executive( 0 );
    }
}

fn rust_main() -> Result<(), core::fmt::Error> {
    let mut console = Console;
    writeln!(console, "Hello from Rust")?;
    Ok(())
}
