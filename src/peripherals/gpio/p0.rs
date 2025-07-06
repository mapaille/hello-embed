#![allow(dead_code)]

use super::gpio_pin::GpioPin;

pub const BASE: u32 = 0x5000_0000;

pub const RING0: GpioPin = unsafe { GpioPin::new(BASE, 0) };

pub const COL2: GpioPin = unsafe { GpioPin::new(BASE, 11) };

pub const BTN_A: GpioPin = unsafe { GpioPin::new(BASE, 14) };

pub const ROW3: GpioPin = unsafe { GpioPin::new(BASE, 15) };

pub const ROW5: GpioPin = unsafe { GpioPin::new(BASE, 19) };

pub const ROW1: GpioPin = unsafe { GpioPin::new(BASE, 21) };

pub const ROW2: GpioPin = unsafe { GpioPin::new(BASE, 22) };

pub const BTN_B: GpioPin = unsafe { GpioPin::new(BASE, 23) };

pub const ROW4: GpioPin = unsafe { GpioPin::new(BASE, 24) };

pub const COL1: GpioPin = unsafe { GpioPin::new(BASE, 28) };

pub const COL5: GpioPin = unsafe { GpioPin::new(BASE, 30) };

pub const COL3: GpioPin = unsafe { GpioPin::new(BASE, 31) };
