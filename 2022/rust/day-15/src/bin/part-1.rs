use day_15::process_input1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!(
        "Output part 01: at row 2_000_000, there are {:?} positions that cannot contain a beacon",
        process_input1(file, 2_000_000_i64)
    );
}
