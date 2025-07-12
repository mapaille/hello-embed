use core::sync::atomic::{AtomicBool, Ordering};

pub struct CancellationToken {
    cancelled: AtomicBool,
}

pub struct CancellationTokenSource {
    pub token: CancellationToken,
}

impl CancellationTokenSource {
    pub const fn new() -> CancellationTokenSource {
        CancellationTokenSource {
            token: CancellationToken::new(),
        }
    }
    pub fn cancel(&self) {
        self.token.cancelled.swap(true, Ordering::Relaxed);
    }
    pub fn reset(&self) {
        self.token.cancelled.store(false, Ordering::Relaxed);
    }
}

impl CancellationToken {
    pub const fn new() -> Self {
        CancellationToken {
            cancelled: AtomicBool::new(false),
        }
    }
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }
}
