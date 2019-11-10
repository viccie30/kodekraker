use std::fmt;

#[derive(Debug)]
pub enum Card {
    AddThreeDigits(Digit, Digit, Digit),
    AddTwoDigits(Digit, Digit),
    MultiplyTwoDigits(Digit, Digit),
    AddAllOfParity(Parity),
    NumberOfParity(Parity),
    PresenceOfNumber(u8),
}

pub type Digit = u8;

#[derive(Debug)]
pub enum Parity {
    Even,
    Odd,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, match self { AddThreeDigits(i,j,k) => 
}
}

pub fn generate_cards() -> Vec<Card> {
    let mut deck = Vec::with_capacity(61);

    for i in 1..5 {
        for j in i + 1..6 {
            for k in j + 1..7 {
                deck.push(Card::AddThreeDigits(i, j, k));
            }
        }
    }

    for i in 1..6 {
        for j in i + 1..7 {
            deck.push(Card::AddTwoDigits(i, j));
            deck.push(Card::MultiplyTwoDigits(i, j));
        }
    }

    deck.push(Card::AddAllOfParity(Parity::Even));
    deck.push(Card::AddAllOfParity(Parity::Odd));
    deck.push(Card::NumberOfParity(Parity::Even));
    deck.push(Card::NumberOfParity(Parity::Odd));

    for i in 0..7 {
        deck.push(Card::PresenceOfNumber(i));
    }

    deck
}
