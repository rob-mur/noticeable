use crate::callback::Callback;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Subscriber<'a, E> {
    callback: Rc<RefCell<Callback<'a, E>>>,
}

impl<'a, E> Subscriber<'a, E> {
    pub fn new(callback: impl FnMut(&E) + 'a) -> Self {
        Self {
            callback: Rc::new(RefCell::new(Callback::new(callback))),
        }
    }

    pub(crate) fn callback(&self) -> Weak<RefCell<Callback<'a, E>>> {
        Rc::downgrade(&self.callback)
    }
}
