use std::collections::HashSet;
use std::fs;

use aoc2022::*;

fn main() {
    let text = fs::read_to_string("input/day6/input.txt").expect("failed to read file");
    let text = text.trim();
    dbg!(&text);
    dbg!(marker_start(text, 4));
    dbg!(marker_start(text, 14));
}

fn marker_start(s: &str, marker_len: usize) -> usize {
    let str_len = s.len();
    let mut set: HashSet<char> = HashSet::new();
    for end_idx in marker_len..str_len {
        set.clear();
        set.extend(s[end_idx - marker_len..end_idx].chars());
        if set.len() == marker_len {
            return end_idx;
        }
    }
    panic!("Marker not found");
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_text_1() {
        let text = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(marker_start(text, 4), 7);
    }

    #[test]
    fn test_text_2() {
        let text = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(marker_start(text, 4), 5);
    }

    #[test]
    fn test_text_3() {
        let text = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(marker_start(text, 4), 6);
    }

    #[test]
    fn test_text_4() {
        let text = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(marker_start(text, 4), 10);
    }

    #[test]
    fn test_text_5() {
        let text = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(marker_start(text, 4), 11);
    }
}
