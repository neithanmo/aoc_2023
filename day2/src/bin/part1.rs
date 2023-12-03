use day2::part1::process;

const RESULT: u32 = 2278;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input1.txt")?;

    let sum = process(&input)?;
    println!("sum: {}", sum);

    assert_eq!(sum, RESULT);

    Ok(())
}
