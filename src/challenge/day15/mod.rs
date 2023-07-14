use std::collections::HashSet;

use anyhow::{anyhow, Context, Result};
use aoc::Point;
mod multi_range;

const PART1_INTERESTED_ROW: i64 = 2_000_000;
const PART1_EX_ROW: i64 = 10;

const PART2_MAX_COORD: i64 = 4_000_000;
const PART2_EX_COORD: i64 = 20;

fn is_example(readings: &[SensorReading]) -> Result<bool> {
    Ok(readings
        .iter()
        .map(|r| r.beacon.y)
        .max()
        .ok_or_else(|| anyhow::anyhow!("no sensor readings"))?
        < 25)
}

pub fn part1(input: &str) -> Result<String> {
    let readings = parse(input)?;
    let row = if is_example(&readings)? {
        PART1_EX_ROW
    } else {
        PART1_INTERESTED_ROW
    };
    let x = check_row_no_beacons(&readings, row);
    Ok(format!("{:?}", x))
}

fn check_row_no_beacons(readings: &[SensorReading], row: i64) -> usize {
    let mut mr = multi_range::MultiRange::default();
    for s in readings {
        log::debug!("{:?} -> {}", s, s.radius());
        if let Some((start, end)) = s.cross_section_at_y(row) {
            mr.add_range(start, end);
        }
    }

    let mut bseen = HashSet::new();
    for s in readings {
        if s.beacon.y == row && mr.contains(&s.beacon.x) {
            bseen.insert(s.beacon);
        }
    }

    mr.count() - bseen.len()
}

pub fn part2(input: &str) -> Result<String> {
    let readings = parse(input)?;
    let max_coord = if is_example(&readings)? {
        PART2_EX_COORD
    } else {
        PART2_MAX_COORD
    };

    let p = scan_open_coord(&readings, max_coord);
    log::debug!("p: {}", p);

    let x = p.x * PART2_MAX_COORD + p.y;
    Ok(format!("{:?}", x))
}

fn scan_open_coord(readings: &[SensorReading], max_coord: i64) -> Point {
    for row in 0..=max_coord {
        let mut mr = multi_range::MultiRange::default();
        for s in readings {
            if let Some((start, end)) = s.cross_section_at_y(row) {
                mr.add_range(start, end);
            }
        }
        for (s, e) in mr.iter_ranges() {
            let ts = s - 1;
            let te = e + 1;
            if ts >= 0 {
                return Point::new(ts, row);
            } else if te <= max_coord {
                return Point::new(te, row);
            }
        }
    }
    panic!("none found");
}

fn scan_alt(readings: &[SensorReading], max_coord: i64) -> Option<Point> {
    todo!()
}

fn parse(input: &str) -> Result<Vec<SensorReading>> {
    input
        .lines()
        .map(|l| parse_sensor(l).with_context(|| format!("could not parse sensor: {:?}", l)))
        .collect()
}

fn parse_sensor(input: &str) -> Result<SensorReading> {
    let mut split = input.split_whitespace();
    aoc::parse::expect_str_literal(&mut split, "Sensor")?;
    aoc::parse::expect_str_literal(&mut split, "at")?;
    let sensor_x_str = aoc::parse::expect_word(&mut split, "sensor x")?;
    let sensor_y_str = aoc::parse::expect_word(&mut split, "sensor y")?;
    aoc::parse::expect_str_literal(&mut split, "closest")?;
    aoc::parse::expect_str_literal(&mut split, "beacon")?;
    aoc::parse::expect_str_literal(&mut split, "is")?;
    aoc::parse::expect_str_literal(&mut split, "at")?;
    let beacon_x_str = aoc::parse::expect_word(&mut split, "beacon x")?;
    let beacon_y_str = aoc::parse::expect_word(&mut split, "beacon y")?;

    Ok(SensorReading {
        loc: Point::new(parse_coord(sensor_x_str)?, parse_coord(sensor_y_str)?),
        beacon: Point::new(parse_coord(beacon_x_str)?, parse_coord(beacon_y_str)?),
    })
}

fn parse_coord(input: &str) -> Result<i64> {
    let input = input.strip_suffix(',').unwrap_or(input);
    let input = input.strip_suffix(':').unwrap_or(input);

    let mut split = input.split('=');
    aoc::parse::expect_word(&mut split, "dimm")?;

    aoc::parse::expect_parse(&mut split, "coord")
}

#[derive(Debug)]
struct SensorReading {
    loc: Point,
    beacon: Point,
}

impl SensorReading {
    fn radius(&self) -> i64 {
        let d = self.loc - self.beacon;
        d.x.abs() + d.y.abs()
    }
    fn cross_section_at_y(&self, y: i64) -> Option<(i64, i64)> {
        let vertical_dist = (y - self.loc.y).abs();
        let width = self.radius() - vertical_dist;
        let (l, r) = (self.loc.x - width, self.loc.x + width);
        if l <= r {
            Some((l, r))
        } else {
            None
        }
    }
    fn is_inside_range(&self, point: Point) -> bool {
        let d = self.loc - point;
        let r = d.x.abs() + d.y.abs();
        self.radius() - r > 0
    }
    // fn border(&self) -> impl Iterator<Item = Point> + '_ {
    //     let r = self.radius();
    //     let star = [
    //         (Point::new(self.loc.x, self.loc.y + r), Point::new(1, -1)),
    //         (Point::new(self.loc.x + r, self.loc.y), Point::new(-1, -1)),
    //         (Point::new(self.loc.x, self.loc.y - r), Point::new(-1, 1)),
    //         (Point::new(self.loc.x - r, self.loc.y), Point::new(1, 1)),
    //     ];

        // star.iter()
        //     .flat_map(move |(p, d)| (0..r).map(|e| *p + d.scale(e)))
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day15");
    const EX: &str = include_str!("../../../input/day15_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "4737443")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "11482462818989")
    }

    #[test]
    fn p1_ex() {
        assert_eq!(part1(EX).unwrap().as_str(), "26")
    }
    #[test]
    fn p2_ex() {
        assert_eq!(part2(EX).unwrap().as_str(), "56000011")
    }
}
