use day_14::process_input2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!(
        "Output part 02: sum of sand particles come to rest is {:?}",
        process_input2(file)
    );
}
