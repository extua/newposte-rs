use chrono::prelude::*;
use std::fs::File;
use std::io::stdin;
use std::io::stdout;
use std::io::{BufWriter, Write};


fn main() {

    fn set_tags() -> String {
        let mut tag_list:Vec<String> = Vec::new();
        println!("enter tag:");
        let mut inputted_string = set_input().trim().to_string();
        tag_list.push(inputted_string);
        println!("the tag list is {:?}", tag_list);
        println!("do you want to enter more tags? (y/n)");
        while set_input().to_lowercase().trim() == "y" {
                println!("enter tag:");
                let mut inputted_string = set_input().trim().to_string();
                tag_list.push(inputted_string);
                println!("the tag list is now {:?}", tag_list);
                println!("do you want to enter more tags? (y/n)");
           }
        let tag_list_string = format!("[{}]",tag_list.join(", "));
        return tag_list_string
    }
    
    println!("Enter tag input function? (y/n)");
    let tag_list = if set_input().to_lowercase().trim() == "y" {
        let array_result = set_tags();
        array_result
    } else {"".to_string()};
    
    println!("{:?}",tag_list);

    // Set date today
    let local: DateTime<Local> = Local::now();
    let date_today = format!("{}", local.format("%Y-%m-%d"));
    println!("date now is {}", date_today);
    
    // function to request input from user
    fn set_input() -> String {
        let mut input_string = String::new();
        stdin().read_line(&mut input_string).unwrap();
        return input_string;
    }
    
    // filter special characters from title
    fn filter_input(raw_string: &str) -> String {
            let filtered_string = raw_string.to_lowercase()
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
            }).collect();
            return filtered_string
    }
    
    // set title from input
    println!("write title");
    let title = set_input();
    let lowertitle = filter_input(&title);

    println!("Lowercase title is {lowertitle}");

    println!("write location");
    let location = set_input();

    println!("You set the location as {:?}", location);

    // Create file and write to it
    let filename = format!("{}-{}.md", date_today, lowertitle);
    println!("You set the filename as {}", filename);

    let file = File::create(filename).expect("unable to create file");
    let mut file = BufWriter::new(file);
    
    write!(file, "---\n").expect("failed to write top front matter dots");
    write!(file, "title: {}", title).expect("failed to write title to file");
    if !location.trim().is_empty() {
        println!("writing the location to file");
        write!(file, "location: {}\n", location.trim()).expect("failed to write the location to file");
    }
    if !tag_list.is_empty() {
        println!("writing tags to file");
        write!(file, "tags: {}\n", tag_list).expect("failed to write tags to file");
    }
    write!(file, "---\n\n").expect("failed to write bottom front matter dots");
    println!("everything written to file");

}
