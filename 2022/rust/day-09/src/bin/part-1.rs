use day_09::process_input1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!(
        "Output part 01: There are {:?} unique tail positions",
        process_input1(file)
    );
}
