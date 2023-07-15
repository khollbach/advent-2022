use anyhow::{bail, Result};

use crate::input;

#[test]
fn part_1() -> Result<()> {
    let input = input!(8);
    let grid = Grid::parse(input)?;
    dbg!(grid.num_visible_trees());
    Ok(())
}

#[test]
fn part_2() -> Result<()> {
    let input = input!(8);
    let grid = Grid::parse(input)?;
    dbg!(grid.best_score());
    Ok(())
}

// A non-empty, rectangular grid of digits.
#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<u8>>,
}

/// Helper for Grid::parse.
fn char_to_digit(c: char) -> Result<u8> {
    match c.to_digit(10) {
        Some(d) => Ok(d as u8),
        None => bail!("non-digit char {c:?}"),
    }
}

impl Grid {
    fn parse(input: &str) -> Result<Self> {
        let grid: Result<Vec<Vec<u8>>> = input
            .lines()
            .map(|line| line.chars().map(char_to_digit).collect())
            .collect();
        let grid = grid?;

        if grid.is_empty() {
            bail!("empty grid not allowed");
        }

        let width = grid[0].len();
        if width == 0 {
            bail!("empty rows not allowed");
        }

        for (i, row) in grid.iter().enumerate() {
            let len = row.len();
            if len != width {
                bail!("row 0 and row {i} have different widths: {width} vs {len}");
            }
        }

        Ok(Grid { grid })
    }

    fn dims(&self) -> (usize, usize) {
        let w = self.grid[0].len();
        let h = self.grid.len();
        (w, h)
    }

    /// Idea: send a "probe" along each row/col from both directions.
    ///
    /// When you see a tree that's taller than all the previous ones, it's
    /// marked 'visible'. All other trees aren't visible (at least from that
    /// direction), but you keep probing in case you run into another tall tree.
    fn num_visible_trees(&self) -> usize {
        let (w, h) = self.dims();
        let mut is_visible = vec![vec![false; w]; h];

        for i in 0..h {
            // Send probes along row i in both directions.
            self.probe((0..w).map(|j| (i, j)), &mut is_visible);
            self.probe((0..w).rev().map(|j| (i, j)), &mut is_visible);
        }
        for j in 0..w {
            // Send probes along col j in both directions.
            self.probe((0..h).map(|i| (i, j)), &mut is_visible);
            self.probe((0..h).rev().map(|i| (i, j)), &mut is_visible);
        }

        // Count 'em up.
        let mut num_visible = 0;
        for row in is_visible {
            for col in row {
                if col {
                    num_visible += 1;
                }
            }
        }
        num_visible
    }

    /// Helper for num_visible_trees.
    fn probe(
        &self,
        mut coords: impl Iterator<Item = (usize, usize)>,
        is_visible: &mut Vec<Vec<bool>>,
    ) {
        let (i, j) = coords.next().unwrap();
        let mut tallest_so_far = self.grid[i][j];
        is_visible[i][j] = true;

        for (i, j) in coords {
            if self.grid[i][j] > tallest_so_far {
                is_visible[i][j] = true;
                tallest_so_far = self.grid[i][j];
            }
        }
    }

    fn best_score(&self) -> usize {
        let (w, h) = self.dims();

        (0..h)
            .flat_map(|i| (0..w).map(move |j| self.score(i, j)))
            .max()
            .unwrap()
    }

    fn score(&self, i: usize, j: usize) -> usize {
        let (w, h) = self.dims();
        let limit = self.grid[i][j];

        // look in each NESW, count trees (but stopping at/after the first tree that's as tall as us)
        let mut total = 1;

        // rows
        total *= self.probe_2((j + 1..w).map(|j2| (i, j2)), limit);
        total *= self.probe_2((0..j).rev().map(|j2| (i, j2)), limit);

        // cols
        total *= self.probe_2((i + 1..h).map(|i2| (i2, j)), limit);
        total *= self.probe_2((0..i).rev().map(|i2| (i2, j)), limit);

        total
    }

    /// Helper for `score`.
    fn probe_2(&self, coords: impl Iterator<Item = (usize, usize)>, limit: u8) -> usize {
        let mut count = 0;
        for (i, j) in coords {
            count += 1;

            // The first tall tree marks the end of our line-of-sight.
            if self.grid[i][j] >= limit {
                break;
            }
        }
        count
    }
}
