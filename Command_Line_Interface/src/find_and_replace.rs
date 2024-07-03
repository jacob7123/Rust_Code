use text_colorizer::*;
use std::env;

#[derive(Debug)]
#[allow(dead_code)]
struct Argument{
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}

fn print_help(){
    eprintln!("{} - replace a string with a new string", "Find and Replace".green());
    eprintln!("Usage: <target string> <replace string> <INPUT_FILE> <OUTPUT_FILE>");
}

pub fn run(){
    // print_help();

    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4{
        print_help();
        eprintln!("{} wrong number of arguments give. Expected 4, got {}", "Error".red().bold(), args.len());
        std::process::exit(1);
    }
} 