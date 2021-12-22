pub struct DeterministicDie {
    next: u16,
    roll_count: u32,
}

impl DeterministicDie {
    pub fn new() -> Self {
        DeterministicDie {
            next: 1,
            roll_count: 0,
        }
    }

    pub fn roll(&mut self) -> u16 {
        let result = self.next;
        if self.next == 100 {
            self.next = 1;
        } else {
            self.next += 1;
        }
        self.roll_count += 1;
        result
    }

    pub fn get_roll_count(&self) -> u32 {
        self.roll_count
    }
}
