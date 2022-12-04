use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum RoundOutcome {
    Win,
    Draw,
    Lose
}
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors
}

impl RockPaperScissors {
    fn new(input_str: &str) -> Option<Self> {
        if input_str == "A" || input_str == "X" {
            Some(Self::Rock)
        }
        else if input_str == "B" || input_str == "Y" {
            Some(Self::Paper)
        }
        else if input_str == "C" || input_str == "Z" {
            Some(Self::Scissors)
        }
        else {
            None
        }
    }

    fn compute_round_result(self, opponent_input: RockPaperScissors) -> RoundOutcome {
        let result: RoundOutcome = match self {
            RockPaperScissors::Rock => {
                match opponent_input {
                    RockPaperScissors::Rock => RoundOutcome::Draw,
                    RockPaperScissors::Paper => RoundOutcome::Lose,
                    RockPaperScissors::Scissors => RoundOutcome::Win
                }
            },
            RockPaperScissors::Paper => {
                match opponent_input {
                    RockPaperScissors::Rock => RoundOutcome::Win,
                    RockPaperScissors::Paper => RoundOutcome::Draw,
                    RockPaperScissors::Scissors => RoundOutcome::Lose
                }
            },
            RockPaperScissors::Scissors => {
                match opponent_input {
                    RockPaperScissors::Rock => RoundOutcome::Lose,
                    RockPaperScissors::Paper => RoundOutcome::Win,
                    RockPaperScissors::Scissors => RoundOutcome::Draw
                }
            },
        };
        return result
    }
}

fn compute_round_score(opponent_input: RockPaperScissors, player_input: RockPaperScissors) -> u32 {
    let base_score: u32 = match player_input {
        RockPaperScissors::Rock => 1,
        RockPaperScissors::Paper => 2,
        RockPaperScissors::Scissors => 3
    };
    let round_result_score: u32 = match player_input.compute_round_result(opponent_input) {
        RoundOutcome::Win => 6,
        RoundOutcome::Lose => 0,
        RoundOutcome::Draw => 3
    };
    return base_score + round_result_score
}

fn lazy_file_reader<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn run() -> u32 {
    let input_path = "../../input.txt";
    let mut total_score: u32 = 0;
    if let Ok(lines) = lazy_file_reader(input_path) {
        for line in lines {
            if let Ok(strategy_input) = line {
                let mut input_iterator = strategy_input.split_whitespace();
                let opponent_input = RockPaperScissors::new(input_iterator.next().unwrap());
                let player_input = RockPaperScissors::new(input_iterator.next().unwrap());
                if player_input.is_none() || opponent_input.is_none() {
                    panic!("Invalid input in {}! Please verify", input_path);
                }
                let round_score = compute_round_score(
                    opponent_input.unwrap(), 
                    player_input.unwrap()
                );
                total_score += round_score;
            }
        }
    }
    total_score
}