use std::collections::{BTreeMap, HashSet};

use anyhow::Result;

#[derive(Debug)]
pub enum Value {
    Empty,
    Symbol(char),
    Number(u32),
}

pub fn process(input: &str) -> Result<u32> {
    // sum part numbers only if they are near a symbol
    // enumerate would provide the row index(ID)
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(row_id, line)| {
            line.chars().enumerate().map(move |(column_id, ch)| {
                let value = match ch {
                    '.' => Value::Empty,
                    c if ch.is_ascii_digit() => Value::Number(c.to_digit(10).expect("is digit!")),
                    _ => Value::Symbol(ch),
                };
                ((row_id, column_id), value)
            })
        })
        .collect::<BTreeMap<(usize, usize), Value>>();

    let numbers = map_to_numbers(&map);

    let gear_ratios = find_gears(&map, &numbers);
    let sum_of_gear_ratios: u32 = gear_ratios.iter().sum();

    Ok(sum_of_gear_ratios)
}

fn find_gears(
    map: &BTreeMap<(usize, usize), Value>,
    numbers: &BTreeMap<(usize, usize), (usize, u32)>,
) -> Vec<u32> {
    let mut gear_ratios = Vec::new();

    for ((row, column), value) in map {
        if let Value::Symbol('*') = value {
            let adjacent_numbers = find_adjacent_numbers(map, numbers, *row, *column);
            if adjacent_numbers.len() == 2 {
                let gear_ratio = adjacent_numbers.iter().product();
                gear_ratios.push(gear_ratio);
            }
        }
    }

    gear_ratios
}

fn find_adjacent_numbers(
    map: &BTreeMap<(usize, usize), Value>,
    numbers: &BTreeMap<(usize, usize), (usize, u32)>,
    row: usize,
    column: usize,
) -> Vec<u32> {
    let adjacent_positions = [
        // above
        (row.wrapping_sub(1), column),
        (row + 1, column),
        // left/right
        (row, column.wrapping_sub(1)),
        (row, column + 1),
        // diagonals above left/right
        (row.wrapping_sub(1), column.wrapping_sub(1)),
        (row.wrapping_sub(1), column + 1),
        // diagonals below left/right
        (row + 1, column.wrapping_sub(1)),
        (row + 1, column + 1),
    ];

    let mut found_numbers = HashSet::new();
    for &(r, c) in &adjacent_positions {
        if let Some(Value::Number(..)) = map.get(&(r, c)) {
            // Find the start of the number
            let start_col = backtrack_to_start_of_number(map, r, c);
            if let Some((_, num)) = numbers.get(&(r, start_col)) {
                found_numbers.insert(*num);
            }
        }
    }

    found_numbers.into_iter().collect()
}

// get the first digit of a number starting at row and col
fn backtrack_to_start_of_number(
    map: &BTreeMap<(usize, usize), Value>,
    row: usize,
    mut col: usize,
) -> usize {
    while let Some(Value::Number(_)) = map.get(&(row, col.wrapping_sub(1))) {
        col = col.wrapping_sub(1);
    }
    col
}

fn map_to_numbers(map: &BTreeMap<(usize, usize), Value>) -> BTreeMap<(usize, usize), (usize, u32)> {
    // x,y, number_len(number of digits)
    let mut numbers: Vec<((usize, usize, usize), u32)> = vec![];
    let mut visited = std::collections::HashSet::new();

    for ((row, column), value) in map.iter() {
        if let Value::Number(digit) = value {
            if visited.insert((*row, *column)) {
                let mut number = *digit;
                let current_row = *row;
                let mut current_column = *column;
                let mut num_digits = 0;

                // Check for horizontal number
                while let Some(Value::Number(next_digit)) =
                    map.get(&(current_row, current_column + 1))
                {
                    if !visited.insert((current_row, (current_column + 1))) {
                        break;
                    }
                    number = number * 10 + next_digit;
                    current_column += 1;
                    num_digits += 1;
                }

                numbers.push(((*row, *column, num_digits + 1), number));
            }
        }
    }

    numbers
        .into_iter()
        .map(|((row, column, digits), number)| ((row, column), (digits, number)))
        .collect::<BTreeMap<(usize, usize), (usize, u32)>>()
}
