#![allow(dead_code)]

use super::gpio_pin::GpioPin;

pub const BASE: u32 = 0x5000_0300;

pub const I2C_EXT_SDA: GpioPin = GpioPin {
    base: BASE,
    offset: 0,
};

pub const GPIO3: GpioPin = GpioPin {
    base: BASE,
    offset: 2,
};

pub const FACE_TOUCH: GpioPin = GpioPin {
    base: BASE,
    offset: 4,
};

pub const COL4: GpioPin = GpioPin {
    base: BASE,
    offset: 5,
};

pub const UART_INT_TX: GpioPin = GpioPin {
    base: BASE,
    offset: 8,
};
