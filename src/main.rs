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

use crate::app::hardware;
use crate::traits::Displayable;
use app::state;
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

    let mut hardware = hardware::Hardware::new();
    hardware.screen.clear();

    let mut app = app::App::new(hardware);
    app.run(state::get_cancellation_token())
}
