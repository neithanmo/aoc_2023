use anyhow::Result;
use day1::part1::process;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input1.txt")?;

    let sum = process(&input)?;
    println!("sum: {}", sum);

    Ok(())
}
