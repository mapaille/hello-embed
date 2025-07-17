use crate::drivers::screens::{EmbeddedScreen, animations};
use crate::programs::{CancellationToken, Program};
use crate::traits::Screen;

pub struct StartupProgram;

impl StartupProgram {
    pub fn new() -> Self {
        StartupProgram
    }
}

impl Program for StartupProgram {
    fn run(&mut self, screen: &mut EmbeddedScreen<5, 5>, cancellation_token: &CancellationToken) {
        while !cancellation_token.is_cancelled() {
            screen.play_animation_once(&animations::ANIMATION_LOADING, 30, cancellation_token);
        }
    }
}
