use std::{iter, str::FromStr};

use anyhow::{ensure, Context, Result};
use itertools::Itertools;

use super::Packet;

impl FromStr for Packet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let tokens = tokenize(s);
        parse(tokens)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Open,
    Close,
    Comma,
    Int(i32),
}

fn tokenize(s: &str) -> impl Iterator<Item = Result<Token>> + '_ {
    let mut curr_int = String::new();

    // Mark EOF as None, to make lexing easier.
    let tagged = s.chars().map(Some).chain([None]);

    tagged
        .map(move |symbol| {
            // Did we just finish reading an int?
            let non_numeric = matches!(symbol, None | Some('[' | ']' | ','));
            let parse_int = non_numeric && !curr_int.is_empty();

            let int_token = if parse_int {
                let x = curr_int.parse()?;
                curr_int.clear();
                Some(Token::Int(x))
            } else {
                None
            };

            let main_token = match symbol {
                Some(c) => match c {
                    '[' => Some(Token::Open),
                    ']' => Some(Token::Close),
                    ',' => Some(Token::Comma),
                    _ => {
                        curr_int.push(c);
                        None
                    }
                },
                None => None,
            };

            Ok([int_token, main_token].into_iter().flatten())
        })
        .flatten_ok()
}

fn parse(tokens: impl Iterator<Item = Result<Token>>) -> Result<Packet> {
    let mut contexts: Vec<Vec<Packet>> = vec![vec![]];

    for t in validate_commas(tokens) {
        match t? {
            Token::Comma => unreachable!("validate_commas removes commas"),

            // Open a new context.
            Token::Open => contexts.push(vec![]),

            // Close the current context.
            Token::Close => {
                let inner = contexts.pop().unwrap();
                let outer = contexts.last_mut().context("unexpected Close token")?;
                outer.push(Packet::List(inner));
            }

            // Append a value to the current context.
            Token::Int(x) => {
                let curr = contexts.last_mut().unwrap();
                curr.push(Packet::Int(x))
            }
        }

        debug_assert!(!contexts.is_empty());
    }

    ensure!(contexts.len() == 1, "unclosed Open token(s)");

    let mut top_level = contexts.pop().unwrap();
    ensure!(!top_level.is_empty(), "empty token stream");
    ensure!(top_level.len() == 1, "more than one top-level expression");

    Ok(top_level.pop().unwrap())
}

/// Helper function for `parse`.
///
/// This was surprisingly tricky to write.
fn validate_commas(
    tokens: impl Iterator<Item = Result<Token>>,
) -> impl Iterator<Item = Result<Token>> {
    // Insert an extra comma after every '[' and before every ']'.
    let mut preceeding_open = false;
    let with_extras = tokens
        .map_ok(move |t| {
            let open = t == Token::Open;
            let tokens = match t {
                // Edge-case: don't insert two commas between "[]".
                // Since the '[' already generated a comma, leave this one out.
                t @ Token::Close if preceeding_open => [Some(t), None],

                t @ Token::Open => [Some(t), Some(Token::Comma)],
                t @ Token::Close => [Some(Token::Comma), Some(t)],

                t => [Some(t), None],
            };
            preceeding_open = open;

            tokens.into_iter().flatten()
        })
        .flatten_ok();

    // Add a comma at the beginning and end.
    let wrapped = iter::once(Ok(Token::Comma))
        .chain(with_extras)
        .chain([Ok(Token::Comma)]);

    // Ensure the modified sequence is comma-separated.
    let mut preceeding_comma = false;
    let validated = wrapped.map(move |t| {
        let t = t?;
        if matches!(&t, Token::Comma) {
            ensure!(!preceeding_comma, "unexpected comma");
            preceeding_comma = true;
        } else {
            ensure!(preceeding_comma, "expected comma, got {t:?}");
            preceeding_comma = false;
        }
        Ok(t)
    });

    // Remove all commas.
    validated.filter_ok(|t| !matches!(t, Token::Comma))
}
