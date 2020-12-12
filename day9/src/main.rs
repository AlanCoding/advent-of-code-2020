use std::env;
use std::fs;

fn load_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Reading file failed")
}

fn parse_input(data: &str) -> Vec<u64> {
    data.lines().map(|x| x.parse::<u64>().unwrap()).collect()
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
    let preamble_size = 25;
    let (i, _) = p
        .iter()
        .enumerate()
        .skip(preamble_size)
        .take_while(|(i, v)| {
            p[*i - (preamble_size)..*i].iter().any(|x| {
                p[*i - (preamble_size)..*i]
                    .iter()
                    .any(|y| x != y && x + y == **v)
            })
        })
        .last()
        .unwrap();
    let p1 = p[i + 1];
    println!("part 1: {}", p1);

    for (mut i, v) in p.iter().enumerate() {
        let mut smallest = *v;
        let mut largest = *v;
        let mut sum = *v;
        while sum < p1 {
            i += 1;
            sum += p[i];
            if p[i] > largest {
                largest = p[i];
            } else if p[i] < smallest {
                smallest = p[i];
            }
        }
        if sum == p1 {
            println!("part 2: {}", smallest + largest);
            return;
        }
    }
}
