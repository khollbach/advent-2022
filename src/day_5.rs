use lazy_regex::regex;

use crate::input;

#[test]
fn part_1() {
    let input = input!(5);

    let (stacks, instructions) = input.split_once("\n\n").unwrap();

    let mut stacks: Vec<_> = stacks.lines().map(String::from).collect();

    for line in instructions.lines() {
        let caps = regex!(r"^move (\d+) from (\d+) to (\d+)$")
            .captures(line)
            .unwrap();
        let amount: u32 = caps[1].parse().unwrap();
        let source: usize = caps[2].parse().unwrap();
        let dest: usize = caps[3].parse().unwrap();

        // 0-indexed
        let source = source - 1;
        let dest = dest - 1;

        for _ in 0..amount {
            let c = stacks[source].pop().unwrap();
            stacks[dest].push(c);
        }
    }

    let message: String = stacks
        .iter()
        .map(|s| s.chars().next_back().unwrap())
        .collect();
    dbg!(message);
}
