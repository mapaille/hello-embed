#![allow(dead_code)]

use super::gpio_pin::GpioPin;
use crate::peripherals::gpio::GPIO_P0;

pub const RING0: GpioPin = GpioPin::new(&GPIO_P0, 0);
pub const COL2: GpioPin = GpioPin::new(&GPIO_P0, 11);
pub const BTN_A: GpioPin = GpioPin::new(&GPIO_P0, 14);
pub const ROW3: GpioPin = GpioPin::new(&GPIO_P0, 15);
pub const ROW5: GpioPin = GpioPin::new(&GPIO_P0, 19);
pub const ROW1: GpioPin = GpioPin::new(&GPIO_P0, 21);
pub const ROW2: GpioPin = GpioPin::new(&GPIO_P0, 22);
pub const BTN_B: GpioPin = GpioPin::new(&GPIO_P0, 23);
pub const ROW4: GpioPin = GpioPin::new(&GPIO_P0, 24);
pub const COL1: GpioPin = GpioPin::new(&GPIO_P0, 28);
pub const COL5: GpioPin = GpioPin::new(&GPIO_P0, 30);
pub const COL3: GpioPin = GpioPin::new(&GPIO_P0, 31);
