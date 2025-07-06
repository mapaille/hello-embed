use crate::{clock, interrupt, peripherals::rtc, power};

pub unsafe fn init() {
    clock::use_high_frequency_clock();
    interrupt::enable_global_interrupts();
    rtc::init();
    power::enable_low_power();
    clock::use_low_frequency_clock();
}
