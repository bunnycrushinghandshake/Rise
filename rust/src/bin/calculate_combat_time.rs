use rise::calculations::statistical_combat::run_combat;
use rise::core_mechanics::HasDamageAbsorption;
use rise::creatures::{Character, Monster};
use rise::monsters::ChallengeRating;

fn main() {
    for level in vec![2, 8, 14, 20] {
        for red in vec![
            ChallengeRating::standard_encounter(level, 8),
            ChallengeRating::standard_encounter(level, 4),
            ChallengeRating::standard_encounter(level, 2),
            ChallengeRating::standard_encounter(level, 1),
        ] {
            // PCs
            let mut blue = vec![
                Character::standard_character(level, true).creature,
                Character::standard_character(level, true).creature,
                Character::standard_character(level, true).creature,
                Character::standard_character(level, true).creature,
            ];
            let blue_damage_absorption: i32 = blue
                .iter()
                .map(|c| c.calc_hit_points() + c.calc_damage_resistance())
                .sum();
            let red_damage_absorption: i32 = red
                .iter()
                .map(|c| c.calc_hit_points() + c.calc_damage_resistance())
                .sum();
            let results = run_combat(blue, red);
            println!(
                "Count{:.1}, L{:>2}, BDA{:>4}, RDA{:>4}, {}",
                red.len(),
                level,
                blue_damage_absorption,
                red_damage_absorption,
                results
            );
        }
    }
}
