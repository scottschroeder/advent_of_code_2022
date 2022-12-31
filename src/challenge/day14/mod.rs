use anyhow::{Context, Result};
use aoc::{grid::pointmap_grid::PointMap, Point};

pub fn part1(input: &str) -> Result<String> {
    let grid = WallGrid {
        has_floor: false,
        ..Default::default()
    };
    let c = count_falling_grains(grid, input)?;
    Ok(format!("{:?}", c))
}

pub fn part2(input: &str) -> Result<String> {
    let grid = WallGrid {
        has_floor: true,
        ..Default::default()
    };
    let c = count_falling_grains(grid, input)?;
    Ok(format!("{:?}", c))
}

fn count_falling_grains(mut grid: WallGrid, input: &str) -> Result<usize> {
    let segments = parse(input)?;
    log::debug!("{:#?}", segments);

    for p in segments.iter().flat_map(|s| s.trace()) {
        grid.add_rock(p)
    }
    grid.calculate_floor();
    let mut c = 0;
    while let Some(grain) = sand_fall(&grid, Point::new(500, 0)) {
        if grid.add_sand(grain) {
            log::debug!("add {}\n{}", grain, grid.grid);
            c += 1;
        } else {
            break;
        }
    }
    Ok(c)
}

fn parse(input: &str) -> Result<Vec<LineSegment>> {
    input
        .lines()
        .map(|l| parse_line_segment(l).with_context(|| format!("could not parse number: {:?}", l)))
        .collect()
}

fn parse_line_segment(input: &str) -> Result<LineSegment> {
    let points = input
        .split("->")
        .map(parse_point)
        .collect::<Result<Vec<_>>>()?;
    Ok(LineSegment { points })
}

fn parse_point(input: &str) -> Result<Point> {
    let input = input.trim();
    let mut split = input.split(',');
    let x: i64 = aoc::parse::expect_parse(&mut split, "point x")?;
    let y: i64 = aoc::parse::expect_parse(&mut split, "point y")?;
    Ok(Point::new(x, y))
}

fn sand_fall(grid: &WallGrid, src: Point) -> Option<Point> {
    const FALL_PRIORITY: &[Point] = &[Point::new(0, 1), Point::new(-1, 1), Point::new(1, 1)];
    let mut grain = src;
    while !grid.in_abyss(&grain) {
        if let Some(fall) = FALL_PRIORITY
            .iter()
            .map(|d| grain + *d)
            .find(|p| grid.is_open(p))
        {
            grain = fall
        } else {
            return Some(grain);
        }
    }
    None
}

#[derive(Default)]
struct WallGrid {
    grid: PointMap<WallData>,
    has_floor: bool,
    floor: Option<i64>,
}

impl WallGrid {
    fn calculate_floor(&mut self) {
        if self.has_floor {
            if let Some(bounds) = self.grid.bounds() {
                self.floor = Some(bounds.max_y + 2);
            }
        }
    }
    fn is_open(&self, pt: &Point) -> bool {
        if let Some(floor) = self.floor {
            if pt.y >= floor {
                return false;
            }
        }
        self.grid.get(pt).is_none()
    }
    fn in_abyss(&self, pt: &Point) -> bool {
        if let Some(bounds) = self.grid.bounds() {
            !self.has_floor && pt.y > bounds.max_y
        } else {
            true
        }
    }
    fn add_sand(&mut self, pt: Point) -> bool {
        self.grid.insert(pt, WallData::Sand).is_none()
    }
    fn add_rock(&mut self, pt: Point) {
        self.grid.insert(pt, WallData::Rock);
    }
}

enum WallData {
    Rock,
    Sand,
}

impl std::fmt::Display for WallData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WallData::Rock => '#',
                WallData::Sand => 'o',
            }
        )
    }
}

#[derive(Debug)]
struct LineSegment {
    points: Vec<Point>,
}

impl LineSegment {
    fn trace(&self) -> impl Iterator<Item = Point> + '_ {
        self.points
            .windows(2)
            .flat_map(|w| iter_line(w[0], w[1]))
            .chain(self.points.last().copied())
    }
}

fn iter_line(start: Point, end: Point) -> impl Iterator<Item = Point> {
    let delta = end - start;
    let unit = Point::new(unit(delta.x), unit(delta.y));
    let count = (delta.x + delta.y).abs();
    assert_eq!(
        (unit.x + unit.y).abs(),
        1,
        "line step must be exactly length 1"
    );

    (0..count).map(move |c| unit.scale(c) + start)
}

fn unit(x: i64) -> i64 {
    if x == 0 {
        0
    } else {
        x / x.abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day14");
    const EX: &str = include_str!("../../../input/day14_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "715")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "25248")
    }
    #[test]
    fn pt1_ex() {
        assert_eq!(part1(EX).unwrap().as_str(), "24")
    }

    #[test]
    fn pt2_ex() {
        assert_eq!(part2(EX).unwrap().as_str(), "93")
    }
}
