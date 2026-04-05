#[allow(clippy::wildcard_imports)]
use crate::drivers::audio::notes::*;
use crate::programs::Program;
use crate::timing::wait_ticks;
use crate::traits::Cancellable;

pub struct AudioProgram;

impl Program for AudioProgram {
    fn run(&self, app: &crate::app::App) {
        while !app.cancellation_token.is_cancelled() {
            app.hardware.speaker.beep(app.cancellation_token);
            wait_ticks(2000, app.cancellation_token);
        }
    }
}
