use crate::cancellation::CancellationToken;
use core::sync::atomic::AtomicU8;

pub const STARTUP_PROGRAM_ID: u8 = 1;
pub const LOVE_PROGRAM_ID: u8 = 2;
pub const TEMP_PROGRAM_ID: u8 = 3;

static ACTIVE_PROGRAM: AtomicU8 = AtomicU8::new(STARTUP_PROGRAM_ID);
static CANCELLATION_TOKEN: CancellationToken = CancellationToken::new();

pub fn set_active_program(program: u8) {
    ACTIVE_PROGRAM.store(program, core::sync::atomic::Ordering::Relaxed)
}

pub fn clear_active_program() {
    ACTIVE_PROGRAM.store(0, core::sync::atomic::Ordering::Relaxed)
}

pub fn get_active_program() -> u8 {
    ACTIVE_PROGRAM.load(core::sync::atomic::Ordering::Relaxed)
}

pub fn get_cancellation_token() -> &'static CancellationToken {
    &CANCELLATION_TOKEN
}
