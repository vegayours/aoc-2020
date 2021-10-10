use regex::Regex;
use std::convert::TryInto;

#[derive(Debug)]
struct Entry {
    from: i64,
    to: i64,
    letter: char,
    password: String,
}

fn parse_entry<'a>(line: &'a str) -> Option<Entry> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w*)").unwrap();
    }
    match RE
        .captures(line)
        .unwrap()
        .iter()
        .map(|x| x.unwrap().as_str())
        .collect::<Vec<&str>>()
        .as_slice()
    {
        [_, from, to, c, password] => Some(Entry {
            from: from.parse::<i64>().unwrap(),
            to: to.parse::<i64>().unwrap(),
            letter: c.chars().next().unwrap(),
            password: String::from(*password),
        }),
        _ => None,
    }
}

fn first_valid_entry(entry: &Entry) -> bool {
    let count: i64 = entry
        .password
        .chars()
        .filter(|&c| c == entry.letter)
        .count()
        .try_into()
        .unwrap();
    entry.from <= count && count <= entry.to
}

fn first(lines: &Vec<String>) -> Option<i64> {
    let entries = lines.iter().map(|x| parse_entry(&x).unwrap());
    let valid_entries = entries.filter(|x| first_valid_entry(x));
    Some(valid_entries.count() as i64)
}

fn second_valid_entry(entry: &Entry) -> bool {
    let bytes = entry.password.as_bytes();
    (bytes[(entry.from - 1) as usize] as char == entry.letter)
        ^ (bytes[(entry.to - 1) as usize] as char == entry.letter)
}

fn second(lines: &Vec<String>) -> Option<i64> {
    let entries = lines.iter().map(|x| parse_entry(&x).unwrap());
    let valid_entries = entries.filter(|x| second_valid_entry(x));
    Some(valid_entries.count() as i64)
}

pub fn solve(lines: &Vec<String>) -> (Option<i64>, Option<i64>) {
    (first(lines), second(lines))
}
