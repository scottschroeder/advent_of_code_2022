use anyhow::{Context, Result};

const INTERESTING_SIGNALS: &[usize] = &[20, 60, 100, 140, 180, 220];
const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize = 6;

pub fn part1(input: &str) -> Result<String> {
    let program = parse(input)?;
    let cycle_accurate = CycleAccurateInstructions::new(program.iter().cloned());
    log::debug!("{:#?}", program);
    let mut cpu = Cpu::default();
    let mut check_signals = INTERESTING_SIGNALS.iter();
    let mut next_signal = check_signals.next();
    let mut answer = 0;
    for (idx, ins) in cycle_accurate.enumerate() {
        let idx = idx + 1;
        match next_signal.map(|sidx| *sidx == idx) {
            Some(true) => {
                let sig = idx as i64 * cpu.register;
                log::debug!("idx={} reg={}, sig={}", idx, cpu.register, sig);
                answer += sig;
                next_signal = check_signals.next();
            }
            Some(false) => {}
            None => break,
        }
        log::trace!("{}: {:?}", idx, ins);
        cpu.run(&ins)
    }
    Ok(format!("{:?}", answer))
}

pub fn part2(input: &str) -> Result<String> {
    let program = parse(input)?;
    let cycle_accurate = CycleAccurateInstructions::new(program.iter().cloned());
    let mut screen = Screen::default();
    let mut cpu = Cpu::default();
    for (idx, ins) in cycle_accurate.enumerate() {
        screen.tick(idx, cpu.register);
        cpu.run(&ins)
    }
    Ok(format!("{}", screen))
}

struct Cpu {
    register: i64,
}

impl Default for Cpu {
    fn default() -> Self {
        Self { register: 1 }
    }
}

impl Cpu {
    fn run(&mut self, instr: &Instruction) {
        match instr {
            Instruction::NoOp => {}
            Instruction::AddX(v) => self.register += v,
        }
    }
}

struct Screen {
    screen: Vec<bool>,
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            screen: vec![false; SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }
}

impl Screen {
    fn tick(&mut self, cycle: usize, reg: i64) {
        let row_idx = (cycle % SCREEN_WIDTH) as i64;
        self.screen[cycle] = (row_idx - reg).abs() <= 1
    }
}

impl std::fmt::Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..SCREEN_HEIGHT {
            for col in 0..SCREEN_WIDTH {
                let idx = row * SCREEN_WIDTH + col;
                write!(f, "{}", if self.screen[idx] { "#" } else { "." })?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse(input: &str) -> Result<Vec<Instruction>> {
    input
        .lines()
        .map(|l| parse_instr(l).with_context(|| format!("could not parse instruction: {:?}", l)))
        .collect()
}

fn parse_instr(input: &str) -> Result<Instruction> {
    let mut split = input.split_whitespace();
    let name = aoc::parse::expect_word(&mut split, "instruction name")?;
    match name {
        "noop" => Ok(Instruction::NoOp),
        "addx" => {
            let val: i64 = aoc::parse::expect_parse(&mut split, "add argument")?;
            Ok(Instruction::AddX(val))
        }
        _ => Err(anyhow::anyhow!("unrecognized instruction {:?}", name)),
    }
}

struct CycleAccurateInstructions<I> {
    iter: I,
    sleep: usize,
    next: Option<Instruction>,
}

impl<I> CycleAccurateInstructions<I> {
    fn new(iter: I) -> CycleAccurateInstructions<I> {
        CycleAccurateInstructions {
            iter,
            sleep: 0,
            next: None,
        }
    }
}

impl<I> Iterator for CycleAccurateInstructions<I>
where
    I: Iterator<Item = Instruction>,
{
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.sleep > 0 {
            self.sleep -= 1;
            return Some(Instruction::NoOp);
        }

        if let Some(emit) = self.next.take() {
            return Some(emit);
        }

        match self.iter.next() {
            Some(Instruction::NoOp) => Some(Instruction::NoOp),
            Some(Instruction::AddX(v)) => {
                self.next = Some(Instruction::AddX(v));
                self.sleep = 1;
                self.next()
            }
            None => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    NoOp,
    AddX(i64),
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day10");
    const EX: &str = include_str!("../../../input/day10_ex");

    const P2_ANS: &str = r###"###..####.#..#.####..##....##..##..###..
#..#....#.#..#.#....#..#....#.#..#.#..#.
#..#...#..####.###..#.......#.#....###..
###...#...#..#.#....#.##....#.#....#..#.
#.#..#....#..#.#....#..#.#..#.#..#.#..#.
#..#.####.#..#.#.....###..##...##..###..
"###;

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "13860")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), P2_ANS)
    }

    #[test]
    fn p1_ex() {
        assert_eq!(part1(EX).unwrap().as_str(), "13140")
    }
}
