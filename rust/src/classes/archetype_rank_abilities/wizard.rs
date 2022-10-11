use crate::classes::archetype_rank_abilities::RankAbility;
use crate::core_mechanics::{Defense, Resource};
use crate::creatures::Modifier;

use super::standard_modifiers::add_standard_spell_modifiers;

pub fn alchemist<'a>() -> Vec<RankAbility<'a>> {
    return vec![
        RankAbility {
            name: "Alchemical Power",
            is_magical: true,
            rank: 1,
            description: r"
                You can add half your \glossterm{power} to the damage or healing caused by any alchemical items you use.
            ",
            // TODO: add alchemical items as unique attacks
            modifiers: None,
        },
        RankAbility {
            name: "Portable Workshop",
            is_magical: true,
            rank: 1,
            description: r"
                You carry materials necessary to refine low-grade alchemical items wherever you are.
                Where you lack material components, you fill in with some of your own magic, allowing you to create items more easily.
                The items are just as effective when used as items created normally.
                However, they are less durable, since they are partially sustained by your magic.
                Items created with this ability deteriorate and become useless after 24 hours or after you finish a long rest, whichever comes first.

                You can use this ability to create alchemical items with a rank up to your rank in this archetype (see \pcref{Item Ranks}).
                Creating an item in this way functions in the same way as crafting alchemical items normally, with the following changes.
                First, you do not require any raw materials.
                Second, you can create up to three items with this ability with 5 minutes of work.
                Third, you can only maintain the existence of three items with this ability at once.
                If you try to create a fourth item, you must stop maintaining the existence of another item created.
                You can do this as a \glossterm{free action} regardless of distance.
                This removes any lingering effects from the removed item, such as the protective qualities of an \textit{antitoxin elixir}.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Portable Workshop+",
            is_magical: true,
            rank: 5,
            description: r"
                The number of items you can simultaneously create and maintain with this ability increases to be equal to your rank in this archetype.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Alchemical Discovery",
            is_magical: true,
            rank: 2,
            description: r"
                You learn how to create alchemical items more effectively.
                You gain your choice of one of the following benefits.
                Each benefit can only be chosen once.
                {
                    \parhead{Aerodynamic Construction} You double the range of thrown alchemical items you create.
                        This does not affect alchemical items that are not designed to be thrown.
                    \parhead{Complex Construction} You can use your portable workshop ability to create items with a rank up to one higher than your rank in this archetype.
                    \parhead{Efficient Crafting} When you craft an alchemical item without using your \textit{portable workshop} ability, you treat it as if it was one rank lower than its actual rank for the purpose of determining its material requirements.
                    % TODO: wording
                    % \parhead{Enduring Construction} The duration of any alchemical item you create is doubled.
                        % In addition, alchemical items that last for a fixed number of uses have their number of uses doubled.
                    \parhead{Explosive Construction} The area affected by any alchemical item you create is doubled.
                    \parhead{Potent Construction} Whenever you create an alchemical item that deals damage or regains hit points, you double the item's flat modifier to damage or healing.
                    For example, a firebomb would deal 1d10+2 damage instead of 1d10+1 damage.
                    This modifier applies before you calculate any other damage modifiers, such as the power bonus from your \textit{alchemical infusion} ability.
                    \parhead{Repetitive Construction} Whenever you use your \textit{portable workshop} ability, you can create two copies of the same alchemical item.
                    This only counts as one item for the purpose of determining the number of items you can maintain with that ability.
                }
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Alchemical Discovery+",
            is_magical: true,
            rank: 4,
            description: r"
                You gain an additional \textit{alchemical discovery} ability.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Alchemical Discovery+",
            is_magical: true,
            rank: 6,
            description: r"
                You gain an additional \textit{alchemical discovery} ability.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Alchemical Infusion",
            is_magical: true,
            rank: 3,
            description: r"
                Whenever you use an alchemical item, you gain a \plus1d bonus to any damage or healing caused by the item for each rank by which your rank in this archetype exceeds the item's rank.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Alchemical Tolerance",
            is_magical: true,
            rank: 3,
            description: r"
                You gain a \plus1 bonus to your Fortitude defense.
                In addition, you are immune to poisons.
            ",
            modifiers: Some(vec![Modifier::Defense(Defense::Fortitude, 1)]),
        },
        RankAbility {
            name: "Alchemical Tolerance+",
            is_magical: true,
            rank: 7,
            description: r"
                The defense bonus increases to +2.
                In addition, you are immune to acid damage.
            ",
            modifiers: Some(vec![Modifier::Defense(Defense::Fortitude, 1)]),
        },
        RankAbility {
            name: "Experienced Quaffing",
            is_magical: false,
            rank: 5,
            description: r"
                You can drink up to two doses of potions, elixirs, and other drinkable alchemical items as part of the same standard action.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Experienced Quaffing+",
            is_magical: false,
            rank: 7,
            description: r"
                You can drink a single dose of a potion, elixir, or other drinkable alchemical item as a \glossterm{minor action}.
            ",
            modifiers: None,
        },
    ];
}

pub fn arcane_magic<'a>() -> Vec<RankAbility<'a>> {
    let mut abilities = vec![
        RankAbility {
            name: "Arcane Spells",
            is_magical: true,
            rank: 1,
            description: r"
                Your extensive studies grant you the ability to use arcane magic.
                You gain access to one arcane \glossterm{mystic sphere}, plus the \sphere{universal} mystic sphere (see \pcref{Arcane Mystic Spheres}).
                You may spend \glossterm{insight points} to gain access to one additional arcane \glossterm{mystic sphere} per two \glossterm{insight points}.
                You can only learn arcane spells from arcane mystic spheres that you have access to.

                You automatically learn all \glossterm{cantrips} from each mystic sphere you have access to.
                In addition, you learn two rank 1 arcane \glossterm{spells}.
                You can also spend \glossterm{insight points} to learn one additional rank 1 spell per insight point.

                Arcane spells require both \glossterm{verbal components} and \glossterm{somatic components} to cast (see \pcref{Casting Components}).
                Unless otherwise noted in a spell's description, casting any spell requires a \glossterm{standard action}.
                For details about mystic spheres and casting spells, see \pcref{Spell and Ritual Mechanics}.

                When you gain access to a new \glossterm{mystic sphere} or spell \glossterm{rank},
                    you can forget any number of spells you know to learn that many new spells in exchange,
                    including spells of the higher rank.

                \advancement The maximum rank of arcane spells that you can learn is equal to your rank in this archetype.
                Arcane spells also increase in power in unique ways based on your rank in this archetype, as indicated in their descriptions.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Arcane Spells+",
            is_magical: true,
            rank: 2,
            description: r"
                You learn an additional arcane spell.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Arcane Spells+",
            is_magical: true,
            rank: 4,
            description: r"
                You learn an additional arcane spell.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Arcane Spells+",
            is_magical: true,
            rank: 7,
            description: r"
                You learn an additional arcane spell.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Mage Armor",
            is_magical: true,
            rank: 1,
            description: r"
                You can use the \textit{mage armor} ability as a standard action.
                \begin{activeability}{Mage Armor}
                    \rankline
                    You create a translucent suit of magical armor on your body and over your hands.
                    This functions like body armor that provides a \plus2 bonus to your Armor defense and has no \glossterm{encumbrance}.
                    It also provides a bonus to \glossterm{damage resistance} equal to twice your rank in this archetype.

                    You can also use a \glossterm{free hand} to wield the barrier as a shield.
                    This functions like a buckler, granting you a \plus1 bonus to your Armor defense, except that you do not need to be proficient with light armor.
                    Since this bonus comes from a shield, it does not stack with the benefits of using any other shield.

                    This ability lasts until you use it again or until you \glossterm{dismiss} it as a free action.
                    In addition, it is automatically dismissed if you wear other body armor of any kind.
                \end{activeability}
            ",
            // Assuming no other armor
            modifiers: Some(vec![
                Modifier::Defense(Defense::Armor, 3),
                Modifier::DamageResistance(2),
            ]),
        },
        RankAbility {
            name: "Mage Armor+",
            is_magical: true,
            rank: 3,
            description: r"        
                The damage resistance bonus increases to three times your rank in this archetype.
            ",
            // Rank 2: 4. Rank 3: 9.
            modifiers: None,
        },
        RankAbility {
            name: "Mage Armor+",
            is_magical: true,
            rank: 6,
            description: r"
                The damage resistance bonus increases to four times your rank in this archetype.
                In addition, the defense bonus from the body armor increases to \plus3.
            ",
            modifiers: Some(vec![Modifier::Defense(Defense::Armor, 1)]),
        },
        RankAbility {
            name: "Mage Armor",
            is_magical: true,
            rank: 2,
            description: "",
            modifiers: Some(vec![Modifier::DamageResistance(4)]),
        },
        RankAbility {
            name: "Mage Armor",
            is_magical: true,
            rank: 3,
            description: "",
            modifiers: Some(vec![Modifier::DamageResistance(9)]),
        },
        RankAbility {
            name: "Mage Armor",
            is_magical: true,
            rank: 4,
            description: "",
            modifiers: Some(vec![Modifier::DamageResistance(12)]),
        },
        RankAbility {
            name: "Mage Armor",
            is_magical: true,
            rank: 5,
            description: "",
            modifiers: Some(vec![Modifier::DamageResistance(15)]),
        },
        RankAbility {
            name: "Mage Armor",
            is_magical: true,
            rank: 6,
            description: "",
            modifiers: Some(vec![Modifier::DamageResistance(24)]),
        },
        RankAbility {
            name: "Mage Armor",
            is_magical: true,
            rank: 7,
            description: "",
            modifiers: Some(vec![Modifier::DamageResistance(28)]),
        },
    ];
    add_standard_spell_modifiers(&mut abilities);
    return abilities;
}

pub fn arcane_scholar<'a>() -> Vec<RankAbility<'a>> {
    return vec![
        RankAbility {
            name: "Ritualist",
            is_magical: true,
            rank: 1,
            description: r"
                You gain the ability to perform arcane rituals to create unique magical effects (see \pcref{Rituals}).
                The maximum \glossterm{rank} of arcane ritual you can learn or perform is equal to the maximum rank of arcane spell that you can cast.
                In addition, you automatically learn one free arcane ritual of each rank you have access to, including new ranks as you gain access to them.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Spell Knowledge",
            is_magical: true,
            rank: 1,
            description: r"
                You learn an additional spell from any arcane \glossterm{mystic sphere} that you have access to.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Scholastic Insight",
            is_magical: true,
            rank: 2,
            description: r"
                You gain one of the following insights.
                Some insights can be chosen multiple times, as indicated in their descriptions.

                {
                    \parhead{Esoteric Spell Knowledge} You learn a single spell from any arcane \glossterm{mystic sphere}.
                    You do not not need to have access to that mystic sphere.
                    This does not grant you access to that mystic sphere for any other purposes.
                    Whenever you gain access to a new mystic sphere or spell rank, you may choose a different spell with this ability.
                    \par You can choose this insight multiple times, learning an additional spell each time.

                    \parhead{Expanded Sphere Access} You gain access to a new \glossterm{mystic sphere}.
                    \par You cannot choose this insight multiple times.

                    \parhead{Memorized Sphere} % TODO: clarify you need to be high enough rank?
                    Choose a \glossterm{mystic sphere} you have access to.
                    You can perform rituals from that \glossterm{mystic sphere} without having them written in your ritual book.
                    \par You can choose this insight multiple times, choosing a different \glossterm{mystic sphere} each time.

                    \parhead{Sphere Specialization}\label{Sphere Specialization} Choose a a \glossterm{mystic sphere} you have access to.
                    You gain a \glossterm{power} bonus equal to your rank in this archetype and a \plus1 \glossterm{accuracy} bonus with abilities from that \glossterm{mystic sphere}.
                    In exchange, you must lose access to another \glossterm{mystic sphere} you have.
                    You must exchange all spells you know from that \glossterm{mystic sphere} with spells from other \glossterm{mystic spheres} you have access to.
                    \par You cannot choose this insight multiple times.
                }
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Scholastic Insight+",
            is_magical: true,
            rank: 4,
            description: r"
                You gain an additional scholastic insight.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Scholastic Insight+",
            is_magical: true,
            rank: 6,
            description: r"
                You gain an additional scholastic insight.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Contingency",
            is_magical: true,
            rank: 3,
            description: r"
                You gain the ability to prepare a spell so it takes effect automatically if specific circumstances arise.
                % If any spells take more than one standard action, they would need to be excluded from Contingency, but none exist
                % You can apply this ability to any arcane spell that can be cast as a \glossterm{standard action} or \glossterm{minor action}.
                Preparing a spell with this ability takes 5 minutes.
                When the preparation is complete, the spell has no immediate effect.
                Instead, it automatically takes effect when some specific circumstances arise.
                During the time required to cast the spell, you specify what circumstances cause the spell to take effect.

                The spell can be set to trigger in response to any circumstances that a typical human observing you and your situation could detect.
                For example, you could specify ``when I fall at least 50 feet'' or ``when I take a \glossterm{vital wound}'', but not ``when there is an invisible creature within 50 feet of me'' or ``when I have only one \glossterm{hit point} remaining.''
                The more specific the required circumstances, the better -- vague requirements, such as ``when I am in danger'', may cause the spell to trigger unexpectedly or fail to trigger at all.
                If you attempt to specify multiple separate triggering conditions, such as ``when I take damage or when an enemy is adjacent to me'', the spell will randomly ignore all but one of the conditions.

                If the spell needs to be targeted, the trigger condition can specify a simple rule for identifying how to target the spell, such as ``the closest enemy''.
                If the rule is poorly worded or imprecise, the spell may target incorrectly or fail to activate at all.
                Any spells which require decisions, such as the \spell{dimension door} spell, must have those decisions made at the time it is cast.
                You cannot alter those decisions when the contingency takes effect.

                You can have only one spell with this ability active at a time.
                If you use this ability again with a different spell, the old contingency is removed.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Contingency+",
            is_magical: true,
            rank: 7,
            description: r"
                You may have two separate contingencies active at the same time.
                Each contingency may have separate triggering conditions.
                Only one contigency can trigger each round.
                If multiple contingencies would activate simultaneously, choose one to activate randomly.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Ritual Leader",
            is_magical: true,
            rank: 5,
            description: r"
                Whenever you lead a ritual, it requires half the normal number of \glossterm{fatigue levels} and half the normal time to complete, to a minimum of zero fatigue levels.
            ",
            modifiers: None,
        },
    ];
}

pub fn arcane_spell_mastery<'a>() -> Vec<RankAbility<'a>> {
    return vec![
        RankAbility {
            name: "Metamagic",
            is_magical: true,
            rank: 1,
            description: r"
                You learn how to further refine your spellcasting abilities.
                Choose two metamagic abilities from the list below.
                You can also spend \glossterm{insight points} to learn one additional metamagic ability per insight point.
                You cannot choose the same spell with more than two metamagic abilities.
                {
                    \parhead{Distant Spell} Choose an arcane \glossterm{spell} you know with a standard \glossterm{range}: Short, Medium, Long, Distant, or Extreme.
                        You increase that spell's range to the next standard range category, to a maximum of Extreme range.
                        You can choose this ability multiple times, choosing a different spell each time.
                        Whenever you learn a new spell, you may change which of your spells this ability affects.
                    \parhead{Mystic Sphere} You gain access to an additional arcane \glossterm{mystic sphere}, including all \glossterm{cantrips} from that sphere.
                        You cannot choose this ability multiple times.
                    \parhead{Precise Spell} Choose an arcane \glossterm{spell} you know.
                        You gain a \plus1 bonus to \glossterm{accuracy} with that spell.
                        You can choose this ability multiple times, choosing a different spell each time.
                        Whenever you learn a new spell, you may change which of your spells this ability affects.
                    \parhead{Rituals} You gain the ability to perform arcane rituals to create unique magical effects (see \pcref{Rituals}).
                        The maximum \glossterm{rank} of arcane ritual you can learn or perform is equal to the maximum \glossterm{rank} of arcane spell that you can cast.
                        In addition, you automatically learn one free arcane ritual of each rank you have access to, including new ranks as you gain access to them.
                        You cannot choose this ability multiple times.
                    \parhead{Widened Spell} Choose an arcane \glossterm{spell} you know with a standard \glossterm{area}: Small, Medium, Large, Huge, or Gargantuan.
                        You increase that spell's area to the next standard area category, to a maximum of a Gargantuan area.
                        You can choose this ability multiple times, choosing a different spell each time.
                        Whenever you learn a new spell, you may change which of your spells this ability affects.
                }
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Metamagic+",
            is_magical: true,
            rank: 4,
            description: r"
                You gain an additional metamagic ability.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Metamagic+",
            is_magical: true,
            rank: 7,
            description: r"
                You gain two additional metamagic abilities.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Intricate Spell",
            is_magical: true,
            rank: 2,
            description: r"
                Whenever you cast a spell, you can use this ability to make the spell's incantations more nuanced and complex.
                If you do, you gain a \plus1 accuracy bonus with the spell.
                However, you take a \minus2 penalty to your Armor and Reflex defenses this round.
                This defense penalty is \abilitytag{Swift}.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "Wellspring of Power",
            is_magical: true,
            rank: 3,
            description: r"
                You gain a \plus2 bonus to your \glossterm{power}.
            ",
            modifiers: Some(vec![Modifier::Power(2)]),
        },
        RankAbility {
            name: "Wellspring of Power+",
            is_magical: true,
            rank: 6,
            description: r"
                The power bonus increases to +6.
            ",
            modifiers: Some(vec![Modifier::Power(4)]),
        },
        RankAbility {
            name: "Attunement Point",
            is_magical: true,
            rank: 5,
            description: r"
                You gain an additional \glossterm{attunement point}.
            ",
            modifiers: Some(vec![Modifier::Resource(Resource::AttunementPoint, 1)]),
        },
    ];
}

pub fn school_specialist<'a>() -> Vec<RankAbility<'a>> {
    return vec![
        RankAbility {
            name: "School Specialization",
            is_magical: true,
            rank: 1,
            description: r"
                The arcane mystic spheres can be divided into six traditional schools of magic.
                Choose one of the following schools of magic.
                You are a specialist in your chosen school.
                You cannot gain access to any arcane mystic spheres outside of your specialized school, and you cannot learn spells or rituals from those spheres by any means.
                In exchange, you gain an additional \glossterm{insight point}, and you gain a benefit based on your specialized school.

                \subcf{Abjuration} The \sphere{telekinesis} and \sphere{thaumaturgy} mystic spheres.
                    If you specialize in this school, you gain a bonus equal to three times your rank in this archetype to your \glossterm{damage resistance}.
                    In addition, you gain a \plus1 bonus to your Armor defense.

                \subcf{Conjuration} The \sphere{astromancy}, \sphere{fabrication}, and \sphere{summoning} mystic spheres.
                    If you specialize in this school, you gain a \plus30 foot bonus to the \glossterm{range} of arcane spells you cast.

                \subcf{Evocation} The \sphere{cryomancy}, \sphere{electromancy}, and \sphere{pyromancy} mystic spheres.
                    If you specialize in this school, you gain a \plus2 bonus to your \glossterm{power}.

                \subcf{Illusion} The \sphere{enchantment}, \sphere{photomancy}, and \sphere{umbramancy} mystic spheres.
                    If you specialize in this school, you gain a \plus1 bonus to your \glossterm{accuracy}.

                \subcf{Transmutation} The \sphere{chronomancy}, \sphere{polymorph}, and \sphere{terramancy} mystic spheres.
                    If you specialize in this school, you gain a \plus2 bonus to your Fortitude, Reflex, or Mental defense.
                    You can change which defense this bonus applies to as a \glossterm{minor action}.

                \subcf{Necromancy} The \sphere{revelation} and \sphere{vivimancy} mystic spheres.
                    If you specialize in this school, you gain a bonus equal to three times your rank in this archetype to your maximum \glossterm{hit points}.
                    In addition, you gain a \plus1 bonus to your Fortitude defense.
            ",
            // Assume evocation
            modifiers: Some(vec![
                Modifier::Power(2),
                Modifier::Resource(Resource::InsightPoint, 1),
            ]),
        },
        RankAbility {
            name: "School Specialization+",
            is_magical: true,
            rank: 4,
            description: r"
                Your understanding of your chosen school improves.
                {
                    \subcf{Abjuration} The bonus to damage resistance increases to four times your rank in this archetype.

                    \subcf{Conjuration} The range improvement increases to \plus60 feet.

                    \subcf{Evocation} The power bonus increases to \plus5.

                    \subcf{Illusion} You gain \trait{low-light vision}, allowing you to see in \glossterm{shadowy illumination} (see \pcref{Low-light Vision}).
                    In addition, you gain \trait{darkvision} with a 60 foot range, allowing you to see in complete darkness (see \pcref{Darkvision}).
                    If you already have that ability, you increase its range by 60 feet.

                    \subcf{Transmutation} The defense bonus increases to \plus3.

                    \subcf{Necromancy} The hit point bonus increases to four times your rank in this archetype.
                    In addition, the Fortitude bonus increases to \plus2.
                }
            ",
            modifiers: Some(vec![Modifier::Power(3)]),
        },
        RankAbility {
            name: "School Specialization+",
            is_magical: true,
            rank: 7,
            description: r"
                Your understanding of your chosen school reaches its full potential.
                {
                    \subcf{Abjuration} The bonus to damage resistance increases to five times your rank in this archetype.
                    In addition, the Armor bonus increases to \plus2.

                    \subcf{Conjuration} The range improvement increases to \plus120 feet.

                    \subcf{Evocation} The power bonus increases to \plus12.

                    \subcf{Illusion} The accuracy bonus increases to \plus2.

                    \subcf{Transmutation} The defense bonus increases to \plus4.
                    In addition, you can change which defense the bonus applies to as a \glossterm{free action}.

                    \subcf{Necromancy} The hit point bonus increases to five times your rank in this archetype.
                    In addition, the Fortitude bonus increases to \plus3.
                }
            ",
            modifiers: Some(vec![Modifier::Power(10)]),
        },
        RankAbility {
            name: "School Knowledge",
            is_magical: true,
            rank: 2,
            description: r"
                You learn an additional arcane spell from your chosen school.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "School Knowledge+",
            is_magical: true,
            rank: 5,
            description: r"
                You learn an additional arcane spell from your chosen school.
            ",
            modifiers: None,
        },
        RankAbility {
            name: "School Attunement",
            is_magical: true,
            rank: 3,
            description: r"
                You gain an additional \glossterm{attunement point}.
                You can only use this attunement point to \glossterm{attune} to a spell from your chosen school.
            ",
            modifiers: Some(vec![Modifier::Resource(Resource::AttunementPoint, 1)]),
        },
        RankAbility {
            name: "School Resilience",
            is_magical: true,
            rank: 6,
            description: r"
                You gain a defensive ability based on your chosen school.
                {
                    \subcf{Abjuration} You are immune to \glossterm{push} and \glossterm{knockback} effects.
                    In addition, your spells cannot be \glossterm{suppressed} or \glossterm{dismissed} by \glossterm{magical} effects other than your own.

                    \subcf{Conjuration} You passively flicker into the Astral Plane, causing all \glossterm{targeted} attacks against you to have a 10\% \glossterm{failure chance}.

                    \subcf{Evocation} You are \trait{impervious} to cold damage, electricity damage, and fire damage.

                    \subcf{Illusion} You are immune to being \dazzled and \blinded.

                    \subcf{Transmutation} You are immune to being \slowed and \immobilized.

                    \subcf{Necromancy} You are \trait{impervious} to attacks from creatures with less than half of their maximum hit points remaining and undead creatures.
                }
            ",
            // TODO: represent this somehow?
            modifiers: None,
        },
    ];
}
