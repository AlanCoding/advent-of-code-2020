#![feature(str_split_once)]

use std::env;
use std::fs;

mod part1;
mod part2;

fn load_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Reading file failed")
}

fn parse_input(data: &str) -> impl Iterator<Item = (&str, &str)> + Clone {
    data.lines().map(|l| {
        let mut s_l = l.split(" = ");
        (s_l.next().unwrap(), s_l.next().unwrap())
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
    let p = parse_input(&input);
    println!("Part 1: {}", part1::solve(p.clone()));
    println!("Part 2: {}", part2::solve(p));
}
