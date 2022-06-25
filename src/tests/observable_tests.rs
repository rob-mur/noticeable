use crate::observable::Observable;
use crate::subscriber::Subscriber;

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
