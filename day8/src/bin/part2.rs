use day8::part2::process;

const RESULT: usize = 251824095;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input1.txt")?;

    let steps = process(&input)?;
    println!("steps: {}", steps);

    assert_eq!(steps, RESULT);

    Ok(())
}
