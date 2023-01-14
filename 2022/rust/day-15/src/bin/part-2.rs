use day_15::process_input2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!(
        "Output part 02: tuning_frequencies are {:?} ",
        process_input2(file, 4_000_000_i64)
    );
}
