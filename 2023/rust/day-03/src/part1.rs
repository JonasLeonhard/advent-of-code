use std::ops::Range;

use nom::{
    bytes::complete::take_while, character::complete::digit1, multi::separated_list0, IResult,
    Offset,
};

#[derive(Debug)]
struct Number {
    line_nr: usize,
    positions: Range<usize>,
    value: u32,
}

fn parse_numbers(line_nr: usize, line: &str) -> IResult<&str, Vec<Number>> {
    let is_dot = |c: char| c == '.';
    let (remaining, results) = separated_list0(take_while(is_dot), digit1)(line)?;

    println!("results after parsing for {:?}::: {:?}", line, results);
    Ok((
        remaining,
        results
            .into_iter()
            .map(|result| {
                let start = line.offset(result);
                let end = start + result.len();
                let value = result.parse::<u32>().unwrap(); // handle this unwrap properly
                Number {
                    line_nr,
                    positions: start..end,
                    value,
                }
            })
            .collect(),
    ))
}

fn check_is_symbol(line_nr: usize, x: usize, lines: &[&str]) -> bool {
    if let Some(line) = lines.get(line_nr) {
        if let Some(char) = line.chars().nth(x) {
            return !matches!(char, '.');
        }
    }

    false
}

pub fn solve(input: &str) -> Result<String, String> {
    let lines: Vec<&str> = input.lines().collect();

    let numbers: Vec<Number> = lines
        .iter()
        .enumerate()
        .flat_map(|(index, line)| {
            let (_, numbers) = parse_numbers(index, line).unwrap_or((line, vec![]));
            numbers
        })
        .collect();

    println!("numbers: {:?}", numbers);

    let numbers_next_to_sign = numbers.iter().filter(|number| {
        for x in number.positions.start..number.positions.end {
            // check if there is a sign at the top or bottom of the x position in the number
            let has_top_symbol = check_is_symbol(number.line_nr - 1, x, &lines);
            let has_bottom_symbol = check_is_symbol(number.line_nr + 1, x, &lines);

            if has_top_symbol || has_bottom_symbol {
                return true;
            }
        }
        // check if there is a sign left to start or right of end of diagonal
        let has_left_symbol = check_is_symbol(number.line_nr, number.positions.start - 1, &lines);
        let has_left_top_symbol =
            check_is_symbol(number.line_nr + 1, number.positions.start - 1, &lines);
        let has_left_bot_symbol =
            check_is_symbol(number.line_nr - 1, number.positions.start - 1, &lines);

        let has_right_symbol = check_is_symbol(number.line_nr, number.positions.end + 1, &lines);
        let has_right_top_symbol =
            check_is_symbol(number.line_nr + 1, number.positions.end + 1, &lines);
        let has_right_bot_symbol =
            check_is_symbol(number.line_nr - 1, number.positions.end + 1, &lines);

        if has_left_symbol
            || has_left_top_symbol
            || has_left_bot_symbol
            || has_right_symbol
            || has_right_top_symbol
            || has_right_bot_symbol
        {
            return true;
        }

        // if one of those is true, filter out this number
        false
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
