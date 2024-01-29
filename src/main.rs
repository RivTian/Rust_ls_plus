use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn main() {
    // TODO Do This or That:
    for entry in WalkDir::new(".")
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
    {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
    }
}
