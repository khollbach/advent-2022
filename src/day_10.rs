use std::str::FromStr;

use anyhow::{ensure, Context, Result};
use itertools::Itertools;

use crate::input;

#[test]
fn part_1() -> Result<()> {
    let input = input!(10);

    let instrs: Vec<_> = parse_input(input).collect::<Result<_>>()?;
    let register_values = execute(&instrs);
    let answer = signal_strength_sum(&register_values);
    dbg!(answer);

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    AddX { delta: i32 },
    NoOp,
}

fn parse_input(input: &str) -> impl Iterator<Item = Result<Instr>> + '_ {
    input.lines().map(Instr::from_str)
}

impl FromStr for Instr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s == "noop" {
            Ok(Self::NoOp)
        }
        // Must be an addx operation.
        else {
            let (op, amount) = s
                .split_whitespace()
                .collect_tuple()
                .context("expected exactly two words")?;
            ensure!(op == "addx", "expected op to be addx");
            let amount = amount.parse()?;

            Ok(Self::AddX { delta: amount })
        }
    }
}

fn execute(instrs: &[Instr]) -> Vec<i32> {
    let n = instrs.len();
    let mut register_values = Vec::with_capacity(n * 2 + 1);

    let mut curr_value = 1;
    register_values.push(curr_value);

    for &instr in instrs {
        match instr {
            Instr::NoOp => {
                register_values.push(curr_value);
            }
            Instr::AddX { delta } => {
                register_values.push(curr_value);
                curr_value += delta;
                register_values.push(curr_value);
            }
        }
    }

    register_values.shrink_to_fit();
    register_values
}

fn signal_strength_sum(register_values: &[i32]) -> i32 {
    let mut sss = 0;

    let mut i = 19;
    while i < register_values.len() {
        sss += (i as i32 + 1) * register_values[i];

        i += 40;
    }

    sss
}
