use day_14::process_input1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!(
        "Output part 01: sum of sand particles come to rest is {:?}",
        process_input1(file)
    );
}
