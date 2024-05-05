/// This library - "walkdir" does not work on Windows.

// use std::fs;
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn sort_files_by_date(dir: &str) {
    let entries = WalkDir::new(dir);
    let mut iterator = entries.into_iter();
    let _ = iterator.next();
    for entry in iterator {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
    }

    let entries = WalkDir::new(dir).follow_links(true);
    let mut iterator = entries.into_iter();
    let _ = iterator.next();
    for entry in iterator {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
    }

    let entries = WalkDir::new(dir);
    let mut iterator = entries.into_iter();
    let _ = iterator.next();
    for entry in iterator.filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
    }

    // let mut file_paths = Vec::new();

    // let walker = WalkDir::new(dir).max_open(1).follow_links(true).follow_root_links(true).into_iter();

    // for entry in walker.filter_map(|e| e.ok()) {
    //     if entry.file_type().is_file() {
    //         file_paths.push(entry.path().to_owned());
    //     }
    // }

    // println!("All files:");
    // for path in &file_paths {
    //     println!("{}", path.display());
    // }

    //   let mut entries: Vec<_> = WalkDir::new(dir)
    //     .into_iter().
    //     filter_map(|e| e.ok()).filter(|entry| entry
    //     .file_type().is_file()).collect();

    //   println!("Entries: {entries:?}");

    //   entries.sort_by(|a, b| {
    //       let a_metadata = fs::metadata(a.path()).unwrap();
    //       let b_metadata = fs::metadata(b.path()).unwrap();
    //       a_metadata
    //           .modified()
    //           .unwrap()
    //           .cmp(&b_metadata.modified().unwrap())
    //   });

    //   for entry in entries {
    //       println!("{}", entry.path().display());
    //   }
}
