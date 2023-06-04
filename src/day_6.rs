use crate::prelude::*;

#[test]
fn part_1() {
    let input = input!(6);

    let num_chars = find_magic(input, 4).unwrap();
    dbg!(num_chars);
}

/// Return the number of chars up to and including the magic window.
fn find_magic(input: &str, window_size: usize) -> Option<usize> {
    for (i, window) in input.as_bytes().windows(window_size).enumerate() {
        if all_unique(window) {
            return Some(i + window_size);
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

#[test]
fn part_2() {
    let input = input!(6);

    let num_chars = find_magic(input, 14).unwrap();
    dbg!(num_chars);
}
