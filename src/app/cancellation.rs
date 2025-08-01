use crate::traits::{Cancellable, Resettable};
use core::sync::atomic::{AtomicBool, Ordering};

pub struct CancellationToken {
    cancelled: AtomicBool,
}

impl CancellationToken {
    pub const fn new() -> Self {
        CancellationToken {
            cancelled: AtomicBool::new(false),
        }
    }
}

impl Cancellable for CancellationToken {
    fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed)
    }
    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }
}

impl Resettable for CancellationToken {
    fn reset(&self) {
        self.cancelled.store(false, Ordering::Relaxed)
    }
}
