use std::io;
use std::str::FromStr;
mod petstuff;
mod game_commands;

use petstuff::VirtuaPet;
use petstuff::PetExp;

pub fn start_game() {
    let mut your_pet = petstuff::VirtuaPet::create_pet();

    input_player_command(&mut your_pet)
}

pub fn input_player_command(pet: &mut VirtuaPet) {

    let mut pinput = String::new();
    let mut pet_xp = PetExp {
        attack: 0,
        defense: 0,
        speed: 0,
    };

    'game: loop {
        println!("Commands: >[S] Show Status >[F] Test Fight >[Q] Quit Game");
        match io::stdin().read_line(&mut pinput) { 
            Ok(_) => {},
            Err(_) => { println!("Failed to read input"); } 
        }
        let s = match char::from_str(&pinput[..1]) {
            Ok(s) => s, 
            Err(_) => '0',
        };
        pinput.clear();
           
        match s {
            's' => game_commands::show_status(pet, &pet_xp),
            'q' => { break 'game; }
            'f' => game_commands::start_test_fight(pet, &mut pet_xp),
            'x' => game_commands::save_game(pet, &pet_xp),
            '\n' => {},
            _ => println!("Didn't recieve command"),
        }
    }
}

