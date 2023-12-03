use anyhow::Result;

// in each game you played, what is the fewest number of cubes of each color that could have been in
// the bag to make the game possible?
pub fn process(input: &str) -> Result<u32> {
    let sum = input
        .lines()
        .filter_map(|line| {
            let (_, game_data) = line.split_once(':')?;
            process_impl(game_data)
        })
        .sum();

    Ok(sum)
}

// need to find the highest number for each color among all sets per game.
pub fn process_impl(input: &str) -> Option<u32> {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    // ( 3 blue, 4 red ) ( 1 red, 2 green, 6 blue ) ( 2 green )
    for game in input.split(';') {
        // (3blue,4red) remove whitespaces
        let game = game.split(',');

        for set in game {
            let set = set.trim();
            let (num, color) = set.split_once(' ')?;
            let count = num.parse::<u32>().ok()?;

            match color {
                "red" => {
                    if count > red {
                        red = count;
                    }
                }
                "green" => {
                    if count > green {
                        green = count;
                    }
                }
                "blue" => {
                    if count > blue {
                        blue = count;
                    }
                }
                _ => return None,
            }
        }
    }
    Some(red * green * blue)
}
