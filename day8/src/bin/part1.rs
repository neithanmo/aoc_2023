use day8::part1::process;

const RESULT: usize = 21389;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input1.txt")?;

    let points = process(&input)?;
    println!("steps: {}", points);

    assert_eq!(points, RESULT);

    Ok(())
}
