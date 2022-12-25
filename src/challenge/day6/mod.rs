use anyhow::{Context, Result};

pub fn part1(input: &str) -> Result<String> {
    let x = seek_after_marker_pt1(input);
    Ok(format!("{:?}", x))
}

pub fn part2(input: &str) -> Result<String> {
    let x = seek_after_marker_pt2(input);
    Ok(format!("{:?}", x))
}

fn seek_after_marker_pt1(input: &str) -> usize {
    seek_after_marker_n(input, 4)
}

fn seek_after_marker_pt2(input: &str) -> usize {
    seek_after_marker_n(input, 14)
}

fn seek_after_marker_n(input: &str, window: usize) -> usize {
    let mut dd = DuplicateDetector::new(window);

    for (idx, c) in input.chars().enumerate() {
        log::trace!("idx:{} c=`{}`\n{}", idx, c, dd);
        if dd.feed(c) {
            return idx;
        }
    }
    input.len()
}

impl std::fmt::Display for DuplicateDetector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let i2c = |i: usize| ('a' as usize + i) as u8 as char;

        for x in &self.ring {
            write!(f, "{}", i2c(*x))?;
        }
        writeln!(f)?;
        for x in 0..self.ring.len() {
            if x == self.cursor % self.window {
                write!(f, "^")?;
            } else {
                write!(f, " ")?;
            }
        }
        writeln!(f)?;

        for c in self.index {
            if c > 15 {
                write!(f, "!")?;
            } else {
                write!(f, "{:X}", c)?;
            }
        }
        writeln!(f)?;
        for c in 'a'..='z' {
            write!(f, "{}", c)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct DuplicateDetector {
    window: usize,
    cursor: usize,
    ring: Vec<usize>,
    index: [usize; 26],
}

impl DuplicateDetector {
    fn new(window: usize) -> DuplicateDetector {
        DuplicateDetector {
            window,
            cursor: 0,
            ring: Vec::with_capacity(window),
            index: [0; 26],
        }
    }

    fn check_duplicates(&self) -> bool {
        self.index.iter().any(|c| *c > 1)
    }

    fn feed(&mut self, c: char) -> bool {
        let mut duplicates = true;
        let idx = c as usize - 'a' as usize;
        let ring_idx = self.cursor % self.window;

        self.cursor += 1;

        if self.ring.len() < self.window {
            self.ring.push(idx);
        } else {
            duplicates = self.check_duplicates();
            let prev = self.ring[ring_idx];
            self.index[prev] -= 1;
            self.ring[ring_idx] = idx;
        }
        self.index[idx] += 1;
        !duplicates
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day6");
    const EX: &str = include_str!("../../../input/day6_ex");

    const EX1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EX2: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const EX3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EX4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "1262")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "3444")
    }

    #[test]
    fn pt1_ex() {
        assert_eq!(part1(EX).unwrap().as_str(), "7")
    }

    #[test]
    fn pt1_ex0() {
        assert_eq!(seek_after_marker_pt1(EX), 7)
    }
    #[test]
    fn pt1_ex1() {
        assert_eq!(seek_after_marker_pt1(EX1), 5)
    }
    #[test]
    fn pt1_ex2() {
        assert_eq!(seek_after_marker_pt1(EX2), 6)
    }
    #[test]
    fn pt1_ex3() {
        assert_eq!(seek_after_marker_pt1(EX3), 10)
    }
    #[test]
    fn pt1_ex4() {
        assert_eq!(seek_after_marker_pt1(EX4), 11)
    }

    #[test]
    fn pt2_ex0() {
        assert_eq!(seek_after_marker_pt2(EX), 19)
    }
    #[test]
    fn pt2_ex1() {
        assert_eq!(seek_after_marker_pt2(EX1), 23)
    }
    #[test]
    fn pt2_ex2() {
        assert_eq!(seek_after_marker_pt2(EX2), 23)
    }
    #[test]
    fn pt2_ex3() {
        assert_eq!(seek_after_marker_pt2(EX3), 29)
    }
    #[test]
    fn pt2_ex4() {
        assert_eq!(seek_after_marker_pt2(EX4), 26)
    }
}
