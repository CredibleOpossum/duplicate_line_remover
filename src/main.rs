use std::collections::HashSet;
use std::process;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};


fn main() {
    let args: Vec<String> = env::args().collect();

    let err = match args.len() {
        1 => "No filename provided",
        i if i > 2 => "Too many arguments!",
        _ => "",
    };
    if err != "" {
        eprintln!("{}", err);
        process::exit(1);
    }

    match line_remover(args) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

fn line_remover(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open(&args[1])?);

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
