use crate::peripherals::{gpio, rtc};
use crate::{clock, interrupt, power};

pub fn init() {
    clock::use_high_frequency_clock();
    interrupt::enable_global_interrupts();
    rtc::init();
    gpio::init();
    power::enable_low_power();
    clock::use_low_frequency_clock();
}
