use std::collections::{BTreeMap, HashSet};

use anyhow::{anyhow, Result};

pub fn process(input: &str) -> Result<usize> {
    let mut lines = input.lines();

    // get instructions interator from first line
    // we should repeat all instrution forever until we reach our destination.
    let instructions = lines
        .next()
        .ok_or(anyhow!("No instructions"))?
        .chars()
        .map(|c| if c == 'R' { 1usize } else { 0 })
        .collect::<Vec<_>>();

    let map = lines
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                parse_map(line)
            }
        })
        .collect::<BTreeMap<_, _>>();

    let active_nodes = get_initial_active_nodes(&map);

    Ok(compute_steps(active_nodes, &map, &instructions))
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
    active_nodes: HashSet<&'a String>,
    map: &'a BTreeMap<String, [String; 2]>,
    instructions: &[usize],
) -> usize {
    // use lcm, counting cycles.
    // 1. for each node ending with A, get the instruction index
    // that took to the corresponding node ending with Z. XXA -> XXZ
    // 2. get the list of number of instructions for each node to get to the equivalent Z and
    // compute lcm
    let cycles = active_nodes
        .iter()
        .map(|node| {
            let mut visited_nodes = vec![*node];
            let mut current_node = *node;
            instructions
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(num, ins)| {
                    let [left, right] = map.get(current_node).expect("Key must exists");

                    let next_node = if *ins == 0 { left } else { right };
                    if next_node.ends_with('Z') {
                        Some(num + 1)
                    } else {
                        current_node = next_node;
                        visited_nodes.push(next_node);
                        None
                    }
                })
                .expect("Must find a cycle")
        })
        .collect::<Vec<usize>>();
    lcm(&cycles)
}

pub fn get_initial_active_nodes(map: &BTreeMap<String, [String; 2]>) -> HashSet<&String> {
    map.keys()
        .filter(|k| k.ends_with('A'))
        .collect::<HashSet<_>>()
}

// Compute the "Least Common Multiple"
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }

    let a = nums[0];
    let b = lcm(&nums[1..]);

    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
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
