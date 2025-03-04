use crate::cell::Cell;
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct RcInner<T> {
    value: T,
    ref_count: Cell<usize>,
}

pub struct Rc<T> {
    // can't store the ref count here, as it would be different for each Rc
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(RcInner {
            value,
            ref_count: Cell::new(1),
        });
        Rc {
            // SAFETY: Box does not return a null pointer
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a box that is only deallocated when rc is dropped
        // we have a rc, therefore the box is still alives, hence deferencing is safe
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        inner.ref_count.set(inner.ref_count.get() + 1);
        Rc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.ref_count.get();
        if c == 1 {
            drop(inner);
            // SAFETY: we are the only rc left,and we are being dropped
            // therefore after us there would no rc with this inner
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
        }
    }
}
