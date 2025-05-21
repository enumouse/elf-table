//! A simple elf header table viewer

use std::env;
use std::fs::File;
use std::io::Read;
use table::Table;

mod table;

fn main() {
    let args: Vec<String> = env::args().collect();
    let table = Table::new();
    
    // Check we have parameters
    if args.len() < 2 {
        eprintln!("usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    // Load File
    let filename = &args[1];
    let mut file = File::open(filename)
        .expect("Unable to open file");

    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("Unable to read file");

    // Fetch Header Data

}
