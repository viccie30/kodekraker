use std::fmt;

#[derive(Debug)]
pub enum Card {
    AddThreeDigits(Digit, Digit, Digit),
    AddTwoDigits(Digit, Digit),
    MultiplyTwoDigits(Digit, Digit),
    AddAllOfParity(Parity),
    NumberOfParity(Parity),
    PresenceOfNumber(Number),
}

pub struct Digit(u8);
pub struct Number(u8);

#[derive(Debug)]
pub enum Parity {
    Even,
    Odd,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
match self {
    AddThreeDigits(d1, d2, d3),
    AddTwoDigits(d1, d2),
    MultiplyTwoDigits(d1, d2),
    AddAllOfParity(parity),
    NumberOfParity(parity),
    PresenceOfNumber(n),
}
}

pub fn generate_cards() -> Vec<Card> {
    let mut deck = Vec::with_capacity(61);

    for i in 1..=4 {
        for j in i + 1..=5 {
            for k in j + 1..=6 {
                deck.push(Card::AddThreeDigits(i, j, k));
            }
        }
    }

    for i in 1..=5 {
        for j in i + 1..=6 {
            deck.push(Card::AddTwoDigits(i, j));
            deck.push(Card::MultiplyTwoDigits(i, j));
        }
    }

    deck.push(Card::AddAllOfParity(Parity::Even));
    deck.push(Card::AddAllOfParity(Parity::Odd));
    deck.push(Card::NumberOfParity(Parity::Even));
    deck.push(Card::NumberOfParity(Parity::Odd));

    for i in 0..=6 {
        deck.push(Card::PresenceOfNumber(i));
    }

    deck
}
