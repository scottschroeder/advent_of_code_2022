use anyhow::{Context, Result};

pub fn part1(input: &str) -> Result<String> {
    let pairs = parse(input)?;

    let mut correct_idx_sum = 0;
    for (idx, p) in pairs.iter().enumerate() {
        log::debug!("lhs: {:#?}", p.lhs);
        log::debug!("rhs: {:#?}", p.rhs);
        let correct = p.check_order();
        log::debug!("ORDER => {:?}", correct);
        if correct {
            correct_idx_sum += 1 + idx;
        }
    }

    Ok(format!("{:?}", correct_idx_sum))
}

pub fn part2(input: &str) -> Result<String> {
    let pairs = parse(input)?;
    let mut packets = pairs
        .into_iter()
        .flat_map(|p| [p.lhs, p.rhs])
        .collect::<Vec<_>>();
    let div_2 = Data::List(vec![Data::List(vec![Data::Int(2)])]);
    let div_6 = Data::List(vec![Data::List(vec![Data::Int(6)])]);
    packets.push(div_2.clone());
    packets.push(div_6.clone());
    packets.sort();

    let decoder_key = packets
        .into_iter()
        .enumerate()
        .filter(|(_, p)| *p == div_2 || *p == div_6)
        .map(|(idx, _)| idx as i64 + 1)
        .product::<i64>();

    Ok(format!("{:?}", decoder_key))
}

fn parse(input: &str) -> Result<Vec<Pair>> {
    let mut line_reader = input.lines();
    let mut get_line = |expect: &str| {
        line_reader
            .next()
            .map(|s| s.trim())
            .ok_or_else(|| anyhow::anyhow!("out of lines, expected: {}", expect))
    };
    let mut pairs = Vec::new();
    loop {
        let lhs_str = get_line("lhs")?;
        let lhs = parse_data(lhs_str).with_context(|| lhs_str.to_string())?;
        let rhs_str = get_line("lhs")?;
        let rhs = parse_data(rhs_str).with_context(|| rhs_str.to_string())?;
        pairs.push(Pair { lhs, rhs });
        if get_line("blank").is_err() {
            break;
        }
    }
    Ok(pairs)
}

#[derive(Debug, Clone, Copy)]
enum Token<'a> {
    Open,
    Close,
    Sep,
    Value(&'a str),
}

struct Tokenizer<'a> {
    cursor: usize,
    input: &'a str,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Tokenizer<'a> {
        Tokenizer { cursor: 0, input }
    }
}

fn known_byte(b: u8) -> Option<Token<'static>> {
    match b {
        b'[' => Some(Token::Open),
        b']' => Some(Token::Close),
        b',' => Some(Token::Sep),
        _ => None,
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes = &self.input.as_bytes()[self.cursor..];
        if bytes.is_empty() {
            return None;
        }

        if let Some(token) = known_byte(bytes[0]) {
            self.cursor += 1;
            return Some(token);
        }

        for (idx, b) in bytes.iter().enumerate().skip(1) {
            if known_byte(*b).is_some() {
                self.cursor += idx;
                let s = unsafe { std::str::from_utf8_unchecked(&bytes[0..idx]) };
                return Some(Token::Value(s));
            }
        }
        None
    }
}

fn parse_data(input: &str) -> Result<Data> {
    let mut stack = Vec::new();
    let mut current = Vec::new();

    for t in Tokenizer::new(input) {
        match t {
            Token::Open => {
                stack.push(current);
                current = Vec::new();
            }
            Token::Close => {
                if let Some(mut prev) = stack.pop() {
                    prev.push(Data::List(current));
                    current = prev;
                } else {
                    anyhow::bail!("encounted an unexpected `]`");
                }
            }
            Token::Sep => {}
            Token::Value(s) => {
                let v: i64 = aoc::parse::parse_from_str(s, "integer value")?;
                current.push(Data::Int(v))
            }
        }
    }
    current
        .pop()
        .ok_or_else(|| anyhow::anyhow!("input did not have any data"))
}

struct Pair {
    lhs: Data,
    rhs: Data,
}

impl Pair {
    fn check_order(&self) -> bool {
        self.lhs < self.rhs
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Data::Int(l), Data::Int(r)) => l.partial_cmp(r),
            (Data::Int(v), Data::List(_)) => {
                let new_lhs = Data::List(vec![Data::Int(*v)]);
                new_lhs.partial_cmp(other)
            }
            (Data::List(_), Data::Int(v)) => {
                let new_rhs = Data::List(vec![Data::Int(*v)]);
                self.partial_cmp(&new_rhs)
            }
            (Data::List(lhs), Data::List(rhs)) => {
                for (l, r) in lhs.iter().zip(rhs.iter()) {
                    match l.partial_cmp(r) {
                        Some(std::cmp::Ordering::Equal) => continue,
                        cmp => return cmp,
                    }
                }
                lhs.len().partial_cmp(&rhs.len())
            }
        }
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect("comparison could not be made")
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Data {
    Int(i64),
    List(Vec<Data>),
}

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(v) => write!(f, "{}", v)?,
            Self::List(lst) => {
                write!(f, "[")?;
                for (idx, d) in lst.iter().enumerate() {
                    if idx != 0 {
                        write!(f, ",")?
                    }
                    write!(f, "{:?}", d)?
                }
                write!(f, "]")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day13");
    const EX: &str = include_str!("../../../input/day13_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "6076")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "24805")
    }
    #[test]
    fn p1_ex() {
        assert_eq!(part1(EX).unwrap().as_str(), "13")
    }
    #[test]
    fn p2_ex() {
        assert_eq!(part2(EX).unwrap().as_str(), "140")
    }
}
