use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let re = Regex::new("^([0-9]*) *([0-9]*)$").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().map(|rs| rs.unwrap()).collect();
    let capturess: Vec<_> = lines.iter().map(|s| re.captures(s).unwrap()).collect();
    let first_ids: Vec<_> = capturess.iter().map(|cs| (&cs[1]).parse::<u64>().unwrap()).collect();
    let second_ids: Vec<_> = capturess.iter().map(|cs| (&cs[2]).parse::<u64>().unwrap()).collect();
    println!("{}", first_ids.iter().map(|f| *f * (second_ids.iter().filter(|s| *s == f).count() as u64)).sum::<u64>());
    Ok(())
}
