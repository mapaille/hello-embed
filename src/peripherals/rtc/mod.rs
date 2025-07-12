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

use core::sync::atomic::{AtomicU32, Ordering};
use crate::app::{ActiveProgram, ACTIVE_PROGRAM, CANCELLATION_TOKEN_SOURCE};
use crate::peripherals::gpio;

pub static RTC_TICKS: AtomicU32 = AtomicU32::new(0);

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

fn check_buttons_and_update_program() {
    let active_program = ACTIVE_PROGRAM.load(Ordering::Relaxed);
    let new_program = determine_program_from_buttons();
    
    if active_program != new_program {
        ACTIVE_PROGRAM.store(new_program, Ordering::Relaxed);
        CANCELLATION_TOKEN_SOURCE.cancel();
    }
}

fn determine_program_from_buttons() -> u8 {
    if gpio::p0::BTN_A.is_low() && gpio::p0::BTN_B.is_low() {
        ActiveProgram::Startup as u8
    } else if gpio::p0::BTN_A.is_low() {
        ActiveProgram::Love as u8
    } else if gpio::p0::BTN_B.is_low() {
        ActiveProgram::Temp as u8
    } else {
        ACTIVE_PROGRAM.load(Ordering::Relaxed)
    }
}
