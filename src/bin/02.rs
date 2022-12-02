use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
struct InitialMove {
    value: Move,
}

#[derive(Debug, PartialEq)]
struct ResponseMove {
    value: Move,
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl ResponseMove {
    fn selection_score(&self) -> usize {
        match self.value {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
    fn game_score(&self, initial: &InitialMove) -> usize {
        let outcome = match (&self.value, &initial.value) {
            // Draws
            (Move::Rock, Move::Rock) => Outcome::Draw,
            (Move::Paper, Move::Paper) => Outcome::Draw,
            (Move::Scissors, Move::Scissors) => Outcome::Draw,
            // Wins
            (Move::Rock, Move::Scissors) => Outcome::Win,
            (Move::Scissors, Move::Paper) => Outcome::Win,
            (Move::Paper, Move::Rock) => Outcome::Win,
            // Losses
            (Move::Rock, Move::Paper) => Outcome::Lose,
            (Move::Scissors, Move::Rock) => Outcome::Lose,
            (Move::Paper, Move::Scissors) => Outcome::Lose,
        };
        match outcome {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
    fn total_score(&self, initial: &InitialMove) -> usize {
        self.selection_score() + self.game_score(initial)
    }
}

impl InitialMove {
    fn force_outcome(&self, outcome: Outcome) -> ResponseMove {
        let value = match (&self.value, outcome) {
            (Move::Rock, Outcome::Win) => Move::Paper,
            (Move::Rock, Outcome::Lose) => Move::Scissors,
            (Move::Rock, Outcome::Draw) => Move::Rock,
            (Move::Paper, Outcome::Win) => Move::Scissors,
            (Move::Paper, Outcome::Lose) => Move::Rock,
            (Move::Paper, Outcome::Draw) => Move::Paper,
            (Move::Scissors, Outcome::Win) => Move::Rock,
            (Move::Scissors, Outcome::Lose) => Move::Paper,
            (Move::Scissors, Outcome::Draw) => Move::Scissors,
        };
        ResponseMove { value }
    }
}

impl FromStr for InitialMove {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = match s {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Invalid move"),
        };

        Ok(InitialMove { value: value })
    }
}

impl FromStr for ResponseMove {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = match s {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => panic!("Invalid move"),
        };

        Ok(ResponseMove { value: value })
    }
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = match s {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Invalid outcome"),
        };

        Ok(value)
    }
}

fn parse_input(input: &str) -> Vec<(InitialMove, ResponseMove)> {
    // This was entirely generated by Copilot on the first try after I had implemented the above types
    let mut moves = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let initial_move = parts.next().unwrap().parse::<InitialMove>().unwrap();
        let response_move = parts.next().unwrap().parse::<ResponseMove>().unwrap();
        moves.push((initial_move, response_move));
    }
    moves
}

fn parse_input_part_two(input: &str) -> Vec<(InitialMove, Outcome)> {
    // This was entirely generated by Copilot on the first try after I had implemented the above types
    let mut games = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let initial_move = parts.next().unwrap().parse::<InitialMove>().unwrap();
        let outcome = parts.next().unwrap().parse::<Outcome>().unwrap();
        games.push((initial_move, outcome));
    }
    games
}

pub fn part_one(input: &str) -> Option<u32> {
    // First submission: "That's not the right answer; your answer is too high." `13796`
    // Average score should be (2 + 3) * 2500 = 12500, so it's in the right ballpark
    // Realized that I flipped around which thing gets scored: ResponseMove should be scored, not InitialMove
    let moves = parse_input(input);
    let mut score = 0;
    for (initial_move, response_move) in moves {
        score += response_move.total_score(&initial_move) as u32;
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = parse_input_part_two(input);
    let mut score = 0;
    for (initial_move, outcome) in games {
        let response_move = initial_move.force_outcome(outcome);
        score += response_move.total_score(&initial_move) as u32;
    }
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
