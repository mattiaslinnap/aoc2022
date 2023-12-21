use std::collections::HashMap;

use aoc2022::*;

#[derive(Copy, Clone, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
    Lose,
    Draw,
    Win,
}

fn main() {
    let pairs = read_lines_as_string_pairs("input/day2/input.txt").expect("Failed to read input");

    let mut mapping = HashMap::new();
    mapping.insert(String::from("A"), Hand::Rock);
    mapping.insert(String::from("B"), Hand::Paper);
    mapping.insert(String::from("C"), Hand::Scissors);
    mapping.insert(String::from("X"), Hand::Lose);
    mapping.insert(String::from("Y"), Hand::Draw);
    mapping.insert(String::from("Z"), Hand::Win);

    let hands = map_pairs(&pairs, &mapping);
    dbg!(&hands);
    let scores: Vec<i32> = hands.iter().map(score).collect();
    dbg!(&scores);
    dbg!(&scores.iter().sum::<i32>());
}

fn map_pairs(pairs: &Vec<(String, String)>, mapping: &HashMap<String, Hand>) -> Vec<(Hand, Hand)> {
    pairs.iter().map(|(a, b)| (mapping.get(a).copied().unwrap(), mapping.get(b).copied().unwrap())).collect()
}

fn score((a, b): &(Hand, Hand)) -> i32 {
    use Hand::*;

    let win_points = match b {
        Lose => 0,
        Draw => 3,
        Win => 6,
        _ => panic!("Unexpected hand {b:?}")
    };

    // 1 for Rock, 2 for Paper, and 3 for Scissors
    let choice_points = match b {
        Lose => match a {
            Rock => 3,
            Paper => 1,
            Scissors => 2,
            _ => panic!("Unexpected hand {a:?}")
        },
        Draw => match a {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
            _ => panic!("Unexpected hand {a:?}")
        },
        Win => match a {
            Rock => 2,
            Paper => 3,
            Scissors => 1,
            _ => panic!("Unexpected hand {a:?}")
        },
        _ => panic!("Unexpected hand {b:?}")
    };
    return choice_points + win_points;
}
