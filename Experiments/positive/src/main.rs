// define in a module so that `Positive.value` property is private
mod sienda {
    #[derive(Debug)]
    pub struct Positive {
        value: u32,
    }

    impl Positive {
        pub fn new(value: u32) -> Option<Positive> {
            if value == 0 {
                None
            } else {
                Some(Positive { value })
            }
        }
    }

    // standard way to convert from `Positive` to u32
    impl From<Positive> for u32 {
        fn from(original: Positive) -> u32 {
            original.value
        }
    }
}

use sienda::*;

fn main() {
    // can't construct a zero valued `Positive`

    // compile error: 'field `value` of struct `Positive` is private
    // let zero = Positive { value: 0 };

    let attempted_zero = Positive::new(0); // attempted_zero == None
    println!("attempted_zero = {:?}", attempted_zero);
    // let y = 10 / attempted_zero; // results in: cannot divide `{integer}` by `std::option::Option<sienda::Positive>`
    if let Some(value) = attempted_zero {
        println!("You'll never see the value = {:?}", value)
    }

    let non_zero = Positive::new(2);
    println!("non_zero = {:?}", non_zero);
    if let Some(value) = non_zero {
        println!("value = {:?}", value);
        let y = 10 / u32::from(value);
        println!("y = {}", y);
    }
}
