use day_12::process_input1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!(
        "Output part 01: the shortes path is {:?} long",
        process_input1(file)
    );
}
