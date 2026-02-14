#![allow(unused)]

mod left_button;
mod right_button;
mod temp_sensor;

pub use temp_sensor::TemperatureSensor;

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

const TEMP_SENSOR_MAX_ATTEMPTS: usize = 10;

pub struct Hardware {
    pub screen: screens::EmbeddedScreen<5, 5>,
    pub left_button: left_button::LeftButton,
    pub right_button: right_button::RightButton,
    pub temp_sensor: temp_sensor::TempSensor,
}

impl Hardware {
    pub const fn new() -> Self {
        Self {
            screen: screens::EmbeddedScreen::new(SCREEN_ROW_PINS, SCREEN_COL_PINS),
            left_button: left_button::LeftButton::new(),
            right_button: right_button::RightButton::new(),
            temp_sensor: temp_sensor::TempSensor::new(TEMP_SENSOR_MAX_ATTEMPTS),
        }
    }

    pub fn init(&mut self) {
        self.screen.init();
    }
}
