#![allow(dead_code)]

pub mod gpio_pin;
pub mod p0;
pub mod p1;

use crate::traits::Register;
use core::ptr::NonNull;
pub use gpio_pin::GpioPin;

const GPIO_P0: Gpio = Gpio::new(0x5000_0000);
const GPIO_P1: Gpio = Gpio::new(0x5000_0300);

pub struct Gpio {
    base_addr: NonNull<u8>,
}

impl Gpio {
    pub const fn new(base_addr: usize) -> Self {
        Self {
            base_addr: unsafe { NonNull::new_unchecked(base_addr as *mut u8) },
        }
    }

    #[inline]
    pub fn set_outset(&self, pin: &GpioPin) {
        self.write_reg(0x508, pin.pin_mask());
    }

    #[inline]
    pub fn set_outclr(&self, pin: &GpioPin) {
        self.write_reg(0x50C, pin.pin_mask());
    }

    #[inline]
    pub fn is_input(&self) -> u32 {
        self.read_reg(0x510)
    }

    #[inline]
    pub fn set_pin_cnf(&self, pin: &GpioPin, value: u32) {
        self.write_reg(0x700 + (pin.pin_number() as usize) * 4, value);
    }
}

impl Register for Gpio {
    fn base_addr(&self) -> NonNull<u8> {
        self.base_addr
    }
}
