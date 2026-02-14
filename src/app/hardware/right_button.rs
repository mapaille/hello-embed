use crate::peripherals::gpio::p0::BTN_B;
use crate::traits::Pressable;

pub struct RightButton {}

impl RightButton {
    pub const fn new() -> Self {
        RightButton {}
    }
}

impl Pressable for RightButton {
    fn is_pressed(&self) -> bool {
        BTN_B.is_low()
    }
}
