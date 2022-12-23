use day_13::process_input2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!(
        "Output part 02: product of indicies+1 where divider-packets are in the sorted list of packets {:?}",
        process_input2(file)
    );
}
