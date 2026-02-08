use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Campaign {
    members: Vec<u64>,
}

impl Campaign {
    pub fn new(owner: u64) -> Campaign {
        Campaign {
            members: vec![owner],
        }
    }

    pub fn add(&mut self, member: u64) {
        if !self.members.contains(&member) {
            self.members.push(member);
        }
    }

    pub fn remove(&mut self, member: u64) {
        for i in 1..self.members.len() {
            if self.members[i] == member {
                self.members.remove(i);
            }
        }
    }

    pub fn ping_list(&self, glue: &str) -> String {
        (&self.members)
            .into_iter()
            .map(|m| format!("<@{}>", m))
            .collect::<Vec<String>>()
            .join(glue)
    }

    pub fn ping_all(&self) -> String {
        self.ping_list("")
    }

    pub fn owner(&self) -> u64 {
        self.members[0]
    }
}
