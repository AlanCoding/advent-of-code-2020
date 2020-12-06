use std::env;
use std::fs;

fn load_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Reading file failed")
}

fn parse_input<'a>(data: &'a str) -> impl Iterator<Item = &'a str> + 'a + Clone {
    data.lines()
}

fn count_trees<'a>(data: impl Iterator<Item = &'a str>, v: usize, h: usize) -> u64 {
    data.step_by(v)
        .enumerate()
        .filter(|(i, l)| {
            let c = l.chars().cycle().nth(i * h).unwrap();
            c == '#'
        })
        .count() as u64
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

    let tree_count = count_trees(data.clone(), 1, 3);
    println!("part1: {}", tree_count);

    let tc1 = count_trees(data.clone(), 1, 1);
    let tc2 = tree_count;
    let tc3 = count_trees(data.clone(), 1, 5);
    let tc4 = count_trees(data.clone(), 1, 7);
    let tc5 = count_trees(data.clone(), 2, 1);
    println!("part2: {}", tc1 * tc2 * tc3 * tc4 * tc5);
}
