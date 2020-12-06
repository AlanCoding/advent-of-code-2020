use std::env;
use std::fs;

fn load_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Reading file failed")
}

fn parse_input(data: &str) -> impl Iterator<Item = &str> + Clone {
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
    let taken_seats: Vec<u64> = data
        .clone()
        .map(|l| {
            let mut row_min = 0;
            let mut row_max = 127;
            let mut col_min = 0;
            let mut col_max = 7;
            for c in l.chars() {
                match c {
                    'F' => row_max = (row_min + row_max) / 2,
                    'B' => row_min = (row_min + (row_max + 1)) / 2,
                    'L' => col_max = (col_min + col_max) / 2,
                    'R' => col_min = (col_min + (col_max + 1)) / 2,
                    _ => unreachable!(),
                }
            }
            assert_eq!(row_max, row_min);
            assert_eq!(col_max, col_min);
            row_max * 8 + col_max
        })
        .collect();
    println!("part 1: {}", taken_seats.iter().max().unwrap());

    let free_seats: Vec<u64> = (0..=1023_u64)
        .filter(|x| !taken_seats.contains(x))
        .collect();
    for (i, s) in free_seats.iter().enumerate().skip(1) {
        if free_seats[i - 1] != s - 1 && free_seats[i + 1] != s + 1 {
            println!("part 2: {:#?}", s);
            break;
        }
    }
}
