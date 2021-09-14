use super::*;
use crate::core_mechanics::Defense;
use crate::creatures::{Character, Creature, CreatureCategory, HasModifiers, Modifier, Monster, StandardAttack};
use crate::equipment::Weapon;
use crate::monsters::ChallengeRating;

#[test]
fn simple_hit_probability() {
    let hit_probability = calculate_hit_probability(&Attack::from_weapon(Weapon::Broadsword), 0, 6);
    assert_eq!(
        "0.500 single, 0.055 crit",
        format!(
            "{:.3} single, {:.3} crit",
            hit_probability.single_hit_probability, hit_probability.crit_probability
        ),
        "Should be around 50% with +0 vs 6",
    );

    let hit_probability = calculate_hit_probability(&Attack::from_weapon(Weapon::Broadsword), 0, 0);
    assert_eq!(
        "1.000 single, 0.111 crit",
        format!(
            "{:.3} single, {:.3} crit",
            hit_probability.single_hit_probability, hit_probability.crit_probability
        ),
        "Should be around 100% with +0 vs 0",
    );

    let hit_probability = calculate_hit_probability(&Attack::from_weapon(Weapon::Claw), 1, 10);
    assert_eq!(
        "0.400 single, 0.044 crit",
        format!(
            "{:.3} single, {:.3} crit",
            hit_probability.single_hit_probability, hit_probability.crit_probability
        ),
        "Should include weapon accuracy modifier and non-weapon accuracy modifier",
    );
}

#[test]
fn extreme_hit_probability() {
    let hit_probability =
        calculate_hit_probability(&Attack::from_weapon(Weapon::Broadsword), 0, 16);
    assert_eq!(
        "0.050 single, 0.005 crit",
        format!(
            "{:.3} single, {:.3} crit",
            hit_probability.single_hit_probability, hit_probability.crit_probability
        ),
        "Should be around 5% with +0 vs 16",
    );

    let hit_probability =
        calculate_hit_probability(&Attack::from_weapon(Weapon::Broadsword), 10, 6);
    assert_eq!(
        "1.000 single, 0.555 crit",
        format!(
            "{:.3} single, {:.3} crit",
            hit_probability.single_hit_probability, hit_probability.crit_probability
        ),
        "Should be over 100% with +10 vs 6",
    );
}

#[test]
fn glance_probability() {
    let attack = &Attack::from_weapon(Weapon::Broadsword);
    assert_eq!(
        "0.200",
        format!("{:.3}", calculate_glance_probability(attack, 0, 6),),
        "Should be 20% with +0 vs 6",
    );
    assert_eq!(
        "0.000",
        format!("{:.3}", calculate_glance_probability(attack, 0, 0),),
        "Should be 0% with +0 vs 0",
    );
    assert_eq!(
        "0.100",
        format!("{:.3}", calculate_glance_probability(attack, 0, 11),),
        "Should be 10% with +0 vs 11",
    );
    assert_eq!(
        "0.010",
        format!("{:.3}", calculate_glance_probability(attack, 0, 12),),
        "Should be 1% with +0 vs 12",
    );
}

#[test]
fn damage_per_round_no_modifier() {
    let mut attacker = Creature::new(1, CreatureCategory::Character);
    let mut defender = Creature::new(1, CreatureCategory::Character);
    assert_eq!(
        0.0,
        calc_individual_dpr(&attacker, &defender),
        "Should be 0.0 when attacker has no attacks",
    );

    // Ensure that the starting conditions match our expectations
    assert_eq!(
        0,
        attacker.calc_accuracy(),
        "Attacker should have 0 accuracy",
    );
    assert_eq!(
        vec![0, 0, 0, 0],
        Defense::all()
            .iter()
            .map(|d| defender.calc_defense(d))
            .collect::<Vec<i32>>(),
        "Defender should have all defenses 0",
    );

    attacker.add_special_attack(Attack::from_weapon(Weapon::Broadsword));
    assert_eq!(
        "5.000",
        format!("{:.3}", calc_individual_dpr(&attacker, &defender)),
        "Should be 4.5 dph * 1.111 hit % = 5 dpr",
    );

    defender.add_modifier(Modifier::Defense(Defense::Armor, 6), None, None);
    assert_eq!(
        "2.498",
        format!("{:.3}", calc_individual_dpr(&attacker, &defender)),
        "Should be 4.5 dph * 0.555 hit % = 2.498 dpr after increasing defender Armor defense",
    );

    attacker.add_special_attack(StandardAttack::DivineJudgment(1).attack());
    assert_eq!(
        "6.111",
        format!("{:.3}", calc_individual_dpr(&attacker, &defender)),
        "Should be 5.5 dph * 1.111 hit % = 6.111 dpr after adding Divine Judgment",
    );
}

#[test]
fn damage_per_round_with_modifier() {
    let mut attacker = Creature::new(1, CreatureCategory::Character);
    let mut defender = Creature::new(1, CreatureCategory::Character);
    attacker.add_special_attack(Attack::from_weapon(Weapon::Broadsword));
    attacker.add_modifier(Modifier::MundanePower(2), None, None);
    assert_eq!(
        "7.000",
        format!("{:.3}", calc_individual_dpr(&attacker, &defender)),
        "Should be 6.5 dph * 1.00 hit % + 4.5 dpc * .111 crit % = 6.9995 dpr",
    );
    defender.add_modifier(Modifier::Defense(Defense::Armor, 6), None, None);
    assert_eq!(
        "3.498",
        format!("{:.3}", calc_individual_dpr(&attacker, &defender)),
        "Should be 6.5 dph * 0.5 hit % + 4.5 dpc * .055 crit % = 3.4975 dpr",
    );
}

#[test]
fn standard_character_level_1_vs_cr1() {
    let attacker = Character::standard_character(1, true).creature;
    let defender = Monster::standard_monster(ChallengeRating::One, 1, None, None).creature;
    assert_eq!(6, defender.calc_defense(&Defense::Armor));

    let certain_strike = attacker
        .get_attack_by_name("Broadsword Certain Strike")
        .unwrap();
    assert_eq!(
        "3.820",
        format!(
            "{:.3}",
            calc_attack_damage_per_round(&certain_strike, &attacker, &defender)
        ),
        "Certain Strike: 4.5 dph * 0.8 hit % + 2.5 dpc * 0.088 crit % = 3.6 + 0.22 dpr",
    );

    let generic = attacker
        .get_attack_by_name("Broadsword Generic Scaling Strike")
        .unwrap();
    assert_eq!(
        "4.197",
        format!(
            "{:.3}",
            calc_attack_damage_per_round(&generic, &attacker, &defender)
        ),
        "Generic: 6.5 dph * 0.6 hit % + 4.5 dpc * 0.066 crit % = 3.9 + .297 dpr",
    );

    let power_strike = attacker
        .get_attack_by_name("Broadsword Power Strike")
        .unwrap();
    assert_eq!(
        "3.908",
        format!(
            "{:.3}",
            calc_attack_damage_per_round(&power_strike, &attacker, &defender)
        ),
        "Power Strike: 9 dph * 0.4 hit % + 7 dpc * 0.044 crit % = 3.6 + .308 dpr",
    );

    assert_eq!(
        "4.197",
        format!(
            "{:.3}",
            calc_individual_dpr(&attacker, &defender),
        ),
        "Generic should be the best attack",
    );
}

#[test]
fn standard_character_level_1_vs_weak_cr1() {
    let attacker = Character::standard_character(1, true).creature;
    let defender = Monster::standard_monster(ChallengeRating::One, 1, Some(0), None).creature;
    assert_eq!(4, defender.calc_defense(&Defense::Armor));

    let certain_strike = attacker
        .get_attack_by_name("Broadsword Certain Strike")
        .unwrap();
    assert_eq!(
        "4.775",
        format!(
            "{:.3}",
            calc_attack_damage_per_round(&certain_strike, &attacker, &defender)
        ),
        "Certain Strike: 4.5 dph * 1.0 hit % + 2.5 dpc * 0.110 crit % = 4.5 + 0.275 dpr",
    );

    let generic = attacker
        .get_attack_by_name("Broadsword Generic Scaling Strike")
        .unwrap();
    assert_eq!(
        "5.596",
        format!(
            "{:.3}",
            calc_attack_damage_per_round(&generic, &attacker, &defender)
        ),
        "Generic: 6.5 dph * 0.8 hit % + 4.5 dpc * 0.088 crit % = 5.2 + 0.396 dpr",
    );

    let power_strike = attacker
        .get_attack_by_name("Broadsword Power Strike")
        .unwrap();
    assert_eq!(
        "5.862",
        format!(
            "{:.3}",
            calc_attack_damage_per_round(&power_strike, &attacker, &defender)
        ),
        "Power Strike: 9 dph * 0.6 hit % + 7 dpc * 0.066 crit % = 5.4 + 0.462 dpr",
    );

    assert_eq!(
        "5.862",
        format!(
            "{:.3}",
            calc_individual_dpr(&attacker, &defender),
        ),
        "Power Strike should be the best attack",
    );
}

#[test]
fn standard_character_level_1_vs_cr2() {
    let attacker = Character::standard_character(1, true).creature;
    let defender = Monster::standard_monster(ChallengeRating::Two, 1, None, None).creature;
    assert_eq!(7, defender.calc_defense(&Defense::Armor));

    let certain_strike = attacker
        .get_attack_by_name("Broadsword Certain Strike")
        .unwrap();
    assert_eq!(
        "3.342",
        format!(
            "{:.3}",
            calc_attack_damage_per_round(&certain_strike, &attacker, &defender)
        ),
        "Certain Strike: 4.5 dph * 0.7 hit % + 2.5 dpc * 0.077 crit % = 3.15 + 0.1925 dpr",
    );

    let generic = attacker
        .get_attack_by_name("Broadsword Generic Scaling Strike")
        .unwrap();
    assert_eq!(
        "3.498",
        format!(
            "{:.3}",
            calc_attack_damage_per_round(&generic, &attacker, &defender)
        ),
        "Generic: 6.5 dph * 0.5 hit % + 4.5 dpc * 0.055 crit % = 3.4975 dpr",
    );

    let power_strike = attacker
        .get_attack_by_name("Broadsword Power Strike")
        .unwrap();
    assert_eq!(
        "2.931",
        format!(
            "{:.3}",
            calc_attack_damage_per_round(&power_strike, &attacker, &defender)
        ),
        "Power Strike: 9 dph * 0.3 hit % + 7 dpc * 0.033 crit % = 2.7 + .231 dpr",
    );

    assert_eq!(
        "3.498",
        format!(
            "{:.3}",
            calc_individual_dpr(&attacker, &defender),
        ),
        "Generic should be the best attack",
    );
}
