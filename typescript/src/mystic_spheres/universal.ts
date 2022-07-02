import { MysticSphere } from '.';

export const universal: MysticSphere = {
  name: 'Universal',
  shortDescription: 'Spells and rituals that can be accessed by any spellcaster.',
  sources: ['arcane', 'divine', 'nature', 'pact'],
  specialRules: `
    The universal mystic sphere does not work in the same way as other mystic spheres.
    Every spellcaster has access to this mystic sphere without needing to spend insight points.
    It contains a variety of simple magical effects that all spellcasters can manage to replicate, though the details can change based on their individual fields of specialization.

    Whenever you learn a spell or ritual from the universal mystic sphere, you must choose a different mystic sphere that you have access to.
    Treat that spell or ritual as if it belonged to your chosen mystic sphere instead of this one.
    This allows you to gain the benefits of any sphere-specific effects when you use the spell or ritual, such as a wizard's \\textit{specialization} ability (see \\pcref{Sphere Specialization}).
    In addition, the chosen mystic sphere determines the spell's damage type and which targets are valid, as listed in \\tref{Universal Mystic Spheres}.
    If the spell is \\spell{mystic bolt}, your chosen mystic sphere also determines the spell's defense.
    You can learn the same spell from this mystic sphere any number of times, choosing different base mystic spheres for that spell each time.

    The names of the abilities from this mystic sphere have fairly generic names to reflect their universal nature.
    As with all other spells, you should feel free to rename them for your own character to make them more interesting and relevant for you.
    For example, a pyromancer might rename their \\spell{mystic bolt} spell to something more fire-themed, like \\textit{firebolt} or \\textit{flamelance}.

    \\begin{dtable!*}
      \\lcaption{Universal Mystic Spheres}
      \\begin{dtabularx}{\\textwidth}{l >{\\lcol}X >{\\lcol}X >{\\lcol}X}
        \\tb{Mystic Sphere} & \\tb{Affected} & \\tb{Mystic Bolt Defense} & \\tb{Damage Type} \\tableheaderrule
        Aeromancy       & Creatures and objects & Armor     & Bludgeoning \\\\
        Aquamancy       & Creatures and objects & Armor     & Bludgeoning \\\\
        Astromancy      & Creatures and objects & Mental    & Energy      \\\\
        Bless           & Creatures             & Mental    & Energy and \\glossterm{subdual} \\\\
        Channel Divinity  & Creatures           & Mental    & Energy      \\\\
        Chronomancy     & Creatures and objects & Fortitude & Energy      \\\\
        Cryomancy       & Creatures and objects & Fortitude & Cold        \\\\
        Electromancy    & Creatures and objects & Fortitude & Electricity \\\\
        Enchantment     & Creatures             & Mental    & Energy and \\glossterm{subdual} \\\\
        Fabrication     & Creatures and objects & Armor     & Physical    \\\\
        Photomancy      & Creatures and objects & Fortitude & Energy      \\\\
        Polymorph       & Creatures and objects & Fortitude & Physical    \\\\
        Pyromancy       & Creatures and objects & Armor     & Fire        \\\\
        Revelation      & Creatures             & Mental    & Energy and \\glossterm{subdual} \\\\
        Summoning       & Creatures and objects & Armor     & Physical    \\\\
        Telekinesis     & Creatures and objects & Armor     & Physical    \\\\
        Terramancy      & Creatures and objects & Armor     & Bludgeoning \\\\
        Thaumaturgy     & Creatures and objects & Fortitude & Energy      \\\\
        Toxicology      & Creatures and objects & Fortitude & Acid        \\\\
        Umbramancy      & Creatures and objects & Fortitude & Cold        \\\\
        Verdamancy      & Creatures and objects & Armor     & Physical    \\\\
        Vivimancy       & Living creatures      & Fortitude & Energy      \\\\
      \\end{dtabularx}
    \\end{dtable!*}
  `,
  spells: [
    {
      name: "Mystic Bolt",

      attack: {
        hit: `The target takes 1d8 + \\glossterm{power} damage.`,
        targeting: `
          Make an attack against anything within \\medrange.
          The defense depends on the mystic sphere you learn this spell with, as listed in \\tref{Universal Mystic Spheres}.
        `,
      },
      rank: 1,
      scaling: "damage",
    },
    {
      name: "Greater Mystic Bolt",

      attack: {
        hit: `The target takes 2d8 + \\glossterm{power} damage.`,
        targeting: `
          Make an attack against anything within \\medrange.
          The defense depends on the mystic sphere you learn this spell with, as listed in \\tref{Universal Mystic Spheres}.
        `,
      },
      rank: 3,
      scaling: "damage",
    },
    {
      name: "Supreme Mystic Bolt",

      attack: {
        hit: `The target takes 4d10 + \\glossterm{power} damage.`,
        targeting: `
          Make an attack against anything within \\longrange.
          The defense depends on the mystic sphere you learn this spell with, as listed in \\tref{Universal Mystic Spheres}.
        `,
      },
      rank: 6,
      scaling: "damage",
    },
    {
      name: "Mystic Blast",

      attack: {
        hit: `Each target takes 1d8 + half \\glossterm{power} damage.`,
        targeting: `
          Make an attack vs. Reflex against everything in a \\medarea cone from you.
        `,
      },
      rank: 2,
      scaling: "damage",
    },
    {
      name: "Greater Mystic Blast",

      attack: {
        hit: `Each target takes 2d6 + half \\glossterm{power} damage.`,
        targeting: `
          Make an attack vs. Reflex against everything in a \\largearea cone from you.
        `,
      },
      rank: 4,
      scaling: "damage",
    },
    {
      name: "Supreme Mystic Blast",

      attack: {
        hit: `Each target takes 2d10 + half \\glossterm{power} damage.`,
        targeting: `
          Make an attack vs. Reflex against everything in a \\hugearea cone from you.
        `,
      },
      rank: 6,
      scaling: "damage",
    },

    {
      name: "Mystic Discharge",

      attack: {
        hit: `Each target takes 1d6 + half \\glossterm{power} damage.`,
        targeting: `
          Make an attack vs. Reflex against everything in a \\smallarea radius from you.
        `,
      },
      rank: 1,
      scaling: "damage",
    },
    {
      name: "Greater Mystic Discharge",

      attack: {
        hit: `Each target takes 2d6 + half \\glossterm{power} damage.`,
        targeting: `
          Make an attack vs. Reflex against everything in a \\smallarea radius from you.
        `,
      },
      rank: 3,
      scaling: "damage",
    },
    {
      name: "Supreme Mystic Discharge",

      attack: {
        hit: `Each target takes 2d10 + \\glossterm{power} damage.`,
        targeting: `
          Make an attack vs. Reflex against everything in a \\smallarea radius from you.
        `,
      },
      rank: 5,
      scaling: "damage",
    },
  ],
  rituals: [
    {
      name: "Mystic Trap",

      castingTime: "one hour",
      attack: {
        hit: `Each target takes 1d6 + half \\glossterm{power} damage.`,
        targeting: `
          When you perform this ritual, choose a point in space within \\shortrange.
          You can choose a point within an \\glossterm{unattended} container as long as the container is currently open.
          One minute after the ritual is completed, that point becomes a trap.

          When a creature moves within a \\smallarea radius from the chosen point, the trap activates.
          The trap's Awareness bonus to notice creatures moving is +10.
          You can choose the minimum size category of creature required to activate the trap.
          When the trap activates, make an attack vs. Reflex against everything within a \\smallarea radius from the trap.

          After the trap activates, this effect is \\glossterm{dismissed}.
        `,
      },
      rank: 1,
      scaling: {
        "special": `
          You can perform this ritual at a higher rank.
          The damage increases by +1d per rank beyond 1.
        `,
      },
      tags: ['Trap'],
      type: "Attune"
    },
    {
      name: "Enduring Mystic Trap",

      castingTime: "24 hours",
      functionsLike: {
        name: "mystic trap",
        exceptThat: `
          the trap persists for one year.
          Whenever it is activated, it is temporarily \\glossterm{suppressed} for 10 minutes.
        `,
      },
      rank: 1,
      scaling: {
        "special": `
          You can perform this ritual at a higher rank.
          The damage increases by +1d per rank beyond 1.
        `,
      },
      tags: ['Trap'],
    },
    {
      name: "Greater Mystic Trap",

      castingTime: "24 hours",
      functionsLike: {
        name: "mystic trap",
        exceptThat: `
          the damage increases to 2d8 + half \\glossterm{power} damage.
          In addition, the area of both the activation and the attack increases to a \\medarea radius.
        `,
      },
      rank: 4,
      scaling: {
        "special": `
          You can perform this ritual at a higher rank.
          The damage increases by +1d per rank beyond 4.
        `,
      },
      tags: ['Trap'],
    },
    {
      name: "Greater Enduring Mystic Trap",

      castingTime: "24 hours",
      functionsLike: {
        name: "greater mystic trap",
        exceptThat: `
          the trap persists for one year.
          Whenever it is activated, it is temporarily \\glossterm{suppressed} for 10 minutes.
        `,
      },
      rank: 4,
      scaling: {
        "special": `
          You can perform this ritual at a higher rank.
          The damage increases by +1d per rank beyond 4.
        `,
      },
      tags: ['Trap'],
    },
  ],
};
