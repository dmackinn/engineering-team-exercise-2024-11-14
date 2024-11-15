use rand::{thread_rng, Rng};
use crate::dungeon::Dungeon;
use crate::player::Player;

pub struct Game {
    dungeon: Dungeon,
    player: Player,
}

impl Game {
    pub fn new() -> Self {
        Game {
            dungeon: Dungeon::new(5),
            player: Player::new(),
        }
    }

    pub fn describe_current_room(&self) -> String {
        self.dungeon.describe_room(self.player.position).unwrap_or_else(|| "You have reached the end. You discover a chest filled with treaure.".to_string())
    }

    pub fn handle_player_action(&mut self, action: String) -> String {
        match action.as_str() {
            "move" => {
                self.player.move_forward();
                self.describe_current_room()
            },
            "attack" => {
                // TODO could improve by implementing a chance for the player to miss or run away
                if let Some(room) = self.dungeon.rooms.get_mut(self.player.position) {
                    if room.has_enemy {
                        let enemy = room.enemy.as_mut().unwrap();
                        if enemy.is_dead() {
                            return "Your enemy still lays slain before you. Attacking again seems excessive.".to_string()
                        }
                        let player_damage = 10;
                        self.player.take_damage(player_damage);
                        let mut rng = thread_rng();
                        let enemy_damage = rng.gen_range(1..=enemy.health);

                        enemy.take_damage(enemy_damage);
                        if enemy.is_dead() {
                            return format!("You killed an enemy and took {} damage! Health: {}", player_damage, self.player.health)
                        } else {
                            return format!("You attack the enemy for {} damage. They have {} health left. You have taken {} damage! Health: {}", enemy_damage, enemy.health, player_damage,  self.player.health);
                        }
                    } else {
                        "There is no enemy here.".to_string()
                    }
                } else {
                    "Invalid room.".to_string()
                }
            },
            _ => "Invalid action".to_string(),
        }
    }}

impl Default for Game {
    fn default() -> Self {
        Game::new()
    }
}
