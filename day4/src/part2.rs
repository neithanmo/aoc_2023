use std::collections::HashMap;

use anyhow::Result;

pub fn process(input: &str) -> Result<usize, anyhow::Error> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut card_copies = HashMap::new();

    for (i, _) in lines.iter().enumerate() {
        card_copies.entry(i + 1).or_insert(1);
    }

    for (i, line) in lines.iter().enumerate() {
        let card_id = i + 1;

        let (winners, numbers) =
            parse_numbers(line).ok_or(anyhow::Error::msg("Failed parsing numbers"))?;
        let val = get_winners(&winners, &numbers).len();

        let current_copies = *card_copies.get(&card_id).unwrap_or(&0);
        for j in 1..=val {
            let next_card_id = card_id + j;
            *card_copies.entry(next_card_id).or_insert(0) += current_copies;
        }
    }

    let total_cards = card_copies.values().sum();
    Ok(total_cards)
}

fn parse_numbers(input: &str) -> Option<(Vec<usize>, Vec<usize>)> {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    let (winners, numbers) = input
        .split_once(':')
        .and_then(|(_, numbers)| numbers.trim().split_once('|'))?;

    let winners = winners
        .trim()
        .split_ascii_whitespace()
        .map(|v| v.trim().parse::<usize>().ok())
        .collect::<Option<Vec<usize>>>()?;

    let numbers = numbers
        .trim()
        .split_ascii_whitespace()
        .map(|v| v.trim().parse::<usize>().ok())
        .collect::<Option<Vec<usize>>>()?;

    Some((winners, numbers))
}

fn get_winners(winners: &[usize], numbers: &[usize]) -> Vec<usize> {
    let winners_map = winners
        .iter()
        .enumerate()
        .map(|(i, number)| (*number, i))
        .collect::<HashMap<usize, usize>>();

    numbers
        .iter()
        .filter_map(|v| winners_map.get(v).copied())
        .collect()
}
