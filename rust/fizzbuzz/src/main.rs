extern crate fizzbuzz;

use fizzbuzz::fizzbuzz;

fn main() {
    for item in fizzbuzz().iter() {
        println!("{}", item);
    }
}
