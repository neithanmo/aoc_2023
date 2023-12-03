use anyhow::Result;

// example imput:
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
//
// constraints:
// the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
// Game 1:
const GAME_ID_INDEX: usize = 5;

const RED_LIMIT: u32 = 12;
const GREEN_LIMIT: u32 = 13;
const BLUE_LIMIT: u32 = 14;

pub fn process(input: &str) -> Result<u32> {
    let sum = input
        .lines()
        .filter_map(|line| {
            let mut game_id = line.split(':');
            let game_id = game_id.next()?;
            let game_id = game_id.get(GAME_ID_INDEX..)?.trim().parse::<u32>().ok()?;
            // now get substring containing the data
            let (_, game_data) = line.split_once(':')?;
            if process_impl(game_data) {
                Some(game_id)
            } else {
                None
            }
        })
        .sum();

    Ok(sum)
}

pub fn process_impl(input: &str) -> bool {
    // ( 3 blue, 4 red ) ( 1 red, 2 green, 6 blue ) ( 2 green )
    for game in input.split(';') {
        // (3blue,4red) remove whitespaces
        let game = game.split(',');
        for set in game {
            let set = set.trim();
            let Some( (num, color) ) = set.split_once(' ') else {
                return false;
            };
            let count = num.parse::<u32>().unwrap_or_default();

            if is_count_above_limit(count, color) {
                return false;
            }
        }
    }
    true
}

fn is_count_above_limit(count: u32, color: &str) -> bool {
    (color.contains("blue") && count > BLUE_LIMIT)
        || (color.contains("red") && count > RED_LIMIT)
        || (color.contains("green") && count > GREEN_LIMIT)
}
