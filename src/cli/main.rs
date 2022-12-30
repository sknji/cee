use std::env;

pub static DEFAULT_FILENAME: &str = "tmp.s";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("{}: invalid number of arguments", args.len());
        return;
    }

    let input = args.get(1).unwrap();
    chibicc::gen(input, DEFAULT_FILENAME);
}
