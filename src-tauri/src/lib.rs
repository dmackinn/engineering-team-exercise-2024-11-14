mod enemy;
mod dungeon;
mod player;
mod game;

use game::Game;
use tauri::{command, State};

use std::sync::Mutex;

#[derive(Default)]
struct GameState(Mutex<Game>);

#[command]
fn start_game(state: State<GameState>) {
    let mut game = state.0.lock().unwrap();
    *game = Game::new();
}

#[command]
fn get_room_description(state: State<GameState>) -> String {
    let game = state.0.lock().unwrap();
    game.describe_current_room()
}

#[command]
fn player_action(state: State<GameState>, action: String) -> String {
    let mut game = state.0.lock().unwrap();
    game.handle_player_action(action)
}

#[command]
fn exit_game() {
  std::process::exit(0);
}

// #[command]
// fn can_leave(state: State<GameState>) -> bool {
//     let game = state.0.lock().unwrap();
//     game.can_leave()
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(GameState::default())
        .invoke_handler(tauri::generate_handler![
            start_game,
            get_room_description,
            player_action,
            exit_game
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
