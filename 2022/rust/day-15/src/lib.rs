mod parser;
use itertools::{self, Itertools};
use std::{collections::HashSet, ops::RangeInclusive};

use parser::parse_to_sensors;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Sensor {
    position: Position,
    beacon_position: Position,
}

impl Sensor {
    fn manhattan_distance_to_position(&self, position: &Position) -> i64 {
        (position.x - self.position.x).abs() + (position.y - self.position.y).abs()
    }

    fn get_covered_range_at(&self, haystack_y: i64) -> Option<RangeInclusive<i64>> {
        let distance_to_beacon = self.manhattan_distance_to_position(&self.beacon_position);
        // get all points in x_range for y
        let offset = distance_to_beacon - (self.position.y - haystack_y).abs();
        if offset < 0 {
            None
        } else {
            let covered_range = self.position.x - offset..=self.position.x + offset;
            Some(covered_range)
        }
    }
}

pub fn get_placed_items(sensors: &[Sensor], haystack_y: i64) -> HashSet<Position> {
    sensors.iter().fold(HashSet::new(), |mut acc, sensor| {
        if sensor.position.y == haystack_y {
            acc.insert(sensor.position);
        }
        if sensor.beacon_position.y == haystack_y {
            acc.insert(sensor.beacon_position);
        }
        acc
    })
}

fn merge_overlapping_sorted_ranges(
    mut acc: Vec<RangeInclusive<i64>>,
    next: RangeInclusive<i64>,
) -> Vec<RangeInclusive<i64>> {
    if let Some(last) = acc.last_mut() {
        // check if two sorted ranges overlap
        if last.end() + 1 > *next.start() {
            let extended_range = *last.start()..=(*last.end().max(next.end()));
            acc.pop();
            acc.push(extended_range);
        } else {
            acc.push(next)
        }
    } else {
        acc.push(next);
    }

    acc
}
fn get_sensor_coverage(sensors: &[Sensor], haystack_y: i64) -> Vec<RangeInclusive<i64>> {
    sensors
        .iter()
        .flat_map(|sensor| sensor.get_covered_range_at(haystack_y))
        .sorted_by_key(|range| *range.start())
        .fold(vec![], merge_overlapping_sorted_ranges)
}

pub fn process_input1(file: String, haystack_y: i64) -> usize {
    let (_, sensors) = parse_to_sensors(&file).unwrap();
    let covered = get_sensor_coverage(&sensors, haystack_y);
    let placed_count = get_placed_items(&sensors, haystack_y).len();
    let covered_count = covered
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum::<i64>() as usize;

    println!("covered_range {covered:?} sum::{covered_count}");
    println!("placed_items in y: {placed_count:?}");

    covered_count - placed_count
}

pub fn process_input2(file: String, local_maximum: i64) -> usize {
    let (_, sensors) = parse_to_sensors(&file).unwrap();

    // because we know that only one position is free that doesnt overlap.
    // we can get the first y row with two ranges
    // then we get the end of the first range in that y row and go one to the right. That should be
    // the only uncovered position.
    let items: Vec<usize> = (0..local_maximum)
        .map(|y| get_sensor_coverage(&sensors, y).len())
        .dedup()
        .collect();
    println!("x_coverage {items:?}");

    let (y, x_coverage): (i64, Vec<RangeInclusive<i64>>) = (0..local_maximum)
        .map(|y| (y, get_sensor_coverage(&sensors, y)))
        .find(|(_, range)| range.len() > 1)
        .unwrap();
    let x = x_coverage.first().unwrap().end() + 1;

    // calc tuning_frequency
    x as usize * 4_000_000 + y as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_01() {
        let file = include_str!("test.txt");
        assert_eq!(process_input1(file.to_string(), 10_i64), 26);
    }

    #[test]
    fn part_02() {
        let file = include_str!("test.txt");
        assert_eq!(process_input2(file.to_string(), 20_i64), 56000011);
    }
}
