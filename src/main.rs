mod utils;

fn main() {
    let data = utils::read_tests();
    utils::do_multiple_choice(data);
}
