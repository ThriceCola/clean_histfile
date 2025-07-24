use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let home = env::var("HOME").expect("Cannot find HOME environment variable");
    let his_file = PathBuf::from(home).join(".histfile");

    if !his_file.exists() {
        println!("The `.histfile` does not exist");
        return Ok(());
    }

    let mut content = Vec::new();
    File::open(&his_file)?.read_to_end(&mut content)?;

    // 处理非 UTF-8
    let content = String::from_utf8_lossy(&content);

    // 分割
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let lines_len = lines.len();

    // 从后向前处理，保留最新
    let mut seen = HashSet::new();
    let mut unique_lines = Vec::new();

    for line in lines.into_iter().rev() {
        if !line.trim().is_empty() && !seen.contains(&line) {
            seen.insert(line.clone());
            unique_lines.push(line);
        }
    }

    unique_lines.reverse();

    let unique_lines_len = unique_lines.len();

    let mut file = File::create(&his_file)?;
    for line in unique_lines {
        writeln!(file, "{}", line)?;
    }

    println!("{lines_len} lines -> {unique_lines_len} lines");

    Ok(())
}
