fn count_slope(lines: &[String], down: usize, right: usize) -> Option<i64> {
    let (_index, sum) = lines.iter().skip(down).step_by(down).fold(
        (0usize, 0usize),
        |(prev_index, prev_sum), line| {
            let current_index = prev_index + right;
            // 'Fast' byte indexing like you do e.g. in C++.
            // A simple alternative would be just:
            // let current_char = line.chars().nth(current_index % line.len()).unwrap();
            let bytes = line.as_bytes();
            let current_char = bytes[current_index % bytes.len()] as char;
            (
                prev_index + right,
                prev_sum + (current_char == '#') as usize,
            )
        },
    );
    Some(sum as i64)
}

fn first(lines: &Vec<String>) -> Option<i64> {
    count_slope(&lines, 1, 3)
}

fn second(lines: &Vec<String>) -> Option<i64> {
    let product: i64 = vec![
        count_slope(&lines, 1, 1),
        count_slope(&lines, 1, 3),
        count_slope(&lines, 1, 5),
        count_slope(&lines, 1, 7),
        count_slope(&lines, 2, 1),
    ]
    .iter()
    .map(|x| x.unwrap())
    .product();
    Some(product)
}

pub fn solve(lines: &Vec<String>) -> (Option<i64>, Option<i64>) {
    (first(lines), second(lines))
}
