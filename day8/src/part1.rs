use std::collections::BTreeMap;

use anyhow::{anyhow, Result};

pub fn process(input: &str) -> Result<usize> {
    let mut lines = input.lines();

    // get instructions interator from first line
    // we should repeat all instrution forever until we reach our destination.
    let instructions = std::iter::repeat(
        lines
            .next()
            .ok_or(anyhow!("No instructions"))?
            .chars()
            .filter_map(|c| {
                if c == 'R' {
                    Some(1)
                } else if c == 'L' {
                    Some(0)
                } else {
                    None
                }
            }),
    )
    .flatten();

    let map = lines
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                parse_map(line)
            }
        })
        .collect::<BTreeMap<_, _>>();

    println!("num_nodes: {}", map.len());

    Ok(compute_steps("AAA", "ZZZ", &map, instructions))
}

pub fn parse_map(line: &str) -> Option<(String, [String; 2])> {
    let mut parts = line.split(" = ").map(|s| s.trim());
    let key = parts.next()?;
    let mut values = parts
        .next()?
        .trim_start_matches('(')
        .trim_end_matches(')')
        .split(',');
    let value1 = values.next()?.trim().to_owned();
    let value2 = values.next()?.trim().to_owned();

    Some((key.to_owned(), [value1, value2]))
}

pub fn compute_steps<'a>(
    mut start: &'a str,
    end: &'a str,
    map: &'a BTreeMap<String, [String; 2]>,
    instructions: impl Iterator<Item = usize>,
) -> usize {
    let mut num_steps = 0;
    instructions.enumerate().any(|(num, ins)| {
        let [left, right] = map.get(start).unwrap();
        if ins == 0 {
            start = left;
        } else {
            start = right;
        }
        num_steps = num;

        start == end
    });

    num_steps + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_valid_input() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let result = process(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }
}
