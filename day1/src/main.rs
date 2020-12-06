use std::env;
use std::fs;

fn load_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Reading file failed")
}

fn parse_input<'a>(data: &'a str) -> impl Iterator<Item = u64> + 'a + Clone {
    data.lines()
        .map(|x| x.parse::<u64>().expect("Failed to convert string to i64"))
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
    'l1: for x in data.clone() {
        for y in data.clone() {
            if x + y == 2020 {
                let answer = x * y;
                println!("part 1: {}", answer);
                break 'l1;
            }
        }
    }

    'l2: for x in data.clone() {
        for y in data.clone() {
            for z in data.clone() {
                if x + y + z == 2020 {
                    let answer = x * y * z;
                    println!("part 2: {}", answer);
                    break 'l2;
                }
            }
        }
    }
}
