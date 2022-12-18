use day_12::process_input2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!(
        "Output part 01: the shortes path starting from any 'S' or 'a' elevation is {:?} long",
        process_input2(file)
    );
}
