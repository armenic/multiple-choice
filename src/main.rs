mod utils;
use std::fs;

fn main() {
    let data = read_tests();
    utils::do_multiple_choice(data);
}

fn read_tests() -> Vec<Vec<String>> {
    let contents = fs::read_to_string("tests.txt")
        .expect("There must be tests.txt file in the current directory");
    let split_contents = contents
        .split("\n")
        .filter(|s| s.trim().to_string() != "")
        .collect::<Vec<_>>();
    let data = split_contents
        .iter()
        .map(|i| {
            i.split("|")
                .map(|s| s.trim().to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    data
}
