#![allow(dead_code)]

pub const BASE_ADDRESS: u32 = 0x4000_0000;

const TASKS_LOWPWR: *mut u32 = (BASE_ADDRESS + 0x7C) as *mut u32;

pub fn enable_low_power() {
    unsafe {
        let scr = 0xE000_ED10 as *mut u32;
        let current = core::ptr::read_volatile(scr);
        core::ptr::write_volatile(scr, (current & !(1 << 2)) | (current & 0xF0));
        core::ptr::write_volatile(TASKS_LOWPWR, 1);
    }
}
