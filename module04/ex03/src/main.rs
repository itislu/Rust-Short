use std::env;
use std::io;
use std::io::BufRead;
use std::os::unix::process::CommandExt;
use std::process;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let mut input = String::new();
    while io::stdin().lock().read_line(&mut input).unwrap() != 0 {
        args.push(input.trim_end_matches('\n').to_string());
        input.clear();
    }

    if !args.is_empty() {
        process::Command::new(args.remove(0)).args(args).exec();
    }
}
