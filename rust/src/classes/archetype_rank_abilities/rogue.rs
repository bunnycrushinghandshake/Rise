use crate::classes::archetype_rank_abilities::RankAbility;
use crate::core_mechanics::Resource;
use crate::core_mechanics::attacks::Maneuver;
use crate::creatures::Modifier;
use crate::skills::{KnowledgeSubskill, Skill};

pub fn assassin<'a>() -> Vec<RankAbility<'a>> {
    return vec![
        RankAbility {
            name: "Sneak Attack",
            is_magical: false,
            rank: 1,
            description: r"
                You can use the \textit{sneak attack} ability as a standard action.
                \begin{activeability}{Sneak Attack}
                    \rankline
                    Make a \glossterm{strike} with a \glossterm{light weapon} or any weapon with the Stealthy \glossterm{weapon tag} against a creature within \rngshort range.

                    If the target is \unaware or \partiallyunaware of your attack, or if the target is adjacent to one of your \glossterm{allies}, you gain two benefits.
                    First, you gain a \plus2 damage bonus with the strike.
                    Second, if you get a \glossterm{critical hit}, you double all of your damage bonuses along with your damage dice.
                    You do not gain these benefits against creatures that you are unable to score a \glossterm{critical hit} against, such as excessively large creatures or oozes.

                    \rankline
                    \rank{3} The damage bonus increases to \plus4.
                    \rank{5} The damage bonus increases to \plus8.
                    \rank{7} The damage bonus increases to \plus16.
                \end{activeability}

                \advancement At ranks 3, 5, and 7, this ability improves as described above.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Evasion",
            is_magical: false,
            rank: 2,
            description: r"
                You take half damage from abilities that affect an area and attack your Armor or Reflex defense.
                This does not protect you from any non-damaging effects of those abilities, or from abilities that affect multiple specific targets without affecting an area.

                \advancement At rank 6, this ability also protects you from area attacks against your Fortitude and Mental defenses.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Evasion+",
            is_magical: false,
            rank: 6,
            description: r"
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Darkstalker",
            is_magical: false,
            rank: 3,
            description: r"
                You can use the \textit{darkstalker} ability as a standard action.
                \begin{attuneability}{Darkstalker}
                    \rankline
                    You become completely undetectable by your choice of one of the following sense groups:
                    \begin{itemize}
                        \item \trait{Blindsense} and \trait{blindsight}
                        \item \trait{Darkvision}
                        \item \abilitytag{Detection} abilities
                        \item \trait{Lifesense} and \trait{lifesight}
                        \item \trait{Scent}
                        \item \abilitytag{Scrying} abilities
                        \item \trait{Tremorsense} and \trait{tremorsight}
                    \end{itemize}
                    If you have access to any other more unusual senses, such as the \textit{blood sense} ability from the Executioner feat, you may also choose one of those senses as a separate sense group.
                \end{attuneability}

                \advancement At rank 5, you can attune to this ability multiple times.
                Each time, you can choose a different sense group.
                At rank 7, when you use this ability, you become undetectable by any number of the possible sense groups rather than only one.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Darkstalker+",
            is_magical: false,
            rank: 5,
            description: r"
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Darkstalker+",
            is_magical: false,
            rank: 7,
            description: r"
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Assassin's Finesse",
            is_magical: false,
            rank: 3,
            description: r"
                You gain a \plus1d bonus to your damage with \glossterm{light weapons} and any weapon with the Stealthy \glossterm{weapon tag}.

                \advancement At rank 6, this bonus increases to \plus2d.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Assassin's Finesse+",
            is_magical: false,
            rank: 6,
            description: r"
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Hide in Plain Sight",
            is_magical: false,
            rank: 4,
            description: r"
                You can use the \textit{hide} ability without moving in a way that causes observers to lose sight of you.
                This does not remove the bonus that observers receive if you have no cover or concealment at all.
                After you hide in this way, you \glossterm{briefly} cannot do so again.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Assassination",
            is_magical: false,
            rank: 5,
            description: r"
                You can use the \textit{assassination} ability as a \glossterm{minor action}.
                \begin{activeability}{Assassination}
                    \abilitytag{Swift}
                    \rankline
                    You study a creature within \rngmed range, finding weak points you can take advantage of.
                    As a \glossterm{brief} effect, whenever you make a melee \glossterm{strikes} against the target that it is \unaware, the strike deals maximum damage and automatically \glossterm{explodes} regardless of what you roll.
                \end{activeability}
            ",
            modifiers: None,
        },
    ];
}

pub fn bardic_music<'a>() -> Vec<RankAbility<'a>> {
    return vec![
        RankAbility {
            name: "Bardic Lore",
            is_magical: false,
            rank: 1,
            description: r"
                You gain all Knowledge skills as \glossterm{class skills}.
                In addition, you gain a bonus equal to your rank in this archetype to Knowledge skills that you are \glossterm{untrained} with.

                \advancement At rank 6, you gain a \plus2 bonus to all Knowledge skills.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Bardic Lore+",
            is_magical: true,
            rank: 6,
            description: r"
            ",
            modifiers: Some(vec![Modifier::Skill(
                Skill::Knowledge(vec![
                    KnowledgeSubskill::Arcana,
                    KnowledgeSubskill::Dungeoneering,
                    KnowledgeSubskill::Engineering,
                    KnowledgeSubskill::Items,
                    KnowledgeSubskill::Local,
                    KnowledgeSubskill::Nature,
                    KnowledgeSubskill::Planes,
                    KnowledgeSubskill::Religion,
                ]),
                2,
            )]),
        },
        RankAbility {
            name: "Bardic Performances",
            is_magical: true,
            rank: 1,
            description: r"
                You learn two \textit{bardic performances} from the list below.
                You can also spend \glossterm{insight points} to learn one additional bardic performance per \glossterm{insight point}.
                You can use any bardic performance you know as a \glossterm{standard action} unless it specifies that it requires a different type of action to activate.

                All \textit{bardic performances} have the \abilitytag{Auditory} tag.
                When you use a \textit{bardic performance} ability, you begin a performance using one of your Perform skills.
                You must use either an instrumental performance or a vocal performance, and not a visual performance.
                If you use a vocal performance, the bardic performance gains the \abilitytag{Speech} tag, preventing it from affecting creatures that do not speak the language you perform in.
                If you are \glossterm{trained} with a Perform skill capable of making an auditory performance, you gain a \plus1 bonus to \glossterm{accuracy} with any \textit{bardic performance} ability using that perform skill.

                The names of bardic performances do not have to precisely match your actual performance.
                For example, you can use the \textit{palliative poem} ability with a gentle song using Perform (wind instruments) or a distracting joke using Perform (comedy) instead of a poem.

                Many bardic performances require you to sustain the performance as a \glossterm{minor action}.
                If the targets stop being able to see or hear you, depending on the nature of your performance, the effect ends for them as if you had stopped sustaining the performance.
                However, targets do not stop being affected by your performance simply by travelling beyond the initial range of the bardic performance ability.
                Using a bardic performance ability with an immediate effect does not interfere with your ability to sustain other bardic performance abilities.
                {
                    \begin{sustainability}{Aria of Alacrity}[\abilitytag{Sustain} (minor)]
                        \abilitytag{Auditory}
                        \par \noindent Usage time: One \glossterm{minor action}.
                        \rankline
                        Choose one \glossterm{ally} within \medrange.
                        For the duration of your performance, the target gains a \plus5 foot bonus to its speed with all of its \glossterm{movement modes}.

                        \rankline
                        \rank{3} The target also gains a \plus2 bonus to its Reflex defense.
                        \rank{5} The speed bonus increases to \plus10 feet.
                        \rank{7} The bonus to Reflex defense increases to \plus4.
                    \end{sustainability}

                    % Bardic performance power guidelines:
                    % These generally start from the same rank 1 baseline effect as spells.
                    % Since there are no higher rank bardic performances, they need more aggressive rank scalings to ensure
                    % that a rank 7 bardic performance is comparable to a rank 7 spell. That is provided by the greater/supreme
                    % bardic performance class abilities, and doesn't need to be included in each individual performance.
                    % In general, bardsongs are likely to trade damage or accuracy for increased area.
                    %
                    % Bardsong debuffs are interesting, since they can't be removed like conditions, but also can't be stacked.
                    % For now, they're just ranked in the same way as conditions.
                    \begin{sustainability}{Ballad of Belligerence}[\abilitytag{Sustain} (minor)]
                        \abilitytag{Auditory}, \abilitytag{Emotion}
                        \rankline
                        Make an attack vs. Mental against one creature within \medrange.
                        \hit For the duration of your performance, the target is unable to take any \glossterm{standard actions} that do not cause it to make an attack.
                        For example, it could make a \glossterm{strike} or cast an offensive spell, but it could not heal itself or summon a creature.

                        \rankline
                        You gain a \plus2 bonus to \glossterm{accuracy} with the attack for each rank beyond 1.
                    \end{sustainability}

                    \begin{activeability}{Boastful Bravura}
                        \abilitytag{Auditory}
                        \rankline
                        This ability affects all \glossterm{enemies} within a \arealarge radius from you.
                        You \glossterm{briefly} gain a \plus4 bonus to \glossterm{accuracy} with \textit{bardic performance} abilities against each target.

                        \rankline
                        \rank{3} The bonus increases to \plus5.
                        \rank{5} The bonus increases to \plus6.
                        \rank{7} The bonus increases to \plus7.
                    \end{activeability}

                    \begin{activeability}{Cacaphony}
                        \abilitytag{Auditory}
                        \rankline
                        Make an attack vs. Fortitude against all \glossterm{enemies} in a \tinyarea radius from you.
                        \hit Each target takes 1d4 \add half \glossterm{power} sonic damage.

                        \rankline
                        \rank{2} The damage increases to 1d6, and the area increases to a \smallarea radius.
                        \rank{3} The damage increases to 1d8, and the area increases to a \medarea radius.
                        \rank{4} The damage increases to 1d10.
                        \rank{5} The damage increases to 2d6, and the area increases to a \largearea radius.
                        \rank{6} The damage increases to 2d8.
                        \rank{7} The damage increases to 2d10, and the area increases to a \hugearea radius.
                    \end{activeability}

                    \begin{sustainability}{Cadenza of Courage}[Sustain (minor)]
                        \abilitytag{Auditory}, \abilitytag{Emotion}
                        \par \noindent Usage time: One \glossterm{minor action}.
                        \rankline
                        Choose one \glossterm{ally} within \medrange.
                        For the duration of your performance, the target gains a \plus1 bonus to \glossterm{accuracy}.

                        \rankline
                        \rank{3} The target also gains a \plus2 bonus to its Mental defense.
                        \rank{5} The accuracy bonus increases to \plus2.
                        \rank{7} The bonus to Mental defense increases to \plus4.
                    \end{sustainability}

                    \begin{sustainability}{Cantata of Caution}[Sustain (minor)]
                        \abilitytag{Auditory}, \abilitytag{Emotion}
                        \par \noindent Usage time: One \glossterm{minor action}.
                        \rankline
                        Choose one \glossterm{ally} within \medrange.
                        For the duration of your performance, the target gains a \plus1 bonus to its Armor and Reflex defenses.

                        \rankline
                        \rank{3} The bonus applies to all defenses.
                        \rank{5} The bonus to Armor and Reflex defenses increases to \plus2.
                        \rank{7} The bonus to Fortitude and Mental defenses also increases to \plus2.
                    \end{sustainability}

                    \begin{sustainability}{Cleansing Counterpoint}[Sustain (minor)]
                        \abilitytag{Auditory}
                        \rankline
                        Choose yourself or one \glossterm{ally} within \rngmed range.
                        The target chooses one of its \glossterm{brief} effects or \glossterm{conditions}.
                        It cannot choose an effect applied during the current round.
                        For the duration of your performance, the target ignores that effect.

                        \rankline
                        \rank{3} You can target an additional \glossterm{ally} within range.
                        \rank{5} This ability loses the \abilitytag{Sustain} (minor) tag.
                            Instead, the chosen effect is removed entirely.
                        \rank{7} Each target can remove two effects instead of one.
                    \end{sustainability}

                    \begin{activeability}{Dazzling Discordance}
                        \abilitytag{Auditory}
                        \rankline
                        Make an attack vs. Mental against all \glossterm{enemies} within a \areamed radius from you.
                        \hit Each target is \glossterm{briefly} \dazzled.
                        \crit The effect becomes a \glossterm{condition}.

                        \rankline
                        \rank{2} You gain a \plus1 \glossterm{accuracy} bonus with the attack, and the area increases to a \largearea radius.
                        \rank{3} The accuracy bonus increases to \plus2.
                        \rank{4} The accuracy bonus increases to \plus3, and the area increases to a \hugearea radius.
                        \rank{5} The accuracy bonus increases to \plus4.
                        \rank{6} The accuracy bonus increases to \plus5, and the area increases to a \gargarea radius.
                        \rank{7} The accuracy bonus increases to \plus6.
                    \end{activeability}

                    \begin{activeability}{Dirge of Doom}
                        \abilitytag{Auditory}
                        \rankline
                        Make an attack vs. Mental against anything within \medrange.
                        \hit The target takes sonic damage equal to 1d8 plus your \glossterm{power}.

                        \rankline
                        \rank{2} The damage increases to 1d10.
                        \rank{3} The damage increases to 2d8.
                        \rank{4} The damage increases to 2d10.
                        \rank{5} The damage increases to 4d8.
                        \rank{6} The damage increases to 4d10.
                        \rank{7} The damage increases to 6d10.
                    \end{activeability}

                    \begin{sustainability}{Dizzying Ditty}[\abilitytag{Sustain} (minor)]
                        \abilitytag{Auditory}, \abilitytag{Compulsion}
                        \rankline
                        Make an attack vs. Mental against one creature within \medrange.
                        \hit For the duration of your performance, the target is \dazed.
                        \crit For the duration of your performance, the target is \stunned.

                        \rankline
                        You gain a \plus2 bonus to \glossterm{accuracy} with the attack for each rank beyond 1.
                    \end{sustainability}

                    \begin{sustainability}{Frightening Fugue}[\abilitytag{Sustain} (minor)]
                        \abilitytag{Auditory}, \abilitytag{Emotion}
                        \rankline
                        Make an attack vs. Mental against one creature within \medrange.
                        \hit The target takes 1d4 sonic damage.
                        If it loses \glossterm{hit points} from this damage, it is \frightened by you for the duration of your performance.
                        Unlike normal, the target continues to suffer the penalties of being frightened while it is beyond \rngmed range from you as long as it is still affected by your performance.

                        \rankline
                        You gain a \plus1 \glossterm{accuracy} bonus and a \plus1d damage bonus with the attack for each rank beyond 1.
                    \end{sustainability}

                    \begin{sustainability}{Hypnotic Hymn}[\abilitytag{Sustain} (minor)]
                        \abilitytag{Auditory}, \abilitytag{Emotion}
                        \rankline
                        Make an attack vs. Mental against one creature within \medrange.
                        You take a \minus10 penalty to \glossterm{accuracy} with this attack against creatures who have made an attack or been attacked since the start of the last round, including during the current phase.
                        \hit For the duration of your performance, the target is \charmed by you.
                        Any act by you or by creatures that appear to be your allies that threatens or harms the charmed person breaks the effect.
                        Harming the target is not limited to dealing it damage, but also includes causing it significant subjective discomfort.
                        An observant target may interpret overt threats to its allies as a threat to itself.
                        This ability does not have the \abilitytag{Subtle} tag, so an observant target may notice it is being influenced.

                        \rankline
                        You gain a \plus2 bonus to \glossterm{accuracy} with the attack for each rank beyond 1.
                    \end{sustainability}

                    \begin{sustainability}{Intonation of Ingenuity}[\abilitytag{Sustain} (minor)]
                        \abilitytag{Auditory}, \abilitytag{Emotion}
                        \par \noindent Usage time: One \glossterm{minor action}.
                        \rankline
                        Choose yourself or one \glossterm{ally} within \rngmed range.
                        For the duration of your performance, the target gains a \plus2 bonus to \glossterm{checks}.

                        \rankline
                        \rank{3} The bonus increases to \plus3.
                        \rank{5} The bonus increases to \plus4.
                        \rank{7} The bonus increases to \plus5.
                    \end{sustainability}

                    \begin{activeability}{Palliative Poem}
                        \abilitytag{Auditory}
                        \rankline
                        Choose one living \glossterm{ally} within \medrange.
                        The target regains 1d6 \add \glossterm{power} \glossterm{damage resistance} and increases its \glossterm{fatigue level} by one.
                        In addition, it \glossterm{briefly} gains a +2 bonus to its Mental defense.

                        \rankline
                        \rank{2} The recovery increases to 1d8.
                        \rank{3} The recovery increases to 2d6.
                        \rank{4} The recovery increases to 2d8.
                        \rank{5} The recovery increases to 4d6.
                        \rank{6} The recovery increases to 4d8.
                        \rank{7} The recovery increases to 5d10.
                    \end{activeability}

                    \begin{activeability}{Partita of Provocation}
                        \abilitytag{Auditory}, \abilitytag{Emotion}
                        \rankline
                        Make an attack vs. Mental against all \glossterm{enemies} within a \medarea radius from you.
                        \hit Each target is \glossterm{briefly} \goaded by you.
                        \crit The effect becomes a \glossterm{condition}.

                        \rankline
                        You gain a \plus2 bonus to \glossterm{accuracy} with the attack for each rank beyond 1.
                    \end{activeability}

                    \begin{sustainability}{Serenade of Serenity}[\abilitytag{Sustain} (minor)]
                        \abilitytag{Auditory}, \abilitytag{Emotion}
                        \par \noindent Usage time: One \glossterm{minor action}.
                        \rankline
                        For the duration of your performance, you and all \glossterm{allies} within a \largearea radius from you gain a \plus4 bonus to their defenses against \abilitytag{Compulsion} and \abilitytag{Emotion} attacks.

                        \rankline
                        \rank{3} At the end of each round, each target removes all \glossterm{conditions} caused by Compulsion and Emotion effects that were not applied during that round.
                        \rank{5} The area increases to a \areahuge radius.
                        \rank{7} Each target is immune to Compulsion and Emotion attacks.
                    \end{sustainability}

                    \begin{activeability}{Stutterstep Staccato}
                        \abilitytag{Auditory}
                        \rankline
                        Make an attack vs. Fortitude against all \glossterm{enemies} within a \areamed radius from you.
                        \hit Each target is \glossterm{briefly} \slowed.
                        \crit Each target is \slowed as a \glossterm{condition}.

                        \rankline
                        \rank{2} You gain a \plus1 \glossterm{accuracy} bonus with the attack, and the area increases to a \largearea radius.
                        \rank{3} The accuracy bonus increases to \plus2.
                        \rank{4} The accuracy bonus increases to \plus3, and the area increases to a \hugearea radius.
                        \rank{5} The accuracy bonus increases to \plus4.
                        \rank{6} The accuracy bonus increases to \plus5, and the area increases to a \gargarea radius.
                        \rank{7} The accuracy bonus increases to \plus6.
                    \end{activeability}

                    \begin{activeability}{Vigorous Verse}
                        \abilitytag{Auditory}
                        \par \noindent Usage time: One \glossterm{minor action}.
                        \rankline
                        Choose one \glossterm{ally} within \rngmed range.
                        For the duration of your performance, the target gains a \plus4 bonus to its maximum \glossterm{hit points}.
                        In addition, it immediately gains that many hit points.
                        When this effect ends, the target loses hit points equal to the hit points it gained this way.

                        \rankline
                        \rank{3} The bonus increases to \plus8.
                        \rank{5} The bonus increases to \plus16.
                        \rank{7} The bonus increases to \plus32.
                    \end{activeability}
                }

                \advancement At ranks 3, 5, and 7, each bardic performance improves as described above.
                Some bardic performances improve at every rank instead.
                At rank 5, you learn an additional bardic performance.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Musical Tolerance",
            is_magical: false,
            rank: 2,
            description: r"
                You are \glossterm{impervious} to \abilitytag{Auditory} attacks.

                \advancement At rank 5, you become \glossterm{immune} to \abilitytag{Auditory} attacks.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Musical Tolerance+",
            is_magical: false,
            rank: 5,
            description: r"
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Bardic Training",
            is_magical: false,
            rank: 3,
            description: r"
                You gain an additional \glossterm{trained} skill (see \pcref{Trained Skills}).
            ",
            modifiers: Some(vec![Modifier::Resource(Resource::TrainedSkill, 1)]),
        },
        RankAbility {
            name: "Martial Performance",
            is_magical: false,
            rank: 3,
            description: r"
                You gain a \plus1d bonus to your damage with all weapons.

                \advancement At rank 6, this bonus increases to \plus2d.
            ",
            modifiers: Some(vec![Modifier::StrikeDamageDice(1)]),
        },
        RankAbility {
            name: "Martial Performance+",
            is_magical: false,
            rank: 6,
            description: r"
            ",
            modifiers: Some(vec![Modifier::StrikeDamageDice(1)]),
        },
        RankAbility {
            name: "Virtuoso",
            is_magical: true,
            rank: 4,
            description: r"
                Once per round, you can \glossterm{sustain} two bardic performances as a single \glossterm{minor action}.

                \advancement At rank 7, you can sustain up to three bardic performances in this way instead of only two.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Virtuoso+",
            is_magical: true,
            rank: 7,
            description: r"
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Bardic Performance",
            is_magical: true,
            rank: 5,
            description: r"
                You learn an additional bardic performance.
            ",
            modifiers: None,
        },
    ];
}

pub fn combat_trickster<'a>() -> Vec<RankAbility<'a>> {
    return vec![
        RankAbility {
            name: "Maneuvers",
            is_magical: false,
            rank: 1,
            description: "",
            modifiers: Some(vec![
                Modifier::Maneuver(Maneuver::CertainStrike(1)),
                Modifier::Maneuver(Maneuver::GenericScalingStrike(1)),
                Modifier::Maneuver(Maneuver::MightyStrike(1)),
            ]),
        },
        RankAbility {
            name: "Maneuvers",
            is_magical: false,
            rank: 3,
            description: "",
            modifiers: Some(vec![
                Modifier::Maneuver(Maneuver::CertainStrike(3)),
                Modifier::Maneuver(Maneuver::GenericScalingStrike(3)),
                Modifier::Maneuver(Maneuver::MightyStrike(3)),
            ]),
        },
        RankAbility {
            name: "Maneuvers",
            is_magical: false,
            rank: 5,
            description: "",
            modifiers: Some(vec![
                Modifier::Maneuver(Maneuver::CertainStrike(5)),
                Modifier::Maneuver(Maneuver::GenericScalingStrike(5)),
                Modifier::Maneuver(Maneuver::MightyStrike(5)),
            ]),
        },
        RankAbility {
            name: "Maneuvers",
            is_magical: false,
            rank: 7,
            description: "",
            modifiers: Some(vec![
                Modifier::Maneuver(Maneuver::CertainStrike(7)),
                Modifier::Maneuver(Maneuver::GenericScalingStrike(7)),
                Modifier::Maneuver(Maneuver::MightyStrike(7)),
            ]),
        },
        RankAbility {
            name: "Trick Maneuvers",
            is_magical: false,
            rank: 1,
            description: r"
                You can confuse and confound your foes in combat.
                You gain access to one of the following \glossterm{combat styles}: \textit{dirty fighting}, \textit{ebb and flow}, or \textit{mobile assault}.
                In addition, you gain access to any combat style of your choice (see \pcref{Combat Styles}).
                You may spend \glossterm{insight points} to gain access to one additional combat style per insight point.
                You can only learn primal \glossterm{maneuvers} from primal combat styles that you have access to.

                You learn two rank 1 primal \glossterm{maneuvers}.
                You may spend \glossterm{insight points} to learn one additional maneuver per insight point.
                Unless otherwise noted in an ability's description, using a maneuver requires a \glossterm{standard action}.

                When you gain access to a new \glossterm{rank} in this archetype,
                    you can exchange any number of maneuvers you know for other maneuvers,
                    including maneuvers of the higher rank.

                \advancement You learn an additional primal maneuver at rank 4 and rank 7.
                The maximum rank of primal maneuvers that you can learn is equal to your rank in this archetype.
                Trick maneuvers also increase in power in unique ways based on your rank in this archetype, as indicated in their descriptions.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Trick Maneuvers+",
            is_magical: false,
            rank: 4,
            description: r"
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Trick Maneuvers+",
            is_magical: false,
            rank: 7,
            description: r"
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Tricky Force",
            is_magical: false,
            rank: 2,
            description: r"
                You gain a \plus1d bonus to your damage with all weapons.

                \advancement At rank 5, the bonus increases to \plus2d.
            ",
            modifiers: Some(vec![Modifier::StrikeDamageDice(1)]),
        },
        RankAbility {
            name: "Tricky Force+",
            is_magical: false,
            rank: 5,
            description: r"
            ",
            modifiers: Some(vec![Modifier::StrikeDamageDice(1)]),
        },
        RankAbility {
            name: "Tricky Finesse",
            is_magical: false,
            rank: 3,
            description: r"
                You gain a \plus1 bonus to Dexterity-based \glossterm{checks}, except \glossterm{initiative} checks.

                \advancement At rank 6, this bonus increases to \plus2.
            ",
            modifiers: Some(vec![
                Modifier::Skill(Skill::Balance, 1),
                Modifier::Skill(Skill::Flexibility, 1),
                Modifier::Skill(Skill::Ride, 1),
                Modifier::Skill(Skill::SleightOfHand, 1),
                Modifier::Skill(Skill::Stealth, 1),
            ]),
        },
        RankAbility {
            name: "Tricky Finesse+",
            is_magical: false,
            rank: 6,
            description: r"
            ",
            modifiers: Some(vec![
                Modifier::Skill(Skill::Balance, 1),
                Modifier::Skill(Skill::Flexibility, 1),
                Modifier::Skill(Skill::Ride, 1),
                Modifier::Skill(Skill::SleightOfHand, 1),
                Modifier::Skill(Skill::Stealth, 1),
            ]),
        },
    ];
}

pub fn jack_of_all_trades<'a>() -> Vec<RankAbility<'a>> {
    return vec![
        RankAbility {
            name: "Dabbler",
            is_magical: false,
            rank: 1,
            description: r"
                You gain two additional \glossterm{insight points}.
                In addition, you can spend insight points to gain one additional \glossterm{trained skill} per insight point.

                \advancement At rank 4, you gain an additional insight point.
            ",
            modifiers: Some(vec![Modifier::Resource(Resource::InsightPoint, 2)]),
        },
        RankAbility {
            name: "Dabbler+",
            is_magical: false,
            rank: 4,
            description: r"
                You gain an additional \glossterm{insight point}.
            ",
            modifiers: Some(vec![Modifier::Resource(Resource::InsightPoint, 1)]),
        },
        RankAbility {
            name: "Skill Exemplar",
            is_magical: false,
            rank: 2,
            description: r"
                You gain a \plus1 bonus to all skills.
                In addition, using the \ability{desperate exertion} ability to affect a skill check only increases your \glossterm{fatigue level} by one point.

                \advancement At rank 5, this bonus increases to \plus3.
                At rank 7, this bonus increases to \plus5.
                In addition, once per \glossterm{short rest} you can use the \ability{desperate exertion} ability to affect a skill check without increasing your fatigue level.
            ",
            modifiers: Some(vec![
                Modifier::Resource(Resource::TrainedSkill, 1),
                Modifier::Skill(Skill::Awareness, 1),
                Modifier::Skill(Skill::Balance, 1),
                Modifier::Skill(Skill::Climb, 1),
                Modifier::Skill(Skill::Craft, 1),
                Modifier::Skill(Skill::CreatureHandling, 1),
                Modifier::Skill(Skill::Deception, 1),
                Modifier::Skill(Skill::Deduction, 1),
                Modifier::Skill(Skill::Devices, 1),
                Modifier::Skill(Skill::Disguise, 1),
                Modifier::Skill(Skill::Endurance, 1),
                Modifier::Skill(Skill::Flexibility, 1),
                Modifier::Skill(Skill::Intimidate, 1),
                Modifier::Skill(Skill::Jump, 1),
                Modifier::Skill(
                    Skill::Knowledge(vec![
                        KnowledgeSubskill::Arcana,
                        KnowledgeSubskill::Dungeoneering,
                        KnowledgeSubskill::Engineering,
                        KnowledgeSubskill::Items,
                        KnowledgeSubskill::Local,
                        KnowledgeSubskill::Nature,
                        KnowledgeSubskill::Planes,
                        KnowledgeSubskill::Religion,
                    ]),
                    1,
                ),
                Modifier::Skill(Skill::Linguistics, 1),
                Modifier::Skill(Skill::Medicine, 1),
                Modifier::Skill(Skill::Perform, 1),
                Modifier::Skill(Skill::Persuasion, 1),
                Modifier::Skill(Skill::Profession, 1),
                Modifier::Skill(Skill::Ride, 1),
                Modifier::Skill(Skill::SleightOfHand, 1),
                Modifier::Skill(Skill::SocialInsight, 1),
                Modifier::Skill(Skill::Stealth, 1),
                Modifier::Skill(Skill::Survival, 1),
                Modifier::Skill(Skill::Swim, 1),
            ]),
        },
        RankAbility {
            name: "Skill Exemplar+",
            is_magical: false,
            rank: 5,
            description: r"
            ",
            modifiers: Some(vec![
                Modifier::Resource(Resource::TrainedSkill, 2),
                Modifier::Skill(Skill::Awareness, 2),
                Modifier::Skill(Skill::Balance, 2),
                Modifier::Skill(Skill::Climb, 2),
                Modifier::Skill(Skill::Craft, 2),
                Modifier::Skill(Skill::CreatureHandling, 2),
                Modifier::Skill(Skill::Deception, 2),
                Modifier::Skill(Skill::Deduction, 2),
                Modifier::Skill(Skill::Devices, 2),
                Modifier::Skill(Skill::Disguise, 2),
                Modifier::Skill(Skill::Endurance, 2),
                Modifier::Skill(Skill::Flexibility, 2),
                Modifier::Skill(Skill::Intimidate, 2),
                Modifier::Skill(Skill::Jump, 2),
                Modifier::Skill(
                    Skill::Knowledge(vec![
                        KnowledgeSubskill::Arcana,
                        KnowledgeSubskill::Dungeoneering,
                        KnowledgeSubskill::Engineering,
                        KnowledgeSubskill::Items,
                        KnowledgeSubskill::Local,
                        KnowledgeSubskill::Nature,
                        KnowledgeSubskill::Planes,
                        KnowledgeSubskill::Religion,
                    ]),
                    2,
                ),
                Modifier::Skill(Skill::Linguistics, 2),
                Modifier::Skill(Skill::Medicine, 2),
                Modifier::Skill(Skill::Perform, 2),
                Modifier::Skill(Skill::Persuasion, 2),
                Modifier::Skill(Skill::Profession, 2),
                Modifier::Skill(Skill::Ride, 2),
                Modifier::Skill(Skill::SleightOfHand, 2),
                Modifier::Skill(Skill::SocialInsight, 2),
                Modifier::Skill(Skill::Stealth, 2),
                Modifier::Skill(Skill::Survival, 2),
                Modifier::Skill(Skill::Swim, 2),
            ]),
        },
        RankAbility {
            name: "Skill Exemplar+",
            is_magical: false,
            rank: 7,
            description: r"
            ",
            modifiers: Some(vec![
                Modifier::Resource(Resource::TrainedSkill, 2),
                Modifier::Skill(Skill::Awareness, 2),
                Modifier::Skill(Skill::Balance, 2),
                Modifier::Skill(Skill::Climb, 2),
                Modifier::Skill(Skill::Craft, 2),
                Modifier::Skill(Skill::CreatureHandling, 2),
                Modifier::Skill(Skill::Deception, 2),
                Modifier::Skill(Skill::Deduction, 2),
                Modifier::Skill(Skill::Devices, 2),
                Modifier::Skill(Skill::Disguise, 2),
                Modifier::Skill(Skill::Endurance, 2),
                Modifier::Skill(Skill::Flexibility, 2),
                Modifier::Skill(Skill::Intimidate, 2),
                Modifier::Skill(Skill::Jump, 2),
                Modifier::Skill(
                    Skill::Knowledge(vec![
                        KnowledgeSubskill::Arcana,
                        KnowledgeSubskill::Dungeoneering,
                        KnowledgeSubskill::Engineering,
                        KnowledgeSubskill::Items,
                        KnowledgeSubskill::Local,
                        KnowledgeSubskill::Nature,
                        KnowledgeSubskill::Planes,
                        KnowledgeSubskill::Religion,
                    ]),
                    2,
                ),
                Modifier::Skill(Skill::Linguistics, 2),
                Modifier::Skill(Skill::Medicine, 2),
                Modifier::Skill(Skill::Perform, 2),
                Modifier::Skill(Skill::Persuasion, 2),
                Modifier::Skill(Skill::Profession, 2),
                Modifier::Skill(Skill::Ride, 2),
                Modifier::Skill(Skill::SleightOfHand, 2),
                Modifier::Skill(Skill::SocialInsight, 2),
                Modifier::Skill(Skill::Stealth, 2),
                Modifier::Skill(Skill::Survival, 2),
                Modifier::Skill(Skill::Swim, 2),
            ]),
        },
        RankAbility {
            name: "Versatile Power",
            is_magical: false,
            rank: 3,
            description: r"
                You gain a \plus2 bonus to your \glossterm{power} with all abilities.

                \advancement At rank 6, this bonus increases to \plus6.
            ",
            modifiers: Some(vec![Modifier::Power(2)]),
        },
        RankAbility {
            name: "Versatile Power+",
            is_magical: false,
            rank: 6,
            description: r"
            ",
            modifiers: Some(vec![Modifier::Power(4)]),
        },
    ];
}

pub fn suave_scoundrel<'a>() -> Vec<RankAbility<'a>> {
    return vec![
        RankAbility {
            name: "Confound",
            is_magical: false,
            rank: 1,
            description: r"
                You can use the \textit{confound} ability as a standard action.
                \begin{activeability}{Confound}
                    \abilitytag{Compulsion}
                    \rankline
                    Make an attack vs. Mental against a creature within \shortrange.
                    Your \glossterm{accuracy} is equal to your Deception skill.
                    \hit The target is \dazed as a \glossterm{condition}.

                    \rankline
                    \rank{3} You can target an additional creature within range.
                    \rank{5} The range increases to \rngmed.
                    \rank{7} The maximum number of targets increases to 5.
                \end{activeability}

                \advancement At ranks 3, 5, and 7, this ability improves as described above.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Trick Magic Device",
            is_magical: true,
            rank: 2,
            description: r"
                You can use wands as if you were able to cast arcane spells.
                Your maximum spell rank is equal to your rank in this archetype.
                In addition, you gain an additional \glossterm{attunement point}.
                You can only use this attunement point to \glossterm{attune} to magic wands or apparel items.

                \advancement At rank 5, you can use wands as if you were able to cast spells from all \glossterm{magic sources}, not just arcane spells.
                In addition, you gain a \plus2 bonus to \glossterm{accuracy} with abilities granted to you by magic items.
                This includes spells cast from wands, the special strike you can make with a \mitem{surestrike} weapon, and other similar abilities.
                However, it does not include ordinary strikes or maneuvers that simply use a magic weapon.
            ",
            modifiers: Some(vec![Modifier::Resource(Resource::AttunementPoint, 1)]),
        },
        RankAbility {
            name: "Trick Magic Device+",
            is_magical: true,
            rank: 5,
            description: r"
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Exploit Distraction",
            is_magical: false,
            rank: 3,
            description: r"
                You gain a \plus1 \glossterm{accuracy} bonus against creatures affected by any \glossterm{condition}.

                \advancement At rank 6, this bonus increases to \plus2.
            ",
            modifiers: Some(vec![Modifier::Accuracy(1)]),
        },
        RankAbility {
            name: "Exploit Distraction+",
            is_magical: false,
            rank: 6,
            description: r"
            ",
            modifiers: Some(vec![Modifier::Accuracy(1)]),
        },
        RankAbility {
            name: "Deceptive Force",
            is_magical: false,
            rank: 4,
            description: r"
                You gain a \plus1d bonus to your damage with all weapons.

                \advancement At rank 7, this bonus increases to \plus2d.
            ",
            modifiers: Some(vec![Modifier::StrikeDamageDice(1)]),
        },
        RankAbility {
            name: "Deceptive Force+",
            is_magical: false,
            rank: 7,
            description: r"
            ",
            modifiers: Some(vec![Modifier::StrikeDamageDice(1)]),
        },
        RankAbility {
            name: "What's That Over There",
            is_magical: false,
            rank: 4,
            description: r"
                You can use the \textit{what's that over there} ability as a standard action.
                \begin{activeability}{What's That Over There}
                    \abilitytag{Compulsion}
                    \rankline
                    Make a attack vs. Mental against a creature within \medrange.
                    Your \glossterm{accuracy} is equal to your Deception skill.
                    In addition, choose a location on stable ground within range.
                    \hit As a \glossterm{brief} effect, the target is compelled to move to the location you chose if it can do so safely, and it cannot take any actions except to move to the location and look around at it.
                    This effect automatically ends if the target takes any damage.
                    After this effect ends, the target becomes immune to it until it takes a \glossterm{short rest}.

                    \rankline
                    \rank{6} You can target an additional creature within range.
                \end{activeability}

                \advancement At rank 6, this ability improves as described above.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Nothing Is Real",
            is_magical: false,
            rank: 7,
            description: r"
                You can use the \textit{nothing is real} ability as a standard action.
                \begin{activeability}{Nothing Is Real}
                    \abilitytag{Compulsion}
                    \rankline
                    Make an attack vs. Mental against a creature within \shortrange.
                    Your \glossterm{accuracy} is equal to your Deception skill.
                    \hit The target is \glossterm{briefly} convinced that nothing is real.
                    It is unable to take any actions and is \unaware of all attacks against it.
                    After this effect ends, the target becomes immune to it until it takes a \glossterm{short rest}.
                \end{activeability}
            ",
            modifiers: None,
        },
    ];
}
