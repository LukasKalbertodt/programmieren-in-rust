// This module defines types only. Not all fields or variants of those are
// necessarily used, so we don't want all those warnings. This should be safe
// since no algorithm is defined in this module (in which such a warning
// could hint the existance of a bug).
// #![allow(dead_code)]

/// Describes an attack with all its properties. This type is similar to
/// `PokemonModel`, as there are finite many, immutable instances of this type
/// in a database. This is not a type whose instances change over time.
#[derive(Debug, Clone, Copy)]
pub struct Attack {
    pub category: AttackCategory,
    pub name: &'static str,
    /// Base power of the move. The actual inflicted damage is calculated with
    /// a formula using the move's power and a few other parameters.
    pub base_power: u8,
    pub type_: Type,
}

/// Category of an attack.
///
/// Note: currently, the category 'status' is missing.
#[derive(Debug, Clone, Copy)]
pub enum AttackCategory {
    /// Attacks with body contact, like "Tackle" or "Bite"
    Physical,
    /// Attacks without body contact, like "Bubble Beam" or "Thunderbolt"
    Special,
}

/// Describes how effective an attack of one type is on a Pokemon of another
/// type.
///
/// Note that a Pokemon can have two types. In order to determine the
/// effectiveness, the multipliers of the effectivenesses on both types
/// are multiplied. As such, there can be 0.25 and 4.0 multipliers!
#[derive(Debug, Clone, Copy)]
pub enum TypeEffectiveness {
    NotEffective,
    NotVeryEffective,
    Normal,
    SuperEffective,
}


/// Types (sometimes called "elements") of the Pokemon universe. Each
/// attack-move has exactly one type, Pokemons can have one or two types.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Type {
    Normal,
    Fire,
    Fighting,
    Water,
    Flying,
    Grass,
    Poison,
    Electric,
    Ground,
    Psychic,
    Rock,
    Ice,
    Bug,
    Dragon,
    Ghost,
    Dark,
    Steel,
    Fairy,
}

/// Describes the type of a Pokemon. Pokemon can have one or two types.
#[derive(Debug, Clone, Copy)]
pub enum PokemonType {
    One(Type),
    Two(Type, Type),
}

/// Describes a kind of Pokemon, e.g. "Pikachu".
///
/// This is different than an actual, living Pokemon. This struct just
/// describes properties that are the same for every creature of this
/// Pokemon kind.
#[derive(Debug, Clone, Copy)]
pub struct PokemonModel {
    /// Name of the Pokemon
    pub name: &'static str,
    /// ID in the international Pokedex
    pub id: u16,
    pub type_: PokemonType,
    pub base_stats: Stats,
    /// This is different from the real Pokemon games: attacks are not part
    /// of the Pokemon model, but of the Pokemon itself (as they change over
    /// time). A pokemon just has an abstract learnset of potential attacks.
    /// But this is easier for now.
    pub attacks: &'static [&'static Attack]
}

/// Describes the basic stats of a Pokemon.
///
/// Each living Pokemon has an actual stat value, but each Pokemon kind also
/// has so called "base stats". These base stats are used to calculate the
/// actual stats, whose depend on the Pokemon's current level. Stronger Pokemon
/// have higher base stats.
#[derive(Debug, Clone, Copy)]
pub struct Stats {
    /// Health points
    pub hp: u16,
    /// Speed, sometimes called initiative (INIT)
    pub speed: u16,
    /// Strength of physical attacks (like "Tackle")
    pub attack: u16,
    /// Strength of special attacks (like "Bubble Beam")
    pub special_attack: u16,
    /// Defense power against physical attacks (like "Tackle")
    pub defense: u16,
    /// Defense power against special attacks (like "Bubble Beam")
    pub special_defense: u16,
}
