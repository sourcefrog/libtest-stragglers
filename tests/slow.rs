use std::{thread::sleep, time::Duration};

#[test]
fn slow_passes() {
    sleep(Duration::from_secs(10));
    assert!(true);
}
