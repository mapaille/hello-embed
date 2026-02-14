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
use app::state;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_handler() -> ! {
    let mut hardware = hardware::Hardware::new();
    hardware.init();
    system::init();
    let mut app = app::App::new(hardware);
    app.run(state::get_cancellation_token())
}
