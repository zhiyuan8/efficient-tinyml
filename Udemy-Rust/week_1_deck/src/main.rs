use rand::{seq::SliceRandom, thread_rng}; // Importing necessary modules from the 'rand' crate for random number generation and shuffling.

#[derive(Debug)] // This attribute allows the Deck struct to be printed using the {:?} formatter.
struct Deck {
    cards: Vec<String>, // The 'Deck' struct has a field 'cards' which is a vector of strings.
}

impl Deck { // Implement methods for the 'Deck' struct.
    fn new() -> Self { // Define a function 'new' that returns an instance of 'Deck'.
        let suits = ["Hearts", "Spades", "Diamonds"]; // An array of card suits.
        let values = ["Ace", "Two", "Three"]; // An array of card values.

        let mut cards = vec![]; // Initialize an empty vector to store cards.

        for suit in suits { // Loop over each suit.
            for value in values { // Loop over each value.
                let card = format!("{} of {}", value, suit); // Create a string for each card.
                cards.push(card); // Add the card to the 'cards' vector.
            }
        }

        Deck { cards } // Return a new 'Deck' instance with the populated 'cards' vector.
    }

    fn shuffle(&mut self) { // Define a method 'shuffle' that shuffles the cards in the deck.
        let mut rng = thread_rng(); // Create a random number generator.
        self.cards.shuffle(&mut rng); // Shuffle the cards using the random number generator.
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> { // Define a method 'deal' that takes a number of cards to deal and returns a vector of strings.
        self.cards.split_off(self.cards.len() - num_cards) // Remove and return the last 'num_cards' cards from the deck.
    }
}

fn main() { // The main function, entry point of the program.
    let mut deck = Deck::new(); // Create a new deck of cards.

    deck.shuffle(); // Shuffle the deck.
    // Probably need to add error handling!!!!
    let cards = deck.deal(3); // Deal 3 cards from the deck.

    println!("Heres your hand: {:#?}", cards); // Print the dealt cards.
    println!("Heres your deck: {:#?}", deck); // Print the remaining cards in the deck.
}