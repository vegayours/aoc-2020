use std::cmp::min;

fn first(lines: &Vec<String>) -> Option<i64> {
    let num: i64 = lines.get(0).unwrap().parse().unwrap();
    let ids: Vec<i64> = lines
        .get(1)
        .unwrap()
        .split(',')
        .filter(|x| *x != "x")
        .map(|x| x.parse().unwrap())
        .collect();
    let (wait, id) = ids.iter().fold((i64::MAX, 0i64), |acc, id| {
        let current = (id - num % id) % id;
        min(acc, (current, *id))
    });
    Some(wait * id)
}

fn second(lines: &Vec<String>) -> Option<i64> {
    let ids = lines
        .get(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_index, x)| *x != "x")
        .map(|(index, x)| (index as i64, x.parse::<i64>().unwrap()))
        .collect::<Vec<_>>();
    let mut inc: i64 = 1;
    let mut start: i64 = 0;
    let mut iterations: usize = 0;
    for (index, id) in ids {
        while (start + index) % id != 0 {
            start += inc;
            iterations += 1;
        }
        inc *= id;
    }
    println!("Number of iterations: {}", iterations);
    Some(start)
}

pub fn solve(lines: &Vec<String>) -> (Option<i64>, Option<i64>) {
    (first(lines), second(lines))
}
