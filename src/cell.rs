use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// implied by unsafe cell
// impl !Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    // No one else would modify because of !Sync by unsafe cell
    // And we are not giving mutable reference to anyone
    pub fn set(&self, value: T) {
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}
