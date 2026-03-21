use crate::app::cancellation::CancellationToken;
use crate::app::hardware::Hardware;
use crate::app::{CANCELLATION_TOKEN, HARDWARE, SELECTED_PROGRAM_INDEX};
use crate::traits::{Cancellable, Pressable};
use core::sync::atomic::{AtomicUsize, Ordering};

const BUTTON_PRESSED_DELAY_MS: usize = 50;
static LEFT_AND_RIGHT_BUTTON_PRESSED: AtomicUsize = AtomicUsize::new(0);
static LEFT_BUTTON_PRESSED_COUNTER: AtomicUsize = AtomicUsize::new(0);
static RIGHT_BUTTON_PRESSED_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct RtcHandler {
    hardware: &'static Hardware,
    cancellation_token: &'static CancellationToken,
    selected_program_index: &'static AtomicUsize,
}

impl RtcHandler {
    pub const fn new() -> Self {
        Self {
            hardware: &HARDWARE,
            cancellation_token: &CANCELLATION_TOKEN,
            selected_program_index: &SELECTED_PROGRAM_INDEX,
        }
    }

    pub fn on_rtc(&self) {
        let program_index = self.selected_program_index.load(Ordering::Relaxed);
        let new_program_index = self.determine_program_index_from_buttons();

        if program_index != new_program_index {
            self.selected_program_index
                .store(new_program_index, Ordering::Relaxed);
            self.cancellation_token.cancel();
        }
    }

    fn determine_program_index_from_buttons(&self) -> usize {
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

        let left_button_pressed_for_long_enough =
            LEFT_BUTTON_PRESSED_COUNTER.load(Ordering::Relaxed) == BUTTON_PRESSED_DELAY_MS;
        let right_button_pressed_for_long_enough =
            RIGHT_BUTTON_PRESSED_COUNTER.load(Ordering::Relaxed) == BUTTON_PRESSED_DELAY_MS;
        let selected_program_index = self.selected_program_index.load(Ordering::Relaxed);
        let number_of_programs = crate::programs::get_number_of_programs();

        if left_button_pressed_for_long_enough {
            get_previous_program_index(selected_program_index, number_of_programs)
        } else if right_button_pressed_for_long_enough {
            get_next_program_index(selected_program_index, number_of_programs)
        } else {
            selected_program_index
        }
    }
}

#[inline]
const fn get_previous_program_index(
    selected_program_index: usize,
    number_of_programs: usize,
) -> usize {
    if selected_program_index == 0 {
        number_of_programs - 1
    } else {
        selected_program_index - 1
    }
}

#[inline]
const fn get_next_program_index(selected_program_index: usize, number_of_programs: usize) -> usize {
    if selected_program_index == number_of_programs - 1 {
        0
    } else {
        selected_program_index + 1
    }
}
