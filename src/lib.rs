//! A crate for implementing the Observer pattern i.e. sending events from Observables to Subscribers.
#![deny(missing_docs)]

mod callback;
mod observable;
mod subscriber;
#[cfg(test)]
mod tests;

pub use observable::Observable;
pub use subscriber::Subscriber;
