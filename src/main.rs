mod cards;
mod game;

fn main() {
	let deck = cards::Deck::new();

	for card in deck {
		match card {
			cards::Card::PresenceOfNumber(n) => println!("{}", n),
			_ => continue,
		};
	}

	// 	println!(
	// "{:?}",
	// game::Game::new(
	// game::Combination::new(1, 2, 3, 4, 5, 6),
	// game::Combination::new(4, 3, 2, 1, 0, 5)
	// )
	// );
}
