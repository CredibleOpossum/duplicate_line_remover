use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut seen_lines: HashSet<String> = HashSet::new();
    for line in stdin.lock().lines() {
        let text = line.unwrap();
        if text != "" && !seen_lines.contains(&text) {
            println!("{}", text);
            seen_lines.insert(text);
        }
    }
}
