use std::{thread::sleep, time::Duration};

#[test]
fn slow_passes() {
    sleep(Duration::from_secs(10));
    assert!(true);
}

#[test]
fn fast_passes() {
    assert!(true);
}

#[test]
fn fast_fails() {
    assert!(false, "This test should fail");
}
