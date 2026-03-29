use crate::app::App;
use crate::drivers::display::{EmbeddedScreen, animations};
use crate::programs::Program;
use crate::traits::{Cancellable, Displayable};

pub struct StartupProgram;

impl Program for StartupProgram {
    fn run(&self, app: &App) {
        while !app.cancellation_token.is_cancelled() {
            app.hardware.screen.play_animation_once(
                &animations::ANIMATION_LOADING,
                30,
                app.cancellation_token,
            );
        }
    }
}
