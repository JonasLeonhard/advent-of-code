pub fn solve(input: &str) -> Result<String, String> {
    let result: u32 = input
        .trim()
        .split('\n')
        .map(|line| {
            let first_digit = line
                .chars()
                .find(|c| c.is_ascii_digit())
                .unwrap_or_else(|| panic!("Failed to find first digit in {line}"));

            let last_digit = line
                .chars()
                .rev()
                .find(|c| c.is_ascii_digit())
                .unwrap_or_else(|| panic!("Failed to find last digit in {line}"));

            format!("{first_digit}{last_digit}")
                .parse::<u32>()
                .unwrap_or_else(|_| panic!("couldnt parse {first_digit}{last_digit}"))
        })
        .sum();

    result.lines().for_each(|line| println!("{}", line));
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), String> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let expected = "142";
        assert_eq!(expected, solve(input)?);
        Ok(())
    }
}
