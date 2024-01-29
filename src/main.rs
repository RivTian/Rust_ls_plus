use std::path::PathBuf;

use bytesize::ByteSize;
use clap::Parser;
use colored::Colorize;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser)]
struct Options {
    #[arg(short, long, value_name = "PATH}")]
    path: Option<PathBuf>,
    #[arg(long, default_value_t = 1)]
    min_depth: usize,
    #[arg(long, default_value_t = 1)]
    max_depth: usize,
    #[arg(long, default_value_t = false)]
    hidden: bool,
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn main() {
    let options = Options::parse();

    for entry in WalkDir::new(options.path.unwrap_or(".".into()))
        .min_depth(options.min_depth)
        .max_depth(options.max_depth)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
    {
        let entry = entry.unwrap();
        let size = entry.metadata().unwrap().len();
        let formatted_entry = if entry.file_type().is_dir() {
            entry.path().display().to_string().blue()
        } else if entry.file_type().is_file() {
            entry.path().display().to_string().white()
        } else {
            entry.path().display().to_string().yellow()
        };
        println!(
            "{:>9}\t{formatted_entry}",
            format!("{}", ByteSize(size)).green()
        );
    }
}
