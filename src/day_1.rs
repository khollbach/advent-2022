use std::cmp::Reverse;

use itertools::Itertools;

use crate::input;

#[test]
fn part_1() {
    let input = input!(1);

    let max_sum: u32 = input
        .split("\n\n")
        .map(|group| {
            let nums = group.lines().map(|line| {
                let n: u32 = line.parse().unwrap();
                n
            });
            nums.sum()
        })
        .max()
        .unwrap();

    dbg!(max_sum);
}

#[test]
fn part_2() {
    let input = input!(1);

    let three_largest: u32 = input
        .split("\n\n")
        .map(|group| {
            let nums = group.lines().map(|line| {
                let n: u32 = line.parse().unwrap();
                n
            });
            let sum: u32 = nums.sum();
            sum
        })
        .sorted_by_key(|&n| Reverse(n))
        .take(3)
        .sum();

    dbg!(three_largest);
}
