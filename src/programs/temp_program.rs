use crate::app::App;
use crate::drivers::screens::{EmbeddedScreen, frames};
use crate::interrupt;
use crate::programs::{CancellationToken, Program};
use crate::timing::wait_ticks;
use crate::traits::Displayable;

pub struct TempProgram;

impl TempProgram {
    pub const fn new() -> Self {
        Self
    }
}

impl Program for TempProgram {
    fn run(&mut self, app: &mut App, cancellation_token: &CancellationToken) {
        if cancellation_token.is_cancelled() {
            return;
        }

        let sensor_temperature = app.hardware.temp_sensor.read();

        // Round to nearest whole number: add half the divisor (2) before dividing by 4
        let temperature = (sensor_temperature + 2) / 4;

        if temperature < 100 {
            let first_digit = frames::get_digit(temperature / 10).unwrap_or(&frames::DIGIT_0);
            let second_digit = frames::get_digit(temperature % 10).unwrap_or(&frames::DIGIT_0);

            app.hardware
                .screen
                .refresh_for(first_digit, 500, cancellation_token);
            app.hardware
                .screen
                .refresh_for(second_digit, 500, cancellation_token);
        }

        app.hardware.screen.clear();
        wait_ticks(500, cancellation_token);
    }
}
