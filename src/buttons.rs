#![allow(dead_code)]

use crate::peripherals;
use crate::traits::Pressable;

pub struct LeftButton {}

pub struct RightButton {}

impl LeftButton {
    pub fn new() -> Self {
        LeftButton {}
    }
}

impl RightButton {
    pub fn new() -> Self {
        RightButton {}
    }
}

impl Pressable for LeftButton {
    fn is_pressed(&self) -> bool {
        peripherals::gpio::p0::BTN_A.is_low()
    }
}

impl Pressable for RightButton {
    fn is_pressed(&self) -> bool {
        peripherals::gpio::p0::BTN_B.is_low()
    }
}
