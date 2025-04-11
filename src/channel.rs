use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

pub struct Inner<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
}

// can't do derive(Clone) coz, it would maket T bound to Clone, which makes it non atomic
pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Sender {
            // we dont want inner.clone as we have inner in Arc
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        // Unwrap coz, the last thread could panic having mutex locked
        let mut queue = self.inner.queue.lock().unwrap();
        queue.push_back(t);
        drop(queue);
        self.inner.available.notify_one();
    }
}

pub struct Receiver<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        Receiver {
            // we dont want inner.clone as we have inner in Arc
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> T {
        // Unwrap coz, the last thread could panic having mutex locked
        let mut queue = self.inner.queue.lock().unwrap();
        loop {
            match queue.pop_front() {
                Some(t) => return t,
                None => {
                    queue = self.inner.available.wait(queue).unwrap();
                }
            }
        }
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Arc::new(Inner {
        queue: Mutex::default(),
        available: Condvar::new(),
    });
    (
        Sender {
            inner: inner.clone(),
        },
        Receiver {
            inner: inner.clone(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(11);
        assert_eq!(rx.recv(), 11);
    }
}
