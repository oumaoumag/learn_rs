#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "B" => Ok(Antigen::B),
            "AB" => Ok(Antigen::AB),
            "O" => Ok(Antigen::O),
            _ => Err(format!("Invalid antigen: {}", s)),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(format!("Invalid RhFactor: {}", s)),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        // First compare by antigen
        match self.antigen.cmp(&other.antigen) {
            Ordering::Equal => {
                // If antigens are equal, compare by rh_factor
                self.rh_factor.cmp(&other.rh_factor)
            }
            ordering => ordering,
        }
    }
}
impl FromStr for BloodType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(format!("Blood type string too short: {}", s));
        }
        
        // Split the string into antigen and rh_factor parts
        let (antigen_str, rh_str) = s.split_at(s.len() - 1);
        
        // Parse the antigen and rh_factor
        let antigen = Antigen::from_str(antigen_str)?;
        let rh_factor = RhFactor::from_str(rh_str)?;
        
        Ok(BloodType { antigen, rh_factor })
    }
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format antigen
        match self.antigen {
            Antigen::A => write!(f, "A")?,
            Antigen::B => write!(f, "B")?,
            Antigen::AB => write!(f, "AB")?,
            Antigen::O => write!(f, "O")?,
        }
        
        // Format rh_factor
        match self.rh_factor {
            RhFactor::Positive => write!(f, "+"),
            RhFactor::Negative => write!(f, "-"),
        }
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        // Check antigen compatibility
        let antigen_compatible = match self.antigen {
            Antigen::O => other.antigen == Antigen::O,
            Antigen::A => other.antigen == Antigen::A || other.antigen == Antigen::O,
            Antigen::B => other.antigen == Antigen::B || other.antigen == Antigen::O,
            Antigen::AB => true, // AB can receive from any antigen
        };
        
        // Check Rh factor compatibility
        let rh_compatible = match self.rh_factor {
            RhFactor::Positive => true, // Positive can receive from any Rh factor
            RhFactor::Negative => other.rh_factor == RhFactor::Negative,
        };
        
        antigen_compatible && rh_compatible
    }

    pub fn donors(&self) -> Vec<Self> {
        // Create all possible blood types
        let all_blood_types = Self::all_types();
        
        // Filter to only those that can donate to self
        all_blood_types
            .into_iter()
            .filter(|donor| self.can_receive_from(donor))
            .collect()
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        // Create all possible blood types
        let all_blood_types = Self::all_types();
        
        // Filter to only those that can receive from self
        all_blood_types
            .into_iter()
            .filter(|recipient| recipient.can_receive_from(self))
            .collect()
    }

    // Helper method to create all possible blood types
    fn all_types() -> Vec<BloodType> {
        let mut types = Vec::new();
        
        // Create all combinations of antigen and rh_factor
        for antigen in [Antigen::A, Antigen::B, Antigen::AB, Antigen::O] {
            for rh_factor in [RhFactor::Positive, RhFactor::Negative] {
                types.push(BloodType { 
                    antigen: antigen.clone(), 
                    rh_factor: rh_factor.clone() 
                });
            }
        }
        
        types
    }
}