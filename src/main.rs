

// script to take input from user
use std::io;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use colored::*;

fn open_file(file:&str) {
    let path = Path::new(file);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("{} contains:\n{}", display, s),
    }

}

fn open_file_optimised(file: &str) {
    let path = Path::new(file);
    if let Ok(contents) = fs::read_to_string(&path) {
        println!("{} contains:\n{}", path.display(), contents);
    } else {
        eprintln!("Couldn't open or read {}", path.display());
    }
}

fn print_instructions() {
    println!("{}","Welcome to custom rust cli!!".green());
    println!("{}","Here are your commands:".green());
    println!("{}","echo <MESSAGE>: prints <MESSGAE> to the screen".blue());
    println!("{}","cat <FILE>: show contents of the <FILE>".blue());
}

fn get_input_and_get_params() -> Vec<String> {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read input");
    //trim -> removes leading and trailing whitespaces
    //split_whitespace -> splits a string into an iterator based on space as delimiter
    //map(String::from) -> converts &str to String
    //collect -> is used to collect the results of the map operation into a Vec<String>. It gathers all the converted substrings into a vector of owned strings.
    let params: Vec<String> = guess.trim().split_whitespace().map(String::from).collect();
    params
}

fn handle_cli(mut command: Vec<String>) {
    match command[0].as_str() {
        "echo" => {
            if command.len() == 1 {
                command.push(" ".to_string());
            }
            println!("{}",command[1]);
        }
        "cat" => {
            open_file(command[1].as_str())
        }
        _ => {
            println!("Unknown command: {}", command[0]);
        }
    }
}

fn handle_cli_optimised(command: Vec<String>) {
    match command.get(0).map(String::as_str) {
        Some("echo") => {
            //.get(): This retrieves the element at index 1 from the command vector. If there's an element at that index, it returns Some(element), otherwise, it returns None.
            //.cloned(): If there's a reference in the Some(element) variant, cloned() creates a new owned copy of the element. If the variant is None, it does nothing. This ensures that you get an owned value (either a cloned value or a new default value) instead of a reference.
            //.unwrap_or_else(|| String::from(" ")): This is used to handle the possibility of None from get(1). If Some(element) is returned, it unwraps and returns the cloned element. If None is returned, it executes the closure passed to unwrap_or_else, which creates a new String containing a space.
            let argument = command.get(1).cloned().unwrap_or_else(|| String::from(" "));
            println!("{}", argument);
        }
        Some("cat") => {
            if let Some(file) = command.get(1) {
                open_file(file);
            } else {
                println!("Usage: cat <filename>");
            }
        }
        _ => {
            println!("Unknown command: {}", command.get(0).map_or("", String::as_str));
        }
    }
}

fn main() {
    print_instructions();
    let params:Vec<String> = get_input_and_get_params();
    assert!(params.len() >= 1);
    handle_cli(params);
}
