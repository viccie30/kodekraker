use super::cards::{Card::*, *};

#[derive(Debug)]
pub struct Game {
	deck: Deck,
	states: [PlayerState; 2],
}

#[derive(Debug)]
pub struct PlayerState {
	combination: Combination,
	answers: Vec<AnsweredQuestion>,
}

#[derive(Debug)]
pub struct Combination([Number; 6]);

#[derive(Debug)]
pub struct AnsweredQuestion {
	card: Card,
	answer: u8,
}

impl Game {
	pub fn new(comb_player1: Combination, comb_player2: Combination) -> Game {
		Game {
			deck: Deck::new(),
			states: [
				PlayerState {
					combination: comb_player1,
					answers: Vec::new(),
				},
				PlayerState {
					combination: comb_player2,
					answers: Vec::new(),
				},
			],
		}
	}
}

impl Combination {
	pub fn new<T: Into<Number>>(n1: T, n2: T, n3: T, n4: T, n5: T, n6: T) -> Combination {
		Combination([
			n1.into(),
			n2.into(),
			n3.into(),
			n4.into(),
			n5.into(),
			n6.into(),
		])
	}
}

impl Card {
	pub fn answer(self, answer: u8) -> AnsweredQuestion { AnsweredQuestion { card: self, answer } }
}

impl PlayerState {
	pub fn answer(&self, card: &Card) -> u8 {
		match card {
			AddThreeDigits(d1, d2, d3) => d1 + d2 + d3,
			AddTwoDigits(d1, d2) => d1 + d2,
			MultiplyTwoDigits(d1, d2) => d1 * d2,
			AddAllOfParity(parity) => todo!(),
			NumberOfParity(parity) => todo!(),
			PresenceOfNumber(n) => todo!(),
		}
	}
}
