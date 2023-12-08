use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Copy, Clone, Hash)]
pub enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
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
            _ => Err(anyhow!("Invalid card character")),
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

impl From<CardValues> for Hand {
    fn from(cards: CardValues) -> Self {
        let mut counts = std::collections::HashMap::new();

        // Count the occurrences of each card
        for &card in cards.0.iter() {
            *counts.entry(card).or_insert(0) += 1;
        }

        let mut counts: Vec<_> = counts.into_iter().collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

        // AAAAA -> Five
        // AA8AA -> Four
        // 23332 -> Full house
        // TTT98 -> Three
        // 23432 -> two
        // A23A4 -> one
        // 23456 -> High
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
    let card = CardValues::try_from(it.next().ok_or(anyhow!("Invalid card input"))?)?;
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
    fn test_process_valid_input() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = process(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 6440);
    }

    #[test]
    fn test_process_empty_input() {
        let input = "";
        let result = process(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_process_invalid_input() {
        let input = "123";
        let result = process(input);
        assert!(result.is_err());
    }
}
