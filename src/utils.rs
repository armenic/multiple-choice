use rand::{seq, Rng};
use std::{fs, io};

fn loop_for_user_answer(
    correct_entry: &Vec<String>,
    shuffled_options: &Vec<String>,
    q_n: usize,
    d_l: usize,
) -> usize {
    let guess_num: usize;

    loop {
        println!(
            "{}",
            make_question(correct_entry, shuffled_options, q_n, d_l)
        );

        let guess = get_user_input();

        match guess.trim().parse() {
            Ok(num) => {
                if num > 3 {
                    continue;
                }
                guess_num = num;
                return guess_num;
            }
            Err(_) => continue,
        };
    }
}

fn make_question(
    correct_entry: &Vec<String>,
    shuffled_options: &Vec<String>,
    q_n: usize,
    d_l: usize,
) -> String {
    format!(
        "Question {} out of {}, find the match of\n\n{}\n\n{}{}",
        q_n.to_string(),
        d_l.to_string(),
        correct_entry[0],
        shuffled_options.join("\n"),
        "\n\nPlease enter the corresponding number, between 0 and 3\n",
    )
}

fn get_user_input() -> String {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");

    guess
}

fn randomize(data: &Vec<Vec<String>>) -> (usize, Vec<String>) {
    let mut rng = rand::thread_rng();
    let correct_index = rng.gen_range(0..data.len());
    let shuffled_indexes = seq::index::sample(&mut rng, data.len(), 4).into_vec();

    let shuffled_options = shuffled_indexes
        .iter()
        .enumerate()
        // 1) բառ բ
        .map(|(i, ii)| i.to_string() + ") " + &data[*ii][1])
        .collect::<Vec<_>>();

    (correct_index, shuffled_options)
}

pub fn do_multiple_choice(data: Vec<Vec<String>>) {
    let mut used_indexes = Vec::new();
    loop {
        let (correct_index, shuffled_options) = randomize(&data);

        if used_indexes.len() == data.len() {
            println!("Game over\n");
            return;
        }
        if used_indexes.contains(&correct_index) {
            continue;
        }
        used_indexes.push(correct_index);

        let correct_entry = &data[correct_index];

        let guess_num = loop_for_user_answer(
            &correct_entry,
            &shuffled_options,
            used_indexes.len(),
            data.len(),
        );

        let guessed_option = &shuffled_options[guess_num][3..];
        println!("\nYour answer was {}", guessed_option);
        let guessed_correctly = guessed_option == correct_entry[1];
        if guessed_correctly {
            println!("✅ That's correct, keep it up!\n")
        } else {
            println!("⚠ Sorry, that's not correct. Keep practicing!");
            println!("Correct answer was {}\n", correct_entry[1]);
        }
    }
}

pub fn read_tests() -> Vec<Vec<String>> {
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
