import { MysticSphere } from ".";

export const cryomancy: MysticSphere = {
  name: "Cryomancy",
  shortDescription: "Drain heat to injure and freeze foes.",
  sources: ["arcane", "nature", "pact"],

  cantrips: [
    {
      name: "Chill",

      attack: {
        hit: `
          The subject takes 2 cold damage.
        `,
        targeting: `
          Make an attack vs. Fortitude against anything within \\shortrange.
        `,
      },
      scaling: {
        2: "The damage increases to 5.",
        4: "The damage increases to 10.",
        6: "The damage increases to 20.",
      },
      type: "Instant",
    },
    {
      name: "Chill Air",

      effect: `
        The temperatuture of the air within a \\areamed radius \\glossterm{emanation} from you is reduced by an amount of your choice, to a maximum reduction of 20 degrees Fahrenheit.
        You cannot reduce the temperature below 0 degrees in this way.
        This typically imposes no direct penalties on other creatures, but it may make them more or less comfortable depending on their preferred temperature.

        This ability lasts until you use it again or until you \\glossterm{dismiss} it as a \\glossterm{free action}.
      `,
      scaling: {
        2: "The area increases to a \\largearea radius \\glossterm{emanation}, and the maximum temperature reduction increases to 30 degrees.",
        4: "The area increases to a \\hugearea radius \\glossterm{emanation}, and the maximum temperature reduction increases to 40 degrees.",
        6: "The area increases to a \\gargarea radius \\glossterm{emanation}, and the maximum temperature reduction increases to 50 degrees.",
      },
      type: "Duration",
    },
  ],
  spells: [
    {
      name: "Freezing Grasp",

      attack: {
        hit: `The subject takes 1d10 + \\glossterm{power} cold damage.`,
        targeting: `
        This spell does not have the \\glossterm{Focus} tag.
        You must have a \\glossterm{free hand} to cast this spell.

        Make a melee attack vs. Reflex against anything within your \\glossterm{reach}.
        `,
      },
      focus: false,
      rank: 1,
      scaling: "damage",
      type: "Instant",
    },

    {
      name: "Cone of Cold",

      attack: {
        hit: `Each subject takes 1d8 + half \\glossterm{power} cold damage.`,
        targeting: `
        Make an attack vs. Fortitude against everything in a \\smallarea cone from you.
        `,
      },
      rank: 1,
      scaling: "damage",
      type: "Instant",
    },

    {
      name: "Greater Cone of Cold",

      attack: {
        glance: `Half damage.`,
        hit: `Each subject takes 2d8 + half \\glossterm{power} cold damage.`,
        targeting: `
          Make an attack vs. Fortitude against everything in a \\hugearea cone from you.
        `,
      },
      rank: 4,
      scaling: "damage",
      type: "Instant",
    },

    {
      name: "Supreme Cone of Cold",

      attack: {
        glance: `Half damage.`,
        hit: `Each subject takes 4d10 + half \\glossterm{power} cold damage.`,
        targeting: `
          Make an attack vs. Fortitude against everything in a \\gargarea cone from you.
        `,
      },
      rank: 7,
      type: "Instant",
    },

    {
      name: "Frozen Legs",

      attack: {
        glance: `Half damage.`,
        hit: `The subject takes 2d6 cold damage.
        If it loses \\glossterm{hit points} from this damage, it is \\glossterm{immobilized} as a \\glossterm{condition}.`,
        targeting: `
          Make an attack vs. Fortitude against one creature within \\medrange.
        `,
      },
      rank: 4,
      scaling: "damage",
      type: "Duration",
    },

    {
      name: "Mass Frozen Legs",

      attack: {
        glance: "The effect lasts until the end of the next round.",
        hit: `Each subject that has no remaining \\glossterm{resistance} to cold damage is \\glossterm{immobilized} as a \\glossterm{condition}.`,
        targeting: `
          Make an attack vs. Fortitude against all creatures in a \\smallarea radius within \\medrange.
        `,
      },
      rank: 6,
      scaling: "accuracy",
      type: "Duration",
    },

    {
      name: "Ice Lance",

      attack: {
        glance: `Half damage.`,
        hit: `Each subject takes piercing and cold damage equal to 2d8 plus half your \\glossterm{power} (see \\pcref{Multiple Damage Types}).`,
        targeting: `
        Make an attack vs. Armor against everything in a \\medarea long, 10 ft. wide line from you.
        `,
      },

      rank: 4,
      scaling: "damage",
      tags: ["Manifestation"],
      type: "Instant",
    },

    {
      name: "Ice Spike",

      attack: {
        glance: `Half damage.`,
        hit: `The subject takes 2d8 + \\glossterm{power} piercing and cold damage (see \\pcref{Multiple Damage Types}).`,
        targeting: `
          Make an attack vs. Armor against anything within \\medrange.
        `,
      },
      rank: 3,
      scaling: "damage",
      tags: ["Manifestation"],
      type: "Instant",
    },

    {
      name: "Greater Ice Spike",

      attack: {
        glance: `Half damage.`,
        hit: `The subject takes 4d10 + \\glossterm{power} piercing and cold damage (see \\pcref{Multiple Damage Types}).`,
        targeting: `
          Make an attack vs. Armor against anything within \\longrange.
        `,
      },
      rank: 6,
      scaling: "damage",
      tags: ["Manifestation"],
      type: "Instant",
    },

    {
      name: "Freeze Poison",

      effect: `
        Choose yourself or one \\glossterm{ally} within \\medrange.
        The subject takes 1d4 cold damage.
        In addition, it gains an additional success to resist a poison currently affecting it (see \\pcref{Poison}).
      `,
      rank: 1,
      scaling: {
        3: `The number of additional successes increases to two.
            The subject can split these successes among any number of different poisons affecting it.`,
        5: `The number of additional successes increases to three.`,
        7: `The number of additional successes increases to four.`,
      },
      type: "Instant",
    },

    {
      name: "Brittle Chill",

      attack: {
        glance: `Half damage.`,
        hit: `The subject takes 2d6 cold damage.
        If it loses \\glossterm{hit points} from this damage, it is \\glossterm{vulnerable} to bludgeoning damage as a \\glossterm{condition}.`,
        targeting: `
          Make an attack vs. Fortitude against anything within \\medrange.
        `,
      },
      rank: 3,
      scaling: "damage",
      type: "Duration",
    },

    {
      name: "Chilled Mind",

      attack: {
        hit: `The subject takes 1d6 cold damage.
        If it loses \\glossterm{hit points} from this damage, it is \\glossterm{stunned} as a \\glossterm{condition}.`,
        targeting: `
          Make an attack vs. Fortitude against one creature within \\medrange.
        `,
      },

      rank: 1,
      scaling: "damage",
      type: "Duration",
    },

    {
      name: "Mass Chilled Mind",

      attack: {
        glance: "The effect lasts until the end of the next round.",
        hit: `Each subject that has no remaining \\glossterm{resistance} to cold damage is \\glossterm{stunned} as a \\glossterm{condition}.`,
        targeting: `
          Make an attack vs. Fortitude against all \\glossterm{enemies} in a \\arealarge radius from you.
        `,
      },
      rank: 4,
      scaling: "accuracy",
      type: "Duration",
    },

    {
      name: "Skate",

      castingTime: "minor action",
      effect: `
        You can move on top of calm water as if it were land.
        You treat the water as \\glossterm{difficult terrain}.
      `,
      rank: 1,
      scaling: {
        3: `You can also move on top of rough or stormy water.`,
        5: `You no longer treat the water as difficult terrain.`,
        7: `You also gain a +10 foot \\glossterm{magic bonus} to your land speed.`,
      },
      type: "Attune (self)",
    },

    {
      name: "Mass Skate",

      functionsLike: {
        mass: true,
        spell: "Skate",
      },
      // narrative: '',
      rank: 3,
      scaling: {
        3: `The subject can also move on top of rough or stormy water.`,
        5: `The subject no longer treats the water as difficult terrain.`,
      },
      type: "Attune (target)",
    },

    {
      name: "Skyskate",

      effect: `
        Whenever you move, you can leave a trail of ice behind you.
        The ice lasts until the end of the round before disappearing.

        While you are leaving a trail of ice behind you, you can move into thin air by walking on your own ice trail, just as if it was solid ground.
        If you are still standing on your own ice trail when it disappears at the end of the round, you fall.

        Creatures following closely behind you while you move may also be able to use your ice trail.
        However, most Large or larger creatures will break the ice trail if they step onto it, which may cause both of you to fall.
      `,
      rank: 3,
      scaling: {
        5: `Your ice trail collapses more gradually.  If you are still standing on your own ice trail when it disappears, you treat your fall as if it were 60 feet shorter than it actually was for the purpose of determining \\glossterm{falling damage}.`,
        7: `Your ice trail lasts until the end of the next round after your movement.`,
      },
      tags: ["Manifestation"],
      type: "Attune (self)",
    },

    {
      name: "Icy Shell",

      effect: `
        You cover your body with three overlapping layers of ice that crumple when they take damage.
        The ice does not cover your joints, allowing you to move freely.
        You are \\glossterm{impervious} to \\glossterm{physical damage} and \\glossterm{fire damage}.
        Whenever you take physical damage or fire damage, one layer of ice is destroyed.
        When the last layer of ice is destroyed, this ability provides no further benefit.
      `,
      rank: 2,
      scaling: {
        4: `The spell creates four layers of ice.`,
        6: `The spell creates five layers of ice.`,
      },
      tags: ["Manifestation"],
      type: "Attune (self)",
    },

    {
      name: "Frostbite",

      attack: {
        glance: `Half damage.`,
        hit: `The subject takes 1d8 + half \\glossterm{power} cold damage.
        If it loses \\glossterm{hit points} from this damage, it is \\glossterm{slowed} as a \\glossterm{condition}.`,
        targeting: `
          Make an attack vs. Fortitude against anything within \\medrange.
        `,
      },

      rank: 1,
      scaling: "damage",
      type: "Duration",
    },

    {
      name: "Greater Frostbite",

      attack: {
        glance: `Half damage.`,
        hit: `The subject takes 2d8 + half \\glossterm{power} cold damage.
        If it loses \\glossterm{hit points} from this damage, it is \\glossterm{decelerated} as a \\glossterm{condition}.`,
        targeting: `
          Make an attack vs. Fortitude against anything within \\medrange.
        `,
      },

      rank: 4,
      scaling: "damage",
      type: "Duration",
    },

    {
      name: "Hailstorm",

      attack: {
        glance: `Half damage.`,
        hit: `Each subject takes 2d10 + half \\glossterm{power} bludgeoning and cold damage (see \\pcref{Multiple Damage Types}).`,
        targeting: `
          Make an attack vs. Armor against everything in a \\smallarea radius within \\medrange.
        `,
      },

      rank: 5,
      scaling: "damage",
      type: "Instant",
    },

    {
      name: "Blizzard",

      attack: {
        glance: `Half damage.`,
        hit: `Each subject takes 1d10 + half \\glossterm{power} cold damage.`,
        targeting: `
          Make an attack vs. Fortitude against everything in a \\medarea radius from you.
        `,
      },
      rank: 2,
      scaling: "damage",
      type: "Instant",
    },

    {
      name: "Greater Blizzard",

      attack: {
        glance: `Half damage.`,
        hit: `Each subject takes 4d6 + half \\glossterm{power} cold damage.`,
        targeting: `
          Make an attack vs. Fortitude against everything in a \\largearea radius from you.
        `,
      },
      rank: 5,
      scaling: "damage",
      type: "Instant",
    },

    {
      name: "Icecraft",

      effect: `
        Choose one pool of \\glossterm{unattended}, nonmagical water within \\shortrange.
        This spell creates an icy weapon or a suit of icy armor from the subject pool of water.
        You can create any weapon, shield, or body armor that you are proficient with, and which would normally be made entirely from metal, except for heavy armor.
        The pool of water targeted must be at least as large as the item you create.

        The item functions like a normal item of its type, except that it is easier to destroy with fire damage.
        When a creature wearing armor created in this way takes fire damage, that damage is also dealt to the armor.
      `,
      rank: 1,
      scaling: {
        3: `You can also create heavy armor.
                In addition, the item created is magically enhanced.
                A weapon grants a +2 \\glossterm{magic bonus} to your \\glossterm{mundane} \\glossterm{power},
                    and armor grants a +1 \\glossterm{magic bonus} to Armor defense.`,
        5: `The magic bonus for a weapon increases to +4, and the magic bonus for armor increases to +2.`,
        7: `The magic bonus for a weapon increases to +8, and the magic bonus for armor increases to +3.`,
      },
      type: "Attune (self)",
    },

    {
      name: "Frost Breath",

      castingTime: "minor action",
      attack: {
        glance: `Half damage.`,
        // +1d for attune + every other round
        hit: `Each subject takes 2d8 + half \\glossterm{power} cold damage.`,
        targeting: `
          For the duration of this spell, you can breathe cold like a dragon as a standard action.
          When you do, make an attack vs. Fortitude against everything in a \\largearea cone from you.
          After you use this ability, you cannot use it again until after the end of the next round.
        `,
      },
      rank: 3,
      scaling: "damage",
      type: "Attune (self)",
    },

    {
      name: "Frostburn",

      // original targets: One creature within \shortrange
      attack: {
        crit: `The damage from the condition is doubled.`,
        glance:
          "The effect lasts until the end of the next round. The subject still takes damage during that round.",
        hit: `As a \\glossterm{condition}, the subject is seared by painful cold.
        % TODO: standardize "ignite" damage
        At the end of each round, it takes 4d10 cold damage.`,
        targeting: `
          Make an attack vs. Fortitude against one creature within \\shortrange.
        `,
      },
      rank: 7,
      type: "Duration",
    },

    {
      name: "Frigid Aura",

      // original targets: ['Yourself', 'See text']
      castingTime: "minor action",
      attack: {
        glance: `Half damage.`,
        // full dice, but half power
        hit: `Each subject takes 2d8 + half \\glossterm{power} cold damage.`,
        targeting: `
        At the end of each round, make an attack vs. Fortitude against each creature adjacent to you that either is \\glossterm{grappling} with you or that attacked you with a melee weapon that round.
        `,
      },

      rank: 3,
      scaling: "damage",
      type: "Attune (self)",
    },

    {
      name: "Chillwind Dash",

      attack: {
        glance: `Half damage.`,
        hit: `Each subject takes 2d6 + half \\glossterm{power} cold damage.`,
        targeting: `
          You teleport into an unoccupied destination on a stable surface within \\shortrange.
          In addition, make an attack vs. Reflex against everything in a 5 ft.\\ wide line between your starting location and your ending location.
        `,
      },
      rank: 3,
      scaling: "damage",
      type: "Instant",
    },

    {
      name: "Greater Chillwind Dash",

      attack: {
        glance: `Half damage.`,
        hit: `Each subject takes 4d6 + half \\glossterm{power} cold damage.`,
        targeting: `
          You teleport into an unoccupied destination on a stable surface within \\distrange.
          In addition, make an attack vs. Reflex against everything in a 5 ft.\\ wide line between your starting location and your ending location.
        `,
      },
      rank: 6,
      scaling: "damage",
      type: "Instant",
    },
  ],
  rituals: [
    {
      name: "Frostfall",

      castingTime: "one hour",
      effect: `
        The temperature in a two mile radius cylinder-shaped \\glossterm{zone} from your location decreases rapidly.
        Over the next minute after you finish this ritual, the temperature decreases by 40 degrees Fahrenheit, to a minimum of \\minus30 degrees.
        Unlike normal, this effect does not require \\glossterm{line of effect} to you.
        Instead, it affects all outdoor locations within the area.
        Even a thin barrier, such as a tent, is enough to protect locations from the effect of this ritual.
      `,
      rank: 4,
      type: "Attune (self)",
    },

    {
      name: "Froststorm",

      castingTime: "one hour",
      functionsLike: {
        exceptThat: `
          the temperature in the area decreases by 60 degrees, to a minimum of \\minus70 degrees.
        `,
        spell: "frostfall",
      },
      rank: 7,
      type: "Attune (self)",
    },
  ],
};