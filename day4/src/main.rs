use std::env;
use std::fs;

#[derive(Default)]
struct FieldTracker {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
}

fn load_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Reading file failed")
}

fn parse_input(
    data: &str,
) -> impl Iterator<Item = impl Iterator<Item = (&str, &str)> + Clone> + Clone {
    data.split("\n\n").map(|block| {
        block.split_whitespace().map(|w| {
            let mut s_string = w.split(':');
            (s_string.next().unwrap(), s_string.next().unwrap())
        })
    })
}

fn validate_byr(byr: &str) -> bool {
    let p_byr = byr.parse::<u64>();
    match p_byr {
        Ok(byr_u64) => byr_u64 >= 1920 && byr_u64 <= 2002,
        Err(_) => false,
    }
}

fn validate_iyr(iyr: &str) -> bool {
    let p_iyr = iyr.parse::<u64>();
    match p_iyr {
        Ok(iyr_u64) => iyr_u64 >= 2010 && iyr_u64 <= 2020,
        Err(_) => false,
    }
}

fn validate_eyr(eyr: &str) -> bool {
    let p_eyr = eyr.parse::<u64>();
    match p_eyr {
        Ok(eyr_u64) => eyr_u64 >= 2020 && eyr_u64 <= 2030,
        Err(_) => false,
    }
}

fn validate_hgt(hgt: &str) -> bool {
    if hgt.ends_with("in") {
        let s = hgt.strip_suffix("in").unwrap();
        let s_u64 = s.parse::<u64>().unwrap();
        return s_u64 >= 59 && s_u64 <= 76;
    } else if hgt.ends_with("cm") {
        let s = hgt.strip_suffix("cm").unwrap();
        let s_u64 = s.parse::<u64>().unwrap();
        return s_u64 >= 150 && s_u64 <= 193;
    }
    false
}

fn validate_hcl(hcl: &str) -> bool {
    hcl.chars().nth(0).unwrap() == '#'
        && hcl.chars().skip(1).all(|c| match c {
            '0'..='9' => true,
            'a'..='f' => true,
            _ => false,
        })
}

fn validate_ecl(ecl: &str) -> bool {
    match ecl {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

fn validate_pid(pid: &str) -> bool {
    pid.len() == 9
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
        .filter(|b| {
            let mut ft = FieldTracker::default();
            for (k, _) in b.clone() {
                match k {
                    "byr" => ft.byr = true,
                    "iyr" => ft.iyr = true,
                    "eyr" => ft.eyr = true,
                    "hgt" => ft.hgt = true,
                    "hcl" => ft.hcl = true,
                    "ecl" => ft.ecl = true,
                    "pid" => ft.pid = true,
                    _ => {}
                }
            }
            ft.byr && ft.iyr && ft.eyr && ft.hgt && ft.hcl && ft.ecl && ft.pid
        })
        .count();

    println!("part1: {}", count);

    let count = data
        .clone()
        .filter(|b| {
            let mut ft = FieldTracker::default();
            for (k, v) in b.clone() {
                match k {
                    "byr" => ft.byr = validate_byr(v),
                    "iyr" => ft.iyr = validate_iyr(v),
                    "eyr" => ft.eyr = validate_eyr(v),
                    "hgt" => ft.hgt = validate_hgt(v),
                    "hcl" => ft.hcl = validate_hcl(v),
                    "ecl" => ft.ecl = validate_ecl(v),
                    "pid" => ft.pid = validate_pid(v),
                    _ => {}
                }
            }
            ft.byr && ft.iyr && ft.eyr && ft.hgt && ft.hcl && ft.ecl && ft.pid
        })
        .count();

    println!("part2: {}", count);
}
