use std::ops::RangeInclusive;

fn first(numbers: &[i64]) -> Option<i64> {
    let (eq_1, eq_3) = numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .map(|(prev, next)| next - prev)
        .fold((0, 0), |(eq_1, eq_3), diff| match diff {
            1 => (eq_1 + 1, eq_3),
            3 => (eq_1, eq_3 + 1),
            _ => (eq_1, eq_3),
        });

    Some(eq_1 * eq_3)
}

fn second(numbers: &[i64]) -> Option<i64> {
    let mut dp = vec![0i64; numbers.len()];
    const RANGE: RangeInclusive<i64> = 1..=3;
    dp[0] = 1;
    dp[1] = 1;
    dp[2] = if RANGE.contains(&(numbers[2] - numbers[0])) {
        2
    } else {
        1
    };

    for i in 3..numbers.len() {
        dp[i] = dp[i - 1];
        if RANGE.contains(&(numbers[i] - numbers[i - 2])) {
            dp[i] += dp[i - 2];
        }
        if RANGE.contains(&(numbers[i] - numbers[i - 3])) {
            dp[i] += dp[i - 3];
        }
    }

    dp.last().copied()
}

pub fn solve(lines: &Vec<String>) -> (Option<i64>, Option<i64>) {
    let mut numbers: Vec<i64> = lines.iter().map(|line| line.parse().unwrap()).collect();
    numbers.push(0);
    numbers.sort();
    numbers.push(numbers.last().unwrap() + 3);

    (first(&numbers), second(&numbers))
}
