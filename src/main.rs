#![no_std]
#![no_main]
#![allow(dead_code)]

mod app;
mod clock;
mod drivers;
mod interrupt;
mod peripherals;
mod power;
mod programs;
mod system;
mod timing;
mod traits;
mod vector_table;

use crate::app::App;
use core::panic::PanicInfo;

#[panic_handler]
const fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// # Safety
///
/// This is the entry point of the program. It is called by the reset vector
#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_handler() -> ! {
    system::init();
    let app = App::new();
    app.hardware.speaker.init();
    app.hardware.speaker.beep();
    app.run();
}
