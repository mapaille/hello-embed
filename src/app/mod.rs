#![allow(dead_code)]

pub mod cancellation;
pub mod hardware;
pub mod rtc_handler;
pub mod selected_program_id;

use crate::app::cancellation::CancellationToken;
use crate::app::hardware::Hardware;
use crate::app::selected_program_id::SelectedProgramId;
use crate::drivers::screens::animations::ANIMATION_LOADING;
use crate::interrupt::wfi;
use crate::programs::{LoveProgram, Program, ProgramId, StartupProgram, TemperatureProgram};
use crate::traits::{Cancellable, Displayable, Resettable};

static HARDWARE: Hardware = Hardware::new();
static CANCELLATION_TOKEN: CancellationToken = CancellationToken::new();
static SELECTED_PROGRAM_ID: SelectedProgramId = SelectedProgramId::new();

pub struct App {
    pub hardware: &'static Hardware,
    pub cancellation_token: &'static CancellationToken,
    pub selected_program_id: &'static SelectedProgramId,
}

impl App {
    pub const fn new() -> Self {
        Self {
            hardware: &HARDWARE,
            cancellation_token: &CANCELLATION_TOKEN,
            selected_program_id: &SELECTED_PROGRAM_ID,
        }
    }

    pub fn run(&self) -> ! {
        let startup_program = StartupProgram::new();
        let love_program = LoveProgram::new();
        let temperature_program = TemperatureProgram::new();

        self.hardware
            .screen
            .play_animation_for(&ANIMATION_LOADING, 30, 2, self.cancellation_token);

        loop {
            self.cancellation_token.reset();

            let program_id = self.selected_program_id.get();

            let program: Option<&dyn Program> = match program_id {
                ProgramId::Startup => Some(&startup_program),
                ProgramId::Love => Some(&love_program),
                ProgramId::Temperature => Some(&temperature_program),
                ProgramId::None => None,
            };

            if let Some(program) = program {
                program.run(self);
            }

            if !self.cancellation_token.is_cancelled() {
                self.selected_program_id.set(ProgramId::None);
            }

            wfi();
        }
    }
}
