#![allow(dead_code)]

use crate::peripherals::pin::Pin;

pub const BASE: u32 = 0x5000_0300;

pub const I2C_EXT_SDA: Pin = Pin {
    base: BASE,
    offset: 0,
};

pub const GPIO3: Pin = Pin {
    base: BASE,
    offset: 2,
};

pub const FACE_TOUCH: Pin = Pin {
    base: BASE,
    offset: 4,
};

pub const COL4: Pin = Pin {
    base: BASE,
    offset: 5,
};

pub const UART_INT_TX: Pin = Pin {
    base: BASE,
    offset: 8,
};
