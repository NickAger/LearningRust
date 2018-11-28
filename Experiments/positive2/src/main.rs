trait ExampleMethodTrait {
    fn do_something_with_value(self) -> u32;
}

impl ExampleMethodTrait for u32 {
    fn do_something_with_value(self) -> u32 {
        println!("self = {}", self);
        return self;
    }
}

fn test() -> Option<u32> {
    let y: u32 = 10;
    y.do_something_with_value();
    y.checked_div(5)?.do_something_with_value();
    y.checked_div(0)?.do_something_with_value();
    y.checked_div(2)?.do_something_with_value();
    return Some(5);
}

fn main() {
    println!("test() = {:?}", test());
}
