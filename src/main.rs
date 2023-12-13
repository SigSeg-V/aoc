mod common;
mod day_one;
mod day_two;
mod day_three;
mod day_four;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    println!(
        "{}",
        day_four::count_winning_cards(common::read_file("data/day_four_submission.txt")?)
    );
    Ok(())
}
