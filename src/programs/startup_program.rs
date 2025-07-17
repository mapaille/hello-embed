use crate::app::App;
use crate::drivers::screens::{EmbeddedScreen, animations};
use crate::programs::{CancellationToken, Program};
use crate::traits::Displayable;

pub struct StartupProgram;

impl StartupProgram {
    pub const fn new() -> Self {
        StartupProgram
    }
}

impl Program for StartupProgram {
    fn run(&mut self, app: &mut App, cancellation_token: &CancellationToken) {
        while !cancellation_token.is_cancelled() {
            app.hardware.screen.play_animation_once(
                &animations::ANIMATION_LOADING,
                30,
                cancellation_token,
            );
        }
    }
}
