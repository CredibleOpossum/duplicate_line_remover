use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn deduplicate_file(input_path: &str, output_path: &str, executable: &str) {
    let input_file = File::open(input_path);
    let input_file = match input_file {
        Ok(file) => file,
        Err(error) => {
            eprintln!("{}: could not open file with error: {}", executable, error);
            return;
        }
    };

    let output_file = File::create(output_path);
    let mut output_file = match output_file {
        Ok(file) => file,
        Err(error) => {
            eprintln!("{}: cannot create file with error: {}", executable, error);
            return;
        }
    };

    let mut seen_lines = HashSet::new();
    let reader = BufReader::new(input_file);
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                eprintln!("{}: could read line with error: {}", executable, error);
                return;
            }
        };
        if line != "" && !seen_lines.contains(&line) {
            match output_file.write_all(format!("{}\n", line).as_bytes()) {
                Ok(()) => {}
                Err(error) => {
                    panic!("failed writing to file with error: {}", error);
                }
            }
            seen_lines.insert(line);
        }
    }
}

fn deduplicate_iterable<'a>(
    iterable: impl Iterator<Item = Result<String, std::io::Error>>,
    executable: &str,
) {
    let mut seen_lines = HashSet::new();
    for line in iterable {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                eprintln!("{}: could read line with error: {}", executable, error);
                return;
            }
        };
        if line != "" && !seen_lines.contains(&line) {
            println!("{}", line);
            seen_lines.insert(line);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_count = args.len();
    if arg_count > 1 {
        match arg_count {
            2 => {
                eprintln!("{}: destination file not specified", args[0])
            }
            3 => {
                deduplicate_file(&args[1], &args[2], &args[0]);
            }
            _ => {
                eprintln!("{}: too many arguments specified", args[0])
            }
        }
    } else {
        deduplicate_iterable(io::stdin().lock().lines(), &args[0])
    }
}
