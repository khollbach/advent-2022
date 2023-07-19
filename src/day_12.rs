use anyhow::{ensure, Context, Result};

use crate::input;

#[test]
fn part_1() -> Result<()> {
    let input = input!(12);

    let (grid, start, end) = Grid::parse(input)?;
    let answer = grid.shortest_path(start, end);
    dbg!(answer);

    Ok(())
}

/// A non-empty, rectangular grid.
#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<u8>>,
}

impl Grid {
    fn parse(s: &str) -> Result<(Self, (usize, usize), (usize, usize))> {
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

    fn dims(&self) -> (usize, usize) {
        let h = self.grid.len();
        let w = self.grid[0].len();
        (h, w)
    }

    fn shortest_path(&self, start: (usize, usize), end: (usize, usize)) -> usize {
        todo!()
    }
}
