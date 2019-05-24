// Functions used to read input from command line, read files,
// easily manipulate strings.

use std::fs::File;
use std::io;
use std::io::prelude::*; 
use std::path::Path;

pub fn prompt_user_choice(user_choices: &Vec<UserChoices>) -> usize {
    println!("\n===============");
    println!("=== Choices ===");
    println!("===============");
    for (i, choice) in user_choices.iter().enumerate() {
        println!("[{}] {}", i, choice.prompt);
    }
    println!("-----");
    
    prompt_read_idx("Index of chosen? ")
}


pub fn prompt_read_idx(prompt: &str) -> usize {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut my_idx = String::new();
    io::stdin().read_line(&mut my_idx)
        .expect("Failed to read line");
    // let user_input: Vec<char> = my_idx.trim().chars().collect();
    // user_input[0].to_digit(10).expect("Index not found. Input a digit")
    my_idx.trim().parse().expect("Please type a valid index")
}

pub fn prompt_read_string(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut my_idx = String::new();
    io::stdin().read_line(&mut my_idx)
        .expect("Failed to read line");
    my_idx.trim().parse().expect("Please type a valid response")
}

pub fn clear_string_vals(strings: Vec<&mut String>) {
    for s in strings {
        s.clear()
    }
}


#[derive(Deserialize, Debug)]
pub struct UserChoices {
    pub prompt: String,
    pub url_segment: String,
    pub next_prompts: Vec<UserChoices>,
}

pub fn read_json_file(fp: &Path) -> Vec<UserChoices> {
    // Read the input file to string
    let json_file = File::open(fp).expect("File not there");
    let deserialized: Vec<UserChoices> =
        serde_json::from_reader(json_file).expect("error while reading json");
    deserialized
}


