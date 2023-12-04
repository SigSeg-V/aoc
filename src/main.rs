mod common;
mod day_one;
mod day_two;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    println!(
        "{}",
        day_two::find_invalid_games(common::read_file("data/day_two_submission.txt")?)
    );
    Ok(())
}
