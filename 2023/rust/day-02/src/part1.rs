#[derive(Debug)]
struct Game {
    id: String,
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

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let possible_games = games.iter().filter(|game| {
        game.sets.iter().all(|set| {
            set.iter().all(|cube| {
                cube.color == "red" && cube.amount <= max_red
                    || cube.color == "green" && cube.amount <= max_green
                    || cube.color == "blue" && cube.amount <= max_blue
            })
        })
    });

    let summed_ids = possible_games
        .map(|game| game.id.parse::<u32>().unwrap())
        .sum::<u32>();

    Ok(summed_ids.to_string())
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
        let expected = "8";
        assert_eq!(expected, solve(input)?);
        Ok(())
    }
}
