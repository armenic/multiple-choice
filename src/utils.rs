use rand::seq::SliceRandom;
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
                if num < 1 || num > 4 {
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
        "\n\nPlease enter the corresponding number, between 1 and 4\n",
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
    let mut shuffled_indexes;
    loop {
        shuffled_indexes = seq::index::sample(&mut rng, data.len(), 3).into_vec();
        if !shuffled_indexes.contains(&correct_index) {
            shuffled_indexes.push(correct_index);
            shuffled_indexes = shuffled_indexes
                .choose_multiple(&mut rng, 4)
                .cloned()
                .collect();
            break;
        }
    }

    let shuffled_options = shuffled_indexes
        .iter()
        .enumerate()
        // 1) բառ բ
        // We show 1 based index
        .map(|(i, ii)| (i + 1).to_string() + ") " + &data[*ii][1])
        .collect::<Vec<_>>();

    (correct_index, shuffled_options)
}

pub fn do_multiple_choice(data: Vec<Vec<String>>) {
    let mut used_indexes = Vec::new();
    let mut score = 0;
    loop {
        let (correct_index, shuffled_options) = randomize(&data);

        if used_indexes.len() == data.len() {
            println!("Game over");
            println!("Score was {} out of {}", score, data.len());
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

        // We are showing 1 based index, hence need to subtract 1
        let guessed_option = &shuffled_options[guess_num - 1][3..];
        println!("\nYour answer was {}", guessed_option);
        let guessed_correctly = guessed_option == correct_entry[1];
        if guessed_correctly {
            println!("✅ That's correct, keep it up!\n");
            score += 1
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
