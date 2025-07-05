#![allow(dead_code)]

pub const BASE_ADDRESS: u32 = 0x40000000;

// Clock registers
const CLOCK_HFCLKSTAT: *const u32 = (BASE_ADDRESS + 0x40C) as *const u32;
const CLOCK_LFCLKSTAT: *const u32 = (BASE_ADDRESS + 0x418) as *const u32;
const CLOCK_HFCLKRUN: *mut u32 = (BASE_ADDRESS + 0x408) as *mut u32;
const CLOCK_HFCLKSTART: *mut u32 = (BASE_ADDRESS + 0x000) as *mut u32;
const CLOCK_HFCLKSTOP: *mut u32 = (BASE_ADDRESS + 0x004) as *mut u32;
const CLOCK_LFCLKSTART: *mut u32 = (BASE_ADDRESS + 0x008) as *mut u32;
const CLOCK_LFCLKSTOP: *mut u32 = (BASE_ADDRESS + 0x00C) as *mut u32;

/// Switch to the low-frequency (32.768 kHz) clock
/// This significantly reduces power consumption
pub fn use_low_frequency_clock() {
    unsafe {
        // Stop the high-frequency clock
        core::ptr::write_volatile(CLOCK_HFCLKSTOP, 1);

        // Start the low-frequency RC oscillator
        core::ptr::write_volatile(CLOCK_LFCLKSTART, 1);

        // Wait until the low-frequency clock is running
        while core::ptr::read_volatile(CLOCK_LFCLKSTAT) & 0x10000 == 0 {
            core::hint::spin_loop();
        }
    }
}

/// Switch to the high-frequency (64 MHz) clock
/// This is the default clock with higher performance but higher power consumption
pub fn use_high_frequency_clock() {
    unsafe {
        // Start the high-frequency crystal oscillator
        core::ptr::write_volatile(CLOCK_HFCLKSTART, 1);

        // Wait until the high-frequency clock is running
        while core::ptr::read_volatile(CLOCK_HFCLKSTAT) & 0x10000 == 0 {
            core::hint::spin_loop();
        }
    }
}
