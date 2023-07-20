use std::collections::{HashSet, VecDeque};

use anyhow::{ensure, Context, Result};

use crate::input;

#[test]
fn part_1() -> Result<()> {
    let input = input!(12);

    let (grid, start, end) = Grid::parse(input)?;
    let dist = grid.shortest_path(start, end).context("no path exists")?;
    dbg!(dist);

    Ok(())
}

#[test]
fn part_2() -> Result<()> {
    let input = input!(12);

    let (grid, _start, end) = Grid::parse(input)?;
    let dist = grid.shortest_path_2(end).context("no path exists")?;
    dbg!(dist);

    Ok(())
}

/// A non-empty, rectangular grid.
#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<u8>>,
}

type Point = (usize, usize);

impl Grid {
    fn parse(s: &str) -> Result<(Self, Point, Point)> {
        let mut grid: Vec<_> = s.lines().map(|l| l.to_owned().into_bytes()).collect();

        ensure!(!grid.is_empty(), "no rows");
        ensure!(!grid[0].is_empty(), "empty row");
        for row in &grid {
            ensure!(row.len() == grid[0].len(), "jagged grid");
        }

        // Find the special "start" and "end" markers.
        let mut start = None;
        let mut end = None;
        for (i, row) in grid.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                match *col {
                    b'S' => {
                        ensure!(start.is_none(), "multiple starts");
                        start = Some((i, j));
                        *col = b'a';
                    }
                    b'E' => {
                        ensure!(end.is_none(), "multiple ends");
                        end = Some((i, j));
                        *col = b'z';
                    }
                    _ => (),
                }
            }
        }

        Ok((
            Self { grid },
            start.context("no start")?,
            end.context("no end")?,
        ))
    }

    fn dims(&self) -> Point {
        let h = self.grid.len();
        let w = self.grid[0].len();
        (h, w)
    }

    /// Return None if no path exists.
    fn shortest_path(&self, start: Point, end: Point) -> Option<usize> {
        let mut q = VecDeque::new();
        let mut seen = HashSet::new();

        // Discover the initial node.
        seen.insert(start);
        q.push_back((start, 0));

        while let Some((curr, dist)) = q.pop_front() {
            if curr == end {
                return Some(dist);
            }

            for nbr in self.nbrs(curr) {
                if self.has_edge(curr, nbr) && !seen.contains(&nbr) {
                    seen.insert(nbr);
                    q.push_back((nbr, dist + 1));
                }
            }
        }

        None
    }

    /// Find the shortest path starting from any 'a', and ending at `end`.
    fn shortest_path_2(&self, end: Point) -> Option<usize> {
        let mut q = VecDeque::new();
        let mut seen = HashSet::new();

        // Start at the `end` node, and travel backwards along edges.
        seen.insert(end);
        q.push_back((end, 0));

        while let Some((curr, dist)) = q.pop_front() {
            if self.grid[curr.0][curr.1] == b'a' {
                return Some(dist);
            }

            for nbr in self.nbrs(curr) {
                // Note the swapped order of arguments!
                if self.has_edge(nbr, curr) && !seen.contains(&nbr) {
                    seen.insert(nbr);
                    q.push_back((nbr, dist + 1));
                }
            }
        }

        None
    }

    fn has_edge(&self, (i, j): Point, (i2, j2): Point) -> bool {
        // Ascend by at most 1.
        self.grid[i2][j2] <= self.grid[i][j] + 1
    }

    /// Neighboring coords; up to 4 of them.
    fn nbrs(&self, (i, j): Point) -> impl Iterator<Item = Point> {
        let (h, w) = self.dims();

        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(move |(di, dj)| {
                let i2 = checked_add(i, di)?;
                let j2 = checked_add(j, dj)?;

                if i2 < h && j2 < w {
                    Some((i2, j2))
                } else {
                    None
                }
            })
    }
}

/// Return None if the result would be negative.
fn checked_add(x: usize, y: isize) -> Option<usize> {
    let z = (x as isize) + y;
    if z >= 0 {
        Some(z as usize)
    } else {
        None
    }
}
