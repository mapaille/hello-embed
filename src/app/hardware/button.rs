use crate::peripherals::gpio::GpioPin;
use crate::traits::Pressable;

pub struct Button {
    pin: GpioPin,
}

impl Button {
    pub const fn new(pin: GpioPin) -> Self {
        Self { pin }
    }
}

impl Pressable for Button {
    fn is_pressed(&self) -> bool {
        self.pin.is_low()
    }
}
