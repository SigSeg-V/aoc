use phf::phf_map;

pub fn find_invalid_games(inp: Box<str>) -> usize {
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

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::common;

    #[test]
    fn max_elf() -> Result<(), Box<dyn Error + 'static>> {
        let data = common::read_file("data/day_two.txt")?;
        assert_eq!(8, super::find_invalid_games(data));
        Ok(())
    }
}