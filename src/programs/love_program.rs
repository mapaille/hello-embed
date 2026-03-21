use crate::app::App;
use crate::drivers::screens::{EmbeddedScreen, animations};
use crate::programs::Program;
use crate::timing::wait_ticks;
use crate::traits::{Cancellable, Displayable};

pub struct LoveProgram;

impl Program for LoveProgram {
    fn run(&self, app: &App) {
        while !app.cancellation_token.is_cancelled() {
            app.hardware.screen.play_animation_once(
                &animations::ANIMATION_LOVE,
                2,
                app.cancellation_token,
            );

            app.hardware.screen.play_animation_for(
                &animations::ANIMATION_HEARTBEAT,
                5,
                3,
                app.cancellation_token,
            );

            wait_ticks(10000, app.cancellation_token);
        }
    }
}
