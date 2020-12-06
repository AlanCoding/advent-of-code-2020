use std::env;
use std::fs;

#[derive(Debug)]
struct PasswordPolicy {
    min: u64,
    max: u64,
    c: char,
    password: String,
}

fn load_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Reading file failed")
}

fn parse_input<'a>(data: &'a str) -> impl Iterator<Item = PasswordPolicy> + 'a + Clone {
    data.lines().map(|line| {
        let split_line: Vec<&str> = line.split(" ").collect();
        let password = split_line[2].to_string();
        let c = split_line[1].chars().next().unwrap();
        let mut bounds = split_line[0].split("-");
        let min = bounds.next().unwrap().parse::<u64>().unwrap();
        let max = bounds.next().unwrap().parse::<u64>().unwrap();

        PasswordPolicy {
            min,
            max,
            c,
            password,
        }
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

    let count = data
        .clone()
        .filter(|x| {
            let r = x.password.chars().filter(|&c| c == x.c).count() as u64;
            r >= x.min && r <= x.max
        })
        .count();
    println!("part 1: {}", count);

    let count = data
        .filter(|x| {
            let c1 = x.password.chars().nth(x.min as usize - 1);
            let m1 = match c1 {
                Some(ch) => ch == x.c,
                None => false,
            };
            let c2 = x.password.chars().nth(x.max as usize - 1);
            let m2 = match c2 {
                Some(ch) => ch == x.c,
                None => false,
            };
            (m1 || m2) && !(m1 && m2)
        })
        .count();
    println!("part 2: {}", count);
}
