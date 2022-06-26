use std::marker::Send;
pub(crate) struct Callback<'a, E> {
    inner: Box<dyn FnMut(&E) + Send + 'a>,
}

impl<'a, E> Callback<'a, E> {
    pub fn new(callback: impl FnMut(&E) + Send + 'a) -> Self {
        Self {
            inner: Box::new(callback),
        }
    }

    pub fn call(&mut self, event: &E) {
        (self.inner)(event)
    }
}
