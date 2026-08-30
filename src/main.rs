use anyhow::Result;
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Files to process
    files: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let files = if args.files.is_empty() {
        vec![PathBuf::from("-")]
    } else {
        args.files
    };

    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;

    for file in &files {
        let content = if file.to_str() == Some("-") {
            use std::io::Read;
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf
        } else {
            fs::read_to_string(file)?
        };

        let lines = content.lines().count();
        let words = content.split_whitespace().count();
        let bytes = content.len();

        println!(
            "{}: {} lines, {} words, {} bytes",
            file.display(),
            lines,
            words,
            bytes
        );

        total_lines += lines;
        total_words += words;
        total_bytes += bytes;
    }

    if files.len() > 1 {
        println!(
            "\ntotal: {} lines, {} words, {} bytes",
            total_lines, total_words, total_bytes
        );
    }
    Ok(())
}
