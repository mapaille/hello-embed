use crate::peripherals;
use crate::traits;

pub struct RightButton {}

impl RightButton {
    pub fn new() -> Self {
        RightButton {}
    }
}

impl traits::Pressable for RightButton {
    fn is_pressed(&self) -> bool {
        peripherals::gpio::p0::BTN_B.is_low()
    }
}
