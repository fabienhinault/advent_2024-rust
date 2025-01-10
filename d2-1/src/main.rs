use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

fn line_to_numbers_panic(line: &str, re: &Regex) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    let mut o_captures = re.captures(line);
    let captures = o_captures.unwrap();
    result.push(captures[1].parse().unwrap());
    o_captures = re.captures( &(captures[2]));
    while o_captures.is_some() {
        let captures = o_captures.unwrap();
        result.push(captures.get(1).unwrap().as_str().parse().unwrap());
        o_captures = re.captures(captures.get(2).unwrap().as_str());
    }
    result
}


fn line_to_numbers_gpt(line: &str, re: &Regex) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    let mut current_line = line;

    while let Some(captures) = re.captures(current_line) {
        // Parse the first capture group and push to the result vector
        if let Some(num_str) = captures.get(1) {
            if let Ok(num) = num_str.as_str().parse::<i64>() {
                result.push(num);
            } else {
                break;
            }

        }
        // Update `current_line` to the remainder captured group
        let o_remainder = captures.get(2);
        if o_remainder.is_none() {
            break;
        }
        current_line = o_remainder.unwrap().as_str(); 
    }

    result
}

fn is_safe(report: &[i64]) -> bool {
    if report.len() < 2 {
        return false;
    }
    let increasing = report[0] < report[1];
    for (i_current, i_next) in report.iter().zip(report.iter().skip(1)) {
        if i_current == i_next {
            return false;
        }
        if (i_current < i_next) != increasing {
            return false;
        }
        if (i_current - i_next).abs() > 3 {
            return false;
        }
    }
    true
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|rs| rs.unwrap()).collect();
    let numberss: Vec<Vec<i64>> = lines.iter().map(|l| l.split_whitespace().filter_map(|n| n.parse::<i64>().ok()).collect()).collect();
    println!("{:?}", numberss);
    println!("{:?}", numberss.iter().map(|r| (r, is_safe(r))).collect::<Vec<_>>());
    println!("{:?}", numberss.iter().filter(|r| is_safe(r)).count());
    Ok(())
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test_line_to_numbers() {
        assert_eq!(&[6, 8, 9, 11, 14, 12], line_to_numbers_panic("6 8 9 11 14 12", &Regex::new("^ *([0-9]*)(( *([0-9]*))*)$").unwrap()).as_slice());
    }

    #[test]
    fn test_line_to_numbers_gpt() {
        assert_eq!(&[6, 8, 9, 11, 14, 12], line_to_numbers_gpt("6 8 9 11 14 12", &Regex::new("^ *([0-9]*)(( *([0-9]*))*)$").unwrap()).as_slice());
    }
}
