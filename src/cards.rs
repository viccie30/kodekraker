use std::fmt;

use rand::prelude::*;

use Card::*;
use Parity::*;

#[derive(Debug)]
pub struct Deck(Vec<Card>);

#[derive(Debug)]
pub enum Card {
	AddThreeDigits(Digit, Digit, Digit),
	AddTwoDigits(Digit, Digit),
	MultiplyTwoDigits(Digit, Digit),
	AddAllOfParity(Parity),
	NumberOfParity(Parity),
	PresenceOfNumber(Number),
}
#[derive(Debug, Copy, Clone)]
pub struct Digit(u8);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Number(u8);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Parity {
	Even,
	Odd,
}

impl fmt::Display for Card {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			AddThreeDigits(d1, d2, d3) => write!(
				f,
				"Tel het {}, het {} en het {} cijfer bij elkaar",
				d1, d2, d3
			),
			AddTwoDigits(d1, d2) => write!(f, "Tel het {} en het {} cijfer bij elkaar", d1, d2),
			MultiplyTwoDigits(d1, d2) => {
				write!(f, "Vermenigvuldig het {} met het {} cijfer", d1, d2)
			},
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
		match self {
			Digit(0) => write!(f, "eerste"),
			Digit(1) => write!(f, "tweede"),
			Digit(2) => write!(f, "derde"),
			Digit(3) => write!(f, "vierde"),
			Digit(4) => write!(f, "vijfde"),
			Digit(5) => write!(f, "zesde"),
			_ => panic!("Illegal digit"),
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
			Even => write!(f, "even"),
			Odd => write!(f, "oneven"),
		}
	}
}

impl Deck {
	pub fn new() -> Deck {
		let mut deck = Vec::with_capacity(61);

		for i in 0..4 {
			for j in i + 1..5 {
				for k in j + 1..6 {
					deck.push(AddThreeDigits(Digit(i), Digit(j), Digit(k)));
				}
			}
		}

		for i in 0..5 {
			for j in i + 1..6 {
				deck.push(AddTwoDigits(Digit(i), Digit(j)));
				deck.push(MultiplyTwoDigits(Digit(i), Digit(j)));
			}
		}

		deck.push(AddAllOfParity(Even));
		deck.push(AddAllOfParity(Odd));
		deck.push(NumberOfParity(Even));
		deck.push(NumberOfParity(Odd));

		for i in 0..=6 {
			deck.push(PresenceOfNumber(Number(i)));
		}

		let mut rng = StdRng::from_rng(thread_rng()).unwrap();
		deck.shuffle(&mut rng);

		Deck(deck)
	}
}

impl Iterator for Deck {
	type Item = Card;

	fn next(&mut self) -> Option<Self::Item> { self.0.pop() }
}

impl Digit {
	pub fn as_u8(self) -> u8 { self.0 }
}

impl From<u8> for Digit {
	fn from(digit: u8) -> Digit {
		assert!(digit < 6);

		Digit(digit)
	}
}

impl Number {
	pub fn as_u8(self) -> u8 { self.0 }

	pub fn parity(self) -> Parity {
		let Number(n) = self;

		if n % 2 == 0 {
			Even
		} else {
			Odd
		}
	}
}

impl From<u8> for Number {
	fn from(number: u8) -> Number {
		assert!(number <= 6);

		Number(number)
	}
}
