use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Cursor},
    result::Result as StdResult,
};

use anyhow::{bail, ensure, Context, Result};
use lazy_regex::regex;

use crate::input;

#[test]
fn part_1() -> Result<()> {
    let input = input!(7);
    let commands = parse_input(BufReader::new(Cursor::new(input)))?;
    let tree = build_tree(commands)?;
    let sizes = tree.dir_sizes();

    let mut small_sum = 0;
    for s in sizes {
        if s < 100_000 {
            small_sum += s;
        }
    }
    dbg!(small_sum);

    Ok(())
}

/// Assumptions:
/// * all commands are "successful"; either:
///   1. cd into a directory, with no console output
///   2. ls the current directory, outputing zero or more file/dir entries
fn parse_input(input: impl BufRead) -> Result<Vec<Command>> {
    let mut commands = vec![];

    let mut err = Ok(());
    let mut lines = input.lines().scan(&mut err, until_err).peekable();

    while let Some(line) = lines.next() {
        if let Some(caps) = regex!(r"^\$ cd ([^\s]+)$").captures(&line) {
            let dir = caps[1].to_owned();
            commands.push(Command::Cd { dir })
        } else if regex!(r"^\$ ls$").is_match(&line) {
            let mut dirs = vec![];
            let mut files = vec![];

            loop {
                match lines.peek() {
                    None => break,
                    Some(s) if s.starts_with("$") => break,
                    _ => (),
                }

                let line = lines.next().unwrap();
                if let Some(caps) = regex!(r"^dir ([^\s]+)$").captures(&line) {
                    let name = caps[1].to_owned();
                    dirs.push(name);
                } else if let Some(caps) = regex!(r"^(\d+) ([^\s]+)$").captures(&line) {
                    let size = caps[1].parse()?;
                    let name = caps[2].to_owned();
                    files.push(File { _name: name, size });
                } else {
                    bail!("failed to match line against ls entry regexes: {line:?}");
                }
            }

            commands.push(Command::Ls { dirs, files });
        } else {
            bail!("failed to match line against cd/ls regexes: {line:?}");
        }
    }

    err?;

    Ok(commands)
}

/// Helper function for Iterator::scan.
fn until_err<T, E>(err: &mut &mut StdResult<(), E>, item: StdResult<T, E>) -> Option<T> {
    match item {
        Ok(item) => Some(item),
        Err(e) => {
            **err = Err(e);
            None
        }
    }
}

enum Command {
    Cd { dir: String },
    Ls { dirs: Vec<String>, files: Vec<File> },
}

struct File {
    _name: String,
    size: usize,
}

/// Assumptions:
/// * ls is called exactly once per directory
/// * ls is called on the current directory *before* cd-ing into its children
///   * i.e. before we cd into a directory, we already "know" that it exists
fn build_tree(commands: Vec<Command>) -> Result<Tree> {
    // The root directory is its own parent.
    let root = Dir::new(0);
    let mut tree = Tree { dirs: vec![root] };
    let mut ls_called = vec![false];

    // Start at the root.
    let mut curr_dir = 0;

    for c in commands {
        match c {
            Command::Ls { dirs, files } => {
                ensure!(
                    !ls_called[curr_dir],
                    "ls must be called at most once per dir"
                );
                ls_called[curr_dir] = true;

                // Create a new Dir object for each child directory.
                let map = dirs
                    .into_iter()
                    .map(|name| {
                        tree.dirs.push(Dir::new(curr_dir));
                        ls_called.push(false);
                        (name, tree.dirs.len() - 1)
                    })
                    .collect();

                tree.dirs[curr_dir].children = map;
                tree.dirs[curr_dir].files = files;
            }
            Command::Cd { dir } => match dir.as_str() {
                "/" => curr_dir = 0,
                ".." => curr_dir = tree.dirs[curr_dir].parent,
                name => {
                    let idx = *tree.dirs[curr_dir]
                        .children
                        .get(name)
                        .context("dir does not exist")?;
                    curr_dir = idx;
                }
            },
        }
    }

    ensure!(
        ls_called.iter().all(|&b| b),
        "ls must be called at least once per dir"
    );

    Ok(tree)
}

struct Tree {
    /// dirs[0] is the root directory, "/"
    ///
    /// Because of how we build the tree, each node has a smaller index than its
    /// children.
    dirs: Vec<Dir>,
}

#[derive(Default)]
struct Dir {
    parent: usize,
    children: HashMap<String, usize>,
    files: Vec<File>,
}

impl Dir {
    fn new(parent: usize) -> Self {
        Self {
            parent,
            ..Self::default()
        }
    }
}

impl Tree {
    /// We can take advantage of the fact that the `dirs` list is already
    /// topologically sorted -- that is, parents come before children.
    ///
    /// So we'll process the list from right-to-left, so that a node's
    /// dependencies are processed before the node itself.
    fn dir_sizes(&self) -> Vec<usize> {
        let n = self.dirs.len();
        let mut sizes = vec![0; n];

        for i in (0..n).rev() {
            let curr = &self.dirs[i];

            for f in &curr.files {
                sizes[i] += f.size;
            }

            for (_, &child_idx) in &curr.children {
                assert!(i < child_idx);
                sizes[i] += sizes[child_idx];
            }
        }

        sizes
    }
}
