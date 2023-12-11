use std::rc::Rc;

pub struct Point(usize, usize);

pub fn get_part_number(inp: Rc<str>) -> usize {
    let mut total: usize = 0;

    inp.split("\n").enumerate().for_each(|(line_index, line)| {
        // to flag if we count the number
        let mut is_part_number = false;
        // container to collect chars for parsing
        let mut number = "".to_string();

        line.chars().enumerate().for_each(|(ch_index, ch)| {
            match ch {
                '0'..='9' => {
                    number.push(ch);
                    if check_for_symbols(Rc::clone(&inp), Point(line_index, ch_index)) {
                        
                        is_part_number = true;
                    }
                },
                _ => {
                    if is_part_number {
                        // add number to the total
                        total += number.parse::<usize>().unwrap();
                        
                    }
                    number.clear();
                    is_part_number = false;
                }   
            }
        });
        // potential last number not added so need to do that here
        if number.len() > 0 {
            total += number.parse::<usize>().unwrap();
        }
        
    });

    total
}

fn check_for_symbols(inp: Rc<str>, current_position: Point) -> bool {
    for row in -1..=1 {
        let pos_row = current_position.0 as isize + row;
        if pos_row < 0 {
            continue
        } 

        for col in -1..=1 {
            let pos_col = current_position.1 as isize + col;
            if pos_col < 0 {
                continue
            } 

            if let Some(ln) = inp.split("\n").nth(pos_row as usize) {
                if let Some(c) = ln.chars().into_iter().nth(pos_col as usize) {
                    if !c.is_ascii_digit() && c != '.' {
                        return true
                    }
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::common;

    #[test]
    fn part_1() -> Result<(), Box<dyn Error + 'static>> {
        let data = common::read_file_rc("data/day_three.txt")?;
        assert_eq!(4361, super::get_part_number(data));
        Ok(())
    }

    // #[test]
    // fn part_2() -> Result<(), Box<dyn Error + 'static>> {
    //     let data = common::read_file("data/day_two.txt")?;
    //     assert_eq!(2286, super::sq_min_games(data));

    //     // assert_eq!(48, super::fold_to_power("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"));
    //     Ok(())
    // }
}