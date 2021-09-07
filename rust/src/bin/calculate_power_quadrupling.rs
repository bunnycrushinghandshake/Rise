use rise::monsters;
use rise::monsters::ChallengeRating;
use rise::simulation::combat::run_combat;

fn main() {
    for level in vec![8, 11, 14, 17, 20] {
        let blue = vec![monsters::Monster::standard_monster(
            ChallengeRating::Two,
            level,
            None,
            None,
        )];
        let level_difference = 6;
        let red = vec![
            monsters::Monster::standard_monster(
                ChallengeRating::Two,
                level - level_difference,
                None,
                None,
            ),
            monsters::Monster::standard_monster(
                ChallengeRating::Two,
                level - level_difference,
                None,
                None,
            ),
            monsters::Monster::standard_monster(
                ChallengeRating::Two,
                level - level_difference,
                None,
                None,
            ),
            monsters::Monster::standard_monster(
                ChallengeRating::Two,
                level - level_difference,
                None,
                None,
            ),
        ];
        let results = run_combat(blue, red);
        println!("L{}: {}", level, results);
    }
}
