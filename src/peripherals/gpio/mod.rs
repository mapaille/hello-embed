#![allow(dead_code)]

pub mod gpio_pin;
pub mod p0;
pub mod p1;

use core::ptr::NonNull;
pub use gpio_pin::GpioPin;

const GPIO_P0: Gpio = Gpio::new(0x5000_0000);
const GPIO_P1: Gpio = Gpio::new(0x5000_0300);

pub struct Gpio {
    base_addr: NonNull<u32>,
}

impl Gpio {
    pub const fn new(base_addr: usize) -> Self {
        Self {
            base_addr: NonNull::new(base_addr as *mut u32).unwrap(),
        }
    }

    #[inline]
    pub const unsafe fn outset(&self) -> *mut u32 {
        unsafe { self.base_addr.as_ptr().add(0x508 / 4) }
    }

    #[inline]
    pub const unsafe fn outclr(&self) -> *mut u32 {
        unsafe { self.base_addr.as_ptr().add(0x50C / 4) }
    }

    #[inline]
    pub const unsafe fn input(&self) -> *const u32 {
        unsafe { self.base_addr.as_ptr().add(0x510 / 4) }
    }

    #[inline]
    pub const unsafe fn pin_cnf(&self, pin: u8) -> *mut u32 {
        unsafe { self.base_addr.as_ptr().add(0x700 / 4 + pin as usize) }
    }
}
