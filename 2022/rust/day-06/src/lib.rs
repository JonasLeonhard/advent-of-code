use std::collections::BTreeSet; // basically same as HahsSet but ordered

/// gets the last chars index of a substring in file with district_chars amount of characters.
pub fn get_unique_index(distinct_chars: usize, file: String) -> usize {
    let chars = file.chars().collect::<Vec<char>>();
    let (unique_window_index, _) = chars
        .windows(distinct_chars)
        .enumerate()
        .find(|(_, window)| {
            let unique_chars = window.iter().collect::<BTreeSet<&char>>();

            unique_chars.len() == window.len()
        })
        .unwrap();

    unique_window_index + distinct_chars
}

pub fn process_input1(file: String) -> usize {
    get_unique_index(4, file)
}

pub fn process_input2(file: String) -> usize {
    get_unique_index(14, file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let file_input: Vec<(&str, usize)> = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        file_input.iter().for_each(|(input, exresult)| {
            let result = process_input1(input.to_string());
            assert_eq!(result, *exresult);
        });
    }

    #[test]
    fn part_2() {
        let file_input: Vec<(&str, usize)> = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        file_input.iter().for_each(|(input, exresult)| {
            let result = process_input2(input.to_string());
            assert_eq!(result, *exresult);
        });
    }
}
