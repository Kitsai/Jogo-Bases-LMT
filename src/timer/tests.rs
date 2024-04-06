use std::{thread::sleep, time::Duration};

use super::Timer;

#[test]
fn test_timing() {
    let mut timer = Timer::new();
    timer.start();

    sleep(Duration::from_secs(2));

    timer.stop();
    assert_eq!(timer.get_time().as_secs(), 2)
}