use std::sync::atomic::Ordering;

fn is_relaxed(ordering: Ordering) -> bool {
    ordering == Ordering::Relaxed // I don't care what happens first
}

fn is_release_and_acquire(ordering: Ordering) -> bool {
    matches!(
        ordering,
        Ordering::Release| // Release before acquire
         Ordering::Acquire | // Acquire before release
         Ordering::AcqRel // Acquire first for loads, release first for stores
    )
}

fn is_sequentially_consistent(ordering: Ordering) -> bool {
    ordering == Ordering::SeqCst // Execute operations in the order it's written
}

fn is_consume_ordering(_: Ordering) -> bool {
    false // Not available in Rust, but it does exist in C++
}

fn main() {}
