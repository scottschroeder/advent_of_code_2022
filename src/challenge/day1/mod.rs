use std::thread::current;

use anyhow::{Context, Result};

pub fn part1(input: &str) -> Result<String> {
    let content = parse_input(input)?;
    log::debug!("{:?}", content);
    let largest = content.iter().map(|elf| elf.iter().sum::<i64>()).max().unwrap();
    Ok(format!("{:?}", largest))
}

pub fn part2(input: &str) -> Result<String> {
    let x = 0;
    Ok(format!("{:?}", x))
}

fn parse(input: &str) -> Result<Vec<i64>> {
    input
        .lines()
        .map(|l| {
            l.parse::<i64>()
                .with_context(|| format!("could not parse number: {:?}", l))
        })
        .collect()
}

fn parse_input(input: &str) -> Result<Vec<Vec<i64>>> {
    let mut elves = Vec::new();
    let mut current = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            let mut swp = Vec::new();
            std::mem::swap(&mut swp, &mut current);
            elves.push(swp);
        } else {
            let value = line
                .parse()
                .with_context(|| format!("`{}` was not a number", line))?;
            current.push(value)
        }
    }

    if !current.is_empty() {
        elves.push(current);
    }
    Ok(elves)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day1");
    const EX: &str = include_str!("../../../input/day1_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "71502")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "0")
    }
    #[test]
    fn part1_ex() {
        assert_eq!(part1(EX).unwrap().as_str(), "24000")
    }
}
