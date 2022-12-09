use day_08::process_input2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!(
        "Output part 02: highest tree scenic score: {:?} ",
        process_input2(file)
    );
}
