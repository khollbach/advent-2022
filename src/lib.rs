#![cfg(test)]

mod day_1;
mod day_5;

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

pub(crate) use input;
