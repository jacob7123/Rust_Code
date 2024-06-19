use text_colorizer::*;

#[derive(Debug)]
#[allow(dead_code)]
struct Argument{
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}

fn main() {
    print_help();
}

fn print_help(){
    eprintln!("{} - replace a string with a new string", "Find and Replace".green());
    eprintln!("Usage: <target string> <replace string> <INPUT_FILE> <OUTPUT_FILE>");
}