use std::collections::{BTreeMap, HashSet};

use anyhow::{anyhow, Result};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;

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

    let active_nodes = get_initial_active_nodes(&map);

    Ok(compute_steps(active_nodes, &map, instructions))
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
    mut active_nodes: HashSet<&'a String>,
    map: &'a BTreeMap<String, [String; 2]>,
    instructions: impl Iterator<Item = usize>,
) -> usize {
    let mut num_steps = 0;

    // 1. get each active node and apply instructions.
    // 2. put the outputs according to instructions in the next_active_nodes set.
    // 3. check if that set contains only "end_with_z" nodes. if so break we are done.
    // 4. otherwise repeat
    instructions.enumerate().any(|(num, ins)| {
        num_steps = num;

        active_nodes = active_nodes
            .par_iter()
            .map(|node| {
                let [left, right] = map.get(*node).unwrap();
                // next_active_nodes.insert(if ins == 0 { left } else { right });
                if ins == 0 {
                    left
                } else {
                    right
                }
            })
            .collect::<HashSet<_>>();

        active_nodes.iter().all(|n| n.ends_with('Z'))
    });

    num_steps + 1
}

pub fn get_initial_active_nodes(map: &BTreeMap<String, [String; 2]>) -> HashSet<&String> {
    map.keys()
        .filter(|k| k.ends_with('A'))
        .collect::<HashSet<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_valid_input_part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let result = process(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 6);
    }
}
