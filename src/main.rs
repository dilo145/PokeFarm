use std::fs::File;
use std::io::{self, Write, BufReader};
use std::collections::HashMap;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum Type {
    Fire,
    Water,
    Plant,
    Electric,
    Rock,
    Psychic,
    Flying,
    Bug,
    Normal,
    Fighting,
    Poison,
    Ghost,
    Dragon,
    Ice,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Pokemon {
    name: String,
    level: u8,
    #[serde(rename = "type")]
    type_: Type,
    experience: u32,
    gender: Gender,
}

impl Pokemon {
    fn new(name: String, type_: Type, gender: Gender) -> Self {
        Pokemon {
            name,
            level: 1,
            type_,
            experience: 0,
            gender,
        }
    }

    fn gain_xp(&mut self, xp: u32) {
        self.experience += xp;

        let levels_gained = (self.experience / 100) as u8;
        if levels_gained > self.level - 1 {
            let new_levels = levels_gained - (self.level - 1);
            self.level += new_levels;
            println!("{} leveled up to level {}!", self.name, self.level);
        }
    }

    fn display(&self) {
        println!("🔍 {} (Lv. {}) - Type: {:?}", self.name, self.level, self.type_);
        println!("   XP: {}/100 - Gender: {:?}", self.experience % 100, self.gender);
    }

    fn can_breed(&self, other: &Pokemon) -> bool {
        self.type_ == other.type_ &&
            self.gender != other.gender &&
            self.level >= 5 &&
            other.level >= 5
    }
}

struct Breeding {
    pokemons: Vec<Pokemon>,
    breeding_counter: HashMap<String, u32>,
}

impl Breeding {
    fn new() -> Self {
        Breeding {
            pokemons: Vec::new(),
            breeding_counter: HashMap::new(),
        }
    }

    fn add_pokemon(&mut self, pokemon: Pokemon) {
        println!("➕ {} has joined the breeding!", pokemon.name);
        self.pokemons.push(pokemon);
    }

    fn display_all_pokemons(&self) {
        println!("\n===== POKEMON LIST ({}) =====", self.pokemons.len());
        for (i, pokemon) in self.pokemons.iter().enumerate() {
            println!("#{}", i + 1);
            pokemon.display();
            println!();
        }
    }

    fn train_all_pokemons(&mut self, xp: u32) {
        println!("\n===== TRAINING SESSION =====");
        println!("All Pokémon gain {} XP!", xp);

        for pokemon in &mut self.pokemons {
            pokemon.gain_xp(xp);
        }
    }

    fn train_single_pokemon(&mut self, index: usize, xp: u32) -> Result<(), String> {
        if index >= self.pokemons.len() {
            return Err("Invalid Pokémon index".to_string());
        }

        println!("\n===== INDIVIDUAL TRAINING =====");
        println!("{} gains {} XP!", self.pokemons[index].name, xp);

        self.pokemons[index].gain_xp(xp);
        Ok(())
    }

    fn try_breeding(&mut self, index1: usize, index2: usize) -> Result<(), String> {
        if index1 >= self.pokemons.len() || index2 >= self.pokemons.len() {
            return Err("Invalid Pokémon indices".to_string());
        }

        let pokemon1 = self.pokemons[index1].clone();
        let pokemon2 = self.pokemons[index2].clone();

        if pokemon1.can_breed(&pokemon2) {
            let mut rng = rand::thread_rng();

            let key = format!("{}_{}", pokemon1.name, pokemon2.name);
            let count = self.breeding_counter.entry(key).or_insert(0);
            *count += 1;

            let baby_name = if rng.gen_bool(0.5) {
                format!("Baby {} #{}", pokemon1.name, count)
            } else {
                format!("Baby {} #{}", pokemon2.name, count)
            };

            let baby_gender = if rng.gen_bool(0.5) {
                Gender::Male
            } else {
                Gender::Female
            };

            let baby = Pokemon::new(
                baby_name,
                pokemon1.type_.clone(),
                baby_gender,
            );

            println!("\n🎉 An egg has hatched! A new Pokémon has been born!");
            baby.display();

            self.pokemons.push(baby);
            Ok(())
        } else {
            Err("These Pokémon cannot breed together.".to_string())
        }
    }

    fn sort_by_level(&mut self) {
        self.pokemons.sort_by(|a, b| b.level.cmp(&a.level));
        println!("Pokémon sorted by level (descending)");
    }

    fn sort_by_type(&mut self) {
        self.pokemons.sort_by(|a, b| format!("{:?}", a.type_).cmp(&format!("{:?}", b.type_)));
        println!("Pokémon sorted by type");
    }

    fn save_to_json(&self, path: &str) -> io::Result<()> {
        let pokemons_json: Vec<PokemonJson> = self.pokemons.iter().map(|p| {
            PokemonJson {
                name: p.name.clone(),
                level: p.level,
                type_str: format!("{:?}", p.type_),
                experience: p.experience,
                gender: format!("{:?}", p.gender),
            }
        }).collect();

        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, &pokemons_json)?;
        println!("Data successfully saved to {}", path);
        Ok(())
    }
}

fn load_pokemons_from_json(file_path: &str) -> Vec<Pokemon> {
    let file = File::open(file_path).expect("Failed to open JSON file");
    let reader = BufReader::new(file);

    let pokemons: Vec<PokemonJson> = serde_json::from_reader(reader)
        .expect("Failed to parse JSON data");

    pokemons.into_iter().map(|p| p.into_pokemon()).collect()
}

#[derive(Debug, Deserialize, Serialize)]
struct PokemonJson {
    name: String,
    level: u8,
    #[serde(rename = "type")]
    type_str: String,
    experience: u32,
    gender: String,
}

impl PokemonJson {
    fn into_pokemon(self) -> Pokemon {
        let type_ = match self.type_str.as_str() {
            "Fire" => Type::Fire,
            "Water" => Type::Water,
            "Grass" => Type::Plant,
            "Electric" => Type::Electric,
            "Rock" => Type::Rock,
            "Psychic" => Type::Psychic,
            "Flying" => Type::Flying,
            "Bug" => Type::Bug,
            "Normal" => Type::Normal,
            "Fighting" => Type::Fighting,
            "Poison" => Type::Poison,
            "Ghost" => Type::Ghost,
            "Dragon" => Type::Dragon,
            "Ice" => Type::Ice,
            _ => Type::Normal,
        };

        let gender = match self.gender.as_str() {
            "Male" => Gender::Male,
            "Female" => Gender::Female,
            _ => Gender::Male,
        };

        Pokemon {
            name: self.name,
            level: self.level,
            type_,
            experience: self.experience,
            gender,
        }
    }
}

fn menu() {
    println!("\n===== BREEDING MENU =====");
    println!("1. Display all Pokémon");
    println!("2. Add a new Pokémon");
    println!("3. Train all Pokémon");
    println!("4. Train a single Pokémon");
    println!("5. Try breeding");
    println!("6. Sort by level");
    println!("7. Sort by type");
    println!("0. Quit");
    print!("Your choice: ");
    io::stdout().flush().unwrap();
}

fn main() {
    let mut breeding = Breeding::new();

    let starting_pokemons = load_pokemons_from_json("src/pokemons_data.json");
    for pokemon in starting_pokemons {
        breeding.pokemons.push(pokemon);
    }

    let mut input = String::new();

    loop {
        menu();

        input.clear();
        io::stdin().read_line(&mut input).expect("Reading error");

        match input.trim() {
            "1" => {
                breeding.display_all_pokemons();
                if let Err(e) = breeding.save_to_json("src/pokemons_data.json") {
                    println!("Error saving data: {}", e);
                }
            },

            "2" => {
                println!("\n--- Add a Pokémon ---");

                print!("Name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Reading error");
                let name = name.trim().to_string();

                println!("Available types:");
                println!("1. Fire    2. Water      3. Plant    4. Electric");
                println!("5. Rock   6. Psychic   7. Flying    8. Bug");
                println!("9. Normal 10. Fighting 11. Poison   12. Ghost");
                println!("13. Dragon 14. Ice");

                print!("Choose the type (1-14): ");
                io::stdout().flush().unwrap();
                let mut type_choice = String::new();
                io::stdin().read_line(&mut type_choice).expect("Reading error");

                let type_ = match type_choice.trim().parse::<u8>() {
                    Ok(1) => Type::Fire,
                    Ok(2) => Type::Water,
                    Ok(3) => Type::Plant,
                    Ok(4) => Type::Electric,
                    Ok(5) => Type::Rock,
                    Ok(6) => Type::Psychic,
                    Ok(7) => Type::Flying,
                    Ok(8) => Type::Bug,
                    Ok(9) => Type::Normal,
                    Ok(10) => Type::Fighting,
                    Ok(11) => Type::Poison,
                    Ok(12) => Type::Ghost,
                    Ok(13) => Type::Dragon,
                    Ok(14) => Type::Ice,
                    _ => {
                        println!("Invalid type, using Normal type by default");
                        Type::Normal
                    }
                };

                print!("Gender (1: Male, 2: Female): ");
                io::stdout().flush().unwrap();
                let mut gender_choice = String::new();
                io::stdin().read_line(&mut gender_choice).expect("Reading error");

                let gender = match gender_choice.trim().parse::<u8>() {
                    Ok(1) => Gender::Male,
                    Ok(2) => Gender::Female,
                    _ => {
                        println!("Invalid gender, using Male by default");
                        Gender::Male
                    }
                };

                let pokemon = Pokemon::new(name, type_, gender);
                breeding.add_pokemon(pokemon);
                if let Err(e) = breeding.save_to_json("src/pokemons_data.json") {
                    println!("Error saving data: {}", e);
                }
            },

            "3" => {
                print!("Amount of XP to gain: ");
                io::stdout().flush().unwrap();
                let mut xp_input = String::new();
                io::stdin().read_line(&mut xp_input).expect("Reading error");

                match xp_input.trim().parse::<u32>() {
                    Ok(xp) => {
                        breeding.train_all_pokemons(xp);
                        if let Err(e) = breeding.save_to_json("src/pokemons_data.json") {
                            println!("Error saving data: {}", e);
                        }
                    },
                    Err(_) => println!("Invalid amount of XP")
                }
            },

            "4" => {
                breeding.display_all_pokemons();

                print!("First Pokémon index: ");
                io::stdout().flush().unwrap();
                let mut idx1 = String::new();
                io::stdin().read_line(&mut idx1).expect("Reading error");

                print!("Amount of XP to gain: ");
                io::stdout().flush().unwrap();
                let mut xp_input = String::new();
                io::stdin().read_line(&mut xp_input).expect("Reading error");

                match (idx1.trim().parse::<usize>(), xp_input.trim().parse::<u32>()) {
                    (Ok(i1), Ok(xp)) => {
                        match breeding.train_single_pokemon(i1 - 1, xp) {
                            Ok(_) => {
                                if let Err(e) = breeding.save_to_json("src/pokemons_data.json") {
                                    println!("Error saving data: {}", e);
                                }
                            },
                            Err(e) => println!("Error: {}", e)
                        }
                    },
                    _ => println!("Invalid indices")
                }
            },

            "5" => {
                breeding.display_all_pokemons();

                print!("First Pokémon index: ");
                io::stdout().flush().unwrap();
                let mut idx1 = String::new();
                io::stdin().read_line(&mut idx1).expect("Reading error");

                print!("Second Pokémon index: ");
                io::stdout().flush().unwrap();
                let mut idx2 = String::new();
                io::stdin().read_line(&mut idx2).expect("Reading error");

                match (idx1.trim().parse::<usize>(), idx2.trim().parse::<usize>()) {
                    (Ok(i1), Ok(i2)) => {
                        match breeding.try_breeding(i1 - 1, i2 - 1) {
                            Ok(_) => {
                                if let Err(e) = breeding.save_to_json("src/pokemons_data.json") {
                                    println!("Error saving data: {}", e);
                                }
                            },
                            Err(e) => println!("Error: {}", e)
                        }
                    },
                    _ => println!("Invalid indices")
                }
            },

            "6" => {
                breeding.sort_by_level();
                if let Err(e) = breeding.save_to_json("src/pokemons_data.json") {
                    println!("Error saving data: {}", e);
                }
            },

            "7" => {
                breeding.sort_by_type();
                if let Err(e) = breeding.save_to_json("src/pokemons_data.json") {
                    println!("Error saving data: {}", e);
                }
            },

            "8" => {
                if let Err(e) = breeding.save_to_json("src/pokemons_data.json") {
                    println!("Error saving data: {}", e);
                }
            },

            "0" => {
                println!("Goodbye!");
                if let Err(e) = breeding.save_to_json("src/pokemons_data.json") {
                    println!("Error saving data: {}", e);
                }
                break;
            },

            _ => println!("Invalid option")
        }
    }
}
