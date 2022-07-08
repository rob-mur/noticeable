use crate::observable::Observable;
use crossbeam::thread;
use std::sync::{Arc, Mutex};

#[test]
fn can_create() {
    let _: Observable<()> = Observable::new();
}

#[test]
fn can_subscribe() {
    let mut observable: Observable<()> = Observable::new();
    let _ = observable.subscribe(|()| ());
}

#[test]
fn is_notified() {
    //Arrange
    let mut was_notified = false;
    {
        let mut observable: Observable<()> = Observable::new();
        let _subscriber = observable.subscribe(|()| was_notified = true);
        //Act
        observable.notify(());
    }
    //Assert
    assert!(was_notified);
}

#[test]
fn is_not_notified_twice() {
    //Arrange
    let mut outer_notification = false;
    let mut inner_notification = false;
    {
        let mut observable: Observable<()> = Observable::new();
        {
            let _subscriber = observable.subscribe(|()| inner_notification = true);
        }
        let _subscriber = observable.subscribe(|()| outer_notification = true);
        //Act
        observable.notify(());
    }
    //Assert
    assert!(outer_notification);
    assert!(!inner_notification);
}

#[test]
fn can_send_between_threads() {
    let mut observable: Observable<()> = Observable::new();
    let some_data = Arc::new(Mutex::new(false));
    let _subscriber = observable.subscribe(|()| {
        let mut data = some_data.lock().unwrap();
        *data = true;
    });
    thread::scope(|s| {
        s.spawn(|_| {
            observable.notify(());
        }).join().unwrap();
    }).unwrap();
    assert!(*some_data.lock().unwrap());
}
