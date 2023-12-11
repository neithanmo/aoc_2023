use std::collections::VecDeque;

use anyhow::Result;

pub fn process(input: &str) -> Result<i64> {
    let mut queue = VecDeque::new();
    let sum = input
        .lines()
        .map(|line| {
            make_queue(line, &mut queue);
            let value = compute_item(&mut queue);
            debug_assert!(queue.is_empty());
            value
        })
        .sum();

    Ok(sum)
}

pub fn make_queue(line: &str, queue: &mut VecDeque<i64>) {
    let mut vec: Vec<i64> = line
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().expect("Invalid number"))
        .collect::<Vec<i64>>();

    if vec.is_empty() {
        return;
    }

    queue.push_front(vec[0]);

    loop {
        vec = vec
            .iter()
            .zip(vec.iter().skip(1))
            .map(|(prev, next)| next - prev)
            .collect::<Vec<i64>>();

        if vec.iter().all(|&n| n == 0) {
            break;
        }

        queue.push_front(vec[0]);
    }
}
pub fn compute_item(queue: &mut VecDeque<i64>) -> i64 {
    let mut num = 0;
    while let Some(last) = queue.pop_front() {
        num = last - num;
    }

    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_valid_input_part2() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result = process(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }
}
