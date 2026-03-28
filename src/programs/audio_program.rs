#[allow(clippy::wildcard_imports)]
use crate::drivers::audio::*;
use crate::programs::Program;
use crate::timing::wait_ticks;
use crate::traits::Cancellable;

pub struct AudioProgram;

impl Program for AudioProgram {
    fn run(&self, app: &crate::app::App) {
        while !app.cancellation_token.is_cancelled() {
            let duration_ms = 100;

            app.hardware.speaker.play_tone(NOTE_C4, duration_ms);
            app.hardware.speaker.play_tone(NOTE_D4, duration_ms);
            app.hardware.speaker.play_tone(NOTE_E4, duration_ms);
            app.hardware.speaker.play_tone(NOTE_F4, duration_ms);
            app.hardware.speaker.play_tone(NOTE_G4, duration_ms);
            app.hardware.speaker.play_tone(NOTE_A4, duration_ms);
            app.hardware.speaker.play_tone(NOTE_B4, duration_ms);
            app.hardware.speaker.play_tone(NOTE_C5, duration_ms);
            app.hardware.speaker.play_tone(NOTE_D5, duration_ms);
            app.hardware.speaker.play_tone(NOTE_E5, duration_ms);
            app.hardware.speaker.play_tone(NOTE_F5, duration_ms);
            app.hardware.speaker.play_tone(NOTE_G5, duration_ms);
            app.hardware.speaker.play_tone(NOTE_A5, duration_ms);
            app.hardware.speaker.play_tone(NOTE_B5, duration_ms);

            wait_ticks(2000, app.cancellation_token);
        }
    }
}
