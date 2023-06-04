use crate::prelude::*;

#[test]
fn part_1() {
    let input = input!(6);

    let num_chars = find_magic(input).unwrap();
    dbg!(num_chars);
}

/// Return the number of chars up to and including the magic 4.
fn find_magic(input: &str) -> Option<usize> {
    for (i, window) in input.as_bytes().windows(4).enumerate() {
        if all_unique(window) {
            return Some(i + 4);
        }
    }

    None
}

fn all_unique(window: &[u8]) -> bool {
    let n = window.len();
    (0..n).all(|i| {
        (i + 1..n).all(|j| {
            window[i] != window[j]
        })
    })
}
