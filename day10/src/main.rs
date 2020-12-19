use rayon::prelude::*;
use std::env;
use std::fs;
use std::iter;

fn load_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Reading file failed")
}

fn parse_input(data: &str) -> Vec<u64> {
    data.lines().map(|x| x.parse::<u64>().unwrap()).collect()
}

fn count_paths<'a>(last_element: u64, mut iter: impl Iterator<Item = &'a u64> + Clone) -> u64 {
    let mut count = 0;
    let mut diff = 0;
    while diff <= 3 {
        let next_element_option = iter.next();
        match next_element_option {
            Some(n) => {
                diff = n - last_element;
                if diff <= 3 {
                    count += count_paths(*n, iter.clone());
                }
            }
            None => return 1,
        }
    }
    return count;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() >= 2 {
        args[1].clone()
    } else {
        "input".to_string()
    };
    let input = load_input(&filename);
    let mut p = parse_input(&input);
    p.sort();
    let mut one_diff = 0;
    let mut three_diff = 0;
    let max_val = p[p.len() - 1] + 3;
    let mut p_iter = [0].iter().chain(p.iter()).chain(iter::once(&max_val));
    for (x, y) in p_iter.clone().zip(p_iter.clone().skip(1)) {
        if y - x == 1 {
            one_diff += 1;
        } else if y - x == 3 {
            three_diff += 1;
        }
    }
    println!("Part 1: {}", one_diff * three_diff);

    println!("Part 2: {}", count_paths(*p_iter.next().unwrap(), p_iter));
}
