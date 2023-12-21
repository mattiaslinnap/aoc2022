#![allow(dead_code, unused)]

use aoc2022::*;

fn main() {
    let lines = read_lines_as_str("input/day25/input.txt").expect("Failed to read input");
    dbg!(&lines);
    let mut sum = 0;
    for line in &lines {
        sum += snafu_to_decimal(line);
    }
    println!("{}", sum);
    println!("{}", decimal_to_snafu(sum));
}

fn snafu_to_decimal(snafu: &str) -> i64 {
    // 2, 1, 0, minus (written -), and double-minus (written =)
    let mut dec = 0;
    let mut base = 1;
    for c in snafu.chars().rev() {
        let digit = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("Unknown digit {c}")
        };
        dec += base * digit;
        base *= 5;
    }
    dec
}

fn decimal_to_snafu(decimal: i64) -> String {
    assert!(decimal >= 0);
    let mut remaining = decimal;
    let mut snafu = String::new();
    let mut base = 1;

    while remaining != 0 {
        let last_decimal_digit = remaining % 5;
        snafu.push(match last_decimal_digit {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => panic!("Should never happen")
        });
        remaining /= 5;
        if last_decimal_digit >= 3 {
            remaining += 1;
        }
    }

    snafu.chars().rev().collect()
}


#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_PAIRS: [(i64, &str); 15] = [
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (15, "1=0"),
        (20, "1-0"),
        (2022, "1=11-2"),
        (12345, "1-0---0"),
        (314159265, "1121-1110-1=0"),
    ];

    #[test]
    fn test_snafu_to_decimal() {
        for (decimal, snafu) in TEST_PAIRS {
            let decimal_res = snafu_to_decimal(snafu);
            assert_eq!(decimal_res, decimal, "Converting snafu {} -> decimal failed, expected {} got {}", snafu, decimal, decimal_res);
        }
    }

    #[test]
    fn test_decimal_to_snafu() {
        for (decimal, snafu) in TEST_PAIRS {
            let snafu_res = decimal_to_snafu(decimal);
            assert_eq!(snafu_res, snafu, "Converting decimal {} -> snafu failed, expected {} got {}", decimal, snafu, snafu_res);
        }
    }
}
