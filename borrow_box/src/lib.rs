#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16
}

impl GameSession {
    // Initializes a new game session with player names
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        Box::new(GameSession {
            id, 
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games
        })
    }

    // Returns the name and score of the winning player or a tie message
    pub fn read_winner(&self) -> (String, u16) {
        if self.p1.1 == self.p2.1 {
            ("Same score! tied".to_string(), self.p1.1)
        } else if self.p1.1 > self.p2.1 {
            (self.p1.0.clone(), self.p1.1)
        } else {
            (self.p2.0.clone(), self.p2.1)
        }
    }

    // Updates the player's score
    pub fn update_score(&mut self, user_name: String) {
        // Calculate the winning score (more than half of total games)
        let winning_score = (self.nb_games / 2) + 1;

        if self.p1.1 >= winning_score || self.p2.1 >= winning_score {
            return;  // Game is already finished, do nothing
        }
        
        // Update the score for the matching player
        if user_name == self.p1.0 {
            self.p1.1 += 1;
        } else if user_name == self.p2.0 {
            self.p2.1 += 1;
        }
    }
    
    pub fn delete(self) -> String {
        format!("game deleted: id -> {}", self.id)
    }
}