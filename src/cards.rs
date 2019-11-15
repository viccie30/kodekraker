use std::fmt;

use rand::prelude::*;

use Card::*;

#[derive(Debug)]
pub enum Card {
    AddThreeDigits(Digit, Digit, Digit),
    AddTwoDigits(Digit, Digit),
    MultiplyTwoDigits(Digit, Digit),
    AddAllOfParity(Parity),
    NumberOfParity(Parity),
    PresenceOfNumber(Number),
}
#[derive(Debug)]
pub struct Digit(u8);
#[derive(Debug)]
pub struct Number(u8);

#[derive(Debug)]
pub enum Parity {
    Even,
    Odd,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AddThreeDigits(d1, d2, d3) => write!(
                f,
                "Tel het {:#}, het {:#} en het {:#} cijfer bij elkaar",
                d1, d2, d3
            ),
            AddTwoDigits(d1, d2) => write!(f, "Tel het {:#} en het {:#} cijfer bij elkaar", d1, d2),
            MultiplyTwoDigits(d1, d2) => {
                write!(f, "Vermenigvuldig het {:#} met het {:#} cijfer", d1, d2)
            }
            AddAllOfParity(parity) => write!(f, "Tel alle {} cijfers bij elkaar", parity),
            NumberOfParity(parity) => write!(
                f,
                "Hoeveel {} cijfers komen er in de combinatie voor?",
                parity
            ),
            PresenceOfNumber(n) => write!(f, "Komt er een {} in de combinatie voor?", n),
        }
    }
}

impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_assert!(self.0 <= 6 && self.0 >= 1);

        if f.alternate() {
            match self {
                Digit(1) => write!(f, "eerste"),
                Digit(2) => write!(f, "tweede"),
                Digit(3) => write!(f, "derde"),
                Digit(4) => write!(f, "vierde"),
                Digit(5) => write!(f, "vijfde"),
                Digit(6) => write!(f, "zesde"),
                _ => panic!("Illegal digit"),
            }
        } else {
            match self {
                Digit(1) => write!(f, "één"),
                Digit(2) => write!(f, "twee"),
                Digit(3) => write!(f, "drie"),
                Digit(4) => write!(f, "vier"),
                Digit(5) => write!(f, "vijf"),
                Digit(6) => write!(f, "zes"),
                _ => panic!("Illegal digit"),
            }
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_assert!(self.0 <= 6);

        let Number(n) = self;

        fmt::Display::fmt(n, f)
    }
}

impl fmt::Display for Parity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Parity::Even => write!(f, "even"),
            Parity::Odd => write!(f, "oneven"),
        }
    }
}

pub fn generate_cards() -> Vec<Card> {
    let mut deck = Vec::with_capacity(61);

    for i in 1..=4 {
        for j in i + 1..=5 {
            for k in j + 1..=6 {
                deck.push(AddThreeDigits(Digit(i), Digit(j), Digit(k)));
            }
        }
    }

    for i in 1..=5 {
        for j in i + 1..=6 {
            deck.push(AddTwoDigits(Digit(i), Digit(j)));
            deck.push(MultiplyTwoDigits(Digit(i), Digit(j)));
        }
    }

    deck.push(AddAllOfParity(Parity::Even));
    deck.push(AddAllOfParity(Parity::Odd));
    deck.push(NumberOfParity(Parity::Even));
    deck.push(NumberOfParity(Parity::Odd));

    for i in 0..=6 {
        deck.push(PresenceOfNumber(Number(i)));
    }

    let mut rng = StdRng::from_rng(thread_rng()).unwrap();
    deck.shuffle(&mut rng);

    deck
}
