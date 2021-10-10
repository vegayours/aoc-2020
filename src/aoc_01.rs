use std::collections::HashSet;

// Basic 2-SUM problem, O(n) time, O(n) extra space complexity for HashSet.
fn two_sum(numbers: &[i64], target: i64) -> Option<i64> {
    let mut seen_numbers = HashSet::<i64>::new();
    for x in numbers {
        if let Some(y) = seen_numbers.get(&(target - x)) {
            return Some(*x * y);
        } else {
            seen_numbers.insert(*x);
        }
    }
    None
}

fn first(lines: &Vec<String>) -> Option<i64> {
    let numbers: Vec<i64> = lines.iter().map(|x| x.parse::<i64>().unwrap()).collect();
    two_sum(&numbers, 2020)
}

// 3-SUM solution, O(n^2) time, O(1) space (if we don't consider original array).
fn second(lines: &Vec<String>) -> Option<i64> {
    let mut numbers: Vec<i64> = lines.iter().map(|x| x.parse::<i64>().unwrap()).collect();
    numbers.sort();
    let target: i64 = 2020;
    for i in 0..(numbers.len() - 2) {
        let (mut j, mut k) = (i + 1, numbers.len() - 1);
        while j < k {
            let sum = numbers[i] + numbers[j] + numbers[k];
            if sum > target {
                k -= 1;
            } else if sum < target {
                j += 1;
            } else {
                return Some(numbers[i] * numbers[j] * numbers[k]);
            }
        }
    }
    /* Reusing 2-SUM
    This will be
    for i in 0..numbers.len() {
        if let Some(result) = two_sum(&numbers[(i + 1)..], 2020 - numbers[i]) {
            return Some(result * numbers[i]);
        }
    }
    */
    None
}

pub fn solve(lines: &Vec<String>) -> (Option<i64>, Option<i64>) {
    (first(lines), second(lines))
}
