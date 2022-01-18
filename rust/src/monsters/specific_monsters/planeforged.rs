use crate::core_mechanics::abilities::{AbilityTag, AbilityType, ActiveAbility, StandardAttack};
use crate::core_mechanics::{
    DamageType, Debuff, FlightManeuverability, MovementMode, PassiveAbility, Sense, Size,
    SpecialDefenseType, SpeedCategory,
};
use crate::creatures::{calculate_standard_rank, Maneuver, Modifier, Monster};
use crate::equipment::{StandardWeapon, Weapon, WeaponMaterial};
use crate::monsters::challenge_rating::ChallengeRating;
use crate::monsters::creature_type::CreatureType::Planeforged;
use crate::monsters::knowledge::Knowledge;
use crate::monsters::monster_entry::MonsterEntry;
use crate::monsters::{monster_group, FullMonsterDefinition};
use crate::skills::Skill;

struct FullPlaneforgedDefinition {
    alignment: String,
    attributes: Vec<i32>,
    challenge_rating: ChallengeRating,
    description: Option<&'static str>,
    knowledge: Option<Knowledge>,
    level: i32,
    modifiers: Option<Vec<Modifier>>,
    movement_modes: Option<Vec<MovementMode>>,
    name: String,
    senses: Option<Vec<Sense>>,
    size: Size,
    trained_skills: Option<Vec<Skill>>,
    weapons: Vec<Weapon>,
}

impl FullPlaneforgedDefinition {
    fn monster(self) -> Monster {
        return FullMonsterDefinition {
            // From self
            alignment: self.alignment,
            attributes: self.attributes,
            challenge_rating: self.challenge_rating,
            description: self.description,
            knowledge: self.knowledge,
            level: self.level,
            modifiers: self.modifiers,
            movement_modes: self.movement_modes,
            name: self.name,
            senses: self.senses,
            size: self.size,
            trained_skills: self.trained_skills,
            weapons: self.weapons,

            // Default values
            creature_type: Planeforged,
        }
        .monster();
    }
}

pub fn planeforgeds() -> Vec<MonsterEntry> {
    let mut monsters: Vec<MonsterEntry> = vec![];

    add_angels(&mut monsters);

    add_demons(&mut monsters);

    add_elementals(&mut monsters);

    monsters.push(MonsterEntry::MonsterGroup(monster_group::MonsterGroup {
        name: "Imps".to_string(),
        knowledge: None,
        monsters: vec![FullPlaneforgedDefinition {
            alignment: "Always chaotic evil".to_string(),
            attributes: vec![2, 3, 2, 1, 0, -2],
            challenge_rating: ChallengeRating::Half,
            description: None,
            knowledge: None,
            level: 13,
            modifiers: None,
            movement_modes: None,
            name: "Flamefist Imp".to_string(),
            senses: None,
            size: Size::Small,
            trained_skills: None,
            weapons: vec![StandardWeapon::Slam
                .weapon()
                .except(|w| w.damage_types.push(DamageType::Fire))],
        }
        .monster()],
    }));

    monsters.push(MonsterEntry::Monster(
        FullPlaneforgedDefinition {
            alignment: "Always chaotic evil".to_string(),
            attributes: vec![3, 2, 3, 2, 4, 4],
            challenge_rating: ChallengeRating::Six,
            description: None,
            knowledge: None,
            level: 13,
            modifiers: Some(vec![
                Modifier::Attack(StandardAttack::Combustion(6).attack()),
                Modifier::Attack(StandardAttack::Inferno(6).attack()),
                Modifier::Attack(StandardAttack::Pyrohemia(6).attack()),
                Modifier::Attack(StandardAttack::Ignition(6).attack()),
                Modifier::Attack(StandardAttack::Pyrophobia(6).attack()),
                Modifier::Vulnerable(SpecialDefenseType::WeaponMaterial(WeaponMaterial::ColdIron)),
            ]),
            movement_modes: None,
            name: "Soulfire Demon".to_string(),
            senses: None,
            size: Size::Large,
            trained_skills: None,
            weapons: vec![StandardWeapon::HeavyFlail
                .weapon()
                .except(|w| w.damage_types.push(DamageType::Fire))],
        }
        .monster(),
    ));

    return monsters;
}

fn add_angels(monsters: &mut Vec<MonsterEntry>) {
    struct Angel {
        alignment: String,
        attributes: Vec<i32>,
        challenge_rating: ChallengeRating,
        knowledge: Option<Knowledge>,
        level: i32,
        modifiers: Option<Vec<Modifier>>,
        name: String,
        size: Size,
        trained_skills: Option<Vec<Skill>>,
        weapons: Vec<Weapon>,
    }

    impl Angel {
        fn monster(self) -> Monster {
            let rank = calculate_standard_rank(self.level, self.challenge_rating);
            let teleport_range = if rank >= 7 {
                "\\extrange"
            } else if rank >= 5 {
                "\\distrange"
            } else if rank >= 3 {
                "\\longrange"
            } else {
                "\\medrange"
            };

            let mut modifiers = self.modifiers.unwrap_or(vec![]);
            modifiers.push(Modifier::Immune(SpecialDefenseType::Debuff(
                Debuff::Shaken("".to_string()),
            )));
            modifiers.push(Modifier::Immune(SpecialDefenseType::Debuff(
                Debuff::Frightened("".to_string()),
            )));
            modifiers.push(Modifier::Immune(SpecialDefenseType::Debuff(
                Debuff::Panicked("".to_string()),
            )));
            modifiers.push(Modifier::Attack(
                StandardAttack::DivineJudgment(rank).attack(),
            ));
            modifiers.push(Modifier::Attack(
                StandardAttack::GlimpseOfDivinity(rank).attack(),
            ));
            modifiers.push(Modifier::Attack(StandardAttack::WordOfFaith(rank).attack()));
            modifiers.push(Modifier::ActiveAbility(ActiveAbility {
                ability_type: AbilityType::Instant,
                cooldown: None,
                effect: format!(
                    "
                        The $name teleports horizontally into an unoccupied location within {range}.
                        If the destination is invalid, this ability fails with no effect.
                    ",
                    range = teleport_range,
                ),
                is_magical: true,
                name: "Divine Translocation".to_string(),
                tags: None,
                usage_time: None,
            }));
            modifiers.push(Modifier::PassiveAbility(PassiveAbility {
                description: format!("
                    The $name can perform any ritual of rank {} or lower from the \\sphere{{bless}} or \\sphere{{channel divinity}} mystic spheres.
                    It does not need to expend material components or increase its \\glossterm{{fatigue level}} to perform those ritauls.
                ", rank),
                is_magical: true,
                name: "Divine Rituals".to_string(),
            }));

            return FullPlaneforgedDefinition {
                // From self
                alignment: self.alignment,
                attributes: self.attributes,
                challenge_rating: self.challenge_rating,
                knowledge: self.knowledge,
                level: self.level,
                modifiers: Some(modifiers),
                name: self.name,
                size: self.size,
                trained_skills: self.trained_skills,

                // Default values
                description: None,
                movement_modes: Some(vec![
                    MovementMode::Fly(SpeedCategory::Fast, FlightManeuverability::Perfect),
                    MovementMode::Land(SpeedCategory::Normal),
                ]),
                senses: Some(vec![Sense::Darkvision(120), Sense::LowLightVision]),
                weapons: self.weapons,
            }
            .monster();
        }
    }

    monsters.push(MonsterEntry::MonsterGroup(monster_group::MonsterGroup {
        knowledge: Some(Knowledge::new(vec![
            (-10, "
                Angels are the ultimate champions of good in the endless battle of good and evil.
                They are native to the Celestial Heavens, and they often serve the interests of good-aligned deities.
            "),
            (0, "
                All angels have a striking and highly memorable appearance that evokes strong emotions in most viewers.
                Most angels evoke an overpowering sense of awe and beauty, but individual angels may have highly varied appearances.
            "),
            (10, "
                In battle, angels are feared for their fundamental perfection.
                They tend not to have any weaknesses for attackers to use against them.
                Their only true foes are demons, who use overwhelming hordes rather than any clever tactics.
            "),
        ])),
        name: "Angels".to_string(),
        monsters: vec![
            Angel {
                alignment: "Always neutral good".to_string(),
                attributes: vec![5, 6, 4, 4, 4, 6],
                challenge_rating: ChallengeRating::Six,
                knowledge: Some(Knowledge::new(vec![
                    (0, "
                        Seraphim are six-winged angels of immense power.
                        They burn with holy fire, which they use to immolate evildoers.
                        A seraph resembles a massive serpent that leaves a trail of fire as it flies.
                    "),
                    (5, "
                        Despite their serpentine appearance, seraphim have beautiful singing voices.
                        They sing almost constaintly both in and out of combat.
                    "),
                ])),
                level: 16,
                modifiers: Some(vec![
                    Modifier::Attack(StandardAttack::Combustion(7).attack()),
                    Modifier::Attack(
                        Maneuver::TenderizingSmash(7)
                            .attack(StandardWeapon::MonsterRam.weapon())
                            .except_hit_damage(|w| w.damage_types.push(DamageType::Fire)),
                    ),
                ]),
                name: "Seraph".to_string(),
                size: Size::Huge,
                trained_skills: Some(vec![
                    Skill::Awareness,
                    Skill::Endurance,
                ]),
                weapons: vec![
                    StandardWeapon::MonsterBite.weapon()
                        .except(|w| w.damage_types.push(DamageType::Fire)),
                    StandardWeapon::MonsterRam.weapon()
                        .except(|w| w.damage_types.push(DamageType::Fire)),
                ],
            }
            .monster(),
            Angel {
                alignment: "Always lawful good".to_string(),
                attributes: vec![5, 5, 5, 4, 6, 4],
                challenge_rating: ChallengeRating::Six,
                knowledge: Some(Knowledge::new(vec![
                    (0, "
                        Justicars enforce justice on good-aligned planes.
                        They are extremely skilled at identifying the truth of any situation, and act to deal justice however they see fit.
                        Physically, a justicar appears similar to a large human with strong muscles and a constantly stern expression.
                    "),
                    (5, "
                        In rare circumstances, justicars may leave good-aligned planes to pursue those they see as exceptionally heinous criminals.
                        Generally, this requires that the perpetrator committed a direct offense against a good deity or desecrated an area of a good-aligned plane.
                        Justicars have no interest in mortal matters or minor crimes.
                    "),
                    (10, "
                        Once, a powerful group of thugs and murderers broke through a magic seal guarding an ancient wizard's tower, intending to loot everything inside.
                        They were shocked when a justicar suddenly appeared in front of them, and prepared to fight for their lives.
                        However, the justicar ignored them.
                        Instead, it murdered the ancient wizard of the tower and disappeared, leaving the spoils to the evildoers who broke the seal.

                        This is the morality of a justicar.
                        They consider only truly immense evils to be worthy of their attention, and ignore all lesser sins.
                    "),
                ])),
                level: 14,
                modifiers: Some(vec![
                    Modifier::Attack(
                        Maneuver::StripTheFlesh(6)
                            .attack(StandardWeapon::Greatsword.weapon())
                    ),
                ]),
                name: "Justicar".to_string(),
                size: Size::Large,
                trained_skills: Some(vec![
                    Skill::Awareness,
                    Skill::Deduction,
                    Skill::Endurance,
                    Skill::Intimidate,
                    Skill::SocialInsight,
                ]),
                weapons: vec![
                    StandardWeapon::Greatsword.weapon()
                        .except(|w| w.damage_types.push(DamageType::Energy)),
                ],
            }
            .monster(),
            Angel {
                alignment: "Always neutral good".to_string(),
                attributes: vec![4, 5, 7, 4, 4, 6],
                challenge_rating: ChallengeRating::Six,
                knowledge: Some(Knowledge::new(vec![
                    (0, "
                        Ophanim resemble burning wheels rimmed with many eyes.
                        They serve as sentries and guardians of planar portals in good-aligned planes.
                        In combat, they spin into a raging whirlwind.
                    "),
                ])),
                level: 12,
                modifiers: Some(vec![
                    Modifier::Attack(StandardAttack::Inferno(5).attack()),
                    Modifier::Attack(
                        Maneuver::Whirlwind(5, Size::Large.reach(false))
                            .attack(StandardWeapon::Slam.weapon())
                            .except_hit_damage(|w| w.damage_types.push(DamageType::Fire)),
                    ),
                ]),
                name: "Ophan".to_string(),
                size: Size::Large,
                trained_skills: Some(vec![
                    Skill::Awareness,
                    Skill::Endurance,
                ]),
                weapons: vec![
                    StandardWeapon::Slam.weapon()
                        .except(|w| w.damage_types.push(DamageType::Fire)),
                ],
            }
            .monster(),
        ],
    }));
}

fn add_demons(monsters: &mut Vec<MonsterEntry>) {
    struct Demon {
        alignment: String,
        attributes: Vec<i32>,
        challenge_rating: ChallengeRating,
        knowledge: Option<Knowledge>,
        level: i32,
        modifiers: Option<Vec<Modifier>>,
        movement_modes: Option<Vec<MovementMode>>,
        name: String,
        size: Size,
        trained_skills: Option<Vec<Skill>>,
        weapons: Vec<Weapon>,
    }

    impl Demon {
        fn monster(self) -> Monster {
            // let rank = calculate_standard_rank(self.level, self.challenge_rating);

            let mut modifiers = self.modifiers.unwrap_or(vec![]);
            modifiers.push(Modifier::Immune(SpecialDefenseType::Damage(
                DamageType::Fire,
            )));

            return FullPlaneforgedDefinition {
                // From self
                alignment: self.alignment,
                attributes: self.attributes,
                challenge_rating: self.challenge_rating,
                knowledge: self.knowledge,
                level: self.level,
                modifiers: Some(modifiers),
                movement_modes: self.movement_modes,
                name: self.name,
                size: self.size,
                trained_skills: self.trained_skills,

                // Default values
                description: None,
                senses: Some(vec![Sense::Darkvision(120), Sense::LowLightVision]),
                weapons: self.weapons,
            }
            .monster();
        }
    }

    let painborn_demon_spike = StandardWeapon::Slam
        .weapon()
        .except(|w| w.name = "Spike".to_string())
        .except(|w| w.damage_types = vec![DamageType::Piercing]);

    monsters.push(MonsterEntry::MonsterGroup(monster_group::MonsterGroup {
        knowledge: Some(Knowledge::new(vec![
            (0, "
                Demonspawn are infernal beings that live in the Abyss.
                They are the weakest and least intelligent type of demon, but they are still dangerous to mortals.
            "),
            (5, "
                Demonspawn were formed in the torturous flames of the Abyss.
                They all share an immunity to fire.
            "),
        ])),
        name: "Demonspawn".to_string(),
        monsters: vec![
            Demon {
                alignment: "Always chaotic evil".to_string(),
                attributes: vec![6, 4, 3, -4, 2, 4],
                challenge_rating: ChallengeRating::Four,
                knowledge: Some(Knowledge::new(vec![
                    (0, "
                        Rageborn demons are anger personified.
                        They lash out constantly and violently at everything around them.
                        If they are left alone, they simply destroy their environment.
                    "),
                    (5, "
                        Since rageborn demons normally feel only anger, they have little experience with other emotions.
                        This makes them easy to mislead with magical effects that manipulate their emotions.
                    "),
                ])),
                level: 5,
                modifiers: Some(vec![
                    Modifier::Attack(StandardAttack::Enrage(2).attack()),
                    Modifier::Maneuver(Maneuver::PowerFlurry(2)),
                    Modifier::Vulnerable(SpecialDefenseType::AbilityTag(
                        AbilityTag::Emotion,
                    )),
                ]),
                movement_modes: None,
                name: "Rageborn Demon".to_string(),
                size: Size::Large,
                trained_skills: Some(vec![
                    Skill::Endurance,
                ]),
                weapons: vec![
                    StandardWeapon::MonsterBite.weapon(),
                    StandardWeapon::MonsterClaws.weapon(),
                ],
            }
            .monster(),
            Demon {
                alignment: "Always neutral evil".to_string(),
                attributes: vec![3, 2, 6, -4, 1, 1],
                challenge_rating: ChallengeRating::Four,
                knowledge: Some(Knowledge::new(vec![
                    (0, "
                        Painborn demons are pain personified.
                        They are covered in spikes that pierce their own skin, shifting and causing them pain whenever they move.
                        These unfortunate creatures suffer continously, and they try to share that suffering with anything around them.
                    "),
                    (5, "
                        Painborn demons have a hidden desire that most of them do not even consciously realize: the desire to give up control.
                        Fighting through their constant pain is mentally taxing.
                        Magical effects that compel their actions, freeing them from the burden of choice, are their greatest weakness.
                    "),
                ])),
                level: 5,
                modifiers: Some(vec![
                    Modifier::Attack(
                        Maneuver::GraspingStrike(2).attack(StandardWeapon::MonsterClaws.weapon())
                        .except(|a| a.name = "Impale".to_string())
                    ),
                    Modifier::Attack(StandardAttack::MonsterSpikes(2).attack()),
                    Modifier::Vulnerable(SpecialDefenseType::AbilityTag(
                        AbilityTag::Compulsion,
                    )),
                ]),
                movement_modes: None,
                name: "Painborn Demon".to_string(),
                size: Size::Medium,
                trained_skills: Some(vec![
                    Skill::Endurance,
                ]),
                weapons: vec![painborn_demon_spike],
            }
            .monster(),
        ],
    }));
}

fn add_elementals(monsters: &mut Vec<MonsterEntry>) {
    struct FireElemental {
        attributes: Vec<i32>,
        challenge_rating: ChallengeRating,
        level: i32,
        modifiers: Vec<Modifier>,
        name: String,
        size: Size,
    }

    impl FireElemental {
        fn monster(mut self) -> Monster {
            self.modifiers
                .push(Modifier::Vulnerable(SpecialDefenseType::Damage(
                    DamageType::Cold,
                )));
            return FullPlaneforgedDefinition {
                // From self
                attributes: self.attributes,
                challenge_rating: self.challenge_rating,
                level: self.level,
                modifiers: Some(self.modifiers),
                name: self.name,
                size: self.size,

                // Default values
                alignment: "Usually chaotic neutral".to_string(),
                description: None,
                knowledge: None,
                movement_modes: Some(vec![MovementMode::Land(SpeedCategory::Fast)]),
                senses: None,
                trained_skills: None,
                weapons: vec![StandardWeapon::Slam
                    .weapon()
                    .except(|w| w.damage_types.push(DamageType::Fire))],
            }
            .monster();
        }
    }

    monsters.push(MonsterEntry::MonsterGroup(monster_group::MonsterGroup {
        knowledge: Some(Knowledge::new(vec![(
            0,
            "
                Fire elementals are formed from the pure essence of the Plane of Fire.
                They tend to be fast and agile, and they are usually vulnerable to cold.
            ",
        )])),
        name: "Fire Elementals".to_string(),
        monsters: vec![
            FireElemental {
                attributes: vec![2, 4, 0, -3, 0, 2],
                challenge_rating: ChallengeRating::One,
                level: 4,
                modifiers: vec![
                    Modifier::Attack(StandardAttack::Combustion(2).attack()),
                    Modifier::Attack(StandardAttack::Firebolt(2).attack()),
                ],
                name: "Ember".to_string(),
                size: Size::Small,
            }
            .monster(),
            FireElemental {
                attributes: vec![4, 5, 0, -2, 0, 2],
                challenge_rating: ChallengeRating::Two,
                level: 8,
                modifiers: vec![
                    Modifier::Attack(StandardAttack::Firebolt(3).attack()),
                    Modifier::Attack(StandardAttack::Combustion(3).attack()),
                    Modifier::Attack(StandardAttack::Ignition(3).attack()),
                    Modifier::Attack(StandardAttack::Fireball(3).attack()),
                ],
                name: "Kindled".to_string(),
                size: Size::Medium,
            }
            .monster(),
            FireElemental {
                attributes: vec![4, 5, 0, -2, 0, 2],
                challenge_rating: ChallengeRating::Two,
                level: 12,
                modifiers: vec![
                    Modifier::Attack(StandardAttack::Firebolt(4).attack()),
                    Modifier::Attack(StandardAttack::Combustion(4).attack()),
                    Modifier::Attack(StandardAttack::Ignition(4).attack()),
                    Modifier::Attack(StandardAttack::Fireball(4).attack()),
                ],
                name: "Bonfire".to_string(),
                size: Size::Large,
            }
            .monster(),
            FireElemental {
                attributes: vec![4, 6, 2, 1, 2, 4],
                challenge_rating: ChallengeRating::Four,
                level: 16,
                modifiers: vec![
                    Modifier::Attack(StandardAttack::Firebolt(6).attack()),
                    Modifier::Attack(StandardAttack::Combustion(6).attack()),
                    Modifier::Attack(StandardAttack::Ignition(6).attack()),
                    Modifier::Attack(StandardAttack::Fireball(6).attack()),
                ],
                name: "Elder".to_string(),
                size: Size::Large,
            }
            .monster(),
            FireElemental {
                attributes: vec![6, 6, 2, 2, 2, 4],
                challenge_rating: ChallengeRating::Four,
                level: 20,
                modifiers: vec![
                    Modifier::Attack(StandardAttack::Firebolt(7).attack()),
                    Modifier::Attack(StandardAttack::Combustion(7).attack()),
                    Modifier::Attack(StandardAttack::Ignition(7).attack()),
                    Modifier::Attack(StandardAttack::Fireball(7).attack()),
                ],
                name: "Inferno".to_string(),
                size: Size::Huge,
            }
            .monster(),
        ],
    }));

    struct MagmaElemental {
        attributes: Vec<i32>,
        challenge_rating: ChallengeRating,
        level: i32,
        modifiers: Vec<Modifier>,
        name: String,
        size: Size,
    }

    impl MagmaElemental {
        fn monster(mut self) -> Monster {
            self.modifiers
                .push(Modifier::Vulnerable(SpecialDefenseType::Damage(
                    DamageType::Piercing,
                )));
            self.modifiers
                .push(Modifier::Impervious(SpecialDefenseType::Damage(
                    DamageType::Cold,
                )));
            return FullPlaneforgedDefinition {
                // From self
                attributes: self.attributes,
                challenge_rating: self.challenge_rating,
                level: self.level,
                modifiers: Some(self.modifiers),
                name: self.name,
                size: self.size,

                // Default values
                alignment: "Usually true neutral".to_string(),
                description: None,
                knowledge: None,
                movement_modes: None,
                senses: None,
                trained_skills: None,
                weapons: vec![StandardWeapon::Slam
                    .weapon()
                    .except(|w| w.damage_types.push(DamageType::Fire))],
            }
            .monster();
        }
    }

    fn generate_magma_throw(rank: i32) -> Modifier {
        return Modifier::Attack(
            StandardAttack::Firebolt(rank)
                .attack()
                .except(|a| a.name = "Magma Throw".to_string())
                .except(|a| a.is_magical = false)
                .except_hit_damage(|d| d.damage_types.push(DamageType::Bludgeoning)),
        );
    }

    monsters.push(MonsterEntry::MonsterGroup(monster_group::MonsterGroup {
        knowledge: Some(Knowledge::new(vec![(
            0,
            "
                Magma elementals are a fusion of the Plane of Earth and the Plane of Fire.
                They combine the durability of earth elementals with some of the agility of fire elementals.
                Their outer shell appears rocky, but inside that shell they hold molten rock at incredible temperatures.
            ",
        ), (
            5,
            "
                Magma elementals lack the usual weaknesses of both fire elementals and earth elementals.
                Their massive internal heat, shielded from outside attack, actually makes them less vulnerable to cold.
                However, piercing attacks can penetrate their outer shell, causing the magma inside to spew out until it cools.
            ",
        )])),
        name: "Magma Elementals".to_string(),
        monsters: vec![
            MagmaElemental {
                attributes: vec![4, 4, 5, -4, 0, 0],
                challenge_rating: ChallengeRating::Two,
                level: 6,
                modifiers: vec![
                    Modifier::Attack(StandardAttack::Combustion(2).attack()),
                    generate_magma_throw(2),
                ],
                name: "Volcanite".to_string(),
                size: Size::Medium,
            }
            .monster(),
            MagmaElemental {
                attributes: vec![5, 5, 6, -3, 0, 0],
                challenge_rating: ChallengeRating::Four,
                level: 12,
                modifiers: vec![
                    Modifier::Attack(StandardAttack::Combustion(4).attack()),
                    generate_magma_throw(4),
                ],
                name: "Volcano".to_string(),
                size: Size::Large,
            }
            .monster(),
            MagmaElemental {
                attributes: vec![5, 5, 6, -3, 0, 0],
                challenge_rating: ChallengeRating::Four,
                level: 18,
                modifiers: vec![
                    Modifier::Attack(StandardAttack::Combustion(6).attack()),
                    generate_magma_throw(6),
                ],
                name: "Volcanic Titan".to_string(),
                size: Size::Huge,
            }
            .monster(),
        ],
    }));
}
