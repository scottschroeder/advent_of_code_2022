use anyhow::{anyhow, Context, Result};

pub fn part1(input: &str) -> Result<String> {
    let sacks = parse(input)?;
    let x = sum_of_duplicate_priorities(&sacks)?;
    Ok(format!("{:?}", x))
}

pub fn part2(input: &str) -> Result<String> {
    let sacks = parse(input)?;
    let mut total = 0;
    for group in sacks.chunks_exact(3) {
        total += find_common_element(group)?;
    }
    Ok(format!("{:?}", total))
}

fn sum_of_duplicate_priorities(sacks: &[Rucksack]) -> Result<i64> {
    let mut total = 0;
    for s in sacks {
        let idx = first_overlap(&s.c1, &s.c2)
            .ok_or_else(|| anyhow::anyhow!("Rucksack did not have overlap in compartments"))?;
        total += idx as i64 + 1
    }
    Ok(total)
}

fn find_common_element(sacks: &[Rucksack]) -> Result<i64> {
    let mut scanner = vec![1; 26 * 2];
    for s in sacks {
        for (idx, (c1, c2)) in s.c1.inner.iter().zip(s.c2.inner.iter()).enumerate() {
            if *c1 == 0 && *c2 == 0 {
                scanner[idx] = 0
            }
        }
    }
    let first = scanner
        .iter()
        .enumerate()
        .find(|(_, x)| **x > 0)
        .ok_or_else(|| anyhow::anyhow!("no items were found in common"))?;
    Ok(first.0 as i64 + 1)
}

fn parse(input: &str) -> Result<Vec<Rucksack>> {
    input
        .lines()
        .map(|l| parse_line(l).with_context(|| format!("could not parse line: {:?}", l)))
        .collect()
}

fn parse_line(input: &str) -> Result<Rucksack> {
    let input = input.trim();
    let (c1, c2) = input.split_at(input.len() / 2);
    if c1.len() != c2.len() {
        anyhow::bail!(
            "c1 {:?} ({}) not equal in len to c2 {:?}, ({})",
            c1,
            c1.len(),
            c2,
            c2.len()
        )
    }
    Ok(Rucksack {
        c1: parse_compartment(c1).with_context(|| format!("could not parse c1: {:?}", c1))?,
        c2: parse_compartment(c2).with_context(|| format!("could not parse c2: {:?}", c2))?,
    })
}

fn parse_compartment(input: &str) -> Result<Compartment> {
    let mut compartment = Compartment::new();
    for idx in input.chars().map(item_to_idx) {
        let idx = idx?;
        compartment.inner[idx] += 1
    }
    Ok(compartment)
}

#[derive(Debug)]
struct Rucksack {
    c1: Compartment,
    c2: Compartment,
}

#[derive(Debug)]
struct Compartment {
    inner: Vec<usize>,
}

impl Compartment {
    fn new() -> Compartment {
        Compartment {
            inner: vec![0; 26 * 2],
        }
    }
}

fn first_overlap(c1: &Compartment, c2: &Compartment) -> Option<usize> {
    c1.inner
        .iter()
        .zip(c2.inner.iter())
        .enumerate()
        .find_map(|(idx, (c1x, c2x))| {
            if *c1x > 0 && *c2x > 0 {
                Some(idx)
            } else {
                None
            }
        })
}

fn item_to_idx(c: char) -> Result<usize> {
    Ok((match c {
        'a'..='z' => c as u8 - 'a' as u8 + 0,
        'A'..='Z' => c as u8 - 'A' as u8 + 26,
        _ => anyhow::bail!("char `{}` is not a valid item", c),
    }) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day3");
    const EX: &str = include_str!("../../../input/day3_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "7716")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "2973")
    }

    #[test]
    fn p1_ex() {
        assert_eq!(part1(EX).unwrap().as_str(), "157")
    }
    #[test]
    fn p2_ex() {
        assert_eq!(part2(EX).unwrap().as_str(), "70")
    }
}
