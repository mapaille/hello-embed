use crate::drivers::screen::{Screen, animations};
use crate::programs::{CancellationToken, Program};

pub struct StartupProgram;

impl Program for StartupProgram {
    fn run(&mut self, screen: &mut Screen<5, 5>, cancellation_token: &CancellationToken) {
        while !cancellation_token.is_cancelled() {
            screen.play_animation_once(&animations::ANIMATION_LOADING, 30);
        }
    }
}
