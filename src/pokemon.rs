use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypePokemon {
    Feu,
    Eau,
    Plante,
    Electrik,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Genre {
    Male,
    Femelle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pokemon {
    pub nom: String,
    pub niveau: u32,
    pub xp: u32,
    pub type_pokemon: TypePokemon,
    pub genre: Genre,
}

impl Pokemon {
    pub fn gagner_xp(&mut self, quantite: u32) {
        self.xp += quantite;
        while self.xp >= 100 {
            self.niveau += 1;
            self.xp = 0;
            println!("{} passe au niveau {}!", self.nom, self.niveau);
        }
    }

    pub fn peut_se_reproduire(&self, autre: &Pokemon) -> bool {
        self.type_pokemon == autre.type_pokemon &&
        self.genre != autre.genre &&
        self.niveau >= 5 &&
        autre.niveau >= 5
    }
}

impl fmt::Display for Pokemon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "{} | Niveau: {} | XP: {} | Type: {:?} | Genre: {:?}",
            self.nom, self.niveau, self.xp, self.type_pokemon, self.genre
        )
    }
}
