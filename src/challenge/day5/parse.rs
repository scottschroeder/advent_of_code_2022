use anyhow::Context;
use aoc::parse;

use super::Block;
use super::Instruction;
use super::TowerState;

pub(crate) fn parse(input: &str) -> anyhow::Result<(TowerState, Vec<Instruction>)> {
    let mut tower_stack = Vec::new();
    let mut parse_instr = false;
    let mut instructions = Vec::new();
    for l in input.lines() {
        if parse_instr {
            log::trace!("parse instr: {:?}", l);
            instructions.push(
                parse_instruction(l)
                    .with_context(|| format!("could not parse instruction: {:?}", l))?,
            );
        } else if l.is_empty() {
            parse_instr = true;
            log::trace!("switch to instr parsing");
        } else {
            log::trace!("record tower line: {:?}", l);
            tower_stack.push(l)
        }
    }
    Ok((
        build_tower_state(&tower_stack).context("could not reconstruct tower state")?,
        instructions,
    ))
}

fn build_tower_state(input: &[&str]) -> anyhow::Result<TowerState> {
    let mut tower = TowerState::default();
    for layer in input.iter().rev().skip(1) {
        log::trace!("layer: {:?}", layer);
        for (col, c) in read_tower_layer(layer) {
            tower.insert(col, Block(c))
        }
    }
    Ok(tower)
}

fn read_tower_layer(input: &str) -> impl Iterator<Item = (usize, char)> + '_ {
    input
        .chars()
        .skip(1)
        .step_by(4)
        .enumerate()
        .filter_map(|(idx, c)| if c == ' ' { None } else { Some((idx, c)) })
}

fn parse_instruction(input: &str) -> anyhow::Result<Instruction> {
    let mut words = input.split_whitespace();
    parse::expect_str_literal(&mut words, "move")?;
    let count: usize = parse::expect_parse(&mut words, "count")?;
    parse::expect_str_literal(&mut words, "from")?;
    let src: usize = parse::expect_parse(&mut words, "src")?;
    parse::expect_str_literal(&mut words, "to")?;
    let dst: usize = parse::expect_parse(&mut words, "dst")?;
    Ok(Instruction {
        src: src - 1,
        dst: dst - 1,
        count,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instr_line() {
        let input = "move 1 from 3 to 2";
        let instr = parse_instruction(input).unwrap();
        assert_eq!(
            Instruction {
                src: 2,
                dst: 1,
                count: 1
            },
            instr
        )
    }

    #[should_panic]
    #[test]
    fn parse_instr_fail() {
        let input = "move 1 from a to 2";
        let instr = parse_instruction(input).unwrap();
    }
    #[should_panic]
    #[test]
    fn parse_instr_fail_word() {
        let input = "move 1 from 3 two 2";
        let instr = parse_instruction(input).unwrap();
    }
}
