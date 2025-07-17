#![no_std]
#![no_main]
#![allow(dead_code)]

mod app;
mod buttons;
mod cancellation;
mod clock;
mod drivers;
mod hardware;
mod interrupt;
mod peripherals;
mod power;
mod programs;
mod state;
mod system;
mod timing;
mod traits;
mod vector_table;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_handler() -> ! {
    system::init();
    let mut app = app::App::new();
    app.run(state::get_cancellation_token())
}
