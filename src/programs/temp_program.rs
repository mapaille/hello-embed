use crate::app::hardware::TemperatureSensor;
use crate::app::{App, cancellation, hardware};
use crate::drivers::screens::{EmbeddedScreen, frames};
use crate::interrupt;
use crate::programs::RunnableProgram;
use crate::timing::wait_ticks;
use crate::traits::Displayable;

pub struct TempProgram;

impl TempProgram {
    pub const fn new() -> Self {
        Self
    }
}

impl RunnableProgram for TempProgram {
    fn run(&mut self, app: &mut App, cancellation_token: &cancellation::CancellationToken) {
        if cancellation_token.is_cancelled() {
            return;
        }

        let temperature = app.hardware.temp_sensor.read_temperature();

        if let Some(temperature) = temperature {
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
        } else {
            app.hardware
                .screen
                .refresh_for(&frames::LETTER_X, 500, cancellation_token);
        }

        app.hardware.screen.clear();
        wait_ticks(500, cancellation_token);
    }
}
