use crate::app::cancellation::CancellationToken;
use crate::programs::ProgramId;
use core::convert::Into;
use core::sync::atomic::{AtomicU8, Ordering};

static CANCELLATION_TOKEN: CancellationToken = CancellationToken::new();
static PROGRAM_ID: AtomicU8 = AtomicU8::new(ProgramId::Startup as u8);

pub fn set_program_id(program_id: ProgramId) {
    PROGRAM_ID.store(program_id.into(), Ordering::Relaxed)
}

pub fn get_program_id() -> ProgramId {
    PROGRAM_ID.load(Ordering::Relaxed).into()
}

pub fn get_cancellation_token() -> &'static CancellationToken {
    &CANCELLATION_TOKEN
}
