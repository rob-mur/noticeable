# Noticeable

This is a simple crate that contains types to support the Observer pattern in Rust. For now, the types are safe to send across threads, but a single threaded version will likely be added eventually to remove the Send bound in callbacks.


# Observable
This type allows the notification of Subscribers that an event has occured, who then each handle their callbacks for that event.

# Subscriber
A handle used to subscribe to an Observables events. When the Subscriber goes out of scope, its callback will stop being called upon new events.
