use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("not enough arguments")
    }

    let query = args.get(1).unwrap();
    let file_path = args.get(2).unwrap();

    let contexts = fs::read_to_string(file_path).expect("should have been able to read the file");
    for (line, context) in contexts.lines().enumerate() {
        if context.contains(query) {
            println!("{}:{}", line + 1, context);
        }
    }
}
