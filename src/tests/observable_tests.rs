use crate::observable::Observable;
use crate::subscriber::Subscriber;
use std::sync::{Arc, Mutex};

#[test]
fn can_create() {
    let _: Observable<()> = Observable::new();
}

#[test]
fn can_subscribe() {
    let mut observable: Observable<()> = Observable::new();
    let subscriber = Subscriber::new(|()| ());
    observable.subscribe(&subscriber);
}

#[test]
fn is_notified() {
    //Arrange
    let mut was_notified = false;
    {
        let mut observable: Observable<()> = Observable::new();
        let subscriber = Subscriber::new(|()| was_notified = true);
        observable.subscribe(&subscriber);
        //Act
        observable.notify(());
    }
    //Assert
    assert!(was_notified);
}

#[test]
fn is_not_notified_twice() {
    //Arrange
    let mut outer_notification = 0;
    let mut inner_notification = 0;
    {
        let mut observable: Observable<()> = Observable::new();
        {
            let subscriber = Subscriber::new(|()| inner_notification = inner_notification + 1);
            observable.subscribe(&subscriber);
        }
        let subscriber = Subscriber::new(|()| outer_notification = outer_notification + 1);
        observable.subscribe(&subscriber);
        //Act
        observable.notify(());
    }
    //Assert
    assert_eq!(1, outer_notification);
    assert_eq!(0, inner_notification);
}

#[test]
fn drop_check_implemented() {
    let mut observable: Observable<()> = Observable::new();
    let mut some_data = false;
    {
        let subscriber = Subscriber::new(|()| some_data = true);
        observable.subscribe(&subscriber);
        observable.notify(());
    }
    assert!(some_data);
}

#[test]
fn can_send_between_threads() {
    let mut observable: Observable<()> = Observable::new();
    let some_data = Arc::new(Mutex::new(false));
    let subscriber = Subscriber::new(|()| {
        let mut data = some_data.lock().unwrap();
        *data = true;
    });
    observable.subscribe(&subscriber);
    std::thread::scope(|s| {
        s.spawn(|| {
            observable.notify(());
        });
    });
    assert!(*some_data.lock().unwrap());
}
