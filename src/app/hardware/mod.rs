mod left_button;
mod right_button;
mod temp_sensor;

use crate::drivers::screens;
use crate::peripherals::gpio;

const SCREEN_ROW_PINS: [gpio::GpioPin; 5] = [
    gpio::p0::ROW1,
    gpio::p0::ROW2,
    gpio::p0::ROW3,
    gpio::p0::ROW4,
    gpio::p0::ROW5,
];

const SCREEN_COL_PINS: [gpio::GpioPin; 5] = [
    gpio::p0::COL1,
    gpio::p0::COL2,
    gpio::p0::COL3,
    gpio::p1::COL4,
    gpio::p0::COL5,
];

pub struct Hardware {
    pub screen: screens::EmbeddedScreen<5, 5>,
    pub left_button: left_button::LeftButton,
    pub right_button: right_button::RightButton,
    pub temp_sensor: temp_sensor::TempSensor,
}

impl Hardware {
    pub fn new() -> Self {
        Self {
            screen: screens::EmbeddedScreen::new(SCREEN_ROW_PINS, SCREEN_COL_PINS),
            left_button: left_button::LeftButton::new(),
            right_button: right_button::RightButton::new(),
            temp_sensor: temp_sensor::TempSensor::new(),
        }
    }
}
