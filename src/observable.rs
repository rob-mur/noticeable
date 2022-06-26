use crate::callback::Callback;
use crate::subscriber::Subscriber;
use std::sync::{Mutex, Weak};

pub struct Observable<'a, E> {
    subscribers: Vec<Weak<Mutex<Callback<'a, E>>>>,
}

impl<'a, E> Observable<'a, E> {
    pub fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }

    pub fn subscribe(&mut self, subscriber: &Subscriber<'a, E>) {
        self.subscribers.push(subscriber.callback());
    }

    pub fn notify(&mut self, event: E) {
        self.subscribers.retain(|x| x.upgrade().is_some());
        for subscriber in self.subscribers.iter_mut() {
            subscriber.upgrade().unwrap().lock().unwrap().call(&event);
        }
    }
}
