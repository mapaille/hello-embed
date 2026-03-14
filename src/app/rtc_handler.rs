use core::sync::atomic::{AtomicUsize, Ordering};
use crate::app::cancellation::CancellationToken;
use crate::app::hardware::Hardware;
use crate::app::selected_program_id::SelectedProgramId;
use crate::app::{CANCELLATION_TOKEN, HARDWARE, SELECTED_PROGRAM_ID};
use crate::programs::ProgramId;
use crate::traits::{Cancellable, Pressable};

const BUTTON_PRESSED_DELAY_MS: usize = 50;
static LEFT_AND_RIGHT_BUTTON_PRESSED: AtomicUsize = AtomicUsize::new(0);
static LEFT_BUTTON_PRESSED_COUNTER: AtomicUsize = AtomicUsize::new(0);
static RIGHT_BUTTON_PRESSED_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct RtcHandler {
    hardware: &'static Hardware,
    cancellation_token: &'static CancellationToken,
    selected_program_id: &'static SelectedProgramId,
    available_programs: &'static [ProgramId],
}

impl RtcHandler {
    pub const fn new() -> Self {
        Self {
            hardware: &HARDWARE,
            cancellation_token: &CANCELLATION_TOKEN,
            selected_program_id: &SELECTED_PROGRAM_ID,
            available_programs: &[ProgramId::None, ProgramId::Startup, ProgramId::Love, ProgramId::Temperature],
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
        let left_button_pressed = self.hardware.left_button.is_pressed();
        let right_button_pressed = self.hardware.right_button.is_pressed();

        if left_button_pressed && right_button_pressed {
            LEFT_AND_RIGHT_BUTTON_PRESSED.fetch_add(1, Ordering::Relaxed);
            LEFT_BUTTON_PRESSED_COUNTER.store(0, Ordering::Relaxed);
            RIGHT_BUTTON_PRESSED_COUNTER.store(0, Ordering::Relaxed);
        } else if left_button_pressed {
            LEFT_AND_RIGHT_BUTTON_PRESSED.store(0, Ordering::Relaxed);
            LEFT_BUTTON_PRESSED_COUNTER.fetch_add(1, Ordering::Relaxed);
            RIGHT_BUTTON_PRESSED_COUNTER.store(0, Ordering::Relaxed);
        } else if right_button_pressed {
            LEFT_AND_RIGHT_BUTTON_PRESSED.store(0, Ordering::Relaxed);
            LEFT_BUTTON_PRESSED_COUNTER.store(0, Ordering::Relaxed);
            RIGHT_BUTTON_PRESSED_COUNTER.fetch_add(1, Ordering::Relaxed);
        } else {
            LEFT_AND_RIGHT_BUTTON_PRESSED.store(0, Ordering::Relaxed);
            LEFT_BUTTON_PRESSED_COUNTER.store(0, Ordering::Relaxed);
            RIGHT_BUTTON_PRESSED_COUNTER.store(0, Ordering::Relaxed);
        }

        let left_and_right_buttons_pressed_for_long_enough = LEFT_AND_RIGHT_BUTTON_PRESSED.load(Ordering::Relaxed) == BUTTON_PRESSED_DELAY_MS;
        let left_button_pressed_for_long_enough = LEFT_BUTTON_PRESSED_COUNTER.load(Ordering::Relaxed) == BUTTON_PRESSED_DELAY_MS;
        let right_button_pressed_for_long_enough = RIGHT_BUTTON_PRESSED_COUNTER.load(Ordering::Relaxed) == BUTTON_PRESSED_DELAY_MS;

        if left_and_right_buttons_pressed_for_long_enough {
            ProgramId::Startup
        } else if left_button_pressed_for_long_enough {
            self.get_previous_program_id()
        } else if right_button_pressed_for_long_enough {
            self.get_next_program_id()
        } else if self.hardware.touch_button.is_pressed() {
            ProgramId::Startup
        } else {
            self.selected_program_id.get()
        }
    }

    #[inline]
    fn get_previous_program_id(&self) -> ProgramId {
        let selected_program_index = self.get_selected_program_index();

        if selected_program_index == 0 {
            self.get_last_program_id()
        } else {
            self.available_programs[selected_program_index - 1]
        }
    }

    fn get_next_program_id(&self) -> ProgramId {
        let selected_program_index = self.get_selected_program_index();

        if selected_program_index == self.available_programs.len() - 1 {
            self.get_first_program_id()
        } else {
            self.available_programs[selected_program_index + 1]
        }
    }

    fn get_selected_program_index(&self) -> usize {
        self.available_programs.iter().position(|id| id == &self.selected_program_id.get()).unwrap()
    }

    fn get_first_program_id(&self) -> ProgramId {
        self.available_programs[0]
    }

    fn get_last_program_id(&self) -> ProgramId {
        self.available_programs[self.available_programs.len() - 1]
    }
}
