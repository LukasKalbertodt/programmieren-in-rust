use db::types::*;
use db;
use engine::Pokemon;

use term_painter::Color::*;
use term_painter::ToStyle;

pub fn fight(mut red: Pokemon, mut blue: Pokemon) {
    loop {
        fn check_dead(poki: &Pokemon) -> bool {
            if poki.is_alive() {
                false
            } else {
                println!(">>>>> {} fainted!", Yellow.paint(poki.name()));
                true
            }
        }

        // Print status
        println!(
            ">>>>> Status: {} has {} HP, {} has {} HP",
            Yellow.paint(red.name()),
            Red.paint(red.stats().hp),
            Yellow.paint(blue.name()),
            Red.paint(blue.stats().hp),
        );

        // Execute both attack
        if red.stats().speed > blue.stats().speed {
            // Red attacks blue
            execute_round(&red, &mut blue);
            if check_dead(&blue) {
                break;
            }

            // BLue attacks red
            execute_round(&blue, &mut red);
            if check_dead(&red) {
                break;
            }
        } else {
            // BLue attacks red
            execute_round(&blue, &mut red);
            if check_dead(&red) {
                break;
            }

            // Red attacks blue
            execute_round(&red, &mut blue);
            if check_dead(&blue) {
                break;
            }
        }
    }
}


/// Executes one round of one player:
///
/// - the player chooses one attack to execute
/// - the attack is excuted and the enemy's HP
fn execute_round(attacker: &Pokemon, defender: &mut Pokemon) {
    // Tell the user to choose an attack
    println!(
        ">>>>> {} is about to attack! Which move shall it execute?",
        Yellow.paint(attacker.model().name)
    );

    // Print a list of available attacks
    let num_attacks = attacker.model().attacks.len();
    for i in 0..num_attacks {
        println!(
            "    {}: {}",
            i,
            Blue.paint(attacker.model().attacks[i].name)
        );
    }
    println!("    !!! Please give me the attack ID:");

    // Read attack ID from the user
    let attack_id;
    loop {
        let input = read_usize();
        if input >= num_attacks {
            println!("    !!! There is no attack with index {}!", input);
        } else {
            attack_id = input;
            break;
        }
    }

    // Execute attack
    let attack = *attacker.model().attacks[attack_id];
    defender.endure_attack(attacker, attack);

    // Status update
    println!(
        ">>>>> {} uses {}! ({} has {} HP left)",
        Yellow.paint(attacker.model().name),
        Blue.paint(attack.name),
        Yellow.paint(defender.model().name),
        Red.paint(defender.stats().hp),
    );
}

/// Let's the player choose a Pokemon from the database.
pub fn choose_pokemon(player: &str) -> PokemonModel {
    // Loop forever until the user has chosen a Pokemon
    loop {
        println!(
            "{}, please choose a Pokemon (or type '?' to get a complete list)",
            player,
        );

        let input = read_string();
        if input == "?" {
            print_pokemon_list();
        } else {
            // Try to find a Pokemon with the given name
            match db::pokemon_by_name(&input) {
                Some(poki) => return poki,
                None => {
                    println!("No pokemon with the name '{}' was found!", input);
                }
            }
        }
    }
}

/// Prints a list of all Pokemon in the database.
fn print_pokemon_list() {
    for poki in db::data::POKEDEX {
        // This strange formatter will print the pokemon ID with three digits,
        // filling in 0 from the left if necessary (#003).
        println!("#{:0>3} {}", poki.id, poki.name);
    }
}


/// Reads a string from the terminal/user.
fn read_string() -> String {
    let mut buffer = String::new();
    ::std::io::stdin()
        .read_line(&mut buffer)
        .expect("something went horribly wrong...");

    // Discard trailing newline
    let new_len = buffer.trim_right().len();
    buffer.truncate(new_len);

    buffer
}

/// Reads a valid `usize` integer from the terminal/user.
fn read_usize() -> usize {
    loop {
        match read_string().parse() {
            Ok(res) => return res,
            Err(_) => println!("That was not an integer! Please try again!"),
        }
    }
}
