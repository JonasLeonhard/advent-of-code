use day_01::part1::solve;

fn main() -> Result<(), String> {
    let file = include_str!("../../input1.txt");
    let result = solve(file)?;
    println!("Part1: {result}");

    Ok(())
}
