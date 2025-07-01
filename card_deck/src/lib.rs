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

