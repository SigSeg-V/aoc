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


fn find_num_in_string(inp: &str) -> Option<char> {
    static NUMBERS: [&'static str; 10] = ["zero", "one", "two", "three", "four", "five","six","seven","eight","nine"];
    
    for (index, num) in NUMBERS.into_iter().enumerate() {
        if num.len() > inp.len() {
            continue;
        }

        if inp.contains(num) {
            return char::from_digit(index as u32, 10)
        }
    }
    None
}

pub fn find_calibration_sum_with_letters(inp: Box<str>) -> usize {
    let mut total = 0;

    for s in inp.split("\n").into_iter() {
        // match digit or character, then figure out if valid character slice
        let mut first = '\0';
        let mut last = '\0';

        // forward loop
        for (index, c) in s.char_indices().into_iter() {
            match c.is_digit(10) {
                true => {
                    first = c;
                    break;
                },
                false => {
                    first = find_num_in_string(&s[0..=index]).unwrap_or('\0');
                    if first != '\0' {
                        break;
                    }
                },
            }

        }

        // reverse loop
        for (index, c) in s.char_indices().into_iter().rev() {
            match c.is_digit(10) {
                true => {
                    last = c;
                    break;
                },
                false => {
                    last = find_num_in_string(&s[index..].chars().collect::<String>()).unwrap_or('\0');
                    if last != '\0' {
                        break;
                    }
                },
            }

        }

        // some error handling
        if first == '\0' {
            first = '0';
        }
        if last == '\0' {
            last = first;
        }

        let mut calibration_string = "".to_string();
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
        assert_eq!(281, super::find_calibration_sum_with_letters(data));
        Ok(())
    }
}
