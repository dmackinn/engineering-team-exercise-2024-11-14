pub struct Player {
    pub health: i32,
    pub position: usize,
}

impl Player {
    pub fn new() -> Self {
        Player { health: 100, position: 0 }
    }

    pub fn move_forward(&mut self) {
        self.position += 1;
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
    }
}
