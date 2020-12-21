use std::collections::HashMap;

fn apply_mask(mask: &str, addr: &str) -> String {
    let mut new_addr = String::with_capacity(addr.len());
    for (i, c) in mask.chars().enumerate() {
        match c {
            '1' => new_addr.push('1'),
            'X' => new_addr.push('X'),
            '0' => new_addr.push(addr.chars().nth(i).unwrap()),
            _ => panic!("Bad mask input"),
        }
    }
    new_addr
}

fn generate_addrs(addr_superposition: &str) -> Vec<usize> {
    let mut addrs = Vec::with_capacity(1024);
    _generate_addrs(addr_superposition, &mut addrs);
    addrs
}

fn _generate_addrs(addr_superposition: &str, addrs: &mut Vec<usize>) {
    if let Some((l, r)) = addr_superposition.split_once('X') {
        let new_addr = format!("{}0{}", l, r);

        _generate_addrs(&new_addr, addrs);

        let new_addr = format!("{}1{}", l, r);
        _generate_addrs(&new_addr, addrs);
    } else {
        println!("{}", addr_superposition);
        println!("{}", addr_superposition.len());
        addrs.push(usize::from_str_radix(addr_superposition, 2).unwrap());
    }
}

fn generate_memory<'a>(data: impl Iterator<Item = (&'a str, &'a str)>) -> HashMap<usize, u64> {
    let mut memory = HashMap::new();
    let mut mask = "";
    for (line_type, v) in data {
        match line_type.starts_with("mem[") {
            true => {
                let mem_loc = format!(
                    "{:036b}",
                    line_type
                        .strip_prefix("mem[")
                        .unwrap()
                        .strip_suffix("]")
                        .unwrap()
                        .parse::<u64>()
                        .unwrap()
                );
                let mem_value = v.parse::<u64>().unwrap();
                let addr_superposition = apply_mask(mask, &mem_loc);
                for addr in generate_addrs(&addr_superposition) {
                    memory.insert(addr, mem_value);
                }
            }
            false => mask = v,
        }
    }
    memory
}

pub fn solve<'a>(data: impl Iterator<Item = (&'a str, &'a str)>) -> u64 {
    let memory = generate_memory(data);
    let sum: u64 = memory.iter().map(|(_, v)| v).sum();
    sum
}
