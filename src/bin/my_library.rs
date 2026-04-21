use my_library::read_stdin;

fn main() {
    let buffer = read_stdin();
    let result: Vec<&str> = buffer.split(" ").collect();
    for i in 0..result.len() {
        println!("Word {}: {}", i+1, result[i]);
    }
}