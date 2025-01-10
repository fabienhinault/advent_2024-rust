use std::fs::File;
use std::io::{self, prelude::*, BufReader};
fn main() -> io::Result<()> {
    let file = File::open("edge_cases")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|rs| rs.unwrap()).collect();
    let reports: Vec<Vec<i64>> = lines.iter().map(|l| l.split_whitespace().filter_map(|n| n.parse::<i64>().ok()).collect()).collect();
    // println!("{:?}", reports);
    // println!("{:?}", reports.iter().map(|r| (r, is_safe(r))).collect::<Vec<_>>());
    // println!("{:?}", reports.iter().filter(|r| is_safe(r) && !is_really_safe(r)).collect::<Vec<_>>());
    println!("{:?}", reports.iter().filter(|r| is_safe_379_count(r)).count());
    Ok(())
}


fn is_really_safe(report: &[i64]) -> bool {
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

// 379 too high
fn _is_safe_379_count(report: &[i64]) -> bool {
    let mut nb_bad = 0;
    let mut nb_increasing = 0;
    let mut nb_decreasing = 0;
    if report.len() < 2 {
        return false;
    }
    for i_current in 0..(report.len() - 1) {
        let current = report[i_current];
        let next = report[i_current + 1];
        if current == next {
            nb_bad += 1;
        } else if (current - next).abs() > 3 {
            nb_bad += 1;
        }else if current < next {
            nb_increasing += 1;
        } else {
            nb_decreasing += 1;
        }
    }
    if nb_increasing <= nb_decreasing {
        nb_bad += nb_increasing;
    } else {
        nb_bad += nb_decreasing;
    }
    nb_bad <= 1
}


fn _is_safe_349_retry(report: &[i64]) -> bool {
    let mut nb_bad = 0;
    let mut bad_index: Option<usize> = None;
    let mut nb_increasing = 0;
    let mut increasing_index: Option<usize> = None;
    let mut nb_decreasing = 0;
    let mut decreasing_index: Option<usize> = None;
    let mut nb_full_bad: i32;
    if report.len() < 2 {
        return false;
    }
    for i_current in 0..(report.len() - 1) {
        let current = report[i_current];
        let next = report[i_current + 1];
        if current == next {
            nb_bad += 1;
            bad_index = Some(i_current + 1);
        } else if (current - next).abs() > 3 {
            nb_bad += 1;
            bad_index = Some(i_current + 1);
        }else if current < next {
            nb_increasing += 1;
            increasing_index = Some(i_current + 1);
        } else {
            nb_decreasing += 1;
            decreasing_index = Some(i_current + 1);
        }
    }
    if nb_increasing <= nb_decreasing {
        nb_full_bad = nb_bad + nb_increasing;
        if nb_increasing > 0 {
            bad_index = increasing_index;
        }
    } else {
        nb_full_bad = nb_bad + nb_decreasing;
        if nb_decreasing > 0 {
            bad_index = decreasing_index;
        }
    }
    if nb_full_bad == 0 {
        return true;
    } else if nb_full_bad >= 2 {
        return false;
    } else {
        let mut v = report.to_vec();
        v.remove(bad_index.unwrap());
        let result = is_really_safe(&v);
        println!("{:?}", (report, v, result));
        return result;
    }
}


fn is_safe_364_retry_wo_next_or_current(report: &[i64]) -> bool {
    let mut nb_bad = 0;
    let mut bad_index: Option<usize> = None;
    let mut nb_increasing = 0;
    let mut increasing_index: Option<usize> = None;
    let mut nb_decreasing = 0;
    let mut decreasing_index: Option<usize> = None;
    let nb_full_bad: i32;
    if report.len() < 2 {
        return false;
    }
    for i_current in 0..(report.len() - 1) {
        let current = report[i_current];
        let next = report[i_current + 1];
        if current == next {
            nb_bad += 1;
            bad_index = Some(i_current + 1);
        } else if (current - next).abs() > 3 {
            nb_bad += 1;
            bad_index = Some(i_current + 1);
        }else if current < next {
            nb_increasing += 1;
            increasing_index = Some(i_current + 1);
        } else {
            nb_decreasing += 1;
            decreasing_index = Some(i_current + 1);
        }
    }
    if nb_increasing <= nb_decreasing {
        nb_full_bad = nb_bad + nb_increasing;
        if nb_increasing > 0 {
            bad_index = increasing_index;
        }
    } else {
        nb_full_bad = nb_bad + nb_decreasing;
        if nb_decreasing > 0 {
            bad_index = decreasing_index;
        }
    }
    if nb_full_bad == 0 {
        return true;
    } else if nb_full_bad >= 2 {
        return false;
    } else {
        let mut without_next = report.to_vec();
        without_next.remove(bad_index.unwrap());
        if is_really_safe(&without_next) {
            println!("{:?}", (report, without_next, true));
            return true;
        } else {
            let mut without_current = report.to_vec();
            without_current.remove(bad_index.unwrap() - 1);
            let result = is_really_safe(&without_current);
            println!("{:?}", (report, without_next, without_current, result));
            return result;
        }
    }
}


#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test_is_safe() {
        assert!(!is_safe(&    [
            58,
            59,
            60,
            59,
            60
        ],));
    }
}
