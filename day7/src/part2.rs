use anyhow::{anyhow, Result};
use std::fmt;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Copy, Clone, Hash)]
pub enum Card {
    Jack = 1, // jocker the lowest now
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Queen = 11,
    King = 12,
    Ace = 13,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let card_char = match self {
            Card::Jack => 'J',
            Card::Two => '2',
            Card::Three => '3',
            Card::Four => '4',
            Card::Five => '5',
            Card::Six => '6',
            Card::Seven => '7',
            Card::Eight => '8',
            Card::Nine => '9',
            Card::Ten => 'T',
            Card::Queen => 'Q',
            Card::King => 'K',
            Card::Ace => 'A',
        };
        write!(f, "{}", card_char)
    }
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            _ => Err(anyhow!("Invalid card character {}", value)),
        }
    }
}

// being High the lowest in points??
// that would depend on the card
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hand {
    High(CardValues),
    OnePair(CardValues),
    TwoPair(CardValues),
    ThreeOfAKind(CardValues),
    FullHouse(CardValues),
    FourOfAKind(CardValues),
    FiveOfAKind(CardValues),
}

impl Hand {
    fn card_strength(&self) -> u8 {
        match self {
            Hand::FiveOfAKind(_) => 7,
            Hand::FourOfAKind(_) => 6,
            Hand::FullHouse(_) => 5,
            Hand::ThreeOfAKind(_) => 4,
            Hand::TwoPair(_) => 3,
            Hand::OnePair(_) => 2,
            Hand::High(_) => 1,
        }
    }

    fn process_no_jacks(cards: CardValues) -> Self {
        let mut counts = std::collections::HashMap::new();

        // Count the occurrences of each card
        for &card in cards.0.iter() {
            *counts.entry(card).or_insert(0) += 1;
        }

        let mut counts: Vec<_> = counts.into_iter().collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

        match counts.as_slice() {
            [(card, 5)] => Hand::FiveOfAKind(CardValues([*card; 5])),
            [(_, 4), ..] => Hand::FourOfAKind(cards),
            [(_, 3), (_, 2), ..] | [(_, 2), (_, 3), ..] => Hand::FullHouse(cards),
            [(_, 3), ..] => Hand::ThreeOfAKind(cards),
            [(_, 2), (_, 2), ..] => Hand::TwoPair(cards),
            [(_, 2), ..] => Hand::OnePair(cards),
            _ => Hand::High(cards),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.card_strength().cmp(&other.card_strength()) {
            std::cmp::Ordering::Equal => {
                // If hand types are equal, compare the CardValues which
                // internally will look at the first character first, then the following and so on
                // until it founds the higher/lower
                match (self, other) {
                    (Hand::FiveOfAKind(cards1), Hand::FiveOfAKind(cards2))
                    | (Hand::FourOfAKind(cards1), Hand::FourOfAKind(cards2))
                    | (Hand::FullHouse(cards1), Hand::FullHouse(cards2))
                    | (Hand::ThreeOfAKind(cards1), Hand::ThreeOfAKind(cards2))
                    | (Hand::TwoPair(cards1), Hand::TwoPair(cards2))
                    | (Hand::OnePair(cards1), Hand::OnePair(cards2))
                    | (Hand::High(cards1), Hand::High(cards2)) => cards1.partial_cmp(cards2),
                    _ => None,
                }
            }
            other => Some(other),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
#[derive(Debug, Clone, Copy, Eq, Hash)]
pub struct CardValues([Card; 5]);

impl CardValues {
    pub fn num_jacks(&self) -> usize {
        self.0.iter().filter(|c| **c == Card::Jack).count()
    }

    pub fn count_jacks_and_repeated(&self) -> (usize, usize, usize) {
        let mut counts = std::collections::HashMap::new();
        let mut jacks_count = 0;

        for &card in self.0.iter() {
            if card == Card::Jack {
                jacks_count += 1;
            } else {
                *counts.entry(card).or_insert(0) += 1;
            }
        }

        let total_repetitions = counts.values().filter(|&&count| count > 1).sum();
        let unique_sets = counts.values().filter(|&&count| count > 1).count();

        (jacks_count, total_repetitions, unique_sets)
    }
}

impl PartialEq for CardValues {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().zip(other.0.iter()).all(|(a, b)| a == b)
    }
}

impl PartialOrd for CardValues {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0
            .iter()
            .zip(other.0.iter())
            .map(|(a, b)| a.cmp(b))
            .find(|&ordering| ordering != std::cmp::Ordering::Equal)
            .or(Some(std::cmp::Ordering::Equal))
    }
}

impl TryFrom<&str> for CardValues {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        if value.len() > 5 {
            return Err(anyhow!("Invalid card values"));
        }
        let mut cards = [Card::Two; 5];

        for (i, c) in value.chars().enumerate() {
            let card = Card::try_from(c)?;
            cards[i] = card;
        }

        Ok(Self(cards))
    }
}

impl fmt::Display for CardValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let card_strings: String = self.0.iter().map(|card| card.to_string()).collect();
        write!(f, "{}", card_strings)
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Hand::High(cards) => write!(f, "High({})", cards),
            Hand::OnePair(cards) => write!(f, "OnePair({})", cards),
            Hand::TwoPair(cards) => write!(f, "TwoPair({})", cards),
            Hand::ThreeOfAKind(cards) => write!(f, "ThreeOfAKind({})", cards),
            Hand::FullHouse(cards) => write!(f, "FullHouse({})", cards),
            Hand::FourOfAKind(cards) => write!(f, "FourOfAKind({})", cards),
            Hand::FiveOfAKind(cards) => write!(f, "FiveOfAKind({})", cards),
        }
    }
}

impl From<CardValues> for Hand {
    fn from(cards: CardValues) -> Self {
        let (num_jacks, num_repeated, unique_sets) = cards.count_jacks_and_repeated();

        if num_jacks == 0 {
            return Hand::process_no_jacks(cards);
        }

        match (num_jacks, num_repeated, unique_sets) {
            (1, 0, _) => Hand::OnePair(cards),
            (4, 0, _) | (5, 0, _) => Hand::FiveOfAKind(cards),
            (2, 0, _) | (1, 2, _) => Hand::ThreeOfAKind(cards),
            (3, 0, _) | (1, 3, _) | (2, 2, _) => Hand::FourOfAKind(cards),
            // Only case where we need to check for unique_sets
            // if there are two we have a full house.
            // A4JA4 is a FullHouse, unique_sets would be 2
            (1, 4, 2) | (2, 3, 2) => Hand::FullHouse(cards),
            (1, 4, _) | (2, 3, _) | (3, 2, _) => Hand::FiveOfAKind(cards),
            _ => panic!("More cards than allowed!"),
        }
    }
}

pub fn process(input: &str) -> Result<u32> {
    let lines = input.lines();
    let mut hands = lines
        .map(parse_hand)
        .collect::<Result<Vec<(Hand, u32)>>>()?;

    hands.sort_by(|a, b| b.0.cmp(&a.0));

    let num_hands = hands.len();
    // Assign ranks and calculate winnings
    let total_winnings: u32 = hands
        .iter()
        .enumerate() // Enumerate provides the index
        .map(|(index, &(_, bid))| {
            let rank = num_hands - index;
            rank as u32 * bid
        }) // rank + 1 since ranks start from 1
        .sum();

    Ok(total_winnings)
}

pub fn parse_hand(input: &str) -> Result<(Hand, u32)> {
    // T55J5 684
    let mut it = input.trim().split_ascii_whitespace();
    let card_str = it.next().ok_or(anyhow!("Invalid card input"))?;

    let card = CardValues::try_from(card_str)?;
    let bid = it
        .next()
        .ok_or(anyhow!("Invalid bid input"))?
        .parse::<u32>()?;

    Ok((Hand::from(card), bid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_valid_input_part2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = process(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5905);
    }

    #[test]
    fn test_others() {
        let inputs = [
            "KTJJT 2",
            "43J52 4",
            "JJJJ3 100",
            "TAJJA 50",
            "7243A 500",
            "J5A32 52",
        ];
        for input in inputs {
            let result = parse_hand(input);
            assert!(result.is_ok());
        }
    }
}
