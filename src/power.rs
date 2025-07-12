#![allow(dead_code)]

pub const BASE_ADDRESS: u32 = 0x4000_0000;

const TASKS_LOWPWR: *mut u32 = (BASE_ADDRESS + 0x7C) as *mut u32;

pub fn enable_low_power() {
    #[cfg(not(debug_assertions))]
    unsafe {
        // SCR register controls sleep behavior
        let scr = 0xE000_ED10 as *mut u32;

        // Read current value
        let current = core::ptr::read_volatile(scr);

        // Clear SLEEPDEEP bit (bit 2) to use regular sleep not deep sleep
        // ALSO ensure debug remains enabled (don't modify debug bits)
        core::ptr::write_volatile(scr, (current & !(1 << 2)) | (current & 0xF0));

        // Now enable low power mode, but preserve debug functionality
        // You might need to modify this based on your specific chip
        core::ptr::write_volatile(TASKS_LOWPWR, 1);
    }
}
