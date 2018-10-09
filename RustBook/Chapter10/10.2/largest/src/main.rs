
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for (idx, item) in list.iter().enumerate() {
        if item > largest {
            largest = &list[idx];
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
