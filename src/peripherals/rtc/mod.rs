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

use core::sync::atomic::{AtomicU32, Ordering};

pub static RTC_TICKS: AtomicU32 = AtomicU32::new(0);

// Initialize RTC0 for 1ms ticks using 32.768 kHz LFCLK
pub fn init() {
    unsafe {
        // Stop any existing RTC operation
        core::ptr::write_volatile(TASKS_STOP, 1);

        // Set prescaler for roughly 1ms ticks (32.768kHz / (32+1) ≈ 1kHz)
        core::ptr::write_volatile(PRESCALER, 32);

        // Clear counter
        core::ptr::write_volatile(TASKS_CLEAR, 1);

        // Enable tick event interrupt
        core::ptr::write_volatile(INTENSET, 1); // Bit 0: TICK

        // Enable RTC0 interrupt in NVIC
        const RTC0_IRQ_NUMBER: u32 = 11; // Based on your vector table

        const NVIC_ISER0: *mut u32 = 0xE000_E100 as *mut u32;

        core::ptr::write_volatile(NVIC_ISER0, 1 << RTC0_IRQ_NUMBER);

        // Start RTC
        core::ptr::write_volatile(TASKS_START, 1);
    }
}

// RTC0 interrupt handler
#[unsafe(no_mangle)]
pub extern "C" fn rtc0_handler() {
    if unsafe { core::ptr::read_volatile(EVENTS_TICK) } != 0 {
        // Clear the event
        unsafe { core::ptr::write_volatile(EVENTS_TICK, 0) };

        // Increment our tick counter
        RTC_TICKS.fetch_add(1, Ordering::Relaxed);
    }
}

// Wait for a specific number of ticks
pub fn wait_ticks(ticks: u32) {
    let start = RTC_TICKS.load(Ordering::Relaxed);
    let target = start.wrapping_add(ticks);

    while (RTC_TICKS.load(Ordering::Relaxed).wrapping_sub(target) as i32) < 0 {
        // Use WFI to sleep until next interrupt
        unsafe {
            core::arch::asm!("wfi", options(nomem, nostack, preserves_flags));
        }
    }
}
