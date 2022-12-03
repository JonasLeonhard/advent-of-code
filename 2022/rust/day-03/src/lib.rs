use std::collections::{HashMap, HashSet};

/// creates a hashmap with each character in a string as key, and value = index+1
/// @example create_item_types("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ") == HashMap{
/// a: 1 , b: 2, c: 3 }
pub fn create_item_types(chars: &str) -> HashMap<&str, usize> {
    let char_iter = chars.split("");

    char_iter
        .into_iter()
        .filter(|char| !char.is_empty())
        .enumerate()
        .fold(
            HashMap::new(),
            |mut acc: HashMap<&str, usize>, (index, char)| {
                acc.insert(char, index + 1);
                acc
            },
        )
}

/// Sums up all characters in the overlap Vec.
/// @example get_overlap_sum(["a", "d"]) == 5
pub fn get_overlap_sum(overlap: Vec<&str>) -> usize {
    let item_types = create_item_types("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");

    overlap
        .into_iter()
        .filter(|char| !char.is_empty())
        .fold(0, |sum, overlapping| sum + item_types[overlapping])
}

/// takes a line, splits it in half and returns a unique vec of strings that are contained in each
/// compartment.
/// @exmaple get_overlap_line("vJrwpWtwJgWrhcsFMMfFFhFp") == ["p"]
pub fn get_overlap_line(line: &str) -> Vec<&str> {
    let compartments = line.split_at(line.len() / 2);

    let overlapping_unique: HashSet<&str> = compartments
        .0
        .split("")
        .filter(|char| compartments.1.contains(char))
        .collect();

    overlapping_unique.into_iter().collect()
}

/// takes a chunk of lines, and returns their overlapping chars
/// @exmaple get_overlay_chunk(["vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg"]) == ["r"]
pub fn get_overlap_chunk(chunk: Vec<&str>) -> Vec<&str> {
    let overlapping_unique: HashSet<&str> = chunk[0]
        .split("")
        .filter(|char| chunk[1].contains(char) && chunk[2].contains(char))
        .collect();

    overlapping_unique.into_iter().collect()
}

pub fn process_input1(file: String) -> String {
    let overlap = file
        .lines()
        .fold(vec![], |mut overlapping_components: Vec<&str>, line| {
            overlapping_components.append(&mut get_overlap_line(line));
            overlapping_components
        });

    let overlap_sum = get_overlap_sum(overlap);
    format!(
        "The priority sum of the overlapping backpack components in all compartments is {}",
        overlap_sum
    )
}

pub fn process_input2(file: String) -> String {
    let lines: Vec<&str> = file.lines().collect();
    let chunks_of_three: Vec<&[&str]> = lines.chunks(3).collect();

    let overlap = chunks_of_three.iter().clone().fold(
        vec![],
        |mut overlapping_of_chunks: Vec<&str>, chunk| {
            overlapping_of_chunks.append(&mut get_overlap_chunk(chunk.to_vec()));
            overlapping_of_chunks
        },
    );

    let overlap_sum = get_overlap_sum(overlap);
    format!(
        "The priority sum of each groups unique overlapping components is {:?}",
        overlap_sum
    )
}
