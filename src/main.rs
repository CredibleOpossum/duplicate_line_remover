use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::process;

fn main() {
    for file in env::args().skip(1) {
        match line_remover(file) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
    }
}

fn line_remover(file: String) -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open(file)?);
    let mut seen_lines: HashSet<String> = HashSet::new();
    for line in reader.lines() {
        let text = line?;
        if text != "" && !seen_lines.contains(&text) {
            println!("{}", text);
            seen_lines.insert(text);
        }
    }
    Ok(())
}
