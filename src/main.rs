use rand::{seq, Rng};
use std::io;

static DICTIONARY: [(&str, &str); 4] = [
    ("str a", "բառ ա"),
    ("str b", "բառ բ"),
    ("str g", "բառ գ"),
    ("str d", "բառ դ"),
];

fn main() {
    let mut used_indexes = Vec::new();
    loop {
        let (correct_index, shuffled_options) = randomize();

        if used_indexes.len() == DICTIONARY.len() {
            println!("Game over\n");
            return;
        }
        if used_indexes.contains(&correct_index) {
            continue;
        }
        used_indexes.push(correct_index);

        let correct_entry = DICTIONARY[correct_index];

        let guess_num = loop_for_user_answer(&correct_entry, &shuffled_options, used_indexes.len());

        let guessed_option = &shuffled_options[guess_num][3..];
        println!("\nYour answer was {}", guessed_option);
        let guessed_correctly = guessed_option == correct_entry.1;
        if guessed_correctly {
            println!("✅ That's correct, keep it up!\n")
        } else {
            println!("⚠ Sorry, that's not correct. Keep practicing!");
            println!("Correct answer was {}\n", correct_entry.1);
        }
    }
}

fn loop_for_user_answer(
    correct_entry: &(&str, &str),
    shuffled_options: &Vec<String>,
    q_n: usize,
) -> usize {
    let guess_num: usize;

    loop {
        println!("{}", make_question(correct_entry, shuffled_options, q_n));

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

fn get_user_input() -> String {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");

    guess
}

fn make_question(
    correct_entry: &(&str, &str),
    shuffled_options: &Vec<String>,
    q_n: usize,
) -> String {
    format!(
        "Question {}, find the match of\n\n{}\n\n{}{}",
        q_n.to_string(),
        String::from(correct_entry.0),
        shuffled_options.join("\n"),
        "\n\nPlease enter the corresponding number, between 0 and 3\n",
    )
}

fn randomize() -> (usize, Vec<String>) {
    let mut rng = rand::thread_rng();
    let correct_index = rng.gen_range(0..DICTIONARY.len());
    let shuffled_indexes = seq::index::sample(&mut rng, DICTIONARY.len(), 4).into_vec();

    let shuffled_options = shuffled_indexes
        .iter()
        .enumerate()
        // 1) բառ բ
        .map(|(i, ii)| i.to_string() + ") " + DICTIONARY[*ii].1)
        .collect::<Vec<_>>();

    (correct_index, shuffled_options)
}
