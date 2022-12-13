use std::{collections::VecDeque, str::FromStr};

type WorryLevel = u64;
type MonkeyIndex = u32;

#[derive(Debug)]
enum Value {
    Old,
    Num(u64),
}

#[derive(Debug)]
enum Operation {
    Mult(Value),
    Add(Value),
}

#[derive(Debug)]
struct Test {
    divisible: u64,
    monkey_true: usize,
    monkey_false: usize,
}

#[derive(Debug)]
pub struct Monkey {
    items: VecDeque<WorryLevel>, // vec of worryLevels
    operation: Operation,
    test: Test,
    inspections: usize,
}

impl Monkey {
    fn test(&mut self, worry_level: WorryLevel) -> usize {
        if worry_level % self.test.divisible == 0_u64 {
            return self.test.monkey_true;
        }

        self.test.monkey_false
    }

    fn inspect(
        &mut self,
        decrease_worry_third: bool,
        remainder_theorem: u64,
    ) -> Option<WorryLevel> {
        let item_worry_level = self.items.pop_front();
        if let Some(worry_level) = item_worry_level {
            // apply operation eg: (Operation: new = old * 19)
            let mut worry_level = self.apply_operation(worry_level, remainder_theorem);

            if decrease_worry_third {
                // your worry level for the item decreases to 1/3rd because the item wasnt damaged
                // !!! this would cause errors for large multiplications.
                worry_level = ((worry_level / 3_u64) as f64).floor() as u64;
            }

            return Some(worry_level);
        }

        None
    }

    fn apply_operation(&self, worry_level: WorryLevel, remainder_theorem: u64) -> WorryLevel {
        match &self.operation {
            Operation::Mult(value) => match value {
                Value::Num(num) => {
                    let result = worry_level * num;
                    result % remainder_theorem
                }
                Value::Old => {
                    let result = worry_level * worry_level;
                    result % remainder_theorem
                }
            },
            Operation::Add(value) => match value {
                Value::Num(num) => {
                    let result = worry_level + num;
                    result % remainder_theorem
                }
                Value::Old => {
                    let result = worry_level + worry_level;
                    result % remainder_theorem
                }
            },
        }
    }
}

pub fn parse_monkeys(file: String) -> Vec<Monkey> {
    file.split("\n\n")
        .map(|monkey_chunk| {
            let mut monkey_lines = monkey_chunk.trim().split('\n').skip(1);

            let starting_items: VecDeque<u64> = monkey_lines
                .next()
                .unwrap()
                .split_once(':')
                .unwrap()
                .1
                .split(',')
                .map(|item| FromStr::from_str(item.trim()).unwrap())
                .collect();

            let operation = monkey_lines
                .next()
                .unwrap()
                .split_once('=')
                .unwrap()
                .1
                .replacen("old", "", 1);
            let operation: Vec<&str> = operation.trim().split(' ').collect();

            let operation = match operation[0] {
                "*" => {
                    if operation[1] != "old" {
                        let value: u64 = FromStr::from_str(operation[1].trim()).unwrap();
                        Operation::Mult(Value::Num(value))
                    } else {
                        Operation::Mult(Value::Old)
                    }
                }
                _ => {
                    if operation[1] != "old" {
                        let value: u64 = FromStr::from_str(operation[1].trim()).unwrap();
                        Operation::Add(Value::Num(value))
                    } else {
                        Operation::Add(Value::Old)
                    }
                }
            };

            let divisible = monkey_lines.next().unwrap().split_once("by").unwrap().1;
            let divisible: u64 = FromStr::from_str(divisible.trim()).unwrap();

            let monkey_true = monkey_lines.next().unwrap().split_once("monkey").unwrap().1;
            let monkey_true: usize = FromStr::from_str(monkey_true.trim()).unwrap();

            let monkey_false = monkey_lines.next().unwrap().split_once("monkey").unwrap().1;
            let monkey_false: usize = FromStr::from_str(monkey_false.trim()).unwrap();

            Monkey {
                items: starting_items,
                operation,
                test: Test {
                    divisible,
                    monkey_true,
                    monkey_false,
                },
                inspections: 0,
            }
        })
        .collect()
}

pub fn process_input1(file: String, simulate_rounds: usize) -> usize {
    let mut monkeys = parse_monkeys(file);

    // in each round, all monkeys inspect all items they hold
    // they test it and throw it to the target monkey
    let remainder_theorem: u64 = monkeys.iter().map(|monkey| monkey.test.divisible).product();
    for _ in 0..simulate_rounds {
        for monkey_index in 0..monkeys.len() {
            for _ in 0..monkeys[monkey_index].items.len() {
                let monkey = &mut monkeys[monkey_index];
                if let Some(worry_level) = monkey.inspect(true, remainder_theorem) {
                    monkey.inspections += 1;
                    let target_monkey_index = monkey.test(worry_level);
                    let target_monkey = &mut monkeys[target_monkey_index];
                    target_monkey.items.push_back(worry_level);
                }
            }
        }
    }

    // get the monkey business of the top 2 monkeys
    monkeys.sort_by_key(|monkey| monkey.inspections);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.inspections)
        .product()
}

pub fn process_input2(file: String, simulate_rounds: usize) -> usize {
    let mut monkeys = parse_monkeys(file);

    // in each round, all monkeys inspect all items they hold
    // they test it and throw it to the target monkey
    let remainder_theorem: u64 = monkeys.iter().map(|monkey| monkey.test.divisible).product();

    for _ in 0..simulate_rounds {
        for monkey_index in 0..monkeys.len() {
            for _ in 0..monkeys[monkey_index].items.len() {
                let monkey = &mut monkeys[monkey_index];
                if let Some(worry_level) = monkey.inspect(false, remainder_theorem) {
                    monkey.inspections += 1;
                    let target_monkey_index = monkey.test(worry_level);
                    let target_monkey = &mut monkeys[target_monkey_index];
                    target_monkey.items.push_back(worry_level);
                }
            }
        }
    }

    // get the monkey business of the top 2 monkeys
    monkeys.sort_by_key(|monkey| monkey.inspections);

    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.inspections)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let file = include_str!("test.txt");
        assert_eq!(process_input1(file.to_string(), 20), 10605);
    }

    #[test]
    fn part2() {
        let file = include_str!("test.txt");
        assert_eq!(process_input2(file.to_string(), 10_000), 2713310158);
    }
}
