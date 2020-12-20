#[derive(Debug, Clone)]
struct BusTimes {
    start: u64,
    bus_ids: Vec<u64>,
}

fn parse_input(data: &str) -> BusTimes {
    let mut lines = data.lines();
    let start = lines.next().unwrap().parse::<u64>().unwrap();
    let bus_ids = lines
        .next()
        .unwrap()
        .split(",")
        .filter(|element| match element {
            &"x" => false,
            _ => true,
        })
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    BusTimes { start, bus_ids }
}

pub fn solve(input: &str) -> u64 {
    let data = parse_input(&input);
    let mut first_arrivals: Vec<(u64, u64)> = data
        .bus_ids
        .iter()
        .map(|&x| (x, (data.start..).filter(|y| y % x == 0).nth(0).unwrap()))
        .collect();
    first_arrivals.sort_by_key(|(_, v)| *v);
    first_arrivals[0].0 * (first_arrivals[0].1 - data.start)
}
