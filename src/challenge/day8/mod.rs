use anyhow::{Context, Result};
use aoc::grid::compass::Direction;
use aoc::grid::fixed_grid::FixedGrid;
use aoc::grid::grid_types::GridWidth;
use aoc::Point;

pub fn part1(input: &str) -> Result<String> {
    let grid = aoc::grid::fixed_grid::FixedGrid::parse_ascii_grid(input, char_to_int)
        .context("could not parse input grid")?;
    log::debug!("\n{}", grid);

    let views = build_view_tree(&grid);
    log::debug!("\n{}", views);

    let visible_trees = grid
        .points()
        .filter(|pt| {
            let h = grid[*pt];
            let view = &views[*pt];
            h > view.min_view()
        })
        .count();

    Ok(format!("{:?}", visible_trees))
}

pub fn part2(input: &str) -> Result<String> {
    let grid = aoc::grid::fixed_grid::FixedGrid::parse_ascii_grid(input, char_to_int)
        .context("could not parse input grid")?;

    log::debug!("trees\n{}", grid);

    // let s = all_senic_scores(&grid);
    // log::debug!("senic\n{}", s);

    let best_score = grid
        .points()
        .map(|pt| senic_score(&grid, pt))
        .max()
        .ok_or_else(|| anyhow::anyhow!("no trees had a positive senic score"))?;

    Ok(format!("{:?}", best_score))
}

fn iterate_sightline(
    trees: &FixedGrid<i64>,
    src: Point<i64>,
    direction: Direction,
) -> impl Iterator<Item = i64> + '_ {
    let delta = direction.delta();
    (1..)
        .map(move |distance| {
            let offset = delta.scale(distance);
            let view_pt = src + offset;
            trees.maybe_point_to_idx(view_pt)
        })
        .take_while(|x| x.is_some())
        .flatten()
        .map(|idx| trees.as_slice()[idx])
}

fn total_sightline(trees: &FixedGrid<i64>, src: Point<i64>, direction: Direction) -> i64 {
    let height_our_tree = trees[src];
    let mut view_distance = 0;
    for view_tree in iterate_sightline(trees, src, direction) {
        view_distance += 1;
        if view_tree >= height_our_tree {
            break;
        }
    }
    view_distance
}

fn senic_score(trees: &FixedGrid<i64>, pt: Point<i64>) -> i64 {
    Direction::iter()
        .map(|d| total_sightline(trees, pt, d))
        .product()
}

#[derive(Debug, Default)]
struct CardinalView {
    north: i64,
    south: i64,
    east: i64,
    west: i64,
}

impl std::fmt::Display for CardinalView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mv = self.min_view();
        if mv < 0 {
            write!(f, "x")
        } else {
            write!(f, "{}", mv)
        }
    }
}

impl CardinalView {
    fn min_view(&self) -> i64 {
        std::cmp::min(
            self.north,
            std::cmp::min(self.south, std::cmp::min(self.east, self.west)),
        )
    }
}

fn build_view_tree(trees: &FixedGrid<i64>) -> FixedGrid<CardinalView> {
    let mut views: FixedGrid<CardinalView> = FixedGrid::from_dimm(trees.height(), trees.width());

    for idx in 0..views.width() {
        let mut local_max = -1;
        for idy in 0..views.height() {
            let view_point = Point::new(idx, idy);
            let tree_height = trees[view_point];
            views[view_point].north = local_max;
            local_max = std::cmp::max(local_max, tree_height);
        }
    }

    for idx in 0..views.width() {
        let mut local_max = -1;
        for idy in (0..views.height()).rev() {
            let view_point = Point::new(idx, idy);
            let tree_height = trees[view_point];
            views[view_point].south = local_max;
            local_max = std::cmp::max(local_max, tree_height);
        }
    }

    for idy in 0..views.height() {
        let mut local_max = -1;
        for idx in 0..views.width() {
            let view_point = Point::new(idx, idy);
            let tree_height = trees[view_point];
            views[view_point].west = local_max;
            local_max = std::cmp::max(local_max, tree_height);
        }
    }

    for idy in 0..views.height() {
        let mut local_max = -1;
        for idx in (0..views.width()).rev() {
            let view_point = Point::new(idx, idy);
            let tree_height = trees[view_point];
            views[view_point].east = local_max;
            local_max = std::cmp::max(local_max, tree_height);
        }
    }

    views
}

fn char_to_int(c: char) -> Result<i64> {
    let value = c as i64 - '0' as i64;
    if (0..10).contains(&value) {
        Ok(value)
    } else {
        Err(anyhow::anyhow!("char `{}` was not an ascii number", c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day8");
    const EX: &str = include_str!("../../../input/day8_ex");

    #[test]
    fn ascii_int_parser() {
        assert_eq!(char_to_int('0').unwrap(), 0);
        assert_eq!(char_to_int('1').unwrap(), 1);
        assert_eq!(char_to_int('9').unwrap(), 9);
        assert!(char_to_int('/').is_err());
        assert!(char_to_int(':').is_err());
        assert!(char_to_int('a').is_err());
    }

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "1825")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "235200")
    }

    #[test]
    fn p1_ex() {
        assert_eq!(part1(EX).unwrap().as_str(), "21")
    }
    #[test]
    fn p2_ex() {
        assert_eq!(part2(EX).unwrap().as_str(), "8")
    }
}
