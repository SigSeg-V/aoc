mod common;
mod day_one;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    println!(
        "{}",
        day_one::find_calibration_sum(common::read_file("data/day_one_submission.txt")?)
    );
    Ok(())
}
