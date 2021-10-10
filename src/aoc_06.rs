use std::collections::{HashMap, HashSet};
use std::iter;

fn first(lines: &Vec<String>) -> Option<i64> {
    let empty_str = String::from("");
    let aux_lines = lines.iter().chain(iter::once(&empty_str));

    let mut char_set: HashSet<char> = HashSet::new();
    let mut sum: usize = 0;

    for line in aux_lines {
        if !line.is_empty() {
            char_set.extend(line.chars());
        } else {
            sum += char_set.len();
            char_set.clear();
        }
    }
    Some(sum as i64)
}

fn second(lines: &Vec<String>) -> Option<i64> {
    let empty_str = String::from("");
    let aux_lines = lines.iter().chain(iter::once(&empty_str));

    let mut char_set: HashMap<char, usize> = HashMap::new();
    let mut group: usize = 0;
    let mut sum: usize = 0;

    for line in aux_lines {
        if !line.is_empty() {
            for c in line.chars() {
                *char_set.entry(c).or_default() += 1;
            }
            group += 1;
        } else {
            for (_c, count) in char_set.iter() {
                if *count == group {
                    sum += 1;
                }
            }
            char_set.clear();
            group = 0;
        }
    }
    Some(sum as i64)
}

pub fn solve(lines: &Vec<String>) -> (Option<i64>, Option<i64>) {
    (first(lines), second(lines))
}
