use crate::core_mechanics::{
    Attribute, HasAttributes, MovementMode, PassiveAbility, Sense, Size, SpecialDefenseModifier,
    SpeedCategory,
};
use crate::creatures::CreatureCategory;
use crate::creatures::{
    attacks::{Attack, HasAttacks},
    creature::Creature,
    Monster,
};
use crate::equipment::Weapon;
use crate::monsters::{ChallengeRating, CreatureType, Knowledge};
use crate::skills::{HasSkills, Skill};

pub struct FullMonsterDefinition {
    pub alignment: String,
    pub attributes: Vec<i32>,
    pub challenge_rating: ChallengeRating,
    pub creature_type: CreatureType,
    pub description: Option<&'static str>,
    pub knowledge: Option<Knowledge>,
    pub level: i32,
    pub movement_modes: Option<Vec<MovementMode>>,
    pub name: String,
    pub passive_abilities: Option<Vec<PassiveAbility>>,
    pub senses: Option<Vec<Sense>>,
    pub size: Size,
    pub special_attacks: Option<Vec<Attack>>,
    pub special_defense_modifiers: Option<Vec<SpecialDefenseModifier>>,
    pub trained_skills: Option<Vec<Skill>>,
    pub weapons: Vec<Weapon>,
}

impl FullMonsterDefinition {
    pub fn monster(self) -> Monster {
        let mut monster = Monster::new(self.challenge_rating, self.creature_type, self.level);
        if let Some(d) = self.description {
            monster.description = Some(d.to_string());
        }
        monster.movement_modes = self
            .movement_modes
            .unwrap_or(vec![MovementMode::Land(SpeedCategory::Normal)]);

        let creature = &mut monster.creature;
        creature.set_name(&self.name);
        for (i, attribute) in Attribute::all().iter().enumerate() {
            creature.set_base_attribute(attribute.clone(), self.attributes[i]);
        }
        for weapon in self.weapons {
            creature.weapons.push(weapon);
        }
        creature.set_size(self.size);
        if let Some(passive_abilities) = self.passive_abilities {
            for ability in passive_abilities {
                creature.passive_abilities.push(ability);
            }
        }
        if let Some(senses) = self.senses {
            for sense in senses {
                creature.add_sense(sense);
            }
        }
        if let Some(trained_skills) = self.trained_skills {
            for skill in trained_skills {
                creature.set_skill_trained(skill, true);
            }
        }
        if let Some(special_attacks) = self.special_attacks {
            for a in special_attacks {
                creature.add_special_attack(a);
            }
        }
        if let Some(special_defense_modifiers) = self.special_defense_modifiers {
            for d in special_defense_modifiers {
                creature.add_special_defense_modifier(d);
            }
        }

        return monster;
    }
}
