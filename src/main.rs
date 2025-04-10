use std::io::{self, Write};
use rand::Rng;

// ****************************
// Partie 1 : Définition des types
// ****************************

#[derive(Debug, PartialEq, Clone)]
enum PokemonType {
    Feu,
    Eau,
    Plante,
    Electrik,
    // D'autres types peuvent être ajoutés ici
}

#[derive(Debug, PartialEq, Clone)]
enum Genre {
    Male,
    Femelle,
}

#[derive(Debug, Clone)]
struct Pokemon {
    nom: String,
    niveau: u32,
    xp: u32,
    p_type: PokemonType,
    genre: Genre,
}

impl Pokemon {
    /// Crée un nouveau Pokémon (niveau 1 et 0 XP par défaut)
    fn new(nom: &str, p_type: PokemonType, genre: Genre) -> Self {
        Pokemon {
            nom: nom.to_string(),
            niveau: 1,
            xp: 0,
            p_type,
            genre,
        }
    }

    /// Fait gagner de l'expérience au Pokémon. À chaque 100 XP, le niveau augmente.
    fn gagner_xp(&mut self, xp_gagne: u32) {
        self.xp += xp_gagne;
        while self.xp >= 100 {
            self.xp -= 100;
            self.niveau += 1;
            println!("{} a évolué vers le niveau {} !", self.nom, self.niveau);
        }
    }

    /// Affiche l'ensemble des informations du Pokémon.
    fn afficher(&self) {
        println!("Nom    : {}", self.nom);
        println!("Niveau : {}", self.niveau);
        println!("XP     : {}", self.xp);
        println!("Type   : {:?}", self.p_type);
        println!("Genre  : {:?}", self.genre);
    }

    /// Vérifie la compatibilité de reproduction entre deux Pokémon.
    /// Conditions : même type, niveau ≥ 5 et genres opposés.
    fn peut_se_reproduire_avec(&self, autre: &Pokemon) -> bool {
        let niveau_minimum = 5;
        if self.p_type != autre.p_type {
            return false;
        }
        if self.niveau < niveau_minimum || autre.niveau < niveau_minimum {
            return false;
        }
        if self.genre == autre.genre {
            return false;
        }
        true
    }
}

// ****************************
// Fonctions et comportements supplémentaires
// ****************************

/// Tente la reproduction de deux Pokémon. Retourne Some(nouveau Pokémon) en cas de succès.
fn tenter_reproduction(p1: &Pokemon, p2: &Pokemon) -> Option<Pokemon> {
    if p1.peut_se_reproduire_avec(p2) {
        let mut rng = rand::thread_rng();
        let genre_nouveau = if rng.gen_bool(0.5) {
            Genre::Male
        } else {
            Genre::Femelle
        };

        let nouveau_pokemon = Pokemon {
            nom: "Mystere".to_string(),
            niveau: 1,
            xp: 0,
            p_type: p1.p_type.clone(),
            genre: genre_nouveau,
        };
        println!("Reproduction réussie entre {} et {} !", p1.nom, p2.nom);
        Some(nouveau_pokemon)
    } else {
        println!("Les Pokémon {} et {} ne peuvent pas se reproduire.", p1.nom, p2.nom);
        None
    }
}

// ****************************
// Gestion de l'élevage
// ****************************

struct Elevage {
    pokemons: Vec<Pokemon>,
}

impl Elevage {
    /// Crée un nouvel élevage vide.
    fn new() -> Self {
        Elevage {
            pokemons: Vec::new(),
        }
    }

    /// Ajoute un Pokémon dans l'élevage.
    fn ajouter(&mut self, pokemon: Pokemon) {
        self.pokemons.push(pokemon);
    }

    /// Affiche tous les Pokémon de l'élevage.
    fn afficher(&self) {
        if self.pokemons.is_empty() {
            println!("L'élevage est vide.");
        } else {
            for (i, pokemon) in self.pokemons.iter().enumerate() {
                println!("-------------------------");
                println!("Index: {}", i);
                pokemon.afficher();
            }
            println!("-------------------------");
        }
    }

    /// Entraîne tous les Pokémon en leur faisant gagner un certain XP.
    fn entrainer(&mut self, xp: u32) {
        for pokemon in &mut self.pokemons {
            pokemon.gagner_xp(xp);
        }
    }

    /// Tente la reproduction entre deux Pokémon choisis par leur indice.
    fn tenter_reproduction_par_indices(&mut self, indice1: usize, indice2: usize) {
        if indice1 >= self.pokemons.len() || indice2 >= self.pokemons.len() {
            println!("Indices invalides pour la reproduction.");
            return;
        }
        let p1 = &self.pokemons[indice1];
        let p2 = &self.pokemons[indice2];
        if let Some(nouveau) = tenter_reproduction(p1, p2) {
            self.ajouter(nouveau);
        }
    }

    /// Trie les Pokémon par niveau.
    fn trier_par_niveau(&mut self) {
        self.pokemons.sort_by(|a, b| a.niveau.cmp(&b.niveau));
        println!("L'élevage a été trié par niveau.");
    }
}

// ****************************
// Fonctions utilitaires pour le menu
// ****************************

/// Affiche le menu principal.
fn afficher_menu() {
    println!("\n------ MENU ------");
    println!("1. Afficher tous les Pokémon");
    println!("2. Entraîner tous les Pokémon");
    println!("3. Tenter une reproduction");
    println!("4. Ajouter un Pokémon");
    println!("5. Trier les Pokémon par niveau");
    println!("6. Quitter");
    print!("Votre choix : ");
    io::stdout().flush().expect("Erreur de flush");
}

/// Lit une ligne entrée par l'utilisateur et la retourne.
fn lire_entree() -> String {
    let mut entree = String::new();
    io::stdin()
        .read_line(&mut entree)
        .expect("Erreur lors de la lecture de l'entrée");
    entree.trim().to_string()
}

/// Permet d'ajouter un Pokémon via le menu en demandant les informations à l'utilisateur.
fn ajouter_pokemon_menu(elevage: &mut Elevage) {
    println!("Entrez le nom du Pokémon :");
    let nom = lire_entree();

    println!("Choisissez le type du Pokémon :");
    println!("1. Feu");
    println!("2. Eau");
    println!("3. Plante");
    println!("4. Electrik");
    print!("Votre choix : ");
    io::stdout().flush().unwrap();
    let type_choice = lire_entree();
    let p_type = match type_choice.as_str() {
        "1" => PokemonType::Feu,
        "2" => PokemonType::Eau,
        "3" => PokemonType::Plante,
        "4" => PokemonType::Electrik,
        _ => {
            println!("Type non reconnu, valeur par défaut : Feu");
            PokemonType::Feu
        }
    };

    println!("Choisissez le genre du Pokémon :");
    println!("1. Male");
    println!("2. Femelle");
    print!("Votre choix : ");
    io::stdout().flush().unwrap();
    let genre_choice = lire_entree();
    let genre = match genre_choice.as_str() {
        "1" => Genre::Male,
        "2" => Genre::Femelle,
        _ => {
            println!("Genre non reconnu, valeur par défaut : Male");
            Genre::Male
        }
    };

    let nouveau = Pokemon::new(&nom, p_type, genre);
    elevage.ajouter(nouveau);
    println!("Le Pokémon a été ajouté à l'élevage.");
}

// ****************************
// Fonction main() avec menu interactif
// ****************************

fn main() {
    let mut elevage = Elevage::new();

    // Création de quelques Pokémon par défaut
    let pikachu = Pokemon::new("Pikachu", PokemonType::Electrik, Genre::Male);
    let salameche = Pokemon::new("Salamèche", PokemonType::Feu, Genre::Male);
    let bulbizarre = Pokemon::new("Bulbizarre", PokemonType::Plante, Genre::Femelle);

    // Ajout initial dans l'élevage
    elevage.ajouter(pikachu);
    elevage.ajouter(salameche);
    elevage.ajouter(bulbizarre);

    // Boucle principale du menu
    loop {
        afficher_menu();
        let choix = lire_entree();

        match choix.as_str() {
            "1" => {
                println!("\nAffichage de tous les Pokémon dans l'élevage :");
                elevage.afficher();
            },
            "2" => {
                println!("Entrez la quantité d'XP à ajouter à chaque Pokémon :");
                let xp_str = lire_entree();
                if let Ok(xp) = xp_str.parse::<u32>() {
                    elevage.entrainer(xp);
                    println!("Les Pokémon ont été entraînés.");
                } else {
                    println!("Veuillez entrer un nombre valide.");
                }
            },
            "3" => {
                println!("Entrez l'indice du premier Pokémon :");
                let idx1_str = lire_entree();
                println!("Entrez l'indice du second Pokémon :");
                let idx2_str = lire_entree();
                if let (Ok(idx1), Ok(idx2)) = (idx1_str.parse::<usize>(), idx2_str.parse::<usize>()) {
                    elevage.tenter_reproduction_par_indices(idx1, idx2);
                } else {
                    println!("Indices invalides.");
                }
            },
            "4" => {
                ajouter_pokemon_menu(&mut elevage);
            },
            "5" => {
                elevage.trier_par_niveau();
            },
            "6" => {
                println!("Au revoir !");
                break;
            },
            _ => {
                println!("Choix non reconnu, veuillez réessayer.");
            }
        }
    }
}