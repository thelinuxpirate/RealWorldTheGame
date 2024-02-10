use clap::Parser;
use crate::game::{
		market::NFT,
		game::run_main,
		data::GameData,
		data::load_game
};
use std::env;

mod game;


// `matches` here is a Struct with { args, subcommand }.
// `args` is `HashMap<String, ArgData>` where `ArgData` is a struct with { value, occurrences }.
// `subcommand` is `Option<Box<SubcommandMatches>>` where `SubcommandMatches` is a struct with { name, matches }.

#[cfg(all(not(debug_assertions), windows))]
fn remove_windows_console() {
		unsafe {
				windows_sys::Win32::System::Console::FreeConsole();
		}
}

#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            if env::args_os().count() <= 1 {
                return Ok(())
            }
            let matches = app.get_cli_matches()?;
            for (key, value) in matches.args {
                if value.occurrences > 0 {
                    match key.as_str() {
                        "load" => {
                            let args = Args::parse();
                            let game = GameData::default();
                            let mut nft = NFT::default();
                            
                            if let Some(filename) = args.load {
                                match load_game(filename) {
                                    Ok(player) => {
                                        println!("Player data loaded successfully:");
                                        println!("Username: {}", player.username);
                                        println!("Bank: {}", player.bank);

                                        println!("Starting Game...");
                                        run_main(&player, &game, &mut nft);
                                    }
                                    Err(err) => eprintln!("Error loading player data: {}", err),
                                }
                            }
                        },
                        _ => {
                            println!("sleeping for unhandled cli arg: {}", key);
                            std::thread::sleep(std::time::Duration::from_secs(5));
                            app.handle().exit(1);
                        },
                    }
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
