use std::fmt::{Debug, Formatter};

use regex::Regex;

use aoc2022::*;

struct Stacks {
    stacks: Vec<String>,
}


impl Stacks {
    fn do_move_single(&mut self, count: usize, from: usize, to: usize) {
        let from = from - 1;
        let to = to - 1;
        for _ in 0..count {
            let item = self.stacks[from].pop().expect("not enough boxes");
            self.stacks[to].push(item);
        }
    }

    fn do_move_many(&mut self, count: usize, from: usize, to: usize) {
        if from == to {
            return;
        }
        // From now on can use unsafe access

        let from = from - 1;
        let to = to - 1;

        dbg!(&self.stacks);
        println!("Moving {} from {} to {}", count, &from, &to);

        let (from, to) = access::independent_index(&mut self.stacks, from, to);

        let from_idx = from.len() - count;
        let (_, after) = from.split_at(from_idx);

        to.push_str(after);
        from.truncate(from_idx);
        dbg!(&self.stacks);
        println!();
    }

    fn tops(&self) -> String {
        self.stacks.iter().map(|s| s.chars().last().unwrap_or('-')).collect()
    }
}


impl Debug for Stacks {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for (pos, stack) in self.stacks.iter().enumerate() {
            write!(f, "\n")?;
            write!(f, "{}: {}", pos + 1, stack)?;
        }
        Ok(())
    }
}

fn main() {
    let mut s = Stacks::new_input();
    let lines = read_lines_as_str("input/day5/input.txt").expect("Failed to read input");

    let move_regex = Regex::new(r"^move ([0-9]+) from ([0-9]+) to ([0-9]+)$").unwrap();

    //let lines = vec!["Hello world"];

    for line in &lines {
        if let Some(numstrs) = captured_strs(&move_regex, line) {
            let nums: Vec<usize> = numstrs.iter().map(|s| s.parse().expect("numbers")).collect();
            let count = nums[0];
            let from = nums[1];
            let to = nums[2];
            println!("{}", line);
            s.do_move_many(count, from, to);
        }
    }

    dbg!(&s);
    println!("Tops: {}", s.tops());
}


impl Stacks {
    fn new_text() -> Self {
        //     [D]
        // [N] [C]
        // [Z] [M] [P]
        //  1   2   3
        Stacks {
            stacks: vec![
                String::from("ZN"),
                String::from("MCD"),
                String::from("P"),
            ]
        }
    }

    fn new_input() -> Self {
        //         [H]     [W] [B]
        //     [D] [B]     [L] [G] [N]
        // [P] [J] [T]     [M] [R] [D]
        // [V] [F] [V]     [F] [Z] [B]     [C]
        // [Z] [V] [S]     [G] [H] [C] [Q] [R]
        // [W] [W] [L] [J] [B] [V] [P] [B] [Z]
        // [D] [S] [M] [S] [Z] [W] [J] [T] [G]
        // [T] [L] [Z] [R] [C] [Q] [V] [P] [H]
        //  1   2   3   4   5   6   7   8   9
        Stacks {
            stacks: vec![
                String::from("TDWZVP"),
                String::from("LSWVFJD"),
                String::from("ZMLSVTBH"),
                String::from("RSJ"),
                String::from("CZBGFMLW"),
                String::from("QWVHZRGB"),
                String::from("VJPCMDN"),
                String::from("PTBQ"),
                String::from("HGZRC"),
            ]
        }
    }
}
