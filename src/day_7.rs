use crate::prelude::*;

#[test]
fn part_1() {
    let input = input!(7);
    let commands = parse_input(input)?;
    let tree = build_tree(commands)?;
    let sizes = tree.dir_sizes();

    let mut small_sum = 0;
    for s in sizes {
        if s < 100_000 {
            small_sum += s;
        }
    }
    dbg!(small_sum);
}

/// Assumptions:
/// * all commands are "successful"; either:
///   1. cd into a directory, with no console output
///   2. ls the current directory, outputing zero or more file/dir entries
fn parse_input(input: &str) -> Result<Vec<Command>> {
    todo!()
}

enum Command {
    Cd { dir: String },
    Ls { dirs: Vec<String>, files: Vec<File> },
}

struct File {
    name: String,
    size: usize,
}

/// Assumptions:
/// * ls is called exactly once per directory
/// * ls is called on the current directory *before* cd-ing into its children
///   * i.e. before we cd into a directory, we already "know" that it exists
fn build_tree(commands: impl Iterator<Item = Command>) -> Result<Tree> {
    todo!()
}

struct Tree {
    /// dirs[0] is the root directory, "/"
    /// 
    /// Because of how we build the tree, each node has a smaller index than its
    /// children.
    dirs: Vec<Dir>,
}

struct Dir {
    parent: usize,
    name: String,
    dirs: Vec<usize>,
    files: Vec<File>,
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

            for &child_idx in &curr.dirs {
                assert!(i < child_idx);
                sizes[i] += sizes[child_idx];
            }
        }

        sizes
    }
}
