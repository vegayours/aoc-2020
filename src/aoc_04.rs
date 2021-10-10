use std::collections::HashMap;
use std::iter;
use std::ops::Fn;

fn add_line(line: &str, keys: &mut HashMap<String, String>) {
    for item in line.split(' ') {
        let parts: Vec<&str> = item.split(':').collect();
        keys.insert(String::from(parts[0]), String::from(parts[1]));
    }
}

fn validate<F>(lines: &Vec<String>, is_valid: F) -> Option<i64>
where
    F: Fn(&HashMap<String, String>) -> bool,
{
    let mut valid_count = 0i64;
    let mut current_keys = HashMap::<String, String>::new();

    let once_string = String::from("");

    let aux_lines = lines.iter().chain(iter::once(&once_string));

    for line in aux_lines {
        if !line.is_empty() {
            add_line(&line, &mut current_keys)
        } else {
            if is_valid(&current_keys) {
                valid_count += 1;
            }
            current_keys.clear();
        }
    }
    Some(valid_count)
}

fn first_is_valid(keys: &HashMap<String, String>) -> bool {
    const REQUIRED_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    REQUIRED_FIELDS.iter().all(|&key| keys.contains_key(key))
}

fn first(lines: &Vec<String>) -> Option<i64> {
    validate(lines, first_is_valid)
}

fn second_is_valid(keys: &HashMap<String, String>) -> bool {
    let validators: &[(&str, fn(&str) -> bool)] = &[
        ("byr", |byr| {
            byr.len() == 4
                && match byr.parse::<usize>() {
                    Ok(val) if val >= 1920 && val <= 2002 => true,
                    _ => false,
                }
        }),
        ("iyr", |iyr| {
            iyr.len() == 4
                && match iyr.parse::<usize>() {
                    Ok(val) if val >= 2010 && val <= 2020 => true,
                    _ => false,
                }
        }),
        ("eyr", |eyr| {
            eyr.len() == 4
                && match eyr.parse::<usize>() {
                    Ok(val) if val >= 2020 && val <= 2030 => true,
                    _ => false,
                }
        }),
        ("hgt", |hgt| {
            if hgt.len() < 4 {
                return false;
            }
            let num = &hgt[0..(hgt.len() - 2)];
            let suffix = &hgt[(hgt.len() - 2)..];
            match (suffix, num.parse::<usize>()) {
                ("in", Ok(val)) if val >= 59 && val <= 76 => true,
                ("cm", Ok(val)) if val >= 150 && val <= 193 => true,
                _ => false,
            }
        }),
        ("hcl", |hcl| {
            if hcl.len() != 7 {
                return false;
            }
            &hcl[0..1] == "#"
                && hcl[1..]
                    .chars()
                    .all(|c| c.is_ascii_digit() || (c >= 'a' && c <= 'f'))
        }),
        ("ecl", |ecl| {
            const VALUES: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            VALUES.iter().any(|value| *value == ecl)
        }),
        ("pid", |pid| {
            pid.len() == 9 && pid.chars().all(|c| c.is_ascii_digit())
        }),
    ];
    validators
        .into_iter()
        .all(|(key, validator)| match keys.get(*key) {
            Some(value) => validator(value),
            None => false,
        })
}

fn second(lines: &Vec<String>) -> Option<i64> {
    validate(lines, second_is_valid)
}

pub fn solve(lines: &Vec<String>) -> (Option<i64>, Option<i64>) {
    (first(lines), second(lines))
}
