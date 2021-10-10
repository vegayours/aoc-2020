use std::collections::HashMap;

#[derive(Clone, Copy)]
struct Mask {
    and: u64,
    or: u64,
}

enum Operation {
    UpdateMask(Mask),
    Write { address: u64, value: u64 },
}

fn parse_ops(lines: &Vec<String>) -> Vec<Operation> {
    lines
        .iter()
        .map(|line| line.split(" = ").collect::<Vec<_>>())
        .map(|parts| match &parts[..] {
            ["mask", mask_str] => {
                let mut mask = Mask {
                    and: u64::MAX,
                    or: 0u64,
                };
                for (shift, c) in mask_str.chars().rev().enumerate() {
                    if c == '0' {
                        mask.and = mask.and & (!(1 << shift));
                    } else if c == '1' {
                        mask.or = mask.or | (1 << shift);
                    }
                }
                Operation::UpdateMask(mask)
            }
            [mem, value] if mem.starts_with("mem[") => Operation::Write {
                address: mem
                    .strip_prefix("mem[")
                    .unwrap()
                    .strip_suffix(']')
                    .unwrap()
                    .parse()
                    .unwrap(),
                value: value.parse().unwrap(),
            },
            _ => panic!("Unsupported command: {}", parts[0]),
        })
        .collect()
}

fn first(lines: &Vec<String>) -> Option<i64> {
    let ops = parse_ops(lines);
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask = Mask {
        and: u64::MAX,
        or: 0u64,
    };
    for op in &ops {
        match op {
            Operation::UpdateMask(new_mask) => mask = *new_mask,
            Operation::Write { address, value } => {
                let new_value = (*value | mask.or) & mask.and;
                memory.insert(*address, new_value);
            }
        }
    }
    Some(memory.values().sum::<u64>() as i64)
}

fn second(lines: &Vec<String>) -> Option<i64> {
    None
}

pub fn solve(lines: &Vec<String>) -> (Option<i64>, Option<i64>) {
    (first(lines), second(lines))
}
