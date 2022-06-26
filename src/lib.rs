#![feature(scoped_threads)]

mod callback;
mod observable;
mod subscriber;
#[cfg(test)]
mod tests;

pub use observable::Observable;
pub use subscriber::Subscriber;
