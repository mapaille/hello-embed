#![allow(dead_code)]

use super::gpio_pin::GpioPin;

const BASE: u32 = 0x5000_0300;

pub const I2C_EXT_SDA: GpioPin = GpioPin::new(BASE, 0);
pub const GPIO3: GpioPin = GpioPin::new(BASE, 2);
pub const FACE_TOUCH: GpioPin = GpioPin::new(BASE, 4);
pub const COL4: GpioPin = GpioPin::new(BASE, 5);
pub const UART_INT_TX: GpioPin = GpioPin::new(BASE, 8);
