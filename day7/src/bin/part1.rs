use day7::part1::process;

const RESULT: u32 = 250946742;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input1.txt")?;
    //     let input = "32T3K 765
    // T55J5 684
    // KK677 28
    // KTJJT 220
    // QQQJA 483";

    let points = process(&input)?;
    println!("total_winnings: {}", points);

    assert_eq!(points, RESULT);

    Ok(())
}
