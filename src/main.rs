#![allow(dead_code)]

#[derive(Debug, PartialEq)]
enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}

impl Card {
    fn to_usize(&self) -> Option<usize> {
        use Card::*;
        match self {
            Ace => None,
            Two => Some(2),
            Three => Some(3),
            Four => Some(4),
            Five => Some(5),
            Six => Some(6),
            Seven => Some(7),
            Eight => Some(8),
            Nine => Some(9),
            Jack => Some(10),
            Queen => Some(10),
            King => Some(10),
        }
    }
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Hand { cards: vec![] }
    }

    fn hit(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> usize {
        let sum = self
            .cards
            .iter()
            .map(Card::to_usize)
            .fold(0, |sum, i| sum + i.unwrap_or(0));

        let aces = self
            .cards
            .iter()
            .filter(|card| matches!(card, Card::Ace))
            .count();

        // possible ace values combination
        (aces..=aces * 11)
            .step_by(10)
            .filter_map(|x| if x + sum <= 21 { Some(x + sum) } else { None })
            .max()
            .unwrap_or(sum)
    }

    fn is_loosing_hand(&self) -> bool {
        self.value() > 21
    }
}

fn main() {
    let mut hand = Hand::new();
    hand.hit(Card::King);
    hand.hit(Card::Ace);
}

#[test]
fn empty_hand() {
    let hand = Hand::new();

    assert_eq!(hand.value(), 0);
}

#[test]
fn strong_hand() {
    let mut hand = Hand::new();
    hand.hit(Card::Queen);
    hand.hit(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn risky_hand() {
    let mut hand = Hand::new();
    hand.hit(Card::King);
    hand.hit(Card::Queen);
    hand.hit(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn oops() {
    let mut hand = Hand::new();
    hand.hit(Card::King);
    hand.hit(Card::Seven);
    hand.hit(Card::Five);

    assert!(hand.is_loosing_hand());
    assert_eq!(hand.value(), 22);
}

#[test]
fn weird_hand() {
    let mut hand = Hand::new();
    hand.hit(Card::Ace);
    hand.hit(Card::Ace);

    assert_eq!(hand.value(), 12);
}
