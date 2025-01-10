use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let re = Regex::new("^([0-9]*) *([0-9]*)$").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().map(|rs| rs.unwrap()).collect();
    let capturess: Vec<_> = lines.iter().map(|s| re.captures(s).unwrap()).collect();
    let mut first_ids: Vec<_> = capturess.iter().map(|cs| (&cs[1]).parse::<u64>().unwrap()).collect();
    first_ids.sort();
    let mut second_ids: Vec<_> = capturess.iter().map(|cs| (&cs[2]).parse::<u64>().unwrap()).collect();
    second_ids.sort();
    println!("{}", first_ids.iter().zip(second_ids.iter()).map(|(f, s)| {if f > s {(s, f)} else {(f, s)}}).map(|(f, s)| s - f).sum::<u64>());
    Ok(())
}
