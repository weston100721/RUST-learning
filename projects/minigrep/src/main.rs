use std::env;
use std::fs;
use std::string::String;
use std::vec::Vec;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];
    println!("searching for {} in {}", query, filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n\n{}", contents);
}

fn parse_args(args: &[String]) -> (&String, &String) {
    let keywords = &args[1];
    let filename = &args[2];

    (keywords, filename)
}
