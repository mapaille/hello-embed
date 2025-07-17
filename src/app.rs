#![allow(dead_code)]

use crate::cancellation::CancellationToken;
use crate::drivers::screens::animations::ANIMATION_LOADING;
use crate::hardware::Hardware;
use crate::interrupt;
use crate::programs::Program;
use crate::programs::love_program::LoveProgram;
use crate::programs::startup_program::StartupProgram;
use crate::programs::temp_program::TempProgram;
use crate::state;
use crate::traits::{Displayable, Resettable};

pub struct App {
    pub hardware: Hardware,
}

impl App {
    pub fn new() -> Self {
        Self {
            hardware: Hardware::new(),
        }
    }

    pub fn run(&mut self, cancellation_token: &CancellationToken) -> ! {
        let mut startup_program = StartupProgram::new();
        let mut love_program = LoveProgram::new();
        let mut temp_program = TempProgram::new();

        self.hardware
            .screen
            .play_animation_for(&ANIMATION_LOADING, 30, 2, cancellation_token);

        loop {
            cancellation_token.reset();

            let program_id = state::get_program_id();

            let program: Option<&mut dyn Program> = match program_id {
                state::STARTUP_PROGRAM_ID => Some(&mut startup_program),
                state::LOVE_PROGRAM_ID => Some(&mut love_program),
                state::TEMP_PROGRAM_ID => Some(&mut temp_program),
                _ => None,
            };

            if let Some(program) = program {
                program.run(self, cancellation_token);
            }

            if !cancellation_token.is_cancelled() {
                state::clear_program_id();
            }

            interrupt::wfi()
        }
    }
}
