use crate::app::rtc_handler::RtcHandler;
use crate::peripherals::rtc;
use crate::{clock, interrupt, power};

const RTC_HANDLER: RtcHandler = RtcHandler::new();

pub fn init() {
    if cfg!(debug_assertions) {
        clock::use_high_frequency_clock();
        rtc::init(rtc_callback);
        interrupt::enable_global_interrupts();
    } else {
        clock::use_low_frequency_clock();
        rtc::init(rtc_callback);
        interrupt::enable_global_interrupts();
        power::enable_low_power();
    }
}

#[inline]
fn rtc_callback() {
    RTC_HANDLER.on_rtc();
}
