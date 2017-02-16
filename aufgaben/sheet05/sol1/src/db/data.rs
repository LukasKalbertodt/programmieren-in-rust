use db::types::*;


pub const POKEDEX: &'static [PokemonModel] = &[
    PokemonModel {
        name: "Bulbasaur",
        id: 001,
        type_: PokemonType::Two(Type::Poison, Type::Grass),
        base_stats: Stats {
            hp: 45,
            attack: 49,
            defense: 49,
            special_attack: 65,
            special_defense: 65,
            speed: 45,
        },
        attacks: &[TACKLE],
    },
    PokemonModel {
        name: "Ivysaur",
        id: 002,
        type_: PokemonType::Two(Type::Poison, Type::Grass),
        base_stats: Stats {
            hp: 60,
            attack: 62,
            defense: 63,
            special_attack: 80,
            special_defense: 80,
            speed: 60,
        },
        attacks: &[TACKLE, VINE_WHIP],
    },
    PokemonModel {
        name: "Venusaur",
        id: 003,
        type_: PokemonType::Two(Type::Poison, Type::Grass),
        base_stats: Stats {
            hp: 80,
            attack: 82,
            defense: 83,
            special_attack: 100,
            special_defense: 100,
            speed: 80,
        },
        attacks: &[TACKLE, VINE_WHIP],
    },
    PokemonModel {
        name: "Charmander",
        id: 004,
        type_: PokemonType::One(Type::Fire),
        base_stats: Stats {
            hp: 39,
            attack: 52,
            defense: 43,
            special_attack: 60,
            special_defense: 50,
            speed: 65,
        },
        attacks: &[TACKLE],
    },
    PokemonModel {
        name: "Charmeleon",
        id: 005,
        type_: PokemonType::One(Type::Fire),
        base_stats: Stats {
            hp: 58,
            attack: 64,
            defense: 58,
            special_attack: 80,
            special_defense: 65,
            speed: 80,
        },
        attacks: &[TACKLE, EMBER],
    },
    PokemonModel {
        name: "Charizard",
        id: 006,
        type_: PokemonType::Two(Type::Fire, Type::Flying),
        base_stats: Stats {
            hp: 78,
            attack: 84,
            defense: 78,
            special_attack: 109,
            special_defense: 85,
            speed: 100,
        },
        attacks: &[TACKLE, EMBER],
    },
    PokemonModel {
        name: "Squirtle",
        id: 007,
        type_: PokemonType::One(Type::Water),
        base_stats: Stats {
            hp: 44,
            attack: 48,
            defense: 65,
            special_attack: 50,
            special_defense: 64,
            speed: 43,
        },
        attacks: &[TACKLE],
    },
    PokemonModel {
        name: "Wartortle",
        id: 008,
        type_: PokemonType::One(Type::Water),
        base_stats: Stats {
            hp: 59,
            attack: 63,
            defense: 80,
            special_attack: 65,
            special_defense: 80,
            speed: 58,
        },
        attacks: &[TACKLE, WATER_GUN],
    },
    PokemonModel {
        name: "Blastoise",
        id: 009,
        type_: PokemonType::One(Type::Water),
        base_stats: Stats {
            hp: 79,
            attack: 83,
            defense: 100,
            special_attack: 85,
            special_defense: 105,
            speed: 78,
        },
        attacks: &[TACKLE, WATER_GUN],
    },
];


/// List of all attacks.
///
/// Of course, these are not all attacks. We will probably provide a much
/// bigger database with the next sheet.
pub const ATTACK_DB: &'static [Attack] = &[
    Attack {
        category: AttackCategory::Physical,
        name: "Tackle",
        base_power: 50,
        type_: Type::Normal,
    },
    Attack {
        category: AttackCategory::Special,
        name: "Vine Whip",
        base_power: 45,
        type_: Type::Grass,
    },
    Attack {
        category: AttackCategory::Special,
        name: "Ember",
        base_power: 40,
        type_: Type::Fire,
    },
    Attack {
        category: AttackCategory::Special,
        name: "Water Gun",
        base_power: 40,
        type_: Type::Water,
    },
];

// These are just some easy names to be more expressive in the Pokedex.
pub const TACKLE: &'static Attack = &ATTACK_DB[0];
pub const VINE_WHIP: &'static Attack = &ATTACK_DB[1];
pub const EMBER: &'static Attack = &ATTACK_DB[2];
pub const WATER_GUN: &'static Attack = &ATTACK_DB[3];
