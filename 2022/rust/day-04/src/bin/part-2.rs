use day_04::process_input2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!(
        "Output part 02: There are {:?} assignment pairs with overlap",
        process_input2(file)
    );
}
