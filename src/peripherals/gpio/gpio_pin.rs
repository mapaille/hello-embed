#![allow(dead_code)]

use crate::peripherals::gpio::Gpio;

// nRF52833 GPIO register offsets (divided by 4 for u32 indexing)
const CNF_OUTPUT_STD: u32 = 0b0000_0000_0000_0000_0000_0000_0000_0001; // DIR=1, DRIVE=S0S1=0, rest default
const CNF_INPUT_PULLUP_STD: u32 = 0b0000_0000_0000_0000_0000_1100_0000_0000; // PULL=3, DIR=0, DRIVE=0

#[derive(Clone, Copy)]
pub struct GpioPin {
    port: &'static Gpio,
    pin: u8,
}

impl GpioPin {
    #[inline]
    pub const fn new(port: &'static Gpio, pin: u8) -> Self {
        Self { port, pin }
    }

    #[inline]
    pub const fn pin_number(&self) -> u32 {
        self.pin as u32
    }

    #[inline]
    const fn pin_mask(&self) -> u32 {
        1u32 << self.pin
    }

    pub fn configure_output(&self) {
        unsafe {
            self.port.pin_cnf(self.pin).write_volatile(CNF_OUTPUT_STD);
        }
    }

    #[inline]
    pub fn configure_input_pullup(&self) {
        unsafe {
            self.port
                .pin_cnf(self.pin)
                .write_volatile(CNF_INPUT_PULLUP_STD);
        }
    }

    #[inline]
    pub fn is_high(&self) -> bool {
        (unsafe { self.port.input().read_volatile() } & self.pin_mask()) != 0
    }

    #[inline]
    pub fn is_low(&self) -> bool {
        !self.is_high()
    }

    #[inline]
    pub fn set_high(&self) {
        unsafe {
            self.port.outset().write_volatile(self.pin_mask());
        }
    }

    #[inline]
    pub fn set_low(&self) {
        unsafe {
            self.port.outclr().write_volatile(self.pin_mask());
        }
    }

    #[inline]
    pub fn set(&self, state: bool) {
        if state {
            self.set_high();
        } else {
            self.set_low();
        }
    }

    #[inline]
    pub fn toggle(&self) {
        // This pattern usually compiles to better code than read-modify-write
        if self.is_high() {
            self.set_low();
        } else {
            self.set_high();
        }
    }
}

unsafe impl Send for GpioPin {}
unsafe impl Sync for GpioPin {}
