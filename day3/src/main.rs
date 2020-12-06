use std::env;
use std::fs;

fn load_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Reading file failed")
}

fn parse_input<'a>(data: &'a str) -> impl Iterator<Item = &'a str> + 'a + Clone {
    data.lines()
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

    let tree_count = data
        .clone()
        .enumerate()
        .filter(|(i, l)| {
            let c = l.chars().cycle().nth(i * 3).unwrap();
            c == '#'
        })
        .count();
    println!("part1: {}", tree_count);

    let tc1 = data
        .clone()
        .enumerate()
        .filter(|(i, l)| {
            let c = l.chars().cycle().nth(*i).unwrap();
            c == '#'
        })
        .count();
    let tc2 = data
        .clone()
        .enumerate()
        .filter(|(i, l)| {
            let c = l.chars().cycle().nth(i * 3).unwrap();
            c == '#'
        })
        .count();
    let tc3 = data
        .clone()
        .enumerate()
        .filter(|(i, l)| {
            let c = l.chars().cycle().nth(i * 5).unwrap();
            c == '#'
        })
        .count();
    let tc4 = data
        .clone()
        .enumerate()
        .filter(|(i, l)| {
            let c = l.chars().cycle().nth(i * 7).unwrap();
            c == '#'
        })
        .count();
    let tc5 = data
        .clone()
        .step_by(2)
        .enumerate()
        .filter(|(i, l)| {
            let c = l.chars().cycle().nth(*i).unwrap();
            c == '#'
        })
        .count();
    println!("part2: {}", tc1 * tc2 * tc3 * tc4 * tc5);
}
