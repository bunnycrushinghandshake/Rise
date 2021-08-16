#!/usr/bin/env python3

import click
from rise.latex_generation.book_path import book_path
from rise.latex.magic_item import MagicItem
from rise.latex.util import latexify, longtablify


def generate_apparel():
    apparel = []

    apparel += [
        MagicItem(
            name="Blind Seer's Circlet",
            level=7,
            material_type="Circlet",
            tags=[],
            description="""
                If you have the \\glossterm<blindsense> ability, you increase its range by 30 feet.
                If you have the \\glossterm<blindsight> ability, you increase its range by 15 feet.
            """,
            short_description="Increases range of blindsense and blindsight",
        ),
        MagicItem(
            name="Blind Seer's Circlet, Greater",
            level=13,
            material_type="Circlet",
            tags=[],
            description="""
                If you have the \\glossterm<blindsense> ability, you increase its range by 60 feet.
                If you have the \\glossterm<blindsight> ability, you increase its range by 30 feet.
            """,
            short_description="Greatly increases range of blindsense and blindsight",
        ),
        MagicItem(
            name="Blind Seer's Circlet, Supreme",
            level=19,
            material_type="Circlet",
            tags=[],
            description="""
                If you have the \\glossterm<blindsense> ability, you increase its range by 120 feet.
                If you have the \\glossterm<blindsight> ability, you increase its range by 60 feet.
            """,
            short_description="Drastically increases range of blindsense and blindsight",
        ),
    ]

    # Arm

    apparel += [
        MagicItem(
            name="Gloves of Potency",
            # +2 since gloves are secondary for power
            level=6,
            material_type="Glove",
            description="""
                You gain a +2 \\glossterm<magic bonus> to your \\glossterm<power>.
            """,
            short_description="Grants +2 power",
        ),
        MagicItem(
            name="Gloves of Potency, Greater",
            # +2 since gloves are secondary for power
            level=12,
            material_type="Gauntlet",
            description="""
                You gain a +4 \\glossterm<magic bonus> to your \\glossterm<power>.
            """,
            short_description="Grants +4 power",
        ),
        MagicItem(
            name="Gloves of Potency, Supreme",
            # +2 since gloves are secondary for power
            level=18,
            material_type="Gauntlet",
            description="""
                You gain a +8 \\glossterm<magic bonus> to your \\glossterm<power>.
            """,
            short_description="Grants +8 power",
        ),
    ]

    apparel += [
        MagicItem(
            name="Bracers of Archery",
            level=1,
            material_type="Bracers",
            description="""
                You are proficient with bows.
            """,
            short_description="Grants bow proficiency",
        ),
        MagicItem(
            name="Bracers of Archery, Greater",
            level=7,
            material_type="Bracers",
            description="""
                You are proficient with bows, including exotic bows.
            """,
            short_description="Grants exotic bow proficiency",
        ),
    ]

    apparel += [
        MagicItem(
            name="Bracers of Armor",
            level=4,
            material_type="Bracers",
            tags=[],
            description="""
                You have a translucent suit of magical armor on your body and over your hands.
                This functions like body armor that provides a +2 bonus to Armor defense and has no \\glossterm<encumbrance>.
                It also provides a +4 bonus to \\glossterm{damage resistance}.

                As long as you have a free hand, the barrier also manifests as a shield that provides a +1 bonus to Armor defense.
                This bonus is considered to come from a shield, and does not stack with the benefits of using any other shield.

                The armor and shield provided from this ability are dismissed if you have other body armor of any kind.
            """,
            short_description="Grants encumbrance-free +2 armor",
        ),
        MagicItem(
            name="Bracers of Armor, Greater",
            level=10,
            material_type="Bracers",
            tags=[],
            description="""
                These bracers function like \\textit<bracers of armor>, except that the defense bonus from the body armor increases to +3.
                In addition, its bonus to \\glossterm{damage resistance} increases to +8.
            """,
            short_description="Grants encumbrance-free +3 armor",
        ),
        MagicItem(
            name="Bracers of Armor, Supreme",
            level=16,
            material_type="Bracers",
            tags=[],
            description="""
                These bracers function like \\textit<bracers of armor>, except that the defense bonus from the body armor increases to +4.
                In addition, its bonus to \\glossterm{damage resistance} increases to +16.
            """,
            short_description="Grants encumbrance-free +4 armor",
        ),
    ]

    apparel += [
        MagicItem(
            name="Shieldburst Bracers",
            level=3,
            material_type="Bracers",
            tags=["Swift"],
            description="""
                As a \\glossterm<free action>, you activate these bracers.
                When you do, you increase your \\glossterm<fatigue level> by one and gain a +2 bonus to Armor defense until the end of the round.
                This ability has the \\glossterm<Swift> tag, so it protects you against attacks against you during the current phase.
            """,
            short_description="Can grant brief +2 Armor defense",
        ),
        MagicItem(
            name="Shieldburst Bracers, Greater",
            level=9,
            material_type="Bracers",
            tags=[],
            description="""
                These bracers function like \\textit<shieldburst bracers>, except that the defense bonus increases to +3.
            """,
            short_description="Can grant brief +3 Armor defense",
        ),
        MagicItem(
            name="Shieldburst Bracers, Supreme",
            level=15,
            material_type="Bracers",
            tags=[],
            description="""
                These bracers function like \\textit<shieldburst bracers>, except that the defense bonus increases to +4.
            """,
            short_description="Can grant brief +4 Armor defense",
        ),
    ]

    apparel += [
        MagicItem(
            name="Bracers of Repulsion",
            level=7,
            material_type="Bracers",
            description="""
                As a standard action, you can activate these bracers.
                When you do, they emit a telekinetic burst of force.
                Make an attack vs. Fortitude against everything within a \\areasmall radius burst from you.
                If you use this item during the \\glossterm<delayed action phase>,
                    you gain a +4 bonus to \\glossterm<accuracy> with this attack against any creature that attacked you during the \\glossterm<action phase>.
                On a hit, you \\glossterm<knockback> each target up to 20 feet in a straight line directly away from you.
            """,
            short_description="Can knock nearby creatures back",
        ),
        MagicItem(
            name="Bracers of Repulsion, Greater",
            level=16,
            material_type="Bracers",
            description="""
                These bracers function like \\mitem<bracers of repulsion>, except that it targets your \\glossterm<enemies> within a \\arealarge radius burst.
            """,
            short_description="Can knock enemies back",
        ),
    ]

    apparel.append(
        MagicItem(
            name="Torchlight Gloves",
            level=2,
            material_type="Gloves",
            tags=[],
            description="""
            These gloves shed light as a torch.
            As a \\glossterm<standard action>, you may snap your fingers to suppress or resume the light from either or both gloves.
        """,
            short_description="Sheds light as a torch",
        )
    )

    apparel += [
        MagicItem(
            name="Gauntlets of Improvisation",
            level=3,
            material_type="Gauntlet",
            description="""
                You gain a +2 \\glossterm<magic bonus> to \\glossterm<power> with \\glossterm<strikes> using \\glossterm<improvised weapons>.
            """,
            short_description="Grants +2 power with improvised weapons",
        ),
        MagicItem(
            name="Gauntlets of Improvisation, Greater",
            level=9,
            material_type="Gauntlet",
            description="""
                You gain a +4 \\glossterm<magic bonus> to \\glossterm<power> with \\glossterm<strikes> using \\glossterm<improvised weapons>.
            """,
            short_description="Grants +4 power with improvised weapons",
        ),
        MagicItem(
            name="Gauntlets of Improvisation, Supreme",
            level=15,
            material_type="Gauntlet",
            description="""
                You gain a +8 \\glossterm<magic bonus> to \\glossterm<power> with \\glossterm<strikes> using \\glossterm<improvised weapons>.
            """,
            short_description="Grants +8 power with improvised weapons",
        ),
    ]

    apparel += [
        MagicItem(
            name="Gauntlet of the Ram",
            level=6,
            material_type="Gauntlet",
            description="""
                This item has the Forceful \\glossterm<weapon tag> (see \\pcref<Weapon Tags>).
            """,
            short_description="Knocks back foe when used to strike",
        ),
        MagicItem(
            name="Gauntlet of the Ram, Greater",
            level=14,
            material_type="Gauntlet",
            description="""
                This item has the Forceful \\glossterm<weapon tag> (see \\pcref<Weapon Tags>).
                In addition, the \\glossterm<knockback> distance from that tag increases to 30 feet.
            """,
            short_description="Knocks back foe farther when use to strike",
        ),
    ]

    apparel += [
        MagicItem(
            name="Greatreach Bracers",
            level=12,
            material_type="Bracers",
            description="""
                You gain a +5 foot \\glossterm<magic bonus> to your \\glossterm<reach> with melee weapons.
            """,
            short_description="Increases reach by five feet",
        ),
        MagicItem(
            name="Greatreach Bracers, Greater",
            level=20,
            material_type="Bracers",
            description="""
                You gain a +10 foot \\glossterm<magic bonus> to your \\glossterm<reach> with melee weapons.
            """,
            short_description="Increases reach by ten feet",
        ),
    ]

    apparel.append(
        MagicItem(
            name="Throwing Gloves",
            level=5,
            material_type="Gloves",
            description="""
            % TODO: reference basic "not designed to be thrown" mechanics?
            You can throw any item as if it was designed to be thrown.
            This does not improve your ability to throw items designed to be thrown, such as darts.
        """,
            short_description="Allows throwing any item accurately",
        )
    )
    # Head

    apparel += [
        # close range, +1d = rank 2 spell, so this is priced as rank 3
        # This could be med range +0d, but that's weird as a legacy item choice
        MagicItem(
            name="Circlet of Blasting",
            level=7,
            material_type="Circlet",
            tags=[],
            description="""
                As a standard action, you can activate this circlet.
                If you do, make an attack vs. Armor against a creature or object within \\rngshort range.
                \\hit The target takes 2d10+3 fire damage.
                \\glance As above, except that that the target takes half damage.
            """,
            short_description="Can blast foe with fire",
        ),
        # close range, +2d = rank 4 spell, so this is priced as rank 5
        MagicItem(
            name="Circlet of Blasting, Greater",
            level=13,
            material_type="Circlet",
            tags=[],
            description="""
                As a standard action, you can activate this circlet.
                If you do, make an attack vs. Armor against a creature or object within \\rngshort range.
                \\hit The target takes 4d10+6 fire damage.
                \\glance As above, except that that the target takes half damage.
            """,
            short_description="Can blast foe with intense fire",
        ),
        # close range, +3d = rank 6 spell, so this is priced as rank 7
        MagicItem(
            name="Circlet of Blasting, Supreme",
            level=19,
            material_type="Circlet",
            tags=[],
            description="""
                As a standard action, you can activate this circlet.
                If you do, make an attack vs. Armor against a creature or object within \\rngmed range.
                \\hit The target takes 7d10+9 fire damage.
                \\glance As above, except that that the target takes half damage.
            """,
            short_description="Can blast foe with supremely intense fire",
        ),
    ]

    apparel += [
        MagicItem(
            name="Circlet of Persuasion",
            level=3,
            material_type="Circlet",
            description="""
                You gain a +2 \\glossterm<magic bonus> to the Persuasion skill (see \\pcref<Persuasion>).
            """,
            short_description="Grants +2 Persuasion",
        ),
        MagicItem(
            name="Circlet of Persuasion, Greater",
            level=9,
            material_type="Circlet",
            description="""
                You gain a +3 \\glossterm<magic bonus> to the Persuasion skill (see \\pcref<Persuasion>).
            """,
            short_description="Grants +3 Persuasion",
        ),
        MagicItem(
            name="Circlet of Persuasion, Supreme",
            level=15,
            material_type="Circlet",
            description="""
                You gain a +4 \\glossterm<magic bonus> to the Persuasion skill (see \\pcref<Persuasion>).
            """,
            short_description="Grants +4 Persuasion",
        ),
    ]

    apparel += [
        MagicItem(
            name="Circlet of Foresight",
            level=3,
            material_type="Circlet",
            description="""
                You gain a +2 \\glossterm<magic bonus> to \\glossterm<initiative> checks.
            """,
            short_description="Grants +2 initiative",
        ),
        MagicItem(
            name="Circlet of Foresight, Greater",
            level=9,
            material_type="Circlet",
            description="""
                You gain a +3 \\glossterm<magic bonus> to \\glossterm<initiative> checks.
            """,
            short_description="Grants +3 initiative",
        ),
        MagicItem(
            name="Circlet of Foresight, Supreme",
            level=15,
            material_type="Circlet",
            description="""
                You gain a +4 \\glossterm<magic bonus> to \\glossterm<initiative> checks.
            """,
            short_description="Grants +4 initiative",
        ),
    ]

    apparel += [
        MagicItem(
            name="Circlet of Many Eyes",
            level=10,
            material_type="Circlet",
            description="""
                You reduce your penalties for being \\glossterm<surrounded> by 1.
            """,
            short_description="Reduces penalty for being \\glossterm<surrounded> by 1",
        ),
        MagicItem(
            name="Circlet of Many Eyes, Greater",
            level=16,
            material_type="Circlet",
            description="""
                You reduce your penalties for being \\glossterm<surrounded> by 2.
                You are still considered to be surrounded for the purpose of other abilities, even this reduces your penalties to 0.
            """,
            short_description="Reduces penalty for being \\glossterm<surrounded> by 2",
        ),
    ]

    apparel.append(
        MagicItem(
            name="Mask of Water Breathing",
            level=4,
            material_type="Mask",
            description="""
            You can breathe water through this mask as easily as a human breaths air.
            This does not grant you the ability to breathe other liquids.
        """,
            short_description="Allows breathing water like air",
        )
    )

    apparel.append(
        MagicItem(
            name="Ring of Breath",
            level=1,
            material_type="Ring",
            tags=[],
            description="""
            As a \\glossterm<minor action>, you can activate this item.
            When you do, you increase your \\glossterm<fatigue level> by one, and you can \\glossterm<briefly> breathe in clean, fresh air regardless of your environment.
            This can be used in emergencies to save yourself from drowning or other perils.
        """,
            short_description="Allows limited breathing",
        )
    )

    apparel.append(
        MagicItem(
            name="Mask of Air",
            level=9,
            material_type="Mask",
            description="""
            If you breathe through this mask, you breathe in clean, fresh air, regardless of your environment.
            This can protect you from inhaled poisons and similar effects.
        """,
            short_description="Allows breathing in any environment",
        )
    )

    apparel += [
        MagicItem(
            name="Crown of Flame",
            level=4,
            material_type="Crown",
            tags=[],
            description="""
                This crown is continuously on fire.
                The flame sheds light as a torch.

                You and your \\glossterm<allies> within a \\areamed radius emanation from you
                    gain a +2 bonus to \\glossterm<defenses> against attacks that deal fire damage.
            """,
            short_description="Grants you and allies +2 defenses vs fire",
        ),
        MagicItem(
            name="Crown of Flame, Greater",
            level=13,
            material_type="Crown",
            tags=[],
            description="""
                This crown is continuously on fire.
                The flame sheds light as a torch.

                You and your \\glossterm<allies> within a \\areamed radius emanation from you
                    gain a +4 bonus to \\glossterm<defenses> against attacks that deal fire damage.
            """,
            short_description="Grants you and allies +4 defenses vs fire",
        ),
    ]

    apparel.append(
        MagicItem(
            name="Crown of Lightning",
            level=7,
            material_type="Crown",
            tags=[],
            description="""
                This crown continuously crackles with electricity.
                The constant sparks shed light as a torch.

                As a standard action, you can intensify the crown's energy to shock nearby enemies.
                When you do, make an attack vs. Fortitude against your \\glossterm<enemies> within a \\areasmall radius from you.
                On a hit, each target takes 2d6+1 electricity damage.
                On a \\glossterm<glancing blow>, each target takes half damage.
            """,
            short_description="Can damage nearby enemies",
        )
    )

    apparel.append(
        MagicItem(
            name="Crown of Frost",
            level=13,
            material_type="Crown",
            tags=[],
            description="""
            This crown continuously emits a chilling aura around you.
            You gain a +4 bonus to \\glossterm<defenses> against attacks that deal cold damage.

            As a standard action, you can intensify the crown's energy to freeze nearby enemies.
            When you do, make an attack vs. Fortitude against all \\glossterm<enemies> within a \\areasmall radius from you.
            On a hit, each target with no remaining \\glossterm<damage resistance> is \\glossterm{briefly} \\glossterm<immobilized>.
            On a critical hit, each target with no remaining \\glossterm<damage resistance> is immobilized as a \\glossterm<condition>.
        """,
            short_description="Can freeze nearby enemies",
        )
    )

    apparel.append(
        MagicItem(
            name="Crown of Thunder",
            level=11,
            material_type="Crown",
            tags=[],
            description="""
            The crown constantly emits a low-pitched rumbling.
            To you and your \\glossterm<allies>, the sound is barely perceptible.
            However, all other creatures within a \\arealarge radius emanation from you hear the sound as a deafening, continuous roll of thunder.
            The noise blocks out all other sounds quieter than thunder, causing them to be \\deafened while they remain in the area.
        """,
            short_description="Continously deafens nearby enemies",
        )
    )

    # Legs

    apparel.append(
        MagicItem(
            name="Crater Boots",
            level=10,
            material_type="Boots",
            description="""
            % This only works if you only take falling damage during the movement phase, which seems possible?
            When you take \\glossterm<falling damage>, make an attack vs Reflex against everything within a \\areasmall radius from you.
            \\hit Each target takes bludgeoning damage equal to the damage you took from falling.
            \\crit As above, and each target is knocked \\glossterm<prone>.
            This does not deal double damage on a critical hit.
        """,
            short_description="Deals your falling damage to enemies",
        )
    )

    apparel += [
        MagicItem(
            name="Phasestep Boots",
            level=4,
            material_type="Boots",
            tags=["Swift"],
            description="""
                As a \\glossterm<free action>, you can activate these boots.
                When you do, you increase your \\glossterm<fatigue level> by one, and you may move through creatures freely when you move using one of your movement speeds until the end of the round.
                This does not allow you to move through inanimate objects.
                If you end your movement in spaces occupied by other creatures, both of you are still \\squeezing.
                If you are not able to move normally, such as if you are \\grappled, these boots do not help you.
            """,
            short_description="Can briefly move through creatures",
        ),
        MagicItem(
            name="Phasestep Boots, Greater",
            level=10,
            material_type="Boots",
            tags=["Swift"],
            description="""
                These boots function like \\mitem<phasestep boots>, except that their effect is always active.
            """,
            short_description="Can move through creatures",
        ),
        MagicItem(
            name="Phasestep Boots, Supreme",
            level=16,
            material_type="Boots",
            tags=["Swift"],
            description="""
                These boots function like \\mitem<phasestep boots>, except that their effect is always active.
                In addition, you ignore all sources of \\glossterm<difficult terrain>.
            """,
            short_description="Can move through creatures and some terrain",
        ),
    ]

    apparel += [
        MagicItem(
            name="Boots of the Skydancer",
            level=7,
            material_type="Boots",
            tags=["Swift"],
            description="""
                As a \\glossterm<free action>, you can activate these boots.
                When you do, you may treat air as if it were solid ground to your feet for the rest of the current phase.
                You may selectively choose when to treat the air as solid ground, allowing you to walk or jump on air freely.
                After using this ability, you cannot use it again until these boots touch the ground.
            """,
            short_description="Can very briefly walk on air",
        ),
        MagicItem(
            name="Boots of the Skydancer, Greater",
            level=13,
            material_type="Boots",
            tags=["Swift"],
            description="""
                These boots function like \\magicitem<boots of the skydancer>, except that the ability lasts until the end of the round.
                In addition, you can use this item twice before the boots touch the ground.
            """,
            short_description="Can briefly walk on air",
        ),
    ]

    apparel += [
        MagicItem(
            name="Boots of Freedom",
            level=10,
            material_type="Boots",
            description="""
                You are immune to being \\slowed, \\decelerated, \\immobilized, and \\paralyzed.
            """,
            short_description="Grants immunity to common mobility restrictions",
        ),
        MagicItem(
            name="Boots of Freedom, Greater",
            level=16,
            material_type="Boots",
            description="""
                You are immune to all effects that reduce your movement speed or prevent you from moving, including nonmagical effects such as \\glossterm<difficult terrain>.
                This removes all penalties you would suffer for acting underwater, except for those relating to using ranged weapons.
                This does not prevent you from being \\grappled, but you gain a +10 bonus to defenses against the \\textit<grapple> ability (see \\pcref<Grapple>).
            """,
            short_description="Grants immunity to almost all mobility restrictions",
        )
    ]

    apparel += [
        MagicItem(
            name="Boots of Gravitation",
            level=7,
            material_type="Boots",
            description="""
                Once per phase, while you are within 5 feet of an \\glossterm<unattended> object at least one size category larger than you, you can take a \\glossterm<free action> to adjust your personal gravity.
                When you do, gravity pulls you towards that surface instead of in the normal direction.
                This allows you to walk normally on walls or even ceilings.

                Whenever you change the direction that gravity pulls you, you must make a \\glossterm<difficulty rating> 10 Balance check to keep your feet.
                Failure means you fall \\prone and your movement for that phase ends.
            """,
            short_description="Redirects personal gravity to adjacent objects",
        ),
        MagicItem(
            name="Boots of Gravitation, Greater",
            level=13,
            material_type="Boots",
            description="""
                These boots function like \\mitem<boots of gravitation>, except that the maximum distance increases to 15 feet.
                This can allow you to pull yourself towards distant objects, though you may take falling damage if you fall too far.
            """,
            short_description="Redirects personal gravity to nearby objects",
        ),
        MagicItem(
            name="Boots of Gravitation, Supreme",
            level=19,
            material_type="Boots",
            description="""
                These boots function like \\mitem<boots of gravitation>, except that the maximum distance increases to 30 feet.
                This can allow you to pull yourself towards distant objects, though you may take falling damage if you fall too far.
            """,
            short_description="Redirects personal gravity to distant objects",
        ),
    ]

    apparel += [
        MagicItem(
            name="Boots of Speed",
            level=4,
            material_type="Boots",
            tags=[],
            description="""
                You gain a +5 foot \\glossterm<magic bonus> to your land speed.
            """,
            short_description="Increases speed by 5 feet",
        ),
        MagicItem(
            name="Boots of Speed, Greater",
            level=10,
            material_type="Boots",
            tags=[],
            description="""
                You gain a +10 foot \\glossterm<magic bonus> to your land speed.
            """,
            short_description="Increases speed by 10 feet",
        ),
        MagicItem(
            name="Boots of Speed, Supreme",
            level=16,
            material_type="Boots",
            tags=[],
            description="""
                You gain a +15 foot \\glossterm<magic bonus> to your land speed.
            """,
            short_description="Increases speed by 15 feet",
        ),
    ]

    apparel += [
        MagicItem(
            name="Boots of the Fearful",
            level=5,
            material_type="Boots",
            tags=[],
            description="""
                While you are \\shaken, \\frightened, or \\panicked and within \\medrange of the source of your fear, you gain a +10 foot \\glossterm<magic bonus> to your land speed.
            """,
            short_description="Increases speed by 10 feet while afraid",
        ),
        MagicItem(
            name="Boots of the Fearful, Greater",
            level=11,
            material_type="Boots",
            tags=[],
            description="""
                While you are \\shaken, \\frightened, or \\panicked and within \\medrange of the source of your fear, you gain a +15 foot \\glossterm<magic bonus> to your land speed.
            """,
            short_description="Increases speed by 15 feet while afraid",
        ),
        MagicItem(
            name="Boots of the Fearful, Supreme",
            level=17,
            material_type="Boots",
            tags=[],
            description="""
                While you are \\shaken, \\frightened, or \\panicked and within \\medrange of the source of your fear, you gain a +20 foot \\glossterm<magic bonus> to your land speed.
            """,
            short_description="Increases speed by 20 feet while afraid",
        ),
    ]

    apparel += [
        MagicItem(
            name="Belt of Scuttling",
            level=5,
            material_type="Belt",
            tags=[],
            description="""
                Being \\prone does not reduce your movement speed.
            """,
            short_description="Move at full speed while prone",
        ),
        MagicItem(
            name="Belt of Scuttling, Greater",
            level=11,
            material_type="Belt",
            tags=[],
            description="""
                Being \\prone does not reduce your movement speed or your defenses.
            """,
            short_description="Move at full speed and defend normally while prone",
        ),
        MagicItem(
            name="Belt of Scuttling, Supreme",
            level=17,
            material_type="Belt",
            tags=[],
            description="""
                Being \\prone does not reduce your movement speed, defenses, or accuracy.
            """,
            short_description="Act normally while prone",
        ),
    ]

    apparel += [
        MagicItem(
            name="Lifesaver Ring",
            level=4,
            material_type="Ring",
            tags=['Healing'],
            description="""
                At the end of each round, if you are at or below half your maximum \\glossterm<hit points>, you regain 1d6 hit points.
            """,
            short_description="Regains 1d6 hit points while at half hit points",
        ),
        MagicItem(
            name="Lifesaver Ring, Greater",
            level=10,
            material_type="Ring",
            tags=['Healing'],
            description="""
                At the end of each round, if you are at or below half your maximum \\glossterm<hit points>, you regain 2d6 hit points.
            """,
            short_description="Regains 2d6 hit points while at half hit points",
        ),
        MagicItem(
            name="Lifesaver Ring, Supreme",
            level=16,
            material_type="Ring",
            tags=['Healing'],
            description="""
                At the end of each round, if you are at or below half your maximum \\glossterm<hit points>, you regain 4d6 hit points.
            """,
            short_description="Regains 4d6 hit points while at half hit points",
        ),
    ]

    apparel += [
        MagicItem(
            name="Dazeshare Ring",
            level=6,
            material_type="Ring",
            tags=['Healing'],
            description="""
                Whenever you become \\dazed, \\stunned, or \\confused, make an attack vs. Mental against all \\glossterm<enemies> adjacent to you.
                On a hit, each subject becomes \\glossterm<briefly> dazed.
            """,
            short_description="Dazes enemies when you are dazed or worse",
        ),
        MagicItem(
            name="Stunshare Ring",
            level=15,
            material_type="Ring",
            tags=['Healing'],
            description="""
                Whenever you become \\dazed, \\stunned, or \\confused, make an attack vs. Mental against all \\glossterm<enemies> adjacent to you.
                On a hit, each subject becomes \\glossterm<briefly> stunned.
            """,
            short_description="Stuns enemies when you are dazed or worse",
        ),
    ]

    apparel.append(
        MagicItem(
            name="Astral Boots",
            level=16,
            material_type="Boots",
            tags=[],
            description="""
                When you move using one of your movement speeds, you can teleport the same distance instead.
                This does not change the total distance you can move, but you can teleport in any direction, even vertically.
                You must teleport into an unoccupied location onto a stable surface that can support your weight.
                You cannot teleport to locations you do not have \\glossterm<line of sight> and \\glossterm<line of effect> to.
            """,
            short_description="Allows teleporting instead of moving",
        )
    )

    apparel.append(
        MagicItem(
            name="Boots of Water Walking",
            level=7,
            material_type="Boots",
            description="""
            You treat the surface of all liquids as if they were firm ground.
            Your feet hover about an inch above the liquid's surface, allowing you to traverse dangerous liquids without harm as long as the surface is calm.

            If you are below the surface of the liquid, you rise towards the surface at a rate of 60 feet per round.
            Thick liquids, such as mud and lava, may cause you to rise more slowly.
        """,
            short_description="Allows walking on liquids",
        )
    )

    apparel.append(
        MagicItem(
            name="Boots of the Winterlands",
            level=2,
            material_type="Boots",
            description="""
            You can travel across snow and ice without slipping or suffering movement penalties for the terrain.
            % TODO: degree symbol?
            In addition, the boots keep you warm, protecting you in environments as cold as -50 degrees Fahrenheit.
        """,
            short_description="Eases travel in cold areas",
        )
    )

    apparel.append(
        MagicItem(
            name="Cloak of the Unseen Hunter",
            level=17,
            material_type="Cloak",
            description="""
                You gain a +1 \\glossterm<accuracy> bonus against creatures that you have \\glossterm<concealment> from.
            """,
            short_description="Grants +1 accuracy while you have concealment",
        ),
    )

    apparel.append(
        MagicItem(
            name="Boots of the Desertlands",
            level=2,
            material_type="Boots",
            description="""
            You can travel across sand, including quicksand, without slipping or suffering movement penalties for the terrain.
            % TODO: degree symbol?
            In addition, the boots keep you cool, protecting you in environments as warm as 100 degrees Fahrenheit.
        """,
            short_description="Eases travel in deserts",
        )
    )

    apparel.append(
        MagicItem(
            name="Seven League Boots",
            level=12,
            material_type="Boots",
            tags=[],
            description="""
            As a standard action, you can activate these boots.
            When you do, you increase your \\glossterm<fatigue level> by one and teleport horizontally exactly 25 miles in a direction you specify.
            If this would place you within a solid object or otherwise impossible space, the boots will shunt you up to 1,000 feet in any direction to the closest available space.
            If there is no available space within 1,000 feet of your intended destination, the effect fails and you take 4d6 energy damage.
        """,
            short_description="Teleport seven leages with a step",
        )
    )

    apparel += [
        MagicItem(
            name="Winged Boots",
            level=13,
            material_type="Boots",
            description="""
                You gain a \\glossterm<fly speed> equal to the \\glossterm<base speed> for your size with a maximum height of 15 feet (see \\pcref<Flying>).
                If you are above that height, you gain a \\glossterm<glide speed> equal to the base speed for your size instead.
            """,
            short_description="Grants flight up to 15 feet high",
        ),
        MagicItem(
            name="Winged Boots, Greater",
            level=19,
            material_type="Boots",
            description="""
                These boots function like \\mitem<winged boots>, except that the \\glossterm<height limit> increases to 30 feet.
            """,
            short_description="Grants flight up to 30 feet high",
        ),
    ]

    apparel += [
        MagicItem(
            name="Boots of Elvenkind",
            level=3,
            material_type="Boots",
            description="""
                You gain a +2 \\glossterm<magic bonus> to the Stealth skill (see \\pcref<Stealth>).
            """,
            short_description="Grants +2 Stealth",
        ),
        MagicItem(
            name="Boots of Elvenkind, Greater",
            level=6,
            material_type="Boots",
            description="""
                You gain a +3 \\glossterm<magic bonus> to the Stealth skill (see \\pcref<Stealth>).
            """,
            short_description="Grants +3 Stealth",
        ),
        MagicItem(
            name="Boots of Elvenkind, Supreme",
            level=12,
            material_type="Boots",
            description="""
                You gain a +4 \\glossterm<magic bonus> to the Stealth skill (see \\pcref<Stealth>).
            """,
            short_description="Grants +4 Stealth",
        ),
    ]

    # Rings

    apparel.append(
        MagicItem(
            name="Ring of Elemental Endurance",
            level=2,
            material_type="Ring",
            tags=[],
            description="""
            You can exist comfortably in conditions between -50 and 140 degrees Fahrenheit without any ill effects.
            You suffer the normal penalties in temperatures outside of that range.
        """,
            short_description="Grants tolerance of temperature extremes",
        )
    )

    apparel += [
        MagicItem(
            name="Ring of the True Form",
            level=3,
            material_type="Ring",
            tags=[],
            description="""
                You gain a +4 bonus to defenses against attacks from the \\sphere<polymorph> sphere.
                This bonus also applies against other attacks that significantly alter your physical form, such as an aboleth's slime.
            """,
            short_description="Grants +4 defenses vs form-altering attacks",
        ),
        MagicItem(
            name="Ring of the True Form, Greater",
            level=9,
            material_type="Ring",
            tags=[],
            description="""
                You gain a +8 bonus to defenses against attacks from the \\textit<polymorph> sphere.
                This bonus also applies against other attacks that significantly alter your physical form, such as an aboleth's slime.
            """,
            short_description="Grants +8 defenses vs form-altering attacks",
        ),
        MagicItem(
            name="Ring of the True Form, Supreme",
            level=15,
            material_type="Ring",
            tags=[],
            description="""
                You are immune to attacks from the \\textit<polymorph> sphere.
                This immunity also applies against other attacks that significantly alter your physical form, such as an aboleth's slime.
            """,
            short_description="Grants immunity to form-altering attacks",
        ),
    ]

    apparel += [
        MagicItem(
            name="Ring of Honeyed Words",
            level=6,
            material_type="Ring",
            tags=[],
            description="""
                You gain a +2 \\glossterm<magic bonus> to the Deception, Intimidate and Persuasion skills.
            """,
            short_description="Grants +2 to social skills",
        ),
        MagicItem(
            name="Ring of Honeyed Words, Greater",
            level=12,
            material_type="Ring",
            tags=[],
            description="""
                You gain a +3 \\glossterm<magic bonus> to the Deception, Intimidate and Persuasion skills.
            """,
            short_description="Grants +3 to social skills",
        ),
        MagicItem(
            name="Ring of Honeyed Words, Supreme",
            level=18,
            material_type="Ring",
            tags=[],
            description="""
                You gain a +4 \\glossterm<magic bonus> to the Deception, Intimidate and Persuasion skills.
            """,
            short_description="Grants +4 to social skills",
        ),
    ]

    apparel += [
        MagicItem(
            name="Poisonbane Ring",
            level=2,
            material_type="Ring",
            tags=[],
            description="""
                You gain a +2 bonus to defenses against \\glossterm<poisons>.
            """,
            short_description="Grants +2 defenses vs poisons",
        ),
        MagicItem(
            name="Poisonbane Ring, Greater",
            level=8,
            material_type="Ring",
            tags=[],
            description="""
                You gain a +4 bonus to defenses against \\glossterm<poisons>.
            """,
            short_description="Grants +4 defenses vs poisons",
        ),
        MagicItem(
            name="Poisonbane Ring, Supreme",
            level=14,
            material_type="Ring",
            tags=[],
            description="""
                You are immune to \\glossterm<poisons>.
            """,
            short_description="Grants immunity to poisons",
        ),
    ]

    apparel += [
        MagicItem(
            name="Ring of Blessed Protection",
            level=5,
            material_type="Ring",
            tags=[],
            description="""
                Whenever you are hit by a \\glossterm<critical hit> from a \\glossterm<strike>, you may activate this item.
                When you do, you increase your \\glossterm<fatigue level> by two, and the attacker rerolls the attack against you, which may prevent the attack from getting a critical hit against you.
                This does not protect any other targets of the attack.
                You can choose to use this item after you learn the effects that the critical hit would have, but you must do so during the phase that the attack was made.
            """,
            short_description="Can protect against critical strikes",
        ),
        MagicItem(
            name="Ring of Blessed Protection, Greater",
            level=11,
            material_type="Ring",
            tags=[],
            description="""
                This item functions like a \\textit<ring of blessed protection>, except that it also protects against any \\glossterm<mundane> attack, not just strikes.
            """,
            short_description="Can protect against critical mundane attacks",
        ),
        MagicItem(
            name="Ring of Blessed Protection, Supreme",
            level=17,
            material_type="Ring",
            tags=[],
            description="""
                This item functions like a \\textit<ring of blessed protection>, except that it protects against any attack, not just strikes.
            """,
            short_description="Can protect against critical attacks",
        ),
    ]

    apparel += [
        MagicItem(
            name="Ring of Nourishment",
            level=3,
            material_type="Ring",
            tags=["Creation"],
            description="""
                You continuously gain nourishment, and no longer need to eat or drink.
                This ring must be worn for 24 hours before it begins to work.
            """,
            short_description="Provides food and water",
        ),
        MagicItem(
            name="Ring of Sustenance",
            level=7,
            material_type="Ring",
            tags=["Creation"],
            description="""
                You continuously gain nourishment, and no longer need to eat or drink.
                The ring must be worn for 24 hours before it begins to work.

                In addition, you need only one-quarter your normal amount of sleep (or similar activity, such as elven trance) each day.
                """,
            short_description="Provides food, water, and rest",
        ),
    ]

    apparel.append(
        MagicItem(
            name="Ring of Vital Regeneration",
            level=15,
            material_type="Ring",
            tags=[],
            description="""
                At the end of each round, you can remove one of your \\glossterm<vital wounds>.
                This cannot remove a vital wound you gained during the current round.
                When you do, you increase your \\glossterm<fatigue level> by two.
            """,
            short_description="Automatically removes vital wounds",
        )
    )

    apparel += [
        MagicItem(
            name="Bracers of Mighty Fists",
            level=3,
            material_type="Bracers",
            description="""
                You gain a +2 \\glossterm<magic bonus> to \\glossterm<power> with \\glossterm<strikes> using \\glossterm<unarmed attacks> and natural weapons.
            """,
            short_description="Grants +2 power with natural and unarmed attacks",
        ),
        MagicItem(
            name="Bracers of Mighty Fists, Greater",
            level=9,
            material_type="Bracers",
            description="""
                You gain a +4 \\glossterm<magic bonus> to \\glossterm<power> with \\glossterm<strikes> using \\glossterm<unarmed attacks> and natural weapons.
            """,
            short_description="Grants +4 power with natural and unarmed attacks",
        ),
        MagicItem(
            name="Bracers of Mighty Fists, Supreme",
            level=15,
            material_type="Bracers",
            description="""
                You gain a +8 \\glossterm<magic bonus> to \\glossterm<power> with \\glossterm<strikes> using \\glossterm<unarmed attacks> and natural weapons.
            """,
            short_description="Grants +8 power with natural and unarmed attacks",
        ),
    ]

    apparel += [
        MagicItem(
            name="Belt of Health",
            level=4,
            material_type="Belt",
            description="""
                You gain a +4 \\glossterm<magic bonus> to your \\glossterm<hit points>.
            """,
            short_description="Grants +4 hit points",
        ),
        MagicItem(
            name="Belt of Health, Greater",
            level=10,
            material_type="Belt",
            description="""
                You gain a +8 \\glossterm<magic bonus> to your \\glossterm<hit points>.
            """,
            short_description="Grants +8 hit points",
        ),
        MagicItem(
            name="Belt of Health, Supreme",
            level=16,
            material_type="Belt",
            description="""
                You gain a +16 \\glossterm<magic bonus> to your \\glossterm<hit points>.
            """,
            short_description="Grants +16 hit points",
        ),
    ]

    apparel.append(
        MagicItem(
            name="Ring of the Planes",
            level=12,
            material_type="Ring",
            tags=[],
            description="""
            When you perform the \\ritual<plane shift> ritual, this ring provides all \\glossterm<fatigue levels> required.
            This does not grant you the ability to perform the \\ritual<plane shift> ritual if you could not already.
        """,
            short_description="Aids travel with \\ritual<plane shift>",
        )
    )

    apparel += [
        MagicItem(
            name="Cloak of Nondetection",
            level=6,
            material_type="Cloak",
            tags=[],
            description="""
                You gain a +4 bonus to defenses against attacks with the \\glossterm<Detection> or \\glossterm<Scrying> tags.
            """,
            short_description="Grants +4 to defenses against detection",
        ),
        MagicItem(
            name="Cloak of Nondetection, Greater",
            level=14,
            material_type="Cloak",
            tags=[],
            description="""
                You gain a +8 bonus to defenses against attacks with the \\glossterm<Detection> or \\glossterm<Scrying> tags.
            """,
            short_description="Grants +8 to defenses against detection",
        ),
    ]

    apparel += [
        MagicItem(
            name="Challenger's Cloak",
            level=13,
            material_type="Cloak",
            tags=[],
            description="""
                Each creature \\goaded by you takes an additional -1 \\glossterm<accuracy> penalty against creatures other than you while it is within \\medrange of you.
            """,
            short_description="Increases accuracy penalty on goaded foes by 1",
        ),
        MagicItem(
            name="Challenger's Cloak",
            level=19,
            material_type="Cloak",
            tags=[],
            description="""
                Each creature \\goaded by you takes an additional -2 \\glossterm<accuracy> penalty against creatures other than you while it is within \\medrange of you.
            """,
            short_description="Increases accuracy penalty on goaded foes by 2",
        ),
    ]

    # Cloaks

    apparel += [
        MagicItem(
            name="Quilled Cloak",
            level=6,
            material_type="Cloak",
            tags=[],
            description="""
                Whenever a creature grapples you, you immediately deal it 2d6+3 piercing damage.
                This does not affect creatures that you initiate a grapple with.
            """,
            short_description="Deals damage to creatures that grapple you",
        ),
        MagicItem(
            name="Greater Quilled Cloak",
            level=12,
            material_type="Cloak",
            tags=[],
            description="""
                Whenever a creature grapples you, you immediately deal it 4d6+6 piercing damage.
                This does not affect creatures that you initiate a grapple with.
            """,
            short_description="Deals more damage to creatures that grapple you",
        ),
        MagicItem(
            name="Supreme Quilled Cloak",
            level=18,
            material_type="Cloak",
            tags=[],
            description="""
                Whenever a creature grapples you, you immediately deal it 5d10+9 piercing damage.
                This does not affect creatures that you initiate a grapple with.
            """,
            short_description="Deals even more damage to creatures that grapple you",
        ),
    ]

    apparel.append(
        MagicItem(
            name="Avian Cloak",
            level=9,
            material_type="Cloak",
            tags=[],
            description="""
            You gain a \\glossterm<glide speed> equal to the \\glossterm<base speed> for your size.
        """,
            short_description="Grants a glide speed",
        )
    )

    apparel += [
        MagicItem(
            name="Assassin's Cloak",
            level=8,
            material_type="Cloak",
            tags=["Sensation"],
            description="""
                At the end of each round, if you took no actions that round, you become \\glossterm<invisible>.
                This invisibility ends after you take any action.
            """,
            short_description="Grants invisibility while inactive",
        ),
        MagicItem(
            name="Assassin's Cloak, Greater",
            level=17,
            material_type="Cloak",
            tags=["Sensation"],
            description="""
                At the end of each round, if you took no actions that round, you \\glossterm{briefly} become \\glossterm<invisible>.
            """,
            short_description="Grants longer invisibility while inactive",
        ),
    ]

    apparel += [
        MagicItem(
            name="Cloak of Mist",
            level=8,
            material_type="Cloak",
            tags=["Manifestation"],
            description="""
                Fog constantly fills a \\areamed radius emanation from you.

                If a 5-foot square of fog takes at least 4 fire damage from a single attack, the fog \\glossterm{briefly} disappears from that area.
                This fog does not fully block sight, but it provides \\glossterm<concealment>.
            """,
            short_description="Fills nearby area with fog",
        ),
        MagicItem(
            name="Cloak of Mist, Greater",
            level=16,
            material_type="Cloak",
            tags=["Manifestation"],
            description="""
                A thick fog constantly fills a \\areamed radius emanation from you.
                This fog completely blocks sight beyond 10 feet.

                If a 5-foot square of fog takes at least 16 fire damage from a single attack, the fog \\glossterm{briefly} disappears from that area.
            """,
            short_description="Fills nearby area with thick fog",
        ),
    ]

    apparel.append(
        MagicItem(
            name="Vanishing Cloak",
            level=13,
            material_type="Cloak",
            tags=["Sensation"],
            description="""
            As a standard action, you can activate this cloak.
            When you do, you teleport to an unoccupied location within \\rngmed range of your original location.
            In addition, you become \\glossterm{briefly} \\glossterm<invisible>.

            If your intended destination is invalid, or if your teleportation otherwise fails, you still become invisible.
        """,
            short_description="Can teleport a short distance and grant invisibility",
        )
    )

    apparel += [
        MagicItem(
            # Maybe too strong?
            name="Hexward Ring",
            level=7,
            material_type="Ring",
            tags=[],
            description="""
                You gain a +1 bonus to defenses against spells that target you directly.
                This does not protect you from abilities that affect an area.
            """,
            short_description="Grants +1 defenses against targeted spells",
        ),
        MagicItem(
            name="Hexproof Ring, Greater",
            level=16,
            material_type="Ring",
            tags=[],
            description="""
                This item functions like a \\mitem<hexward ring>, except that the bonus increases to +2.
            """,
            short_description="Grants +2 defenses against targeted spells",
        ),
    ]

    # Belts

    apparel += [
        MagicItem(
            name="Belt of Healing",
            level=4,
            material_type="Belt",
            tags=['Healing'],
            description="""
                As a standard action, you can use this belt to regain 1d10+2 hit points.
                After you use this ability, you \\glossterm{briefly} cannot use it or any other \\abilitytag{Healing} ability.
            """,
            short_description="Grants healing",
        ),
        MagicItem(
            name="Belt of Healing, Greater",
            level=10,
            material_type="Belt",
            tags=['Healing'],
            description="""
                As a standard action, you can use this belt to regain 2d10+5 hit points.
                After you use this ability, you \\glossterm{briefly} cannot use it or any other \\abilitytag{Healing} ability.
            """,
            short_description="Grants more healing",
        ),
        MagicItem(
            name="Belt of Healing, Supreme",
            level=16,
            material_type="Belt",
            tags=['Healing'],
            description="""
                As a standard action, you can use this belt to regain 4d10+8 hit points.
                After you use this ability, you \\glossterm{briefly} cannot use it or any other \\abilitytag{Healing} ability.
            """,
            short_description="Grants more healing",
        ),
    ]

    apparel += [
        MagicItem(
            name="Enlarging Belt",
            level=10,
            material_type="Belt",
            tags=[],
            description="""
                As a standard action, you can activate this belt.
                If you do, your size increases by one \\glossterm<size category>, to a maximum of Huge.
                This increases the \\glossterm<base speed> for your size and reduces your Stealth skill.
                It may also increase your \\glossterm<reach> (see \\pcref<Size in Combat>).

                Your physical form is not altered fully to match your new size.
                Your Strength and Dexterity are unchanged from your original size, and you take a -5 foot penalty to your speed with all of your \\glossterm{movement modes.}
                This effect lasts until you activate the belt again, which returns you to your original size.
            """,
            short_description="Increases your size",
        ),
        MagicItem(
            name="Enlarging Belt, Greater",
            level=16,
            material_type="Belt",
            tags=[],
            description="""
                This belt functions like an \\mitem<enlarging belt>, except that you also gain a +2 \\glossterm<magic bonus> to Strength-based checks, and you gain a +2 \\glossterm<magic bonus> to Strength for the purpose of determining your weight limits (see \\pcref{Weight Limits}).
            """,
            short_description="Increases your size and strength",
        ),
    ]

    apparel += [
        MagicItem(
            name="Shrinking Belt",
            level=7,
            material_type="Belt",
            tags=[],
            description="""
                As a standard action, you can activate this belt.
                If you do, your size decreases by one \\glossterm<size category>, to a minimum of Tiny.
                This decreases the \\glossterm<base speed> for your size and improves your Stealth skill.
                It may also decrease your \\glossterm<reach> (see \\pcref<Size in Combat>).
                This effect lasts until you activate the belt again, which returns you to your original size.
            """,
            short_description="Reduces your size",
        ),
        MagicItem(
            name="Shrinking Belt, Greater",
            level=13,
            material_type="Belt",
            tags=[],
            description="""
                This belt functions like a \\mitem<shrinking belt>, except that your size decreases by two size categories instead of one.
            """,
            short_description="Greatly reduces your size",
        ),
    ]

    apparel += [
        MagicItem(
            name="Utility Belt",
            level=6,
            material_type="Belt",
            tags=[],
            description="""
                This belt contains five pockets, each of which is larger on the inside than the outside.
                The inside of each pocket is a six inch cube.
                You can put anything you want in each pocket, but you still carry the weight of anything in the pockets.
                If you put reactive objects in a pocket, such as acid or burning alchemist's fire, it may destroy the pocket until the belt is repaired.

                As long as each pocket is no more than half full, or is full of completely interchangeable items, you can reach into any pocket just as easily as you can reach into a nonmagical pocket.
                Overstuffed pockets may take more time to sift through to find the specific item you want, just like rummaging through a backpack.

                If you take off this belt or stop attuning to it, the items in the belt become inaccessible.
                If this belt is destroyed, the items within it become lost in the Astral Plane.
            """,
            short_description="Contains five large pockets",
        ),
        MagicItem(
            name="Utility Belt, Greater",
            level=13,
            material_type="Belt",
            tags=[],
            description="""
                This belt functions like a \\mitem<utility belt>, except that the belt has ten pockets, each of which is a one foot cube on the inside.
            """,
            short_description="Contains ten very large pcokets",
        ),
    ]

    apparel += [
        MagicItem(
            name="Frenzied Gloves",
            level=9,
            material_type="Gloves",
            tags=[],
            description="""
                Whenever you make a \\glossterm<strike>, you \\glossterm<briefly> gain a +1 bonus to \\glossterm<accuracy> with \\glossterm<strikes>.
                As normal, this bonus does not stack with itself.
            """,
            short_description="Grants +1 accuracy to continuous strikes",
        ),
        MagicItem(
            name="Frenzied Gloves, Greater",
            level=15,
            material_type="Gloves",
            tags=[],
            description="""
                Whenever you make a \\glossterm<strike>, you \\glossterm<briefly> gain a +2 bonus to \\glossterm<accuracy> with \\glossterm<strikes>.
                As normal, this bonus does not stack with itself.
            """,
            short_description="Grants +2 accuracy to continuous strikes",
        ),
    ]

    apparel += [
        MagicItem(
            name="Gloves of Infused Force",
            level=6,
            material_type="Gloves",
            tags=[],
            description="""
                As a standard action, you can activate these gloves to infuse them with power.
                When you hit with a \\glossterm<strike> while these gloves are infused, you gain a +4 bonus to \\glossterm<power> with the strike and the gloves stop being infused.
            """,
            short_description="Grants +4 power to next strike",
        ),
        MagicItem(
            name="Gloves of Infused Force, Greater",
            level=12,
            material_type="Gloves",
            tags=[],
            description="""
                These gloves function like \\mitem<gloves of infused force>, except that the power bonus increases to +8.
            """,
            short_description="Grants +8 power to next strike",
        ),
        MagicItem(
            name="Gloves of Infused Force, Supreme",
            level=18,
            material_type="Gloves",
            tags=[],
            description="""
                These gloves function like \\mitem<gloves of infused force>, except that the power bonus increases to +16.
            """,
            short_description="Grants +16 power to next strike",
        ),
    ]

    apparel += [
        MagicItem(
            name="Belt of Vital Persistence",
            level=1,
            material_type="Belt",
            tags=[],
            description="""
                When you make a \\glossterm<vital roll>, you can activate this item.
                When you do, you increase your \\glossterm<fatigue level> by one, and you gain a +1 bonus to the vital roll.
                You can use this ability after you see the result of the vital roll.
            """,
            short_description="Can grant +1 to a vital roll",
        ),
        MagicItem(
            name="Belt of Vital Persistence, Greater",
            level=7,
            material_type="Belt",
            tags=[],
            description="""
                This item functions like a \\textit<belt of vital persistence>, except that the bonus it grants increases to +2.
            """,
            short_description="Can grant +2 to a vital roll",
        ),
        MagicItem(
            name="Belt of Vital Persistence, Supreme",
            level=13,
            material_type="Belt",
            tags=[],
            description="""
                This item functions like a \\textit<belt of vital persistence>, except that the bonus it grants increases to +3.
            """,
            short_description="Can grant +3 to a vital roll",
        ),
    ]

    apparel += [
        MagicItem(
            name="Lifekeeping Belt",
            level=7,
            material_type="Belt",
            tags=[],
            description="""
                You gain a +1 \\glossterm<magic bonus> to \\glossterm<vital rolls>.
            """,
            short_description="Grants +1 bonus to \\glossterm<vital rolls>",
        ),
        MagicItem(
            name="Lifekeeping Belt, Greater",
            level=13,
            material_type="Belt",
            tags=[],
            description="""
                You gain a +2 \\glossterm<magic bonus> to \\glossterm<vital rolls>.
            """,
            short_description="Grants +2 bonus to \\glossterm<vital rolls>",
        ),
        MagicItem(
            name="Lifekeeping Belt, Supreme",
            level=19,
            material_type="Belt",
            tags=[],
            description="""
                You gain a +3 \\glossterm<magic bonus> to \\glossterm<vital rolls>.
            """,
            short_description="Grants +3 bonus to \\glossterm<vital rolls>",
        ),
    ]

    apparel += [
        MagicItem(
            name="Ocular Circlet",
            level=3,
            material_type="Circlet",
            tags=["Scrying"],
            description="""
                As a \\glossterm<standard action>, you can concentrate to use this item.
                If you do, a \\glossterm<scrying sensor> appears floating in the air in an unoccupied square within \\rngshort range.
                As long as you \\glossterm<sustain> the effect as a standard action, you see through the sensor instead of from your body.

                While viewing through the sensor, your visual acuity is the same as your normal body,
                    except that it does not share the benefits of any \\glossterm<magical> effects that improve your vision.
                You otherwise act normally, though you may have difficulty moving or taking actions if the sensor cannot see your body or your intended targets, effectively making you \\blinded.
            """,
            short_description="Can allow you to see at a distance",
        ),
        MagicItem(
            name="Ocular Circlet, Greater",
            level=9,
            material_type="Circlet",
            tags=["Scrying"],
            description="""
                This item functions like a \\mitem<ocular circlet>, except that it only takes a \\glossterm<minor action> to activate and sustain the item's effect.
                In addition, the sensor appears anywhere within \\rngmed range.
            """,
            short_description="Can allow you to see at a greater distance",
        ),
    ]

    apparel += [
        MagicItem(
            name="Ring of Spell Investment",
            level=5,
            material_type="Ring",
            tags=[],
            description="""
                When you or an adjacent ally casts a spell that does not have the \\abilitytag<Attune> or \\abilitytag<Sustain> tags,
                    you can invest the magic of the spell in the ring.
                If you do, the spell does not have its normal effect.
                All decisions about the spell's effect must be made at the time that the spell is invested in this way.
                Only one spell can be stored this way.

                As a standard action, you can activate this ring.
                When you do, you cause the effect of the last spell invested in the ring.
                This does not require \\glossterm<casting components> and does not have the \\abilitytag<Focus> tag, even if casting the spell normally would have those limitations.
                The spell's effect is determined based on the \\glossterm<power> and other abilities of the original caster who invested the spell into the ring, not yours.
                You do not have to have the ability to cast the spell to activate a spell in this way.
                The \\textit<desperate exertion> ability cannot be used to affect the spell, either at the time it is invested or when it is activated.
                After you use a spell in this way, the energy in the ring is spent, and you must invest a new spell to activate the ring again.
            """,
            short_description="Can invest a spell to gain its effect later",
        ),
        MagicItem(
            name="Ring of Spell Investment, Greater",
            level=11,
            material_type="Ring",
            tags=[],
            description="""
                This item functions like a \\mitem<ring of spell investment>, except that you can store up to three spells in the gloves.
                When you activate the ring, you choose which spell to use.
            """,
            short_description="Can invest three spells to gain their effects later",
        ),
    ]

    apparel += [
        MagicItem(
            name="Belt of Hill Giant's Strength",
            level=8,
            material_type="Belt",
            tags=[],
            description="""
                You gain a +1 bonus to Strength-based \\glossterm<checks>, and you gain a +1 bonus to Strength for the purpose of determining your \\glossterm<carrying capacity> (see \\pcref<Carrying Capacity>).
                In addition, you reduce your \\glossterm<encumbrance> by 1.
            """,
            short_description="Grants +1 Strength for specific purposes",
        ),
        MagicItem(
            name="Belt of Stone Giant's Strength",
            level=14,
            material_type="Belt",
            tags=[],
            description="""
                You gain a +2 bonus to Strength-based \\glossterm<checks>, and you gain a +2 bonus to Strength for the purpose of determining your \\glossterm<carrying capacity> (see \\pcref<Carrying Capacity>).
                In addition, you reduce your \\glossterm<encumbrance> by 2.
            """,
            short_description="Grants +2 Strength for specific purposes",
        ),
        MagicItem(
            name="Belt of Storm Giant's Strength",
            level=20,
            material_type="Belt",
            tags=[],
            description="""
                You gain a +3 bonus to Strength-based \\glossterm<checks>, and you gain a +3 bonus to Strength for the purpose of determining your \\glossterm<carrying capacity> (see \\pcref<Carrying Capacity>).
                In addition, you reduce your \\glossterm<encumbrance> by 3.
            """,
            short_description="Grants +3 Strength for specific purposes",
        ),
    ]

    apparel += [
        MagicItem(
            name="Cloak of Translocation",
            level=5,
            material_type="Cloak",
            tags=[],
            description="""
                As a standard action, you can \\glossterm<teleport> yourself into an unoccupied location within \\rngshort range on a stable surface that can support your weight.
                If the destination is invalid, this ability has no effect.
            """,
            short_description="Can teleport up to 30 feet",
        ),
        MagicItem(
            name="Cloak of Translocation, Greater",
            level=11,
            material_type="Cloak",
            tags=[],
            description="""
                This cloak functions like a \\mitem<cloak of translocation>, except that the range increases to \\rngmed.
            """,
            short_description="Can teleport up to 60 feet",
        ),
        MagicItem(
            name="Cloak of Translocation, Supreme",
            level=17,
            material_type="Cloak",
            tags=[],
            description="""
                This cloak functions like a \\mitem<cloak of translocation>, except that the range increases to \\rnglong.
            """,
            short_description="Can teleport up to 120 feet",
        ),
    ]

    apparel += [
        MagicItem(
            name="Boots of Reliable Motion",
            level=2,
            material_type="Boots",
            tags=[],
            description="""
                Whenever you roll a 1 on an attack or check using the Balance, Climb, Jump, or Swim skills, you may reroll and take the higher result.
                You can only reroll any individual roll once in this way.
            """,
            short_description="Can reroll 1s with movement-based skills",
        ),
        MagicItem(
            name="Boots of Reliable Motion, Greater",
            level=8,
            material_type="Boots",
            tags=[],
            description="""
                Whenever you roll a 1 on an attack or check using the Balance, Climb, Jump, or Swim skills, you may reroll and take the higher result.
                You can only reroll any individual roll once in this way.
                In addition, using the \\textit<desperate exertion> ability to affect those skills only causes you to increase your \\glossterm<fatigue level> by one instead of two (see \\pcref<desperate exertion>).
            """,
            short_description="Can reroll 1s and exert more easily with movement-based skills",
        ),
        MagicItem(
            name="Boots of Reliable Motion, Supreme",
            level=14,
            material_type="Boots",
            tags=[],
            description="""
                Whenever you roll a 1 or 2 on an attack or check using the Balance, Climb, Jump, or Swim skills, you may reroll and take the higher result.
                You can only reroll any individual roll once in this way.
                In addition, using the \\textit<desperate exertion> ability to affect those skills only causes you to increase your \\glossterm<fatigue level> by one instead of two (see \\pcref<desperate exertion>).
            """,
            short_description="Can reroll 1s and 2s and exert more easily with movement-based skills",
        ),
        MagicItem(
            name="Gloves of Reliable Finesse",
            level=2,
            material_type="Gloves",
            tags=[],
            description="""
                Whenever you roll a 1 on an attack or check using the Craft, Devices, Flexibility, or Sleight of Hand skills, you may reroll and take the higher result.
                You can only reroll any individual roll once in this way.
            """,
            short_description="Can reroll 1s with finesse-based skills",
        ),
        MagicItem(
            name="Gloves of Reliable Finesse, Greater",
            level=8,
            material_type="Gloves",
            tags=[],
            description="""
                Whenever you roll a 1 on an attack or check using the Craft, Devices, Flexibility, or Sleight of Hand skills, you may reroll and take the higher result.
                You can only reroll any individual roll once in this way.
                In addition, using the \\textit<desperate exertion> ability to affect those skills only causes you to increase your \\glossterm<fatigue level> by one instead of two (see \\pcref<desperate exertion>).
            """,
            short_description="Can reroll 1s and exert more easily with finesse-based skills",
        ),
        MagicItem(
            name="Gloves of Reliable Finesse, Supreme",
            level=14,
            material_type="Gloves",
            tags=[],
            description="""
                Whenever you roll a 1 or 2 on an attack or check using the Craft, Devices, Medicine, or Sleight of Hand skills, you may reroll and take the higher result.
                You can only reroll any individual roll once in this way.
                In addition, using the \\textit<desperate exertion> ability to affect those skills only causes you to increase your \\glossterm<fatigue level> by one instead of two (see \\pcref<desperate exertion>).
            """,
            short_description="Can reroll 1s and 2s and exert more easily with finesse-based skills",
        ),
        MagicItem(
            name="Circlet of Reliable Observation",
            level=2,
            material_type="Circlet",
            tags=[],
            description="""
                Whenever you roll a 1 on an attack or check using the Awareness, Deduction, Social Insight, or Spellsense skills, you may reroll and take the higher result.
                You can only reroll any individual roll once in this way.
            """,
            short_description="Can reroll 1s with observation-based skills",
        ),
        MagicItem(
            name="Circlet of Reliable Observation, Greater",
            level=8,
            material_type="Circlet",
            tags=[],
            description="""
                Whenever you roll a 1 on an attack or check using the Awareness, Deduction, Social Insight, or Spellsense skills, you may reroll and take the higher result.
                You can only reroll any individual roll once in this way.
                In addition, using the \\textit<desperate exertion> ability to affect those skills only causes you to increase your \\glossterm<fatigue level> by one instead of two (see \\pcref<desperate exertion>).
            """,
            short_description="Can reroll 1s and exert more easily with observation-based skills",
        ),
        MagicItem(
            name="Circlet of Reliable Observation, Supreme",
            level=14,
            material_type="Circlet",
            tags=[],
            description="""
                Whenever you roll a 1 or 2 on an attack or check using the Awareness, Deduction, Social Insight, or Spellsense skills, you may reroll and take the higher result.
                You can only reroll any individual roll once in this way.
                In addition, using the \\textit<desperate exertion> ability to affect those skills only causes you to increase your \\glossterm<fatigue level> by one instead of two (see \\pcref<desperate exertion>).
            """,
            short_description="Can reroll 1s and 2s and exert more easily with observation-based skills",
        ),
    ]

    apparel += [
        MagicItem(
            name="Blindfold of the Third Eye",
            # Blindsight is a rank 2 self-only spell, so this would be level 7
            # if it followed that model normally. It gets +2 levels for also
            # granting blindsense and -1 level for requiring blindness.
            level=8,
            material_type="Fabric",
            tags=[],
            description="""
                While you wear this blindfold covering your eyes, you gain \\glossterm<blindsight> with a 30 foot range and \\glossterm<blindsense> with a 120 foot range.
                You are also blind, as normal for wearing a blindfold.
                Shifting this blindfold to cover or stop covering your eyes is a \\glossterm<free action> that requires a \\glossterm<free hand>.
            """,
            short_description="Grants blindsight and blindsense",
        ),
        MagicItem(
            name="Blindfold of the Third Eye, Greater",
            level=14,
            material_type="Fabric",
            tags=[],
            description="""
                This blindfold functions like the \\mitem<blindfold of the third eye>, except that the range of the blindsight increases to 60 feet and the range of the blindsense increases to 240 feet.
            """,
            short_description="Grants distant blindsight and blindsense",
        ),
        MagicItem(
            name="Blindfold of the Third Eye, Supreme",
            level=20,
            material_type="Fabric",
            tags=[],
            description="""
                This blindfold functions like the \\mitem<blindfold of the third eye>, except that the range of the blindsight increases to 120 feet and the range of the blindsense increases to 480 feet.
            """,
            short_description="Grants very distant blindsight and blindsense",
        ),
    ]

    apparel += [
        MagicItem(
            name="Charging Boots",
            level=4,
            material_type="Boots",
            tags=[],
            description="""
                You reduce your defense penalties from using the \\ability<charge> action by 1.
            """,
            short_description="Reduces penalties for charging by 1",
        ),
        MagicItem(
            name="Greater Charging Boots",
            level=10,
            material_type="Boots",
            tags=[],
            description="""
                You do not take defense penalties from using the \\ability<charge> action.
            """,
            short_description="Removes penalties for charging",
        ),
    ]

    apparel += [
        MagicItem(
            name="Cleansing Belt",
            level=4,
            material_type="Belt",
            tags=[],
            description="""
                As a standard action, you can activate this belt.
                When you do, you remove one \\glossterm<brief> effect or \\glossterm<condition> affecting you.
                This cannot remove an effect applied during the current round.

                After you use this ability, you increase your \\glossterm<fatigue level> by one.
            """,
            short_description="Fatigue to remove a debuff",
        ),
        MagicItem(
            name="Greater Cleansing Belt",
            level=10,
            material_type="Belt",
            tags=[],
            description="""
                As a standard action, you can activate this belt.
                When you do, you remove one \\glossterm<brief> effect or \\glossterm<condition> affecting you.
                This cannot remove an effect applied during the current round.
            """,
            short_description="Remove a debuff",
        ),
        MagicItem(
            name="Supreme Cleansing Belt",
            level=16,
            material_type="Belt",
            tags=[],
            description="""
                As a standard action, you can activate this belt.
                When you do, you remove up to two \\glossterm<brief> effects or \\glossterm<conditions> affecting you.
                This cannot remove an effects applied during the current round.
            """,
            short_description="Freely remove two debuffs",
        ),
    ]

    apparel += [
        MagicItem(
            name="Quickcleanse Belt",
            level=13,
            material_type="Belt",
            tags=[],
            description="""
                As a \\glossterm<minor action>, you can activate this belt.
                When you do, you remove one \\glossterm<brief> effect or \\glossterm<condition> affecting you.
                This cannot remove an effect applied during the current round.

                After you use this belt, you increase your \\glossterm<fatigue level> by two.
            """,
            short_description="Quickly remove a debuff",
        ),

        MagicItem(
            name="Quickcleanse Belt, Greater",
            level=19,
            material_type="Belt",
            tags=[],
            description="""
                This item functions like a \\mitem<quickcleanse belt>, except that you only increase your \\glossterm<fatigue level> by one instead of two.
            """,
            short_description="Quickly remove a debuff more easily",
        ),
    ]

    apparel += [
        MagicItem(
            name="Sprinting Boots",
            level=14,
            material_type="Boots",
            tags=[],
            description="""
                You can use these boots when you take the \\textit<sprint> action to avoid increasing your\\glossterm<fatigue level> (see \\pcref<Sprint>).
                After you use this boots in this way, you \\glossterm{briefly} cannot use them again.
            """,
            short_description="Can sprint more easily",
        ),
    ]

    apparel += [
        MagicItem(
            name="Cloak of Transportation",
            level=11,
            material_type="Cloak",
            tags=[],
            description="""
                All \\glossterm<magical> abilities that \\glossterm<teleport> you have the maximum distance they can teleport you doubled.
            """,
            short_description="Doubles distance you can teleport",
        ),
        MagicItem(
            name="Cloak of Transportation, Greater",
            level=17,
            material_type="Cloak",
            tags=[],
            description="""
                All \\glossterm<magical> abilities that \\glossterm<teleport> you have the maximum distance they can teleport you tripled.
            """,
            short_description="Triples distance you can teleport",
        ),
    ]

    apparel += [
        MagicItem(
            name="Boots of Desperate Flight",
            level=6,
            material_type="Boots",
            tags=[],
            description="""
                When you use the \\textit<recover> action, you can also move up to your normal movement speed.
            """,
            short_description="Can move when you recover",
        ),
        MagicItem(
            name="Boots of Desperate Flight, Greater",
            level=12,
            material_type="Boots",
            tags=[],
            description="""
                When you use the \\textit<recover> action, you can also move up to twice your normal movement speed.
            """,
            short_description="Can sprint when you recover",
        ),
    ]

    apparel += [
        MagicItem(
            name="Ring of Mastery",
            level=10,
            material_type="Ring",
            tags=[],
            description="""
                You gain a +4 \\glossterm<magic bonus> to \\glossterm<hit points> and \\glossterm<damage resistance>.
                In addition, you gain a \\plus2 \\glossterm<magic bonus> to \\glossterm<power>.
            """,
            short_description="Grants many small bonuses",
        ),
        MagicItem(
            name="Ring of Mastery, Greater",
            level=16,
            material_type="Ring",
            tags=[],
            description="""
                You gain a +8 \\glossterm<magic bonus> to \\glossterm<hit points> and \\glossterm<damage resistance>.
                In addition, you gain a \\plus4 \\glossterm<magic bonus> to \\glossterm<power>.
            """,
            short_description="Grants many bonuses",
        ),
    ]

    apparel += [
        MagicItem(
            name="Ring of Protection",
            # +2 level since ring is secondary for this effect
            level=6,
            tags=[],
            material_type="Ring",
            description="""
                You gain a +4 \\glossterm<magic bonus> to \\glossterm<damage resistance>.
            """,
            short_description="Grants +4 damage resistance",
        ),
        MagicItem(
            name="Ring of Protection, Greater",
            level=12,
            tags=[],
            material_type="Ring",
            description="""
                You gain a +8 \\glossterm<magic bonus> to \\glossterm<damage resistance>.
            """,
            short_description="Grants +8 damage resistance",
        ),
        MagicItem(
            name="Ring of Protection, Supreme",
            level=18,
            tags=[],
            material_type="Ring",
            description="""
                You gain a +16 \\glossterm<magic bonus> to \\glossterm<damage resistance>.
            """,
            short_description="Grants +16 damage resistance",
        ),
    ]

    apparel += [
        MagicItem(
            name="Anchoring Belt",
            level=5,
            material_type="Belt",
            tags=[],
            description="""
                You are immune to \\glossterm<knockback> or \\glossterm<push> effects from attacks, unless the effects come from an attack that scores a \\glossterm<critical hit>.
                This does not make you immune to \\glossterm<teleportation>, and does not affect movement effects used by your \\glossterm<allies>.
            """,
            short_description="Protects you from most forced movement attacks",
        ),
        MagicItem(
            name="Anchoring Belt, Greater",
            level=11,
            material_type="Belt",
            tags=[],
            description="""
                This belt functions like an \\mitem<anchoring belt>, except that you are also immune to \\glossterm<teleport> effects from attacks that are not critical hits.
            """,
            short_description="Protects you from most forced movement and teleportation attacks",
        ),
        MagicItem(
            name="Anchoring Belt, Supreme",
            level=17,
            material_type="Belt",
            tags=[],
            description="""
                This belt functions like an \\mitem<anchoring belt>, except that the immunities apply even against critical hits.
            """,
            short_description="Protects you from all forced movement and teleportation attacks",
        ),
    ]

    # Class ability buffs - all amulets!

    apparel += [
        MagicItem(
            name="Amulet of Undead Turning",
            level=7,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<turn undead> cleric ability, its area increases to a \\largearea radius.
            """,
            short_description="Increases area of \\ability<turn undead>",
        ),
        MagicItem(
            name="Amulet of Undead Turning, Greater",
            level=13,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<turn undead> cleric ability, its area increases to a \\hugearea radius.
            """,
            short_description="Greatly increases area of \\ability<turn undead>",
        ),
        MagicItem(
            name="Amulet of Undead Turning, Supreme",
            level=19,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<turn undead> cleric ability, its area increases to a \\gargarea radius.
            """,
            short_description="Massively increases area of \\ability<turn undead>",
        ),
    ]

    apparel += [
        MagicItem(
            name="Amulet of Rage",
            level=10,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<rage> barbarian ability, its penalties to Armor and Reflex defense are reduced by 1.
            """,
            short_description="Reduces defense penalties from \\textit<rage>.",
        ),
        MagicItem(
            name="Amulet of Rage, Greater",
            level=19,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<rage> barbarian ability, its penalties to Armor and Reflex defense are removed.
            """,
            short_description="Removes defense penalties from \\textit<rage>.",
        ),
    ]

    apparel += [
        MagicItem(
            name="Amulet of Divine Healing",
            level=4,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<restoration> cleric ability or the \\ability<lay on hands> paladin ability, you gain a +4 \\glossterm<magic bonus> to your \\glossterm<power>.
            """,
            short_description="Grants +4 power with \\ability<restoration> and \\ability<lay on hands>",
        ),
        MagicItem(
            name="Amulet of Divine Healing, Greater",
            level=10,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<restoration> cleric ability or the \\ability<lay on hands> paladin ability, you gain a +8 \\glossterm<magic bonus> to your \\glossterm<power>.
            """,
            short_description="Grants +8 power with \\ability<restoration> and \\ability<lay on hands>",
        ),
        MagicItem(
            name="Amulet of Divine Healing, Supreme",
            level=16,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<restoration> cleric ability or the \\ability<lay on hands> paladin ability, you gain a +16 \\glossterm<magic bonus> to your \\glossterm<power>.
            """,
            short_description="Grants +16 power with \\ability<restoration> and \\ability<lay on hands>",
        ),
    ]

    apparel += [
        MagicItem(
            name="Amulet of Divine Vitality",
            level=11,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<divine healing> cleric ability or the \\ability<lay on hands> paladin ability to remove a vital wound, you only increase your \\glossterm<fatigue level> by one instead of by two.
            """,
            short_description="Reduces fatigue required to use \\ability<divine healing>",
        ),
    ]

    apparel += [
        MagicItem(
            name="Amulet of Revivification",
            level=20,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<revivify> cleric ability, you only increase your \\glossterm<fatigue level> by two instead of by three.
            """,
            short_description="Reduces fatigue required to use \\ability<revivify>",
        ),
    ]

    apparel += [
        MagicItem(
            name="Quickshift Amulet",
            level=16,
            material_type="Amulet",
            tags=[],
            description="""
                If you have the \\ability<shifting defense> druid ability, you can use it as a \\glossterm<minor action> instead of as a standard action.
            """,
            short_description="Allows using \\ability<shifting defense> more quickly",
        ),
    ]

    apparel += [
        MagicItem(
            name="Amulet of Shared Discipline",
            level=11,
            material_type="Amulet",
            tags=[],
            description="""
                Whenever you use the \\ability<discipline> fighter ability, one \\glossterm<ally> adjacent to you can also remove a \\glossterm<brief> effect or \\glossterm<condition>.
                This is not a \\abilitytag<Swift> effect, even if your \\ability<discipline> ability is a \\abilitytag<Swift> effect.
            """,
            short_description="Using \\abilitytag<discipline> also helps an adjacent ally",
        ),
        MagicItem(
            name="Amulet of Shared Discipline, Greater",
            level=17,
            material_type="Amulet",
            tags=[],
            description="""
                This item functions like a \\textit<amulet of shared discipline>, except that the range increases to \\medrange.
            """,
            short_description="Using the \\abilitytag<discipline> ability also helps a nearby ally",
        ),
    ]

    apparel += [
        MagicItem(
            name="Amulet of Shared Clarity",
            level=8,
            material_type="Amulet",
            tags=[],
            description="""
                Whenever you use the \\ability<clear the mind> monk ability, one \\glossterm<ally> within \\medrange of you can also remove a \\glossterm<brief> effect or \\glossterm<condition>.
                If they do, they increase their \\glossterm<fatigue level> by one.
                This is not a \\abilitytag<Swift> effect, even if your \\ability<clear the mind> ability is a \\abilitytag<Swift> effect.
            """,
            short_description="Using \\abilitytag<clear the mind> also helps a nearby ally",
        ),
        MagicItem(
            name="Amulet of Shared Clarity, Greater",
            level=14,
            material_type="Amulet",
            tags=[],
            description="""
                This item functions like a \\textit<amulet of shared clarity>, except that you can target an additional \\glossterm<ally> within range.
            """,
            short_description="Using \\abilitytag<clear the mind> also helps two nearby allies",
        ),
    ]

    apparel += [
        MagicItem(
            name="Guardian's Amulet",
            level=7,
            material_type="Amulet",
            tags=[],
            description="""
                You gain a +1 \\glossterm<accuracy> bonus with the \\ability<guarding strike> fighter ability.
            """,
            short_description="Grants +1 accuracy with \\ability<guarding strike>",
        ),
        MagicItem(
            name="Guardian's Amulet, Greater",
            level=16,
            material_type="Amulet",
            tags=[],
            description="""
                You gain a +2 \\glossterm<accuracy> bonus with the \\ability<guarding strike> fighter ability.
            """,
            short_description="Grants +2 accuracy with \\ability<guarding strike>",
        ),
    ]

    apparel += [
        MagicItem(
            name="Protector's Amulet",
            level=5,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<protect> fighter ability, you can target an ally within \\shortrange of you.
            """,
            short_description="Increases range of the \\ability<protect> ability",
        ),
        MagicItem(
            name="Protector's Amulet, Greater",
            level=11,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<protect> fighter ability, you can target an ally within \\medrange of you.
            """,
            short_description="Greatly increases range of the \\ability<protect> ability",
        ),
        MagicItem(
            name="Protector's Amulet, Supreme",
            level=17,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<protect> fighter ability, you can target an ally within \\longrange of you.
            """,
            short_description="Drastically increases range of the \\ability<protect> ability",
        ),
    ]

    apparel += [
        MagicItem(
            name="Amulet of Sturdy Companionship",
            level=4,
            material_type="Amulet",
            tags=[],
            description="""
                Each creature you command with the \\ability<natural servant> druid ability or the \\ability<animal companion> ranger ability gains a +8 \\glossterm<magic bonus> to its \\glossterm<damage resistance>.
            """,
            short_description="Grants +8 damage resistance to animal allies",
        ),
        MagicItem(
            name="Amulet of Sturdy Companionship, Greater",
            level=10,
            material_type="Amulet",
            tags=[],
            description="""
                Each creature you command with the \\ability<natural servant> druid ability or the \\ability<animal companion> ranger ability gains a +16 \\glossterm<magic bonus> to its \\glossterm<damage resistance>.
            """,
            short_description="Grants +16 damage resistance to animal allies",
        ),
        MagicItem(
            name="Amulet of Sturdy Companionship, Supreme",
            level=16,
            material_type="Amulet",
            tags=[],
            description="""
                Each creature you command with the \\ability<natural servant> druid ability or the \\ability<animal companion> ranger ability gains a +32 \\glossterm<magic bonus> to its \\glossterm<damage resistance>.
            """,
            short_description="Grants +32 damage resistance to animal allies",
        ),
    ]

    apparel += [
        MagicItem(
            name="Amulet of Equality and Perfection",
            level=20,
            material_type="Amulet",
            tags=[],
            description="""
                If you have the \\ability<perfect body> ability, you gain a +1 bonus to the base value of your lowest physical attribute.
            """,
            short_description="Improves \\ability<perfect body> on your lowest attribute",
        ),
    ]

    apparel += [
        MagicItem(
            name="Amulet of Equality and Perfection",
            level=20,
            material_type="Amulet",
            tags=[],
            description="""
                If you have the \\ability<perfect body> ability, you gain a +1 bonus to the base value of your lowest physical attribute.
            """,
            short_description="Improves \\ability<perfect body> on your lowest attribute",
        ),
    ]

    apparel += [
        MagicItem(
            name="Amulet of the Avenger",
            level=8,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<smite> paladin ability, you gain a +1 \\glossterm<accuracy> bonus against each creature that dealt damage to one of your \\glossterm<allies> during the previous round.
            """,
            short_description="Grants +1 accuracy with \\ability<smite> when avenging allies",
        ),
        MagicItem(
            name="Amulet of the Avenger, Greater",
            level=17,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<smite> paladin ability, you gain a +2 \\glossterm<accuracy> bonus against each creature that dealt damage to one of your \\glossterm<allies> during the previous round.
            """,
            short_description="Grants +2 accuracy with \\ability<smite> when avenging allies",
        ),
    ]

    apparel += [
        MagicItem(
            name="Swarmhunter Amulet",
            level=11,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<quarry> ranger ability, you may target an additional creature.
            """,
            short_description="Adds an additional target with \\ability<quarry>",
        ),
        MagicItem(
            name="Swarmhunter Amulet, Greater",
            level=11,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<quarry> ranger ability, you may target two additional creatures.
            """,
            short_description="Adds two additional targets with \\ability<quarry>",
        ),
    ]

    apparel += [
        MagicItem(
            name="Amulet of Distant Stealth",
            level=4,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<sneak attack> rogue ability, you may target a creature within \\medrange instead of \\shortrange.
            """,
            short_description="Increases range with \\ability<sneak attack>",
        ),
        MagicItem(
            name="Amulet of Distant Stealth, Greater",
            level=10,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<sneak attack> rogue ability, you may target a creature within \\longrange instead of \\shortrange.
            """,
            short_description="Greatly increases range with \\ability<sneak attack>",
        ),
        MagicItem(
            name="Amulet of Distant Stealth, Supreme",
            level=16,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<sneak attack> rogue ability, you may target a creature within \\distrange instead of \\shortrange.
            """,
            short_description="Drastically increases range with \\ability<sneak attack>",
        ),
    ]

    apparel += [
        MagicItem(
            name="Darkstalker's Amulet",
            level=11,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<darkstalker> rogue ability, you may choose an additional sense group to hide from.
            """,
            short_description="Hide from more senses with \\ability<darkstalker>",
        ),
    ]

    apparel += [
        MagicItem(
            name="Dabbler's Amulet",
            level=3,
            material_type="Amulet",
            tags=[],
            description="""
                If you have the \\ability<skill exemplar> ability, you gain a +1 \\glossterm<magic bonus> to all \\glossterm<untrained> skills.
            """,
            short_description="Increase bonus from \\ability<skill exemplar> to untrained skills by 1",
        ),
        MagicItem(
            name="Dabbler's Amulet, Greater",
            level=9,
            material_type="Amulet",
            tags=[],
            description="""
                If you have the \\ability<skill exemplar> ability, you gain a +2 \\glossterm<magic bonus> to all \\glossterm<untrained> skills.
            """,
            short_description="Increase bonus from \\ability<skill exemplar> to untrained skills by 2",
        ),
        MagicItem(
            name="Dabbler's Amulet, Supreme",
            level=15,
            material_type="Amulet",
            tags=[],
            description="""
                If you have the \\ability<skill exemplar> ability, you gain a +3 \\glossterm<magic bonus> to all \\glossterm<untrained> skills.
            """,
            short_description="Increase bonus from \\ability<skill exemplar> to untrained skills by 3",
        ),
    ]

    apparel += [
        MagicItem(
            name="Spellfeeding Amulet",
            level=5,
            material_type="Amulet",
            tags=['Healing'],
            description="""
                Once per round, when you absorb a spell with the \\ability<spell absorption> sorcerer ability, you may regain 1d10+2 \\glossterm<hit points>.
                After you heal in this way, you \\glossterm<briefly> gain no benefit from it or any other \\abilitytag<Healing> effect.
            """,
            short_description="Heals 1d8+2 when you use \\ability<spell absorption>",
        ),
        MagicItem(
            name="Spellfeeding Amulet, Greater",
            level=11,
            material_type="Amulet",
            tags=['Healing'],
            description="""
                Once per round, when you absorb a spell with the \\ability<spell absorption> sorcerer ability, you may regain 2d10+5 \\glossterm<hit points>.
                After you heal in this way, you \\glossterm<briefly> gain no benefit from it or any other \\abilitytag<Healing> effect.
            """,
            short_description="Heals 2d8+5 when you use \\ability<spell absorption>",
        ),
        MagicItem(
            name="Spellfeeding Amulet, Supreme",
            level=17,
            material_type="Amulet",
            tags=['Healing'],
            description="""
                Once per round, when you absorb a spell with the \\ability<spell absorption> sorcerer ability, you may regain 4d10+8 \\glossterm<hit points>.
                After you heal in this way, you \\glossterm<briefly> gain no benefit from it or any other \\abilitytag<Healing> effect.
            """,
            short_description="Heals 4d8+8 when you use \\ability<spell absorption>",
        ),
    ]

    apparel += [
        MagicItem(
            name="Amulet of Wild Control",
            level=5,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<wild magic> sorcerer ability and roll a 1 for its wildspell effect, you can treat the result as a 2 instead.
            """,
            short_description="Mitigates total failure with \\ability<wild magic>",
        ),
        MagicItem(
            name="Amulet of Wild Control, Greater",
            level=14,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<wild magic> sorcerer ability and roll a 1 for its wildspell effect, you can treat the result as a 4 instead.
            """,
            short_description="Prevents total failure with \\ability<wild magic>",
        ),
    ]

    apparel += [
        MagicItem(
            name="Amulet of Abyssal Rebuke",
            level=8,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<abyssal blast> warlock ability, you gain a +1 \\glossterm<accuracy> bonus against each creature that dealt damage to you during the previous round.
            """,
            short_description="Grants +1 accuracy with \\ability<abyssal blast> against attackers",
        ),
        MagicItem(
            name="Amulet of Abyssal Rebuke, Greater",
            level=17,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<abyssal blast> warlock ability, you gain a +2 \\glossterm<accuracy> bonus against each creature that dealt damage to you during the previous round.
            """,
            short_description="Grants +2 accuracy with \\ability<abyssal blast> against attackers",
        ),
    ]

    apparel += [
        MagicItem(
            name="Amulet of Possession",
            level=10,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<possession> warlock ability, its penalties to Fortitude and Mental defense are reduced by 1.
            """,
            short_description="Reduces defense penalties from \\textit<possession>.",
        ),
        MagicItem(
            name="Amulet of Possession, Greater",
            level=19,
            material_type="Amulet",
            tags=[],
            description="""
                When you use the \\ability<possession> warlock ability, its penalties to Fortitude and Mental defense are removed.
            """,
            short_description="Removes defense penalties from \\textit<possession>.",
        ),
    ]

    return apparel


def generate_apparel_latex(check=False):
    apparel = sorted(generate_apparel(), key=lambda apparel: apparel.name)
    if check:
        sanity_check(apparel)

    texts = []
    for item in apparel:
        try:
            texts.append(item.latex())
        except Exception as e:
            raise Exception(f"Error converting item '{item.name}' to LaTeX") from e

    text = "\n".join(texts)
    return latexify(text)


def generate_apparel_table():
    apparel = sorted(
        sorted(generate_apparel(), key=lambda item: item.name),
        key=lambda item: item.level,
    )
    rows = [item.latex_table_row() for item in apparel]
    row_text = "\n".join(rows)
    return longtablify(
        f"""
            \\lcaption<Apparel Items> \\\\
            \\tb<Name> & \\tb<Item Level (Cost)> & \\tb<Type> & \\tb<Description> & \\tb<Page> \\tableheaderrule
            {row_text}
        """
    )


def sanity_check(armor):
    pass


def write_to_file():
    apparel_latex = generate_apparel_latex()
    apparel_table = generate_apparel_table()
    with open(book_path("apparel.tex"), "w") as apparel_description_file:
        apparel_description_file.write(apparel_latex)
    with open(book_path("apparel_table.tex"), "w") as apparel_table_file:
        apparel_table_file.write(apparel_table)


@click.command()
@click.option("-c", "--check/--no-check", default=False)
@click.option("-o", "--output/--no-output", default=False)
def main(output, check):
    if output:
        write_to_file()
    else:
        print(generate_apparel_latex())


if __name__ == "__main__":
    main(None, None)
