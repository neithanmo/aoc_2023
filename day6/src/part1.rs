use anyhow::{anyhow, Result};

pub fn process(input: &str) -> Result<usize> {
    let mut lines = input.lines();
    let times = parse_time(lines.next().ok_or(anyhow!("Input empty"))?)?;

    let distances = parse_distance(lines.next().ok_or(anyhow!("Input empty"))?)?;

    if times.len() != distances.len() {
        return Err(anyhow!("Invalid race input"));
    }

    let options =
        times
            .into_iter()
            .zip(distances.into_iter())
            .fold(1, |mut options, (time, distance)| {
                options *= get_options(time, distance).count();
                options
            });

    Ok(options)
}

fn parse_time(line: &str) -> Result<Vec<u32>> {
    parse_values(line)
}

fn parse_distance(line: &str) -> Result<Vec<u32>> {
    parse_values(line)
}

fn parse_values(line: &str) -> Result<Vec<u32>> {
    let (_, numbers) = line.split_once(':').ok_or(anyhow!("Race time inputs"))?;

    let values = numbers
        .trim()
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    if values.is_empty() {
        return Err(anyhow!("Empty race inputs"));
    }

    Ok(values)
}

// get race time, record distance and
// returns the options we can hold the buttom at the start to win
fn get_options(race_time: u32, record: u32) -> impl Iterator<Item = u32> {
    (0..=race_time).filter(move |time| {
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
}
