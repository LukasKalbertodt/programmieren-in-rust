use db::types::*;
use super::Pokemon;

impl TypeEffectiveness {
    /// Returns the type effectiveness of an attack from one attacker type
    /// on one defender type.
    pub fn of_attack(attacker: Type, defender: Type) -> Self {
        use db::Type::*;
        use db::TypeEffectiveness as Te;

        // TODO: complete this
        match (attacker, defender) {
            (Fire, Water) => Te::NotVeryEffective,
            (Fire, Grass) => Te::SuperEffective,
            (Water, Fire) => Te::SuperEffective,
            (Water, Grass) => Te::NotVeryEffective,
            (Grass, Fire) => Te::NotVeryEffective,
            (Grass, Water) => Te::SuperEffective,
            (Ground, Flying) => Te::NotEffective,
            _ => Te::Normal,
        }
    }

    /// Returns the corresponding multiplier for the damage formula.
    pub fn multiplier(&self) -> f64 {
        match *self {
            TypeEffectiveness::NotEffective => 0.0,
            TypeEffectiveness::NotVeryEffective => 0.5,
            TypeEffectiveness::Normal => 1.0,
            TypeEffectiveness::SuperEffective => 2.0,
        }
    }
}


impl Stats {
    /// Given the base stats and a level, this function returns the actual
    /// stats for that level.
    ///
    /// This function doesn't implement the correct formula used by Pokemon
    /// games. It is a simplified version of the original formula for now: we
    /// ignore IVs, EVs and the Pokemon's nature). The complete formula can be
    /// found [here (HP)][1] and [here (other stats)][2].
    ///
    /// [1]: http://bulbapedia.bulbagarden.net/wiki/File:HPStatCalcGen34.png
    /// [2]: http://bulbapedia.bulbagarden.net/wiki/File:OtherStatCalcGen34.png
    pub fn at_level(base: Self, level: u8) -> Self {
        /// The formula is the same for all stats != hp
        fn stat_formula(base: u16, level: u8) -> u16 {
            ((base as f64 * level as f64) / 50.0 + 5.0) as u16
        }

        let hp = (
            (base.hp as f64 * level as f64) / 50.0
                + level as f64
                + 10.0
        ) as u16;

        Stats {
            hp: hp,
            speed: stat_formula(base.speed, level),
            attack: stat_formula(base.attack, level),
            special_attack: stat_formula(base.special_attack, level),
            defense: stat_formula(base.defense, level),
            special_defense: stat_formula(base.special_defense, level),
        }
    }
}

// ===========================================================================
// ===========================================================================
// ===========================================================================
// Formulas to calculate stuff
// ===========================================================================
// ===========================================================================
// ===========================================================================

/// Calculates the damage of an attack. We don't use the exact formula, but
/// a simplified version of it. In particular, we simplified the "Modifier"
/// term quite a bit. The correct and complete formula can be found [here][1].
///
/// [1]: http://bulbapedia.bulbagarden.net/wiki/Damage#Damage_formula
pub fn attack_damage(attacker: &Pokemon, defender: &Pokemon, attack: Attack) -> u16 {
    // Depending on the attack category, get the correct stats
    let (attack_mod, defense_mod) = match attack.category {
        AttackCategory::Physical => {
            (attacker.stats().attack, defender.stats().defense)
        }
        AttackCategory::Special => {
            (attacker.stats().special_attack, defender.stats().special_defense)
        }
    };

    // Cast everything to f64 to reduce noise in actual formula
    let (attack_mod, defense_mod) = (attack_mod as f64, defense_mod as f64);
    let base_power = attack.base_power as f64;
    let attacker_level = attacker.level() as f64;

    // The modifier only depends on the type effectiveness (in our simplified
    // version!).
    let modifier = match defender.model().type_ {
        PokemonType::One(ty) => {
            TypeEffectiveness::of_attack(attack.type_, ty).multiplier()
        }
        PokemonType::Two(ty_a, ty_b) => {
            TypeEffectiveness::of_attack(attack.type_, ty_a).multiplier()
                * TypeEffectiveness::of_attack(attack.type_, ty_b).multiplier()
        }
    };

    // With every parameter prepared above, here is the formula
    (
        (
            ((2.0 * attacker_level + 10.0) / 250.0)
                * (attack_mod / defense_mod)
                * base_power
                + 2.0
        ) * modifier
    ) as u16
}
