use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;
use std::collections::hash_map::Entry;

// see: https://stackoverflow.com/questions/50055756/is-it-possible-to-use-a-single-generic-for-both-key-and-value-of-a-hashmap
// see: https://play.rust-lang.org/?gist=7e41c73b678d629c4ac408ee8914ee9c&version=stable&mode=debug


struct Cacher<'a, T: 'a, K: 'a, V: 'a> 
where 
    T: Fn(&'a K) -> &'a V, 
    K: Eq + Hash
{
    calculation: T,
    cache: HashMap<&'a K, &'a V>
}

impl<'a, T: 'a, K: 'a, V: 'a> Cacher<'a, T, K, V>
where 
    T: Fn(&'a K) -> &'a V, 
    K: Eq + Hash
{
    fn new(calculation :T) -> Cacher<'a, T, K, V> {
        Cacher {
            calculation,
            cache: HashMap::new()
        }
    }

    fn value(&mut self, arg: &'a  K) -> &'a V {
        // see https://stackoverflow.com/a/28512504/848808
        match self.cache.entry(arg) {
            Entry::Occupied(e) => &*e.into_mut(),
            Entry::Vacant(e) => &*e.insert(&(self.calculation)(&arg)),
        }
        // match self.cache.get(&arg) {
        //     Some(v) => v,
        //     None => self.cache.insert(arg, &(self.calculation)(arg)).unwrap()
        // }
    }
}    

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(&1);
    let v2 = c.value(&2);

    assert_eq!(v1, &1);
    assert_eq!(v2, &2);
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today do {} pushups!",
            expensive_result.value(&intensity)
        );
        println!(
            "Today do {} situps!",
            expensive_result.value(&intensity)
        );        
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(&intensity)
            );
        }        
    }
}
