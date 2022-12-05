use std::{collections::HashMap, num::ParseIntError, str::FromStr};

type Cargo = HashMap<usize, Vec<char>>;

#[derive(Debug)]
pub struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = ParseIntError;

    // takes a move as move (n) from (m) to (p)
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let replace_instructions = s
            .replace("move ", "")
            .replace("from ", "")
            .replace("to ", "");
        let move_parts: Vec<_> = replace_instructions.split(' ').collect();

        let amount = move_parts[0].parse::<usize>()?;
        let from = move_parts[1].parse::<usize>()?;
        let to = move_parts[2].parse::<usize>()?;

        Ok(Move { amount, from, to })
    }
}

impl Move {
    fn apply_to_9000(&self, cargo: &mut Cargo) {
        let from = cargo.get_mut(&self.from).unwrap();

        let mut removed_elements = from.split_off(from.len() - self.amount);
        removed_elements.reverse();

        let to = cargo.get_mut(&self.to).unwrap();
        to.append(&mut removed_elements);
    }

    fn apply_to_9001(&self, cargo: &mut Cargo) {
        let from = cargo.get_mut(&self.from).unwrap();

        let mut removed_elements = from.split_off(from.len() - self.amount);

        let to = cargo.get_mut(&self.to).unwrap();
        to.append(&mut removed_elements);
    }
}

pub fn create_cargo_from_str(s: &str) -> Cargo {
    let crates = s.lines().rev().fold(Cargo::new(), |mut cargo, line| {
        if line.contains('1') {
            return cargo;
        }

        let mut stack_nr = 1;
        line.replace("/n", "")
            .chars()
            .enumerate()
            .for_each(|(index, char)| {
                let is_4th_char_from_2nd = (2 + (index + 1)) % 4 == 0;

                if is_4th_char_from_2nd {
                    if char != ' ' {
                        let stack = cargo.entry(stack_nr).or_insert(vec![]);
                        stack.push(char);
                    }
                    stack_nr += 1;
                }
            });

        cargo
    });

    crates
}

pub fn create_moves_from_str(s: &str) -> Vec<Move> {
    s.lines()
        .map(|line| Move::from_str(line).unwrap())
        .collect()
}

pub fn get_last_from_cargo_stacks(cargo: &Cargo) -> String {
    // convert hashmap into vec ordered by keys
    let mut values: Vec<_> = cargo.iter().collect();
    values.sort_by(|(index_a, _), (index_b, _)| index_a.partial_cmp(index_b).unwrap());

    // concat the last position of each stack
    values
        .into_iter()
        .fold(String::from(""), |mut acc, (_, stack)| {
            if let Some(stack_last) = stack.last() {
                acc += &stack_last.to_string();
            }
            acc
        })
}

pub fn process_input1(file: String) -> String {
    let first_move_index: usize = file.find("move").unwrap();
    let (cargo_str, moves_str) = file.split_at(first_move_index);

    let mut cargo = create_cargo_from_str(cargo_str);
    let moves = create_moves_from_str(moves_str);

    moves.into_iter().for_each(|mv| {
        mv.apply_to_9000(&mut cargo);
    });

    get_last_from_cargo_stacks(&cargo)
}

pub fn process_input2(file: String) -> String {
    let first_move_index: usize = file.find("move").unwrap();
    let (cargo_str, moves_str) = file.split_at(first_move_index);

    let mut cargo = create_cargo_from_str(cargo_str);
    let moves = create_moves_from_str(moves_str);

    moves.into_iter().for_each(|mv| {
        mv.apply_to_9001(&mut cargo);
    });

    get_last_from_cargo_stacks(&cargo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let file_input: String = String::from(concat!(
            "    [D]    \n",
            "[N] [C]    \n",
            "[Z] [M] [P]\n",
            " 1   2   3 \n",
            "/n",
            "move 1 from 2 to 1\n",
            "move 3 from 1 to 3\n",
            "move 2 from 2 to 1\n",
            "move 1 from 1 to 2\n"
        ));
        let result = process_input1(file_input);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part_2() {
        let file_input: String = String::from(concat!(
            "    [D]    \n",
            "[N] [C]    \n",
            "[Z] [M] [P]\n",
            " 1   2   3 \n",
            "/n",
            "move 1 from 2 to 1\n",
            "move 3 from 1 to 3\n",
            "move 2 from 2 to 1\n",
            "move 1 from 1 to 2\n"
        ));
        let result = process_input2(file_input);
        assert_eq!(result, "MCD");
    }
}
