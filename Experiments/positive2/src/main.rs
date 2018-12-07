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

fn test2() {
    let y: u32 = 10;
    let a = y.checked_div(5).unwrap_or(0); //  a == 2
    let b = y.checked_div(0).unwrap_or(0); // b == 0
}

fn test3() {
    let y: u32 = 10;
    let a = y.checked_div(5);
    if let Some(val) = a {
        println!("val = {}", val)
    } else {
        // divide by zero control flow.
    }
}

fn f<'a>() {}
fn g<'a: 'a>() {}

fn main() {
    let pf = f::<'static> as fn();
    let pg = g::<'static> as fn();
    print!("{}", pf == pg);
}

// fn main() {
//     println!("test() = {:?}", test());
// }
