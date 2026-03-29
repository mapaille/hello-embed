use crate::app::cancellation_token::CancellationToken;
use crate::app::hardware::Hardware;
use crate::app::{CANCELLATION_TOKEN, HARDWARE, SELECTED_PROGRAM_INDEX};
use crate::traits::{Cancellable, Pressable};
use core::sync::atomic::{AtomicUsize, Ordering};

const PRESS_DEBOUNCE_TICKS: usize = 50;

static LEFT_PRESS_COUNTER: AtomicUsize = AtomicUsize::new(0);
static RIGHT_PRESS_COUNTER: AtomicUsize = AtomicUsize::new(0);

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
        let mut new_index = self.selected_program_index.load(Ordering::Acquire);

        if self.hardware.left_button.is_pressed() {
            let prev = LEFT_PRESS_COUNTER.fetch_add(1, Ordering::Relaxed);

            if prev == PRESS_DEBOUNCE_TICKS - 1 {
                new_index = get_previous_program_index(
                    new_index,
                    crate::programs::get_number_of_programs(),
                );
            }
        } else {
            LEFT_PRESS_COUNTER.store(0, Ordering::Relaxed);
        }

        if self.hardware.right_button.is_pressed() {
            let prev = RIGHT_PRESS_COUNTER.fetch_add(1, Ordering::Relaxed);

            if prev == PRESS_DEBOUNCE_TICKS - 1 {
                new_index =
                    get_next_program_index(new_index, crate::programs::get_number_of_programs());
            }
        } else {
            RIGHT_PRESS_COUNTER.store(0, Ordering::Relaxed);
        }

        if new_index != self.selected_program_index.load(Ordering::Relaxed) {
            self.selected_program_index
                .store(new_index, Ordering::Release);
            self.cancellation_token.cancel();
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
