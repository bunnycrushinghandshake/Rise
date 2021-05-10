#!/usr/bin/env python3

import click
from rise.latex_generation.book_path import book_path
from rise.latex.magic_item import MagicItem
from rise.latex.util import latexify, longtablify


def generate_armor():
    armor = []

    armor += [
        MagicItem(
            name="Protective Armor",
            level=4,
            material_type="Body armor",
            tags=[],
            description="""
                You gain a +1 \\glossterm<magic bonus> to Armor defense.
            """,
            short_description="Grants +1 Armor defense",
        ),
        MagicItem(
            name="Protective Armor, Greater",
            level=10,
            material_type="Body armor",
            tags=[],
            description="""
                You gain a +2 \\glossterm<magic bonus> to Armor defense.
            """,
            short_description="Grants +2 Armor defense",
        ),
        MagicItem(
            name="Protective Armor, Supreme",
            level=16,
            material_type="Body armor",
            tags=[],
            description="""
                You gain a +3 \\glossterm<magic bonus> to Armor defense.
            """,
            short_description="Grants +3 Armor defense",
        ),
        MagicItem(
            name="Protective Shield",
            level=4,
            material_type="Shield",
            tags=[],
            description="""
                You gain a +1 \\glossterm<magic bonus> to Armor defense.
            """,
            short_description="Grants +1 Armor defense",
        ),
        MagicItem(
            name="Protective Shield, Greater",
            level=10,
            material_type="Shield",
            tags=[],
            description="""
                You gain a +2 \\glossterm<magic bonus> to Armor defense.
            """,
            short_description="Grants +2 Armor defense",
        ),
        MagicItem(
            name="Protective Shield, Supreme",
            level=16,
            material_type="Shield",
            tags=[],
            description="""
                You gain a +3 \\glossterm<magic bonus> to Armor defense.
            """,
            short_description="Grants +3 Armor defense",
        ),
    ]

    armor += [
        MagicItem(
            name="Lifebond Retribution Armor",
            level=4,
            material_type="Body armor",
            tags=[],
            description="""
                At the end of each round, make an attack vs. Fortitude against each creature adjacent to you that caused you to lose \\glossterm<hit points> that round.
                \\hit Each target takes 1d10 energy damage.
            """,
            short_description="Damages adjacent attackers when you lose HP",
        ),
        MagicItem(
            name="Lifebond Retribution Armor, Greater",
            level=10,
            material_type="Body armor",
            tags=[],
            description="""
                At the end of each round, make an attack vs. Fortitude against each creature within a \\areamed radius of you that caused you to lose \\glossterm<hit points> that round.
                \\hit Each target takes 2d8 energy damage.
                \\glance Half damage.
            """,
            short_description="Damages nearby attackers when you lose HP",
        ),
        MagicItem(
            name="Lifebond Retribution Armor, Supreme",
            level=16,
            material_type="Body armor",
            tags=[],
            description="""
                At the end of each round, make an attack vs. Fortitude against each creature within a \\areahuge radius of you that caused you to lose \\glossterm<hit points> that round.
                \\hit Each target takes 4d6 energy damage.
                \\glance Half damage.
            """,
            short_description="Damages attackers when you lose HP",
        ),
    ]

    armor += [
        MagicItem(
            name="Shield of Arrow Catching",
            level=5,
            material_type="Shield",
            description="""
                When an \\glossterm<ally> within a \\areasmall radius emanation from you would be attacked by a ranged weapon, the attack is redirected to target you instead.
                Resolve the attack as if it had initially targeted you, except that the attack is not affected by cover or concealment.
                This item can only affect projectiles and thrown objects that are Small or smaller.
            """,
            short_description="Redirects small nearby projectiles to hit you",
        ),
        MagicItem(
            name="Shield of Arrow Catching, Greater",
            level=11,
            material_type="Shield",
            description="""
                This item functions like a \\mitem<shield of arrow catching>, except that it affects a \\arealarge radius from you.
            """,
            short_description="Redirects small projectiles to hit you",
        ),
        MagicItem(
            name="Shield of Boulder Catching",
            level=8,
            material_type="Shield",
            description="""
                This item functions like a \\mitem<shield of arrow catching>, except that it can affect projectile and thrown objects of up to Large size.
            """,
            short_description="Redirects large nearby projectiles to hit you",
        ),
        MagicItem(
            name="Shield of Boulder Catching, Greater",
            level=14,
            material_type="Shield",
            description="""
                This item functions like a \\mitem<greater shield of arrow catching>, except that it can affect projectile and thrown objects of up to Large size.
            """,
            short_description="Redirects large projectiles to hit you",
        ),
    ]

    armor += [
        MagicItem(
            name="Shield of Arrow Deflection",
            level=5,
            material_type="Shield",
            description="""
                You gain a +2 bonus to defenses against ranged \\glossterm<strikes> from weapons or projectiles that are Small or smaller.
            """,
            short_description="Grants +2 defenses vs small projectiles",
        ),
        MagicItem(
            name="Shield of Arrow Deflection, Greater",
            level=11,
            material_type="Shield",
            description="""
                You gain a +4 bonus to defenses against ranged \\glossterm<strikes> from weapons or projectiles that are Small or smaller.
            """,
            short_description="Grants +4 defenses vs small projectiles",
        ),
        MagicItem(
            name="Shield of Arrow Deflection, Supreme",
            level=17,
            material_type="Shield",
            description="""
                You gain a +6 bonus to defenses against ranged \\glossterm<strikes> from weapons or projectiles that are Small or smaller.
            """,
            short_description="Grants +6 defenses vs small projectiles",
        ),
        MagicItem(
            name="Shield of Boulder Deflection",
            level=8,
            material_type="Shield",
            description="""
                This item functions like a \\mitem<shield of arrow deflection>, except that it can affect weapons and projectiles of up to Large size.
            """,
            short_description="Grants +2 defenses vs projectiles",
        ),
        MagicItem(
            name="Shield of Boulder Deflection, Greater",
            level=14,
            material_type="Shield",
            description="""
                This item functions like a \\mitem<greater shield of arrow deflection>, except that it can affect weapons and projectiles of up to Large size.
            """,
            short_description="Grants +4 defenses vs projectiles",
        ),
    ]

    armor += [
        MagicItem(
            name="Gloves of Precision",
            # +2 levels since gloves are secondary for accuracy
            level=6,
            material_type="Gloves",
            tags=[],
            description="""
                You gain a +1 \\glossterm<magic bonus> to accuracy.
            """,
            short_description="Grants +1 accuracy bonus",
        ),
        MagicItem(
            name="Gloves of Precision, Greater",
            level=12,
            material_type="Gloves",
            tags=[],
            description="""
                You gain a +2 \\glossterm<magic bonus> to accuracy.
            """,
            short_description="Grants +2 accuracy bonus",
        ),
        MagicItem(
            name="Gloves of Precision, Supreme",
            level=18,
            material_type="Gloves",
            tags=[],
            description="""
                You gain a +3 \\glossterm<magic bonus> to accuracy.
            """,
            short_description="Grants +3 accuracy bonus",
        ),
    ]

    armor += [
        MagicItem(
            name="Shield of Bashing",
            # -1 level relative to a potency weapon due to being more limited
            level=3,
            material_type="Shield",
            description="""
                You gain a +2 \\glossterm<magic bonus> to \\glossterm<power> with \\glossterm<strikes> using this shield.
            """,
            short_description="Grants +2 power with strikes",
        ),
        MagicItem(
            name="Shield of Bashing, Greater",
            level=9,
            material_type="Shield",
            description="""
                You gain a +4 \\glossterm<magic bonus> to \\glossterm<power> with \\glossterm<strikes> using this shield.
            """,
            short_description="Grants +4 power with strikes",
        ),
        MagicItem(
            name="Shield of Bashing, Supreme",
            level=15,
            material_type="Shield",
            description="""
                You gain a +8 \\glossterm<magic bonus> to \\glossterm<power> with \\glossterm<strikes> using this shield.
            """,
            short_description="Grants +8 power with strikes",
        ),
    ]

    armor += [
        MagicItem(
            name="Covering Shield",
            level=2,
            material_type="Shield",
            description="""
                When you take the \\textit<total defense> action with this shield, you gain a +4 bonus to Armor defense in addition to the normal bonuses from taking that action (see \\pcref<Total Defense>).
                This property cannot be applied to tower shields.
            """,
            short_description="Grants +4 Armor defense during total defense",
        ),
        MagicItem(
            name="Covering Shield, Greater",
            level=8,
            material_type="Shield",
            description="""
                This shield functions like a \\mitem<covering shield>, except that the defense bonus increases to +6.
                This property cannot be applied to tower shields.
            """,
            short_description="Grants +6 Armor defense during total defense",
        ),
        MagicItem(
            name="Covering Shield, Supreme",
            level=14,
            material_type="Shield",
            description="""
                This shield functions like a \\mitem<covering shield>, except that the defense bonus increases to +8.
                This property cannot be applied to tower shields.
            """,
            short_description="Grants +8 Armor defense during total defense",
        ),
    ]

    armor += [
        MagicItem(
            name="Armor of Energy Resistance",
            level=4,
            tags=[],
            material_type="Body armor",
            description="""
                You gain a +5 \\glossterm<magic bonus> to your \\glossterm<resistance> against \\glossterm<energy damage>.
                When you resist energy damage, this item sheds light as a torch until the end of the next round.
                The color of the light depends on the energy damage resisted: blue for cold, yellow for electricity, and red for fire.
            """,
            short_description="Grants +5 energy resistance",
        ),
        MagicItem(
            name="Armor of Energy Resistance, Greater",
            level=10,
            tags=[],
            material_type="Body armor",
            description="""
                This item functions like \\mitem<armor of energy resistance>, except that the bonus is equal to +10.
            """,
            short_description="Grants +10 energy resistance",
        ),
        MagicItem(
            name="Armor of Energy Resistance, Supreme",
            level=16,
            tags=[],
            material_type="Body armor",
            description="""
                This item functions like \\mitem<armor of energy resistance>, except that the bonus is equal to +20.
            """,
            short_description="Grants +20 energy resistance",
        ),
    ]

    armor += [
        MagicItem(
            name="Featherlight Armor",
            level=5,
            material_type="Body armor",
            description="""
                This armor's \\glossterm<encumbrance> is reduced by 1.
            """,
            short_description="Reduces encumbrance by 1",
        ),
        MagicItem(
            name="Featherlight Armor, Greater",
            level=11,
            material_type="Body armor",
            description="""
                This armor's \\glossterm<encumbrance> is reduced by 2.
            """,
            short_description="Reduces encumbrance by 2",
        ),
        MagicItem(
            name="Featherlight Armor, Supreme",
            level=17,
            material_type="Body armor",
            description="""
                This armor's \\glossterm<encumbrance> is reduced by 3.
            """,
            short_description="Reduces encumbrance by 3",
        ),
    ]

    armor += [
        MagicItem(
            name="Armor of Retribution",
            level=10,
            material_type="Body armor",
            description="""
                At the end of each round, make an attack vs. Fortitude against each creature adjacent to you that attacked you that round.
                \\hit Each target takes 2d6 energy damage.
                \\glance Half damage.
            """,
            short_description="Damages adjacent attackers",
        ),
        MagicItem(
            name="Armor of Retribution, Supreme",
            level=16,
            material_type="Body armor",
            description="""
                At the end of each round, make an attack vs. Fortitude against each creature within a \\areamed radius \\glossterm<emanation> from you that attacked you that round.
                \\hit Each target takes 2d10 energy damage.
                \\glance Half damage.
            """,
            short_description="Damages nearby attackers",
        ),
    ]

    armor += [
        MagicItem(
            name="Armor of Fortification",
            level=8,
            material_type="Body armor",
            description="""
                You gain a +4 bonus to defenses when determining whether a \\glossterm<strike> gets a \\glossterm<critical hit> against you instead of a normal hit.
            """,
            short_description="Reduces critical hits from strikes",
        ),
        MagicItem(
            name="Armor of Fortification, Greater",
            level=17,
            material_type="Body armor",
            description="""
                You gain a +8 bonus to defenses when determining whether a \\glossterm<strike> gets a \\glossterm<critical hit> against you instead of a normal hit.
            """,
            short_description="Drastically reduces critical hits from strikes",
        ),
        MagicItem(
            name="Armor of Fortification, Mystic",
            level=14,
            material_type="Body armor",
            description="""
                You gain a +4 bonus to defenses when determining whether any attack gets a \\glossterm<critical hit> against you instead of a normal hit.
            """,
            short_description="Reduces critical hits from magical attacks",
        ),
    ]

    armor += [
        MagicItem(
            name="Hidden Armor",
            level=3,
            tags=["Sensation"],
            material_type="Body armor",
            description="""
                 As a standard action, you can use this item.
                 If you do, it appears to change shape and form to assume the shape of a normal set of clothing.
                 You may choose the design of the clothing.
                 The item retains all of its properties, including weight and sound, while disguised in this way.
                 Only its visual appearance is altered.

                 Alternately, you may return the armor to its original appearance.
            """,
            short_description="Can look like normal clothing",
        ),
        MagicItem(
            name="Hidden Armor, Greater",
            level=9,
            material_type="Body armor",
            tags=["Sensation"],
            description="""
                This item functions like \\mitem<hidden armor>, except that the item also makes sound appropriate to its disguised form while disguised.
            """,
            short_description="Can look and sound like normal clothing",
        ),
    ]

    armor += [
        MagicItem(
            name="Armor of Kinetic Absorption",
            level=4,
            tags=[],
            material_type="Body armor",
            description="""
                You gain a +5 \\glossterm<magic bonus> to \\glossterm<resistances> against \\glossterm<physical damage>.
            """,
            short_description="Grants +5 physical resistance",
        ),
        MagicItem(
            name="Armor of Kinetic Absorption, Greater",
            level=10,
            tags=[],
            material_type="Body armor",
            description="""
                You gain a +10 \\glossterm<magic bonus> to \\glossterm<resistances> against \\glossterm<physical damage>.
            """,
            short_description="Grants +10 physical resistance",
        ),
        MagicItem(
            name="Armor of Kinetic Absorption, Supreme",
            level=16,
            tags=[],
            material_type="Body armor",
            description="""
                You gain a +20 \\glossterm<magic bonus> to \\glossterm<resistances> against \\glossterm<physical damage>.
            """,
            short_description="Grants +20 physical resistance",
        ),
    ]

    armor += [
        MagicItem(
            name="Armor of Invulnerability",
            level=10,
            tags=[],
            material_type="Body armor",
            description="""
                You gain a +7 \\glossterm<magic bonus> to your \\glossterm<resistances> to both \\glossterm<physical damage> and \\glossterm<energy damage>.
            """,
            short_description="Grants +7 energy and physical resistance",
        ),
        MagicItem(
            name="Armor of Invulnerability, Greater",
            level=16,
            tags=[],
            material_type="Body armor",
            description="""
                You gain a +14 \\glossterm<magic bonus> to your \\glossterm<resistances> to both \\glossterm<physical damage> and \\glossterm<energy damage>.
            """,
            short_description="Grants +14 energy and physical resistance",
        ),
    ]

    armor += [
        MagicItem(
            name="Shield of Mystic Reflection",
            level=10,
            tags=[],
            material_type="Shield",
            description="""
                Whenever you use the \\textit<total defense> action, any \\glossterm<targeted> \\glossterm<magical> abilities that would target you until the end of the round are redirected to target the creature using that ability instead of you.
                Any other targets of the ability are affected normally.
                This is a \\glossterm<Swift> ability, so it affects any abilities targeting you in the phase you take the total defense action.
            """,
            short_description="React to reflect magical attacks",
        ),
        MagicItem(
            name="Shield of Mystic Reflection, Greater",
            level=19,
            material_type="Shield",
            tags=[],
            description="""
                This item functions like a \\mitem<shield of mystic reflection>, except that it also affects all \\glossterm<allies> in a \\areasmall radius from you.
            """,
            short_description="React to reflect magical attacks against allies",
        ),
    ]

    armor.append(
        MagicItem(
            name="Acidic Armor",
            level=7,
            material_type="Body armor",
            tags=[],
            description="""
            At the end of each round, if this armor is making significant contact against a creature or object other than you and your equipment, make an attack vs. Fortitude against it.
            Generally, you can only affect another creature with this armor if you are \\grappled by it.
            On a hit, the target takes 2d6 acid damage.
        """,
            short_description="Deals acid damage to anything it touches",
        )
    )

    armor.append(
        MagicItem(
            name="Shield of Medusa",
            level=8,
            material_type="Shield",
            tags=["Visual"],
            description="""
            This shield normally has a cloth covering its face.
            As a standard action, you can pull the cloth back and reveal the horrifying face emblazoned on the shield.
            When you do, make an attack vs. Fortitude against each creature within a \\areasmall cone.
            On a hit, each target with no remaining \\glossterm<resistance> to \\glossterm<physical damage> is \\glossterm<nauseated> until the end of the next round.
            On a \\glossterm<critical hit>, each target with no remaining \\glossterm<resistance> to \\glossterm<physical damage> is \\glossterm<paralyzed> until the end of the next round.
            In either case, each target is immune to this ability until it takes a \\glossterm<short rest>.

            If the cloth is prematurely pulled back, allowing creatures to see the shield without a dramatic reveal, the shield has no effect.
        """,
            short_description="Can briefly nauseate nearby foes",
        )
    )

    # Other

    return armor


def generate_armor_latex(check=False):
    armor = sorted(generate_armor(), key=lambda armor: armor.name)
    if check:
        sanity_check(armor)

    texts = []
    for item in armor:
        try:
            texts.append(item.latex())
        except Exception as e:
            raise Exception(f"Error converting item '{item.name}' to LaTeX") from e

    text = "\n".join(texts)
    return latexify(text)


def generate_armor_table():
    armor = sorted(
        sorted(generate_armor(), key=lambda item: item.name),
        key=lambda item: item.level,
    )
    rows = [item.latex_table_row() for item in armor]
    row_text = "\n".join(rows)
    return longtablify(
        f"""
        \\lcaption<Armor Items> \\\\
        \\tb<Name> & \\tb<Item Level (Cost)> & \\tb<Type> & \\tb<Description> & \\tb<Page> \\tableheaderrule
        {row_text}
    """
    )


def sanity_check(armor, worn):
    pass


def write_to_file():
    armor_latex = generate_armor_latex()
    armor_table = generate_armor_table()
    with open(book_path("armor.tex"), "w") as armor_description_file:
        armor_description_file.write(armor_latex)
    with open(book_path("armor_table.tex"), "w") as armor_table_file:
        armor_table_file.write(armor_table)


@click.command()
@click.option("-c", "--check/--no-check", default=False)
@click.option("-o", "--output/--no-output", default=False)
def main(output, check):
    if output:
        write_to_file()
    else:
        print(generate_armor_latex())


if __name__ == "__main__":
    main()
