#[derive(Debug, Clone)]
struct BusTimes {
    start: u64,
    bus_ids: Vec<Option<u64>>,
}

fn lcm(ns: &[u64]) -> u64 {
    if ns.len() == 1 {
        return ns[0];
    }
    let mut lcm = ns[0];
    for i in ns.iter().skip(1) {
        lcm = num::integer::lcm(lcm, *i);
    }
    lcm
}

fn parse_input(data: &str) -> BusTimes {
    let mut lines = data.lines();
    let start = lines.next().unwrap().parse::<u64>().unwrap();
    let bus_ids = lines
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u64>().ok())
        .collect();
    BusTimes { start, bus_ids }
}

pub fn solve(input: &str) -> u64 {
    let data = parse_input(&input);
    let mut start = 0;
    let mut step = 1;
    let mut matched_values: Vec<u64> = Vec::new();
    let mut all_match = false;
    while !all_match {
        start += step;
        let mut changed = false;
        let mut curr = start;
        all_match = true;
        for i in &data.bus_ids {
            match i {
                Some(n) => {
                    if curr % n == 0 {
                        if !matched_values.contains(n) {
                            matched_values.push(*n);
                            changed = true;
                        }
                    } else {
                        all_match = false;
                        break;
                    }
                }
                _ => {}
            }
            curr += 1;
        }
        if changed {
            step = lcm(&matched_values);
        }
    }
    return start;
}
