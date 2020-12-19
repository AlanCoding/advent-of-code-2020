use std::env;
use std::fs;

mod part1;
mod part2;

pub type SeatMap = Vec<Vec<Option<Seat>>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Seat {
    Taken,
    Empty,
}

fn load_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Reading file failed")
}

fn parse_input(data: &str) -> SeatMap {
    data.lines()
        .map(|l| {
            l.chars()
                .map(|c| if c == 'L' { Some(Seat::Empty) } else { None })
                .collect()
        })
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() >= 2 {
        args[1].clone()
    } else {
        "input".to_string()
    };
    let input = load_input(&filename);
    let seat_data = parse_input(&input);
    println!("Part 1: {}", part1::solve_p1(&seat_data));
    println!("Part 2: {}", part2::solve_p2(&seat_data));
}
