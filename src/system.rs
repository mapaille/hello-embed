use crate::peripherals::gpio;
use crate::{clock, interrupt, peripherals::rtc, power};

pub unsafe fn init() {
    clock::use_high_frequency_clock();
    interrupt::enable_global_interrupts();
    rtc::init();
    power::enable_low_power();
    gpio::p0::BTN_A.as_input_pullup();
    gpio::p0::BTN_B.as_input_pullup();
    clock::use_low_frequency_clock();
}
