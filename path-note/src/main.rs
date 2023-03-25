use inquire::{InquireError, Select, Text};
use std::process::Command;

// * PROGRAM DESCRIPTION *
// * A temp file path note CLA *
// * Given a file path like "C:\Users\user\Documents\file.txt" -> Saves it into the CLI *
// * When one of the items in CLI is selected -> It will open the file path *

fn main() {
    // file paths 
    let mut items: Vec<String> = Vec::new();

    loop {
        let mut options: Vec<&str> = vec!["[Add Item]", "[Remove Item]", "[Exit]"];

        // give an index to each item
        let enumared_items = items
            .iter()
            .enumerate()
            .map(|(i, item)| format!("{}: {}", i, item))
            .collect::<Vec<String>>();

        // convert the items to a &str
        let mut items_as_str = enumared_items.iter().map(|x| x.as_str()).collect();

        options.append(&mut items_as_str);

        let ans: Result<&str, InquireError> =
            Select::new("-- Select an option or file path --", options).prompt();

        match ans {
            Ok(ans) => {
                match ans {
                    "[Add Item]" => {
                        println!("Tip: You can add mutiple paths by seperating them with a space.");

                        let item = Text::new("Enter the file path:").prompt();

                        match item {
                            Ok(item) => {
                                let paths = item.split(" ").collect::<Vec<&str>>();
                                items.append(
                                    &mut paths
                                        .iter()
                                        .map(|x| x.to_string().trim().replace("\"", ""))
                                        .collect(),
                                );
                            }
                            Err(e) => {
                                println!("Path could not be added: {}", e);
                            }
                        }
                    }
                    "[Remove Item]" => {
                        let item = Text::new("Enter the file path to remove:").prompt();

                        match item {
                            Ok(item) => {
                                // chech if item is an index
                                if item.parse::<usize>().is_ok() {
                                    let index = item.parse::<usize>().unwrap();
                                    if index < items.len() {
                                        items.remove(index);
                                    } else {
                                        println!("Index out of range");
                                    }
                                } else {
                                    items.retain(|x| x != &item);
                                }
                            }
                            Err(e) => {
                                println!("Path could not be removed: {}", e);
                            }
                        }
                    }
                    "[Exit]" => {
                        println!("Exiting...");
                        std::process::exit(0);
                    }

                    // open the file path
                    item => {
                        // item format: <index>: <file path>

                        let filepath = item.split(": ").collect::<Vec<&str>>()[1];

                        let filepath = format!(r#"{}"#, filepath); // convert to raw string

                        println!("Opening {}...", filepath);

                        // open the file path
                        Command::new("explorer")
                            .arg(filepath)
                            .output()
                            .expect("Failed to open file path");
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
