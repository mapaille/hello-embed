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
use crate::traits::{Resettable, Screen};

pub struct App {
    hardware: Hardware,
    love_program: LoveProgram,
    temp_program: TempProgram,
    startup_program: StartupProgram,
}

impl App {
    pub fn new() -> Self {
        Self {
            hardware: Hardware::new(),
            love_program: LoveProgram::new(),
            temp_program: TempProgram::new(),
            startup_program: StartupProgram::new(),
        }
    }

    pub fn run(&mut self, cancellation_token: &CancellationToken) -> ! {
        self.hardware
            .screen
            .play_animation_for(&ANIMATION_LOADING, 30, 2, cancellation_token);

        loop {
            cancellation_token.reset();

            let active_program = state::get_active_program();

            let program: Option<&mut dyn Program> = match active_program {
                1 => Some(&mut self.startup_program),
                2 => Some(&mut self.love_program),
                3 => Some(&mut self.temp_program),
                _ => None,
            };

            if let Some(program) = program {
                program.run(&mut self.hardware.screen, cancellation_token);
            }

            if !cancellation_token.is_cancelled() {
                state::clear_active_program();
            }

            interrupt::wfi()
        }
    }
}
