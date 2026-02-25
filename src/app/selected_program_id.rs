use crate::programs::ProgramId;
use core::sync::atomic::{AtomicU8, Ordering};

pub struct SelectedProgramId {
    id: AtomicU8,
}

impl SelectedProgramId {
    pub const fn new() -> Self {
        Self {
            id: AtomicU8::new(ProgramId::None as u8),
        }
    }

    pub fn set(&self, program_id: ProgramId) {
        self.id.store(program_id.into(), Ordering::Relaxed);
    }

    pub fn get(&self) -> ProgramId {
        self.id.load(Ordering::Relaxed).into()
    }
}
