use crate::classes::archetype_rank_abilities::RankAbility;
use crate::core_mechanics::{Attribute, Defense, MovementMode};
use crate::creatures::Modifier;

use super::standard_modifiers::{add_standard_maneuver_modifiers, add_dr_scaling};

pub fn airdancer<'a>() -> Vec<RankAbility<'a>> {
    vec![
        RankAbility {
            name: "Float Like Air",
            is_magical: false,
            rank: 1,
            description: r"
                Your maximum jumping height is equal to your maximum horizontal jump distance, rather than half that distance (see \pcref{Jumping}).
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Float Like Air+",
            is_magical: false,
            rank: 6,
            description: r"
                Your maximum jumping height increases to twice your maximum horizontal jump distance.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Move Like Wind",
            is_magical: false,
            rank: 1,
            description: r"
                You gain a \plus10 foot bonus to your \glossterm{land speed}.
            ",
            modifiers: Some(vec![Modifier::MovementSpeed(MovementMode::Land, 10)]),
        },
        RankAbility {
            name: "Move Like Wind+",
            is_magical: false,
            rank: 6,
            description: r"
                The speed bonus increases to +20 feet.
            ",
            modifiers: Some(vec![Modifier::MovementSpeed(MovementMode::Land, 10)]),
        },
        RankAbility {
            name: "Heart of Air+",
            is_magical: true,
            rank: 4,
            description: r"
                When you jump, you can land in midair as if it was solid ground.
                Your landing location has a \glossterm{height limit} of 30 feet, like a fly speed (see \pcref{Flight}).
                You cannot walk in the air, but you can continue jumping or remain in place.
                The air holds you until the end of the current round, at which point you fall normally.
                After you land on air in this way, you \glossterm{briefly} cannot do so again.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Evasion",
            is_magical: false,
            rank: 2,
            description: r"
                You take no damage from \glossterm{glancing blows} or misses caused by abilities that affect an area and attack your Armor or Reflex defense.
                This does not protect you from any non-damaging effects of those abilities, or from abilities that affect multiple specific targets without affecting an area.
                If you have the \textit{evasion} rogue ability with the same effect as this ability, you also gain a \plus2 bonus to your Armor and Reflex defenses against area attacks.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Evasion+",
            is_magical: false,
            rank: 5,
            description: r"
                This ability also protects you from area attacks against your Fortitude and Mental defenses.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Aerial Strike",
            is_magical: false,
            rank: 3,
            // Expected jump height: 15 from speed, so can reliably jump completely
            // over Medium creatures.
            description: r"
                \begin{activeability}{Aerial Strike}
                    \abilityusagetime Standard action.
                    \rankline
                    You jump and move as normal for the jump (see \pcref{Jumping}).
                    In addition, you can make a \glossterm{strike} with +1d4 \glossterm{extra damage} at any point during that jump.
                    % TODO: is Jump clear enough about how to be directly above a creature?
                    This extra damage is doubled against each creature that you are directly above when you make the strike.

                    \rankline
                    % Weaker scaling than normal because the double damage is easy at high levels
                    \rank{4} The extra damage increases to 1d8.
                    \rank{5} The extra damage increases to 2d6.
                    \rank{6} The extra damage increases to 2d10.
                    \rank{7} The extra damage increases to 4d6.
                \end{activeability}
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Airdance",
            is_magical: true,
            rank: 7,
            description: r"
                You gain a \glossterm{fly speed} equal to your \glossterm{base speed} with a \glossterm{height limit} of 30 feet (see \pcref{Flight}).
                While flying, you can jump as if you were on solid ground, allowing you to rapidly gain height and change directions unexpectedly.
            ",
            modifiers: None,
        },
    ]
}

pub fn esoteric_warrior<'a>() -> Vec<RankAbility<'a>> {
    let mut abilities = vec![
        RankAbility {
            name: "Esoteric Maneuvers",
            is_magical: false,
            rank: 1,
            description: r"
                You can perform a wide variety of unusual attacks.
                You gain access to one of the following \glossterm{combat styles}: \textit{dirty fighting}, \textit{flurry of blows}, or \textit{mobile assault}.
                In addition, you gain access to any combat style of your choice (see \pcref{Combat Styles}).
                You may spend \glossterm{insight points} to gain access to one additional combat style per insight point.
                You can only learn esoteric \glossterm{maneuvers} from esoteric combat styles that you have access to.

                You learn two rank 1 esoteric \glossterm{maneuvers}.
                You may spend \glossterm{insight points} to learn one additional maneuver per insight point.
                Unless otherwise noted in an ability's description, using a maneuver requires a \glossterm{standard action}.

                When you gain access to a new \glossterm{rank} in this archetype,
                    you can exchange any number of maneuvers you know for other maneuvers,
                    including maneuvers of a higher rank.

                \advancement Some esoteric maneuvers also increase in power in unique ways based on your rank in this archetype, as indicated in their descriptions.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Esoteric Maneuvers+",
            is_magical: false,
            rank: 3,
            description: r"
                You learn an additional esoteric maneuver.
                In addition, you gain access to rank 3 esoteric maneuvers.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Esoteric Maneuvers+",
            is_magical: false,
            rank: 5,
            description: r"
                You gain access to rank 5 esoteric maneuvers.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Esoteric Maneuvers+",
            is_magical: false,
            rank: 7,
            description: r"
                You learn an additional esoteric maneuver.
                In addition, you gain access to rank 7 esoteric maneuvers.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Augmented Maneuvers",
            is_magical: false,
            rank: 2,
            description: r"
                You gain the ability to customize your esoteric maneuvers.
                For each rank 1 esoteric maneuver you know, choose one augment from the list below and apply it to that maneuver.
                Augments scale in power with your ``excess rank''.
                Your excess rank with a maneuver is equal to your rank in this archetype minus the rank of the maneuver.

                Whenever you increase your rank in this archetype, you can change your augments.
                However, you must still apply them to rank 1 esoteric maneuvers.
                {
                    \parhead{Counter Maneuver} You gain an accuracy bonus equal to twice your excess rank against creatures who made a \glossterm{strike} against you during the previous round.
                    You can only apply this augment to maneuvers which cause you to make a \glossterm{strike}.

                    \parhead{Debilitating Maneuver} You gain an accuracy bonus equal to twice your excess rank.
                    However, you cannot get a \glossterm{critical hit}.
                    You can only apply this augment to maneuvers which deal damage and can inflict a \glossterm{condition}.

                    \parhead{Mighty Maneuver} You take an accuracy penalty equal to 4 - your excess rank but the strike deals double \glossterm{weapon damage}.
                    If your excess rank is at least 5, this becomes an accuracy bonus.
                    You can only apply this augment to maneuvers which cause you to make a \glossterm{strike}.

                    \parhead{Mobile Maneuver} You can walk up to 5 feet per excess rank before or after using your chosen maneuver, up to a maximum distance equal to your land speed.
                    You cannot apply this augment to maneuvers that already allow you to move using one of your movement modes.

                    \parhead{Precise Maneuver} You gain an accuracy bonus equal to your excess rank.
                }
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Augmented Maneuvers+",
            is_magical: false,
            rank: 4,
            description: r"
                You can also choose an augment for each of your rank 3 esoteric maneuvers.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Augmented Maneuvers+",
            is_magical: false,
            rank: 6,
            description: r"
                You can also choose an augment for each of your rank 5 esoteric maneuvers.
            ",
            modifiers: None,
        },
    ];
    add_standard_maneuver_modifiers(&mut abilities);
    abilities
}

pub fn ki<'a>() -> Vec<RankAbility<'a>> {
    let mut abilities = vec![
        RankAbility {
            name: "Ki Manifestations",
            is_magical: true,
            rank: 2,
            description: r"
                You can channel your ki to temporarily enhance your abilities.
                Choose two \textit{ki manifestations} from the list below.
                You can also spend \glossterm{insight points} to learn one additional \textit{ki manifestation} per \glossterm{insight point}.

                After you use a \textit{ki manifestation}, you \glossterm{briefly} cannot use a \textit{ki manifestation} again.
                {
                    \begin{magicalactiveability}{Abandon the Fragile Self}[\abilitytag{Swift}]
                        \abilityusagetime Free action.
                        \rankline
                        You can negate one \glossterm{condition} that would be applied to you this phase.
                        In exchange, you take a \minus2 penalty to \glossterm{defenses} this phase.

                        \rankline
                        \rank{4} The defense penalty is reduced to \minus1.
                        \rank{6} The defense penalty is removed.
                    \end{magicalactiveability}

                    \begin{magicalactiveability}{Burst of Blinding Speed}
                        \abilityusagetime Free action.
                        \rankline
                        You gain a \plus10 foot bonus to your land speed this phase.

                        \rankline
                        \rank{4} You can also ignore \glossterm{difficult terrain} this phase.
                        \rank{6} The speed bonus increases to \plus20 feet.
                    \end{magicalactiveability}

                    \begin{magicalactiveability}{Calm the Inner Tempest}
                        \abilityusagetime Free action.
                        \rankline
                        You gain a \plus4 bonus to the Endurance skill this round (see \pcref{Endurance}).

                        \rankline
                        \rank{4} The bonus increases to \plus8.
                        \rank{6} The bonus increases to \plus12.
                    \end{magicalactiveability}

                    \begin{magicalactiveability}{Extend the Flow of Ki}
                        \abilityusagetime Free action.
                        \rankline
                        Your melee \glossterm{strikes} gain the \weapontag{Long} weapon tag this round, allowing you to attack targets up to 10 feet away from you (see \pcref{Weapon Tags}).

                        \rankline
                        \rank{4} You can attack enemies up to 15 feet away from you.
                        \rank{6} You can attack enemies up to 20 feet away from you.
                    \end{magicalactiveability}

                    \begin{magicalactiveability}{Flash Step}
                        \abilityusagetime See text.
                        \rankline
                        You can use this ability as part of movement with your land speed.
                        % TODO: is 'horizontally' the correct word?
                        You \glossterm{teleport} horizontally instead of moving normally.
                        If your \glossterm{line of effect} to your destination is blocked, or if this teleportation would somehow place you inside a solid object, your teleportation is cancelled and you remain where you are.

                        Teleporting a given distance costs movement equal to twice that distance.
                        For example, if you have a 30 foot movement speed, you can move 10 feet, teleport 5 feet, and move an additional 10 feet before your movement ends.

                        \rankline
                        \rank{4} The movement cost to teleport is reduced to be equal to the distance you teleport.
                        \rank{6} You can use this ability to move even if you are \immobilized or \grappled.
                    \end{magicalactiveability}

                    \begin{magicalactiveability}{Flurry of a Thousand Cuts}
                        \abilityusagetime Free action.
                        \rankline
                        When you make a \glossterm{strike} this round, you roll the attack roll twice and take the higher result.
                        However, you cannot get a \glossterm{critical hit} or \glossterm{glancing blow} with strikes.

                        \rankline
                        \rank{4} You also gain a +1 \glossterm{accuracy} bonus with strikes.
                        \rank{6} The accuracy bonus increases to +2.
                    \end{magicalactiveability}

                    \begin{magicalactiveability}{Hear the Rustling Wings}
                        \abilityusagetime Free action.
                        \rankline
                        You gain a \plus4 bonus to the Awareness skill this round (see \pcref{Awareness}).

                        \rankline
                        \rank{4} The bonus increases to \plus8.
                        \rank{6} The bonus increases to \plus12.
                    \end{magicalactiveability}

                    \begin{magicalactiveability}{Kindle the Living Flame}
                        \abilityusagetime Free action.
                        \rankline
                        Your \glossterm{strikes} deal fire damage in addition to their other damage types this round.

                        \rankline
                        \rank{4} You also gain +1d4 extra damage with strikes.
                        \rank{6} The extra damage increases to +1d6.
                    \end{magicalactiveability}

                    \begin{magicalactiveability}{Leap of the Heavens}
                        \abilityusagetime Free action.
                        \rankline
                        You gain a \plus10 foot bonus to your maximum horizontal jump distance (see \pcref{Jumping}).
                        This increases your maximum vertical jump distance normally.

                        \rankline
                        \rank{4} The bonus increases to \plus20 feet.
                        \rank{6} The bonus increases to \plus30 feet.
                    \end{magicalactiveability}

                    \begin{magicalactiveability}{Rest Atop the Precipice}
                        \abilityusagetime Free action.
                        \rankline
                        You gain a \plus4 bonus to the Balance skill this round (see \pcref{Balance}).

                        \rankline
                        \rank{4} The bonus increases to \plus8.
                        \rank{6} The bonus increases to \plus12.
                    \end{magicalactiveability}

                    \begin{magicalactiveability}{Scale the Highest Tower}
                        \abilityusagetime Free action.
                        \rankline
                        You gain a \plus4 bonus to the Climb skill this round (see \pcref{Climb}).
                        % TODO: is this wording correct?

                        \rankline
                        \rank{4} The Climb bonus increases to \plus8.
                        \rank{6} The bonus increases to \plus12.
                    \end{magicalactiveability}

                    \begin{magicalactiveability}{Shelter from Falling Rain}[\abilitytag{Swift}]
                        \abilityusagetime Free action.
                        \rankline
                        You gain a +2 bonus to your defenses against ranged \glossterm{strikes}.
                        However, you take a -2 penalty to your defenses against melee \glossterm{strikes}.

                        \rankline
                        \rank{4} The bonus increases to +3.
                        \rank{6} The bonus increases to +4.
                    \end{magicalactiveability}

                    \begin{magicalactiveability}{Step Between the Mystic Worlds}[\abilitytag{Swift}]
                        \abilityusagetime Free action.
                        \rankline
                        All attacks against you have a 20\% \glossterm{failure chance} this round.
                        However, your attacks also have a 20\% failure chance this round.

                        \rankline
                        \rank{4} The failure chance for attacks against you increases to 25\%.
                        \rank{6} The failure chance for attacks against you increases to 30\%.
                    \end{magicalactiveability}

                    \begin{magicalactiveability}{Thread the Eye of the Storm}
                        \abilityusagetime Free action.
                        \rankline
                        You reduce your \glossterm{longshot penalty} with thrown weapons by 1 this round (see \pcref{Weapon Range Limits}).

                        \rankline
                        \rank{4} The penalty reduction increases to 2.
                        \rank{6} The penalty reduction increases to 3.
                    \end{magicalactiveability}

                    \begin{magicalactiveability}{Surpass the Mortal Limits}[\abilitytag{Swift}]
                        \abilityusagetime Free action.
                        \rankline
                        You can add your Willpower to all \glossterm{checks} you make this phase that are based on Strength, Dexterity, or Constitution.
                        However, you take a \minus2 penalty to Strength, Dexterity, and Constitution checks during the next round.

                        \rankline
                        \rank{4} You also gain a \plus2 bonus to those checks while this effect lasts.
                        \rank{6} The penalty during the next round is removed.
                    \end{magicalactiveability}

                    % TODO: add more
                }
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Ki Energy",
            is_magical: true,
            rank: 1,
            description: r"
                Whenever you make a \glossterm{strike}, you can choose to treat it as a \magical ability.
                This allows you to use your \glossterm{magical power} to determine your damage instead of your \glossterm{mundane power} (see \pcref{Power}).
                In addition, that strike does not deal \glossterm{physical damage} or any physical damage subtypes.
                If the strike would normally deal one or more subtype of energy damage, the damage is of those types.
                Otherwise, all damage dealt by the strike is \glossterm{energy damage}.
                You can still use \glossterm{maneuvers} that require specific damage types, as long as you meet the requirements before this damage type conversion.
            ",
            // TODO: use higher of Str/Wil for strikes
            modifiers: None,
        },
        RankAbility {
            name: "Ki Barrier",
            is_magical: true,
            rank: 1,
            description: r"
                While you are not wearing other body armor, you gain a ki barrier around your body.
                This functions like body armor that provides a \plus2 bonus to your Armor defense and has no \glossterm{encumbrance}.
                It also provides a bonus to your \glossterm{damage resistance} equal to four times your rank in this archetype.

                You can also use a \glossterm{free hand} to wield the barrier as a shield.
                This functions like a buckler, granting you a \plus1 bonus to your Armor defense, except that you do not need to be proficient with light armor.
                Since this bonus comes from a shield, it does not stack with the benefits of using any other shield.
            ",
            // This only works if everyone with this archetype doesn't equip actual armor, since
            // the system won't know not to stack the effects
            modifiers: Some(vec![
                Modifier::Defense(Defense::Armor, 2),
            ]),
        },
        RankAbility {
            name: "Ki Barrier+",
            is_magical: true,
            rank: 4,
            description: r"
                The damage resistance bonus increases to five times your rank in this archetype, and the Armor defense bonus increases to \plus3.
            ",
            modifiers: Some(vec![
                Modifier::Defense(Defense::Armor, 1),
            ]),
        },
        RankAbility {
            name: "Ki Barrier++",
            is_magical: true,
            rank: 7,
            description: r"
                The damage resistance bonus increases to seven times your rank in this archetype.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Ki Manifestation+",
            is_magical: true,
            rank: 5,
            description: r"
                You learn an additional \textit{ki manifestation}.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Invested Strike",
            is_magical: true,
            rank: 3,
            description: r"
                \begin{magicalactiveability}{Invested Strike}
                    \abilityusagetime Standard action.
                    \abilitycost One \glossterm{fatigue level} (see text).
                    \rankline
                    Make a \glossterm{strike} that deals double \glossterm{weapon damage}.
                    The strike must target a single creature within \shortrange, with no secondary targets.

                    Whether or not the target takes damage, it becomes invested with your ki.
                    This does not cause it any ill effects.
                    If it dies or falls unconscious, or you take a \glossterm{short rest}, your ki returns to you.
                    When it does, you reduce your \glossterm{fatigue level} by one.

                    \rankline
                    \rank{4} You gain a +1 accuracy bonus with the strike.
                    \rank{5} The accuracy bonus increases to +2.
                    \rank{6} The strike deals triple \glossterm{weapon damage} instead of double weapon damage.
                    \rank{7} The accuracy bonus increases to +4.
                \end{magicalactiveability}
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Ki Power",
            is_magical: true,
            rank: 6,
            description: r"
                You gain a \plus1 bonus to your \glossterm{magical power}.
            ",
            modifiers: Some(vec![Modifier::MagicalPower(1)]),
        },
        RankAbility {
            name: "Endless Ki",
            is_magical: true,
            rank: 7,
            description: r"
                After using a \textit{ki manifestation}, you can use a different \textit{ki manifestation} after the end of the current round.
                You still cannot use the same \textit{ki manifestation} in two consecutive rounds.
            ",
            modifiers: None,
        },
    ];
    add_ki_barrier(&mut abilities);
    abilities
}

fn add_ki_barrier(abilities: &mut Vec<RankAbility<'_>>) {
    // 4x rank
    for rank in 1..4 {
        abilities.append(&mut vec![
            RankAbility {
                name: "Ki Barrier Scaling",
                rank,
                modifiers: Some(vec![Modifier::DamageResistance(rank * 4)]),
                ..Default::default()
            },
        ]);
    }

    // 5x rank
    for rank in 4..7 {
        abilities.append(&mut vec![
            RankAbility {
                name: "Ki Barrier Scaling",
                rank,
                modifiers: Some(vec![Modifier::DamageResistance(rank * 5)]),
                ..Default::default()
            },
        ]);
    }

    // 7x rank
    for rank in 7..8 {
        abilities.append(&mut vec![
            RankAbility {
                name: "Ki Barrier Scaling",
                rank,
                modifiers: Some(vec![Modifier::DamageResistance(rank * 7)]),
                ..Default::default()
            },
        ]);
    }
}

pub fn perfected_form<'a>() -> Vec<RankAbility<'a>> {
    vec![
        RankAbility {
            name: "Unarmed Warrior",
            is_magical: false,
            rank: 1,
            description: r"
                You gain a +2 accuracy bonus with the punch/kick \glossterm{natural weapon}, and you deal 1d4 damage with it (see \pcref{Natural Weapons}).
                In addition, you treat that weapon as having the \weapontag{Light} weapon tag, which allows you to make dual strikes with it more easily (see \pcref{Dual Strikes}).
            ",
            // TODO: selective bonus with only unarmed? It's easy enough to just give people
            // from this archetype weapons
            modifiers: None,
        },
        RankAbility {
            name: "Unarmed Warrior+",
            is_magical: false,
            rank: 4,
            description: r"
                Your punch/kick damage increases to 1d6.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Unarmed Warrior+",
            is_magical: false,
            rank: 7,
            description: r"
                Your punch/kick damage increases to 1d10.
            ",
            // TODO: At this point, you're probably using unarmed? This is weird.
            modifiers: Some(vec![Modifier::StrikeDamageDice(1)]),
        },
        RankAbility {
            name: "Unhindered Agility",
            is_magical: false,
            rank: 1,
            description: r"
                You gain a +1 bonus to your Armor defense while you have no \glossterm{encumbrance}.
            ",
            modifiers: Some(vec![
                Modifier::Defense(Defense::Armor, 1),
            ]),
        },
        RankAbility {
            name: "Unhindered Freedom",
            is_magical: false,
            rank: 4,
            description: r"
                While you have no \glossterm{encumbrance}, you are immune to being \slowed, \immobilized, and \paralyzed.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Unhindered Agility+",
            is_magical: false,
            rank: 7,
            description: r"
                The defense bonus increases to +2.
            ",
            modifiers: Some(vec![
                Modifier::Defense(Defense::Armor, 1),
            ]),
        },
        RankAbility {
            name: "Perfect Precision",
            is_magical: false,
            rank: 2,
            description: r"
                You gain a \plus1 bonus to \glossterm{accuracy} with attacks using weapons from the monk weapons \glossterm{weapon group}, natural weapons, and to any attack using one or more \glossterm{free hands}.
                This does not include abilities that simply use \glossterm{somatic components}.
                In addition, you can use your Dexterity instead of your Strength to determine your base accuracy with \abilitytag{Brawling} abilities (see \pcref{Special Combat Abilities}).
            ",
            modifiers: Some(vec![Modifier::Accuracy(1)]),
        },
        RankAbility {
            name: "Perfect Precision+",
            is_magical: false,
            rank: 5,
            description: r"
                The accuracy bonus increases to \plus2.
            ",
            modifiers: Some(vec![Modifier::Accuracy(1)]),
        },
        RankAbility {
            name: "Perfect Body",
            is_magical: false,
            rank: 3,
            description: r"
                Choose a physical \glossterm{attribute}: Strength, Dexterity, or Constitution (see \pcref{Attributes}).
                You gain a \plus1 bonus to that attribute.
            ",
            modifiers: Some(vec![Modifier::Attribute(Attribute::Dexterity, 1)]),
        },
        RankAbility {
            name: "Perfect Body+",
            is_magical: false,
            rank: 6,
            description: r"
                The bonus applies to all physical attributes, not just the one you chose.
            ",
            modifiers: Some(vec![
                Modifier::Attribute(Attribute::Strength, 1),
                Modifier::Attribute(Attribute::Constitution, 1),
            ]),
        },
    ]
}

pub fn transcendent_sage<'a>() -> Vec<RankAbility<'a>> {
    let mut abilities = vec![
        RankAbility {
            name: "Feel the Flow of Life",
            is_magical: true,
            rank: 1,
            description: r"
                You become so attuned to the natural energy of life that you can sense it even when sight fails you.
                You gain \trait{lifesense} with a 120 foot range, allowing you to sense the location of living creatures without light (see \pcref{Lifesense}).
                In addition, you gain \trait{lifesight} with a 30 foot range, allowing you to see living creatures without light (see \pcref{Lifesight}).
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Transcend Frailty",
            is_magical: false,
            rank: 2,
            description: r"
                You gain a bonus to your \glossterm{damage resistance} equal to three times your rank in this archetype.
                In addition, you gain a \plus1 bonus to your \glossterm{vital rolls} (see \pcref{Vital Wounds}).
            ",
            modifiers: Some(vec![Modifier::VitalRoll(1)]),
        },
        RankAbility {
            name: "Transcend Frailty+",
            is_magical: false,
            rank: 6,
            description: r"
                The damage resistance bonus increases to four times your rank in this archetype.
            ",
            // TODO: represent DR
            modifiers: None,
        },
        RankAbility {
            name: "Transcend Uncertainty",
            is_magical: false,
            rank: 3,
            description: r"
                You are immune to being \stunned and \confused.
            ",
            // TODO: represent immunities?
            modifiers: None,
        },
        RankAbility {
            name: "Transcend Emotion",
            is_magical: false,
            rank: 4,
            description: r"
                You are immune to \abilitytag{Emotion} attacks.
                In addition, you are immune to being \frightened and \panicked.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Transcend Mortality",
            is_magical: true,
            rank: 5,
            description: r"
                You are no longer considered a living creature for the purpose of attacks against you.
                This means that attacks which only affect living creatures have no effect against you.
                In addition, you no longer take penalties to your attributes for aging, and cannot be magically aged.
                You still die of old age when your time is up.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Feel the Flow of Life+",
            is_magical: true,
            rank: 6,
            description: r"
                The range of your lifesense increases by 240 feet, and the range of your lifesight increases by 60 feet.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Inner Transcendence",
            is_magical: false,
            rank: 7,
            description: r"
                You are immune to \glossterm{conditions}.
            ",
            modifiers: None,
        },
    ];
    add_dr_scaling(&mut abilities, 2, 6, None);
    abilities
}
