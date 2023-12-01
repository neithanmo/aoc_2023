use anyhow::Result;

pub fn process(input: &str) -> Result<u32> {
    // 1. split lines
    // 2. look at each one and pass only numeric values.
    // 3. convert it to an iterator
    // 4. get first.
    // 5. get last.
    // 6. match over last for checking corner case.
    // return number.sum
    let sum = input
        .lines()
        .inspect(|line| {
            dbg!(line);
        })
        .filter_map(|line| {
            let mut num_it = line.chars().filter(|c| c.is_ascii_digit());
            let first = num_it.next();
            if let Some(first) = first {
                let last = num_it.last().unwrap_or(first);
                format!("{}{}", first, last).parse::<u32>().ok()
            } else {
                None
            }
        })
        .sum();

    Ok(sum)
}
