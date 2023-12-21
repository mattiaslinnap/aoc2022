use std::collections::HashSet;

use aoc2022::*;

fn main() {
    let lines = read_lines_as_str("input/day3/input.txt").expect("Failed to read input");
    dbg!(&lines);

    let mut sum = 0;
    for lines in lines.chunks(3) {
        let a: HashSet<char> = lines[0].chars().collect();
        let b: HashSet<char> = lines[1].chars().collect();
        let c: HashSet<char> = lines[2].chars().collect();
        let ab: HashSet<char> = a.intersection(&b).copied().collect();
        let abc: char = *ab.intersection(&c).next().expect("there to be intersection");

        let p = prio(abc);
        sum += p;
        println!("{}, {}", abc, p);
    }
    println!("{}", sum);

    // for g in &groups {
    //     let sum: i32 = g.iter().sum();
    //     dbg!(sum);
    // }
    //dbg!(max);
}

fn prio(c: char) -> i32 {
    let x = c as u8;
    let p = match x {
        b'a'..=b'z' => x - b'a' + 1,
        b'A'..=b'Z' => x - b'A' + 27,
        _ => panic!("Unexpected character '{c}' -> {x}")
    };
    p as i32
}
