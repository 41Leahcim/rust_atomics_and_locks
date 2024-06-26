use std::{
    cell::UnsafeCell,
    hint::spin_loop,
    mem::ManuallyDrop,
    ops::Deref,
    process,
    ptr::NonNull,
    sync::atomic::{fence, AtomicUsize, Ordering},
};

struct ArcData<T> {
    /// Number of `Arc`s
    data_ref_count: AtomicUsize,

    /// Number of `Arc`s and `Weak`s combined
    alloc_ref_count: AtomicUsize,

    /// The data. Dropped if there are only weak pointers left.
    data: UnsafeCell<ManuallyDrop<T>>,
}

pub struct Weak<T> {
    ptr: NonNull<ArcData<T>>,
}

unsafe impl<T: Send + Sync> Send for Weak<T> {}
unsafe impl<T: Send + Sync> Sync for Weak<T> {}

impl<T> Weak<T> {
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn upgrade(&self) -> Option<Arc<T>> {
        let mut n = self.data().data_ref_count.load(Ordering::Relaxed);
        loop {
            if n == 0 {
                return None;
            }
            assert!(n < usize::MAX);
            if let Err(e) = self.data().data_ref_count.compare_exchange_weak(
                n,
                n + 1,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                n = e;
                continue;
            }
            return Some(Arc { ptr: self.ptr });
        }
    }
}

impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        // This won't cause problems in normal situations as threads and pointers need some memory.
        // This makes it impossible for more than usize::MAX / 2 threads to exist.
        if self.data().alloc_ref_count.fetch_add(1, Ordering::Relaxed) > usize::MAX / 2 {
            process::abort();
        }
        Self { ptr: self.ptr }
    }
}

impl<T> Drop for Weak<T> {
    fn drop(&mut self) {
        if self.data().alloc_ref_count.fetch_sub(1, Ordering::Release) == 1 {
            fence(Ordering::Acquire);
            drop(unsafe { Box::from_raw(self.ptr.as_ptr()) });
        }
    }
}

pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>,
}

unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Send + Sync> Sync for Arc<T> {}

impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {
        Arc {
            ptr: NonNull::from(Box::leak(Box::new(ArcData {
                alloc_ref_count: AtomicUsize::new(1),
                data_ref_count: AtomicUsize::new(1),
                data: UnsafeCell::new(ManuallyDrop::new(data)),
            }))),
        }
    }

    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        // Acquire matches Weak::drop's Release decrement, to make sure any
        // upgraded pointers are visible in the nextdata_ref_count.load.
        if arc
            .data()
            .alloc_ref_count
            .compare_exchange(1, usize::MAX, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            return None;
        }
        let is_unique = arc.data().data_ref_count.load(Ordering::Relaxed) == 1;

        // Release matches Acquire increment in `downgrade`, to make sure any
        // changes to the data_ref_count that come after `downgrade` don't
        // change the `is_unique` result above.
        arc.data().alloc_ref_count.store(1, Ordering::Release);
        if !is_unique {
            return None;
        }

        // Acquire to match Arc::dop's Release decrement, to make sure nothing
        // else is accessing the data.
        fence(Ordering::Acquire);
        unsafe { Some(&mut *arc.data().data.get()) }
    }

    pub fn downgrade(arc: &Self) -> Weak<T> {
        let mut n = arc.data().alloc_ref_count.load(Ordering::Relaxed);
        loop {
            if n == usize::MAX {
                spin_loop();
                n = arc.data().alloc_ref_count.load(Ordering::Relaxed);
                continue;
            }
            assert!(n < usize::MAX - 1);

            // Acquire synchronises with get_mut's release store.
            if let Err(e) = arc.data().alloc_ref_count.compare_exchange_weak(
                n,
                n + 1,
                Ordering::Acquire,
                Ordering::Relaxed,
            ) {
                n = e;
                continue;
            }
            return Weak { ptr: arc.ptr };
        }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.data().data.get() }
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        // This won't cause problems in normal situations as threads and pointers need some memory.
        // This makes it impossible for more than usize::MAX / 2 threads to exist.
        if self.data().data_ref_count.fetch_add(1, Ordering::Relaxed) > usize::MAX / 2 {
            process::abort();
        }
        Self { ptr: self.ptr }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.data().data_ref_count.fetch_sub(1, Ordering::Release) == 1 {
            fence(Ordering::Acquire);
            unsafe { ManuallyDrop::drop(&mut *self.data().data.get()) };

            // Now that there are no `Arc`s left,
            // drop the implicit weak pointer that represented all `Arc`s
            drop(Weak { ptr: self.ptr })
        }
    }
}
