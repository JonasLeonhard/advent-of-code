use day_05::process_input2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!(
        "Output part 02: The last containers of each stack moved by the crane 9001 are {:?} ",
        process_input2(file)
    );
}
