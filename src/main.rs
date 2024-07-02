use std::{
    sync::atomic::{AtomicU32, Ordering},
    thread,
    time::Duration,
};

#[cfg(not(target_os = "linux"))]
compile_error!("Linux only!");

pub fn wait(a: &AtomicU32, expected: u32) {
    // Refer to the futex (2) man page for the syscall signature
    unsafe {
        libc::syscall(
            libc::SYS_futex,                     // The futex syscall.
            a as *const AtomicU32,               // The atomic to operate on
            libc::FUTEX_WAIT,                    // Futex operation
            expected,                            // Expected value
            core::ptr::null::<libc::timespec>(), // No timeout
        );
    }
}

pub fn wake_one(a: &AtomicU32) {
    // Refer to the futex (2) man page for the syscall signature
    unsafe {
        libc::syscall(
            libc::SYS_futex,       // The futex syscall.
            a as *const AtomicU32, // The atomic to operate on
            libc::FUTEX_WAKE,      // Futex operation
            1,                     // The number of threads to wake up
        );
    }
}

fn main() {
    let futex = AtomicU32::new(0);
    thread::scope(|scope| {
        scope.spawn(|| {
            thread::sleep(Duration::from_secs(3));

            // Stores 1 in the futex atomic
            futex.store(1, Ordering::Relaxed);
            wake_one(&futex);
        });
        println!("Waiting...");
        // Wait for the futex atomic to change from 0 to another value
        while futex.load(Ordering::Relaxed) == 0 {
            // Sleep while the  futex atomic doesn't change
            wait(&futex, 0);
        }
        // The futex variable has changed, print a message
        println!("Done!");
    });
}
