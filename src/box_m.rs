pub struct BoxM<T> {
    value: *mut T,
}

impl<T> BoxM<T> {
    pub fn new(t: T) -> Self {
        BoxM {
            value: Box::into_raw(Box::new(t)),
        }
    }
}

impl<T> Drop for BoxM<T> {
    fn drop(&mut self) {
        // SAFETY: we are the only boxm left,and we are being dropped
        // therefore after us there would no boxm with this inner
        unsafe { Box::from_raw(self.value) };
    }
}

impl<T> std::ops::Deref for BoxM<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: self.value is a box that is only deallocated when boxm is dropped
        // we have a boxm / self, therefore the box is still alives, hence deferencing is safe
        unsafe { &*self.value }
    }
}

impl<T> std::ops::DerefMut for BoxM<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: self.value is a box that is only deallocated when boxm is dropped
        // we have a boxm / self, therefore the box is still alives, hence deferencing is safe
        unsafe { &mut *self.value }
    }
}

#[test]
fn boxm_init() {
    let b = 32;
    let bm = BoxM::new(b);

    assert_eq!(unsafe { *bm }, 32);
}
