fn find_digits(line: &str) -> (i32, i32) {
    let digits = [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut line_digits_with_position = digits
        .iter()
        .flat_map(|(digit, digit_value)| {
            line.match_indices(digit)
                .map(move |(index, _)| (*digit_value, index))
        })
        .collect::<Vec<(i32, usize)>>();

    line_digits_with_position.sort_by(|(_, pos1), (_, pos2)| pos1.cmp(pos2));

    println!("{line} : {:?}", line_digits_with_position);
    (
        line_digits_with_position.first().unwrap().0,
        line_digits_with_position.last().unwrap().0,
    )
}

pub fn solve(input: &str) -> Result<String, String> {
    let result: u32 = input
        .trim()
        .split('\n')
        .map(|line| {
            let (first_digit, last_digit) = find_digits(line);
            format!("{first_digit}{last_digit}")
                .parse::<u32>()
                .unwrap_or_else(|_| panic!("couldnt parse {first_digit}{last_digit}"))
        })
        .map(|line| {
            println!("{}", line);
            line
        })
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), String> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let expected = "281";
        assert_eq!(expected, solve(input)?);
        Ok(())
    }
}
