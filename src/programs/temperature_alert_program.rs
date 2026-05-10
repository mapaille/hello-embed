use crate::app::App;
use crate::drivers::display::animations::ANIMATION_LOADING;
use crate::drivers::display::frames::{LETTER_T, LETTER_X};
use crate::programs::Program;
use crate::timing::wait_ticks;
use crate::traits::{Cancellable, Displayable};

pub struct TemperatureAlertProgram;

const TEMPERATURE_THRESHOLD: u32 = 30;
const SOUND_FREQUENCY: u32 = 2700;
const ALERT_DURATION: usize = 1000;
const ERROR_DURATION: usize = 1000;
const WAIT_DURATION: usize = 1000;

impl Program for TemperatureAlertProgram {
    fn run(&self, app: &App) {
        app.hardware.speaker.stop();
        while !app.cancellation_token.is_cancelled() {
            match app.hardware.thermometer.read_temperature() {
                Some(temperature) => {
                    if temperature <= TEMPERATURE_THRESHOLD {
                        app.hardware.speaker.set_frequency(SOUND_FREQUENCY);
                        app.hardware.screen.refresh_for(
                            &LETTER_T,
                            ALERT_DURATION,
                            app.cancellation_token,
                        );
                        app.hardware.speaker.stop();
                    }
                }
                None => {
                    app.hardware.screen.refresh_for(
                        &LETTER_X,
                        ERROR_DURATION,
                        app.cancellation_token,
                    );
                }
            }

            wait_ticks(WAIT_DURATION, app.cancellation_token);
        }
    }
}
