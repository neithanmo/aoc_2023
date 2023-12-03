use day3::part1::process;

const RESULT: u32 = 525911;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input1.txt")?;

    // missing 755, 467

    let sum = process(&input)?;
    println!("Sum of numbers adjacent to symbols: {}", sum);

    assert_eq!(sum, RESULT);

    Ok(())
}
