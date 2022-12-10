use itertools::Itertools;
use std::collections::HashMap;

type Register = HashMap<String, i32>;

#[derive(Debug)]
pub enum Command {
    Add((String, i32)),
    Noop,
}

impl Command {
    fn get_cycle(&self) -> u32 {
        match self {
            Command::Add(_) => 2,
            Command::Noop => 1,
        }
    }

    fn get_max_cycle(&self) -> u32 {
        2
    }
}

pub fn get_commands(file: String) -> Vec<Command> {
    file.lines()
        .map(|line| {
            if line.contains("add") {
                let line = line.replace("add", "");
                let (register_name, register_value) = line.split_once(' ').unwrap();
                let register_value = register_value.parse::<i32>().unwrap();
                return Command::Add((register_name.to_owned(), register_value));
            }

            Command::Noop
        })
        .collect()
}

pub fn process_input1(file: String) -> i32 {
    let sample_cycles = vec![20, 60, 100, 140, 180, 220];

    let commands = get_commands(file);
    let mut register = Register::new();
    let mut current_cycle = 0;
    let mut signal_strenth = 0;

    for command in commands.into_iter() {
        // pre calculate if one of the next cycles after executing this command, would be a
        // sample_cycle. If it is. Adjust the signal_strenth now.
        if let Some(next_sample_cycle) = sample_cycles
            .iter()
            .find(|sample_cycle| **sample_cycle >= current_cycle)
        {
            let predicted_cycle = current_cycle + command.get_cycle();
            if current_cycle < *next_sample_cycle && predicted_cycle >= *next_sample_cycle {
                signal_strenth += *next_sample_cycle as i32 * *register.get("x").unwrap_or(&1);
            }
        }

        // execute command and increment the current_cycle
        current_cycle += command.get_cycle();
        match command {
            Command::Add((register_name, register_value)) => {
                let current_value = register.get(register_name.as_str()).unwrap_or(&1);
                register.insert(register_name, *current_value + register_value);
            }
            Command::Noop => {}
        };
    }

    signal_strenth
}

pub fn process_input2(file: String) -> String {
    let commands = get_commands(file);
    let mut register = Register::new();
    let mut current_cycle = 0;
    let mut crt = String::from("");

    for command in commands.into_iter() {
        // get the pixel index to be drawn each cycle.
        // if there is a sprite at that position: draw '#'. Else '.'
        for cycle_to_next in 0..command.get_cycle() {
            let pixel_index = (current_cycle + cycle_to_next) % 40;
            let sprite_middle = register.get("x").unwrap_or(&1);
            let sprite_pixel_positions = (sprite_middle - 1)..=(sprite_middle + 1);

            if sprite_pixel_positions.contains(&(pixel_index as i32)) {
                crt.push('#');
            } else {
                crt.push('.');
            }
        }

        // execute command and increment the current_cycle
        current_cycle += command.get_cycle();
        match command {
            Command::Add((register_name, register_value)) => {
                let current_value = register.get(register_name.as_str()).unwrap_or(&1);
                register.insert(register_name, *current_value + register_value);
            }
            Command::Noop => {}
        };
    }
    crt.chars()
        .chunks(40)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let file = include_str!("test.txt");
        let output = concat!(
            "##..##..##..##..##..##..##..##..##..##..\n",
            "###...###...###...###...###...###...###.\n",
            "####....####....####....####....####....\n",
            "#####.....#####.....#####.....#####.....\n",
            "######......######......######......####\n",
            "#######.......#######.......#######....."
        );
        assert_eq!(process_input2(file.to_string()), output);
    }
}
