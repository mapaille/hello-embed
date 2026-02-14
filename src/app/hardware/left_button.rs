use crate::peripherals::gpio::p0::BTN_A;
use crate::traits::Pressable;

pub struct LeftButton {}

impl LeftButton {
    pub const fn new() -> Self {
        LeftButton {}
    }
}

impl Pressable for LeftButton {
    fn is_pressed(&self) -> bool {
        BTN_A.is_low()
    }
}
