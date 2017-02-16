use db::types::*;

pub mod canon;

pub struct Pokemon {
    /// Reference to the kind of the Pokemon, which contains the name, base
    /// stats, id and more global data.
    model: PokemonModel,
    /// These are the actual stats of the pokemon, that fit to the current
    /// level
    stats: Stats,
    /// The current level of the Pokemon.
    level: u8,
}

impl Pokemon {
    /// Creates a new living Pokemon of the given Pokemon kind (model) with the
    /// specified level.
    pub fn with_level(model: PokemonModel, level: u8) -> Self {
        Pokemon {
            model: model,
            stats: Stats::at_level(model.base_stats, level),
            level: level,
        }
    }

    /// Returns the current stats.
    pub fn stats(&self) -> &Stats {
        &self.stats
    }

    /// Returns the Pokemon kind of this Pokemon.
    pub fn model(&self) -> &PokemonModel {
        &self.model
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        self.model.name
    }

    /// Returns the current level.
    pub fn level(&self) -> u8 {
        self.level
    }

    /// Decreases the Pokemon's HP according to the given attack and attacker.
    pub fn endure_attack(&mut self, attacker: &Pokemon, attack: Attack) {
        let damage = canon::attack_damage(attacker, self, attack);
        self.stats.hp = self.stats.hp.saturating_sub(damage);
    }

    /// Returns whether or not the Pokemon is still alive (more than 0 HP).
    pub fn is_alive(&self) -> bool {
        self.stats.hp > 0
    }
}
