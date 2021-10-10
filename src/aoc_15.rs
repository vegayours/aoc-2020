const TOTAL_TURNS_FIRST: usize = 2020;
const TOTAL_TURNS_SECOND: usize = 30000000;

fn solve_for_target(lines: &Vec<String>, target: usize) -> Option<i64> {
    let numbers: Vec<i64> = lines[0].split(',').map(|x| x.parse().unwrap()).collect();

    let mut last_pos = vec![-1 as i64; target];
    let mut last_num = numbers[0];

    for (index, num) in numbers.iter().enumerate() {
        last_pos[last_num as usize] = index as i64;
        last_num = *num;
    }

    for index in numbers.len()..target {
        let last = last_pos[last_num as usize];
        last_pos[last_num as usize] = index as i64;
        last_num = if last == -1 { 0 } else { index as i64 - last };
    }

    Some(last_num)
}

fn first(lines: &Vec<String>) -> Option<i64> {
    solve_for_target(lines, TOTAL_TURNS_FIRST)
}

fn second(lines: &Vec<String>) -> Option<i64> {
    solve_for_target(lines, TOTAL_TURNS_SECOND)
}

pub fn solve(lines: &Vec<String>) -> (Option<i64>, Option<i64>) {
    (first(lines), second(lines))
}
