#![allow(dead_code)]

use crate::cancellation::CancellationTokenSource;
use crate::drivers::screen::Screen;
use crate::drivers::screen::animations::ANIMATION_LOADING;
use crate::interrupt::wfi;
use crate::programs::Program;
use crate::programs::love_program::LoveProgram;
use crate::programs::startup_program::StartupProgram;
use crate::programs::temp_program::TempProgram;
use core::sync::atomic::{AtomicU8, Ordering};

pub static ACTIVE_PROGRAM: AtomicU8 = AtomicU8::new(ActiveProgram::Startup as u8);
pub static CANCELLATION_TOKEN_SOURCE: CancellationTokenSource = CancellationTokenSource::new();

pub fn run(screen: &mut Screen<5, 5>) -> ! {
    screen.play_animation_for(&ANIMATION_LOADING, 30, 2, &CANCELLATION_TOKEN_SOURCE.token);

    loop {
        CANCELLATION_TOKEN_SOURCE.reset();

        let active_program = ACTIVE_PROGRAM.load(Ordering::Relaxed);

        if active_program == ActiveProgram::Love as u8 {
            LoveProgram.run(screen, &CANCELLATION_TOKEN_SOURCE.token);
        } else if active_program == ActiveProgram::Temp as u8 {
            TempProgram.run(screen, &CANCELLATION_TOKEN_SOURCE.token);
        } else if active_program == ActiveProgram::Startup as u8 {
            StartupProgram.run(screen, &CANCELLATION_TOKEN_SOURCE.token);
        }

        if CANCELLATION_TOKEN_SOURCE.token.is_cancelled() {
            continue;
        }

        ACTIVE_PROGRAM.store(ActiveProgram::None as u8, Ordering::Relaxed);
        wfi();
    }
}

pub enum ActiveProgram {
    None = 0,
    Startup = 1,
    Love = 2,
    Temp = 3,
}
