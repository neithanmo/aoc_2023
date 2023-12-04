use std::collections::HashMap;

use anyhow::Result;

pub fn process(input: &str) -> Result<usize> {
    let sum = input
        .lines()
        .filter_map(|line| {
            let (winners, numbers) = parse_numbers(line)?;
            let winners = get_winners(&winners, &numbers);
            let points = get_points(winners.len());
            Some(points)
        })
        .sum();
    Ok(sum)
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

fn get_points(winners_len: usize) -> usize {
    if winners_len > 0 {
        1 << (winners_len - 1) // Use bitwise shift to double the points
    } else {
        0
    }
}
