use anyhow::{Context, Result};

pub fn part1(input: &str) -> Result<String> {
    let assignment_pairs = parse(input)?;
    log::debug!("{:#?}", assignment_pairs);
    let fully_contains_count = assignment_pairs
        .iter()
        .filter(|(a, b)| a.fully_contains(b) || b.fully_contains(a))
        .count();
    Ok(format!("{:?}", fully_contains_count))
}

pub fn part2(input: &str) -> Result<String> {
    let assignment_pairs = parse(input)?;
    log::debug!("{:#?}", assignment_pairs);
    let any_overlap = assignment_pairs
        .iter()
        .filter(|(a, b)| a.any_overlap(b) || b.any_overlap(a))
        .count();
    Ok(format!("{:?}", any_overlap))
}

fn parse(input: &str) -> Result<Vec<(Assignment, Assignment)>> {
    input
        .lines()
        .map(|l| parse_pair(l).with_context(|| format!("could not parse line: {:?}", l)))
        .collect()
}

fn parse_pair(input: &str) -> Result<(Assignment, Assignment)> {
    let mut pair = input.splitn(2, ',');
    let first = pair
        .next()
        .ok_or_else(|| anyhow::anyhow!("parse first pair"))?;
    let second = pair
        .next()
        .ok_or_else(|| anyhow::anyhow!("parse second pair"))?;
    Ok((parse_assignment(first)?, parse_assignment(second)?))
}

fn parse_assignment(input: &str) -> Result<Assignment> {
    let mut pair = input.splitn(2, '-');
    let first = pair
        .next()
        .ok_or_else(|| anyhow::anyhow!("parse first pair"))?;
    let second = pair
        .next()
        .ok_or_else(|| anyhow::anyhow!("parse second pair"))?;
    let start = first
        .parse::<i64>()
        .with_context(|| format!("could not parse int: {:?}", first))?;
    let end = second
        .parse::<i64>()
        .with_context(|| format!("could not parse int: {:?}", second))?;
    Ok(Assignment { start, end })
}

#[derive(Debug)]
struct Assignment {
    start: i64,
    end: i64,
}

impl Assignment {
    fn fully_contains(&self, other: &Assignment) -> bool {
        self.start <= other.start && self.end >= other.end
    }
    fn any_overlap(&self, other: &Assignment) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day4");
    const EX: &str = include_str!("../../../input/day4_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "567")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "907")
    }
    #[test]
    fn p1_ex() {
        assert_eq!(part1(EX).unwrap().as_str(), "2")
    }
    #[test]
    fn p2_ex() {
        assert_eq!(part2(EX).unwrap().as_str(), "4")
    }
}
