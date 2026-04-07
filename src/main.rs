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
use crate::traits::{Clearable, Displayable};
use core::panic::PanicInfo;
use crate::app::cancellation_token::CancellationToken;
use crate::app::hardware::Hardware;
use crate::drivers::display::animations::ANIMATION_LOADING;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    let hardware = Hardware::new();
    let cancellation_token = CancellationToken::new();

    loop {
        hardware.screen.play_animation_once(&ANIMATION_LOADING, 1, &cancellation_token);
    }
}

/// # Safety
///
/// This is the entry point of the program. It is called by the reset vector
#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_handler() -> ! {
    system::init();
    let app = App::new();
    app.hardware.init();
    app.hardware.screen.clear();
    app.run();
}
