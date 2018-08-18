#!/usr/bin/env python3

import click
from rise.latex_generation.book_path import book_path
from rise.latex.magic_item import MagicItem
from rise.latex.util import latexify

def generate_implements():
    implements = []

    implements.append(MagicItem(
        name="Staff of Sympathetic Shielding",
        level=8,
        material_type='staff',
        tags=['Shielding'],
        description="""
            Whenever you cast a \\glossterm<targeted> \\glossterm<Shielding> spell, if you would be a valid target for the spell, you can target yourself in addition to the spell's normal targets.
        """,
        short_description="Shields you when shielding others",
    ))

    implements.append(MagicItem(
        name="Staff of Transit",
        level=5,
        material_type='staff',
        tags=['Teleportation'],
        description="""
            The maximum distance you can teleport with your \\glossterm<Teleportation> spells is doubled.
        """,
        short_description="Doubles your teleportation distance",
    ))

    implements.append(MagicItem(
        name="Spellfeeding Staff",
        level=6,
        material_type='staff',
        tags=['Life'],
        description="""
            Whenever you cast a spell other than a \\glossterm<cantrip>, you heal hit points equal to your \\glossterm<power> with the spell cast.
        """,
        short_description="Heals you when casting spells",
    ))

    implements.append(MagicItem(
        name="Spellfeeding Staff, Greater",
        level=14,
        material_type='staff',
        tags=['Life'],
        description="""
            Whenever you cast a spell other than a \\glossterm<cantrip>, you heal hit points equal to twice your \\glossterm<power> with the spell cast.
        """,
        short_description="Greatly heals you when casting spells",
    ))

    implements.append(MagicItem(
        name="Wand of Spell Focus",
        level=4,
        material_type='wand',
        tags=['Mystic'],
        description="""
            You gain a +1 \\glossterm<magic bonus> to \\glossterm<power> with a particular spell and all of its subspells.
            Many \\textit<wands of spell focus> exist, each for different spells.
        """,
        short_description="Grants +1 power with a single spell",
    ))

    implements.append(MagicItem(
        name="Wand of Spell Focus, Greater",
        level=10,
        material_type='wand',
        tags=['Mystic'],
        description="""
            You gain a +2 \\glossterm<magic bonus> to \\glossterm<power> with a particular spell and all of its subspells.
            Many \\textit<greater wands of spell focus> exist, each for different spells.
        """,
        short_description="Grants +2 power with a single spell",
    ))

    implements.append(MagicItem(
        name="Wand of Spell Focus, Supreme",
        level=16,
        material_type='wand',
        tags=['Mystic'],
        description="""
            You gain a +3 \\glossterm<magic bonus> to \\glossterm<power> with a particular spell and all of its subspells.
            Many \\textit<supreme wands of spell focus> exist, each for different spells.
        """,
        short_description="Grants +3 power with a single spell",
    ))

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

    text = '\n'.join(texts)
    return latexify(text)

def generate_implement_table():
    implements = sorted(
        sorted(generate_implements(), key=lambda item: item.name),
        key=lambda item: item.level
    )
    rows = [
        f"{item.name} & \\nth<{item.level}> & {item.short_description} & \\pageref<item:{item.name}> \\\\"
        for item in implements
    ]
    row_text = '\n'.join(rows)
    return latexify(f"""
        \\begin<longtabuwrapper>
            \\begin<longtabu><l l X l>
                \\lcaption<Implement Items> \\\\
                \\tb<Name> & \\tb<Level> & \\tb<Description> & \\tb<Page> \\\\
                \\bottomrule
                {row_text}
            \\end<longtabu>
        \\end<longtabuwrapper>
    """)


def write_to_file():
    implement_latex = generate_implement_latex()
    implement_table = generate_implement_table()
    with open(book_path('implements.tex'), 'w') as implement_description_file:
        implement_description_file.write(implement_latex)
    with open(book_path('implements_table.tex'), 'w') as implement_table_file:
        implement_table_file.write(implement_table)


@click.command()
@click.option('-c', '--check/--no-check', default=False)
@click.option('-o', '--output/--no-output', default=False)
def main(output, check):
    if output:
        write_to_file()
    else:
        print(generate_implement_latex())


if __name__ == "__main__":
    main()
