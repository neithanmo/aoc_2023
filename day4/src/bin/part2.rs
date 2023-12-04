use day4::part2::process;

const RESULT: usize = 8570000;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input2.txt")?;

    let sum = process(&input)?;
    println!("Total cards: {}", sum);

    assert_eq!(sum, RESULT);

    Ok(())
}
