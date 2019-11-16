use super::cards::*;

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
pub struct Combination([Digit; 6]);

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
	pub fn new(d1: Digit, d2: Digit, d3: Digit, d4: Digit, d5: Digit, d6: Digit) -> Combination {
		Combination([d1, d2, d3, d4, d5, d6])
	}
}
