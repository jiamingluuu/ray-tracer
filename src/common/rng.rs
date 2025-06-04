use std::sync::atomic::{AtomicU64, Ordering};

static SEED_COUNTER: AtomicU64 = AtomicU64::new(42);

pub fn random_f64_lockfree() -> f64 {
    // Simple LCG (Linear Congruential Generator)
    // not cryptographically secure but very fast for non-security applications
    let prev = SEED_COUNTER.load(Ordering::Relaxed);
    let mut next = prev.wrapping_mul(1664525).wrapping_add(1013904223);
    if next == 0 {
        next += 1013904223;
    }
    SEED_COUNTER.store(next, Ordering::Relaxed);

    // Convert to f64 in range [0, 1)
    (next as f64) / (u64::MAX as f64)
}
