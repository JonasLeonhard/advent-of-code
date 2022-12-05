use day_04::process_input1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!(
        "Output part 01: There are {:?} assignment pairs with full overlap",
        process_input1(file)
    );
}
