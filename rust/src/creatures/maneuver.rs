use crate::core_mechanics::abilities::attack_effect::{
    AttackEffectDuration, AttackTriggeredEffect, DebuffEffect,
};
use crate::core_mechanics::abilities::{AbilityMovement, Attack};
use crate::core_mechanics::{DamageType, Debuff, Defense, SpecialDefenseType, SpeedCategory};
use crate::equipment::Weapon;
use std::cmp::max;
use titlecase::titlecase;

#[derive(Clone)]
pub enum Maneuver {
    CertainStrike(i32),
    CrushingStrike(i32),
    ElementalStrike(i32),
    GenericScalingStrike(i32),
    GraspingStrike(i32),
    GreaterHamstring(i32),
    GreaterGraspingStrike(i32),
    Hamstring(i32),
    MightyStrike(i32),
    MonstrousStrike(i32),
    PenetratingStrike(i32),
    PouncingStrike(i32),
    StripTheFlesh(i32),
}

fn standard_damage_scaling(rank: i32) -> i32 {
    if rank >= 11 {
        return 24;
    } else if rank >= 9 {
        return 16;
    } else if rank >= 7 {
        return 8;
    } else if rank >= 5 {
        return 4;
    } else if rank >= 3 {
        return 2;
    } else {
        return 0;
    }
}

impl Maneuver {
    pub fn attack(&self, weapon: Weapon) -> Attack {
        let mut attack = match self {
            Self::CertainStrike(rank) => weapon
                .attack()
                .except(|a| a.accuracy += 2 + (rank - 1) / 2)
                .except_hit_damage(|d| d.power_multiplier = 0.5),
            Self::CrushingStrike(rank) => weapon
                .attack()
                .except(|a| a.accuracy -= 1)
                .except(|a| a.defense = Defense::Fortitude)
                .except_hit_damage(|d| d.damage_modifier += standard_damage_scaling(*rank)),
            // TODO: figure out how to use the higher of two powers
            Self::ElementalStrike(rank) => weapon
                .attack()
                .except_hit_damage(|d| {
                    d.damage_types
                        .append(&mut vec![DamageType::Bludgeoning, DamageType::Fire])
                })
                .except_hit_damage(|d| d.damage_modifier += standard_damage_scaling(*rank)),
            Self::GenericScalingStrike(rank) => weapon
                .attack()
                .except_hit_damage(|d| d.damage_modifier += standard_damage_scaling(*rank)),
            Self::GraspingStrike(rank) => weapon
                .attack()
                .except(|a| a.accuracy += (rank - 1) / 2)
                .except_hit_damage(|d| {
                    d.extra_defense_effect =
                        Some((Defense::Fortitude, AttackTriggeredEffect::Grappled));
                    d.power_multiplier = 0.0;
                }),
            Self::GreaterGraspingStrike(rank) => {
                assert_minimum_rank(5, rank, "Greater Grasping Strike");
                weapon
                    .attack()
                    .except(|a| a.accuracy += (rank - 5) / 2)
                    .except_hit_damage(|d| {
                        d.extra_defense_effect =
                            Some((Defense::Fortitude, AttackTriggeredEffect::Grappled));
                        d.power_multiplier = 0.5;
                    })
            }
            Self::GreaterHamstring(rank) => {
                assert_minimum_rank(6, rank, "Greater Hamstring");
                weapon
                    .attack()
                    .except(|a| a.accuracy += (rank - 3) / 2)
                    .except_hit_damage(|d| {
                        d.power_multiplier = 0.5;
                        d.take_damage_effect = Some(AttackTriggeredEffect::Debuff(DebuffEffect {
                            debuffs: vec![Debuff::Slowed],
                            duration: AttackEffectDuration::Condition,
                        }));
                    })
            }
            Self::Hamstring(rank) => {
                assert_minimum_rank(2, rank, "Hamstring");
                weapon
                    .attack()
                    .except(|a| a.accuracy += (rank - 1) / 2)
                    .except_hit_damage(|d| {
                        d.power_multiplier = 0.5;
                        d.lose_hp_effect = Some(AttackTriggeredEffect::Debuff(DebuffEffect {
                            debuffs: vec![Debuff::Slowed],
                            duration: AttackEffectDuration::Condition,
                        }));
                    })
            }
            Self::MightyStrike(rank) => weapon
                .attack()
                .except(|a| a.accuracy -= 2)
                .except_hit_damage(|d| d.damage_modifier += standard_damage_scaling(rank + 4)),
            Self::MonstrousStrike(rank) => weapon
                .attack()
                .except(|a| a.accuracy += max(0, (rank - 3) / 2)),
            Self::PenetratingStrike(rank) => weapon
                .attack()
                .except(|a| a.accuracy += (rank - 1) / 2)
                .except(|a| a.defense = Defense::Reflex)
                .except_hit_damage(|d| d.power_multiplier = 0.5),
            Self::PouncingStrike(rank) => weapon
                .attack()
                .except(|a| {
                    a.accuracy += (rank - 1) / 2;
                    a.movement = Some(AbilityMovement {
                        move_before_attack: true,
                        requires_straight_line: true,
                        speed: SpeedCategory::Normal,
                    })
                })
                .except_hit_damage(|d| d.power_multiplier = 0.5),
            Self::StripTheFlesh(rank) => {
                assert_minimum_rank(3, rank, "Strip the Flesh");
                weapon
                    .attack()
                    .except(|a| a.accuracy += (rank - 3) / 2)
                    .except_hit_damage(|d| {
                        d.power_multiplier = 0.0;
                        d.lose_hp_effect = Some(AttackTriggeredEffect::Debuff(DebuffEffect {
                            debuffs: vec![Debuff::Vulnerable(Box::new(
                                SpecialDefenseType::AllDamage,
                            ))],
                            duration: AttackEffectDuration::Condition,
                        }));
                    })
            }
        };
        attack.name = self.attack_name(&weapon);
        attack.replaces_weapon = if self.should_replace_weapon() {
            Some(weapon)
        } else {
            None
        };
        return attack;
    }

    pub fn attack_name(&self, weapon: &Weapon) -> String {
        let weapon_name = titlecase(weapon.name.as_str());
        match self {
            Self::CertainStrike(_) => format!("Certain {}", weapon_name),
            Self::CrushingStrike(_) => format!("Crushing {}", weapon_name),
            Self::ElementalStrike(_) => format!("Elemental {}", weapon_name),
            Self::GenericScalingStrike(_) => format!("Generic Scaling {}", weapon_name),
            Self::GraspingStrike(_) => format!("Grasping {}", weapon_name),
            Self::GreaterGraspingStrike(_) => format!("Greater Grasping {}", weapon_name),
            Self::GreaterHamstring(_) => format!("Greater Hamstring -- {}", weapon_name),
            Self::Hamstring(_) => format!("Hamstring -- {}", weapon_name),
            Self::MightyStrike(_) => format!("Mighty {}", weapon_name),
            Self::MonstrousStrike(_) => weapon_name,
            Self::PenetratingStrike(_) => format!("Penetrating {}", weapon_name),
            Self::PouncingStrike(_) => format!("Pouncing {}", weapon_name),
            Self::StripTheFlesh(_) => format!("Strip the Flesh -- {}", weapon_name),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::CertainStrike(_) => "Certain Strike",
            Self::CrushingStrike(_) => "Crushing Strike",
            Self::ElementalStrike(_) => "Elemental Strike",
            Self::GenericScalingStrike(_) => "Generic Scaling Strike",
            Self::GraspingStrike(_) => "Grasping Strike",
            Self::GreaterGraspingStrike(_) => "Greater Grasping Strike",
            Self::GreaterHamstring(_) => "Greater Hamstring",
            Self::Hamstring(_) => "Hamstring",
            Self::MightyStrike(_) => "Mighty Strike",
            Self::MonstrousStrike(_) => "Monstrous Strike",
            Self::PenetratingStrike(_) => "Penetrating Strike",
            Self::PouncingStrike(_) => "Pouncing Strike",
            Self::StripTheFlesh(_) => "Strip the Flesh",
        }
    }

    fn should_replace_weapon(&self) -> bool {
        match self {
            Self::MonstrousStrike(_) => true,
            _ => false,
        }
    }
}

fn assert_minimum_rank(minimum_rank: i32, actual_rank: &i32, name: &str) {
    if actual_rank < &minimum_rank {
        panic!(
            "Maneuver {} requires minimum rank {} instead of {}",
            name, minimum_rank, actual_rank
        );
    }
}
