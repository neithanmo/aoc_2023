use std::collections::BTreeMap;

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
                    c if ch.is_ascii_digit() => {
                        Value::Number(c.to_digit(10).expect("is digit!") as u32)
                    }
                    _ => Value::Symbol(ch),
                };
                ((row_id, column_id), value)
            })
        })
        .collect::<BTreeMap<(usize, usize), Value>>();

    let numbers = map_to_numbers(&map);

    // let sum = map.into_iter().map(|((row, column), value)| {}).sum();
    let mut sum = 0;
    for &((row, column, digits), number) in &numbers {
        if is_adjacent_to_symbol(&map, row, column, digits) {
            sum += number;
        }
    }

    Ok(sum)
}

fn map_to_numbers(map: &BTreeMap<(usize, usize), Value>) -> Vec<((usize, usize, usize), u32)> {
    let mut numbers: Vec<((usize, usize, usize), u32)> = vec![];
    let mut visited = std::collections::HashSet::new();

    for ((row, column), value) in map.iter() {
        if let Value::Number(digit) = value {
            if visited.insert((*row, *column)) {
                let mut number = *digit;
                let mut current_column = *column;
                let mut num_digits = 0;

                // Check for horizontal number
                while let Some(Value::Number(next_digit)) = map.get(&(*row, current_column + 1)) {
                    if !visited.insert((*row, (current_column + 1))) {
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
}

fn is_adjacent_to_symbol(
    map: &std::collections::BTreeMap<(usize, usize), Value>,
    row: usize,
    col: usize,
    digits: usize,
) -> bool {
    let adjacent_positions = [
        // Check above and below each digit
        (row.wrapping_sub(1), col, digits),
        (row + 1, col, digits),
        // Diagonal: Check above/bellow and before each digit
        (row.wrapping_sub(1), col.wrapping_sub(1), digits),
        (row + 1, col.wrapping_sub(1), digits),
        // Diagonal: check above/bellow and after each digit
        (row.wrapping_sub(1), col + 1, digits),
        (row + 1, col + 1, digits),
        // Check left and right of the number
        (row, col.wrapping_sub(1), 1),
        (row, col + digits, 1),
    ];

    adjacent_positions.iter().any(|&(r, c, d)| {
        !(0..d).all(|offset| {
            let c = c.checked_add(offset).unwrap_or(c);
            !matches!(map.get(&(r, c)), Some(Value::Symbol(_)))
        })
    })
}
