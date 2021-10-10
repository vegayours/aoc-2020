use std::{collections::HashMap, ops::RangeInclusive};

struct Rule {
    name: String,
    first: RangeInclusive<i64>,
    second: RangeInclusive<i64>,
}

impl Rule {
    fn matches(&self, num: &i64) -> bool {
        self.first.contains(num) || self.second.contains(num)
    }
    fn parse(line: &str) -> Rule {
        let mut split = line.split(": ");
        let name = String::from(split.next().unwrap());
        let mut ranges = split.next().unwrap().split(" or ").map(|parts| {
            let mut bounds = parts.split('-').map(|n| n.parse::<i64>().unwrap());
            RangeInclusive::new(bounds.next().unwrap(), bounds.next().unwrap())
        });
        Rule {
            name,
            first: ranges.next().unwrap(),
            second: ranges.next().unwrap(),
        }
    }
}

struct Ticket {
    values: Vec<i64>,
}

impl Ticket {
    fn parse(s: &str) -> Ticket {
        Ticket {
            values: s.split(',').map(|x| x.parse().unwrap()).collect(),
        }
    }
    fn invalid_fields(&self, rules: &[Rule]) -> Vec<i64> {
        self.values
            .iter()
            .filter(|num| !rules.iter().any(|r| r.matches(num)))
            .cloned()
            .collect()
    }
}

struct Input {
    rules: Vec<Rule>,
    ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl Input {
    fn parse(lines: &Vec<String>) -> Input {
        let mut rules: Vec<Rule> = Vec::new();
        let mut iter = lines.iter();
        loop {
            match iter.next() {
                Some(s) if s.is_empty() => {
                    break;
                }
                Some(s) => {
                    rules.push(Rule::parse(s));
                }
                None => panic!("Unexpected end"),
            }
        }
        iter.next();
        let ticket = Ticket::parse(iter.next().unwrap());
        let nearby_tickets: Vec<Ticket> = iter.skip(2).map(|s| Ticket::parse(s)).collect();
        Input {
            rules,
            ticket,
            nearby_tickets,
        }
    }
}

fn first(input: &Input) -> Option<i64> {
    let error = input
        .nearby_tickets
        .iter()
        .map(|ticket| ticket.invalid_fields(&input.rules))
        .flatten()
        .sum();
    Some(error)
}

fn find_assignment(
    candidates: &Vec<(usize, Vec<usize>)>,
    consumed: &mut HashMap<usize, usize>,
    index: usize,
) -> bool {
    if index == candidates.len() {
        return true;
    }
    let (rule_index, field_indices) = &candidates[index];

    for field_index in field_indices {
        if consumed.contains_key(field_index) {
            continue;
        }
        consumed.insert(*field_index, *rule_index);
        if find_assignment(candidates, consumed, index + 1) {
            return true;
        }
        consumed.remove(field_index);
    }

    false
}

fn second(input: &Input) -> Option<i64> {
    let valid_tickets = input
        .nearby_tickets
        .iter()
        .filter(|&ticket| ticket.invalid_fields(&input.rules).is_empty())
        .collect::<Vec<_>>();
    let mut validity: Vec<Vec<bool>> = vec![vec![true; input.rules.len()]; input.rules.len()];
    for ticket in valid_tickets {
        for (rule_index, rule) in input.rules.iter().enumerate() {
            for (ticket_index, ticket_num) in ticket.values.iter().enumerate() {
                validity[rule_index][ticket_index] &= rule.matches(ticket_num);
            }
        }
    }
    let mut candidates = (0..validity.len())
        .into_iter()
        .map(|rule_index| {
            let valid_columns: Vec<usize> = validity[rule_index]
                .iter()
                .enumerate()
                .filter(|x| *x.1)
                .map(|x| x.0)
                .collect();
            (rule_index, valid_columns)
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|a, b| (a.1.len(), a.0).partial_cmp(&(b.1.len(), b.0)).unwrap());

    let mut consumed: HashMap<usize, usize> = HashMap::new();

    if !find_assignment(&candidates, &mut consumed, 0) {
        panic!("No assignment was found");
    }

    let value = consumed
        .iter()
        .filter(|&(_, rule_index)| input.rules[*rule_index].name.starts_with("departure"))
        .into_iter()
        .map(|(field_index, _)| input.ticket.values[*field_index])
        .product();
    Some(value)
}

pub fn solve(lines: &Vec<String>) -> (Option<i64>, Option<i64>) {
    let input = Input::parse(lines);
    (first(&input), second(&input))
}
