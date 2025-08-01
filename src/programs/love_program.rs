use crate::app::{App, cancellation};
use crate::drivers::screens::{EmbeddedScreen, animations};
use crate::programs::Program;
use crate::timing::wait_ticks;
use crate::traits::{Cancellable, Displayable};

pub struct LoveProgram;

impl LoveProgram {
    pub const fn new() -> Self {
        LoveProgram
    }
}

impl Program for LoveProgram {
    fn run(&mut self, app: &mut App, cancellation_token: &cancellation::CancellationToken) {
        if cancellation_token.is_cancelled() {
            return;
        }

        app.hardware
            .screen
            .play_animation_once(&animations::ANIMATION_LOVE, 2, cancellation_token);

        app.hardware.screen.play_animation_for(
            &animations::ANIMATION_HEARTBEAT,
            5,
            3,
            cancellation_token,
        );

        wait_ticks(500, cancellation_token);
    }
}
