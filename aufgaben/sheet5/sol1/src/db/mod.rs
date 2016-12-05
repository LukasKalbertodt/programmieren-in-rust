pub mod types;
pub mod data;

pub use self::types::*;


// pub fn pokemon_by_id(id: u16) -> Option<PokemonModel> {
//     match data::POKEDEX.get(id as usize) {
//         None => None,
//         Some(&pm) => Some(pm),
//     }
// }

pub fn pokemon_by_name(name: &str) -> Option<PokemonModel> {
    for &pm in data::POKEDEX {
        if pm.name == name {
            return Some(pm);
        }
    }

    None
}
