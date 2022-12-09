use day_08::process_input1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Output part 01: visible trees: {:?} ", process_input1(file));
}
