enum Rotation {
    Left,
    Right,
}
struct State {
    x: i64,
    y: i64,
    w_x: i64,
    w_y: i64,
}
struct Command {
    c: char,
    arg: i64,
}

fn parse_command(line: &str) -> Command {
    let mut chars = line.chars();
    Command {
        c: chars.next().unwrap(),
        arg: chars.as_str().parse().unwrap(),
    }
}

fn rotate(state: State, angle: i64, rotation: Rotation) -> State {
    let rotate_right_times = match rotation {
        Rotation::Left => 4 - ((angle / 90) % 4),
        Rotation::Right => ((angle / 90) % 4),
    };
    let (w_x, w_y) = (0..rotate_right_times).fold((state.w_x, state.w_y), |(x, y), _| (y, -x));
    State {
        w_x: w_x,
        w_y: w_y,
        ..state
    }
}

fn apply_command(state: State, command: Command) -> State {
    match command.c {
        'N' => State {
            y: state.y + command.arg,
            ..state
        },
        'S' => State {
            y: state.y - command.arg,
            ..state
        },
        'E' => State {
            x: state.x + command.arg,
            ..state
        },
        'W' => State {
            x: state.x - command.arg,
            ..state
        },
        'L' => rotate(state, command.arg, Rotation::Left),
        'R' => rotate(state, command.arg, Rotation::Right),
        'F' => State {
            x: state.x + command.arg * state.w_x,
            y: state.y + command.arg * state.w_y,
            ..state
        },
        _ => {
            panic!("Unsupported command: {}", command.c)
        }
    }
}

fn first(lines: &Vec<String>) -> Option<i64> {
    let state: State = lines.iter().map(|x| parse_command(x)).fold(
        State {
            x: 0,
            y: 0,
            w_x: 1,
            w_y: 0,
        },
        apply_command,
    );
    Some(state.x.abs() + state.y.abs())
}

fn apply_waypoint_command(state: State, command: Command) -> State {
    match command.c {
        'N' => State {
            w_y: state.w_y + command.arg,
            ..state
        },
        'S' => State {
            w_y: state.w_y - command.arg,
            ..state
        },
        'E' => State {
            w_x: state.w_x + command.arg,
            ..state
        },
        'W' => State {
            w_x: state.w_x - command.arg,
            ..state
        },
        'L' => rotate(state, command.arg, Rotation::Left),
        'R' => rotate(state, command.arg, Rotation::Right),
        'F' => State {
            x: state.x + command.arg * state.w_x,
            y: state.y + command.arg * state.w_y,
            ..state
        },
        _ => {
            panic!("Unsupported command: {}", command.c)
        }
    }
}

fn second(lines: &Vec<String>) -> Option<i64> {
    let state = lines.iter().map(|x| parse_command(x)).fold(
        State {
            x: 0,
            y: 0,
            w_x: 10,
            w_y: 1,
        },
        apply_waypoint_command,
    );
    Some(state.x.abs() + state.y.abs())
}

pub fn solve(lines: &Vec<String>) -> (Option<i64>, Option<i64>) {
    (first(lines), second(lines))
}
