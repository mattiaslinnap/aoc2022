use aoc2022::*;

fn main() {
    let pairs = read_lines_as_numrange_pairs("input/day4/input.txt").expect("Failed to read input");
    for pair in &pairs {
        println!("{:?} {:?}", &pair.0, &pair.1);
    }
    let count = pairs.iter().filter(|(a, b)| a.overlaps(&b)).count();
    println!("{}", count);
    //dbg!(&pairs);
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_text_first() {
        let pairs = read_lines_as_numrange_pairs("input/day4/text.txt").expect("Failed to read input");
        let count = pairs.iter().filter(|(a, b)| a.contains(&b) || b.contains(&a)).count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_text_second() {
        let pairs = read_lines_as_numrange_pairs("input/day4/text.txt").expect("Failed to read input");
        let count = pairs.iter().filter(|(a, b)| a.overlaps(&b)).count();
        assert_eq!(count, 4);
    }

    #[test]
    fn test_input_first() {
        let pairs = read_lines_as_numrange_pairs("input/day4/input.txt").expect("Failed to read input");
        let count = pairs.iter().filter(|(a, b)| a.contains(&b) || b.contains(&a)).count();
        assert_eq!(count, 507);
    }
}
