use std::cmp::max;

use crate::core_mechanics::abilities::ActiveAbility;
use crate::core_mechanics::attacks::Attack;
use crate::core_mechanics::{Attribute, HitPointProgression, MovementSpeed, PassiveAbility, Sense, Size, VitalWound};
use crate::creatures::{latex, IdentifiedModifier, Modifier};
use crate::equipment::{Armor, Weapon};
use crate::monsters::{ChallengeRating, Role};
use crate::skills::Skill;
use std::collections::HashMap;

use super::{HasModifiers, ModifierType};

#[derive(Clone, Debug)]
pub struct Creature {
    pub anonymous_modifiers: Vec<Modifier>,
    pub armor: Vec<Armor>,
    pub base_attributes: HashMap<Attribute, i32>,
    pub category: CreatureCategory,
    pub damage_resistance_lost: i32,
    pub hit_point_progression: HitPointProgression,
    pub hit_points_lost: i32,
    pub identified_modifiers: Vec<IdentifiedModifier>,
    pub level: i32,
    pub movement_speeds: Vec<MovementSpeed>,
    pub name: Option<String>,
    pub passive_abilities: Vec<PassiveAbility>,
    pub senses: Option<Vec<Sense>>,
    pub size: Size,
    pub skill_training: Option<HashMap<Skill, bool>>,
    pub special_attacks: Option<Vec<Attack>>,
    pub vital_wounds: Vec<VitalWound>,
    pub weapons: Vec<Weapon>,
}

#[derive(Clone, Debug)]
pub enum CreatureCategory {
    Character,
    Monster(ChallengeRating, Role),
}

impl Creature {
    pub fn new(level: i32, category: CreatureCategory) -> Creature {
        Creature {
            anonymous_modifiers: vec![],
            armor: vec![],
            base_attributes: HashMap::<Attribute, i32>::new(),
            category,
            damage_resistance_lost: 0,
            hit_point_progression: HitPointProgression::Low,
            hit_points_lost: 0,
            identified_modifiers: vec![],
            level,
            movement_speeds: vec![],
            name: None,
            passive_abilities: vec![],
            senses: None,
            size: Size::Medium,
            skill_training: None,
            special_attacks: None,
            vital_wounds: vec![],
            weapons: vec![],
        }
    }

    pub fn add_sense(&mut self, sense: Sense) {
        if self.senses.is_none() {
            self.senses = Some(vec![]);
        }
        self.senses.as_mut().unwrap().push(sense);
    }

    pub fn get_passive_abilities(&self) -> Vec<&PassiveAbility> {
        let mut passive_abilities: Vec<&PassiveAbility> = self.passive_abilities.iter().collect();
        for modifier in self.get_modifiers_by_type(ModifierType::PassiveAbility) {
            match modifier {
                Modifier::PassiveAbility(pa) => passive_abilities.push(pa),
                _ => {}
            };
        }
        passive_abilities
    }

    pub fn lowercase_name(&self) -> Option<String> {
        self.name.as_ref().map(|n| n.to_lowercase())
    }

    pub fn set_movement_speeds(&mut self, movement_speeds: Vec<MovementSpeed>) {
        self.movement_speeds = movement_speeds;
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    pub fn to_latex(&self) -> String {
        latex::format_creature(self)
    }

    pub fn is_character(&self) -> bool {
        match self.category {
            CreatureCategory::Character => true,
            CreatureCategory::Monster(..) => false,
        }
    }

    pub fn is_elite(&self) -> bool {
        match self.category {
            CreatureCategory::Character => false,
            CreatureCategory::Monster(cr, _) => cr == ChallengeRating::Four,
        }
    }

    pub fn can_recover(&self) -> bool {
        match self.category {
            CreatureCategory::Character => true,
            CreatureCategory::Monster(..) => false,
        }
    }

    pub fn rank(&self) -> i32 {
        calculate_standard_rank(self.level)
    }

    pub fn active_abilities(&self) -> Vec<ActiveAbility> {
        let mut active_abilities = vec![];
        for modifier in self.get_modifiers_by_type(ModifierType::ActiveAbility) {
            if let Modifier::ActiveAbility(a) = modifier {
                active_abilities.push(a.clone());
            }
        }
        active_abilities
    }
}

pub fn calculate_standard_rank(level: i32) -> i32 {
    max(0, (level + 2) / 3)
}

pub fn calculate_minimum_level(rank: i32) -> i32 {
    match rank {
        -1 => 0,
        0 => 0,
        1 => 1,
        2 => 4,
        3 => 7,
        4 => 10,
        5 => 13,
        6 => 16,
        7 => 19,
        8 => 21,
        _ => panic!("Unsupported rank {}", rank),
    }
}
