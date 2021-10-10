use std::collections::HashSet;

enum Op {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

fn parse_op(line: &str) -> Op {
    match line.split(' ').collect::<Vec<_>>().as_slice() {
        ["nop", nop_value] => Op::Nop(nop_value.parse::<i64>().unwrap()),
        ["acc", acc_value] => Op::Acc(acc_value.parse::<i64>().unwrap()),
        ["jmp", jmp_value] => Op::Jmp(jmp_value.parse::<i64>().unwrap()),
        _ => panic!("Unxpected op: {}", line),
    }
}

fn detect_loop(ops: &[Op]) -> i64 {
    let mut pc: usize = 0;
    let mut acc: i64 = 0;
    let mut seen_ops: HashSet<usize> = HashSet::new();
    while pc < ops.len() && !seen_ops.contains(&pc) {
        seen_ops.insert(pc);
        match ops[pc] {
            Op::Nop(_) => pc += 1,
            Op::Acc(value) => {
                acc += value;
                pc += 1
            }
            Op::Jmp(jump) => {
                if jump >= 0 {
                    pc = pc + jump as usize
                } else {
                    pc = pc - (jump.wrapping_abs()) as usize
                }
            }
        }
    }
    if pc >= ops.len() {
        panic!("No loop!");
    }
    acc
}

fn first(lines: &Vec<String>) -> Option<i64> {
    let ops: Vec<Op> = lines.iter().map(|x| parse_op(&x)).collect();

    let result = detect_loop(&ops);

    Some(result as i64)
}

fn second(lines: &Vec<String>) -> Option<i64> {
    let ops: Vec<Op> = lines.iter().map(|x| parse_op(&x)).collect();

    // 'Color' each op which belongs to given pass (either looped or reaching the end).
    // At the end of each color pass determine if ops with a given color will reach the
    // end.
    let mut colors: Vec<i64> = vec![-1; ops.len()];
    let mut terminates: Vec<bool> = Vec::new();

    for start in 0..ops.len() {
        if colors[start] == -1 {
            let color = terminates.len() as i64;
            terminates.push(false);

            let mut pc = start;
            while pc < ops.len() && colors[pc] == -1 {
                colors[pc] = color;
                match ops[pc] {
                    Op::Jmp(jump) => {
                        if jump >= 0 {
                            pc = pc + jump as usize
                        } else {
                            pc = pc - (jump.wrapping_abs()) as usize
                        }
                    }
                    _ => pc += 1,
                }
            }
            if pc > ops.len() {
                panic!("Some op jumps outside of the array");
            }

            terminates[color as usize] = pc == ops.len() || terminates[colors[pc] as usize]
        }
    }

    // After coloring we know from which ops the end is reachable.
    // Start from beginning and on each 'jmp' or 'nop' check if the change will lead to
    // the next op be in 'end reachable' color.
    // Once we found such change - execute it and continue execution as normal.
    let mut pc = 0;
    let mut visited = vec![false; ops.len()];

    let mut acc: i64 = 0;
    let mut changed = false;
    while pc < ops.len() && !visited[pc] {
        visited[pc] = true;
        match ops[pc] {
            Op::Jmp(_) if !changed && terminates[colors[pc + 1] as usize] => {
                changed = true;
                pc += 1;
            }
            Op::Nop(jmp)
                if !changed
                    && (pc as i64 + jmp) >= 0
                    && (pc as i64 + jmp) < ops.len() as i64
                    && terminates[colors[(pc as i64 + jmp) as usize] as usize] =>
            {
                changed = true;
                pc += jmp as usize;
            }
            Op::Nop(_) => pc += 1,
            Op::Acc(value) => {
                acc += value;
                pc += 1;
            }
            Op::Jmp(jump) => {
                if jump >= 0 {
                    pc = pc + jump as usize
                } else {
                    pc = pc - (jump.wrapping_abs()) as usize
                };
            }
        }
    }

    if pc == ops.len() {
        Some(acc)
    } else {
        panic!("Can't solve - looped anyway");
    }
}

pub fn solve(lines: &Vec<String>) -> (Option<i64>, Option<i64>) {
    (first(lines), second(lines))
}
