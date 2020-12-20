use std::env;
use std::fs;

mod part1;
mod part2;

#[derive(Debug, Clone)]
pub enum Action {
    N(i64),
    E(i64),
    W(i64),
    S(i64),
    L(i64),
    R(i64),
    F(i64),
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    N,
    E,
    W,
    S,
}

fn load_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Reading file failed")
}

fn parse_input(data: &str) -> impl Iterator<Item = Action> + Clone + '_ {
    data.lines().map(|l| match l.split_at(1) {
        ("N", x) => Action::N(x.parse::<i64>().unwrap()),
        ("E", x) => Action::E(x.parse::<i64>().unwrap()),
        ("W", x) => Action::W(x.parse::<i64>().unwrap()),
        ("S", x) => Action::S(x.parse::<i64>().unwrap()),
        ("L", x) => Action::L(x.parse::<i64>().unwrap()),
        ("R", x) => Action::R(x.parse::<i64>().unwrap()),
        ("F", x) => Action::F(x.parse::<i64>().unwrap()),
        _ => panic!("BAD INPUT"),
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() >= 2 {
        args[1].clone()
    } else {
        "input".to_string()
    };
    let input = load_input(&filename);
    let data = parse_input(&input);
    println!("Part 1: {}", part1::solve(data.clone()));
    println!("Part 2: {}", part2::solve(data.clone()));
}
