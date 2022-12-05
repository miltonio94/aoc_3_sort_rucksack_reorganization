use std::{env, process};

fn main() {
    let file_path = match env::args().skip(1).next() {
        Some(string) => string,
        None => {
            eprint!("Error: Missing file path argument");
            process::exit(1);
        }
    };

    if let Err(e) = aoc_3_sort_rucksack_reorganization::run(file_path) {
        eprint!("Error {e}");
        process::exit(1)
    }
}
