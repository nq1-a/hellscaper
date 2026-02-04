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

    pub fn remove(&mut self, member: u64) -> bool {
        for i in 1..self.members.len() {
            if self.members[i] == member {
                self.members.remove(i);
                return true;
            }
        }

        false
    }

    pub fn ping_all(&self) -> String {
        (&self.members)
            .into_iter()
            .map(|m| format!("<@{}>", m))
            .collect::<Vec<String>>()
            .join("")
    }
}
