use crate::app::App;
use crate::drivers::display::{EmbeddedScreen, frames};
use crate::interrupt;
use crate::programs::Program;
use crate::timing::wait_ticks;
use crate::traits::{Cancellable, Clearable, Displayable};
use core::char::MAX;

pub struct TemperatureProgram;

const DISPLAY_DURATION_MS: usize = 1000;
const MAX_DISPLAYABLE_TEMP: u32 = 100;
const DIGIT_BASE: u32 = 10;

impl Program for TemperatureProgram {
    fn run(&self, app: &App) {
        while !app.cancellation_token.is_cancelled() {
            if read_and_display_temperature(app).is_none() {
                app.hardware.screen.refresh_for(
                    &frames::LETTER_X,
                    DISPLAY_DURATION_MS,
                    app.cancellation_token,
                );
            }

            app.hardware.screen.clear();
            wait_ticks(10000, app.cancellation_token);
        }
    }
}

fn read_and_display_temperature(app: &App) -> Option<()> {
    let Some(temperature) = app.hardware.thermometer.read_temperature() else {
        app.hardware.screen.refresh_for(
            &frames::LETTER_X,
            DISPLAY_DURATION_MS,
            app.cancellation_token,
        );
        return None;
    };

    if temperature >= MAX_DISPLAYABLE_TEMP {
        return None;
    }

    display_temperature_digits(app, temperature)?;
    Some(())
}

/// Display a single digit on the screen for the specified duration
fn display_digit(app: &App, digit_index: u32) -> Option<()> {
    let digit = frames::get_digit(digit_index)?;
    app.hardware
        .screen
        .refresh_for(digit, DISPLAY_DURATION_MS, app.cancellation_token);
    Some(())
}

fn display_temperature_digits(app: &App, temperature: u32) -> Option<()> {
    if temperature >= MAX_DISPLAYABLE_TEMP {
        return None;
    }

    // Display first digit (tens place)
    display_digit(app, temperature / DIGIT_BASE)?;

    // Display second digit (ones place)
    display_digit(app, temperature % DIGIT_BASE)?;

    Some(())
}
