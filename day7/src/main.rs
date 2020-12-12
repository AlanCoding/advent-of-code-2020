#![feature(str_split_once)]

use std::collections::HashMap;
use std::env;
use std::fs;

fn load_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Reading file failed")
}

fn parse_input(data: &str) -> HashMap<&str, Vec<(&str, u16)>> {
    let mut hm = HashMap::new();
    for line in data.lines() {
        let mut s_line = line.split(" bags contain ");
        let bag_name = s_line.next().unwrap();
        let mut vec = Vec::with_capacity(16);
        for chunk in s_line.next().unwrap().split(",") {
            let chunk = chunk.trim();
            let (chunk_left, chunk_right) = chunk.split_once(" ").unwrap();
            let count = chunk_left.parse::<u16>();
            if count.is_err() {
                continue;
            }
            let count = count.unwrap();
            let contained_bag_name = chunk_right.split(" bag").next().unwrap();
            vec.push((contained_bag_name, count));
        }
        hm.insert(bag_name, vec);
    }
    hm
}

fn check_if_bag_contains(
    hm: &HashMap<&str, Vec<(&str, u16)>>,
    bag: &str,
    contained_bag: &str,
) -> bool {
    hm.get(bag)
        .unwrap()
        .iter()
        .any(|(k, _)| _check_if_bag_contains(hm, k, contained_bag))
}

// There's probably a better way to do this without 2 separate functions? This is convenient to
// avoid counting the first bag
fn _check_if_bag_contains(
    hm: &HashMap<&str, Vec<(&str, u16)>>,
    bag: &str,
    contained_bag: &str,
) -> bool {
    if bag == contained_bag {
        return true;
    }
    hm.get(bag)
        .unwrap()
        .iter()
        .any(|(k, _)| _check_if_bag_contains(hm, k, contained_bag))
}

fn count_contained_bags(hm: &HashMap<&str, Vec<(&str, u16)>>, bag: &str) -> usize {
    hm.get(bag).unwrap().iter().fold(0, |a, (k, c)| {
        a + (*c as usize * _count_contained_bags(hm, k))
    })
}

fn _count_contained_bags(hm: &HashMap<&str, Vec<(&str, u16)>>, bag: &str) -> usize {
    hm.get(bag).unwrap().iter().fold(1, |a, (k, c)| {
        a + (*c as usize * _count_contained_bags(hm, k))
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
        .iter()
        .filter(|(k, _)| check_if_bag_contains(&data, k, "shiny gold"))
        .count();
    println!("part 1: {}", count);

    let count = count_contained_bags(&data, "shiny gold");
    println!("part 2: {}", count);
}
