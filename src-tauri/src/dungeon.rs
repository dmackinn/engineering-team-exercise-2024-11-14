use std::collections::HashSet;
use rand::thread_rng;
use crate::enemy::Enemy;


#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Scenery {
    BrokenStatue,
    FlickeringTorch,
    AncientRug,
    Cobwebs,
    CrackedPillar,
}

impl Scenery {
    pub fn description(&self) -> String {
        match self {
            Scenery::BrokenStatue => "a broken statue of a forgotten king",
            Scenery::FlickeringTorch => "a flickering torch on the wall",
            Scenery::AncientRug => "an ancient rug covered in dust",
            Scenery::Cobwebs => "thick cobwebs hanging from the ceiling",
            Scenery::CrackedPillar => "a cracked pillar supporting the ceiling",
        }
        .to_string()
    }
}

#[derive(Debug)]
pub struct Room {
    pub description: String,
    pub has_enemy: bool,
    pub enemy: Option<Enemy>,
    pub scenery: HashSet<Scenery>,  // Use HashSet to avoid duplicates
}

impl Room {
    pub fn new(description: String) -> Self {
        use rand::Rng;
        let mut rng = thread_rng();

        // 50% chance for an enemy
        let has_enemy = rng.gen_bool(0.5);
        let enemy = if has_enemy {
            Some(Enemy::new())
        } else {
            None
        };

        // Randomly generate scenery without duplicates
        let num_scenery_items = rng.gen_range(1..=3); // Rooms will have 1-3 scenery items
        let mut scenery = HashSet::new(); // Ensure no duplicates

        while scenery.len() < num_scenery_items {
            match rng.gen_range(0..5) {
                0 => scenery.insert(Scenery::BrokenStatue),
                1 => scenery.insert(Scenery::FlickeringTorch),
                2 => scenery.insert(Scenery::AncientRug),
                3 => scenery.insert(Scenery::Cobwebs),
                _ => scenery.insert(Scenery::CrackedPillar),
            };
        }

        Room {
            description,
            has_enemy,
            enemy,
            scenery,
        }
    }

    pub fn describe(&self) -> String {
        let base_description = &self.description;

        let scenery_descriptions: Vec<String> = self.scenery.iter().map(|s| s.description()).collect();
        let scenery_text = if !scenery_descriptions.is_empty() {
            format!(" You see {}.", scenery_descriptions.join(", "))
        } else {
            "".to_string()
        };

        let enemy_text = if let Some(enemy) = &self.enemy {
            format!(" You see {} here.", enemy.description())
        } else {
            "".to_string()
        };

        format!("{}{}{}", base_description, scenery_text, enemy_text)
    }
    pub fn can_leave(&self) -> bool {
        if let Some(enemy) = &self.enemy {
            !enemy.is_dead()
        } else {
            true // No enemy, so the player can leave
        }
    }

}

pub struct Dungeon {
    pub rooms: Vec<Room>,
}

impl Dungeon {
    pub fn new(size: usize) -> Self {
        let mut rooms = Vec::new();
        for i in 0..size {
            let description = format!("You have entered room {}. ", i + 1);
            rooms.push(Room::new(description));
        }
        Dungeon { rooms }
    }

    pub fn describe_room(&self, index: usize) -> Option<String> {
        self.rooms.get(index).map(|room| room.describe())
    }
}
