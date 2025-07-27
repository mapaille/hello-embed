#![allow(dead_code)]

use crate::peripherals::gpio::{Gpio, Register};

// nRF52833 GPIO register offsets (divided by 4 for u32 indexing)
pub const OUTSET: usize = 0x508 / 4;
pub const OUTCLR: usize = 0x50C / 4;
pub const DIRSET: usize = 0x518 / 4;
pub const INPUT: usize = 0x510 / 4;
const PIN_CNF: usize = 0x700 / 4;
const PIN_CNF_OUTPUT: u32 = 1 << 0; // DIR = output
const PIN_CNF_INPUT_PULLUP: u32 = 3 << 2; // PULL = pull-up

pub struct GpioPin {
    gpio: &'static Gpio,
    offset: usize,
}

impl GpioPin {
    pub const fn new(gpio: &'static Gpio, offset: usize) -> Self {
        Self { gpio, offset }
    }

    fn reg_at(&self, offset: usize) -> Register<u32> {
        unsafe { Register::new(self.gpio.base_addr.as_ptr().add(offset) as *mut u32) }
    }

    pub fn as_output(&self) {
        self.reg_at(PIN_CNF + self.offset).write(PIN_CNF_OUTPUT);
    }

    pub fn as_input_pullup(&self) {
        self.reg_at(PIN_CNF + self.offset)
            .write(PIN_CNF_INPUT_PULLUP);
    }

    pub fn is_low(&self) -> bool {
        (self.reg_at(INPUT).read() & (1 << self.offset)) == 0
    }

    pub fn is_high(&self) -> bool {
        !self.is_low()
    }

    pub fn set_high(&self) {
        self.reg_at(OUTSET).write(1 << self.offset);
    }

    pub fn set_low(&self) {
        self.reg_at(OUTCLR).write(1 << self.offset);
    }

    pub fn toggle(&self) {
        if self.is_high() {
            self.set_low();
        } else {
            self.set_high();
        }
    }
}
