use crate::app::App;
use crate::drivers::screens::frames::LETTER_X;
use crate::interrupt::wfi;
use crate::programs::Program;
use crate::timing::wait_ticks;
use crate::traits::{Cancellable, Displayable};

pub struct XProgram;

impl Program for XProgram {
    fn run(&self, app: &App) {
        let tick_duration: usize = 1000;

        while !app.cancellation_token.is_cancelled() {
            app.hardware
                .screen
                .refresh_for(&LETTER_X, tick_duration, app.cancellation_token);

            wait_ticks(tick_duration, app.cancellation_token);
        }
    }
}
