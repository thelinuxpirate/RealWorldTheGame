use crate::game::{
    market::NFT,
    market::create_nft,
    market::crypto_to_usd,
    market::open_market,
    data::PlayerData,
    data::GameData,
    data::save_game
};
use std::{
    process::Command,
    io
};
use prettytable::{
    Table,
    row,
};
use rand::Rng;

pub fn cal_rent(username: &str) -> u32 {
    let mut rng = rand::thread_rng();
    let card1 = rng.gen_range(1..=10);
    let card2 = rng.gen_range(1..=10);
    let card3 = rng.gen_range(1..=10);

    println!("Generating Rent Cards...\n");
    println!("> {}'s first card has a value of: {}", username, card1);
    println!("> {}'s second card has a value of: {}", username, card2);
    println!("> {}'s third card has a value of: {}", username, card3);

    let rent = crypto_to_usd(card1 + card2 + card3, 1);

    rent.try_into().unwrap()
}

pub fn hit_return() -> String {
    let mut turn = String::new();
    println!("Hit <Return> to continue: ");
    io::stdin().read_line(&mut turn).unwrap();
    let turn = turn.trim();

    if turn.is_empty() {
        return "[ O K ]".to_string();
    } else {
        return "[ O K ]".to_string();
    }
}

pub fn run_main(
		player: &PlayerData,
		game: &GameData,
		nft: &mut NFT
) {
    let day_counter: u8 = 0;
		let new_game = game.clone();

		println!("> Hello {};\nYour introduction has been completed. \
							Its now time for you to start the Main Game!\n(Type: \"help\" for the manual; \"list\" for a list of options)", player.username);

		if day_counter <= 30 {
				let turn_value: u8 = cal_limit();

				for _ in 0..turn_value {
						take_turn(&player, &game, nft);
				}   
		} else {
				println!("Game Over!");
		}
}

fn cal_limit() -> u8 {
    // TODO: Make a better counter that implements "day_counter"
    30
}

fn take_turn(
		player: &PlayerData,
		game: &GameData,
		mut nft: &mut NFT
) -> u8 {
    let mut turn_counter: u8 = 0;

    loop {
        println!();
        let mut turn = String::new();
        io::stdin().read_line(&mut turn).unwrap();
        let turn = turn.trim();

        match turn.to_lowercase().as_str() {
            "" => {
                println!("Please input a valid option!");
                return take_turn(&player, &game, &mut nft);
            }
            "list" | "help" => {
                let mut list = Table::new();
                println!();

                // Header
                list.add_row(row!["Command", "Description"]);
                // Options
                list.add_row(row!["\"list\"", "Prints this command"]);
                list.add_row(row!["\"market\"", "Opens the Market Menu"]);
                list.add_row(row!["\"create\"", "Creates NFTs (uses 1 turn) N/A"]);
                list.add_row(row!["\"clear\"", "Clears the screen"]);
                list.add_row(row!["\"check\"", "Displays turns left & time limit N/A"]);
                list.add_row(row!["\"end\"", "Manually ends current turn N/A"]);
                list.add_row(row!["\"save\"", "Saves the current game"]);
                list.add_row(row!["\"quit\"", "Quits game (without saving!) N/A"]);
  
                // Table setup...
                list.printstd();
                return take_turn(&player, &game, &mut nft);
            }
            "market" => {
                open_market(&player);
                return take_turn(&player, &game, &mut nft);
            }
            "save" => {
                println!("\nSaving Game Data...");
                save_game(&player, &game);
                return take_turn(&player, &game, &mut nft);
            }
            "clear" => {
                let mut clear_screen = Command::new("clear");
                clear_screen.status().expect("Process failed to execute");
                return take_turn(&player, &game, &mut nft);
            }
            "create" => {
                create_nft(&mut nft, &game);
                return take_turn(&player, &game, &mut nft);
            }
            "end" => {
                println!("You have manually ended your turn...");
                turn_counter += 1;
                println!("Turn Counter: {}", turn_counter);
                break;
            }
            "quit" => {
                println!("[ O K ]");
                println!("Exiting Game...");
                break;
            }
            _ => {
                println!("Please input a valid option!");
                return take_turn(&player, &game, &mut nft);
            }
        }
    }

    turn_counter
}

pub fn end_game() {
		println!("Your game has ended!");
}
