use std::ops::{Add, Sub};

use aoc2022::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {}

impl Add<Delta> for Pos {
    type Output = Self;

    fn add(self, rhs: Delta) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Pos {
    type Output = Delta;

    fn sub(self, rhs: Self) -> Self::Output {
        Delta {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Delta {
    x: i32,
    y: i32,
}

fn main() {
    let lines = read_lines_as_string_int_pairs("input/day9/text.txt").expect("Failed to read input");

    let mut head = Pos { x: 0, y: 0 };
    let mut tail = Pos { x: 0, y: 0 };

    for (dir, steps) in &lines {
        for step in 0..*steps {
            println!("{dir} {step}");
            match dir.as_str() {
                "R" => { head.x += 1; }
                "L" => { head.x -= 1; }
                "U" => { head.y += 1; }
                "D" => { head.y -= 1; }
                _ => panic!("Unexpected direction {dir}")
            }
            let delta = head - tail;
        }
    }
}
