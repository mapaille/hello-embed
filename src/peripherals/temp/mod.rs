#![allow(dead_code)]

use crate::traits::Register;
use core::ptr::NonNull;

pub const TEMP0: Temp = Temp::new(0x4000_C000);

pub struct Temp {
    base_addr: NonNull<u8>,
}

impl Temp {
    pub const fn new(base_addr: usize) -> Self {
        Self {
            base_addr: unsafe { NonNull::new_unchecked(base_addr as *mut u8) },
        }
    }

    pub fn read(&self) -> u32 {
        self.read_reg(0x508)
    }

    pub fn start(&self) {
        self.write_reg(0, 1u32);
    }

    pub fn stop(&self) {
        self.write_reg(0x004, 1u32);
    }

    pub fn is_ready(&self) -> bool {
        self.read_reg(0x100) & 1 != 0
    }

    pub fn clear(&self) {
        self.write_reg(0x508, 0u32);
        self.write_reg(0x100, 0u32);
    }
}

impl Register for Temp {
    fn base_addr(&self) -> NonNull<u8> {
        self.base_addr
    }
}

unsafe impl Sync for Temp {}
unsafe impl Send for Temp {}
