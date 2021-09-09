use rise::core_mechanics::creatures::creature::Creature;
use rise::core_mechanics::creatures::{HasModifiers, Modifier};
use rise::core_mechanics::{Defense, HasDamageAbsorption, HasDefenses};
use rise::equipment::{Armor, HasArmor};
use rise::skills::{HasSkills, Skill};

#[test]
fn it_calculates_armor_effects() {
    let mut creature = Creature::new(1);
    assert_eq!(
        0,
        creature.calc_defense(&Defense::Armor),
        "Should have 0 AD"
    );
    assert_eq!(
        0,
        creature.calc_skill_modifier(&Skill::Climb),
        "Should have 0 Climb"
    );
    assert_eq!(
        0,
        creature.calc_skill_modifier(&Skill::Devices),
        "Should have 0 Devices"
    );

    creature.add_armor(Armor::Breastplate(None));
    assert_eq!(
        3,
        creature.calc_defense(&Defense::Armor),
        "Should have 3 AD"
    );
    assert_eq!(
        -4,
        creature.calc_skill_modifier(&Skill::Climb),
        "Should have -4 Climb"
    );
    // Encumbrance should only modify str/dex skill checks, not Devices
    assert_eq!(
        0,
        creature.calc_skill_modifier(&Skill::Devices),
        "Should have 0 Devices"
    );
}

#[test]
fn it_calculates_modifiers() {
    let mut creature = Creature::new(1);
    assert_eq!(
        0,
        creature.calc_defense(&Defense::Armor),
        "Should have 0 AD"
    );
    assert_eq!(
        0,
        creature.calc_skill_modifier(&Skill::Climb),
        "Should have 0 Climb"
    );
    assert_eq!(
        0,
        creature.calc_skill_modifier(&Skill::Devices),
        "Should have 0 Devices"
    );

    creature.add_modifier(Modifier::Defense(Defense::Armor, 2));
    creature.add_modifier(Modifier::DamageResistance(1));
    creature.add_modifier(Modifier::DamageResistance(2));
    assert_eq!(
        2,
        creature.calc_defense(&Defense::Armor),
        "Should have 2 AD"
    );
    assert_eq!(
        0,
        creature.calc_defense(&Defense::Fortitude),
        "Should have 0 Fort"
    );
    assert_eq!(5, creature.calc_damage_resistance(), "Should have 5 DR");
}