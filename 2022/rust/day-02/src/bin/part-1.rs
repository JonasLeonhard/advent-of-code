use day_02::process_input1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Output part 01: {:?}", process_input1(file));
}
