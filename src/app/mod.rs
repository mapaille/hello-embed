#![allow(dead_code)]

pub mod cancellation_token;
pub mod hardware;
pub mod rtc_handler;

use crate::app::cancellation_token::CancellationToken;
use crate::app::hardware::Hardware;
use crate::interrupt::wfi;
use crate::traits::{Cancellable, Resettable};
use core::sync::atomic::{AtomicUsize, Ordering};

const HARDWARE: Hardware = Hardware::new();
static CANCELLATION_TOKEN: CancellationToken = CancellationToken::new();
static SELECTED_PROGRAM_INDEX: AtomicUsize = AtomicUsize::new(0);

pub struct App {
    pub hardware: &'static Hardware,
    pub cancellation_token: &'static CancellationToken,
    pub selected_program_index: &'static AtomicUsize,
}

impl App {
    pub const fn new() -> Self {
        Self {
            hardware: &HARDWARE,
            cancellation_token: &CANCELLATION_TOKEN,
            selected_program_index: &SELECTED_PROGRAM_INDEX,
        }
    }

    pub fn run(&self) -> ! {
        let programs = crate::programs::get_programs();

        loop {
            programs.iter().enumerate().for_each(|(i, program)| {
                if i == self.selected_program_index.load(Ordering::Relaxed) {
                    program.run(self);
                }
            });

            if !self.cancellation_token.is_cancelled() {
                self.selected_program_index.store(0, Ordering::Relaxed);
            }

            self.cancellation_token.reset();
            wfi();
        }
    }
}
