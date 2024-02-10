use serde::{ Deserialize, Serialize };
use serde_json::json;
use std::{
    fs::File,
    io::Read
};

#[derive(Serialize, Deserialize)]
pub struct PlayerData {
    pub username: String,
    pub bank: usize
}

#[derive(Clone, Default)]
pub struct GameData {
    // Base Game
    pub irsdebt: u128,
    pub carteldebt: u128,
    pub rent: u32,

    // Time Data
    pub current_hrs: u8,
    pub current_day: u8,
}

fn save_playerdata(
    player: &PlayerData,
    game: &GameData,
    filename: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let game_data = json!({
        "player": {
            "username": player.username,
            "bank": player.bank,
        },
        "game": {
            "irsdebt": game.irsdebt,
            "carteldebt": game.carteldebt,
            "rent": game.rent,
        },
    });

    let file = File::create(filename)?;

    serde_json::to_writer(file, &game_data)?;

    Ok(())
}

pub fn save_game(player: &PlayerData, game: &GameData) {
    let player = PlayerData {
        username: player.username.to_string(),
        bank: player.bank,
    };

    let game = GameData {
        irsdebt: game.irsdebt,
        carteldebt: game.carteldebt,
        rent: game.rent,

        current_day: game.current_day,
        current_hrs: game.current_hrs,
    };

    if let Err(err) = save_playerdata(&player, &game, "player_data.json") {
        eprintln!("Error saving player data: {}", err);
    } else {
        println!("Player data saved successfully!");
    }
}

pub fn load_game(filename: String) -> Result<PlayerData, Box<dyn std::error::Error>> {
    let mut file = File::open(filename)?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let player_data: PlayerData = serde_json::from_str(&buffer)?;

    Ok(player_data)
}
