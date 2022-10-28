use std::{io, str::FromStr, path::Path, env};
mod petstuff;
mod game_commands;

use petstuff::VirtuaPet;
use petstuff::PetExp;

pub fn start_game() {
    let (mut your_pet, mut pet_xp) = check_for_save();
    input_player_command(&mut your_pet, &mut pet_xp)
}

pub fn input_player_command(pet: &mut VirtuaPet, pet_xp: &mut PetExp) {

    let mut pinput = String::new();

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
            's' => game_commands::show_status(pet, pet_xp),
            'q' => { break 'game; }
            'f' => game_commands::start_test_fight(pet, pet_xp),
            'x' => game_commands::save_game(pet, pet_xp),
            //'n' => game_commands::new_pet(pet, pet_xp), TODO
            '\n' => {},
            _ => println!("Didn't recieve command"),
        }
    }
}

fn check_for_save() -> (VirtuaPet, PetExp) { //Checks if the save file exists or not, and if it can't be read or if the HOME var doesn't work a new save is created instead
    let save_path = match env::var("HOME") {
        Ok(s) => s,
        Err(_) => { 
            println!("$HOME wasn't set, save couldn't be loaded"); 
            return ( VirtuaPet::create_pet(), PetExp {
                attack: 0,
                defense: 0,
                speed: 0,
            } ) },
    } + "/.config/virtuapetsave.json";
    if Path::new(&save_path).exists() {
        game_commands::load_save()
     }
    else {
        ( VirtuaPet::create_pet(), PetExp {
            attack: 0,
            defense: 0,
            speed: 0,
            } 
        )
    }
}
