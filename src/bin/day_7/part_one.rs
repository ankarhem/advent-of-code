use anyhow::{anyhow, Result};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl std::str::FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let card = match s {
            "2" => Card::Two,
            "3" => Card::Three,
            "4" => Card::Four,
            "5" => Card::Five,
            "6" => Card::Six,
            "7" => Card::Seven,
            "8" => Card::Eight,
            "9" => Card::Nine,
            "T" => Card::T,
            "J" => Card::J,
            "Q" => Card::Q,
            "K" => Card::K,
            "A" => Card::A,
            _ => Err(anyhow!("Invalid card: {s}"))?,
        };

        Ok(card)
    }
}

#[derive(Debug)]
struct Bid(u32);

impl std::str::FromStr for Bid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bid = s.parse::<u32>()?;
        Ok(Bid(bid))
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: Bid,
}

impl Hand {
    fn hand_type<'a>(&'a self) -> HandType<'a> {
        let mut counts = std::collections::HashMap::new();
        for card in &self.cards {
            *counts.entry(card).or_insert(0) += 1;
        }

        let mut counts = counts.into_iter().collect::<Vec<_>>();
        counts.sort_by(|(_, count_a), (_, count_b)| count_b.cmp(count_a));

        let (card, count) = counts[0];
        match count {
            5 => HandType::FiveOfAKind(card),
            4 => HandType::FourOfAKind(card),
            3 => {
                if counts.len() == 2 {
                    HandType::FullHouse(card, counts[1].0)
                } else {
                    HandType::ThreeOfAKind(card)
                }
            }
            2 => {
                if counts.len() == 3 {
                    // AABBC = 2 pairs
                    HandType::TwoPair(card, counts[1].0)
                } else {
                    HandType::OnePair(card)
                }
            }
            1 => HandType::HighCard(card),
            _ => unreachable!("Invalid hand"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Copy, Clone)]
enum HandType<'a> {
    HighCard(&'a Card),
    OnePair(&'a Card),
    TwoPair(&'a Card, &'a Card),
    ThreeOfAKind(&'a Card),
    FullHouse(&'a Card, &'a Card),
    FourOfAKind(&'a Card),
    FiveOfAKind(&'a Card),
}

impl From<HandType<'_>> for u8 {
    fn from(hand_type: HandType) -> Self {
        match hand_type {
            HandType::HighCard(_) => 0,
            HandType::OnePair(_) => 1,
            HandType::TwoPair(_, _) => 2,
            HandType::ThreeOfAKind(_) => 3,
            HandType::FullHouse(_, _) => 4,
            HandType::FourOfAKind(_) => 5,
            HandType::FiveOfAKind(_) => 6,
        }
    }
}

impl std::cmp::Ord for HandType<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_num = u8::from(*self);
        let other_num = u8::from(*other);
        self_num.cmp(&other_num)
    }
}

impl std::str::FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s
            .split_whitespace()
            .next_tuple()
            .ok_or(anyhow!("Invalid hand format"))?;

        let cards = cards
            .chars()
            .map(|c| c.to_string().parse::<Card>())
            .collect::<Result<Vec<Card>>>()?;

        if cards.len() != 5 {
            return Err(anyhow!("Invalid number of cards"));
        }

        let bid = bid.parse::<Bid>()?;

        Ok(Hand { cards, bid })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<Hand>())
        .collect::<Result<Vec<Hand>>>()
        .ok()?;

    lines.sort_by(|hand_one, hand_two| {
        let hand_one_type = hand_one.hand_type();
        let hand_two_type = hand_two.hand_type();

        match hand_one_type.cmp(&hand_two_type) {
            std::cmp::Ordering::Equal => {
                let hand_one_cards = hand_one.cards.iter();
                let hand_two_cards = hand_two.cards.iter();

                let mismatch = hand_one_cards
                    .zip(hand_two_cards)
                    .find(|(card_one, card_two)| card_one != card_two);

                match mismatch {
                    Some((card_one, card_two)) => card_one.cmp(card_two),
                    None => std::cmp::Ordering::Equal,
                }
            }
            ordering => ordering,
        }
    });

    let total_winnings = lines.iter().enumerate().fold(0, |acc, (index, hand)| {
        let rank = index + 1;
        let winnings = hand.bid.0 * rank as u32;
        acc + winnings
    });

    Some(total_winnings)
}

#[cfg(test)]
mod tests {
    use super::super::super::DAY;
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(6440, result.unwrap());
    }
}
