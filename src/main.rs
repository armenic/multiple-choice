mod utils;
use std::fs;

fn main() {
    let contents = fs::read_to_string("tests.txt")
        .expect("There must be tests.txt file in the current directory");
    let split_contents: Vec<&str> = contents
        .split("\n")
        .filter(|s| s.trim().to_string() != "")
        .collect();
    let data: Vec<Vec<&str>> = split_contents
        .iter()
        .map(|i| i.split("|").map(|s| s.trim()).collect())
        .collect();

    utils::do_multiple_choice(data);
}
