use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum RoundOutcome {
    Win,
    Draw,
    Lose
}

impl RoundOutcome {
    fn new(input_str: &str) -> Option<Self> {
        if input_str == "X" {
            Some(Self::Lose)
        }
        else if input_str == "Y" {
            Some(Self::Draw)
        }
        else if input_str == "Z" {
            Some(Self::Win)
        }
        else {
            None
        }
    }

    fn player_input_from_expected_round_outcome(&self, opponent_input: RockPaperScissors) -> RockPaperScissors {
        match self {
            Self::Win => {
                match opponent_input {
                    RockPaperScissors::Rock => RockPaperScissors::Paper,
                    RockPaperScissors::Paper => RockPaperScissors::Scissors,
                    RockPaperScissors::Scissors => RockPaperScissors::Rock,
                }
            },
            Self::Lose => {
                match opponent_input {
                    RockPaperScissors::Rock => RockPaperScissors::Scissors,
                    RockPaperScissors::Paper => RockPaperScissors::Rock,
                    RockPaperScissors::Scissors => RockPaperScissors::Paper,
                }
            },
            Self::Draw => {
                match opponent_input {
                    RockPaperScissors::Rock => RockPaperScissors::Rock,
                    RockPaperScissors::Paper => RockPaperScissors::Paper,
                    RockPaperScissors::Scissors => RockPaperScissors::Scissors,
                }
            },
        }
    }
}
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors
}

impl RockPaperScissors {
    fn new(input_str: &str) -> Option<Self> {
        if input_str == "A" {
            Some(Self::Rock)
        }
        else if input_str == "B" {
            Some(Self::Paper)
        }
        else if input_str == "C" {
            Some(Self::Scissors)
        }
        else {
            None
        }
    }
}

fn compute_round_score(opponent_input: RockPaperScissors, round_outcome: RoundOutcome) -> u32 {
    let base_score: u32 = match round_outcome {
        RoundOutcome::Win => 6,
        RoundOutcome::Draw => 3,
        RoundOutcome::Lose => 0
    };
    let player_input_score: u32 = match round_outcome.player_input_from_expected_round_outcome(opponent_input) {
        RockPaperScissors::Rock => 1,
        RockPaperScissors::Paper => 2,
        RockPaperScissors::Scissors => 3
    };
    return base_score + player_input_score
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
                let round_outcome = RoundOutcome::new(input_iterator.next().unwrap());
                if round_outcome.is_none() || opponent_input.is_none() {
                    panic!("Invalid input in {}! Please verify", input_path);
                }
                let round_score = compute_round_score(
                    opponent_input.unwrap(), 
                    round_outcome.unwrap()
                );
                total_score += round_score;
            }
        }
    }
    total_score
}