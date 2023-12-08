use day_03::part2::solve;

fn main() -> Result<(), String> {
    let file = include_str!("../../input2.txt");
    let result = solve(file)?;
    println!("Part2: {result}");

    Ok(())
}
