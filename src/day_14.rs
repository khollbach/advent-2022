use std::{
    fmt::{self, Debug},
    ops::Add,
};

use itertools::Itertools;

use crate::input;

#[test]
fn part_1() {
    let input = input!(14);
    let paths = parse_input(input);
    let mut grid = Grid::new(&paths);
    let ans = grid.simulate(Point { x: 500, y: 0 });
    dbg!(ans);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl From<(isize, isize)> for Point {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Point>> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Vec<Point> {
    line.split(" -> ").map(parse_point).collect()
}

fn parse_point(word: &str) -> Point {
    let (x, y) = word
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect_tuple()
        .unwrap();
    Point { x, y }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Air,
    Rock,
    Sand,
}

impl Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Air => write!(f, "."),
            Self::Rock => write!(f, "#"),
            Self::Sand => write!(f, "o"),
        }
    }
}

struct Grid {
    /// Rectangular, non-empty.
    ///
    /// Indexed as `self.cells[y][x]`.
    cells: Vec<Vec<Cell>>,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.cells.len() {
            for x in 0..self.cells[y].len() {
                write!(f, "{:?}", self.cells[y][x])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn new(paths: &[Vec<Point>]) -> Self {
        let points = paths.iter().flat_map(|path| path.iter().map(|&p| p));
        let x_max = points.clone().map(|p| p.x).max().unwrap();
        let y_max = points.clone().map(|p| p.y).max().unwrap();
        let h = y_max as usize + 1;
        let w = x_max as usize + 1;
        let cells = vec![vec![Cell::Air; w]; h];

        let mut this = Self { cells };
        this.draw_paths(paths);
        this
    }

    fn draw_paths(&mut self, paths: &[Vec<Point>]) {
        for path in paths {
            for (p1, p2) in path.iter().copied().tuple_windows() {
                self.draw_line(p1, p2);
            }
        }
    }

    /// Inclusive of both endpoints.
    fn draw_line(&mut self, mut p1: Point, p2: Point) {
        let x_dir = (p2.x - p1.x).signum();
        let y_dir = (p2.y - p1.y).signum();
        assert!(x_dir == 0 || y_dir == 0);

        loop {
            *self.get_mut(p1) = Cell::Rock;
            if p1 == p2 {
                break;
            }

            p1.x += x_dir;
            p1.y += y_dir;
        }
    }

    fn dims(&self) -> Point {
        let y = self.cells.len() as isize;
        let x = self.cells[0].len() as isize;
        Point { x, y }
    }

    fn in_bounds(&self, p: Point) -> bool {
        let dims = self.dims();
        0 <= p.x && p.x < dims.x && 0 <= p.y && p.y < dims.y
    }

    fn get(&self, p: Point) -> Cell {
        assert!(self.in_bounds(p));
        self.cells[p.y as usize][p.x as usize]
    }

    fn get_mut(&mut self, p: Point) -> &mut Cell {
        assert!(self.in_bounds(p));
        &mut self.cells[p.y as usize][p.x as usize]
    }

    fn simulate(&mut self, sand_source: Point) -> usize {
        assert!(self.in_bounds(sand_source));
        while self.drop_sand(sand_source) {}
        self.count_sand()
    }

    /// Generate one unit of sand, and then let it fall.
    ///
    /// Return true if the sand got "stuck" somewhere.
    /// Return false if it fell out-of-bounds.
    /// Panic if the source itself is blocked.
    fn drop_sand(&mut self, sand_source: Point) -> bool {
        assert_eq!(self.get(sand_source), Cell::Air);

        let mut curr = sand_source;
        *self.get_mut(curr) = Cell::Sand;

        loop {
            match self.step_sand(curr) {
                StepSand::Stuck => return true,
                StepSand::OutOfBounds => return false,
                StepSand::MovedTo(next) => curr = next,
            }
        }
    }

    fn step_sand(&mut self, curr: Point) -> StepSand {
        assert_eq!(self.get(curr), Cell::Sand);
        for delta in [(0, 1), (-1, 1), (1, 1)] {
            let next = curr + delta.into();
            if !self.in_bounds(next) {
                // fall off the edge of the world
                *self.get_mut(curr) = Cell::Air;
                return StepSand::OutOfBounds;
            }
            if self.get(next) == Cell::Air {
                *self.get_mut(curr) = Cell::Air;
                *self.get_mut(next) = Cell::Sand;
                return StepSand::MovedTo(next);
            }
        }
        StepSand::Stuck
    }

    fn count_sand(&self) -> usize {
        let sand = self
            .cells
            .iter()
            .flat_map(|row| row.iter().filter(|&&cell| cell == Cell::Sand));
        sand.count()
    }
}

enum StepSand {
    Stuck,
    OutOfBounds,
    MovedTo(Point),
}
