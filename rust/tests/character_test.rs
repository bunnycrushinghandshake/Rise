#[cfg(test)]
mod character_tests {
    use rise::classes::{calc_rank_abilities, Class, ClassArchetype};
    use rise::core_mechanics::attacks::HasAttacks;
    use rise::creatures::{Character, HasModifiers};

    #[test]
    fn it_calculates_rank_abilities() {
        let abilities = calc_rank_abilities(
            1,
            &[
                ClassArchetype::BlessingsOfTheAbyss,
                ClassArchetype::PactMagic,
                ClassArchetype::PactSpellMastery,
            ],
        );
        let mut ability_names: Vec<&str> = abilities.iter().map(|a| a.name).collect();
        ability_names.sort();
        assert_eq!(
            vec!["Abyssal Rebuke",],
            ability_names,
            "Should match expected names"
        );
    }

    #[test]
    fn it_calculates_modifiers() {
        let warlock = Character::new(
            Class::Warlock,
            1,
            [
                ClassArchetype::BlessingsOfTheAbyss,
                ClassArchetype::PactMagic,
                ClassArchetype::PactSpellMastery,
            ],
        );

        let modifiers = warlock.creature.get_modifiers();
        let mut modifier_descriptions: Vec<String> =
            modifiers.iter().map(|a| a.description()).collect();
        modifier_descriptions.sort();
        // Note that this ignores the DR 1 ability from rank 0 blessings of the abyss
        assert_eq!(
            vec![
                "attack Abyssal Rebuke",
                "defense armor by 0",
                "defense fortitude by 5",
                "defense mental by 7",
                "defense reflex by 3",
                "power 3",
                "resource attunement point by 4",
                "resource fatigue tolerance by 4",
                "resource insight point by 1",
                "resource trained skill by 3"
            ],
            modifier_descriptions,
            "Should match expected names"
        );
    }

    #[test]
    fn it_calculates_abyssal_blast() {
        let warlock = Character::new(
            Class::Warlock,
            20,
            [
                ClassArchetype::BlessingsOfTheAbyss,
                ClassArchetype::PactMagic,
                ClassArchetype::PactSpellMastery,
            ],
        );

        let attacks = warlock.creature.calc_all_attacks();
        assert_eq!(1, attacks.len(), "Should have one attack");
        let abyssal_blast = &attacks[0];
        assert_eq!(
            "Abyssal Rebuke +10 (The target takes 6d10+35 fire damage.)",
            abyssal_blast.shorthand_description(&warlock.creature),
            "Should have correct description"
        );
    }
}
