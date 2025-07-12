#![allow(dead_code)]

use core::sync::atomic::{AtomicU8, Ordering};
use crate::cancellation::CancellationTokenSource;
use crate::drivers::screen::animations::ANIMATION_LOADING;
use crate::drivers::screen::Screen;
use crate::interrupt::wfi;
use crate::programs::Program;
use crate::programs::love_program::LoveProgram;
use crate::programs::temp_program::TempProgram;

pub static ACTIVE_PROGRAM: AtomicU8 = AtomicU8::new(ActiveProgram::Startup as u8);
pub static CANCELLATION_TOKEN_SOURCE: CancellationTokenSource = CancellationTokenSource::new();

pub fn run(screen: &mut Screen<5, 5>) -> ! {
    screen.play_animation_for(&ANIMATION_LOADING, 30, 2);

    loop {
        let active_program = ACTIVE_PROGRAM.load(Ordering::Relaxed);

        if active_program == ActiveProgram::Love as u8 {
            LoveProgram.run(screen, &CANCELLATION_TOKEN_SOURCE.token);
        }
        else if active_program == ActiveProgram::Temp as u8 {
            TempProgram.run(screen, &CANCELLATION_TOKEN_SOURCE.token);
        }

        ACTIVE_PROGRAM.store(ActiveProgram::Startup as u8, Ordering::Relaxed);
        CANCELLATION_TOKEN_SOURCE.reset();
        wfi();
    }
}

pub enum ActiveProgram {
    Startup,
    Love,
    Temp,
}
