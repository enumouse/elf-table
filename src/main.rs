//! A simple elf header table viewer

use std::env;
use std::fs::File;
use std::io::Read;
use elf::ElfBytes;
use table::Table;

mod table;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut table = Table::new();
    
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
    let elf = ElfBytes::<elf::endian::AnyEndian>::minimal_parse(&contents)
        .expect("Failed to parse ELF");
    let header = elf.ehdr;

    // Add rows
    let rows = vec![
        ("Class", format!("{:?}", header.class)),
        ("Version", format!("{:?}", header.version)),
        ("OS ABI", format!("{:?}", header.osabi)),
        ("ABI Version", format!("{}", header.abiversion)),
        ("Type", format!("{:?}", header.e_type)),
        ("Machine", format!("{:?}", header.e_machine)),
        ("Version (Hex)", format!("{:#x}", header.version)),
        ("Entry Point", format!("{:#x}", header.e_entry)),
        ("Program Header Offset", format!("{}", header.e_phoff)),
        ("Section Header Offset", format!("{}", header.e_shoff)),
        ("Flags", format!("{:#x}", header.e_flags)),
        ("Header Size", format!("{}", header.e_ehsize)),
        ("Program Header Entry Size", format!("{}", header.e_phentsize)),
        ("Program Header Count", format!("{}", header.e_phnum)),
        ("Section Header Entry Size", format!("{}", header.e_shentsize)),
        ("Section Header Count", format!("{}", header.e_shnum)),
        ("Section Header String Index", format!("{}", header.e_shstrndx)),
    ];

    for (title, value) in rows {
        table.add_row(title, &value);
    }

    // Draw Table
    table.draw();
}
