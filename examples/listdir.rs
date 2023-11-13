use std::fs;

fn main() {
    fn print_dir(path_end: &str) {
        let mut entries_list: Vec<String> = Vec::new();
        if let Ok(entries) = fs::read_dir(path_end) {
            for entry in entries {
                if let Ok(entry) = entry {
                    // Here, `entry` is a `DirEntry`.
                    let filename = entry.file_name().into_string().unwrap();
                    // if filename is longer than 4 characters, slice last four characters from string
                    fn is_jpeg(filename: &str) -> bool {
                        if filename.len() > 4 {
                            // saturating sub here will subtract up to the limit of the integer type (zero for unsigned integer)
                            let filetype =
                                &filename[filename.len().saturating_sub(4)..filename.len()];
                            if filetype == ".jpg" {
                                return true;
                            } else {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }

                    if is_jpeg(&filename) {
                        let formatted_filename =
                            format!("% picture {}{} --alt {} %", &path_end, &filename, &filename);
                        println!("yapec here, {}", formatted_filename);
                        entries_list.push(filename);
                    }
                }
            }
        }
        println!("entries are {:?}", entries_list);
    }

    print_dir("./")
}
