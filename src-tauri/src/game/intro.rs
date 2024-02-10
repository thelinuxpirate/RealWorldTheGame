use crate::game::{
    market::NFT,
    market::crypto_to_usd,
    game::cal_rent,
    game::run_main,
    game::hit_return,
    data::PlayerData,
    data::GameData,
};
use std::io::{ self, Write };
use std::process::Command;
use rand::Rng;

fn get_usr() -> PlayerData {
    let mut username = String::new();
    print!("Enter your username:\n(MAX is 10 characters)\n> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();

    username = username.trim().to_string().to_lowercase();
    username = username.replace(" ", "_");
    
    if username.len() > 10 {
        println!("\nYour username exceeds 10 characters.\nTruncated to the first 10 characters.\n");
        username.truncate(10);
    } else if username.is_empty() {
        println!("\nUsername must not be empty!\n");
        return get_usr();
    }

    println!("Username: {}", username);

    loop {
        print!("\nIs this correct? [Y/n]\t> ");
        io::stdout().flush().unwrap();

        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();
        let response = response.trim();

        if response.is_empty() || response.eq_ignore_ascii_case("Y") {
            return PlayerData {
                username,
                bank: 0,
            };
        } else if response.eq_ignore_ascii_case("n") {
            return get_usr();
        } else {
            println!("Invalid input. Please enter 'Y' or 'n'.");
        }
    }
}

fn cal_intro_debt(player: &PlayerData) -> GameData {
    let mut rng = rand::thread_rng();
    let mut rolls_list = [0; 3];
    let irs_debt;
    let cartel_debt;

    println!("Rolling Die...");
    rolls_list[0] = rng.gen_range(1..=20);
    println!("> {} rolled a: {}\n", player.username, rolls_list[0]);

    println!("Rolling Die...");
    rolls_list[1] = rng.gen_range(1..=20);
    println!("> {} rolled a: {}\n", player.username, rolls_list[1]);

    irs_debt = crypto_to_usd(rolls_list[0] * rolls_list[1], 0);
    println!("{} owes {} USD to the IRS...\n", player.username, irs_debt);

    println!("Rolling Die...");
    rolls_list[2] = rng.gen_range(1..=20);
    println!("> {} rolled a: {}", player.username, rolls_list[2]);
    cartel_debt = crypto_to_usd(rolls_list[2] * 3, 0);
    println!("{} owes {} USD to the Cartel...\n", player.username, cartel_debt);

    println!("Calculating {}'s rent total...\n", player.username);
    let rent = cal_rent(&player.username);
    println!("{}'s rent is {} USD.\n", player.username, rent);

    return GameData {
        irsdebt: irs_debt,
        carteldebt: cartel_debt,
        rent: rent,

        // this will be handeled after the intro 
        current_day: 0,
        current_hrs: 0,
    };
}

pub fn run_intro(nft: &mut NFT) {
    let mut clear_screen = Command::new("clear");
    clear_screen.status().expect("Process failed to execute");
    let player = get_usr();
    let gamedata = cal_intro_debt(&player);
    hit_return();
    clear_screen.status().expect("Process failed to execute");
    run_main(&player, &gamedata, nft);
}
