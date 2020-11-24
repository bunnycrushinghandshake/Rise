from rise.latex.mystic_sphere import MysticSphere
from rise.latex.spell import Spell
from rise.latex.effects import Effects


# Primary: damage
# Secondary: utility
# Tertiary: debuff
# None: buff
fabrication=MysticSphere(
    name='Fabrication',
    short_description="Create objects to damage and impair foes",
    cantrips=[
        Effects('Fabricate Trinket', 'Yourself', """
            You make a Craft check to create an object of Tiny size or smaller.
            The object appears in your hand or at your feet.
            It must be made of nonliving, nonmagical, nonreactive vegetable matter, such as wood or cloth.
            At the end of each round, this spell ends if you are not within \\rngmed range of the item.

            This spell lasts until you use it again or until you \\glossterm<dismiss> it as a \\glossterm<free action>.
        """, scaling="""
            \\rank<3> The maximum size of the object increases to Small.
            \\rank<5> The maximum size of the object increases to Medium.
            \\rank<7> The maximum size of the object increases to Large.
        """, tags=['Attune (self)', 'Manifestation']),
    ],
    lists=['Arcane', 'Pact'],
    spells=[
        Spell('Shieldbearer', 2, 'Yourself', """
            You can cast this spell as a \\glossterm<minor action>.

            You gain a +1 \\glossterm<magic bonus> to Armor defense.
        """, scaling="""
            \\rank<4> You are not considered \\glossterm<defenseless> as long as you are not \\glossterm<unaware>, even if you are not wielding a weapon or shield.
            \\rank<6> The bonus increases to +2.
        """, tags=['Attune (self)', 'Manifestation']),
        Spell('Mystic Arrow', 1, 'One creature or object within \\rngmed range', """
            Make an attack vs. Armor against the target.
            \\hit The target takes piercing damage equal to 1d10 plus your \\glossterm<power>.
        """, scaling="damage", tags=['Manifestation']),
        Spell('Mystic Blast Arrow', 3, 'One creature or object within \\rngmed range', """
            Make an attack vs. Armor against the target.
            \\hit The target takes piercing damage equal to 2d8 plus your \\glossterm<power>.
            If the target loses \\glossterm<hit points> from this damage, it is knocked \\glossterm<prone>.
            \\glance As above, except that that the target takes half damage.
        """, scaling="damage", tags=['Manifestation']),
        Spell('Bladestorm', 3, '\\glossterm<Enemies> within a \\areasmall radius of you', """
            This spell does not have the \\glossterm<Focus> tag.
            Make an attack vs. Armor against each target.
            \\hit Each target takes slashing damage equal to 2d6 plus half your \\glossterm<power>.
            \\glance As above, except that that each target takes half damage.
        """, scaling="damage", tags=['Manifestation'], focus=False),
        Spell('Missile Storm', 5, '\\glossterm<Enemies> in a \\arealarge radius from you', """
            Make an attack vs. Armor against each target.
            \\hit Each target takes piercing damage equal to 2d10 plus half your \\glossterm<power>.
            \\glance As above, except that that each target takes half damage.
        """, scaling="damage", tags=['Manifestation']),
        Spell('Hail of Arrows', 3, 'Everything in a \\areasmall radius within \\rngmed range', """
            Make an attack vs. Armor against each target.
            \\hit Each target takes piercing damage equal to 1d8 plus half your \\glossterm<power>.
            \\glance As above, except that that each target takes half damage.
        """, scaling="damage", tags=['Manifestation']),
        Spell('Blade Barrier', 2, 'Each creature that moves through the area (see text)', """
            A wall of whirling blades appears within \\rngmed range.
            The wall takes the form of a 20 ft.\\ high, \\arealarge line.
            The wall provides \\glossterm<cover> against attacks made through it.
            When a creature or object passes through the wall, make an attack vs. Armor against it.
            \\hit The target takes slashing damage equal to 1d10 plus half your \\glossterm<power>.
        """, scaling="damage", tags=['Sustain (minor)']),
        Spell('Blade Perimeter', 3, 'Each creature that moves through the area (see text)', """
            A wall of whirling blades appears within \\rngmed range.
            The wall takes the form of a 20 ft.\\ high, \\areamed radius line.
            The wall provides \\glossterm<cover> against attacks made through it.
            When a creature or object passes through the wall, make an attack vs. Armor against it.
            \\hit The target takes slashing damage equal to 2d6 plus half your \\glossterm<power>.
            \\glance As above, except that that each target takes half damage.
        """, scaling="damage", tags=['Sustain (minor)']),
        Spell('Contracting Blade Perimeter', 6, 'Each creature that moves through the area (see text)', """
            This spell functions like the \\textit<blade perimeter> spell, except that the damage is increased to 4d6 plus half your \\glossterm<power>.
            % TODO: Clarify interaction with solid obstacles that block contraction?
            In addition, the wall's radius shrinks by 5 feet at the end of each round, dealing damage to everything it moves through.
            After the wall shrinks to have no radius, it begins expanding again at a rate of 5 feet per round.
            Once it expands back to its maximum radius, it begins shrinking again.
        """, scaling="damage", tags=[]),
        Spell('Personal Weapon', 1, 'Yourself', """
            Choose a type of weapon that you are proficient with.
            You create a normal item of that type in your hand.
            If the item stops touching you, it disappears, and this effect ends.

            If you create a projectile weapon, you can fire it without ammunition by creating projectiles as you fire.
            The projectiles disappear after the attack is complete.

            % Strange duration for a spell
            This spell lasts until you use it again or until you \\glossterm<dismiss> it as a \\glossterm<free action>.
        """, scaling="""
            \\rank<3> You gain a +1 \\glossterm<magic bonus> to \\glossterm<accuracy> with \\glossterm<strikes> using the weapon.
            \\rank<5> You gain a +2 \\glossterm<magic bonus> to \\glossterm<power> with \\glossterm<strikes> using the weapon.
            \\rank<7> The bonus to accuracy increases to +2.
        """, tags=['Manifestation']),
        Spell('Forge', 1, 'One unoccupied square within \\rngclose range', """
            Choose a type of body armor, weapon, or shield that you are proficient with.
            You cannot create heavy armor.
            You create a normal item of that type at the target location.

            The item cannot be constructed of any magical or extraordinary material.
            It is sized appropriately for you, up to a maximum of a Medium size item.
        """, scaling="""
            \\rank<3> You can also create heavy armor.
            \\rank<5> The item created is magically enhanced.
                A weapon grants a +2 \\glossterm<magic bonus> to \\glossterm<power> with \\glossterm<mundane> abilities,
                    and armor grants a +1 \\glossterm<magic bonus> to Armor defense.
            \\rank<7> You can cast this spell with the \\glossterm<Sustain> (minor) tag instead of the \\glossterm<Attune> (self) tag.
        """, tags=['Attune (self)']),
        # +2 levels for fire + bludgeoning, which breaks resistances
        # +1 level for meteor path
        Spell('Meteor', 6, 'Special', """
            You create a meteor in midair within \\rngmed range that falls to the ground, crushing foes in its path.
            The meteor takes up a \\areasmall radius, and must be created in unoccupied space.
            After being summoned, it falls up to 100 feet before disappearing.
            Make an attack vs. Armor against everything in its path.
            \\hit Each target takes bludgeoning and fire damage equal to 4d6 plus half your \\glossterm<power>.
            \\glance As above, except that that each target takes half damage.
        """, scaling="damage", tags=['Manifestation']),
        # treated like r1 softener, +2 levels for leaving behind something, +1
        # for crit effect
        Spell('Web', 4, 'All Large or smaller creatures in the area (see text)', """
            You fill a \\areasmall radius \\glossterm<zone> within \\rngmed range with webs.
            The webs make the area \\glossterm<difficult terrain>.
            Each 5-ft.\\ square of webbing has a \\glossterm<vital resistance> equal to twice your \\glossterm<power> and is \\glossterm<vulnerable> to fire damage.

            In addition, make an attack vs. Reflex against each target.
            \\hit Each secondary target is \\glossterm<slowed> as long as it has webbing from this ability in its space.
            \\glance As above, except that the effect is removed at the end of the next round.
            \\crit Each secondary target is \\glossterm<immobilized> as long as it has webbing from this ability in its space.
        """, scaling="accuracy", tags=['Manifestation', 'Sustain (minor)']),
        Spell('Caltrops', 2, 'One 5 ft.\\ square within \\rngclose range (see text)', """
            You create exceptionally sharp caltrops in the target location.
            Whenever a creature moves into the area, unless the creature moves at one quarter speed to avoid the danger, you make an attack vs. Armor against them.
            Unlike most attacks, this attack can happen during the \\glossterm<movement phase>.
            Caltrops may not be effective against creatures with an unusual anatomy.
            \\hit The target takes piercing damage equal to 1d10 plus half your \\glossterm<power>.
        """, scaling="damage", tags=['Manifestation', 'Sustain (minor)']),

        # TODO: move all of these acid spells to a different sphere
        Spell('Acid Orb', 1, 'One creature or object within \\rngmed range', """
            Make an attack vs. Fortitude against the target.
            \\hit The target takes acid damage equal to 1d10 plus your \\glossterm<power>.
        """, scaling="damage", tags=['Manifestation']),
        Spell('Cone of Acid', 1, 'Everything in a \\areamed cone from you', """
            Make an attack vs. Fortitude against each target.
            \\hit Each target takes acid damage equal to 1d8 plus half your \\glossterm<power>.
        """, scaling="damage", tags=['Manifestation']),
        Spell('Acid Breath', 4, 'Yourself (see text)', """
            You can cast this spell as a \\glossterm<minor action>.

            As a standard action, you can breathe acid like a dragon.
            When you do, make an attack vs. Fortitude against everything in a \\arealarge cone from you.
            \\hit Each target takes acid damage equal to 2d8 plus half your \\glossterm<power>.
            \\glance As above, except that that each target takes half damage.
        """, scaling="damage", tags=['Attune (self)']),
        Spell('Corrosive Orb', 5, 'One creature or object within \\rngmed range', """
            Make an attack vs. Fortitude against the target.
            \\hit The target takes acid damage equal to 4d6 plus your \\glossterm<power>.
            This attack deals double damage to objects.
            \\glance As above, except that that the target takes half damage.
        """, scaling="damage", tags=['Manifestation']),
        Spell('Acid Rain', 3, 'Everything in a \\areasmall radius within \\rngmed range', """
            Make an attack vs. Fortitude against each target.
            \\hit Each target takes acid damage equal to 2d6 plus half your \\glossterm<power>.
            \\glance As above, except that that each target takes half damage.
        """, scaling="damage", tags=['Manifestation']),
    ],
    rituals=[
        Spell('Manifest Object', 3, 'One unoccupied square within \\rngclose range', """
            Make a Craft check to create an object of Small size or smaller.
            The object appears out of thin air in the target location.
            % TODO: add ability to create objects of other sizes/materials
            It must be made of nonliving, nonmagical, nonreactive vegetable matter, such as wood or cloth.
        """, tags=['Attune (ritual)', 'Manifestation'], ritual_time='one hour'),
        Spell('Create Sustenance', 3, 'One unoccupied squre within \\rngclose range', """
            This ritual creates food and drink in that square that is sufficient to sustain two Medium creatures per \\glossterm<power> for 24 hours.
            The food that this ritual creates is simple fare of your choice -- highly nourishing, if rather bland.
        """, tags=['Creation'], ritual_time='one hour'),
        Spell('Copy Writing', 1, ['One Small or smaller written work within \\rngclose range', 'One Small or smaller set of blank pages within \\rngclose range'], """
            You copy the writing from the primary target onto the secondary target.
            The secondary target must have enough room for the writing.
            This ritual takes half the time required to copy the writing by hand, to a minimum of one minute, and does not require writing materials.
            It requires one \\glossterm<fatigue point> from its participants.
        """, tags=[], ritual_time='special'),
        Spell('Greater Copy Writing', 4, ['One Medium or smaller written work within \\rngclose range', 'One Medium or smaller set of blank pages within \\rngclose range'], """
            This ritual functions like the \\spell<copy writing> ritual, except that it can target objects of Medium or smaller size.
            % "This ritual takes" -- make checks happy
            In addition, the time required to perform this ritual decreases to one tenth of the time required to copy the writing by hand, to a minimum of one minute.
            It requires one \\glossterm<fatigue point> from its participants.
        """, tags=[], ritual_time='special'),
    ],
    category='damage',
)
