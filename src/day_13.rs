mod parse;

use std::{cmp::Ordering, iter::zip};

use anyhow::{ensure, Context, Result};

use crate::input;

#[test]
fn part_1() -> Result<()> {
    let input = input!(13);
    let packets = read_input(input)?;

    let ans: usize = zip(1.., packets)
        .map(|(i, (p1, p2))| if p1 <= p2 { i } else { 0 })
        .sum();
    dbg!(ans);

    Ok(())
}

#[test]
fn part_2() -> Result<()> {
    let input = input!(13);
    let packets = read_input(input)?;

    let d1: Packet = "[[2]]".parse()?;
    let d2: Packet = "[[6]]".parse()?;

    let mut packets: Vec<_> = packets
        .into_iter()
        .flat_map(|(p1, p2)| [p1, p2])
        .chain([d1.clone(), d2.clone()])
        .collect();
    packets.sort_unstable();

    let (i1, _) = packets.iter().enumerate().find(|(_, p)| **p == d1).unwrap();
    let (i2, _) = packets.iter().enumerate().find(|(_, p)| **p == d2).unwrap();

    let ans = (i1 + 1) * (i2 + 1);
    dbg!(ans);

    Ok(())
}

fn read_input(input: &str) -> Result<Vec<(Packet, Packet)>> {
    let mut out = vec![];
    let mut lines = input.lines();

    while let Some(line1) = lines.next() {
        let p1 = line1.parse()?;

        let line2 = lines.next().context("expected a second packet")?;
        let p2 = line2.parse()?;

        out.push((p1, p2));

        // Expect a blank line, or EOF.
        let Some(blank) = lines.next() else {
            break;
        };
        ensure!(
            blank.chars().all(|c| c.is_whitespace()),
            "expected blank line, got: {blank:?}",
        );
    }

    Ok(out)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Int(i32),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::List(l1), Self::List(l2)) => {
                for (p1, p2) in zip(l1, l2) {
                    match p1.cmp(p2) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => (),
                    }
                }

                l1.len().cmp(&l2.len())
            }

            (Self::Int(x), Self::Int(y)) => x.cmp(&y),

            (Self::Int(x), Self::List(_)) => {
                let list_self = Packet::List(vec![Packet::Int(*x)]);
                list_self.cmp(other)
            }

            (Self::List(_), Self::Int(y)) => {
                let list_other = Packet::List(vec![Packet::Int(*y)]);
                self.cmp(&list_other)
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
