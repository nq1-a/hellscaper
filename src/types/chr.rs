use std::fmt;
use std::fmt::{Display, Formatter};

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Skills {
    pub dexterity: i32,
    pub elusion: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stats {
    pub agility: i32,
    pub charisma: i32,
    pub intelligence: i32,
    pub resilience: i32,
    pub strength: i32,
    #[serde(default)]
    pub skills: Skills,
}

impl Stats {
    pub fn new(
        agility: i32,
        charisma: i32,
        intelligence: i32,
        resilience: i32,
        strength: i32,
        max_override: bool
    ) -> Result<Stats, String> {
        let stats: Stats = Stats {
            agility,
            charisma,
            intelligence,
            resilience,
            strength,
            skills: Default::default(),
        };

        if stats.sum() > 2 && !max_override {
            return Err("STAT SUM IS GREATER THAN 2".to_string());
        }

        if stats.abs_max() > 4 {
            return Err("ONE OR MORE STATS ARE OUTSIDE OF BOUND [-4, 4]".to_string());
        }

        Ok(stats)
    }

    pub fn abs_max(&self) -> i32 {
        self.agility.abs()
            .max(self.charisma.abs())
            .max(self.intelligence.abs())
            .max(self.resilience.abs())
            .max(self.strength.abs())
    }

    pub fn sum(&self) -> i32 {
        self.agility +
        self.charisma +
        self.intelligence +
        self.resilience +
        self.strength
    }
}

impl Display for Skills {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DEX: {}\nELU: {}",
            self.dexterity,
            self.elusion,
        )
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AG: {}\nCH: {}\nIN: {}\nRE: {}\nST: {}",
            self.agility,
            self.charisma,
            self.intelligence,
            self.resilience,
            self.strength,
        )
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub stats: Stats,
}
