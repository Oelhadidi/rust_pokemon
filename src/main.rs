mod pokemon;
mod elevage;

use elevage::Elevage;
use std::io;

fn main() {
    let mut elevage = Elevage::new();
    elevage.charger("elevage.json");

    loop {
        println!("\n--- Menu ---");
        println!("1. Ajouter un Pokémon");
        println!("2. Afficher tous les Pokémon");
        println!("3. Entraîner tous les Pokémon");
        println!("4. Tenter une reproduction");
        println!("5. Sauvegarder");
        println!("6. Quitter");

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        let choix = choix.trim();

        match choix {
            "1" => elevage.ajouter_pokemon_interactif(),
            "2" => elevage.afficher_tous(),
            "3" => elevage.entrainer_tous(),
            "4" => elevage.reproduire_interactif(),
            "5" => elevage.sauvegarder("elevage.json"),
            "6" => {
                elevage.sauvegarder("elevage.json");
                println!("Élevage sauvegardé. À bientôt !");
                break;
            }
            _ => println!("Choix invalide, réessaie."),
        }
    }
}

