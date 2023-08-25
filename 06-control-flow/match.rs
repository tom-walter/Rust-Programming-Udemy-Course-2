use crate::CardSuit::*;

enum CardSuit{
    Spade,
    Heart,
    Diamond,
    Club
}

fn print_choice(choice: CardSuit){
    match choice{
        Spade => {println!("♠");},
        Heart => {println!("♥");},
        Diamond => {println!("♦");},
        Club => {println!("♣");},
    }
}

fn country(code: u32){
    let country = match code {
        33 => "FR",
        34 => "ES",
        44 => "UK",
        49 => "DE",
        1..=100 => "unknown",
        _ => "invalid",
    };
    println!("Country is {}", country);
}

fn main(){
    print_choice(Spade);
    print_choice(Heart);
    print_choice(Diamond);
    print_choice(Club);

    country(44);
    country(125);
}