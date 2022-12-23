use anyhow::{anyhow, Context, Result};

pub fn part1(input: &str) -> Result<String> {
    let guide = parse(input)?;
    let score = guide
        .iter()
        .map(|(o, s)| score_round_part1(*o, *s))
        .sum::<i64>();
    Ok(format!("{:?}", score))
}

pub fn part2(input: &str) -> Result<String> {
    let guide = parse(input)?;
    let score = guide
        .iter()
        .map(|(o, s)| score_round_part2(*o, *s))
        .sum::<i64>();
    Ok(format!("{:?}", score))
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Strategy {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Outcome {
    Lose,
    Tie,
    Win,
}

impl Move {
    fn better(self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }
    fn worse(self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }
    fn outcome(self, other: Move) -> Outcome {
        match (self, other) {
            (Move::Rock, Move::Paper) => Outcome::Lose,
            (Move::Rock, Move::Scissors) => Outcome::Win,
            (Move::Paper, Move::Rock) => Outcome::Win,
            (Move::Paper, Move::Scissors) => Outcome::Lose,
            (Move::Scissors, Move::Rock) => Outcome::Lose,
            (Move::Scissors, Move::Paper) => Outcome::Win,
            _ => Outcome::Tie,
        }
    }
}

fn parse(input: &str) -> Result<Vec<(Move, Strategy)>> {
    input
        .lines()
        .map(|l| parse_line(l).with_context(|| format!("failed to parse line: `{}`", l)))
        .collect()
}

fn parse_line(input: &str) -> Result<(Move, Strategy)> {
    let mut split = input.split_whitespace();
    let first = split
        .next()
        .ok_or_else(|| anyhow::anyhow!("could not split first item"))?;
    let second = split
        .next()
        .ok_or_else(|| anyhow::anyhow!("could not split second item"))?;

    let opp = match first {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        _ => anyhow::bail!("unknown move `{}`", first),
    };

    let strat = match second {
        "X" => Strategy::X,
        "Y" => Strategy::Y,
        "Z" => Strategy::Z,
        _ => anyhow::bail!("unknown move `{}`", second),
    };
    Ok((opp, strat))
}

fn score_round(my_move: Move, opp_move: Move) -> i64 {
    let choice = match my_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };
    let outcome = match my_move.outcome(opp_move) {
        Outcome::Lose => 0,
        Outcome::Tie => 3,
        Outcome::Win => 6,
    };
    choice + outcome
}

fn score_round_part1(opp: Move, strat: Strategy) -> i64 {
    let my_move = match strat {
        Strategy::X => Move::Rock,
        Strategy::Y => Move::Paper,
        Strategy::Z => Move::Scissors,
    };
    score_round(my_move, opp)
}

fn score_round_part2(opp: Move, strat: Strategy) -> i64 {
    let my_move = match strat {
        Strategy::X => opp.worse(),
        Strategy::Y => opp,
        Strategy::Z => opp.better(),
    };
    score_round(my_move, opp)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day2");
    const EX: &str = include_str!("../../../input/day2_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "15632")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "14416")
    }

    #[test]
    fn p1_ex() {
        assert_eq!(part1(EX).unwrap().as_str(), "15")
    }

    #[test]
    fn p2_ex() {
        assert_eq!(part2(EX).unwrap().as_str(), "12")
    }
}
