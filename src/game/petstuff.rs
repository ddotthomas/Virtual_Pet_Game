use rand::seq::IteratorRandom;

pub struct PetStats {
    pub name: String,
    pub health: f32,
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
    pub is_players: bool,
}

pub enum VirtuaPet {
    Toad(PetStats),
    Bird(PetStats),
    Snake(PetStats),
    Kitty(PetStats),
    Dog(PetStats),
}

impl VirtuaPet {
    pub fn is_players_pet(self: &VirtuaPet) -> bool {
        match self {
            VirtuaPet::Dog(stats) => stats.is_players,
            VirtuaPet::Kitty(stats) => stats.is_players,
            VirtuaPet::Bird(stats) => stats.is_players,
            VirtuaPet::Snake(stats) => stats.is_players,
            VirtuaPet::Toad(stats) => stats.is_players,
        }
    }

    pub fn return_stat(self: &VirtuaPet, stat: &str) -> f32 { //returns any stat you need based on passed string for any pet
        match stat {
            "health" => match self {
                VirtuaPet::Dog(stats) => stats.health,
                VirtuaPet::Kitty(stats) => stats.health,
                VirtuaPet::Bird(stats) => stats.health,
                VirtuaPet::Snake(stats) => stats.health,
                VirtuaPet::Toad(stats) => stats.health,
            }
            "attack" => match self {
                VirtuaPet::Dog(stats) => stats.attack,
                VirtuaPet::Kitty(stats) => stats.attack,
                VirtuaPet::Bird(stats) => stats.attack,
                VirtuaPet::Snake(stats) => stats.attack,
                VirtuaPet::Toad(stats) => stats.attack,
            }
            "defense" => match self {
                VirtuaPet::Dog(stats) => stats.defense,
                VirtuaPet::Kitty(stats) => stats.defense,
                VirtuaPet::Bird(stats) => stats.defense,
                VirtuaPet::Snake(stats) => stats.defense,
                VirtuaPet::Toad(stats) => stats.defense,
            }
            "speed" => match self {
                VirtuaPet::Dog(stats) => stats.speed,
                VirtuaPet::Kitty(stats) => stats.speed,
                VirtuaPet::Bird(stats) => stats.speed,
                VirtuaPet::Snake(stats) => stats.speed,
                VirtuaPet::Toad(stats) => stats.speed,
            }
            &_ => 666.
        }
    }
    
    pub fn return_name(self: &VirtuaPet) -> &String { //returns the name, keeps me from have a generic type return stat and having to declare the type asked for everytime
        match self {
            VirtuaPet::Dog(stats) => &stats.name,
            VirtuaPet::Kitty(stats) => &stats.name,
            VirtuaPet::Bird(stats) => &stats.name,
            VirtuaPet::Snake(stats) => &stats.name,
            VirtuaPet::Toad(stats) => &stats.name,
        }
    }
    
    pub fn level_up(self: &mut VirtuaPet, stat: &str) {
        match stat {
            "speed" => { 
                match self {
                    VirtuaPet::Dog(stats) => { stats.speed += 1.; println!("{} leveled up their speed to {}!", self.return_name(), self.return_stat("speed"))},
                    VirtuaPet::Kitty(stats) => { stats.speed += 1.; println!("{} leveled up their speed to {}!", self.return_name(), self.return_stat("speed"))},
                    VirtuaPet::Bird(stats) => { stats.speed += 1.; println!("{} leveled up their speed to {}!", self.return_name(), self.return_stat("speed"))},
                    VirtuaPet::Snake(stats) => { stats.speed += 1.; println!("{} leveled up their speed to {}!", self.return_name(), self.return_stat("speed"))},
                    VirtuaPet::Toad(stats) => { stats.speed += 1.; println!("{} leveled up their speed to {}!", self.return_name(), self.return_stat("speed"))},
                } 
            },
            "defense" => {
                match self {
                    VirtuaPet::Dog(stats) => { stats.defense += 1.; println!("{} leveled up their defense to {}!", self.return_name(), self.return_stat("defense"))},
                    VirtuaPet::Kitty(stats) => { stats.defense += 1.; println!("{} leveled up their defense to {}!", self.return_name(), self.return_stat("defense"))},
                    VirtuaPet::Bird(stats) => { stats.defense += 1.; println!("{} leveled up their defense to {}!", self.return_name(), self.return_stat("defense"))},
                    VirtuaPet::Snake(stats) => { stats.defense += 1.; println!("{} leveled up their defense to {}!", self.return_name(), self.return_stat("defense"))},
                    VirtuaPet::Toad(stats) => { stats.defense += 1.; println!("{} leveled up their defense to {}!", self.return_name(), self.return_stat("defense"))},
                } 
            },
            "attack" => {
                match self {
                    VirtuaPet::Dog(stats) => { stats.attack += 1.; println!("{} leveled up their attack to {}!", self.return_name(), self.return_stat("attack"))},
                    VirtuaPet::Kitty(stats) => { stats.attack += 1.; println!("{} leveled up their attack to {}!", self.return_name(), self.return_stat("attack"))},
                    VirtuaPet::Bird(stats) => { stats.attack += 1.; println!("{} leveled up their attack to {}!", self.return_name(), self.return_stat("attack"))},
                    VirtuaPet::Snake(stats) => { stats.attack += 1.; println!("{} leveled up their attack to {}!", self.return_name(), self.return_stat("attack"))},
                    VirtuaPet::Toad(stats) => { stats.attack += 1.; println!("{} leveled up their attack to {}!", self.return_name(), self.return_stat("attack"))},
                } 
            },
            _ => {},
        }
    }

    pub fn create_pet() -> Self {
        let mut rng = rand::thread_rng();
        let petlist = ["Toad", "Bird", "Snake", "Kitty", "Dog"];
        let cpet = match petlist.iter().choose(&mut rng) { 
            Some(&"Toad") => VirtuaPet::Toad(PetStats {name: String::from("Toad"), health: 175. ,  attack: 1. , defense: 3. , speed: 1. , is_players: true,}),  //Tank
            Some(&"Bird") => VirtuaPet::Bird(PetStats {name: String::from("Bird"), health: 75. ,  attack: 2. , defense: 0.1 , speed: 4. , is_players: true,}),  //Fast
            Some(&"Snake") => VirtuaPet::Snake(PetStats {name: String::from("Snake"), health: 100. ,  attack: 3. , defense: 1. , speed: 2. , is_players: true,}),  //Good at attack
            Some(&"Kitty") => VirtuaPet::Kitty(PetStats {name: String::from("Kitty"), health: 105. ,  attack: 2. , defense: 4. , speed: 1. , is_players: true,}), //High defense 
            Some(&"Dog") => VirtuaPet::Dog(PetStats {name: String::from("Dog"), health: 130. ,  attack: 2. , defense: 2. , speed: 2. , is_players: true,}), // Good all around
            Some(_) => VirtuaPet::Toad(PetStats {name: String::from("Toad"), health: 175. ,  attack: 1. , defense: 3. , speed: 1. , is_players: true,}),
            None => VirtuaPet::Toad(PetStats {name: String::from("Toad"), health: 175. ,  attack: 1. , defense: 3. , speed: 1. , is_players: true,}),
        };
        cpet
            
            //let new_pets = [ 
            //    VirtuaPet::Toad(PetStats { health: 150. ,  attack: 1. , defense: 4. , speed: 1. ,}),
            //    VirtuaPet::Bird(PetStats { health: 75. ,  attack: 2. , defense: 0. , speed: 4. ,}),
            //    VirtuaPet::Snake(PetStats { health: 100. ,  attack: 3. , defense: 1. , speed: 2. ,}),
            //    VirtuaPet::Kitty(PetStats { health: 125. ,  attack: 2. , defense: 3. , speed: 1. ,}),
            //    VirtuaPet::Dog(PetStats { health: 130. ,  attack: 2. , defense: 2. , speed: 2. ,}),
            //];
            //let mut rng = rand::thread_rng();
            //let cpet = match new_pets.clone().iter().choose(&mut rng) {
            //    Some(result) => *result,
            //    None => VirtuaPet::Toad(PetStats { health: 150. ,  attack: 1. , defense: 4. , speed: 1. ,}),
            //};
            //cpet
    }
}


pub struct PetExp {
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
}

impl PetExp {
    pub fn gain_xp(self: &mut PetExp,stat: &str, pet: &mut VirtuaPet,xp: i32) { 
        match stat {                      // if the amount of speed experience, is more that 10 to the power of the current level of your character, add a level to that character
            "speed" => { self.speed += xp; if self.speed > match 3i32.checked_pow(pet.return_stat("speed") as u32) { Some(num) => num, _ => 666 } { pet.level_up("speed") }  },
            "attack" => { self.attack += xp; if self.attack > match 3i32.checked_pow(pet.return_stat("attack") as u32) { Some(num) => num, _ => 666 } { pet.level_up("attack") }},
            "defense" => { self.defense += xp; if self.defense > match 3i32.checked_pow(pet.return_stat("defense") as u32) { Some(num) => num, _ => 666 } { pet.level_up("defense") }},
            &_ => {},
        }

    }
}

