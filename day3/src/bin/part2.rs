use day3::part2::process;

const RESULT: u32 = 75805607;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input1.txt")?;

    let sum = process(&input)?;
    println!("Gear ratio: {}", sum);

    assert_eq!(sum, RESULT);

    Ok(())
}
