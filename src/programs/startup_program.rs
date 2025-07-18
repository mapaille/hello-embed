use crate::app::{App, cancellation};
use crate::drivers::screens::{EmbeddedScreen, animations};
use crate::programs::RunnableProgram;
use crate::traits::Displayable;

pub struct StartupProgram;

impl StartupProgram {
    pub const fn new() -> Self {
        StartupProgram
    }
}

impl RunnableProgram for StartupProgram {
    fn run(&mut self, app: &mut App, cancellation_token: &cancellation::CancellationToken) {
        while !cancellation_token.is_cancelled() {
            app.hardware.screen.play_animation_once(
                &animations::ANIMATION_LOADING,
                30,
                cancellation_token,
            );
        }
    }
}
