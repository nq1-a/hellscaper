use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub agility: i32,
    pub charisma: i32,
    pub intelligence: i32,
    pub resillience: i32,
    pub strength: i32,
}

impl Stats {
    pub fn new(
        agility: i32,
        charisma: i32,
        intelligence: i32,
        resillience: i32,
        strength: i32,
    ) -> Result<Stats, String> {
        let stats: Stats = Stats {
            agility,
            charisma,
            intelligence,
            resillience,
            strength,
        };

        if stats.sum() > 2 {
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
            .max(self.resillience.abs())
            .max(self.strength.abs())
    }

    pub fn sum(&self) -> i32 {
        self.agility +
        self.charisma +
        self.intelligence +
        self.resillience +
        self.strength
    }
}

#[derive(Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub stats: Stats,
}
