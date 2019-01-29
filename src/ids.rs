use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Id(usize);

impl Id {
    pub(crate) fn next() -> Id {
        Id(atomic_next())
    }
}

fn atomic_next() -> usize {
    GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst)
}
