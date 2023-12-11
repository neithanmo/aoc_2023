use day9::part2::process;

const RESULT: i64 = 948;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input1.txt")?;

    let sum = process(&input)?;
    println!("sum: {}", sum);

    assert_eq!(sum, RESULT);

    Ok(())
}
