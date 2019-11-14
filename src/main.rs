mod cards;

fn main() {
    for card in cards::generate_cards() {
        println!("{}", card);
    }
}
