#[derive(Debug, Default)]
pub struct ClientAccount {
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool,
}

impl ClientAccount {
    pub fn deposit(&mut self, amount: f64) {
        if !self.locked {
            self.available += amount;
            self.total += amount;
        }
    }

    pub fn withdrawal(&mut self, amount: f64) -> bool {
        if self.available >= amount && !self.locked {
            self.available -= amount;
            self.total -= amount;
            true
        } else {
            false
        }
    }

    pub fn dispute(&mut self, amount: f64) {
        if self.available >= amount && !self.locked {
            self.available -= amount;
            self.held += amount;
        }
    }

    pub fn resolve(&mut self, amount: f64) {
        if self.held >= amount && !self.locked {
            self.held -= amount;
            self.available += amount;
        }
    }

    pub fn chargeback(&mut self, amount: f64) {
        if self.held >= amount && !self.locked {
            self.held -= amount;
            self.total -= amount;
            self.locked = true;
        }
    }
}
