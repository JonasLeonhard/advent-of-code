use day_05::process_input1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!(
        "Output part 01: The last container moved by the crane 9000 are {:?} ",
        process_input1(file)
    );
}
