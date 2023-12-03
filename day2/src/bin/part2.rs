use day2::part2::process;

const RESULT: u32 = 67953;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input1.txt")?;

    let sum = process(&input)?;
    println!("sum: {}", sum);

    assert_eq!(sum, RESULT);

    Ok(())
}
