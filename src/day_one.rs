pub fn find_calibration_sum(inp: Box<str>) -> usize {
    let mut total = 0;

    for s in inp.split("\n").into_iter() {
        let digits_only = s.chars()
        .into_iter()
        .filter(|x| x.is_digit(10))
        .collect::<String>();

        let mut calibration_string = "".to_string();
        let last = digits_only.chars().into_iter().last().unwrap_or_default();
        let first = digits_only.chars().into_iter().nth(0).unwrap_or(last);
        calibration_string.push(first);
        calibration_string.push(last);
        total += calibration_string.parse::<usize>().unwrap_or_default();
    }

    total
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::common;

    #[test]
    fn max_elf() -> Result<(), Box<dyn Error + 'static>> {
        let data = common::read_file("data/day_one.txt")?;
        assert_eq!(142, super::find_calibration_sum(data));
        Ok(())
    }
}
