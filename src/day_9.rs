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

#[test]
fn part_2() -> Result<()> {
    let input = input!(9);

    let mut err = Ok(());
    let motions = parse_input(input).scan(&mut err, until_err);
    let answer = simulate_part_2(motions);
    err?;

    dbg!(answer);
    Ok(())
}

fn parse_input(input: &str) -> impl Iterator<Item = Result<Motion>> + '_ {
    input.lines().map(parse_line)
}

fn parse_line(line: &str) -> Result<Motion> {
    let (dir, amount) = line
        .split_whitespace()
        .collect_tuple()
        .context("expected two words")?;

    let (x, y) = match dir {
        "L" => (-1, 0),
        "R" => (1, 0),
        "D" => (0, -1),
        "U" => (0, 1),
        _ => bail!("not a direction code: {dir:?}"),
    };

    let amount: u32 = amount.parse()?;

    Ok(Motion {
        dir: Point { x, y },
        amount,
    })
}

#[derive(Debug, Clone, Copy)]
struct Motion {
    dir: Point,
    amount: u32,
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
fn simulate(motions: impl Iterator<Item = Motion>) -> usize {
    let mut seen = HashSet::new();

    let mut head = Point::ORIGIN;
    let mut tail = Point::ORIGIN;
    seen.insert(tail);

    for m in motions {
        for _ in 0..m.amount {
            head.x += m.dir.x;
            head.y += m.dir.y;

            if !is_close(head, tail) {
                let dx = head.x - tail.x;
                let dy = head.y - tail.y;
                tail.x += dx.signum();
                tail.y += dy.signum();
                seen.insert(tail);
            }
        }
    }

    seen.len()
}

fn is_close(a: Point, b: Point) -> bool {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    dx.abs() <= 1 && dy.abs() <= 1
}

fn simulate_part_2(motions: impl Iterator<Item = Motion>) -> usize {
    let mut seen = HashSet::new();

    // The head is knots[0] and the tail is knots[9].
    let n = 10;
    let mut knots = vec![Point::ORIGIN; n];
    seen.insert(knots[n - 1]);

    for m in motions {
        for _ in 0..m.amount {
            // Update head.
            knots[0].x += m.dir.x;
            knots[0].y += m.dir.y;

            for i in 0..n - 1 {
                if !is_close(knots[i], knots[i + 1]) {
                    let dx = knots[i].x - knots[i + 1].x;
                    let dy = knots[i].y - knots[i + 1].y;
                    knots[i + 1].x += dx.signum();
                    knots[i + 1].y += dy.signum();
                }
            }

            seen.insert(knots[n - 1]);
        }
    }

    seen.len()
}
