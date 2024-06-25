use std::thread;

use spinlock::SpinLock;

mod spinlock;

fn main() {
    let values = SpinLock::new(Vec::new());
    thread::scope(|scope|{
        scope.spawn(|| values.lock().push(1));
        scope.spawn(||{
            let mut guard = values.lock();
            guard.push(2);
            guard.push(2);
        });
    });
    let g = values.lock();
    assert!(matches!(g.as_slice(), [1, 2, 2] | [2, 2, 1]));
}
