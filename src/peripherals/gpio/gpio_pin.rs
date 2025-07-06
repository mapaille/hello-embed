#![allow(dead_code)]

use core::ptr;

// nRF52833 GPIO register offsets (divided by 4 for u32 indexing)
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
    base: ptr::NonNull<u32>,
    offset: usize,
}

impl GpioPin {
    pub const unsafe fn new(base: u32, offset: usize) -> Self {
        Self {
            base: unsafe { ptr::NonNull::new_unchecked(base as *mut u32) },
            offset,
        }
    }

    pub fn as_output(&self) -> &GpioPin {
        write(self.base, PIN_CNF_BASE + self.offset, PIN_CNF_OUTPUT);
        self
    }

    pub fn as_input_pullup(&self) -> &GpioPin {
        write(self.base, PIN_CNF_BASE + self.offset, PIN_CNF_INPUT_PULLUP);
        self
    }

    pub fn is_low(&self) -> bool {
        (read(self.base, INPUT) & (1 << self.offset)) == 0
    }

    pub fn is_high(&self) -> bool {
        !self.is_low()
    }

    pub fn set_high(&self) {
        write(self.base, OUTSET, 1 << self.offset);
    }

    pub fn set_low(&self) {
        write(self.base, OUTCLR, 1 << self.offset);
    }

    pub fn toggle(&self) {
        if self.is_set_high() {
            self.set_low();
        } else {
            self.set_high();
        }
    }

    pub fn is_set_high(&self) -> bool {
        (read(self.base, INPUT) & (1 << self.offset)) != 0
    }

    pub fn is_set_low(&self) -> bool {
        !self.is_set_high()
    }
}

fn read(base: ptr::NonNull<u32>, offset: usize) -> u32 {
    unsafe { ptr::read_volatile(base.as_ptr().add(offset)) }
}

fn write(base: ptr::NonNull<u32>, offset: usize, value: u32) {
    unsafe { ptr::write_volatile(base.as_ptr().add(offset), value) }
}
