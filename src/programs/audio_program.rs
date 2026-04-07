#[allow(clippy::wildcard_imports)]
use crate::drivers::audio::notes::*;
use crate::programs::Program;
use crate::timing::wait_ticks;
use crate::traits::Cancellable;

pub struct AudioProgram;

const MELODY: [u32; 8] = [
    NOTE_C4, NOTE_D4, NOTE_E4, NOTE_F4, NOTE_G4, NOTE_A4, NOTE_B4, NOTE_C5,
];

/// Duration of each note in RTC ticks (~500ms at 1024 Hz tick rate)
const NOTE_DURATION_TICKS: usize = 512;

impl Program for AudioProgram {
    fn run(&self, app: &crate::app::App) {
        let speaker = app.hardware.speaker;
        while !app.cancellation_token.is_cancelled() {
            for &note in &MELODY {
                if app.cancellation_token.is_cancelled() {
                    break;
                }
                speaker.set_frequency(note);
                wait_ticks(NOTE_DURATION_TICKS, app.cancellation_token);
            }
        }

        speaker.stop();
    }
}
