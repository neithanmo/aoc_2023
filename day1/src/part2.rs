use anyhow::Result;

pub fn process(input: &str) -> Result<u32> {
    Ok(input.lines().filter_map(process_impl).sum())
}

fn process_impl(input: &str) -> Option<u32> {
    let mut it = (0..input.len()).filter_map(|index| {
        let reduced = &input[index..];
        let result = if reduced.starts_with("one") {
            '1'
        } else if reduced.starts_with("two") {
            '2'
        } else if reduced.starts_with("three") {
            '3'
        } else if reduced.starts_with("four") {
            '4'
        } else if reduced.starts_with("five") {
            '5'
        } else if reduced.starts_with("six") {
            '6'
        } else if reduced.starts_with("seven") {
            '7'
        } else if reduced.starts_with("eight") {
            '8'
        } else if reduced.starts_with("nine") {
            '9'
        } else {
            reduced.chars().next().expect("no more data?")
        };
        result.to_digit(10)
    });

    let first = it.next()?;
    let last = it.last().unwrap_or(first);

    format!("{}{}", first, last).parse::<u32>().ok()
}
