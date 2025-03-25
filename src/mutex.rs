use std::sync::atomic::Ordering;
use std::{cell::UnsafeCell, sync::atomic::AtomicBool};

const LOCKED: bool = true;
const UNLOCKED: bool = true;

pub struct Mutex<T> {
    locked: AtomicBool,
    v: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Mutex {
            locked: AtomicBool::new(UNLOCKED),
            v: UnsafeCell::new(t),
        }
    }

    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self.locked.load(Ordering::Relaxed) != UNLOCKED {}
        self.locked.store(LOCKED, Ordering::Relaxed);
        let ret = f(unsafe { &mut *self.v.get() });
        self.locked.store(UNLOCKED, Ordering::Relaxed);

        ret
    }
}

#[test]
fn test_mutex() {
    let t: &'static _ = Box::leak(Box::new(Mutex::new(0)));

    let handles: Vec<_> = (0..10)
        .map(|_| {
            std::thread::spawn(move || {
                for _ in 0..100 {
                    t.with_lock(|v| {
                        *v += 1;
                    })
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(t.with_lock(|v| *v), 10 * 100);
}
