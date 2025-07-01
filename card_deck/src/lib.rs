use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

impl Suit {
    pub fn random() -> Suit {
        // Generate a random number between 1 and 4
        let random_value = rand::thread_rng().gen_range(1..=4);
        Suit::translate(random_value)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => Suit::Heart, // Default case for invalid values
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    Number(u8), // For cards 2-10
    Jack,
    Queen,
    King,
}

impl Rank {
    pub fn random() -> Rank {
        // Generate a random number between 1 and 13
        let random_value = rand::thread_rng().gen_range(1..=13);
        Rank::translate(random_value)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => Rank::Ace, // Default case for invalid values
        }
    }
}

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    // Check if the card is the Ace of Spades
    card.suit == Suit::Spade && card.rank == Rank::Ace
}
