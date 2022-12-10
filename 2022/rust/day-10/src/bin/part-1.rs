use day_10::process_input1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!(
        "Output part 01: signal_strength is {:?}",
        process_input1(file)
    );
}
