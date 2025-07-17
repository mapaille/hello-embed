use crate::cancellation::CancellationToken;
use core::sync::atomic::AtomicU8;

pub const STARTUP_PROGRAM_ID: u8 = 1;
pub const LOVE_PROGRAM_ID: u8 = 2;
pub const TEMP_PROGRAM_ID: u8 = 3;

static PROGRAM_ID: AtomicU8 = AtomicU8::new(STARTUP_PROGRAM_ID);
static CANCELLATION_TOKEN: CancellationToken = CancellationToken::new();

pub fn set_program_id(program_id: u8) {
    PROGRAM_ID.store(program_id, core::sync::atomic::Ordering::Relaxed)
}

pub fn clear_program_id() {
    PROGRAM_ID.store(0, core::sync::atomic::Ordering::Relaxed)
}

pub fn get_program_id() -> u8 {
    PROGRAM_ID.load(core::sync::atomic::Ordering::Relaxed)
}

pub fn get_cancellation_token() -> &'static CancellationToken {
    &CANCELLATION_TOKEN
}
