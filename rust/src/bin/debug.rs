use rise::{classes::Character, monsters::{ChallengeRating, Monster}};

fn main() {
    let standard_monster = Monster::standard_monster(ChallengeRating::Two, 3, None, None);
    println!("standard monster {}", standard_monster.to_section(None));
    let standard_character = Character::standard_character(10, true);
    println!("standard character {}", standard_character.description());
}
