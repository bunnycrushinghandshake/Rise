#!/usr/bin/env python3

import click
from rise.latex.magic_item import MagicItem
from rise.latex.util import latexify

def generate_weapons():
    weapons = []

    weapons.append(MagicItem(
        name="Concussive",
        level=4,
        material_type='weapon',
        # tags=[tag],
        description="""
            As a standard action, you can infuse this weapon with concussive force.
            The next time you make a \\glossterm<strike> with this weapon, if your attack result beats the target's Fortitude defense, it is \\glossterm<dazed> as a \\glossterm<condition>.
        """,
        short_description="Can daze",
    ))

    weapons.append(MagicItem(
        name="Cutthroat",
        level=4,
        material_type='weapon',
        # tags=[tag],
        description="""
            As a standard action, you can make a \\glossterm<strike> with this weapon.
            In addition to the normal effects of the strike, if your attack result beats the target's Fortitude defense, it is \\glossterm<muted> as a \\glossterm<condition>.
        """,
        short_description="Can daze",
    ))

    weapons.append(MagicItem(
        name="Flaming",
        level=5,
        material_type='weapon',
        tags=['Fire'],
        description="""
            This weapon is on fire.
            It sheds light as a torch, and all damage dealt with it is fire damage in addition to its other types.
            As a \\glossterm<minor action>, you can kindle the flames.
            If you do, you gain a +1d bonus to \\glossterm<strike damage> with this weapon.
            This is a \\glossterm<swift ability>, and it lasts until the end of the round.
        """,
        short_description="Can deal +1d fire damage",
    ))

    weapons.append(MagicItem(
        name="Flaming, Greater",
        level=11,
        material_type='weapon',
        tags=['Fire'],
        description="""
            This weapon is on fire.
            It sheds light as a torch, and all damage dealt with it is fire damage in addition to its other types.
            You gain a +1d bonus to \\glossterm<strike damage> with this weapon.
        """,
        short_description="Deals +1d fire damage",
    ))

    weapons.append(MagicItem(
        name="Shocking",
        level=7,
        material_type='weapon',
        tags=['Electricity'],
        description="""
            This weapon continuously crackles with electricity.
            The constant sparks shed light as a torch, and all damage dealt with it is electricity damage in addition to its other types.
            As a \\glossterm<minor action>, you can intensify the electricity.
            If you do, when you make a \\glossterm<strike> with this weapon and your attack result beats the target's Fortitude defense, the target is \\dazed as a \\glossterm<condition>.
            This is a \\glossterm<swift ability>, and it lasts until the end of the round.
        """,
        short_description="Deals electicity damage, can daze",
    ))

    weapons.append(MagicItem(
        name="Shocking, Greater",
        level=13,
        material_type='weapon',
        tags=['Electricity'],
        description="""
            This weapon continuously crackles with electricity.
            The constant sparks shed light as a torch, and all damage dealt with it is electricity damage in addition to its other types.
            When you make a \\glossterm<strike> with this weapon, if your attack result beats the target's Fortitude defense, it is \\dazed as a \\glossterm<condition>.
        """,
        short_description="Deals dazing electicity damage",
    ))

    weapons.append(MagicItem(
        name="Freezing",
        level=4,
        material_type='weapon',
        tags=['Cold'],
        description="""
            This weapon is bitterly cold, and all damage dealt with it is cold damage in addition to its other types.
            As a \\glossterm<minor action>, you can intensify the cold.
            If you do, when you make a \\glossterm<strike> with this weapon and your attack result beats the target's Fortitude defense, the target is \\fatigued as a \\glossterm<condition>.
            This is a \\glossterm<swift ability>, and it lasts until the end of the round.
        """,
        short_description="Deals cold damage, can fatigue",
    ))

    weapons.append(MagicItem(
        name="Freezing, Greater",
        level=10,
        material_type='weapon',
        tags=['Cold'],
        description="""
            This weapon is bitterly cold, and all damage dealt with it is cold damage in addition to its other types.
            When you make a \\glossterm<strike> with this weapon, if your attack result beats the target's Fortitude defense, the target is \\fatigued as a \\glossterm<condition>.
        """,
        short_description="Deals fatiguing cold damage",
    ))

    weapons.append(MagicItem(
        name="Thundering",
        level=5,
        material_type='weapon',
        tags=['Sonic'],
        description="""
            This weapon constantly emits a low-pitched rumbling noise and vibrates slightly in your hand.
            All damage dealt with it is sonic damage in addition to its other types.
            As a \\glossterm<minor action>, you can intensify the vibration.
            If you do, when you make a \\glossterm<strike> with this weapon and your attack result beats the target's Fortitude defense, the target is \\deafened as a \\glossterm<condition>.
            This is a \\glossterm<swift ability>, and it lasts until the end of the round.
        """,
        short_description="Deals sonic damage, can deafen",
    ))

    weapons.append(MagicItem(
        name="Thundering, Greater",
        level=11,
        material_type='weapon',
        tags=['Sonic'],
        description="""
            This weapon constantly emits a low-pitched rumbling noise and vibrates slightly in your hand.
            All damage dealt with it is sonic damage in addition to its other types.
            When you make a \\glossterm<strike> with this weapon and your attack result beats the target's Fortitude defense, the target is \\deafened as a \\glossterm<condition>.
        """,
        short_description="Deals deafening sonic damage",
    ))

    weapons.append(MagicItem(
        name="Defending",
        level=9,
        material_type='weapon',
        tags=['Shielding'],
        description="""
            You gain a +1 \\glossterm<magic bonus> to Armor defense.
        """,
        short_description="Grants +1 Armor defense",
    ))

    weapons.append(MagicItem(
        name="Disorienting",
        level=9,
        material_type='weapon',
        tags=['Compulsion', 'Mind'],
        description="""
            This weapon shimmers with a chaotic pattern of colors.
            As a \\glossterm<minor action>, you can intensify the shimmering.
            If you do, when you make a \\glossterm<strike>  with this weapon and your attack result beats the target's Mental defense, it is \\disoriented as a \\glossterm<condition>.
            This is a \\glossterm<swift ability>, and it lasts until the end of the round.
        """,
        short_description="Can disorient struck foes",
    ))

    weapons.append(MagicItem(
        name="Disorienting, Greater",
        level=15,
        material_type='weapon',
        tags=['Compulsion', 'Mind'],
        description="""
            This weapon shimmers with a chaotic pattern of colors.
            When you make a \\glossterm<strike> with this weapon and your attack result beats the target's Mental defense, it is \\disoriented as a \\glossterm<condition>.
        """,
        short_description="Disorients struck foes",
    ))

    weapons.append(MagicItem(
        name="Longshot",
        level=4,
        material_type='weapon',
        description="""
            Ranged attacks with this weapon have twice the normal \\glossterm<range increment>.
        """,
        short_description="Has twice the normal range increment",
    ))

    weapons.append(MagicItem(
        name="Longshot, Greater",
        level=10,
        material_type='weapon',
        description="""
            Ranged attacks with this weapon have three times the normal \\glossterm<range increment>.
        """,
        short_description="Has three times the normal range increment",
    ))

    weapons.append(MagicItem(
        name="Forceful",
        level=6,
        material_type='weapon',
        tags=['Telekinesis'],
        description="""
            This weapon feels heavy in the hand.
            As a \\glossterm<minor action>, you can intensify the weapon's heft.
            If you do, when you make a \\glossterm<strike> with this weapon, you can also use your attack result as a \\glossterm<shove> attack agsint the target.
            You do not need to move with your foe to move it the full distance of the shove.
            This is a \\glossterm<swift ability>, and it lasts until the end of the round.
        """,
        short_description="Can shove struck foes",
    ))

    weapons.append(MagicItem(
        name="Forceful, Greater",
        level=12,
        material_type='weapon',
        tags=['Telekinesis'],
        description="""
            This weapon feels heavy in the hand.
            When you make a \\glossterm<strike> with this weapon, you can also use your attack result as a \\glossterm<shove> attack agsint the target.
            You do not need to move with your foe to move it the full distance of the shove.
        """,
        short_description="Shoves struck foes",
    ))

    weapons.append(MagicItem(
        name="Fixating",
        level=13,
        material_type='weapon',
        description="""
            When you make a \\glossterm<strike> with this weapon, you gain a +1 bonus to accuracy against the target.
            This bonus lasts until you make a strike with this weapon against a different target.
            This bonus can stack with itself, up to a maximum of +5.
        """,
        short_description="Grants accuracy bonus against struck foe",
    ))

    weapons.append(MagicItem(
        name="Merciful",
        level=3,
        material_type='weapon',
        description="""
            This weapon deals \\glossterm<subdual damage> instead of lethal damage.
        """,
        short_description="Deals subdual damage",
    ))

    weapons.append(MagicItem(
        name="Morphing",
        level=2,
        material_type='weapon',
        tags=['Shaping'],
        description="""
             As a standard action, you can spend an \\glossterm<action point> to activate this item.
             If you do, it changes shape into a new weapon of your choice from the same weapon group.
        """,
        short_description="Can change into similar weapon",
    ))

    weapons.append(MagicItem(
        name="Morphing, Greater",
        level=6,
        material_type='weapon',
        tags=['Shaping'],
        description="""
             As a standard action, you can spend an \\glossterm<action point> to activate this item.
             If you do, it changes shape into a new weapon of your choice that you are proficient with.
             This can only change into existing manufactured weapons, not improvised weapons (see \\pcref<Weapons>).
        """,
        short_description="Can change into any weapon",
    ))

    weapons.append(MagicItem(
        name="Returning",
        level=3,
        material_type='weapon',
        tags=['Teleportation'],
        description="""
            After being thrown, this weapon teleports back into your hand at the end of the current phase.
            Catching a rebounding weapon when it comes back is a free action.
            If you can't catch it, the weapon drops to the ground in the square from which it was thrown.
        """,
        short_description="Teleports back to you after being thrown",
    ))

    weapons.append(MagicItem(
        name="Seeking",
        level=7,
        material_type='weapon',
        tags=['Knowledge'],
        description="""
            This weapon automatically veers towards its intended target.
            \\glossterm<Strikes> with this weapon that would suffer a 50\\% miss chance instead suffer a 20\\% miss chance.
            In addition, attacks that would otherwise suffer a 20\\% miss chance instead suffer no miss chance.
        """,
        short_description="Reduces miss chances",
    ))

    weapons.append(MagicItem(
        name="Soulreaving",
        level=13,
        tags=[],
        material_type='weapon',
        description="""
            This weapon is transluscent and has no physical presence for anyone except you.
            It has no effect on objects or constructs, and creatures do not feel any pain or even notice attacks from it.
            Attacks with this weapon ignore all damage reduction and hardness, but the damage is delayed instead of being dealt immediately.
            Damage that would be dealt by the weapon can be delayed indefinitely.
            While the damage is delayed, it cannot be removed by any means short of the destruction of this weapon or the creature's death.

            As a \\glossterm<minor action>, you can cut yourself with this weapon to activate it.
            This deals no damage to you.
            If you do, all delayed damage dealt by this weapon is converted into real damage.
            Any such damage dealt in excess of a creature's hit points is dealt immediately as \\glossterm<vital damage>.
        """,
        short_description="Deals delayed damage",
    ))

    weapons.append(MagicItem(
        name="Surestrike",
        level=9,
        material_type='weapon',
        tags=['Knowledge'],
        description="""
            You gain a +1 \\glossterm<magic bonus> to accuracy with \\glossterm<strikes> with this weapon.
        """,
        short_description="Gain +1 accuracy bonus",
    ))

    weapons.append(MagicItem(
        name="Heartseeker",
        level=17,
        material_type='weapon',
        tags=['Knowledge'],
        description="""
            Whenever you make a \\glossterm<strike> with this weapon, you can roll twice and take the higher result.
        """,
        short_description="Rolls attacks twice",
    ))

    weapons.append(MagicItem(
        name="Thieving",
        level=7,
        material_type='weapon',
        tags=['Shaping'],
        description="""
            As a \\glossterm<standard action>, you can spend an \\glossterm<action point> to activate this weapon.
            If you do, make a \\glossterm<strike> or a \\glossterm<disarm> attack.
            If your disarm succeeds, or if your strike hit an unattended object, this weapon can absorb the struck object.
            The object must be at least one size category smaller than the weapon.
            An absorbed object leaves no trace that it ever existed.

            This weapon can hold no more than three objects at once.
            If you attempt to absorb an object while the weapon is full, the attempt fails.

            As a standard action, you can retrieve the last item absorbed by the weapon.
            The item appears in your hand, or falls to the ground if your hand is occupied.
        """,
        short_description="Can absorb small items",
    ))

    weapons.append(MagicItem(
        name="Thieving, Greater",
        level=13,
        material_type='weapon',
        tags=['Shaping'],
        description="""
            This item functions like the \\mitem<thieving> item, except that the maximum size category of object it can absorb is one size category larger than the weapon.
        """,
        short_description="Can absorb large items",
    ))

    weapons.append(MagicItem(
        name="Vampiric",
        level=6,
        material_type='weapon',
        tags=['Life'],
        description="""
            When you deal damage to a living creature with a \\glossterm<strike> with this weapon, you heal hit points equal to your level.
        """,
        short_description="Heals you when dealing damage",
    ))

    weapons.append(MagicItem(
        name="Vampiric, Greater",
        level=14,
        material_type='weapon',
        tags=['Life'],
        description="""
            When you deal damage to a living creature with a \\glossterm<strike> with this weapon, you heal hit points equal to twice your level.
        """,
        short_description="Drastically heals you when dealing damage",
    ))

    weapons.append(MagicItem(
        name="Vorpal",
        level=12,
        material_type='weapon',
        description="""
            Critical hits on \\glossterm<strikes> with this weapon deal maximum damage.
        """,
        short_description="Inflicts lethal critical hits",
    ))

    weapons.append(MagicItem(
        name="Phasing",
        level=9,
        material_type='weapon',
        tags=['Planar'],
        description="""
            \\glossterm<Strikes> with this weapon can pass through a single solid obstacle of up to five feet thick on the way to their target.
            This can allow you to ignore \\glossterm<cover>, or even attack through solid walls.
            It does not allow you to ignore armor, shields, or or similar items used by the target of your attacks.
        """,
        short_description="Can ignore obstacles when attacking",
    ))

    return weapons

def sanity_check(weapons):
    pass

def generate_weapon_latex(check=False):
    weapons = sorted(generate_weapons(), key=lambda weapons: weapons.name)
    if check:
        sanity_check(weapons)

    texts = []
    for item in weapons:
        try:
            texts.append(item.latex())
        except Exception as e:
            raise Exception(f"Error converting item '{item.name}' to LaTeX") from e

    text = '\n'.join(texts)
    return latexify(text)

def generate_weapon_table():
    weapons = sorted(
        sorted(generate_weapons(), key=lambda item: item.name),
        key=lambda item: item.level
    )
    rows = [
        f"{item.name} & \\nth<{item.level}> & {item.short_description} & \\pageref<item:{item.name}> \\\\"
        for item in weapons
    ]
    row_text = '\n'.join(rows)
    return latexify(f"""
        \\begin<longtabuwrapper>
            \\begin<longtabu><l l X l>
                \\lcaption<Weapon Items> \\\\
                \\tb<Name> & \\tb<Level> & \\tb<Description> & \\tb<Page> \\\\
                \\bottomrule
                {row_text}
            \\end<longtabu>
        \\end<longtabuwrapper>
    """)

@click.command()
@click.option('-c', '--check/--no-check', default=False)
@click.option('-o', '--output/--no-output', default=False)
def main(output, check):
    weapon_latex = generate_weapon_latex()
    if output is None:
        print(weapon_latex)
    else:
        with open('../../core_book/weapons.tex', 'w') as weapon_description_file:
            weapon_description_file.write(weapon_latex)
        with open('../../core_book/weapons_table.tex', 'w') as weapon_table_file:
            weapon_table_file.write(generate_weapon_table())


if __name__ == "__main__":
    main()
