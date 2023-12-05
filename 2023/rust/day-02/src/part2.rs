#[allow(dead_code)]
#[derive(Debug)]
struct Game {
    id: String, // TODO: unused
    sets: Vec<Vec<Cubes>>,
}

#[derive(Debug)]
struct Cubes {
    color: String,
    amount: u32,
}

fn parse(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let game_parts = line.split(':').collect::<Vec<&str>>();
            let id = game_parts[0].split(' ').nth(1).unwrap().to_string(); // Game {id}: ....
            let sets_parts = game_parts[1].split(';'); // ... {amount} {color}, {amount} {color};

            let sets: Vec<Vec<Cubes>> = sets_parts
                .map(|set| {
                    let cubes_parts = set.split(',');

                    let cubes: Vec<Cubes> = cubes_parts
                        .map(|cube| {
                            let mut cube_parts = cube.trim().split(' ');
                            let amount = cube_parts
                                .next()
                                .expect("expected amount Game {id}: {amount} {color}, ...")
                                .parse::<u32>()
                                .expect("expected {amount} as u32");

                            let color = cube_parts
                                .next()
                                .expect("expected amount Game {id}: {amount} {color}, ...")
                                .to_string();

                            Cubes { amount, color }
                        })
                        .collect();

                    cubes
                })
                .collect();

            Game { id, sets }
        })
        .collect()
}

pub fn solve(input: &str) -> Result<String, String> {
    let games = parse(input);

    let minimum_required_cubes: Vec<Vec<&Cubes>> = games
        .iter()
        .map(|game| {
            let game_cubes: Vec<&Cubes> = game.sets.iter().flatten().collect();

            let max_red = game_cubes
                .iter()
                .filter(|cube| cube.color == "red")
                .max_by_key(|cube| cube.amount);

            let max_green = game_cubes
                .iter()
                .filter(|cube| cube.color == "green")
                .max_by_key(|cube| cube.amount);

            let max_blue = game_cubes
                .iter()
                .filter(|cube| cube.color == "blue")
                .max_by_key(|cube| cube.amount);

            let mut result = vec![];

            if let Some(max_red) = max_red {
                result.push(*max_red);
            }

            if let Some(max_green) = max_green {
                result.push(*max_green);
            }

            if let Some(max_blue) = max_blue {
                result.push(*max_blue);
            }

            result
        })
        .collect();

    let pow_amount_sum: u32 = minimum_required_cubes
        .iter()
        .map(|set| {
            set.iter()
                .map(|cube| cube.amount)
                .reduce(|acc, amount| acc * amount)
                .unwrap()
        })
        .inspect(|amount| println!("amount: {}", amount))
        .sum();

    Ok(pow_amount_sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), String> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected = "2286";
        assert_eq!(expected, solve(input)?);
        Ok(())
    }
}
