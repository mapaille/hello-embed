#[allow(clippy::wildcard_imports)]
use crate::drivers::audio::notes::*;
use crate::programs::Program;
use crate::timing::wait_ticks;
use crate::traits::Cancellable;

pub struct AudioProgram;

impl Program for AudioProgram {
    fn run(&self, app: &crate::app::App) {
        app.hardware.speaker.start(app.cancellation_token);

        while !app.cancellation_token.is_cancelled() {
            wait_ticks(100, app.cancellation_token); // low CPU via wfi
        }

        app.hardware.speaker.stop();
    }
}
