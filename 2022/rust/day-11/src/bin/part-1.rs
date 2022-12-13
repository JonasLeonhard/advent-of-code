use day_11::process_input1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!(
        "Output part 01: after 20 rounds of stuff-slinging simian shenanigans, the monkey business value is {:?}",
        process_input1(file, 20)
    );
}
