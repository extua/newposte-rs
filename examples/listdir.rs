use std::fs;

fn main() {
    fn print_dir(path_end: &str) {
        let mut entries_list: Vec<String> = Vec::new();
        if let Ok(entries) = fs::read_dir(path_end) {
            for entry in entries {
                if let Ok(entry) = entry {
                    // Here, `entry` is a `DirEntry`.
                    let filename = entry.file_name().into_string().unwrap();
                    // slice last four characters from string
                    let filetype = &filename[filename.len().saturating_sub(4)..filename.len()];
                    if filetype == ".jpg" {
                        println!("yapec here");
                        entries_list.push(filename);
                    };

                    println!("the tag list is now {:?}", entries_list);
                }
            }
        }
    }

    print_dir("./")
}
