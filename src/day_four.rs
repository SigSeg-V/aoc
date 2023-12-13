use std::collections::{HashSet, HashMap};

pub fn count_winning_cards(inp: Box<str>) -> usize {
    let mut total = 0;

    inp.split("\n").for_each(|game| {
        let numbers = game.
        split(": ")
        .nth(1)
        .unwrap()
        .split(" | ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();
        
        // 0 -> winning numbers, 1 -> numbers gotten
        let numbers = (&numbers[0], &numbers[1]);

        let num_set = numbers.0.iter().filter(|e| numbers.1.contains(e)).count();
        
        total += if num_set > 0 {
            2_usize.pow(num_set as u32 - 1)
        } else {
            0
        }
    });

    total
}

pub fn count_number_of_cards(inp: Box<str>) -> usize {
    inp
    .split("\n")
    .enumerate()
    .fold(HashMap::new(), |mut acc: HashMap<usize, usize>, (index, game)| {
        let numbers = game
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" | ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();
        
        // 0 -> winning numbers, 1 -> numbers gotten
        let numbers = (&numbers[0], &numbers[1]);

        let num_set = numbers.0.iter().filter(|e| numbers.1.contains(e)).count();

        ((index + 1)..=(index + num_set)).for_each(|offset| *acc.entry(offset).or_insert(0) +=  *acc.get(&index).unwrap_or(&1));
        
        acc
    })
    .iter()
    .fold(0, |acc, (_, v)| acc + v)
}

mod tests {
    use std::error::Error;

    use crate::common;

    #[test]
    fn part_1() -> Result<(), Box<dyn Error + 'static>> {
        let data = common::read_file("data/day_four.txt")?;
        assert_eq!(13, super::count_winning_cards(data));
        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), Box<dyn Error + 'static>> {
        let data = common::read_file("data/day_four.txt")?;
        assert_eq!(30, super::count_number_of_cards(data));
        Ok(())
    }
}