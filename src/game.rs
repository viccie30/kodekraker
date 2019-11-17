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
	answer: Answer,
}

#[derive(Debug)]
pub enum Answer {
	Number(u8),
	Boolean(bool),
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

	pub fn answer(&self, card: Card) -> AnsweredQuestion {
		let answer = match card {
			AddThreeDigits(d1, d2, d3) => {
				Answer::Number(self[d1].as_u8() + self[d2].as_u8() + self[d3].as_u8())
			},
			AddTwoDigits(d1, d2) => Answer::Number(self[d1].as_u8() + self[d2].as_u8()),
			MultiplyTwoDigits(d1, d2) => Answer::Number(self[d1].as_u8() * self[d2].as_u8()),
			AddAllOfParity(parity) => {
				Answer::Number(self.filter_on_parity(parity).filter(|n| n.as_u8()).sum())
			},
			NumberOfParity(parity) => Answer::Number(self.filter_on_parity(parity).count()),
			PresenceOfNumber(number) => {
				Answer::Boolean(self.into_iter().filter(|&&n| n == number).count() > 0)
			},
		};

		AnsweredQuestion { card, answer }
	}

	fn filter_on_parity(&self, parity: Parity) -> impl Iterator + '_ {
		self.into_iter().filter(|n| n.parity() == parity)
	}
}

impl std::ops::Index<Digit> for Combination {
	type Output = Number;

	fn index(&self, index: Digit) -> &Number { &self.0[index.as_u8() as usize] }
}

impl<'a> IntoIterator for &'a Combination {
	type IntoIter = <&'a [Number] as IntoIterator>::IntoIter;
	type Item = &'a Number;

	fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}
