use std::fmt;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub strength: f64,
    pub score: i32,
    pub money: i32,
    pub weapons: Vec<String>,
}

pub struct Fruit {
    pub weight_in_kg: f64,  // Fixed field name
}

pub struct Meat {
    pub weight_in_kg: f64,
    pub fat_content: f64,   // Added missing field
}

impl Player {
    pub fn eat<T: Food>(&mut self, food: T) {  // Made method public
        self.strength += food.gives();
    }
}

pub trait Food {
    fn gives(&self) -> f64;
}

impl Food for Fruit {
    fn gives(&self) -> f64 {
        // 4 units of strength per kg of fruit
        self.weight_in_kg * 4.0
    }
}

impl Food for Meat {
    fn gives(&self) -> f64 {
        // Calculate weight of fat and protein
        let fat_weight = self.weight_in_kg * self.fat_content;  // Fixed syntax
        let protein_weight = self.weight_in_kg * (1.0 - self.fat_content);
        
        // Calculate strength from fat (9 units per kg) and protein (4 units per kg)
        let strength_from_fat = fat_weight * 9.0;
        let strength_from_protein = protein_weight * 4.0;
        
        // Return total strength
        strength_from_fat + strength_from_protein  // Fixed typo
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // First line: player name
        writeln!(f, "{}", self.name)?;
        
        // Second line: strength, score, and money
        writeln!(f, "Strength: {}, Score: {}, Money: {}", 
                self.strength, self.score, self.money)?;
        
        // Third line: list of weapons
        write!(f, "Weapons: {:?}", self.weapons)
    }
}