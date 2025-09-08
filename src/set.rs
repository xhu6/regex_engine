pub struct Set {
    // Set with efficient iter
    pub usizes: Vec<usize>,
    pub bools: Vec<bool>,
}

impl Set {
    pub fn new(n: usize) -> Self {
        Self {
            usizes: Vec::with_capacity(10),
            bools: vec![false; n],
        }
    }

    pub fn insert(&mut self, value: usize) {
        if self.bools[value] {
            return;
        }

        self.bools[value] = true;
        self.usizes.push(value);
    }

    pub fn contains(&self, value: usize) -> bool {
        self.bools[value]
    }

    pub fn clear(&mut self) {
        self.bools.fill(false);
        self.usizes.clear();
    }
}
