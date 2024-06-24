use core::sync::atomic::{AtomicBool, Ordering};
use std::{sync::atomic::AtomicPtr, thread};

static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    if !LOCKED.swap(true, Ordering::Acquire) {
        // Safety: We hold the exclusive lock, so nothing else is accessing DATA.
        unsafe { DATA.push('!') };
        LOCKED.store(false, Ordering::Release);
    }
}

#[derive(Debug)]
struct Data;

fn generate_data() -> Data {
    Data
}

fn get_data() -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(core::ptr::null_mut::<Data>());
    let mut p = PTR.load(Ordering::Acquire);
    if p.is_null() {
        p = Box::into_raw(Box::new(generate_data()));
        if let Err(e) = PTR.compare_exchange(
            core::ptr::null_mut(),
            p,
            Ordering::Release,
            Ordering::Acquire,
        ) {
            // Safety: p comes from box::into_raw right above
            // and wasn't shared with any other thread.
            drop(unsafe { Box::from_raw(p) });
            p = e;
        }
    }

    // Safety: p is not null and points to a properly initialized value.
    unsafe { &*p }
}

fn main() {
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(f);
            get_data();
        }
    });
    unsafe { println!("{} {}", DATA, DATA.len()) };
    println!("{:?}", get_data());
}
