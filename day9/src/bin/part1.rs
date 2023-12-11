use day9::part1::process;

const RESULT: i64 = 21389;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input1.txt")?;

    // 1 3 6 10 15 21
    // 10 13 16 21 30 45";

    let sum = process(&input)?;
    println!("sum: {}", sum);

    assert_eq!(sum, RESULT);

    Ok(())
}
