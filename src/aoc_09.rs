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
    let nums: Vec<i64> = lines.iter().map(|x| x.parse().unwrap()).collect();

    for (index, num) in nums.iter().enumerate().skip(25) {
        if two_sum(&nums[(index - 25)..index], *num).is_none() {
            return Some(*num);
        }
    }
    None
}

fn second(lines: &Vec<String>, target: i64) -> Option<i64> {
    let nums: Vec<i64> = lines.iter().map(|x| x.parse().unwrap()).collect();
    let mut from: usize = 0;
    let mut to: usize = 2;
    let mut sum = nums[0] + nums[1];

    while to <= nums.len() && from < nums.len() {
        if ((to - from) < 2) || sum < target {
            if to < nums.len() {
                sum += nums[to];
            }
            to += 1;
        } else if sum > target {
            sum -= nums[from];
            from += 1;
        } else {
            break;
        }
    }
    if sum != target {
        return None;
    }
    let slice = &nums[from..to];
    Some(slice.iter().min().unwrap() + slice.iter().max().unwrap())
}

pub fn solve(lines: &Vec<String>) -> (Option<i64>, Option<i64>) {
    let first_solution = first(lines);
    let second_solution = second(lines, first_solution.unwrap());
    (first_solution, second_solution)
}
