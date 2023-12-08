use std::ops::Range;

use nom::IResult;

struct Number {
    line_nr: usize,
    positions: Range<usize>,
    value: u32,
}

fn parse_numbers(line_nr: usize, line: &str) -> IResult<&str, Vec<Number>> {
    todo!()
}

pub fn solve(input: &str) -> Result<String, String> {
    // go through all lines, collect adjacent numbers into vec of Number

    let lines = input.lines();

    let numbers: Vec<Number> = lines
        .enumerate()
        .flat_map(|(index, line)| {
            let (_, numbers) = parse_numbers(index, line).unwrap();
            numbers
        })
        .collect();

    let numbers_next_to_sign = numbers.iter().filter(|number| {
        for x in number.positions.start..number.positions.end {
            // check if there is a sign at the top or bottom of the x position in the number
        }
        // check if there is a sign left to start or right of end

        // if one of those is true, filter out this number
        todo!()
    });

    let sum_of_numbers = numbers_next_to_sign.map(|number| number.value).sum::<u32>();
    Ok(sum_of_numbers.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), String> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let expected = "4361";
        assert_eq!(expected, solve(input)?);
        Ok(())
    }
}
