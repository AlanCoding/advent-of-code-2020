use std::env;
use std::fs;

fn load_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Reading file failed")
}

fn parse_input(data: &str) -> impl Iterator<Item = &str> + Clone {
    data.split("\n\n")
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
    let total = data
        .clone()
        .map(|b: &str| ('a'..='z').filter(|&c| b.contains(c)).count())
        .fold(0, |a, v| a + v);
    println!("part 1: {}", total);

    let total = data
        .map(|b: &str| {
            let mut b_s = b.split_whitespace();
            let first = b_s.next().unwrap();
            first
                .chars()
                .filter(|&c| b_s.clone().all(|s| s.contains(c)))
                .count()
        })
        .fold(0, |a, v| a + v);
    println!("part 2: {}", total);
}
