use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

use serde::{Serialize, Deserialize};

const SKILLS: &[&str] = &[
    "dexterity",
    "elusion",
];

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stats {
    pub agility: i32,
    pub charisma: i32,
    pub intelligence: i32,
    pub resilience: i32,
    pub strength: i32,
    pub skills: HashMap<String, i32>,
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
            skills: HashMap::new(),
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

    pub fn skill(&mut self, skill: impl Into<String>) -> i32 {
        *self.skills.entry(skill.into()).or_insert(0)
    }

    pub fn sum(&self) -> i32 {
        self.agility +
        self.charisma +
        self.intelligence +
        self.resilience +
        self.strength
    }

    pub fn view_skills(&mut self) -> String {
        let mut disp: String = String::new();

        for s in SKILLS.iter() {
            disp = format!("{}{}: {}\n",
                disp,
                s[0..3].to_uppercase(),
                self.skill(*s),
            );
        }

        return disp;
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
