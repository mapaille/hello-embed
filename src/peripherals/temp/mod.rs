#![allow(dead_code)]

const BASE_ADDRESS: u32 = 0x4000_C000;
const TASKS_START: *mut u32 = (BASE_ADDRESS + 0x000) as *mut u32;
const TASKS_STOP: *mut u32 = (BASE_ADDRESS + 0x004) as *mut u32;
const EVENTS_DATARDY: *mut u32 = (BASE_ADDRESS + 0x100) as *mut u32;
const INTENSET: *mut u32 = (BASE_ADDRESS + 0x304) as *mut u32;
const INTENCLR: *mut u32 = (BASE_ADDRESS + 0x308) as *mut u32;
const TEMP: *mut u32 = (BASE_ADDRESS + 0x508) as *mut u32;

pub fn read_temp() -> u32 {
    unsafe { core::ptr::read_volatile(TEMP) }
}

pub fn start() {
    unsafe { core::ptr::write_volatile(TASKS_START, 1) }
}

pub fn stop() {
    unsafe { core::ptr::write_volatile(TASKS_STOP, 1) }
}

pub fn is_ready() -> bool {
    unsafe { core::ptr::read_volatile(EVENTS_DATARDY) & 1 != 0 }
}

pub fn clear() {
    unsafe {
        core::ptr::write_volatile(TEMP, 0);
        core::ptr::write_volatile(EVENTS_DATARDY, 0)
    }
}
