use std::collections::VecDeque;

use anyhow::Result;

// 0   3   6   9  12  15   (B)
//  3   3   3   3   3   (A)
//    0   0   0   0   (0)
// compute A and B and sum them up.
// sounds like a stack you get last 0, get previous
// item from the stack and them.
// A needs to be the result of increasing 3 (the value to its left) by 0 (the value below it);
// this means A must be 3
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

    // push last value into the queue
    queue.push_back(vec[vec.len() - 1]);

    // now compute new sequences
    // 0   3   6   9  12  15
    // (0, 3), (3, 6), (6, 9), (9, 12), (12, 15)
    // 3,       3,      3,      3,       3,
    loop {
        vec = vec
            .iter()
            .zip(vec.iter().skip(1))
            .map(|(prev, next)| next - prev)
            .collect();
        let item = vec[vec.len() - 1];

        if vec.iter().all(|n| *n == 0) {
            break;
        }

        // push value into the queue
        queue.push_back(item);
    }
}

pub fn compute_item(queue: &mut VecDeque<i64>) -> i64 {
    let mut num = 0;
    while let Some(last) = queue.pop_back() {
        num = num + last;
    }

    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_valid_input() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result = process(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 114);
    }

    //     #[test]
    //     fn test_process_valid2() {
    //         let input = "-7 -12 -15 -4 52 210 556 1204 2291 3968 6387 9684 13958 19246 25494 32524 39997 47372 53861 58380 59496
    // 26 33 38 39 24 -37 -200 -556 -1249 -2520 -4807 -8956 -16641 -31137 -58573 -109554 -200280 -350497 -572000 -834815 -986998
    // 10 8 -4 -34 -93 -195 -357 -599 -944 -1418 -2050 -2872 -3919 -5229 -6843 -8805 -11162 -13964 -17264 -21118 -25585
    // 21 39 67 108 165 241 339 462 613 795 1011 1264 1557 1893 2275 2706 3189 3727 4323 4980 5701
    // 3 1 1 3 7 13 21 31 43 57 73 91 111 133 157 183 211 241 273 307 343
    // 18 31 60 117 214 379 701 1432 3189 7329 16639 36626 77951 160976 324057 638253 1232796 2338487 4361058 8005003 14482876
    // 15 31 52 76 95 91 35 -105 -328 -491 -43 2534 10778 32073 80618 182013 380053 746451 1394354 2496672 4310409
    // 5 18 50 114 230 437 813 1520 2903 5696 11440 23315 47743 97340 196095 388084 751768 1423532 2638127 4803908 8656019
    // 10 26 46 69 94 120 146 171 194 214 230 241 246 244 234 215 186 146 94 29 -50
    // 8 33 84 174 316 523 808 1184 1664 2261 2988 3858 4884 6079 7456 9028 10808 12809 15044 17526 20268
    // 10 16 22 28 34 40 46 52 58 64 70 76 82 88 94 100 106 112 118 124 130
    // ";
    //         let result = process(input).unwrap();
    //         assert_eq!(result, 18);
    //     }
}
