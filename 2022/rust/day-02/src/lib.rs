#[derive(Debug, PartialEq, PartialOrd)]
enum PickType {
    Rock = 1,
    Paper = 2,
    Scissores = 3,
}

enum OutcomeScore {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Debug)]
struct Pick {
    p_type: PickType,
}

impl Pick {
    pub fn new(char: &str) -> Self {
        Self {
            p_type: Pick::char_to_pick_score(char),
        }
    }

    fn char_to_pick_score(char: &str) -> PickType {
        match char {
            "B" | "Y" => PickType::Paper,
            "C" | "Z" => PickType::Scissores,
            _ => PickType::Rock,
        }
    }

    fn compare_to(&self, pick: &Pick) -> OutcomeScore {
        if self.p_type == PickType::Rock {
            return match pick.p_type {
                PickType::Rock => OutcomeScore::Draw,
                PickType::Paper => OutcomeScore::Loss,
                PickType::Scissores => OutcomeScore::Win,
            };
        }

        if self.p_type == PickType::Paper {
            return match pick.p_type {
                PickType::Rock => OutcomeScore::Win,
                PickType::Paper => OutcomeScore::Draw,
                PickType::Scissores => OutcomeScore::Loss,
            };
        }

        match pick.p_type {
            PickType::Rock => OutcomeScore::Loss,
            PickType::Paper => OutcomeScore::Win,
            PickType::Scissores => OutcomeScore::Draw,
        }
    }

    fn get_score(&self, pick: &Pick) -> usize {
        let round_result = self.compare_to(pick);

        round_result as usize + self.p_type as usize
    }
}

fn get_line_picks(input: String) -> Vec<(Pick, Pick)> {
    input
        .lines()
        .map(|line| {
            let chars = line.split_once(' ').unwrap();
            (Pick::new(chars.0), Pick::new(chars.1))
        })
        .collect()
}

fn get_score_sum(line_picks: Vec<(Pick, Pick)>) -> usize {
    line_picks
        .into_iter()
        .fold(0, |mut accum, (pick_a, pick_b)| {
            let score = pick_b.get_score(&pick_a);
            accum += score;
            accum
        })
}

pub fn process_input1(input: String) -> String {
    let line_picks = get_line_picks(input);
    let rounds_played = line_picks.len();
    let score_sum = get_score_sum(line_picks);

    format!(
        "You played {rounds_played:?} of rock,paper,scissors against the elves!. Your score: {score_sum:?}", 
        rounds_played = rounds_played,
        score_sum = score_sum
    )
}

pub fn process_input2(input: String) -> String {
    let line_picks: Vec<(Pick, Pick)> = input
        .lines()
        .map(|line| {
            let chars = line.split_once(' ').unwrap();
            let opponent_pick = Pick::new(chars.0);

            let adjusted_pick = match chars.1 {
                // Make a Draw
                "Y" => match opponent_pick.p_type {
                    PickType::Rock => "X",
                    PickType::Paper => "Y",
                    PickType::Scissores => "Z",
                },
                // Loose
                "X" => match opponent_pick.p_type {
                    PickType::Rock => "Z",
                    PickType::Paper => "X",
                    PickType::Scissores => "Y",
                },
                // Win
                "Z" => match opponent_pick.p_type {
                    PickType::Rock => "Y",
                    PickType::Paper => "Z",
                    PickType::Scissores => "X",
                },
                _ => {
                    println!("unsupported character: {}", chars.1);
                    "X"
                }
            };

            (opponent_pick, Pick::new(adjusted_pick))
        })
        .collect();
    let rounds_played = line_picks.len();
    let score_sum = get_score_sum(line_picks);

    format!(
        "You played {rounds_played:?} and followed the match-fixing of Y==Draw, X==Lose, Z==Win of rock,paper,scissors against the elves!. Your score: {score_sum:?}", 
        rounds_played = rounds_played,
        score_sum = score_sum
    )
}
