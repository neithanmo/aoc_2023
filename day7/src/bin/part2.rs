use day7::part2::process;

const RESULT: u32 = 251824095;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input1.txt")?;

    let points = process(&input)?;
    println!("points: {}", points);

    assert_eq!(points, RESULT);

    Ok(())
}
