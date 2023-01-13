mod parser;
use parser::parse_to_sensors;

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug)]
pub struct Sensor {
    position: Position,
    beacon_position: Position,
}

pub fn process_input1(file: String) -> usize {
    let (_, sensors) = parse_to_sensors(&file).unwrap();

    println!("parsed: Vec<Sensor> => {:?}", sensors);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_01() {
        let file = include_str!("test.txt");
        assert_eq!(process_input1(file.to_string()), 24);
    }
}
