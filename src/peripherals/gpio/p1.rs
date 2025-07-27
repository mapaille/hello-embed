#![allow(dead_code)]

use super::gpio_pin::GpioPin;
use crate::peripherals::gpio::GPIO_P1;

pub const I2C_EXT_SDA: GpioPin = GpioPin::new(&GPIO_P1, 0);
pub const GPIO3: GpioPin = GpioPin::new(&GPIO_P1, 2);
pub const FACE_TOUCH: GpioPin = GpioPin::new(&GPIO_P1, 4);
pub const COL4: GpioPin = GpioPin::new(&GPIO_P1, 5);
pub const UART_INT_TX: GpioPin = GpioPin::new(&GPIO_P1, 8);
