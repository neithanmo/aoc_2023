use anyhow::{anyhow, Result};

// One millisecond holded -> One milimiter advanced

pub fn process(input: &str) -> Result<usize> {
    let mut lines = input.lines();
    let time = parse_time(lines.next().ok_or(anyhow!("Input empty"))?)?;

    let distance = parse_distance(lines.next().ok_or(anyhow!("Input empty"))?)?;

    let options = get_options(time, distance).len();

    Ok(options)
}

fn parse_time(line: &str) -> Result<usize> {
    parse_value(line)
}

fn parse_distance(line: &str) -> Result<usize> {
    parse_value(line)
}

fn parse_value(line: &str) -> Result<usize> {
    let (_, numbers) = line.split_once(':').ok_or(anyhow!("Race time inputs"))?;

    let values = numbers.trim().split_ascii_whitespace().collect::<String>();

    if values.is_empty() {
        return Err(anyhow!("Empty race inputs"));
    }

    Ok(values.parse::<usize>()?)
}

// get race time, record distance and
// returns the options we can hold the buttom at the start to win
fn get_options(race_time: usize, record: usize) -> Vec<usize> {
    (0..=race_time)
        .filter(|time| {
            if *time == race_time {
                return false;
            }
            let time_left = race_time - time;
            let distance = time * time_left;
            if distance > record {
                return true;
            }
            false
        })
        .collect::<Vec<usize>>()
}
