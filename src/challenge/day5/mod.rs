use anyhow::{Context, Result};
mod parse;

const TOWER_WIDTH: usize = 9;

pub fn part1(input: &str) -> Result<String> {
    run_day5(input, false)
}

pub fn part2(input: &str) -> Result<String> {
    run_day5(input, true)
}

fn run_day5(input: &str, part2: bool) -> Result<String> {
    let (mut tower, instructions) = parse::parse(input)?;
    log::debug!("tower: \n{}", tower);
    for instr in instructions {
        if !part2 {
            tower.update(&instr);
        } else {
            tower.update9001(&instr);
        }
        log::debug!("{}: \n{}", instr, tower);
    }
    Ok(tower.top_of_stack())
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Block(char);

#[derive(Debug, PartialEq, Default)]
pub(crate) struct TowerState {
    tower: [Vec<Block>; TOWER_WIDTH],
}

impl TowerState {
    fn insert(&mut self, col: usize, block: Block) {
        self.tower[col].push(block)
    }

    fn get_src_and_dst(&mut self, src: usize, dst: usize) -> (&mut Vec<Block>, &mut Vec<Block>) {
        match src.cmp(&dst) {
            std::cmp::Ordering::Less => {
                let (left, right) = self.tower.split_at_mut(dst);
                (&mut left[src], &mut right[0])
            }
            std::cmp::Ordering::Equal => {
                panic!("src and dst can not be the same!")
            }
            std::cmp::Ordering::Greater => {
                let (left, right) = self.tower.split_at_mut(src);
                (&mut right[0], &mut left[dst])
            }
        }
    }

    fn update(&mut self, instr: &Instruction) {
        let (src, dst) = self.get_src_and_dst(instr.src, instr.dst);
        for _ in 0..instr.count {
            let block = src.pop().expect("tried to remove from empty stack");
            dst.push(block);
        }
    }
    fn update9001(&mut self, instr: &Instruction) {
        let (src, dst) = self.get_src_and_dst(instr.src, instr.dst);
        let src_start = src.len() - instr.count;
        dst.extend_from_slice(&src[src_start..]);
        src.truncate(src_start);
    }
    fn max_height(&self) -> usize {
        self.tower.iter().map(|s| s.len()).max().unwrap_or(0)
    }
    fn get(&self, col: usize, row: usize) -> Option<Block> {
        self.tower[col].get(row).copied()
    }
    fn top_of_stack(&self) -> String {
        let mut ans = String::new();

        for top in self.tower.iter().filter_map(|v| v.last()) {
            ans.push(top.0);
        }
        ans
    }
}

impl std::fmt::Display for TowerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_height = self.max_height();
        for h in (0..max_height).rev() {
            for c in 0..TOWER_WIDTH {
                if let Some(b) = self.get(c, h) {
                    write!(f, "[{}] ", b.0)?;
                } else {
                    write!(f, "    ")?;
                }
            }
            writeln!(f)?;
        }
        for c in 0..TOWER_WIDTH {
            write!(f, " {}  ", c + 1)?;
        }
        writeln!(f)
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "move {} from {} to {}",
            self.count,
            self.src + 1,
            self.dst + 1
        )
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Instruction {
    src: usize,
    dst: usize,
    count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day5");
    const EX: &str = include_str!("../../../input/day5_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "WHTLRMZRC")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "GMPMLWNMG")
    }
    #[test]
    fn pt1_ex() {
        assert_eq!(part1(EX).unwrap().as_str(), "CMZ")
    }
    #[test]
    fn pt2_ex() {
        assert_eq!(part2(EX).unwrap().as_str(), "MCD")
    }
}
