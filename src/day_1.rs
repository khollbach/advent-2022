use crate::input;

#[test]
fn main() {
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
