use crate::app::cancellation::CancellationToken;
use crate::app::hardware::Hardware;
use crate::app::selected_program_id::SelectedProgramId;
use crate::app::{CANCELLATION_TOKEN, HARDWARE, SELECTED_PROGRAM_ID};
use crate::programs::ProgramId;
use crate::traits::{Cancellable, Pressable};

pub struct RtcHandler {
    hardware: &'static Hardware,
    cancellation_token: &'static CancellationToken,
    selected_program_id: &'static SelectedProgramId,
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
