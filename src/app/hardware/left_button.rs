use crate::peripherals;
use crate::traits;

pub struct LeftButton {}

impl LeftButton {
    pub fn new() -> Self {
        LeftButton {}
    }
}

impl traits::Pressable for LeftButton {
    fn is_pressed(&self) -> bool {
        peripherals::gpio::p0::BTN_A.is_low()
    }
}
