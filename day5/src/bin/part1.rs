use day5::part1::process;

const RESULT: usize = 175622908;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input1.txt")?;

    let lowest = process(&input)?;
    println!("lowest: {}", lowest);

    assert_eq!(lowest, RESULT);

    Ok(())
}
