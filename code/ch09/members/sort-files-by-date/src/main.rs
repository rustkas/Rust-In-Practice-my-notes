use sort_files_by_date::sort_files_by_date;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: sort-files-by-date <directory>");
        std::process::exit(1);
    }
    let dir = &args[1];
    sort_files_by_date(dir);
}
