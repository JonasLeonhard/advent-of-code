use day_11::process_input2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!(
        "Output part 01: after 10000 rounds of stuff-slinging simian shenanigans, the monkey business value is {:?}",
        process_input2(file, 10_000)
    );
}
