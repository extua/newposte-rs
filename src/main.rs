use chrono::prelude::*;
use std::fs::File;
use std::io::stdin;
use std::io::stdout;
use std::io::{BufWriter, Write};


fn main() {

//  function which returns array
// do you want to add tags? y-n
//      if yes, create new array, if no move on
// > function user input to add tag to array
// do you have more tags to add? y-n
//      if yes, return to function, if no move on
   

    fn set_tags() -> Vec<String> {

        let mut tag_list:Vec<String> = Vec::new();

        let mut choice = String::new();
        
        println!("enter tag:");
        let mut inputted_string = set_input();
        tag_list.push(inputted_string);
        println!("tag list is {:?}", tag_list);
        
        println!("do you want to enter more tags? (y/n)");
        
        let mut choice = set_input();

        while choice.to_lowercase().trim() == "y" {
                choice.clear();
                println!("enter tag:");
                
                let mut inputted_string = set_input();
                tag_list.push(inputted_string);
                println!("tag list is {:?}", tag_list);
                println!("do you want to enter more tags? (y/n)");
                let mut choice = set_input();
           }
        return tag_list;
    }
    
    
    println!("Enter tag input function? (y/n)");
    if set_input().to_lowercase().trim() == "y" {
        let array_result = set_tags();
        println!("{:?}",array_result);
    };
    
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
    write!(file, "layout: post\n").expect("failed to write post layout to file");
    write!(file, "title: {}", title).expect("failed to write title to file");
    if !location.trim().is_empty() {
        println!("writing the location to file");
        write!(file, "location: {}\n", location.trim()).expect("failed to write the location to file");
    }
    write!(file, "---\n\n").expect("failed to write bottom front matter dots");
    println!("everything written to file");

}
