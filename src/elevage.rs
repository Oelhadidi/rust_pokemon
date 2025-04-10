use crate::pokemon::{Pokemon, TypePokemon, Genre};
use std::io;
use std::io::{BufReader, Write};
use std::fs::File;
use serde_json;

pub struct Elevage {
    pub pokemons: Vec<Pokemon>,
}

impl Elevage {
    pub fn new() -> Self {
        Elevage { pokemons: Vec::new() }
    }

    pub fn ajouter_pokemon_interactif(&mut self) {
        let mut nom = String::new();
        println!("Nom du Pokémon :");
        io::stdin().read_line(&mut nom).unwrap();
        let nom = nom.trim().to_string();

        println!("Genre ? (m/f)");
        let mut genre_input = String::new();
        io::stdin().read_line(&mut genre_input).unwrap();
        let genre = match genre_input.trim() {
            "m" => Genre::Male,
            "f" => Genre::Femelle,
            _ => {
                println!("Genre invalide.");
                return;
            }
        };

        println!("Type ? (feu, eau, plante, electrik)");
        let mut type_input = String::new();
        io::stdin().read_line(&mut type_input).unwrap();
        let type_pokemon = match type_input.trim().to_lowercase().as_str() {
            "feu" => TypePokemon::Feu,
            "eau" => TypePokemon::Eau,
            "plante" => TypePokemon::Plante,
            "electrik" => TypePokemon::Electrik,
            _ => {
                println!("Type invalide.");
                return;
            }
        };

        let pokemon = Pokemon {
            nom,
            niveau: 1,
            xp: 0,
            type_pokemon,
            genre,
        };

        self.pokemons.push(pokemon);
        println!("Pokémon ajouté !");
    }
    

    pub fn afficher_tous(&self) {
        if self.pokemons.is_empty() {
            println!("Aucun Pokémon dans l'élevage.");
            return;
        }

        for (i, p) in self.pokemons.iter().enumerate() {
            println!("#{}: {}", i, p);
        }
    }

    pub fn entrainer_tous(&mut self) {
        for p in &mut self.pokemons {
            p.gagner_xp(50);
        }
        println!("Tous les Pokémon ont gagné 50 XP !");
    }

    pub fn reproduire_interactif(&mut self) {
        self.afficher_tous();
        if self.pokemons.len() < 2 {
            println!("Pas assez de Pokémon pour une reproduction.");
            return;
        }

        println!("Choisissez le numéro du premier Pokémon :");
        let mut idx1 = String::new();
        io::stdin().read_line(&mut idx1).unwrap();
        let idx1: usize = match idx1.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Entrée invalide.");
                return;
            }
        };

        println!("Choisissez le numéro du second Pokémon :");
        let mut idx2 = String::new();
        io::stdin().read_line(&mut idx2).unwrap();
        let idx2: usize = match idx2.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Entrée invalide.");
                return;
            }
        };

        if idx1 >= self.pokemons.len() || idx2 >= self.pokemons.len() {
            println!("Index invalide.");
            return;
        }

        let p1 = &self.pokemons[idx1];
        let p2 = &self.pokemons[idx2];

        if p1.peut_se_reproduire(p2) {
            let nouveau_genre = if rand::random() { Genre::Male } else { Genre::Femelle };
            let bebe = Pokemon {
                nom: "Mystere".to_string(),
                niveau: 1,
                xp: 0,
                type_pokemon: p1.type_pokemon.clone(),
                genre: nouveau_genre,
            };
            self.pokemons.push(bebe.clone());
            println!("Un nouveau Pokémon est né !");
            println!("{}", bebe);
        } else {
            println!("Ces Pokémon ne sont pas compatibles pour se reproduire.");
        }
    }

    pub fn sauvegarder(&self, chemin: &str) {
        let file = File::create(chemin);
        match file {
            Ok(mut f) => {
                let json = serde_json::to_string_pretty(&self.pokemons).unwrap();
                f.write_all(json.as_bytes()).unwrap();
                println!("Pokémon sauvegardés dans {}", chemin);
            }
            Err(e) => println!("Erreur de sauvegarde : {}", e),
        }
    }

    pub fn charger(&mut self, chemin: &str) {
        let file = File::open(chemin);
        match file {
            Ok(f) => {
                let reader = BufReader::new(f);
                let pokemons_lus: Result<Vec<Pokemon>, _> = serde_json::from_reader(reader);
                match pokemons_lus {
                    Ok(p) => {
                        self.pokemons = p;
                        println!("Chargé {} Pokémon depuis {}", self.pokemons.len(), chemin);
                    }
                    Err(e) => println!("Erreur de lecture JSON : {}", e),
                }
            }
            Err(e) => println!("Aucun fichier trouvé : {} (début avec élevage vide)", e),
        }
    }
}
