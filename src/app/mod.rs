#![allow(dead_code)]

pub mod cancellation;
pub mod hardware;
pub mod state;

use crate::drivers::screens::animations::ANIMATION_LOADING;
use crate::interrupt::wfi;
use crate::programs::{LoveProgram, Program, ProgramId, StartupProgram, TemperatureProgram};
use crate::traits::{Cancellable, Displayable, Resettable};

pub struct App {
    pub hardware: hardware::Hardware,
}

impl App {
    pub fn new(hardware: hardware::Hardware) -> Self {
        Self {
            hardware,
        }
    }

    pub fn run(&mut self, cancellation_token: &cancellation::CancellationToken) -> ! {
        let mut startup_program = StartupProgram::new();
        let mut love_program = LoveProgram::new();
        let mut temperature_program = TemperatureProgram::new();

        self.hardware
            .screen
            .play_animation_for(&ANIMATION_LOADING, 30, 2, cancellation_token);

        loop {
            cancellation_token.reset();

            let program_id = state::get_program_id();

            let program: Option<&mut dyn Program> = match program_id {
                ProgramId::Startup => Some(&mut startup_program),
                ProgramId::Love => Some(&mut love_program),
                ProgramId::Temperature => Some(&mut temperature_program),
                _ => None,
            };

            if let Some(program) = program {
                program.run(self, cancellation_token);
            }

            if !cancellation_token.is_cancelled() {
                state::set_program_id(ProgramId::None);
            }

            wfi()
        }
    }
}
