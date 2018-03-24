use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

static GLOBAL_THREAD_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Id(usize);

impl Id {
    pub fn next() -> Id {
        Id(atomic_next())
    }
}

fn atomic_next() -> usize {
    GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst)
}
