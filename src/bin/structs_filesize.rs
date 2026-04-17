use std::env;

#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
    terabytes: String,
}

fn fill_sizes(bytes: u64) -> Sizes {
    Sizes {
        bytes: format!("{} bytes", bytes),
        kilobytes: format!("{} kilobytes", bytes / 1_000),
        megabytes: format!("{} megabytes", bytes / 1_000_000),
        gigabytes: format!("{} gigabytes", bytes / 1_000_000_000),
        terabytes: format!("{} terabytes", bytes / 1_000_000_000_000),
    }
}

fn get_bytes(s: &String) -> u64 {
    let s_parts: Vec<&str> = s.split_whitespace().collect();
    if s_parts.len() != 2 as usize {
        panic!("Incorrect input: introduce an input of the form: number unit");
    }
    let size: u64 = s_parts[0].parse::<u64>().unwrap();

    match s_parts[1] {
        "kb" => size * 1_000,
        "mb" => size * 1_000_000,
        "gb" => size * 1_000_000_000,
        "tb" => size * 1_000_000_000_000,
        _ => size,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let bytes: u64 = get_bytes(&args[1]);

    println!("{:?}", fill_sizes(bytes));
}
