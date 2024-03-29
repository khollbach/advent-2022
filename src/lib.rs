#![cfg(test)]

mod helpers;

mod day_1;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;

/// Get the input for a given day, as a &str.
macro_rules! input {
    ($n:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/", $n))
    };
}
use input;

#[allow(unused_macros)]
macro_rules! example {
    ($n:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/", $n))
    };
}
#[allow(unused_imports)]
use example;
