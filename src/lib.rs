use std::{fs, io};

use regex::Regex;

use crate::numrange::InclusiveRange;

pub mod numrange;
pub mod access;

pub fn read_lines_as_str(path: &str) -> io::Result<Vec<String>> {
    let text = fs::read_to_string(path)?;
    let lines = text.trim().split("\n").map(|s| s.to_string()).collect();
    return Ok(lines);
}

pub fn read_lines_as_ints(path: &str) -> io::Result<Vec<i32>> {
    let lines = read_lines_as_str(path)?;
    let ints: Vec<i32> = lines.iter().map(|s| s.parse::<i32>().expect("Failed to parse integer")).collect();
    return Ok(ints);
}

pub fn read_lines_as_string_pairs(path: &str) -> io::Result<Vec<(String, String)>> {
    let lines = read_lines_as_str(path)?;
    let mut pairs: Vec<(String, String)> = vec![];
    for line in lines {
        let pair = line.split_once(" ").ok_or(io::ErrorKind::InvalidInput)?;
        pairs.push((String::from(pair.0), String::from(pair.1)));
    }
    Ok(pairs)
}

pub fn read_lines_as_string_int_pairs(path: &str) -> io::Result<Vec<(String, i32)>> {
    let lines = read_lines_as_str(path)?;
    let mut pairs: Vec<(String, i32)> = vec![];
    for line in lines {
        let pair = line.split_once(" ").ok_or(io::ErrorKind::InvalidInput)?;
        pairs.push((String::from(pair.0), pair.1.parse().map_err(|_| io::ErrorKind::InvalidInput)?));
    }
    Ok(pairs)
}

pub fn read_lines_as_numrange_pairs(path: &str) -> io::Result<Vec<(InclusiveRange, InclusiveRange)>> {
    let lines = read_lines_as_str(path)?;
    let mut pairs: Vec<(InclusiveRange, InclusiveRange)> = vec![];
    for line in lines {
        let pair = line.split_once(",").ok_or(io::ErrorKind::InvalidInput)?;
        pairs.push((
            InclusiveRange::try_from(pair.0).map_err(|_| io::ErrorKind::InvalidInput)?,
            InclusiveRange::try_from(pair.1).map_err(|_| io::ErrorKind::InvalidInput)?
        ));
    }
    Ok(pairs)
}

pub fn read_line_groups_as_ints(path: &str) -> io::Result<Vec<Vec<i32>>> {
    let text = fs::read_to_string(path)?;
    let groups: Vec<&str> = text.trim().split("\n\n").collect();

    // let mut intgroups: Vec<Vec<i32>> = vec![];
    // for paragraph in groups {
    //     let ints = paragraph.split("\n").map(|s| s.parse::<i32>().expect("Failed to parse integer")).collect();
    //     intgroups.push(ints);
    // }
    let intgroups = groups.iter().map(
        |&g| g.split("\n").map(|s| s.parse::<i32>().expect("integer")).collect()
    ).collect();
    Ok(intgroups)
}

pub fn read_as_ndarray_char(path: &str) -> io::Result<ndarray::Array2<char>> {
    let text = fs::read_to_string(path)?;
    let text = text.trim();
    let num_rows = text.matches("\n").count() + 1;  //  split("\n\n").collect();
    let (first_row, _rest) = text.split_once("\n").ok_or(io::ErrorKind::InvalidData)?;
    let num_cols = first_row.len();

    let mut arr = ndarray::Array2::<char>::default((num_rows, num_cols));
    for (y, row) in text.split("\n").enumerate() {
        for (x, char) in row.chars().enumerate() {
            arr[[y, x]] = char;
        }
    }
    Ok(arr)
}

pub fn read_as_ndarray_num(path: &str) -> io::Result<ndarray::Array2<u8>> {
    Ok(read_as_ndarray_char(path)?.mapv(|c| u8::try_from(c).unwrap() - b'0'))
}

pub fn captured_strs<'r, 't>(re: &'r Regex, text: &'t str) -> Option<Vec<&'t str>> {
    let caps = re.captures(text)?;
    let len = caps.len();
    let mut strs: Vec<&str> = Vec::new();
    for i in 1..len {
        strs.push(caps.get(i).unwrap().as_str());
    }
    Some(strs)
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Write;

    use tempfile;

    use crate::*;

    fn create_input(input: &str) -> io::Result<tempfile::NamedTempFile> {
        let mut file: tempfile::NamedTempFile = tempfile::NamedTempFile::new()?;
        file.write(input.as_bytes())?;
        return Ok(file);
    }

    #[test]
    fn test_read_line_groups_as_ints() -> io::Result<()> {
        let file = create_input("1\n2\n\n3\n4\n5\n")?;
        let path = file.into_temp_path();
        let result = read_line_groups_as_ints(path.to_str().expect("path to be UTF-8")).expect("parsing to succeed");
        let expect: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4, 5]];
        assert_eq!(result, expect);
        return Ok(());
    }

    #[test]
    fn test_read_lines_as_str_pairs() -> io::Result<()> {
        let file = create_input("A X\nB Y\n")?;
        let path = file.into_temp_path();
        let result = read_lines_as_string_pairs(path.to_str().expect("path to be UTF-8")).expect("parsing to succeed");
        let expect: Vec<(String, String)> = vec![(String::from("A"), String::from("X")), (String::from("B"), String::from("Y"))];
        assert_eq!(result, expect);
        return Ok(());
    }
}
