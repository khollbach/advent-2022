use std::collections::HashSet;

use anyhow::{bail, Context, Result};
use itertools::Itertools;

use crate::{helpers::until_err, input};

#[test]
fn part_1() -> Result<()> {
    let input = input!(9);

    let mut err = Ok(());
    let motions = parse_input(input).scan(&mut err, until_err);
    let answer = simulate(motions);
    err?;

    dbg!(answer);
    Ok(())
}

fn parse_input(input: &str) -> impl Iterator<Item = Result<Point>> + '_ {
    input.lines().map(parse_line)
}

fn parse_line(line: &str) -> Result<Point> {
    let (dir, amount) = line
        .split_whitespace()
        .collect_tuple()
        .context("expected two words")?;

    let (mut x, mut y) = match dir {
        "L" => (-1, 0),
        "R" => (1, 0),
        "D" => (0, -1),
        "U" => (0, 1),
        _ => bail!("not a direction code: {dir:?}"),
    };

    let amount: i32 = amount.parse()?;
    x *= amount;
    y *= amount;

    Ok(Point { x, y })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const ORIGIN: Self = Self { x: 0, y: 0 };
}

/// Return the number of positions visited by "tail".
fn simulate(motions: impl Iterator<Item = Point>) -> usize {
    let mut seen = HashSet::new();

    let mut head = Point::ORIGIN;
    let mut tail = Point::ORIGIN;
    seen.insert(tail);

    for Point { x, y } in motions {
        head.x += x;
        head.y += y;

        while !is_close(head, tail) {
            let dx = head.x - tail.x;
            let dy = head.y - tail.y;
            tail.x += dx.signum();
            tail.y += dy.signum();
            seen.insert(tail);
        }
    }

    seen.len()
}

fn is_close(a: Point, b: Point) -> bool {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    dx.abs() <= 1 && dy.abs() <= 1
}
