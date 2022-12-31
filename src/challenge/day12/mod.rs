use anyhow::{anyhow, Context, Result};
use aoc::grid::{compass::Direction, fixed_grid::FixedGrid};
use petgraph::{data::DataMap, graph::NodeIndex};

type TransitGraph = petgraph::graph::DiGraph<i64, ()>;

pub fn part1(input: &str) -> Result<String> {
    let grid = aoc::grid::fixed_grid::FixedGrid::parse_ascii_grid(input, parse_grid_square)
        .context("could not parse grid")?;

    log::trace!("\n{}", grid);

    let (start, end, g) = transit_graph(&grid, |src, dst| dst - src <= 1)?;

    let c = petgraph::algo::dijkstra(&g, start, Some(end), |_| 1);

    let x = c
        .get(&end)
        .copied()
        .ok_or_else(|| anyhow::anyhow!("no path could be found"))?;

    Ok(format!("{:?}", x))
}

pub fn part2(input: &str) -> Result<String> {
    let grid = aoc::grid::fixed_grid::FixedGrid::parse_ascii_grid(input, parse_grid_square)
        .context("could not parse grid")?;

    let (_, end, g) = transit_graph(&grid, |src, dst| src - dst <= 1)?;

    let c = petgraph::algo::dijkstra(&g, end, None, |_| 1);

    let x = c
        .iter()
        .filter_map(|(nidx, dist)| g.node_weight(*nidx).map(|h| (*h, dist)))
        .filter(|(h, _)| *h == 0)
        .map(|(_, d)| *d)
        .min()
        .ok_or_else(|| anyhow::anyhow!("there was no path found"))?;

    Ok(format!("{:?}", x))
}

fn parse_grid_square(c: char) -> Result<GridSquare> {
    Ok(match c {
        'S' => GridSquare::Start,
        'E' => GridSquare::End,
        _ => GridSquare::Elevation(c as i64 - 'a' as i64),
    })
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum GridSquare {
    Start,
    End,
    Elevation(i64),
}

impl GridSquare {
    fn height(self) -> i64 {
        match self {
            GridSquare::Start => 0,
            GridSquare::End => 25,
            GridSquare::Elevation(x) => x,
        }
    }
}

impl std::fmt::Display for GridSquare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            GridSquare::Start => 'S',
            GridSquare::End => 'E',
            GridSquare::Elevation(c) => ('a' as i64 + *c) as u8 as char,
        };
        write!(f, "{}", c)
    }
}

fn transit_graph<F>(
    grid: &FixedGrid<GridSquare>,
    f: F,
) -> Result<(NodeIndex, NodeIndex, TransitGraph)>
where
    F: Fn(i64, i64) -> bool,
{
    let mut graph = TransitGraph::new();

    let mut start = None;
    let mut end = None;

    let node_map = grid
        .raw_iter()
        .map(|e| graph.add_node(e.height()))
        .collect::<Vec<_>>();

    for (src_idx, src_entry) in grid.as_slice().iter().enumerate() {
        let src = grid.idx_to_point(src_idx);
        let src_height = src_entry.height();

        if start.is_none() && *src_entry == GridSquare::Start {
            start = Some(node_map[src_idx]);
        }
        if end.is_none() && *src_entry == GridSquare::End {
            end = Some(node_map[src_idx]);
        }

        for d in Direction::iter() {
            let dst = src + d.delta();
            let dst_idx = match grid.maybe_point_to_idx(dst) {
                Some(p) => p,
                None => continue,
            };

            if f(src_height, grid[dst].height()) {
                graph.add_edge(node_map[src_idx], node_map[dst_idx], ());
            }
        }
    }

    let start = start.ok_or_else(|| anyhow::anyhow!("no start node found"))?;
    let end = end.ok_or_else(|| anyhow::anyhow!("no start node found"))?;

    Ok((start, end, graph))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day12");
    const EX: &str = include_str!("../../../input/day12_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "350")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "349")
    }

    #[test]
    fn p1_ex() {
        assert_eq!(part1(EX).unwrap().as_str(), "31")
    }
    #[test]
    fn p2_ex() {
        assert_eq!(part2(EX).unwrap().as_str(), "29")
    }
}
