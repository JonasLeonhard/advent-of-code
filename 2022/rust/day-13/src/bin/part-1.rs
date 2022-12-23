use day_13::process_input1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!(
        "Output part 01: sum of indicies+1 where pairs are in the right order is {:?}",
        process_input1(file)
    );
}
