#![allow(dead_code)]

pub mod gpio_pin;
pub mod p0;
pub mod p1;

use core::ptr::{NonNull, read_volatile, write_volatile};
pub use gpio_pin::GpioPin;

const GPIO_P0: Gpio = Gpio::new(0x5000_0000);
const GPIO_P1: Gpio = Gpio::new(0x5000_0300);

pub struct Gpio {
    base_addr: NonNull<usize>,
}
struct Register<T> {
    addr: *mut T,
}

impl Gpio {
    pub const fn new(base_addr: usize) -> Self {
        Self {
            base_addr: NonNull::new(base_addr as *mut usize).unwrap(),
        }
    }
}

impl<T: Copy> Register<T> {
    pub const unsafe fn new(addr: *mut T) -> Self {
        Self { addr }
    }

    pub fn read(&self) -> T {
        unsafe { read_volatile(self.addr) }
    }

    pub fn write(&self, value: T) {
        unsafe { write_volatile(self.addr, value) }
    }
}
