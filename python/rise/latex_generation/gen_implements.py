#!/usr/bin/env python3

import click
from rise.latex_generation.book_path import book_path
from rise.latex.magic_item import MagicItem
from rise.latex.util import latexify, longtablify


def generate_implements():
    implements = []

    implements += [
        MagicItem(
            name="Spell Wand, 1st",
            rank=1,
            material_type="Wand",
            tags=[],
            description="""
                This wand grants you knowledge of a single rank 1 spell that does not have the \\abilitytag<Attune> or \\abilitytag<Sustain> tags.
                Each wand is associated with a specific spell, and a single \\glossterm<magic source> that can grant access to that spell.
                You must have the ability to cast spells of the given rank from the same \\glossterm<magic source> as the wand.
                However, you do not need to have access to the \\glossterm<mystic sphere> that the spell belongs to.
                Spells you know because of a spell wand gain any rank upgrades appropriate to your rank with that form of spellcasting.

                If you stop wielding this wand, deattune from it, or otherwise lose access to its magical effects, the effects of any active spells that you know because of the wand also end, regardless of their normal duration.
            """,
            short_description="Grants knowledge of a rank 1 spell",
        ),
        MagicItem(
            name="Spell Wand, 2nd",
            rank=2,
            material_type="Wand",
            tags=[],
            description="""
                This item functions like a \\mitem<spell wand>, except that it grants knowledge of a single rank 2 spell.
            """,
            short_description="Grants knowledge of a rank 2 spell",
        ),
        MagicItem(
            name="Spell Wand, 3rd",
            rank=3,
            material_type="Wand",
            tags=[],
            description="""
                This item functions like a \\mitem<spell wand>, except that it grants knowledge of a single rank 3 spell.
            """,
            short_description="Grants knowledge of a rank 3 spell",
        ),
        MagicItem(
            name="Spell Wand, 4th",
            rank=4,
            material_type="Wand",
            tags=[],
            description="""
                This item functions like a \\mitem<spell wand>, except that it grants knowledge of a single rank 4 spell.
            """,
            short_description="Grants knowledge of a rank 4 spell",
        ),
        MagicItem(
            name="Spell Wand, 5th",
            rank=5,
            material_type="Wand",
            tags=[],
            description="""
                This item functions like a \\mitem<spell wand>, except that it grants knowledge of a single rank 5 spell.
            """,
            short_description="Grants knowledge of a rank 5 spell",
        ),
        MagicItem(
            name="Spell Wand, 6th",
            rank=6,
            material_type="Wand",
            tags=[],
            description="""
                This item functions like a \\mitem<spell wand>, except that it grants knowledge of a single rank 6 spell.
            """,
            short_description="Grants knowledge of a rank 6 spell",
        ),
        MagicItem(
            name="Spell Wand, 7th",
            rank=7,
            material_type="Wand",
            tags=[],
            description="""
                This item functions like a \\mitem<spell wand>, except that it grants knowledge of a single rank 7 spell.
            """,
            short_description="Grants knowledge of a rank 7 spell",
        ),
    ]

    implements += [
        MagicItem(
            name="Staff of Shared Healing",
            rank=3,
            material_type="Staff",
            tags=[],
            description="""
                Once per round, when you cause a creature other yourself to regain \\glossterm<hit points> using a \\glossterm<magical> ability, you can activate this item as a \\glossterm<free action>.
                When you do, you also regain half that many hit points.
            """,
            short_description="Heals you when you heal others",
        ),
        MagicItem(
            name="Staff of Shared Healing, Greater",
            rank=6,
            material_type="Staff",
            tags=[],
            description="""
                Once per round, when you cause a creature other yourself to regain \\glossterm<hit points> using a \\glossterm<magical> ability, you can activate this item as a \\glossterm<free action>.
                When you do, you also regain that many hit points.
            """,
            short_description="Significantly heals you when you heal others",
        ),
    ]

    implements += [
        MagicItem(
            name="Staff of Transit",
            rank=2,
            material_type="Staff",
            tags=[],
            description="""
                Your \\glossterm<magical> abilities have the maximum distance they can \\glossterm<teleport> targets doubled.
            """,
            short_description="Doubles your teleportation distance",
        ),
        MagicItem(
            name="Staff of Transit, Greater",
            rank=4,
            material_type="Staff",
            tags=[],
            description="""
                Your \\glossterm<magical> abilities have the maximum distance they can \\glossterm<teleport> targets tripled.
            """,
            short_description="Triples your teleportation distance",
        ),
        MagicItem(
            name="Staff of Transit, Supreme",
            rank=6,
            material_type="Staff",
            tags=[],
            description="""
                Your \\glossterm<magical> abilities have the maximum distance they can \\glossterm<teleport> targets quadrupled.
            """,
            short_description="Quadruples your teleportation distance",
        ),
    ]

    implements += [
        MagicItem(
            name="Fearsome Staff",
            rank=3,
            material_type="Staff",
            tags=["Emotion"],
            description="""
                Whenever a creature that is suffering penalties for being \\shaken, \\frightened, or \\panicked by you gets farther than 60 feet from you, it \\glossterm<briefly> continues suffering penalties from those effects regardless of distance.
                It must still have \\glossterm<line of sight> and \\glossterm<line of effect> to you.
            """,
            short_description="Briefly increases range of your fear effects",
        ),
        MagicItem(
            name="Fearsome Staff, Greater",
            rank=6,
            material_type="Staff",
            tags=["Emotion"],
            description="""
                Creatures that are shaken, frightened or panicked by you suffer penalties from those effects at any distance.
                They must still have \\glossterm<line of sight> and \\glossterm<line of effect> to you.
            """,
            short_description="Increases range of your fear effects",
        ),
    ]

    implements += [
        MagicItem(
            name="Staff of Discordance",
            rank=3,
            material_type="Staff",
            tags=[],
            description="""
                Whenever you cause an enemy to be \\dazed, you can also make it \\glossterm<briefly> \\stunned.
            """,
            short_description="Makes dazed creatures briefly stunned",
        ),
        MagicItem(
            name="Staff of Discordance, Greater",
            rank=6,
            material_type="Staff",
            tags=[],
            description="""
                Whenever you cause an enemy to be \\stunned, you can also make it \\glossterm<briefly> \\confused.
            """,
            short_description="Makes stunned creatures briefly confused",
        ),
    ]

    implements += [
        MagicItem(
            name="Staff of Hindrance",
            rank=3,
            material_type="Staff",
            tags=[],
            description="""
                Whenever you cause an enemy to be \\slowed, you can make that condition \\glossterm<briefly> impossible to remove.
            """,
            short_description="Your slowing effects last longer",
        ),
        MagicItem(
            name="Staff of Hindrance, Greater",
            rank=5,
            material_type="Staff",
            tags=[],
            description="""
                Whenever you cause an enemy to be \\slowed, that condition must be removed an additional time before the effect ends.
            """,
            short_description="Your slowing effects last much longer",
        ),
    ]

    implements += [
        MagicItem(
            name="Extending Staff",
            rank=2,
            material_type="Staff",
            description="""
                You gain a +15 foot bonus to the \\glossterm<range> of all of your ranged \\glossterm<magical> abilities.
                This does not affect abilities that do not have a range listed in feet.
            """,
            short_description="Grants +15 foot range",
        ),
        MagicItem(
            name="Extending Staff, Greater",
            rank=4,
            material_type="Staff",
            description="""
                You gain a +30 foot bonus to the \\glossterm<range> of all of your ranged \\glossterm<magical> abilities.
                This does not affect abilities that do not have a range listed in feet.
            """,
            short_description="Grants +30 foot range",
        ),
        MagicItem(
            name="Extending Staff, Supreme",
            rank=6,
            material_type="Staff",
            description="""
                You gain a +60 foot bonus to the \\glossterm<range> of all of your ranged \\glossterm<magical> abilities.
                This does not affect abilities that do not have a range listed in feet.
            """,
            short_description="Grants +60 foot range",
        ),
    ]

    implements += [
        MagicItem(
            name="Staff of Giants",
            rank=3,
            material_type="Staff",
            description="""
                Whenever you use a \\glossterm<magical> ability that has a maximum size category for its targets or any objects it creates, you increase that maximum by one size category, to a maximum of Colossal.
                This does not affect abilities that create creatures of a particular size.
            """,
            short_description="Increases maximum size category of abilities",
        ),
        MagicItem(
            name="Staff of Giants, Greater",
            rank=6,
            material_type="Staff",
            description="""
                This implement functions like a \\mitem<staff of giants> implement, except that the maximum size category increases by two size categories.
            """,
            short_description="Significantly increaases maximum size category of abilities",
        ),
    ]

    implements += [
        MagicItem(
            name="Selective Staff",
            rank=3,
            material_type="Staff",
            tags=[],
            description="""
                Whenever you use a \\glossterm<magical> ability that affects an area and does not have the \\abilitytag<Sustain> or \\abilitytag<Attune> tags, you can freely exclude any areas from the ability's effect.
                All squares in the final area of the spell must be contiguous.
                You cannot create split a spell's area into multiple completely separate areas.
            """,
            short_description="Allows excluding areas",
        ),
        MagicItem(
            name="Selective Staff, Greater",
            rank=5,
            material_type="Staff",
            tags=[],
            description="""
                This implement functions like a \\mitem<selective staff> implement, except that you can split the spell's area into two completely separate areas.
                If you do, each of those two areas must be independently contiguous.
            """,
            short_description="Allows excluding and splitting areas",
        ),
    ]

    implements += [
        MagicItem(
            name="Staff of Silence",
            rank=2,
            material_type="Staff",
            tags=['Swift'],
            description="""
                As a \\glossterm<free action>, you can activate this staff.
                When you do, you increase your \\glossterm<fatigue level> by one and \\glossterm<briefly> gain the ability to cast spells without using \\glossterm<verbal components>.
                This ability has the \\glossterm<Swift> tag, so it allows you to cast a spell without verbal components in the same phase that you activate this staff.
            """,
            short_description="Can exert to cast spells without verbal components",
        ),
        MagicItem(
            name="Staff of Stillness",
            rank=2,
            material_type="Staff",
            tags=['Swift'],
            description="""
                As a \\glossterm<free action>, you can activate this staff.
                When you do, you increase your \\glossterm<fatigue level> by one and \\glossterm<briefly> gain the ability to cast spells without using \\glossterm<somatic components>.
                This ability has the \\glossterm<Swift> tag, so it allows you to cast a spell without somatic components in the same phase that you activate this staff.
            """,
            short_description="Can exert to cast spells without somatic components",
        ),
        MagicItem(
            name="Staff of Silence, Greater",
            rank=4,
            material_type="Staff",
            tags=[],
            description="""
                You can cast spells without using \\glossterm<verbal components>.
            """,
            short_description="Can cast spells without verbal components",
        ),
        MagicItem(
            name="Staff of Stillness, Greater",
            rank=4,
            material_type="Staff",
            tags=[],
            description="""
                You can cast spells without using \\glossterm<somatic components>.
            """,
            short_description="Can cast spells without somatic components",
        ),
        MagicItem(
            name="Staff of Tranquility",
            rank=6,
            material_type="Staff",
            tags=[],
            description="""
                You can cast spells without using \\glossterm<verbal components> or \\glossterm<somatic components>.
            """,
            short_description="Can cast spells without components",
        ),
    ]

    implements += [
        MagicItem(
            name="Reaching Staff",
            rank=3,
            material_type="Staff",
            description="""
                Whenever you use a \\glossterm<magical> ability that does not have the \\abilitytag<Sustain> or \\abilitytag<Attune> tags, you may activate this staff.
                When you do, choose a location within \\rngshort range.
                The ability takes effect as if you were in the chosen location.
                In addition, you increase your \\glossterm<fatigue level> by one.
                This affects your \\glossterm<line of effect> for the ability, but not your \\glossterm<line of sight> (since you still see from your normal location).
                % Wording?
                Since an ability's range is measured from your location, this item can allow you to affect targets outside your normal range.
                For example, a cone that normally bursts out from you would instead originate from your chosen location, potentially avoiding an obstacle between you and your target.
            """,
            short_description="Can exert to use abilities from a short distance away",
        ),
        MagicItem(
            name="Reaching Staff, Greater",
            rank=5,
            material_type="Staff",
            description="""
                This implement functions like a \\textit<reaching staff> implement, except that activating it does not increase your \\glossterm<fatigue level>.
            """,
            short_description="Can use abilities from a short distance away",
        ),
    ]

    implements += [
        MagicItem(
            name="Widening Staff",
            rank=4,
            material_type="Staff",
            tags=[],
            description="""
                Whenever you use a \\glossterm<magical> ability that affects an area and does not have the \\abilitytag<Attune> or \\abilitytag<Sustain> tags, you may activate this staff.
                When you do, you double the spell's area.
                In addition, you increase your \\glossterm<fatigue level> by one.
            """,
            short_description="Can exert to double area size",
        ),
        MagicItem(
            name="Widening Staff, Greater",
            rank=6,
            material_type="Staff",
            tags=[],
            description="""
                This implement functions like a \\textit<widening staff> implement, except that activating it does not increase your \\glossterm<fatigue level>.
            """,
            short_description="Can double area size",
        ),
    ]
    implements += [
        MagicItem(
            name="Staff of Potency",
            rank=2,
            material_type="Staff",
            tags=[],
            description="""
                You gain a +2 \\glossterm<magic bonus> to your \\glossterm<power>.
            """,
            short_description="Grants +2 power",
        ),
        MagicItem(
            name="Staff of Potency, Greater",
            rank=4,
            material_type="Staff",
            tags=[],
            description="""
                You gain a +4 \\glossterm<magic bonus> to your \\glossterm<power>.
            """,
            short_description="Grants +4 power",
        ),
        MagicItem(
            name="Staff of Potency, Supreme",
            rank=6,
            material_type="Staff",
            tags=[],
            description="""
                You gain a +8 \\glossterm<magic bonus> to your \\glossterm<power>.
            """,
            short_description="Grants +8 power",
        ),
    ]

    implements += [
        MagicItem(
            name="Blessed Staff",
            rank=3,
            material_type="Staff",
            tags=[],
            description="""
                When you use the \\textit<desperate exertion> ability to affect a \\glossterm<magical> ability, you gain a +2 bonus to the reroll.
                This stacks with the normal +2 bonus from the \\textit<desperate exertion> ability.
            """,
            short_description="Grants +2 bonus with \\textit<desperate exertion>",
        ),
        MagicItem(
            name="Blessed Staff, Greater",
            rank=5,
            material_type="Staff",
            tags=[],
            description="""
                When you use the \\textit<desperate exertion> ability to affect a \\glossterm<magical> ability, you gain a +3 bonus to the reroll.
                This stacks with the normal +2 bonus from the \\textit<desperate exertion> ability.
            """,
            short_description="Grants +3 bonus with \\textit<desperate exertion>",
        ),
        MagicItem(
            name="Blessed Staff, Supreme",
            rank=7,
            material_type="Staff",
            tags=[],
            description="""
                When you use the \\textit<desperate exertion> ability to affect a \\glossterm<magical> ability, you gain a +4 bonus to the reroll.
                This stacks with the normal +2 bonus from the \\textit<desperate exertion> ability.
            """,
            short_description="Grants +4 bonus with \\textit<desperate exertion>",
        ),
    ]

    implements += [
        MagicItem(
            name="Hexbite Staff",
            rank=2,
            material_type="Staff",
            tags=[],
            description="""
                Whenever a creature removes a \\glossterm<condition> that you inflicted on it, it takes 1d10+4 \\glossterm<energy damage>.
            """,
            short_description="Deals 1d10+4 damage when foes remove conditions",
        ),
        MagicItem(
            name="Hexbite Staff, Greater",
            rank=4,
            material_type="Staff",
            tags=[],
            description="""
                Whenever a creature removes a \\glossterm<condition> that you inflicted on it, it takes 2d10+8 \\glossterm<energy damage>.
            """,
            short_description="Deals 2d10+8 damage when foes remove conditions",
        ),
        MagicItem(
            name="Hexbite Staff, Supreme",
            rank=6,
            material_type="Staff",
            tags=[],
            description="""
                Whenever a creature removes a \\glossterm<condition> that you inflicted on it, it takes 4d10+12 \\glossterm<energy damage>.
            """,
            short_description="Deals 4d10+12 damage when foes remove conditions",
        ),
    ]

    implements += [
        MagicItem(
            name="Staff of the Martyr",
            rank=4,
            material_type="Staff",
            tags=[],
            description="""
                You can use \\abilitytag<Healing> abilities even if you used a Healing ability during the previous round.
                However, \\abilitytag<Healing> abilities cannot cause you to regain any hit points.
            """,
            short_description="Heal others more frequently, but not yourself",
        ),
    ]

    implements += [
        MagicItem(
            name="Staff of Stored Attunement",
            rank=2,
            material_type="Staff",
            tags=[],
            description="""
                When you cast a \\glossterm<targeted> spell that has the \\glossterm<Attune> tag, you can invest the magic of the spell in this staff.
                If you do, the spell does not have its normal effect.
                Up to two spells can be stored this way.
                If there are already spells invested in the staff, you can choose which spell to replace to make room for the new spell.

                As a \\glossterm<minor action>, you can activate this staff.
                When you do, you choose one of the spells that you personally stored in the staff and gain its effects, with yourself as the only target.
                As long as you are attuned to this staff, you do not have to invest an additional attunement point to gain the benefit of a spell in this way, and this does not remove the spell from the staff's storage.
                This effect lasts until you activate the staff again, which can allow you to easily change which benefit you gain.
            """,
            short_description="Change easily between two stored attunements",
        ),
        MagicItem(
            name="Staff of Stored Attunement, Greater",
            rank=4,
            material_type="Staff",
            tags=[],
            description="""
                This staff functions like a \\mitem<staff of stored attunement>, except that you can store up to three spells in the staff.
            """,
            short_description="Change easily between three stored attunements",
        ),
        MagicItem(
            name="Staff of Stored Attunement, Supreme",
            rank=6,
            material_type="Staff",
            tags=[],
            description="""
                This staff functions like a \\mitem<staff of stored attunement>, except that you can store up to four spells in the staff.
            """,
            short_description="Change easily between four stored attunements",
        ),
    ]

    implements += [
        MagicItem(
            name="Baneswallow Staff",
            rank=2,
            material_type="Staff",
            tags=[],
            description="""
                As a standard action, you can activate this staff.
                When you do, you remove one \\glossterm<condition> affecting you.
                This cannot remove an effect applied during the current round.
                If you remove a condition in this way, you \\glossterm<briefly> gain a +2 bonus to your \\glossterm<power>.

                After you use this ability, you increase your \\glossterm<fatigue level> by one.
            """,
            short_description="Can exert and remove a condition to gain power",
        ),
        MagicItem(
            name="Baneswallow Staff, Greater",
            rank=4,
            material_type="Staff",
            tags=[],
            description="""
                As a standard action, you can activate this staff.
                When you do, you remove one \\glossterm<condition> affecting you.
                This cannot remove an effect applied during the current round.
                If you remove a condition in this way, you \\glossterm<briefly> gain a +4 bonus to your \\glossterm<power>.
            """,
            short_description="Can remove a condition to gain power",
        ),
        MagicItem(
            name="Baneswallow Staff, Supreme",
            rank=6,
            material_type="Staff",
            tags=[],
            description="""
                As a standard action, you can activate this staff.
                When you do, you remove one or two \\glossterm<conditions> affecting you.
                This cannot remove effects applied during the current round.
                If you remove at least one condition in this way, you \\glossterm<briefly> gain a +8 bonus to your \\glossterm<power>.
            """,
            short_description="Can remove conditions to gain power",
        ),
    ]

    implements += [
        MagicItem(
            name="Vampiric Staff",
            rank=4,
            material_type="Staff",
            tags=['Healing'],
            description="""
                Once per round, when you cause a creature to lose \\glossterm{hit points} with a \\glossterm{magical} ability, you regain 1d10+4 \\glossterm{hit points}.
            """,
            short_description="Steals 1d10+4 hit points from foes",
        ),
        MagicItem(
            name="Vampiric, Greater",
            rank=6,
            material_type="Staff",
            tags=['Healing'],
            description="""
                Once per round, when you cause a creature to lose \\glossterm{hit points} with a \\glossterm{magical} ability, you regain 2d10+6 \\glossterm{hit points}.
            """,
            short_description="Steals 2d10+6 hit points from foes",
        ),
    ]

    implements += [
        MagicItem(
            name="Splitting Staff",
            rank=4,
            material_type="Staff",
            tags=[],
            description="""
                Whenever you use a \\glossterm<magical> ability that targets a single creature or object and which does not have the \\abilitytag<Sustain> tag, you may activate this staff.
                When you do, increase the number of targets that the spell affects by one.
                In addition, you increase your \\glossterm<fatigue level> by one.
                If the spell does not have a defined range, this staff has no effect on it.
            """,
            short_description="Can exert to add an extra target",
        ),
        MagicItem(
            name="Splitting Staff, Greater",
            rank=6,
            material_type="Staff",
            tags=[],
            description="""
                This staff functions like a \\mitem<splitting staff>, except that activating it does not increase your \\glossterm<fatigue level>.
            """,
            short_description="Can add an extra target",
        ),
    ]

    implements += [
        MagicItem(
            name="Echoing Staff",
            rank=7,
            material_type="Staff",
            tags=[],
            description="""
                Whenever you use a \\glossterm<magical> ability that does not have the \\abilitytag<Sustain> or \\abilitytag<Attune> tags, you may activate this staff.
                When you do, you increase your \\glossterm<fatigue level> by one.
                In addition, during the \glossterm{action phase} of the next round, the spell takes effect again with the same choices for all decisions, such as targets.
            """,
            short_description="Can exert to repeat effect",
        ),
    ]

    # Would be nice to have a staff concept for each mystic sphere

    implements += [
        MagicItem(
            name="Staff of Radiance",
            rank=4,
            material_type="Staff",
            tags=[],
            description="""
                You gain a +1 \\glossterm<accuracy> bonus against creatures that are in \\glossterm<brilliant illumination>.
                In addition, as a standard action, you can \\glossterm<briefly> create \\glossterm<brilliant illumination> in a \\arealarge radius \\glossterm<zone> from you.
            """,
            short_description="Grants +1 accuracy vs foes in brilliant light",
        ),
        MagicItem(
            name="Staff of Radiance, Greater",
            rank=7,
            material_type="Staff",
            tags=[],
            description="""
                You gain a +2 \\glossterm<accuracy> bonus against creatures that are in \\glossterm<brilliant illumination>.
                In addition, as a standard action, you can \\glossterm<briefly> create \\glossterm<brilliant illumination> in a \\areahuge radius \\glossterm<zone> from you.
            """,
            short_description="Grants +2 accuracy vs foes in brilliant light",
        ),
    ]

    implements += [
        MagicItem(
            name="Bushwalker's Staff",
            rank=4,
            material_type="Staff",
            tags=[],
            description="""
                You gain a +1 \\glossterm<accuracy> bonus against creatures that are in \\glossterm<undergrowth>.
                In addition, as a standard action, you can \\glossterm<briefly> create \\glossterm<light undergrowth> in a \\areasmall radius \\glossterm<zone> from you.
            """,
            short_description="Grants +1 accuracy vs foes in undergrowth",
        ),
        MagicItem(
            name="Bushwalker's Staff, Greater",
            rank=7,
            material_type="Staff",
            tags=[],
            description="""
                You gain a +2 \\glossterm<accuracy> bonus against creatures that are in \\glossterm<undergrowth>.
                In addition, as a standard action, you can \\glossterm<briefly> create \\glossterm<light undergrowth> in a \\areamed radius \\glossterm<zone> from you.
            """,
            short_description="Grants +2 accuracy vs foes in undergrowth",
        ),
    ]

    implements += [
        MagicItem(
            name="Staff of Healing Rhythm",
            rank=4,
            material_type="Staff",
            tags=['Healing'],
            description="""
                Whenever you use a \\glossterm<Healing> ability, you gain a +1 \\glossterm<accuracy> bonus during the next round.

                In addition, as a standard action, you can regain 2d10+8 hit points.
                This ability has the \\abilitytag{Healing} tag.
                After you use this ability, you \\glossterm{briefly} cannot use it or any other \\abilitytag{Healing} ability.
            """,
            short_description="Grants +1 accuracy after healing",
        ),
        MagicItem(
            name="Staff of Healing Rhythm, Greater",
            rank=7,
            material_type="Staff",
            tags=['Healing'],
            description="""
                Whenever you use a \\glossterm<Healing> ability, you gain a +2 \\glossterm<accuracy> bonus during the next round.

                In addition, as a standard action, you can regain 5d10+14 hit points.
                This ability has the \\abilitytag{Healing} tag.
                After you use this ability, you \\glossterm{briefly} cannot use it or any other \\abilitytag{Healing} ability.
            """,
            short_description="Grants +2 accuracy after healing",
        ),
    ]

    return implements


def sanity_check(implements):
    pass


def generate_implement_latex(check=False):
    implements = sorted(generate_implements(), key=lambda implements: implements.name)
    if check:
        sanity_check(implements)

    texts = []
    for item in implements:
        try:
            texts.append(item.latex())
        except Exception as e:
            raise Exception(f"Error converting item '{item.name}' to LaTeX") from e

    text = "\n".join(texts)
    return latexify(text)


def generate_implement_table():
    implements = sorted(
        sorted(generate_implements(), key=lambda item: item.name),
        key=lambda item: item.rank,
    )
    rows = [item.latex_table_row() for item in implements]
    row_text = "\n".join(rows)
    return longtablify(
        f"""
        \\lcaption<Implement Items> \\\\
        \\tb<Name> & \\tb<Rank (Cost)> & \\tb<Type> & \\tb<Description> & \\tb<Page> \\tableheaderrule
        {row_text}
    """
    )


def write_to_file():
    implement_latex = generate_implement_latex()
    implement_table = generate_implement_table()
    with open(book_path("implements.tex"), "w") as implement_description_file:
        implement_description_file.write(implement_latex)
    with open(book_path("implements_table.tex"), "w") as implement_table_file:
        implement_table_file.write(implement_table)


@click.command()
@click.option("-c", "--check/--no-check", default=False)
@click.option("-o", "--output/--no-output", default=False)
def main(output, check):
    if output:
        write_to_file()
    else:
        print(generate_implement_latex())


if __name__ == "__main__":
    main()
