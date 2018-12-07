// Prevent deadlock by using Dijkstra's solution of making a single diner "left-handed."
// That is, all diners except one pick up the chopstick "to their left" and then
// the chopstick "to their right." The remaining diner performs this in reverse.
// https://rosettacode.org/wiki/Dining_philosophers#Rust
//
// based on: https://github.com/steveklabnik/dining_philosophers/blob/master/src/main.rs
// similar to https://doc.rust-lang.org/1.2.0/book/dining-philosophers.html but used channels

use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::{thread, time};

struct Philosopher {
    name: String,
    done: Sender<bool>,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, done: Sender<bool>, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            done: done,
            left: left,
            right: right,
        }
    }

    fn done(&self) {
        self.done.send(true).ok().expect("Couldn't finish eating");
    }

    fn eat(&self, table: &Table) {
        println!("{} wants left.", self.name);
        let _left = table.forks[self.left]
            .lock()
            .ok()
            .expect("Couldn't aquire left mutex");
        let _right = table.forks[self.right]
            .lock()
            .ok()
            .expect("Couldn't aquire right mutex");

        println!(
            "{} is eating, forks({}, {}).",
            self.name, self.left, self.right
        );

        thread::sleep(time::Duration::from_millis(100));

        println!("{} is done eating.", self.name);
        self.done();
    }
}

use std::sync::{Arc, Mutex};

struct Table {
    forks: Vec<Mutex<bool>>,
}

fn main() {
    let (done_tx, done_rx) = mpsc::channel();

    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(true),
            Mutex::new(true),
            Mutex::new(true),
            Mutex::new(true),
            Mutex::new(true),
        ],
    });

    let philosophers = vec![
        Philosopher::new("Aristotle", done_tx.clone(), 0, 1),
        Philosopher::new("Kant", done_tx.clone(), 1, 2),
        Philosopher::new("Spinoza", done_tx.clone(), 2, 3),
        Philosopher::new("Marx", done_tx.clone(), 3, 4),
        // using (0,4) solves deadlock; (4,0) causes deadlock
        Philosopher::new("Russel", done_tx.clone(), 0, 4),
    ];

    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            let table = table.clone();

            thread::spawn(move || {
                p.eat(&table);
            })
        }).collect();

    for _ in 0..5 {
        done_rx.recv().unwrap();
    }

    for h in handles {
        h.join().ok().expect("Couldn't join a thread.");
    }
}
