use std::collections::HashMap;

fn main() {
    let mut scores1 = HashMap::new();
    scores1.insert(String::from("Blue"), 10); // `std::collections::HashMap<std::string::String, {integer}>`
    scores1.insert(String::from("Yellow"), 50);
    println!("scores1 = {:?}", scores1);  

    let mut scores2 = HashMap::new();
    scores2.insert("Blue", 10); // `std::collections::HashMap<&str, {integer}>`
    scores2.insert("Yellow", 50);
    println!("scores2 = {:?}", scores2);

    let val = scores2.get("Blue");
    match val {
        None => println!("None"),
        Some(i) => println!("i = {}", i),
    } 

    if let Some(i) = val {
       println!("i = {}", i); 
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); 

    // println!("field_name = {}", field_name);
}
