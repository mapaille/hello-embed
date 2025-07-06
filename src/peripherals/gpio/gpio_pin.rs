#![allow(dead_code)]

use core::ptr;

pub const OUTSET: usize = 0x508 / 4;
pub const OUTCLR: usize = 0x50C / 4;
pub const DIRSET: usize = 0x518 / 4;
pub const INPUT: usize = 0x510 / 4;

const PIN_CNF_BASE: usize = 0x700 / 4;
const PIN_CNF_OUTPUT: u32 = 1 << 0; // DIR = output
const PIN_CNF_INPUT_PULLUP: u32 = (0 << 0) | // DIR   = input
    (0 << 1) | // INPUT = connect
    (3 << 2); // PULL  = pull-up

pub struct GpioPin {
    pub base: u32,
    pub offset: usize,
}

impl GpioPin {
    pub fn as_output(&self) -> &GpioPin {
        write(self.base as *mut u32, PIN_CNF_BASE + &self.offset, PIN_CNF_OUTPUT);
        self
    }

    pub fn as_input_pullup(&self) -> &GpioPin {
        write(self.base as *mut u32, PIN_CNF_BASE + self.offset, PIN_CNF_INPUT_PULLUP);
        self
    }

    pub fn is_low(&self) -> bool {
        (read(self.base as *mut u32, INPUT) & (1 << self.offset)) == 0
    }

    pub fn is_high(&self) -> bool {
        (read(self.base as *mut u32, INPUT) & (1 << self.offset)) == 1
    }

    pub fn set_high(&self) {
        write(self.base as *mut u32, OUTSET, 1 << self.offset);
    }

    pub fn set_low(&self) {
        write(self.base as *mut u32, OUTCLR, 1 << self.offset);
    }
}

pub fn read(base: *mut u32, offset: usize) -> u32 {
    unsafe {
        ptr::read_volatile(base.add(offset))
    }
}

pub fn write(base: *mut u32, offset: usize, value: u32) {
    unsafe {
        ptr::write_volatile(base.add(offset), value);
    }
}
