use crate::game::{
		data::GameData,
		data::PlayerData,
		game::end_game
};

use std::io;
use rand::Rng;
use prettytable::{
    Table,
    row,
};

#[derive(Default)]
pub struct NFT {
    limit: u8,
    value: usize,
    trash: bool
}

struct Stock {
    title: String,
    price: u8,
    desc: String,
    buff: String,
    stock: StockConditions
}

enum StockConditions {
    InStock,
    OutOfStock,
    Limit(u16),
}

pub fn crypto_to_usd(x: u32, c: u8) -> u128 {
    let exchange_rate = match c {
        0 => 6500, // BTC
        1 => 4891, // ETH
        2 => 517,  // XMR
        3 => 413,  // LTC
        _ => {
            eprintln!("Conversion Error...");
            std::process::exit(1);
						return 0
        }
    };

    (x * exchange_rate).into()
}

pub fn create_nft(
    mut nft: &mut NFT,
    game: &GameData
) -> bool {
    let mut rng = rand::thread_rng();
    let mut rolls_list = [0; 2];
    let mut nft_counter: u8 = 0;
    let mut hrs: u8 = game.current_hrs;
    let mut days: u8 = game.current_day;

    println!("Creating NFT...");
    nft_counter += 1;

    // get that time shit logic
    for _ in 0..nft_counter {
        hrs += 2;
        println!("Spent two hours creating nft..");
        
        if hrs >= 24 {
            hrs = 0;
            days += 1;

            println!("days: {}", days);
        } else if days >= 30 {
            end_game();
        }
    }

    let total_hours: u8 = hrs * nft_counter;
    rolls_list[0] = rng.gen_range(1..=6);
    rolls_list[1] = rng.gen_range(1..=6);

    println!("Set the price of your NFT\n");
    let mut nft_price = String::new();
    io::stdin().read_line(&mut nft_price).unwrap();
    let nft_price = nft_price.trim();
    
    let price = if let Ok(parsed_price) = nft_price.parse::<usize>() {
        println!("NFT's Price: {}", parsed_price);
        Some(parsed_price)
    } else {
        println!("You must set a price of non-zero!");
        return create_nft(&mut nft, game);
    };

    // R1 | Determines if purchase was successful
    let purchase_success = match rolls_list[0] {
        1..=3 => {
            println!("Purchase Succeded! (You rolled a: {})", rolls_list[0]);
            true
        },
        4..=6 => {
            println!("Purchase Failed! (You rolled a: {})", rolls_list[0]);
            false
        }
        _ => unreachable!(),
    };
    
    println!("Checking the price adjustment...");
        
    // R2 | Determines price adjustment 
    let price_adjustment = match rolls_list[1] {
        1..=3 => {
            println!("Your NFT's price stays at its original.");
            1.0
        }
        4..=6 => {
            println!("Your NFT's price has doubled!");
            2.0
        }
        _ => unreachable!(),
    };

    // if the purchase ran, then do NFT shit 
    if purchase_success {
        nft.limit = nft_counter;
        if let Some(parsed_price) = price {
            nft.value = parsed_price;
        }
        nft.trash = false;

        let nft_price = (nft.value as f64 * price_adjustment) as u32;

        // end results
        println!("NFT Created Successfully!");
        println!("NFT Price: {} <USD>", nft_price);
        println!("Time Spent: {} hours", hrs);
        println!("Total Hours: {} hours", total_hours)
    } else {
        println!("\nNo NFT created due to failed purchase!\n");
    }

    purchase_success
}

pub fn open_market(player: &PlayerData) {
    let mut market: Vec<Stock> = Vec::new();
    let mut menu = Table::new();

    // Market items
    market.push(Stock {
        title: String::from("Red Bull"),
        price: 4,
        desc: String::from("A very tasty energy drink"),
        buff: String::from("5 NFTs Buff"),
        stock: StockConditions::InStock,
    });

    market.push(Stock {
        title: String::from("Monster"),
        price: 3,
        desc: String::from("Nasty ass energy drink"),
        buff: String::from("3 NFTs Buff"),
        stock: StockConditions::InStock,
    });
    
    // Display
    println!("\nWelcome to the Market {}!", player.username);
    println!("Here are your options:\n");

    // Header
    menu.add_row(row!["Item Name", "Price (USD)", "Description", "Effect", "Stock"]);
    // Options
    for item in &market {
        let stock_display = match &item.stock {
            StockConditions::InStock => "True".to_string(),
            StockConditions::OutOfStock => "0".to_string(),
            StockConditions::Limit(count) => count.to_string(),
        };

        menu.add_row(row![&item.title, &item.price, &item.desc, &item.buff, &stock_display]);
    }
    menu.printstd();
}
