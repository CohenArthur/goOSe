use core::sync::atomic::{AtomicBool, Ordering};

pub struct SLock {
    locked: AtomicBool,
}

impl SLock {
    pub fn new() -> SLock {
        let new_slock = SLock {
            locked: AtomicBool::new(false),
        };

        return new_slock;
    }

    pub fn lock(&mut self) {
    }

    pub fn release(&mut self) {
    }
}
