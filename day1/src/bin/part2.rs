use anyhow::Result;
use day1::part2::process;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input2.txt")?;

    let sum = process(&input)?;
    println!("sum: {}", sum);

    Ok(())
}
