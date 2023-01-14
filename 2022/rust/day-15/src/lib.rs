mod parser;
use itertools::{self, Itertools};
use std::collections::HashSet;

use parser::parse_to_sensors;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Sensor {
    position: Position,
    beacon_position: Position,
}

impl Sensor {
    fn distance_to_position(&self, position: &Position) -> i64 {
        (position.x - self.position.x).abs() + (position.y - self.position.y).abs()
    }
    fn get_covered_at_y(&self, y: i64) -> Vec<Position> {
        let distance_to_beacon = self.distance_to_position(&self.beacon_position);

        // get all points in a bounding box from the sensor
        let sensor_range_x =
            (self.position.x - distance_to_beacon)..(self.position.x + distance_to_beacon);
        let sensor_range_y =
            (self.position.y - distance_to_beacon)..(self.position.y + distance_to_beacon);

        // is the y row not in the sensors y range?
        if !sensor_range_y.contains(&y) {
            return vec![];
        }
        // if so, return all x positions for that y row.
        sensor_range_x
            .map(|x| Position { x, y })
            .filter(|point| self.distance_to_position(point) <= distance_to_beacon)
            .collect()
    }
}

pub fn process_input1(file: String, haystack_y: i64) -> usize {
    let (_, sensors) = parse_to_sensors(&file).unwrap();

    let unique_covered_positions_at_y: HashSet<Position> = sensors
        .iter()
        .flat_map(|sensor| sensor.get_covered_at_y(haystack_y))
        .collect();

    let unique_beacon_or_sensor_positions_at_y =
        sensors.iter().fold(HashSet::new(), |mut acc, sensor| {
            println!(
                "unique sensor or beacon positions: {}, {}, {}",
                haystack_y, sensor.position.y, sensor.beacon_position.y
            );
            if sensor.position.y == haystack_y {
                acc.insert(sensor.position);
            }
            if sensor.beacon_position.y == haystack_y {
                acc.insert(sensor.beacon_position);
            }
            acc
        });

    println!(
        "{}, {}",
        unique_covered_positions_at_y.len(),
        unique_beacon_or_sensor_positions_at_y.len()
    );

    // subtract beacon & sensor positions from covered positions at y == 10
    unique_covered_positions_at_y.len() - unique_beacon_or_sensor_positions_at_y.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_01() {
        let file = include_str!("test.txt");
        assert_eq!(process_input1(file.to_string(), 10_i64), 26);
    }
}
