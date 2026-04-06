#![allow(dead_code)]

use crate::peripherals::gpio::Gpio;

// For PWM output pins (speaker, LEDs, etc.)
const CNF_OUTPUT_STD: u32 = 0b0000_0000_0000_0000_0000_0000_0000_0001u32;
// Bit breakdown:
// - DIR   = 1
// - INPUT = 0 (disconnected)
// - PULL  = 00 (no pull)
// - DRIVE = 000 (S0S1 standard)

// For regular input pins with pull-up (your second constant is fine, but clearer name helps)
const CNF_INPUT_PULLUP_STD: u32 = 0b0000_0000_0000_0000_0000_1100_0000_0000u32;
// - DIR   = 0
// - INPUT = 0 (or 1 depending on use)
// - PULL  = 11 (PullUp)
// - DRIVE = 000

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

    pub fn configure_speaker(&self) {
        unsafe {
            self.port.pin_cnf(self.pin).write_volatile(0b0000_0000_0000_0000_0000_0000_0000_0011u32);
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
