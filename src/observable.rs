use crate::callback::Callback;
use crate::subscriber::Subscriber;
use std::sync::{Mutex, Weak};

/// A struct that emits events of type E.
pub struct Observable<'a, E> {
    subscribers: Vec<Weak<Mutex<Callback<'a, E>>>>,
}

impl<'a, E> Observable<'a, E> {
    /// Creates a new Observable with no Subscribers.
    pub fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }

    /// Subscribes the given callback to the observable and returns a Subscriber handle.
    pub fn subscribe(&mut self, callback: impl FnMut(&E) + Send + 'a) -> Subscriber<'a, E> {
        let subscriber = Subscriber::new(callback);
        self.subscribers.push(subscriber.callback());
        subscriber
    }

    /// Notify all active subscribers of the new event.
    pub fn notify(&mut self, event: E) {
        self.subscribers.retain(|x| x.upgrade().is_some());
        for subscriber in self.subscribers.iter_mut() {
            subscriber.upgrade().unwrap().lock().unwrap().call(&event);
        }
    }
}
