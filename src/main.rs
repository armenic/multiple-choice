use rand::{seq, Rng};
use std::io;

static DICTIONARY: [(&str, &str); 4] = [
    ("str a", "բառ ա"),
    ("str b", "բառ բ"),
    ("str g", "բառ գ"),
    ("str d", "բառ դ"),
];

fn main() {
    loop {
        let (correct_entry, wrong_options) = randomize();

        let guess_num = loop_for_user_answer(&correct_entry, &wrong_options);

        let guessed_option = &wrong_options[guess_num][3..];
        println!("\nYour answer was {}", guessed_option);
        let guessed_correctly = guessed_option == correct_entry.1;
        if guessed_correctly {
            println!("That's correct, keep it up!")
        } else {
            println!("Sorry, that's not correct. Keep practicing!");
            println!("Correct answer was {}", correct_entry.1);
        }
    }
}

fn loop_for_user_answer(correct_entry: &(&str, &str), wrong_options: &Vec<String>) -> usize {
    let guess_num: usize;

    loop {
        println!("{}", make_question(correct_entry, wrong_options));

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

fn make_question(correct_entry: &(&str, &str), wrong_options_new: &Vec<String>) -> String {
    format!(
        "What is the translation of\n\n{}\n\n{}{}",
        String::from(correct_entry.0),
        wrong_options_new.join("\n"),
        "\n\nPlease enter the corresponding number, between 0 and 3\n",
    )
}

fn randomize() -> ((&'static str, &'static str), Vec<String>) {
    let mut rng = rand::thread_rng();
    let correct_index = rng.gen_range(0..DICTIONARY.len());
    let mut wrong_indexes;
    loop {
        wrong_indexes = seq::index::sample(&mut rng, DICTIONARY.len(), 3).into_vec();
        if !wrong_indexes.contains(&correct_index) {
            break;
        }
    }

    let correct_entry = DICTIONARY[correct_index];

    let wrong_entries = wrong_indexes
        .iter()
        .map(|i| DICTIONARY[*i])
        .collect::<Vec<_>>();

    let mut wrong_options = wrong_entries.iter().map(|i| i.1).collect::<Vec<_>>();
    wrong_options.push(correct_entry.1);

    let wrong_options_indexes =
        seq::index::sample(&mut rng, wrong_options.len(), wrong_options.len()).into_vec();

    let wrong_options_new = wrong_options_indexes
        .iter()
        .enumerate()
        .map(|(i, it)| i.to_string() + ") " + wrong_options[*it])
        .collect::<Vec<_>>();

    (correct_entry, wrong_options_new)
}
