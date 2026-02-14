use core::ptr::{read_volatile, write_volatile};

pub struct Register<T> {
    addr: *mut T,
}

impl<T: Copy> Register<T> {
    pub const fn new(addr: *mut T) -> Self {
        Self { addr }
    }

    pub fn read(&self) -> T {
        unsafe { read_volatile(self.addr) }
    }

    pub fn write(&self, value: T) {
        unsafe { write_volatile(self.addr, value) }
    }
}
