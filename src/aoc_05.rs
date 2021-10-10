use std::collections::HashSet;

fn calculate_num(input: &str, positive: char) -> i64 {
    let mut n: i64 = 1 << (input.len() - 1);
    let mut sum = 0;
    for c in input.chars() {
        if c == positive {
            sum += n;
        }
        n >>= 1;
    }
    sum
}

fn calculate_id(line: &str) -> i64 {
    calculate_num(&line[..7], 'B') * 8 + calculate_num(&line[7..], 'R')
}

fn first(lines: &Vec<String>) -> Option<i64> {
    lines.iter().map(|x| calculate_id(x)).max()
}

fn second(lines: &Vec<String>) -> Option<i64> {
    // Or alternatively sort and find x, x + 2 in sequence.
    // That would be O(n*log(n)) instead of current O(n) (on avg)
    let ids: HashSet<i64> = lines.iter().map(|x| calculate_id(x)).collect();
    for x in ids.iter() {
        if !ids.contains(&(x + 1)) && ids.contains(&(x + 2)) {
            return Some(x + 1);
        }
    }
    None
}

pub fn solve(lines: &Vec<String>) -> (Option<i64>, Option<i64>) {
    (first(lines), second(lines))
}
