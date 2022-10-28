use super::petstuff::VirtuaPet;
use super::petstuff::PetStats;
use super::petstuff::PetExp;
use serde_derive::{Serialize, Deserialize};
use serde_json::json;
use rand::Rng;
use std::{io, env, str::FromStr};

pub fn show_status(pet: &VirtuaPet, pet_xp: &PetExp) { //pull stats and prints them to screen
    println!("-==Pet Stats==-");
    println!("Pet Name: {}", pet.return_name());
    println!("Health: {}", pet.return_stat("health"));
    println!("Attack: {}", pet.return_stat("attack"));
    println!("Defense: {}", pet.return_stat("defense"));
    println!("Speed: {}", pet.return_stat("speed"));
    //Print Xp and how close to next level each skill is
    println!("\n-==Pet Exp==-");
    println!("Attack: {} Xp to Level up: {}", 
        pet_xp.attack,
        match 3i32.checked_pow(pet.return_stat("attack") as u32) { Some(num) => num, _ => 666 } - pet_xp.attack );
    println!("Defense: {} Xp to Level up: {}",
        pet_xp.defense,
        match 3i32.checked_pow(pet.return_stat("defense") as u32) { Some(num) => num, _ => 666 } - pet_xp.defense );
    println!("Speed: {} Xp to Level up: {}",
        pet_xp.speed,
        match 3i32.checked_pow(pet.return_stat("speed") as u32) { Some(num) => num, _ => 666 } - pet_xp.speed );
}

pub fn start_test_fight(pet: &mut VirtuaPet, pet_xp: &mut PetExp) {  //create a dummy enemy and start a fight, used to test fighting system
    let mut enemy = VirtuaPet::Toad(PetStats {name: String::from("Enemy Toad"),health: 50., attack: 1., defense: 1., speed: 1., is_players: false,});
    //determine who goes first based on speed
    if pet.return_stat("speed") > enemy.return_stat("speed") { 
        start_fight(pet, &mut enemy, pet_xp)
    } 
    else { 
        start_fight(&mut enemy, pet, pet_xp) 
    };
}

fn start_fight(fighter1: &mut VirtuaPet, fighter2: &mut VirtuaPet, player_xp: &mut PetExp) {  //fighting system, should be able to take any two virtual pets and battle them against each other
    let mut health1 = fighter1.return_stat("health"); //create health buffers for fight
    let mut health2 = fighter2.return_stat("health");
    'fight: loop { //start fight, checking after each 'turn' if other player ran out of hp
        deal_damage(fighter1, &mut health2, fighter2, player_xp); //the turns are based on the order the pets are passed to the function, fighter1 then 2
        if health2 < 0. { 
            println!("{} has ran out of health.\n{} has won!", fighter2.return_name(), fighter1.return_name());
            if fighter1.is_players_pet() {
                let xp = (5. / (fighter1.return_stat("speed") / fighter2.return_stat("speed"))) as i32;
                player_xp.gain_xp("speed",fighter1, xp); //gain speed xp when fight finishes
                println!("gained {} speed xp", xp)
            }
            break 'fight;
        }
        else {
            println!("{} has {} health left.", fighter2.return_name(), health2);
        }
        deal_damage(fighter2, &mut health1, fighter1, player_xp);
        if health1 < 0. { 
            println!("{} has ran out of health.\n{} has won!", fighter1.return_name(), fighter2.return_name());
            if fighter2.is_players_pet() {
                let xp = (5. / (fighter2.return_stat("speed") / fighter1.return_stat("speed"))) as i32;
                player_xp.gain_xp("speed",fighter2, xp); //gain speed when fight finishes
                println!("gained {} speed xp", xp)
            }
            break 'fight;
        }
        else {
            println!("{} has {} health left.", fighter1.return_name(), health1);
        }
        println!("Commands: >[F] Fight >[R] Run");  //let the player make decisions between turns, later option to use items or other things
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read input");
        let i = match char::from_str(&input[..1]) {
            Ok(c) => c,
            Err(_) => '0',
        };
        match i {
            'f' => {}, 
            'r' => { break 'fight; },
            _ => {},
        }
    }
}

fn deal_damage(damage_dealer: &mut VirtuaPet, opponent_health: &mut f32, opponent: &mut VirtuaPet, pet_xp: &mut PetExp ) {  //calculates the damage dealt and mutates the health value passed
    
    let dattack = damage_dealer.return_stat("attack");
    let odefense = opponent.return_stat("defense");
    let crit =  if damage_dealer.return_stat("speed") > rand::thread_rng().gen_range(1f32..=100f32) { 1 } //got a crit 
    else { 0 };
    let mut damage = rand::thread_rng().gen_range((dattack*0.6)..=(dattack*1.2)) / if crit == 1 {0.75} else { rand::thread_rng().gen_range((odefense*0.5)..=(odefense*2.2))} ;
    
    damage = if damage <= 0. { 0. } else { damage.round() };
    if crit == 1 { //got a crit
        println!("{} got a Critical Hit for {} damage against {}!", damage_dealer.return_name(), damage * 1.5, opponent.return_name());
        damage *= 1.5;
        if damage_dealer.is_players_pet() {
            let xp = (10. / (damage_dealer.return_stat("speed") / opponent.return_stat("speed"))) as i32;
            pet_xp.gain_xp("speed",damage_dealer, xp); //gain speed xp on crit
            println!("gained {} speed xp", xp)
        }
    }
    else {
        println!("{} hit {} for {} damage.", damage_dealer.return_name(), opponent.return_name(), damage);
    }
    *opponent_health -= damage;
    //now find out if players pet was attacker or not and award defense or attack xp
    if damage_dealer.is_players_pet() {
        let xp = (damage / (damage_dealer.return_stat("attack") / opponent.return_stat("defense") )).ceil() as i32; //all xp should be lower if your pet is much better than the enemy, if the enemy is stronger than gain more xp
        pet_xp.gain_xp("attack", damage_dealer, xp);
        println!("gained {} attack xp", xp);
    }
    else {
        let xp = (damage / (opponent.return_stat("defense") / damage_dealer.return_stat("defense") )).ceil() as i32; //all xp should be lower if your pet is much better than the enemy, if the enemy is stronger than gain more xp
        pet_xp.gain_xp("defense", opponent, xp);
        println!("gained {} defense xp", xp);
    }
}

#[derive(Serialize, Deserialize)]
struct SaveData {
    pet_type: String, //All the string types have to be String in order to move their values from one scope to another
    name:  String,
    health: f32,
    attack: f32,
    defense: f32,
    speed: f32,
    attack_xp: i32,
    defense_xp: i32,
    speed_xp: i32,
}

impl SaveData {
    fn create_data(players_pet: &VirtuaPet, players_xp: &PetExp) -> Self {
        match players_pet {
            VirtuaPet::Bird(stats) => SaveData {
                pet_type: String::from("bird"),
                name: String::from(&stats.name),
                health: stats.health,
                attack: stats.attack,
                defense: stats.defense,
                speed: stats.speed,
                attack_xp: players_xp.attack,
                defense_xp: players_xp.defense,
                speed_xp: players_xp.speed,
            },
            VirtuaPet::Dog(stats) => SaveData {
                pet_type: String::from("dog"),
                name: String::from(&stats.name),
                health: stats.health,
                attack: stats.attack,
                defense: stats.defense,
                speed: stats.speed,
                attack_xp: players_xp.attack,
                defense_xp: players_xp.defense,
                speed_xp: players_xp.speed,
            },
            VirtuaPet::Kitty(stats) => SaveData {
                pet_type: String::from("kitty"),
                name: String::from(&stats.name),
                health: stats.health,
                attack: stats.attack,
                defense: stats.defense,
                speed: stats.speed,
                attack_xp: players_xp.attack,
                defense_xp: players_xp.defense,
                speed_xp: players_xp.speed,
            },
            VirtuaPet::Snake(stats) => SaveData {
                pet_type: String::from("snake"),
                name: String::from(&stats.name),
                health: stats.health,
                attack: stats.attack,
                defense: stats.defense,
                speed: stats.speed,
                attack_xp: players_xp.attack,
                defense_xp: players_xp.defense,
                speed_xp: players_xp.speed,
            },
            VirtuaPet::Toad(stats) => SaveData {
                pet_type: String::from("toad"),
                name: String::from(&stats.name),
                health: stats.health,
                attack: stats.attack,
                defense: stats.defense,
                speed: stats.speed,
                attack_xp: players_xp.attack,
                defense_xp: players_xp.defense,
                speed_xp: players_xp.speed,
            },
        } 
    }
}

pub fn save_game(players_pet: &VirtuaPet, players_xp: &PetExp ) {
    let json_save = json!(SaveData::create_data(players_pet, players_xp));
    let save_location = env::var("HOME").expect("$HOME is not set") + "/.config/virtuapetsave.json";
    std::fs::write(save_location, serde_json::to_string_pretty(&json_save).unwrap()).expect("Should have write permissions");
    println!("Saved Game");
}

pub fn load_save() -> (VirtuaPet, PetExp) {  //This function grabs the data from the save file and returns it as the game data types
    let input = env::var("HOME").expect("$HOME is not set") + "/.config/virtuapetsave.json";
    let save_data = { 
        let save_data_text = std::fs::read_to_string(&input).expect("test");
        serde_json::from_str::<SaveData>(&save_data_text).unwrap()
    };
    let pet = match save_data.pet_type.as_str() {
        "bird" => VirtuaPet::Bird(PetStats {
            name: save_data.name,
            health: save_data.health,
            attack: save_data.attack,
            defense: save_data.defense,
            speed: save_data.speed,
            is_players: true,
            }
        ),
        "dog" => VirtuaPet::Dog(PetStats {
            name: save_data.name,
            health: save_data.health,
            attack: save_data.attack,
            defense: save_data.defense,
            speed: save_data.speed,
            is_players: true,
            }
        ),
        "kitty" => VirtuaPet::Kitty(PetStats {
            name: save_data.name,
            health: save_data.health,
            attack: save_data.attack,
            defense: save_data.defense,
            speed: save_data.speed,
            is_players: true,
            }
        ),
        "snake" => VirtuaPet::Snake(PetStats {
            name: save_data.name,
            health: save_data.health,
            attack: save_data.attack,
            defense: save_data.defense,
            speed: save_data.speed,
            is_players: true,
            }
        ),
        "toad" => VirtuaPet::Toad(PetStats {
            name: save_data.name,
            health: save_data.health,
            attack: save_data.attack,
            defense: save_data.defense,
            speed: save_data.speed,
            is_players: true,
            }
        ),
        _ => VirtuaPet::Toad(PetStats {
            name: String::from("failed to load-save"),
            health: 100.,
            attack: 1.,
            defense: 1.,
            speed: 1.,
            is_players: true,
            }
        ),
    };  
    let xp = PetExp {
        attack: save_data.attack_xp,
        defense: save_data.defense_xp,
        speed: save_data.speed_xp,
    };
    (pet, xp)
}