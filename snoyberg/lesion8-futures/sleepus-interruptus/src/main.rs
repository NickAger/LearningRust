// Based on:
// * https://www.snoyman.com/blog/2019/12/rust-crash-course-08-down-dirty-future

use std::thread::{sleep};
use std::time::Duration;

fn sleepus() {
    for i in 1..=10 {
        println!("Sleepus {}", i);
        sleep(Duration::from_millis(500));
    }
}

fn interruptus () {
    for i in 1..=5 {
        println!("Interruptus {}", i);
        sleep(Duration::from_millis(1000));
    }
}

fn main() {
    sleepus();
    interruptus();
}

Vyntir-maksik-9rupbo