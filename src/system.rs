use crate::app::hardware::Hardware;
use crate::app::state;
use crate::peripherals::{gpio, rtc};
use crate::programs::ProgramId;
use crate::traits::{Cancellable, Pressable};
use crate::{clock, interrupt, power};

const HARDWARE: Hardware = Hardware::new();

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
    let program_id = state::get_program_id();
    let new_program_id = determine_program_id_from_buttons();

    if program_id != new_program_id {
        state::set_program_id(new_program_id);
        state::get_cancellation_token().cancel();
    }
}

#[inline]
fn determine_program_id_from_buttons() -> ProgramId {
    if HARDWARE.left_button.is_pressed() && HARDWARE.right_button.is_pressed() {
        ProgramId::Startup
    } else if HARDWARE.left_button.is_pressed() {
        ProgramId::Love
    } else if HARDWARE.right_button.is_pressed() {
        ProgramId::Temperature
    } else if HARDWARE.touch_button.is_pressed() {
        ProgramId::Startup
    } else {
        state::get_program_id()
    }
}
