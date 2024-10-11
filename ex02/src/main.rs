use std::env;
use std::fs;
use std::io;
use std::path;

fn main() {
    if env::args().len() != 2 {
        return eprintln!("Exactly 1 argument required");
    }
    let arg = env::args().nth(1).unwrap();
    let path = path::Path::new(&arg);

    match get_size(path) {
        Ok(size) => print_formatted_size(size),
        Err(e) => eprintln!("{e}"),
    }
}

fn get_size(path: &path::Path) -> Result<u64, io::Error> {
    let mut size: u64 = 0;

    if path.is_dir() {
        size += path.metadata()?.len();
        for entry in fs::read_dir(path)? {
            if let Ok(good_entry) = entry {
                size += get_size(good_entry.path().as_path())?;
            } else {
                continue;
            }
        }
    } else {
        size += path.metadata()?.len();
    }
    Ok(size)
}

fn print_formatted_size(size: u64) {
    let fsize: f64;
    let unit: &str;
    match size {
        0..=1000 => {
            unit = "bytes";
            fsize = size as f64;
        }
        1001..=1000000 => {
            unit = "kilobytes";
            fsize = size as f64 / 1000.0;
        }
        1000001..=1000000000 => {
            unit = "megabytes";
            fsize = size as f64 / 1000000.0;
        }
        _ => {
            unit = "gigabytes";
            fsize = size as f64 / 1000000000.0;
        }
    }
    println!("{fsize:.1} {unit}");
}
