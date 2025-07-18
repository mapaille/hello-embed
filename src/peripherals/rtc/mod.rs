#![allow(dead_code)]

pub const RTC0_BASE: u32 = 0x4000_B000;

const TASKS_START: *mut u32 = (RTC0_BASE + 0x000) as *mut u32;
const TASKS_STOP: *mut u32 = (RTC0_BASE + 0x004) as *mut u32;
const TASKS_CLEAR: *mut u32 = (RTC0_BASE + 0x008) as *mut u32;
const EVENTS_TICK: *mut u32 = (RTC0_BASE + 0x100) as *mut u32;
const EVENTS_COMPARE0: *mut u32 = (RTC0_BASE + 0x140) as *mut u32;
const INTENSET: *mut u32 = (RTC0_BASE + 0x304) as *mut u32;
const COUNTER: *const u32 = (RTC0_BASE + 0x504) as *const u32;
const PRESCALER: *mut u32 = (RTC0_BASE + 0x508) as *mut u32;
const CC0: *mut u32 = (RTC0_BASE + 0x540) as *mut u32;
const RTC0_IRQ_NUMBER: u32 = 11;
const NVIC_ISER0: *mut u32 = 0xE000_E100 as *mut u32;

use crate::app::state;
use crate::peripherals::gpio;
use crate::traits::Cancellable;
use core::sync::atomic::{AtomicU32, Ordering};

pub static RTC_TICKS: AtomicU32 = AtomicU32::new(0);
pub static BTN_A_PRESSED: AtomicU32 = AtomicU32::new(0);
pub static BTN_B_PRESSED: AtomicU32 = AtomicU32::new(0);

pub fn init() {
    unsafe {
        core::ptr::write_volatile(TASKS_STOP, 1);
        core::ptr::write_volatile(PRESCALER, 32);
        core::ptr::write_volatile(TASKS_CLEAR, 1);
        core::ptr::write_volatile(INTENSET, 1);
        core::ptr::write_volatile(NVIC_ISER0, 1 << RTC0_IRQ_NUMBER);
        core::ptr::write_volatile(TASKS_START, 1);
    }
}

#[unsafe(no_mangle)]
#[inline(always)]
pub extern "C" fn rtc0_handler() {
    if unsafe { core::ptr::read_volatile(EVENTS_TICK) != 0 } {
        unsafe { core::ptr::write_volatile(EVENTS_TICK, 0) };
        RTC_TICKS.fetch_add(1, Ordering::Relaxed);
        check_buttons_and_update_program();
    }
}

#[inline(always)]
fn check_buttons_and_update_program() {
    let program_id = state::get_program_id();
    let new_program_id = determine_program_id_from_buttons();

    if program_id != new_program_id {
        state::set_program_id(new_program_id);
        state::get_cancellation_token().cancel();
    }
}

#[inline(always)]
fn determine_program_id_from_buttons() -> u8 {
    if gpio::p0::BTN_A.is_low() && gpio::p0::BTN_B.is_low() {
        state::STARTUP_PROGRAM_ID
    } else if gpio::p0::BTN_A.is_low() {
        state::LOVE_PROGRAM_ID
    } else if gpio::p0::BTN_B.is_low() {
        state::TEMP_PROGRAM_ID
    } else {
        state::get_program_id()
    }
}
