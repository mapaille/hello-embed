use crate::app::App;
use crate::peripherals::{gpio, rtc};
use crate::{clock, interrupt, power};

pub struct AppBuilder {}

impl AppBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self) -> App {
        clock::use_high_frequency_clock();
        interrupt::enable_global_interrupts();
        rtc::init();
        power::enable_low_power();
        gpio::p0::BTN_A.as_input_pullup();
        gpio::p0::BTN_B.as_input_pullup();
        clock::use_low_frequency_clock();
        App::new()
    }
}
