use chrono::prelude::*;
use std::fs;
use std::fs::File;
use std::io::stdin;
//use std::io::stdout;
use std::io::{BufWriter, Write};

fn main() {
    // function to request input from user
    fn set_input() -> String {
        let mut input_string = String::new();
        stdin().read_line(&mut input_string).unwrap();
        input_string
    }

    // Set date today
    let local: DateTime<Local> = Local::now();
    let date_today = format!("{}", local.format("%Y-%m-%d"));
    println!("date now is {}", date_today);

    // Filter special characters from title
    fn filter_input(raw_string: &str) -> String {
        let filtered_string = raw_string
            .to_lowercase()
            .chars()
            .filter_map(|c| {
                if c.is_alphanumeric() {
                    Some(c)
                } else if c == '\n' {
                    None
                } else if c.is_whitespace() {
                    Some('_')
                } else {
                    None
                }
            })
            .collect();
        filtered_string
    }

    // Set title from input
    println!("write title");
    let title = set_input();
    let lowertitle = filter_input(&title);

    println!("Lowercase title is {lowertitle}");

    // Create file and write to it
    // for future working out, drafts must exist, and if not
    // you must create it!
    let filename = format!("_drafts/{}-{}.md", date_today, lowertitle);
    println!("You set the filename as {}", filename);

    // Function to set and format tags
    fn set_tags() -> String {
        let mut tag_list: Vec<String> = Vec::new();
        println!("enter tag:");
        let inputted_string = set_input().trim_end().to_string();
        tag_list.push(inputted_string);
        println!("the tag list is {:?}", tag_list);
        println!("do you want to enter more tags? (y/n)");
        while set_input().to_lowercase().trim_end() == "y" {
            println!("enter tag:");
            let inputted_string = set_input().trim_end().to_string();
            tag_list.push(inputted_string);
            println!("the tag list is now {:?}", tag_list);
            println!("do you want to enter more tags? (y/n)");
        }
        let tag_list_string = format!("[{}]", tag_list.join(", "));
        tag_list_string
    }

    println!("Enter tags? (y/n)");
    let tag_list: String = if set_input().to_lowercase().trim_end() == "y" {
        set_tags()
    } else {
        "".to_string()
    };
    println!("{:?}", tag_list);

    println!("write location");
    let location = set_input();

    println!("You set the location as {:?}", location);

    fn print_dir(path_end: &str) -> Vec<String> {
        let mut entries_list: Vec<String> = Vec::new();
        let full_path = format!("media/2023/{}", path_end);
        if let Ok(entries) = fs::read_dir(&full_path) {
            for entry in entries.flatten() {
                // Here, `entry` is a `DirEntry`.
                let filename = entry.file_name().into_string().unwrap();
                // if filename is longer than 4 characters,
                // slice last four characters from string
                fn is_photo(filename: &str) -> bool {
                    if filename.len() > 4 {
                        // saturating sub here will subtract
                        // up to the limit of the integer type
                        // (zero for unsigned integer)
                        let filetype: &str = &filename
                            [filename.len().saturating_sub(4)..filename.len()];
                        let filetypes_to_match: [&str; 2] = [".jpg", ".jxl"];
                        if filetypes_to_match.contains(&filetype) {
                            // println!("{} found", &filetype);
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }

                if is_photo(&filename) {
                    // define alt_text and convert underscores to spaces
                    let alt_text: &mut String =
                        &mut filename.replace("'_'", " ");
                    // cut off the last 4 characters of the filename
                    let alt_text_len: usize = alt_text.len();
                    alt_text.truncate(alt_text_len - 4);

                    let formatted_filename: String = format!(
                        "{{% picture {}{} --alt {} %}}",
                        &full_path[6..],
                        &filename,
                        &alt_text
                    );
                    println!("found {}", &filename);
                    entries_list.push(formatted_filename);
                }
            }
        } else {
            println!("Directory not found")
        }
        entries_list
    }

    println!("Add pictures? (y/n)");
    let pictures_list_string: String =
        if set_input().to_lowercase().trim_end() == "y" {
            println!("Enter media directory:");
            println!("in month/title/ format, eg. 10/art_museum/");
            let directory_stem = set_input();
            let pictures_list = print_dir(&directory_stem.trim_end());
            pictures_list.join("\n")
        } else {
            println!("Not adding pictures");
            "".to_string()
        };

    // Now write everything out to a file
    let file = File::create(filename).expect("unable to create file");
    let mut file = BufWriter::new(file);

    write!(file, "---\n").expect("failed to write top front matter dots");
    write!(file, "layout: post\n")
        .expect("failed to write post layout to file");
    write!(file, "title: {}", title).expect("failed to write title to file");
    if !location.trim_end().is_empty() {
        println!("writing the location to file");
        write!(file, "location: {}\n", location.trim_end())
            .expect("failed to write the location to file");
    }
    if !tag_list.is_empty() {
        println!("writing tags to file");
        write!(file, "tags: {}\n", tag_list)
            .expect("failed to write tags to file");
    }
    write!(file, "---\n\n").expect("failed to write bottom front matter dots");
    write!(file, "{}", pictures_list_string)
        .expect("failed to write picture list");

    println!("everything written to file");
}
