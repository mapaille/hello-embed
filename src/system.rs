use crate::app::state;
use crate::peripherals::{gpio, rtc};
use crate::programs::ProgramId;
use crate::traits::Cancellable;
use crate::{clock, interrupt, power};

pub fn init() {
    clock::use_high_frequency_clock();
    interrupt::enable_global_interrupts();
    rtc::init(rtc_callback);
    gpio::init();
    power::enable_low_power();
    clock::use_low_frequency_clock();
}

#[inline(always)]
fn rtc_callback() {
    let program_id = state::get_program_id();
    let new_program_id = determine_program_id_from_buttons();

    if program_id != new_program_id {
        state::set_program_id(new_program_id);
        state::get_cancellation_token().cancel();
    }
}

#[inline(always)]
fn determine_program_id_from_buttons() -> ProgramId {
    if gpio::p0::BTN_A.is_low() && gpio::p0::BTN_B.is_low() {
        ProgramId::Startup
    } else if gpio::p0::BTN_A.is_low() {
        ProgramId::Love
    } else if gpio::p0::BTN_B.is_low() {
        ProgramId::Temperature
    } else {
        state::get_program_id()
    }
}
