use crate::peripherals::gpio::GpioPin;
use crate::traits::Pressable;

pub struct FaceTouch {
    gpio_pin: GpioPin,
}

impl FaceTouch {
    pub const fn new(gpio_pin: GpioPin) -> Self {
        Self { gpio_pin }
    }
}

impl Pressable for FaceTouch {
    fn is_pressed(&self) -> bool {
        false
    }
}
