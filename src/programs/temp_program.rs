use crate::app::hardware::TemperatureSensor;
use crate::app::{App, cancellation, hardware};
use crate::drivers::screens::{EmbeddedScreen, frames};
use crate::interrupt;
use crate::programs::RunnableProgram;
use crate::timing::wait_ticks;
use crate::traits::Displayable;
use core::char::MAX;

pub struct TempProgram;

const DISPLAY_DURATION_MS: u32 = 500;
const MAX_DISPLAYABLE_TEMP: u32 = 100;
const DIGIT_BASE: u32 = 10;

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

        if read_and_display_temperature(app, cancellation_token).is_none() {
            app.hardware.screen.refresh_for(
                &frames::LETTER_X,
                DISPLAY_DURATION_MS,
                cancellation_token,
            );
        }

        app.hardware.screen.clear();
        wait_ticks(500, cancellation_token);
    }
}

fn read_and_display_temperature(
    app: &mut App,
    cancellation_token: &cancellation::CancellationToken,
) -> Option<()> {
    let temperature = app.hardware.temp_sensor.read_temperature().unwrap();

    if temperature >= MAX_DISPLAYABLE_TEMP {
        return None;
    }

    display_temperature_digits(app, temperature, cancellation_token)?;
    Some(())
}

fn display_temperature_digits(
    app: &mut App,
    temperature: u32,
    cancellation_token: &cancellation::CancellationToken,
) -> Option<()> {
    if temperature < MAX_DISPLAYABLE_TEMP {
        let first_digit = frames::get_digit(temperature / DIGIT_BASE).unwrap();
        let second_digit = frames::get_digit(temperature % DIGIT_BASE).unwrap();

        app.hardware
            .screen
            .refresh_for(first_digit, DISPLAY_DURATION_MS, cancellation_token);

        app.hardware
            .screen
            .refresh_for(second_digit, DISPLAY_DURATION_MS, cancellation_token);

        Some(())
    } else {
        None
    }
}
