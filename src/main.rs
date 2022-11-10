use rand::{seq, Rng};

fn main() {
    let dictionary = [
        ("str a", "բառ ա"),
        ("str b", "բառ բ"),
        ("str g", "բառ գ"),
        ("str d", "բառ դ"),
    ];

    let mut rng = rand::thread_rng();
    let correct_index = rng.gen_range(0..dictionary.len());
    let mut wrong_indexes;
    loop {
        wrong_indexes = seq::index::sample(&mut rng, dictionary.len(), 3).into_vec();
        if !wrong_indexes.contains(&correct_index) {
            break;
        }
    }

    let correct_entry = dictionary[correct_index];

    let wrong_entries = wrong_indexes
        .iter()
        .map(|i| dictionary[*i])
        .collect::<Vec<_>>();

    let mut wrong_options = wrong_entries.iter().map(|i| i.1).collect::<Vec<_>>();
    wrong_options.push(correct_entry.1);

    let mut wrong_options_indexes;
    loop {
        wrong_options_indexes =
            seq::index::sample(&mut rng, wrong_options.len(), wrong_options.len()).into_vec();
        if !wrong_indexes.contains(&correct_index) {
            break;
        }
    }

    let wrong_options_new = wrong_options_indexes
        .iter()
        .enumerate()
        .map(|(i, it)| i.to_string() + ") " + wrong_options[*it])
        .collect::<Vec<_>>();

    let question = format!(
        "What is the translation of\n\n{}\n\n{}",
        String::from(correct_entry.0),
        &wrong_options_new.join("\n")
    );

    println!("{}", question);
}
