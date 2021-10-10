use std::collections::HashMap;
use std::collections::HashSet;

type Graph = HashMap<String, Vec<(usize, String)>>;

enum Direction {
    Forward,
    Backward,
}

fn get_key(input: &str, from: usize) -> String {
    input
        .split(' ')
        .skip(from)
        .take(2)
        .collect::<Vec<_>>()
        .join(" ")
}

fn add_line(line: &str, graph: &mut Graph, d: Direction) {
    let from = get_key(line, 0);
    let from_line = line.split(' ').skip(4).collect::<Vec<_>>().join(" ");
    for item in from_line.split(", ") {
        if item == "no other bags." {
            continue;
        }
        match d {
            Direction::Forward => {
                let num_str = &item[..item.find(' ').unwrap()];
                let num = num_str.parse::<usize>().unwrap();
                let to = get_key(item, 1);
                graph.entry(from.clone()).or_default().push((num, to));
            }
            Direction::Backward => {
                let to = get_key(item, 1);
                graph.entry(to).or_default().push((0, from.clone()));
            }
        };
    }
}

fn traverse_graph(from: &str, graph: &Graph, visited: &mut HashSet<String>) -> usize {
    if visited.contains(from) {
        return 0usize;
    }
    visited.insert(String::from(from));
    let mut count: usize = 1;
    for (_, to) in graph.get(from).or(Some(&Vec::new())).unwrap() {
        count += traverse_graph(to, graph, visited)
    }
    return count;
}

fn first(lines: &Vec<String>) -> Option<i64> {
    let mut graph: Graph = Graph::new();

    for line in lines {
        add_line(&line, &mut graph, Direction::Backward);
    }

    let mut visited: HashSet<String> = HashSet::new();

    let sum = traverse_graph("shiny gold", &graph, &mut visited) - 1;

    Some(sum as i64)
}

fn count_nodes(from: &str, graph: &Graph, cache: &mut HashMap<String, usize>) -> usize {
    if let Some(value) = cache.get(from) {
        return *value;
    }

    let mut count: usize = 1;

    for (num, to) in graph.get(from).or(Some(&Vec::new())).unwrap() {
        count += num * count_nodes(to, graph, cache);
    }

    cache.insert(String::from(from), count);
    count
}

fn second(lines: &Vec<String>) -> Option<i64> {
    let mut graph: Graph = Graph::new();

    for line in lines {
        add_line(&line, &mut graph, Direction::Forward);
    }

    let mut cache: HashMap<String, usize> = HashMap::new();

    let sum = count_nodes("shiny gold", &graph, &mut cache) - 1;

    Some(sum as i64)
}

pub fn solve(lines: &Vec<String>) -> (Option<i64>, Option<i64>) {
    (first(lines), second(lines))
}
