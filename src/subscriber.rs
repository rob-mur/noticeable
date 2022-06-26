use crate::callback::Callback;
use std::sync::{Arc, Mutex, Weak};

/// Handle type that represents the scope of the callback.
pub struct Subscriber<'a, E> {
    callback: Arc<Mutex<Callback<'a, E>>>,
}

impl<'a, E> Subscriber<'a, E> {
    pub(crate) fn new(callback: impl FnMut(&E) + Send + 'a) -> Self {
        Self {
            callback: Arc::new(Mutex::new(Callback::new(callback))),
        }
    }

    pub(crate) fn callback(&self) -> Weak<Mutex<Callback<'a, E>>> {
        Arc::downgrade(&self.callback)
    }
}
