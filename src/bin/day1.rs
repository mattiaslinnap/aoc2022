use aoc2022::*;

fn main() {
    let groups = read_line_groups_as_ints("input/day1/input1.txt").expect("Failed to read input");
    let mut group_total: Vec<i32> = groups.iter().map(|g| g.iter().sum::<i32>()).collect();
    group_total.sort_unstable();

    dbg!(&group_total);
    dbg!(group_total.iter().rev().take(3).sum::<i32>());
    dbg!(&group_total[(group_total.len() - 3)..].iter().sum::<i32>());

    // for g in &groups {
    //     let sum: i32 = g.iter().sum();
    //     dbg!(sum);
    // }
    //dbg!(max);
}
