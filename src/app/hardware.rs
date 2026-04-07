use crate::drivers::audio::Speaker;
use crate::drivers::button::Button;
use crate::drivers::display::EmbeddedScreen;
use crate::drivers::temperature_sensor::TemperatureSensor;
use crate::peripherals::gpio;
use crate::peripherals::gpio::GpioPin;
use crate::peripherals::pwm::PWM0;

const TEMPERATURE_SENSOR_MAX_ATTEMPS: usize = 10;
const LEFT_BUTTON_PIN: GpioPin = gpio::p0::BTN_A;
const RIGHT_BUTTON_PIN: GpioPin = gpio::p0::BTN_B;
const FACE_TOUCH_PIN: GpioPin = gpio::p1::FACE_TOUCH;
const SPEAKER_PIN: GpioPin = gpio::p0::SPEAKER;

const SCREEN_ROW_PINS: [GpioPin; 5] = [
    gpio::p0::ROW1,
    gpio::p0::ROW2,
    gpio::p0::ROW3,
    gpio::p0::ROW4,
    gpio::p0::ROW5,
];

const SCREEN_COL_PINS: [GpioPin; 5] = [
    gpio::p0::COL1,
    gpio::p0::COL2,
    gpio::p0::COL3,
    gpio::p1::COL4,
    gpio::p0::COL5,
];

const LEFT_BUTTON: Button = Button::new(LEFT_BUTTON_PIN);
const RIGHT_BUTTON: Button = Button::new(RIGHT_BUTTON_PIN);
const TEMPERATURE_SENSOR: TemperatureSensor =
    TemperatureSensor::new(TEMPERATURE_SENSOR_MAX_ATTEMPS);
const SCREEN: EmbeddedScreen<5, 5> = EmbeddedScreen::new(SCREEN_ROW_PINS, SCREEN_COL_PINS);
static SPEAKER: Speaker = Speaker::new(&SPEAKER_PIN, &PWM0);

pub struct Hardware {
    pub screen: EmbeddedScreen<5, 5>,
    pub left_button: Button,
    pub right_button: Button,
    pub temperature_sensor: TemperatureSensor,
    pub speaker: &'static Speaker,
}

impl Hardware {
    pub const fn new() -> Self {
        Self {
            screen: SCREEN,
            left_button: LEFT_BUTTON,
            right_button: RIGHT_BUTTON,
            temperature_sensor: TEMPERATURE_SENSOR,
            speaker: &SPEAKER,
        }
    }

    pub fn init(&self) {
        self.left_button.init();
        self.right_button.init();
        self.screen.init();
        self.speaker.init();
    }
}
