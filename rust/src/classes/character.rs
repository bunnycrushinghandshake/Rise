use crate::classes::{Class, ClassArchetype, RankAbility};
use crate::core_mechanics::creatures::attacks::{self, HasAttacks};
use crate::core_mechanics::creatures::{
    creature, latex, HasCreatureMechanics, HasModifiers, Modifier, ModifierType,
};
use crate::core_mechanics::{
    Attribute, Defense, HasAttributes, HasDamageAbsorption, HasDefenses, HasResources, Resource,
};
use crate::equipment::{Armor, ArmorMaterial, ArmorUsageClass, HasArmor, HasWeapons, Weapon};
use crate::skills::{HasSkills, Skill};

pub struct Character {
    archetypes: [ClassArchetype; 3],
    class: Class,
    creature: creature::Creature,
}

impl Character {
    // archetypes should be provided in the order that they should be ranked up
    pub fn new(class: Class, level: i32, archetypes: [ClassArchetype; 3]) -> Character {
        let mut creature = creature::Creature::new(level);

        for rank_ability in calc_rank_abilities(level, &archetypes) {
            if let Some(rank_modifiers) = rank_ability.modifiers {
                for modifier in rank_modifiers {
                    creature.add_modifier(modifier.clone());
                }
            }
        }

        return Character {
            archetypes,
            class,
            creature,
        };
    }

    // Currently this creates a Martial Mastery fighter
    pub fn standard_character(level: i32, use_point_buy: bool) -> Self {
        let mut character = Self::new(
            Class::Fighter,
            level,
            [
                ClassArchetype::MartialMastery,
                ClassArchetype::EquipmentTraining,
                ClassArchetype::CombatDiscipline,
            ],
        );

        character.creature.add_weapon(Weapon::Broadsword);
        character
            .creature
            .add_armor(standard_armor_by_level(level, ArmorUsageClass::Heavy));
        character.creature.add_armor(Armor::StandardShield);
        character
            .creature
            .set_name("Standard Character".to_string());

        if use_point_buy {
            character
                .creature
                .set_base_attribute(Attribute::Strength, 4);
            character
                .creature
                .set_base_attribute(Attribute::Dexterity, 0);
            character
                .creature
                .set_base_attribute(Attribute::Constitution, 2);
            character
                .creature
                .set_base_attribute(Attribute::Intelligence, 1);
            character
                .creature
                .set_base_attribute(Attribute::Perception, 2);
            character
                .creature
                .set_base_attribute(Attribute::Willpower, 0);
        }

        for modifier in calc_standard_magic_modifiers(level) {
            character.creature.add_modifier(modifier);
        }

        return character;
    }

    pub fn description(&self) -> String {
        // let mut attacks = self.calc_all_attacks();
        // attacks.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        return format!(
            "
                {creature_latex}
                {class_name} {level}
                AP {ap}, FT {ft}, IP {ip}, SP {sp}
            ",
            creature_latex = latex::format_creature(self),
            class_name = self.class.name(),
            level = self.creature.level,
            ap = self.calc_resource(&Resource::AttunementPoint),
            ft = self.calc_resource(&Resource::FatigueTolerance),
            ip = self.calc_resource(&Resource::InsightPoint),
            sp = self.calc_resource(&Resource::TrainedSkill),
            // attacks = attacks
            //     .iter()
            //     .map(|a| a.shorthand_description(self))
            //     .collect::<Vec<String>>()
            //     .join("\n"),
        );
    }
}

impl HasModifiers for Character {
    fn add_modifier(&mut self, modifier: Modifier) {
        self.creature.add_modifier(modifier);
    }

    fn get_modifiers(&self) -> Vec<Modifier> {
        return self.creature.get_modifiers();
    }

    fn calc_total_modifier(&self, mt: ModifierType) -> i32 {
        return self.creature.calc_total_modifier(mt);
    }
}

impl HasAttributes for Character {
    fn get_base_attribute(&self, attribute: &Attribute) -> i32 {
        return self.creature.get_base_attribute(attribute);
    }
    fn calc_total_attribute(&self, attribute: &Attribute) -> i32 {
        return self.creature.calc_total_attribute(attribute);
    }
    fn set_base_attribute(&mut self, attribute: Attribute, value: i32) {
        self.creature.set_base_attribute(attribute, value);
    }
}

impl HasAttacks for Character {
    fn add_special_attack(&mut self, attack: attacks::Attack) {
        self.creature.add_special_attack(attack);
    }

    fn calc_all_attacks(&self) -> Vec<attacks::Attack> {
        return self.creature.calc_all_attacks();
    }

    fn calc_accuracy(&self) -> i32 {
        return self.creature.calc_accuracy();
    }

    fn calc_damage_per_round_multiplier(&self) -> f64 {
        return 1.0;
    }

    fn calc_damage_increments(&self, is_strike: bool) -> i32 {
        return self.creature.calc_damage_increments(is_strike);
    }

    fn calc_power(&self, is_magical: bool) -> i32 {
        return self.creature.calc_power(is_magical);
    }
}

impl HasArmor for Character {
    fn add_armor(&mut self, armor: Armor) {
        self.creature.add_armor(armor);
    }

    fn get_armor(&self) -> Vec<&Armor> {
        return self.creature.get_armor();
    }

    fn calc_encumbrance(&self) -> i32 {
        return self.creature.calc_encumbrance();
    }
}

impl HasWeapons for Character {
    fn add_weapon(&mut self, weapon: Weapon) {
        self.creature.add_weapon(weapon);
    }

    fn get_weapons(&self) -> Vec<&Weapon> {
        return self.creature.get_weapons();
    }
}

impl HasDamageAbsorption for Character {
    fn calc_damage_resistance(&self) -> i32 {
        return self.creature.calc_damage_resistance();
    }

    fn calc_hit_points(&self) -> i32 {
        return self.creature.calc_hit_points();
    }
}

impl HasDefenses for Character {
    fn calc_defense(&self, defense: &Defense) -> i32 {
        let mut value = self.creature.calc_defense(defense) + self.class.defense_bonus(defense);
        match defense {
            // TODO: check for light armor
            Defense::Armor => {
                value = value
                    + self.get_base_attribute(&Attribute::Dexterity) / 2
                    + self.get_base_attribute(&Attribute::Constitution) / 2;
            }
            _ => {}
        };
        return value;
    }
}

impl HasResources for Character {
    fn calc_resource(&self, resource: &Resource) -> i32 {
        return self.creature.calc_resource(resource) + self.class.resource_bonus(resource);
    }
}

impl HasSkills for Character {
    fn set_skill_trained(&mut self, skill: Skill, trained: bool) {
        return self.creature.set_skill_trained(skill, trained);
    }

    fn is_skill_trained(&self, skill: &Skill) -> bool {
        return self.creature.is_skill_trained(skill);
    }

    fn calc_skill_modifier(&self, skill: &Skill) -> i32 {
        return self.creature.calc_skill_modifier(skill);
    }
}

// No need for explicit funtions here - it's handled by the above functions
impl HasCreatureMechanics for Character {}

struct StandardMagicBonuses {
    damage_resistance: i32,
    hit_points: i32,
    power: i32,
}

fn calc_standard_magic_modifiers(level: i32) -> Vec<Modifier> {
    let mut modifiers = vec![];
    // Wealth is one item of current level, two items of one level lower, and two items of two
    // levels lower.
    // For most characters, power is most important, followed by damage resistance, and finally
    // hit points.
    // The level breakpoints for standard power and DR items are 4/10/16.
    // This ignores legacy items, but assumes that items are acquired as soon as possible. On
    // average, this should make the levels reasonably accurate.

    let mut power = 0;
    if level >= 16 {
        power = 8;
    } else if level >= 10 {
        power = 4;
    } else if level >= 4 {
        power = 2;
    }
    if power > 0 {
        modifiers.push(Modifier::MagicalPower(power));
        modifiers.push(Modifier::MundanePower(power));
    }

    let mut dr = 0;
    if level >= 17 {
        dr = 16;
    } else if level >= 11 {
        dr = 8;
    } else if level >= 5 {
        dr = 4;
    }
    if dr > 0 {
        modifiers.push(Modifier::DamageResistance(dr));
    }

    let mut hp = 0;
    if level >= 18 {
        hp = 16;
    } else if level >= 12 {
        hp = 8;
    } else if level >= 6 {
        hp = 4;
    }
    if hp > 0 {
        modifiers.push(Modifier::HitPoints(hp));
    }

    return modifiers;
}

pub fn calc_rank_abilities<'a>(
    level: i32,
    archetypes: &'a [ClassArchetype; 3],
) -> Vec<RankAbility<'a>> {
    let mut rank_abilities: Vec<RankAbility> = vec![];
    // Add rank 0 abilities
    for archetype in archetypes.iter() {
        rank_abilities.append(archetype.abilities_at_rank(0).as_mut());
    }
    // Add higher rank abilities
    for i in 0..level {
        rank_abilities.append(
            archetypes[(i % 3) as usize]
                .abilities_at_rank((i / 3) + 1)
                .as_mut(),
        );
    }
    return rank_abilities;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_rank_abilities() {
        let mut fighter_1_abilities = calc_rank_abilities(
            1,
            &[
                ClassArchetype::MartialMastery,
                ClassArchetype::EquipmentTraining,
                ClassArchetype::CombatDiscipline,
            ],
        )
        .iter()
        .map(|a| a.name)
        .collect::<Vec<&str>>();
        fighter_1_abilities.sort();
        assert_eq!(
            vec![
                "Armor Expertise",
                "Combat Styles",
                "Martial Expertise",
                "Mental Discipline",
            ],
            fighter_1_abilities,
            "Should have correct abilities for a level 1 fighter",
        );

        let mut fighter_10_abilities = calc_rank_abilities(
            10,
            &[
                ClassArchetype::MartialMastery,
                ClassArchetype::EquipmentTraining,
                ClassArchetype::CombatDiscipline,
            ],
        )
        .iter()
        .map(|a| a.name)
        .collect::<Vec<&str>>();
        fighter_10_abilities.sort();
        assert_eq!(
            vec![
                "Armor Expertise",
                "Cleansing Discipline",
                "Combat Style Rank",
                "Combat Style Rank",
                "Combat Style Rank",
                "Combat Styles",
                "Disciplined Force",
                "Enduring Discipline",
                "Equipment Efficiency",
                "Glancing Strikes",
                "Greater Armor Expertise",
                "Martial Expertise",
                "Martial Force",
                "Martial Maneuver",
                "Mental Discipline",
                "Weapon Training"
            ],
            fighter_10_abilities,
            "Should have correct abilities for a level 10 fighter",
        );
    }
}

// Use a relatively smooth level progression for a (level - 1) item
fn standard_armor_by_level(level: i32, max_usage_class: ArmorUsageClass) -> Armor {
    match max_usage_class {
        ArmorUsageClass::Heavy => {
            if level == 21 {
                return Armor::FullPlate(Some(ArmorMaterial::AncientDragonscale(
                    "red".to_string(),
                )));
            } else if level >= 18 {
                return Armor::FullPlate(Some(ArmorMaterial::PureDeepforged));
            } else if level >= 15 {
                return Armor::FullPlate(Some(ArmorMaterial::Dragonscale("red".to_string())));
            } else if level >= 12 {
                return Armor::FullPlate(Some(ArmorMaterial::Deepforged));
            } else if level >= 9 {
                return Armor::LayeredHide(Some(ArmorMaterial::Elvenweave));
            } else if level >= 6 {
                return Armor::FullPlate(None);
            } else if level >= 3 {
                return Armor::LayeredHide(None);
            } else {
                return Armor::ScaleMail(None);
            }
        },
        ArmorUsageClass::Medium => {
            if level >= 18 {
                return Armor::Breastplate(Some(ArmorMaterial::AncientDragonscale(
                    "red".to_string(),
                )));
            } else if level >= 15 {
                return Armor::Breastplate(Some(ArmorMaterial::PureDeepforged));
            } else if level >= 12 {
                return Armor::Breastplate(Some(ArmorMaterial::Dragonscale("red".to_string())));
            } else if level >= 9 {
                return Armor::Breastplate(Some(ArmorMaterial::Deepforged));
            } else if level >= 3 {
                return Armor::Breastplate(None);
            } else {
                return Armor::ScaleMail(None);
            }
        },
        ArmorUsageClass::Light => {
            if level >= 17 {
                return Armor::ChainShirt(Some(ArmorMaterial::AncientDragonscale(
                    "red".to_string(),
                )));
            } else if level >= 14 {
                return Armor::ChainShirt(Some(ArmorMaterial::PureDeepforged));
            } else if level >= 11 {
                return Armor::ChainShirt(Some(ArmorMaterial::Dragonscale("red".to_string())));
            } else if level >= 8 {
                return Armor::ChainShirt(Some(ArmorMaterial::Deepforged));
            } else {
                return Armor::ChainShirt(None);
            }
        },
    }
}
