#![no_std]
#![no_main]
mod app;
mod cancellation;
mod clock;
mod drivers;
mod interrupt;
mod peripherals;
mod power;
mod programs;
mod system;
mod timing;
mod vector_table;

use core::panic::PanicInfo;
use drivers::screen::Screen;
use peripherals::gpio;

const SCREEN_ROW_PINS: [gpio::GpioPin; 5] = [
    gpio::p0::ROW1,
    gpio::p0::ROW2,
    gpio::p0::ROW3,
    gpio::p0::ROW4,
    gpio::p0::ROW5,
];
const SCREEN_COL_PINS: [gpio::GpioPin; 5] = [
    gpio::p0::COL1,
    gpio::p0::COL2,
    gpio::p0::COL3,
    gpio::p1::COL4,
    gpio::p0::COL5,
];

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_handler() -> ! {
    unsafe {
        system::init();
    }

    let mut screen = Screen::init(SCREEN_ROW_PINS, SCREEN_COL_PINS);

    app::run(&mut screen);
}
