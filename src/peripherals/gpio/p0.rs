#![allow(dead_code)]

use super::gpio_pin::GpioPin;

pub const BASE: u32 = 0x5000_0000;

pub const RING0: GpioPin = GpioPin {
    base: BASE,
    offset: 0,
};

pub const COL2: GpioPin = GpioPin {
    base: BASE,
    offset: 11,
};

pub const BTN_A: GpioPin = GpioPin {
    base: BASE,
    offset: 14,
};

pub const ROW3: GpioPin = GpioPin {
    base: BASE,
    offset: 15,
};

pub const ROW5: GpioPin = GpioPin {
    base: BASE,
    offset: 19,
};

pub const ROW1: GpioPin = GpioPin {
    base: BASE,
    offset: 21,
};

pub const ROW2: GpioPin = GpioPin {
    base: BASE,
    offset: 22,
};

pub const BTN_B: GpioPin = GpioPin {
    base: BASE,
    offset: 23,
};

pub const ROW4: GpioPin = GpioPin {
    base: BASE,
    offset: 24,
};

pub const COL1: GpioPin = GpioPin {
    base: BASE,
    offset: 28,
};

pub const COL5: GpioPin = GpioPin {
    base: BASE,
    offset: 30,
};

pub const COL3: GpioPin = GpioPin {
    base: BASE,
    offset: 31,
};
