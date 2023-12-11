mod common;
mod day_one;
mod day_two;
mod day_three;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    println!(
        "{}",
        day_three::get_part_number(common::read_file_rc("data/day_three_submission.txt")?)
    );
    Ok(())
}
