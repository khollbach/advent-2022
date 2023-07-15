#![cfg(test)]

mod helpers;

mod day_1;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

/// Get the input for a given day, as a &str.
macro_rules! input {
    ($n:expr) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/",
            stringify!($n)
        ))
    };
}
use input;

#[allow(unused_macros)]
macro_rules! example {
    ($n:expr) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/examples/",
            stringify!($n)
        ))
    };
}
#[allow(unused_imports)]
use example;
