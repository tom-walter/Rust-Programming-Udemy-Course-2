#[macro_use]
extern crate serde_derive;

use std::io;
use std::io::{Write};
use std::process;

mod blockchain;


fn main() {

    // read inputs from user
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    println!("Input a miner address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_addr);

    println!("Input a difficulty: ");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("Difficulty must be integer");

    println!("Generating genesis block: ");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu: ");
        println!("1. New Transaction");
        println!("2. Mine a Block");
        println!("3. Change Difficulty");
        println!("4. Change Reward");
        println!("0. Exit");
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);

        match choice.trim().parse().unwrap() {
            0 => {
                println!("Exiting");
                process::exit(0);
            },
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                println!("Enter sender address:");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);

                println!("Enter receiver address:");
                io::stdout().flush();
                io::stdin().read_line(&mut receiver);

                println!("Enter amount:");
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap()
                );

                match res {
                    true => println!("transaction added"),
                    false => println!("transaction failed"),
                };
            },
            2 => {
                println!("Generating block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("block generating success"),
                    false => println!("block generating failure"),
                }
            },
            3 => {
                let mut new_diff = String::new();
                println!("Please enter new Difficulty: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_diff);
                let res = chain.update_difficulty(
                    new_diff.trim().parse().unwrap()
                );
                match res {
                    true => println!("difficulty update success"),
                    false => println!("difficulty update failure"),
                }
            },
            4 => {
                let mut new_reward = String::new();
                println!("Please enter new Reward: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);
                let res = chain.update_reward(
                    new_reward.trim().parse().unwrap()
                );
                match res {
                    true => println!("reward update success"),
                    false => println!("reward update failure"),
                }
            },
            _ => {println!{"Invalid Choice. Try again: "};}
        } 
    }
}
