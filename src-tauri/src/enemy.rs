
#[derive(Debug)]
pub enum EnemyType {
    Skeleton,
    Orc,
    Troll,
    Goblin,
}

#[derive(Debug)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub health: u32,
}

impl Enemy {
    pub fn new() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let enemy_type = match rng.gen_range(0..4) {
            0 => EnemyType::Skeleton,
            1 => EnemyType::Orc,
            2 => EnemyType::Troll,
            _ => EnemyType::Goblin,
        };

        let health = match enemy_type {
            EnemyType::Skeleton => 20,
            EnemyType::Orc => 30,
            EnemyType::Troll => 40,
            EnemyType::Goblin => 15,
        };
        Enemy { enemy_type, health }
    }
    pub fn take_damage(&mut self, damage: u32) {
        if self.health > damage {
            self.health -= damage;
        } else {
            self.health = 0;
        }
    }
    pub fn description(&self) -> String {
        match self.enemy_type {
            EnemyType::Skeleton => "a rattling skeleton",
            EnemyType::Orc => "a brutish orc",
            EnemyType::Troll => "a hulking troll",
            EnemyType::Goblin => "a sneaky goblin",
        }
        .to_string()
    }

    pub fn is_dead(&self) -> bool {
        self.health == 0
    }
}