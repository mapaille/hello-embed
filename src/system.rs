use crate::app::rtc_handler::RtcHandler;
use crate::peripherals::{gpio, rtc};
use crate::{clock, interrupt, power};

static RTC_HANDLER: RtcHandler = RtcHandler::new();

pub fn init() {
    clock::use_high_frequency_clock();
    gpio::init();
    rtc::init(rtc_callback);
    interrupt::enable_global_interrupts();
    power::enable_low_power();
    clock::use_low_frequency_clock();
}

#[inline]
fn rtc_callback() {
    RTC_HANDLER.on_rtc();
}
