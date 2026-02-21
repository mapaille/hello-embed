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

    p0::COL1.as_output();
    p0::COL2.as_output();
    p0::COL3.as_output();
    p1::COL4.as_output();
    p0::COL5.as_output();

    p0::ROW1.as_output();
    p0::ROW2.as_output();
    p0::ROW3.as_output();
    p0::ROW4.as_output();
    p0::ROW5.as_output();
}
