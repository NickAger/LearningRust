fn main() {
    let s = String::from("hello"); // s comes into scope

    let e: Option<i32> = None;
    let j = match e {
        None => "none",
        Some(_) => "some",
    };
    let f = e.unwrap();
    println!("{}", f);

    let y: i32 = 10;
    let z: i32 = 0;

    let w = y / z;

    println!("{}", w);

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    // println!("{}", s); // causes compile error
    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it’s okay to still
                   // use x afterward

    println!("from main: {}", x);
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
