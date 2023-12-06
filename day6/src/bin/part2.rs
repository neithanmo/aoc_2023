use day6::part2::process;

const RESULT: usize = 35865985;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input1.txt")?;

    let ways = process(&input)?;
    println!("ways: {}", ways);

    assert_eq!(ways, RESULT);

    Ok(())
}
