use anyhow::Result;

use crate::input;

#[test]
fn part_1() -> Result<()> {
    let input = input!(14);

    Ok(())
}

type Point = (usize, usize);

fn parse_input() -> Vec<Vec<Point>> {}

enum Cell {
    Air,
    Rock,
    Sand,
}

struct Grid {
    /// Rectangular, non-empty.
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn draw_rocks(paths: &[impl AsRef<[Point]>]) -> Self {}

    /// Generate one unit of sand, and then let it fall.
    ///
    /// Keep doing this until either:
    /// * the sand start to fall out of bounds, or
    /// * the source becomes blocked -- in this case, return an error.
    fn simulate(&mut self, sand_source: Point) -> Result<()> {}

    fn count_sand(&self) -> usize {}
}
