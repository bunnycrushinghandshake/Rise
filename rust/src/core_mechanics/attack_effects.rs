use crate::core_mechanics::{damage_dice, damage_types, debuffs, HasCreatureMechanics};
use crate::equipment::weapons;
use crate::latex_formatting;

#[derive(Clone)]
pub enum AttackEffect {
    Damage(DamageEffect),
    Debuff(DebuffEffect),
    Poison(PoisonEffect),
}

#[derive(Clone)]
pub struct DamageEffect {
    pub damage_dice: damage_dice::DamageDice,
    pub damage_types: Vec<damage_types::DamageType>,
    pub damage_modifier: i8,
    pub lose_hp_effects: Option<Vec<AttackEffect>>,
    pub power_multiplier: f64,
    pub take_damage_effects: Option<Vec<AttackEffect>>,
}

#[derive(Clone)]
pub struct DebuffEffect {
    pub debuffs: Vec<debuffs::Debuff>,
    pub duration: AttackEffectDuration,
}

#[derive(Clone)]
pub struct PoisonEffect {
    pub stage1: Vec<debuffs::Debuff>,
    pub stage3: Option<Vec<debuffs::Debuff>>,
}

#[derive(Clone)]
pub enum AttackEffectDuration {
    Brief,
    Condition,
    NextRound,
}

impl AttackEffect {
    pub fn area_damage(rank: i8, damage_types: Vec<damage_types::DamageType>) -> Self {
        return Self::Damage(DamageEffect {
            damage_dice: damage_dice::DamageDice::new(damage_dice::D8 + rank - 1),
            damage_modifier: 0,
            damage_types,
            lose_hp_effects: None,
            power_multiplier: 0.5,
            take_damage_effects: None,
        });
    }

    pub fn from_weapon(weapon: weapons::Weapon) -> Self {
        return Self::Damage(DamageEffect {
            damage_dice: weapon.damage_dice(),
            damage_modifier: 0,
            damage_types: weapon.damage_types(),
            lose_hp_effects: None,
            power_multiplier: 1.0,
            take_damage_effects: None,
        });
    }

    pub fn description<T: HasCreatureMechanics>(
        &self,
        creature: &T,
        is_magical: bool,
        is_strike: bool,
    ) -> String {
        match self {
            Self::Damage(effect) => {
                let take_damage_effect = if let Some(ref t) = effect.take_damage_effects {
                    format!(
                        "
                            Each creature damaged by this attack is {effect}
                        ",
                        effect = latex_formatting::join_string_list(
                            &t.iter()
                                .map(|e| e.description(creature, is_magical, is_strike))
                                .collect()
                        )
                        .unwrap(),
                    )
                } else {
                    String::from("")
                };
                let lose_hp_effect = if let Some(ref t) = effect.lose_hp_effects {
                    format!(
                        "
                            Each creature that loses \\glossterm<hit points> from this attack is {effect}
                        ",
                        effect = latex_formatting::join_string_list(
                            &t.iter().map(|e| e.description(creature, is_magical, is_strike)).collect()
                        ).unwrap(),
                    )
                } else {
                    String::from("")
                };
                // TODO: damage types
                return format!(
                    "
                        {damage_dice}{damage_modifier} {damage_types} damage. {take_damage_effect} {lose_hp_effect}
                    ",
                    damage_dice = effect
                        .damage_dice
                        .add(creature.calc_damage_increments(is_strike))
                        .to_string(),
                    damage_modifier = latex_formatting::modifier(
                        effect.damage_modifier
                            + (creature.calc_power(is_magical) as f64 * effect.power_multiplier)
                                as i8,
                    ),
                    damage_types = latex_formatting::join_formattable_list(&effect.damage_types).unwrap_or(String::from("")),
                    take_damage_effect = take_damage_effect.trim(),
                    lose_hp_effect = lose_hp_effect.trim(),
                );
            }
            Self::Debuff(effect) => {
                return format!(
                    "{debuffs} {duration}.",
                    debuffs = effect
                        .debuffs
                        .iter()
                        .map(|d| d.latex_link())
                        .collect::<Vec<&str>>()
                        .join(", "),
                    duration = effect.duration.description(),
                );
            }
            Self::Poison(effect) => {
                let third_stage = if let Some(ref debuffs) = effect.stage3 {
                    format!(
                        "If a creature reaches the third poison stage, it becomes {debuffs} as long as it is poisoned.",
                        debuffs = latex_formatting::join_str_list(&debuffs.iter().map(|d| d.latex_link()).collect()).unwrap(),
                    )
                } else {
                    String::from("")
                };
                return format!(
                    "
                        \\glossterm<poisoned>.
                        As long as it is poisoned, it is {debuffs}.

                        At the end of each subsequent round, make an attack with the same accuracy against each poisoned creature's Fortitude defense, as normal for poisons (see \\pcref<Poison>).
                        {third_stage}
                    ",
                    debuffs = latex_formatting::join_str_list(&effect.stage1.iter().map(|d| d.latex_link()).collect()).unwrap(),
                    third_stage = third_stage,
                );
            }
        };
    }
}

impl AttackEffectDuration {
    pub fn description(&self) -> &str {
        match self {
            Self::Brief => "until the end of the next round",
            Self::Condition => "as a \\glossterm{condition}",
            Self::NextRound => "during the next round",
        }
    }
}