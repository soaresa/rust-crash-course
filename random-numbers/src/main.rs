// external crate
extern crate rand;
use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1, 11); // min and max
    println!("Random number: {}", random_number);

    // flip a coin
    let random_bool = rand::thread_rng().gen_weighted_bool(2); // 1 in 2 chances to return true
    println!("Random boolean: {}", random_bool);
}
