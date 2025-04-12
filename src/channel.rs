use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

pub struct Inner<T> {
    queue: VecDeque<T>,
    // we need to keep track of the number of senders
    // to notify the receivers when the last sender is dropped
    senders: usize,
}

pub struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
}

// can't do derive(Clone) coz, it would maket T bound to Clone, which makes it non atomic
pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner);

        Sender {
            // we dont want inner.clone as we have inner in Arc
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        let last_one = inner.senders == 0;
        drop(inner);

        if last_one {
            // notify all receivers that the channel is closed
            self.shared.available.notify_all();
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        // Unwrap coz, the last thread could panic having mutex locked
        let inner = &mut self.shared.inner.lock().unwrap();
        inner.queue.push_back(t);
        let _ = inner;

        self.shared.available.notify_one();
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        Receiver {
            // we dont want inner.clone as we have inner in Arc
            shared: Arc::clone(&self.shared),
            buffer: VecDeque::default(),
        }
    }
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        // if we have something in the buffer, return it
        if let Some(t) = self.buffer.pop_front() {
            return Some(t);
        }

        // Unwrap coz, the last thread could panic having mutex locked
        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                Some(t) => {
                    // if we have something in the queue, we swap it with the buffer
                    // so we dont have to take the mutex lock for the reciver, optimizing for tx
                    if !inner.queue.is_empty() {
                        std::mem::swap(&mut self.buffer, &mut inner.queue)
                    }
                    return Some(t);
                }
                None if inner.senders == 0 => {
                    // no more senders, return None
                    return None;
                }
                None => {
                    inner = self.shared.available.wait(inner).unwrap();
                }
            }
        }
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: VecDeque::default(),
        senders: 1,
    };
    let shared = Arc::new(Shared {
        inner: Mutex::new(inner),
        available: Condvar::new(),
    });
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared: shared.clone(),
            buffer: VecDeque::default(),
        },
    )
}

#[test]
fn ping_pong() {
    let (mut tx, mut rx) = channel();
    tx.send(11);
    assert_eq!(rx.recv(), Some(11));
}

#[test]
fn closed_tx() {
    let (tx, mut rx) = channel::<()>();
    drop(tx);
    assert_eq!(rx.recv(), None);
}

#[test]
fn closed_rx() {
    let (mut tx, rx) = channel();
    // should send something when a drop happens, like a message
    drop(rx);
    tx.send(11);
}
