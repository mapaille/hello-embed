#![allow(dead_code)]

pub mod gpio_pin;
pub mod p0;
pub mod p1;

use core::ptr::NonNull;
pub use gpio_pin::GpioPin;

const GPIO_P0: Gpio = Gpio::new(0x5000_0000);
const GPIO_P1: Gpio = Gpio::new(0x5000_0300);

pub struct Gpio {
    base_addr: NonNull<usize>,
}

impl Gpio {
    pub const fn new(base_addr: usize) -> Self {
        Self {
            base_addr: NonNull::new(base_addr as *mut usize).unwrap(),
        }
    }
}

pub fn init() {
    p0::BTN_A.as_input_pullup();
    p0::BTN_B.as_input_pullup();
}
