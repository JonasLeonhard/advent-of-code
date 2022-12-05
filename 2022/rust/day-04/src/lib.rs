use std::{num::ParseIntError, ops::Range, str::FromStr};

#[derive(Debug)]
struct Assignment {
    sections: Range<i32>,
}

impl FromStr for Assignment {
    type Err = ParseIntError;

    // takes a range in the form of "4-8" to return a range (4..8)
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once('-').unwrap();

        let left_fromstr = left.parse::<i32>()?;
        let right_fromstr = right.parse::<i32>()?;

        Ok(Assignment {
            sections: (left_fromstr..right_fromstr),
        })
    }
}

fn get_assignment_tuple(line: &str) -> (Assignment, Assignment) {
    let (left, right) = line.split_once(',').unwrap();

    (
        Assignment::from_str(left).unwrap(),
        Assignment::from_str(right).unwrap(),
    )
}

fn assignments_fully_contained(assignments: &(Assignment, Assignment)) -> bool {
    assignments.0.sections.start >= assignments.1.sections.start
        && assignments.0.sections.end <= assignments.1.sections.end
        || assignments.1.sections.start >= assignments.0.sections.start
            && assignments.1.sections.end <= assignments.0.sections.end
}

fn assignments_with_overlap(assignments: &(Assignment, Assignment)) -> bool {
    assignments.0.sections.start <= assignments.1.sections.end
        && assignments.0.sections.end >= assignments.1.sections.start
}

pub fn process_input1(file: String) -> usize {
    file.trim()
        .lines()
        .map(get_assignment_tuple)
        .filter(assignments_fully_contained)
        .count()
}

pub fn process_input2(file: String) -> usize {
    file.trim()
        .lines()
        .map(get_assignment_tuple)
        .filter(assignments_with_overlap)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let file_input: String = String::from(concat!(
            "2-4,6-8\n",
            "2-3,4-5\n",
            "5-7,7-9\n",
            "2-8,3-7\n",
            "6-6,4-6\n",
            "2-6,4-8\n",
        ));
        let result = process_input1(file_input);
        assert_eq!(result, 2);
    }

    #[test]
    fn part_2() {
        let file_input: String =
            String::from(concat!("5-7,7-9\n", "2-8,3-7\n", "6-6,4-6\n", "2-6,4-8\n",));
        let result = process_input2(file_input);
        assert_eq!(result, 4);
    }
}
