use std::collections::HashMap;

fn apply_mask(mask: &str, mut val: u64) -> u64 {
    let bitshift_offset = mask.len() - 1;
    for (i, c) in mask.chars().enumerate() {
        match c {
            '0' => val &= !(1 << (bitshift_offset - i)),
            '1' => val |= 1 << (bitshift_offset - i),
            'X' => {}
            _ => panic!("Bad mask input"),
        }
    }
    val
}

fn generate_memory<'a>(data: impl Iterator<Item = (&'a str, &'a str)>) -> HashMap<usize, u64> {
    let mut memory = HashMap::new();
    let mut mask = "";
    for (line_type, v) in data {
        match line_type.starts_with("mem[") {
            true => {
                let mem_loc = line_type
                    .strip_prefix("mem[")
                    .unwrap()
                    .strip_suffix("]")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let mem_value = apply_mask(mask, v.parse::<u64>().unwrap());
                memory.insert(mem_loc, mem_value);
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
