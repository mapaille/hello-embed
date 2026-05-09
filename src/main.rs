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

use crate::app::{App, CANCELLATION_TOKEN, HARDWARE};
use crate::drivers::display::animations::ANIMATION_LOADING;
use crate::traits::{Clearable, Displayable};
use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    let hardware = &HARDWARE;
    let cancellation_token = &CANCELLATION_TOKEN;

    loop {
        hardware
            .screen
            .play_animation_once(&ANIMATION_LOADING, 1, cancellation_token);
    }
}

/// # Safety
///
/// This is the entry point of the program. It is called by the reset vector and executes
/// before any Rust initialization. It is safe because no other code has run yet, and
/// we are solely responsible for setting up the execution environment.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_handler() -> ! {
    system::init();
    let app = App::new();
    app.hardware.init();
    app.hardware.screen.clear();
    app.run();
}
