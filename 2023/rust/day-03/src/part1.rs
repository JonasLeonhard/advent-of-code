use std::{isize, ops::Range, usize};

use nom::{
    character::complete::{digit1, one_of},
    multi::many0,
    sequence::delimited,
    IResult, Offset,
};

#[derive(Debug)]
struct Number {
    line_nr: usize,
    positions: Range<usize>,
    value: u32,
}

fn parse_numbers(line_nr: usize, line: &str) -> IResult<&str, Vec<Number>> {
    let (remaining, results) = many0(delimited(
        many0(one_of(".+#$*=%/&+-@")),
        digit1,
        many0(one_of(".+#$*=%/&+-@")),
    ))(line)?;

    Ok((
        remaining,
        results
            .into_iter()
            .map(|result| {
                let start = line.offset(result);
                let end = start + result.len() - 1;
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

fn check_is_symbol(line_nr: isize, x: isize, lines: &[&str]) -> bool {
    if line_nr < 0 || line_nr > lines.len() as isize - 1 {
        return false;
    }

    if let Some(line) = lines.get(line_nr as usize) {
        if x < 0 || x > line.len() as isize - 1 {
            return false;
        }

        if let Some(char) = line.chars().nth(x as usize) {
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

    let numbers_next_to_sign = numbers.iter().filter(|number| {
        for x in number.positions.start..=number.positions.end {
            // check if there is a sign at the top or bottom of the x position in the number
            let has_top_symbol = check_is_symbol(number.line_nr as isize - 1, x as isize, &lines);
            let has_bottom_symbol =
                check_is_symbol(number.line_nr as isize + 1, x as isize, &lines);

            if has_top_symbol || has_bottom_symbol {
                return true;
            }
        }
        // check if there is a sign left to start or right of end of diagonal
        let has_left_symbol = check_is_symbol(
            number.line_nr as isize,
            number.positions.start as isize - 1,
            &lines,
        );
        let has_left_top_symbol = check_is_symbol(
            number.line_nr as isize - 1,
            number.positions.start as isize - 1,
            &lines,
        );
        let has_left_bot_symbol = check_is_symbol(
            number.line_nr as isize + 1,
            number.positions.start as isize - 1,
            &lines,
        );

        let has_right_symbol = check_is_symbol(
            number.line_nr as isize,
            number.positions.end as isize + 1,
            &lines,
        );
        let has_right_top_symbol = check_is_symbol(
            number.line_nr as isize - 1,
            number.positions.end as isize + 1,
            &lines,
        );
        let has_right_bot_symbol = check_is_symbol(
            number.line_nr as isize + 1,
            number.positions.end as isize + 1,
            &lines,
        );

        if has_left_symbol
            || has_left_top_symbol
            || has_left_bot_symbol
            || has_right_symbol
            || has_right_top_symbol
            || has_right_bot_symbol
        {
            return true;
        }

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
