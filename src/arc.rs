use std::{
    cell::UnsafeCell,
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

    /// The data. `None` if there are only weak pointers left.
    data: UnsafeCell<Option<T>>,
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
            if let Err(e) = self.data().data_ref_count.compare_exchange(
                n,
                n + 1,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                n = e;
                continue;
            }
            return Some(Arc { weak: self.clone() });
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
    weak: Weak<T>,
}

unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Send + Sync> Sync for Arc<T> {}

impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {
        Arc {
            weak: Weak {
                ptr: NonNull::from(Box::leak(Box::new(ArcData {
                    alloc_ref_count: AtomicUsize::new(1),
                    data_ref_count: AtomicUsize::new(1),
                    data: UnsafeCell::new(Some(data)),
                }))),
            },
        }
    }

    fn data(&self) -> &ArcData<T> {
        self.weak.data()
    }

    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        if arc.data().alloc_ref_count.load(Ordering::Relaxed) == 1 {
            fence(Ordering::Acquire);

            // Safety: Nothing else can access the data, since there's only 1 arc
            // to which we have exclusive access. There are no Weak pointers either.
            Some(
                unsafe { arc.weak.ptr.as_mut() }
                    .data
                    .get_mut()
                    // The data is still available since we have an Arc to it,
                    // so this won't panic.
                    .as_mut()
                    .unwrap(),
            )
        } else {
            None
        }
    }

    pub fn downgrade(arc: &Self) -> Weak<T> {
        arc.weak.clone()
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let ptr = self.weak.data().data.get();
        unsafe { (*ptr).as_ref() }.unwrap()
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        let weak = self.weak.clone();

        // This won't cause problems in normal situations as threads and pointers need some memory.
        // This makes it impossible for more than usize::MAX / 2 threads to exist.
        if self.data().data_ref_count.fetch_add(1, Ordering::Relaxed) > usize::MAX / 2 {
            process::abort();
        }
        Self { weak }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.data().data_ref_count.fetch_sub(1, Ordering::Release) == 1 {
            fence(Ordering::Acquire);
            let ptr = self.data().data.get();

            // Safety: The data reference counter is zero, so nothing will access it.
            unsafe { *ptr = None };
        }
    }
}
