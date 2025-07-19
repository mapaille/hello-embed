#![allow(dead_code)]

pub mod cancellation;
pub mod hardware;
pub mod state;

use crate::drivers::screens::animations::ANIMATION_LOADING;
use crate::interrupt;
use crate::programs;
use crate::traits::{Displayable, Resettable};

pub struct App {
    pub hardware: hardware::Hardware,
}

impl App {
    pub fn new() -> Self {
        Self {
            hardware: hardware::Hardware::new(),
        }
    }

    pub fn run(&mut self, cancellation_token: &cancellation::CancellationToken) -> ! {
        let mut startup_program = programs::StartupProgram::new();
        let mut love_program = programs::LoveProgram::new();
        let mut temp_program = programs::TempProgram::new();

        self.hardware
            .screen
            .play_animation_for(&ANIMATION_LOADING, 30, 2, cancellation_token);

        loop {
            cancellation_token.reset();

            let program_id = state::get_program_id();

            let program: Option<&mut dyn programs::Program> = match program_id {
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
