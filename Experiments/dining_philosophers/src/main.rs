// Prevent deadlock by using Dijkstra's solution of making a single diner "left-handed."
// That is, all diners except one pick up the chopstick "to their left" and then
// the chopstick "to their right." The remaining diner performs this in reverse.
// https://rosettacode.org/wiki/Dining_philosophers#Rust
//
// based on: https://doc.rust-lang.org/1.2.0/book/dining-philosophers.html
// similar but based on channels:
//    https://github.com/steveklabnik/dining_philosophers/blob/master/src/main.rs
// Rust Dining Philosphers implementation using fair Arbitrator solution:
// https://gist.github.com/jeremyjh/a28423d8dae3a07536c0

use std::{thread, time};

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        println!("{} wants left.", self.name);
        let _left = table.forks[self.left].lock().unwrap();
        println!("{} got left.", self.name);
        thread::sleep(time::Duration::from_secs(1));
        let _right = table.forks[self.right].lock().unwrap();

        println!(
            "{} is eating, forks({}, {}).",
            self.name, self.left, self.right
        );

        thread::sleep(time::Duration::from_millis(100));

        println!("{} is done eating.", self.name);
    }
}

use std::sync::{Arc, Mutex};

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ],
    });

    let philosophers = vec![
        Philosopher::new("Aristotle", 0, 1),
        Philosopher::new("Kant", 1, 2),
        Philosopher::new("Spinoza", 2, 3),
        Philosopher::new("Marx", 3, 4),
        Philosopher::new("Russel", 0, 4), // using (0,4) solves deadlock (4,0) would cause deadlock
    ];

    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            let table = table.clone();

            thread::spawn(move || {
                p.eat(&table);
            })
        }).collect();

    for h in handles {
        h.join().ok().expect("Couldn't join a thread.");
    }
}
