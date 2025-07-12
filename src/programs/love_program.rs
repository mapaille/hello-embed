use crate::drivers::screen::{Screen, animations};
use crate::programs::{CancellationToken, Program};
use crate::timing::wait_ticks;

pub struct LoveProgram;

impl Program for LoveProgram {
    fn run(&mut self, screen: &mut Screen<5, 5>, cancellation_token: &CancellationToken) {
        if cancellation_token.is_cancelled() {
            return;
        }

        screen.play_animation_once(&animations::ANIMATION_LOVE, 2);
        screen.play_animation_for(&animations::ANIMATION_HEARTBEAT, 5, 3);
        wait_ticks(500, cancellation_token);
    }
}
