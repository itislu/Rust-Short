use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let mut input = Vec::new();
    if let Err(e) = io::stdin().lock().read_to_end(&mut input) {
        eprintln!("{e}");
    }

    if let Err(e) = io::stdout().lock().write_all(&input) {
        eprintln!("{e}");
    }

    for arg in args {
        match fs::File::create(arg) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(&input) {
                    eprintln!("{e}");
                }
            }
            Err(e) => eprintln!("{e}"),
        }
    }
}
