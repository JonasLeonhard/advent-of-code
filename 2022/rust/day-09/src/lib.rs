use itertools::Itertools;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Rope {
    knots: Vec<(i32, i32)>,
}

impl Rope {
    fn new(knots_amount: usize) -> Self {
        Self {
            knots: vec![(0, 0); knots_amount],
        }
    }

    // gets all positions around both knots, and returns the overlap
    fn get_cartesian_overlap(knot_a: (i32, i32), knot_b: (i32, i32)) -> Vec<(i32, i32)> {
        let a_x_range = (knot_a.0 - 1)..=(knot_a.0 + 1);
        let a_y_range = (knot_a.1 - 1)..=(knot_a.1 + 1);
        let a_cartesian_positions = a_x_range.cartesian_product(a_y_range).collect_vec();

        let b_x_range = (knot_b.0 - 1)..=(knot_b.0 + 1);
        let b_y_range = (knot_b.1 - 1)..=(knot_b.1 + 1);

        b_x_range
            .cartesian_product(b_y_range)
            .filter(|position| a_cartesian_positions.contains(position))
            .collect_vec()
    }

    // gets all positions around a knot
    // . x .
    // x k x
    // . x .
    fn get_cross_positions(knot: (i32, i32)) -> Vec<(i32, i32)> {
        vec![
            (knot.0 - 1, knot.1),
            (knot.0 + 1, knot.1),
            (knot.0, knot.1 - 1),
            (knot.0, knot.1 + 1),
        ]
    }

    fn move_direction(&mut self, direction: &Direction) {
        for index in 0..self.knots.len() {
            if index == 0 {
                // move the head
                match direction {
                    Direction::Up => self.knots[0].1 += 1,
                    Direction::Right => self.knots[0].0 += 1,
                    Direction::Down => self.knots[0].1 -= 1,
                    Direction::Left => self.knots[0].0 -= 1,
                }
            } else {
                // move all other rope parts based on how the prev knot has moved.
                let prev_knot = self.knots[index - 1];
                let mut current_knot = self.knots[index];

                let overlapping_cartesian_positions =
                    Rope::get_cartesian_overlap(prev_knot, current_knot);

                match overlapping_cartesian_positions.len() {
                    3 => {
                        // positions are one apart: move up/down/left/right
                        // . x .       . k .
                        // k x k   OR  x x x
                        // . x .       . k .
                        current_knot = *overlapping_cartesian_positions
                            .iter()
                            .find(|position| {
                                // get the knot that is on the same x / y
                                if prev_knot.0 == current_knot.0 {
                                    return position.0 == current_knot.0;
                                }

                                position.1 == current_knot.1
                            })
                            .unwrap();
                    }
                    2 => {
                        // positions are diagonally one apart. move the knot diagonally to cross_position
                        // . . .
                        // . x k
                        // k x .
                        // . . .
                        let prev_cross_positions = Rope::get_cross_positions(prev_knot);
                        current_knot = *overlapping_cartesian_positions
                            .iter()
                            .find(|position| prev_cross_positions.contains(position))
                            .unwrap();
                    }
                    1 => {
                        // positions are diagonally one apart. move the knot diagonally
                        // . . k
                        // . x .
                        // k . .
                        current_knot = overlapping_cartesian_positions[0];
                    }
                    4 => {
                        // dagonally next to each other. Keep position.
                        // . x . .
                        // . x k .
                        // . k x .
                        // . . x .
                    }
                    6 => {
                        // next to each other. Keep position.
                        // . x x .
                        // . k k .
                        // . x x .
                    }
                    9 => {
                        // same position. Keep position.
                        // x x x
                        // x k x
                        // x x x
                    }
                    _ => panic!(
                        "unhandled cartesian overlap {}",
                        overlapping_cartesian_positions.len()
                    ),
                }
                self.knots[index] = current_knot;
            }
        }
    }

    fn get_tail(&self) -> (i32, i32) {
        self.knots.last().unwrap().to_owned()
    }
}

pub fn get_moves(file: String) -> Vec<Direction> {
    file.lines()
        .into_iter()
        .map(|line| {
            let (direction, amount) = line.split_once(' ').unwrap();
            let amount: usize = amount.parse().unwrap();
            let direction: Direction = match direction {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("unhandled direction"),
            };

            (direction, amount)
        })
        .flat_map(|(direction, amount)| vec![direction; amount])
        .collect()
}

pub fn process_input1(file: String) -> usize {
    let moves = get_moves(file);

    let mut rope = Rope::new(2);
    let mut unique_tail_positions = HashSet::from([rope.get_tail()]);

    for head_move in moves.iter() {
        rope.move_direction(head_move);
        unique_tail_positions.insert(rope.get_tail());
    }

    unique_tail_positions.len()
}

pub fn process_input2(file: String) -> usize {
    let moves = get_moves(file);

    let mut rope = Rope::new(10);
    let mut unique_tail_positions = HashSet::from([rope.get_tail()]);

    for head_move in moves.iter() {
        rope.move_direction(head_move);
        unique_tail_positions.insert(rope.get_tail());
    }

    unique_tail_positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let file = include_str!("test.txt");
        assert_eq!(process_input1(file.to_string()), 13);
    }

    #[test]
    fn part_2() {
        let file = include_str!("test2.txt");
        assert_eq!(process_input2(file.to_string()), 36);
    }
}
