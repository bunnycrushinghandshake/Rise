use super::creature::CreatureCategory;
use crate::classes::{calc_rank_abilities, Class, ClassArchetype};
use crate::core_mechanics::{Attribute, Defense, HasAttributes, HasResources, Resource};
use crate::creatures::{creature, latex, HasModifiers, Modifier};
use crate::equipment::{Armor, ArmorMaterial, ArmorUsageClass, HasArmor, HasWeapons, Weapon};

pub struct Character {
    archetypes: [ClassArchetype; 3],
    class: Class,
    pub creature: creature::Creature,
}

impl Character {
    // archetypes should be provided in the order that they should be ranked up
    pub fn new(class: Class, level: i32, archetypes: [ClassArchetype; 3]) -> Character {
        let mut creature = creature::Creature::new(level, CreatureCategory::Character);

        for rank_ability in calc_rank_abilities(level, &archetypes) {
            if let Some(rank_modifiers) = rank_ability.modifiers {
                for modifier in rank_modifiers {
                    creature.add_modifier(
                        modifier.clone(),
                        Some(rank_ability.name),
                        Some(rank_ability.rank),
                    );
                }
            }
        }

        for defense in Defense::all() {
            creature.add_modifier(
                Modifier::Defense(defense, class.defense_bonus(&defense)),
                Some(class.name()),
                None,
            );
        }
        for resource in Resource::all() {
            creature.add_modifier(
                Modifier::Resource(resource, class.resource_bonus(&resource)),
                Some(class.name()),
                None,
            );
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
        character.creature.set_name("Standard Character");

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
            character.creature.add_magic_modifier(modifier);
        }

        return character;
    }

    pub fn standard_sorcerer(level: i32, use_point_buy: bool) -> Self {
        let mut character = Self::new(
            Class::Sorcerer,
            level,
            [
                ClassArchetype::SorcererArcaneMagic,
                ClassArchetype::SorcererArcaneSpellMastery,
                ClassArchetype::DraconicMagic,
            ],
        );

        character.creature.set_name("Standard Sorcerer");

        if use_point_buy {
            character
                .creature
                .set_base_attribute(Attribute::Strength, 0);
            character
                .creature
                .set_base_attribute(Attribute::Dexterity, 1);
            character
                .creature
                .set_base_attribute(Attribute::Constitution, 2);
            character
                .creature
                .set_base_attribute(Attribute::Intelligence, 0);
            character
                .creature
                .set_base_attribute(Attribute::Perception, 2);
            character
                .creature
                .set_base_attribute(Attribute::Willpower, 4);
        }

        for modifier in calc_standard_magic_modifiers(level) {
            character.creature.add_magic_modifier(modifier);
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
            creature_latex = latex::format_creature(&self.creature),
            class_name = self.class.name(),
            level = self.creature.level,
            ap = self.creature.calc_resource(&Resource::AttunementPoint),
            ft = self.creature.calc_resource(&Resource::FatigueTolerance),
            ip = self.creature.calc_resource(&Resource::InsightPoint),
            sp = self.creature.calc_resource(&Resource::TrainedSkill),
            // attacks = attacks
            //     .iter()
            //     .map(|a| a.shorthand_description(self))
            //     .collect::<Vec<String>>()
            //     .join("\n"),
        );
    }
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
        }
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
        }
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
        }
    }
}

#[cfg(test)]
mod tests;
