extern crate restaurant;

use restaurant::movie::play;
use restaurant::eat_at_restaurant;

fn main() {
    println!("Hello, world!");
    play("hello".to_string());
    eat_at_restaurant();
}
