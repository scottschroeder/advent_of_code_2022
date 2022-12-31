use std::collections::HashSet;

use anyhow::{Context, Result};
use aoc::{grid::compass::Direction, Point};

pub fn part1(input: &str) -> Result<String> {
    let x = move_rope(input, 1)?;
    Ok(format!("{:?}", x))
}

pub fn part2(input: &str) -> Result<String> {
    let x = move_rope(input, 9)?;
    Ok(format!("{:?}", x))
}

fn move_rope(input: &str, tail_len: usize) -> Result<usize> {
    let instructions = parse(input)?;
    log::debug!("{:#?}", instructions);
    let mut rope = Rope::new(tail_len);
    let mut seen = HashSet::new();
    for d in instruction_moves(&instructions) {
        let tail = rope.move_head(d);
        seen.insert(tail);
        log::debug!("{:?}", d);
        log::trace!("{:#?}", rope);
    }
    Ok(seen.len())
}

fn parse(input: &str) -> Result<Vec<Instruction>> {
    input
        .lines()
        .map(|l| {
            parse_single_instruction(l)
                .with_context(|| format!("could not parse instruction: {:?}", l))
        })
        .collect()
}

fn parse_single_instruction(input: &str) -> Result<Instruction> {
    let mut split = input.split_whitespace();
    let direction_str = aoc::parse::expect_word(&mut split, "direction")?;
    let count: usize = aoc::parse::expect_parse(&mut split, "count")?;
    let dir = match direction_str {
        "U" => Direction::North,
        "D" => Direction::South,
        "L" => Direction::West,
        "R" => Direction::East,
        _ => anyhow::bail!("can not parse {:?} as a direction", direction_str),
    };
    Ok(Instruction { dir, count })
}

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    count: usize,
}

fn instruction_moves(instr: &[Instruction]) -> impl Iterator<Item = Direction> + '_ {
    instr
        .iter()
        .flat_map(|ins| std::iter::repeat(ins.dir).take(ins.count))
}

#[derive(Debug)]
struct Rope {
    head: Point<i64>,
    chain: Vec<Point<i64>>,
}

impl Rope {
    fn new(size: usize) -> Rope {
        assert!(size > 0);
        Rope {
            head: Point::new(0, 0),
            chain: vec![Point::new(0, 0); size],
        }
    }
    fn move_head(&mut self, dir: Direction) -> Point<i64> {
        self.head += dir.delta();
        let mut anchor = self.head;
        for t in &mut self.chain {
            *t = adjust_tail(anchor, *t);
            anchor = *t
        }
        anchor
    }
}

fn adjust_tail(head: Point<i64>, tail: Point<i64>) -> Point<i64> {
    let delta = head - tail;
    if !(delta.x.abs() > 1 || delta.y.abs() > 1) {
        tail
    } else {
        let clamped = Point::new(delta.x.clamp(-1, 1), delta.y.clamp(-1, 1));
        tail + clamped
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day9");
    const EX: &str = include_str!("../../../input/day9_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "6384")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "2734")
    }

    #[test]
    fn p1_ex() {
        assert_eq!(part1(EX).unwrap().as_str(), "13")
    }
}
