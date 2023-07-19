use std::{mem, cmp::Reverse};

use anyhow::Result;
use itertools::Itertools;

use crate::input;

#[test]
fn part_1() -> Result<()> {
    let _input = input!(11);

    let answer = Monkeys::new_part1().play_game(20);
    dbg!(answer);

    Ok(())
}

#[test]
fn part_2() -> Result<()> {
    let answer = Monkeys::new_part2().play_game(10_000);
    dbg!(answer);

    Ok(())
}

/// Yes, I really just typed this out by hand.
fn hardcoded_monkeys() -> [Monkey; 8] {
    let args: [(_, fn(u32) -> u32, _, _, _); 8] = [
        (vec![57], |x| x * 13, 11, 3, 2),
        (vec![58, 93, 88, 81, 72, 73, 65], |x| x + 2, 7, 6, 7),
        (vec![65, 95], |x| x + 6, 13, 3, 5),
        (vec![58, 80, 81, 83], |x| x.pow(2), 5, 4, 5),
        (vec![58, 89, 90, 96, 55], |x| x + 3, 3, 1, 7),
        (vec![66, 73, 87, 58, 62, 67], |x| x * 7, 17, 4, 1),
        (vec![85, 55, 89], |x| x + 4, 2, 2, 0),
        (vec![73, 80, 54, 94, 90, 52, 69, 58], |x| x + 7, 19, 6, 0),
    ];

    args.map(|(a, b, c, d, e)| Monkey::new(a, b, c, d, e))
}

struct Monkeys {
    monkeys: [Monkey; 8],
    divide_by_three: bool,
}

impl Monkeys {
    fn new_part1() -> Self {
        Self {
            monkeys: hardcoded_monkeys(),
            divide_by_three: true,
        }
    }

    fn new_part2() -> Self {
        Self {
            monkeys: hardcoded_monkeys(),
            divide_by_three: false,
        }
    }

    /// Return the level of monkey business after this many rounds.
    fn play_game(mut self, num_rounds: usize) -> usize {
        for _ in 0..num_rounds {
            self.round();
        }

        self.monkey_business()
    }

    fn round(&mut self) {
        for i in 0..self.monkeys.len() {
            self.turn(i);
        }
    }

    /// Take monkey `i`s turn.
    fn turn(&mut self, i: usize) {
        for item in mem::take(&mut self.monkeys[i].items) {
            let m = &mut self.monkeys[i];
            m.num_items_inspected += 1;

            let mut new_item = (m.operation)(item);
            if self.divide_by_three {
                new_item /= 3;
            }

            let target = if new_item % m.test_divisible_by == 0 {
                m.true_target
            } else {
                m.false_target
            };

            self.monkeys[target].items.push(new_item);
        }
    }

    fn monkey_business(&self) -> usize {
        self.monkeys
            .iter()
            .map(|m| m.num_items_inspected)
            .sorted_by_key(|&n| Reverse(n))
            .take(2)
            .product()
    }
}

struct Monkey {
    items: Vec<u32>,
    /// Remember to divide by 3 after.
    operation: fn(u32) -> u32,
    test_divisible_by: u32,
    true_target: usize,
    false_target: usize,

    num_items_inspected: usize,
}

impl Monkey {
    const fn new(
        items: Vec<u32>,
        operation: fn(u32) -> u32,
        test_divisible_by: u32,
        true_target: usize,
        false_target: usize,
    ) -> Self {
        Self {
            items,
            operation,
            test_divisible_by,
            true_target,
            false_target,
            num_items_inspected: 0,
        }
    }
}
