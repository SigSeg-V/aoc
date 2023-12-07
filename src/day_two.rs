use std::{usize, collections::HashMap};

use phf::phf_map;

pub fn find_valid_games(inp: Box<str>) -> usize {
    static COLOR_COUNT: phf::Map<&'static str, usize> = phf_map! {
        "blue" => 14,
        "green" => 13,
        "red" => 12,
    };

    let mut total = 0;
    'game: for game in inp.split("\n") {
        let mut split_game = game.split(": ");
        let id = split_game.nth(0).unwrap().split(" ").nth(1).unwrap().parse::<usize>().unwrap_or_default();
        let game = split_game.nth(0).unwrap();

        for pulls in game.split("; ") {
            for pull in pulls.split(", ") {
                let mut pull = pull.split(" ");
                let num = pull.nth(0).unwrap().parse::<usize>().unwrap_or_default();
                let color = pull.nth(0).unwrap();

                if let Some(val) = COLOR_COUNT.get(color) {
                    if *val < num {
                        continue 'game;
                    }
                }
            }
        }
        
        total += id;
    };
    
    total
}

pub fn sq_min_games(inp: Box<str>) -> usize {
    inp.split("\n").fold(0, |acc, s| {
        acc + fold_to_power(s)
    })
}

fn fold_to_power(inp: &str) -> usize {
    inp
    .split(": ")
    .nth(1)
    .unwrap()
    .split("; ")
    .fold(HashMap::from([
            ("red", 0),
            ("blue", 0),
            ("green", 0),
        ]), |mut acc, pull| {
        let counts = pull.split(", ")
        .map(|kind| {
          let number = kind.split(" ").nth(0).unwrap().parse::<usize>().unwrap();
          let color = kind.split(" ").nth(1).unwrap();

          (color, number)
        })
        .collect::<HashMap<&str, usize>>(); 

        counts.into_iter().for_each(|(k, v)| {
            if v > *acc.get(k).unwrap() {
                acc.insert(k, v);
            }
        });
        acc
    })
    .into_iter()
    .fold(1, |acc, (_, v)| acc * v)
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::common;

    #[test]
    fn part_1() -> Result<(), Box<dyn Error + 'static>> {
        let data = common::read_file("data/day_two.txt")?;
        assert_eq!(8, super::find_valid_games(data));
        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), Box<dyn Error + 'static>> {
        let data = common::read_file("data/day_two.txt")?;
        assert_eq!(2286, super::sq_min_games(data));

        // assert_eq!(48, super::fold_to_power("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"));
        Ok(())
    }
}