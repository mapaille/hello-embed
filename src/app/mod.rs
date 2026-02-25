#![allow(dead_code)]

pub mod cancellation;
pub mod hardware;
mod selected_program_id;

use crate::app::cancellation::CancellationToken;
use crate::app::hardware::Hardware;
use crate::app::selected_program_id::SelectedProgramId;
use crate::drivers::screens::animations::ANIMATION_LOADING;
use crate::interrupt::wfi;
use crate::programs::{LoveProgram, Program, ProgramId, StartupProgram, TemperatureProgram};
use crate::traits::{Cancellable, Displayable, Pressable, Resettable};

static HARDWARE: Hardware = Hardware::new();
static CANCELLATION_TOKEN: CancellationToken = CancellationToken::new();
static SELECTED_PROGRAM_ID: SelectedProgramId = SelectedProgramId::new();

pub struct App {
    pub hardware: &'static Hardware,
    pub cancellation_token: &'static CancellationToken,
    pub selected_program_id: &'static SelectedProgramId,
}

pub struct RtcHandler {
    hardware: &'static Hardware,
    cancellation_token: &'static CancellationToken,
    selected_program_id: &'static SelectedProgramId,
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

impl RtcHandler {
    pub const fn new() -> Self {
        Self {
            hardware: &HARDWARE,
            cancellation_token: &CANCELLATION_TOKEN,
            selected_program_id: &SELECTED_PROGRAM_ID,
        }
    }

    pub fn on_rtc(&self) {
        let program_id = self.selected_program_id.get();
        let new_program_id = self.determine_program_id_from_buttons();

        if program_id != new_program_id {
            self.selected_program_id.set(new_program_id);
            self.cancellation_token.cancel();
        }
    }

    fn determine_program_id_from_buttons(&self) -> ProgramId {
        if self.hardware.left_button.is_pressed() && self.hardware.right_button.is_pressed() {
            ProgramId::Startup
        } else if self.hardware.left_button.is_pressed() {
            ProgramId::Love
        } else if self.hardware.right_button.is_pressed() {
            ProgramId::Temperature
        } else if self.hardware.touch_button.is_pressed() {
            ProgramId::Startup
        } else {
            self.selected_program_id.get()
        }
    }
}
