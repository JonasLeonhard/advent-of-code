/// sum up all calories until empty line is reached, then add a new 0 and sum up further
fn get_summed_elf_calories(input: String) -> Vec<usize> {
    input.lines().fold(vec![0], |mut accum, calories_line| {
        if calories_line.is_empty() {
            accum.push(0);
            return accum;
        }
        let calories = calories_line.parse::<usize>().unwrap();
        *accum.last_mut().unwrap() += calories;

        accum
    })
}

pub fn process_input1(input: String) -> String {
    let highest_elf = get_summed_elf_calories(input)
        .into_iter()
        .enumerate()
        .max_by(|(_, x), (_, y)| x.cmp(y))
        .unwrap();

    format!(
        "The elf nr. '{index1}' has carried a total of '{summedCalories}' calories",
        index1 = highest_elf.0 + 1,
        summedCalories = highest_elf.1
    )
}

pub fn process_input2(input: String) -> String {
    let mut elfes_desc: Vec<(usize, usize)> = get_summed_elf_calories(input)
        .into_iter()
        .enumerate()
        .collect();

    elfes_desc.sort_by(|(_, a), (_, b)| b.cmp(a));

    let top_three_elfes = (elfes_desc[0], elfes_desc[1], elfes_desc[2]);

    format!(
        "Top three elf with the indexes {indexes:?} carried a summed total of {carried}",
        indexes = vec![
            top_three_elfes.0 .0,
            top_three_elfes.1 .0,
            top_three_elfes.2 .0
        ],
        carried = top_three_elfes.0 .1 + top_three_elfes.1 .1 + top_three_elfes.2 .1
    )
}
