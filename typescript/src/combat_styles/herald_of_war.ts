import { CombatStyle } from ".";

export const heraldOfWar: CombatStyle = {
  name: "Herald of War",
  shortDescription:
    "Demoralize foes and inspire allies with battlecries and a commanding presence.",

  maneuvers: [
    {
      name: "Boastful Battlecry",

      effect: `
        This ability targets all \\glossterm{enemies} within a \\largearea radius from you.
        You \\glossterm{briefly} gain a +4 bonus to \\glossterm{accuracy} with \\glossterm{strikes} against each target.
      `,
      rank: 1,
      scaling: {
        3: "The bonus increases to +5.",
        5: "The bonus increases to +6.",
        7: "The bonus increases to +7.",
      },
      type: "Instant",
    },

    {
      name: "Dazing Roar",

      attack: {
        crit: `The effect becomes a \\glossterm{condition} on each target.`,
        hit: `Each target is \\glossterm{briefly} \\dazed.`,
        targeting: `
          Make an attack vs. Mental against all \\glossterm{enemies} in a \\medarea radius from you.
        `,
      },
      scaling: "accuracy",
      rank: 2,
      type: "Duration",
    },

    {
      name: "Stunning Roar",

      attack: {
        crit: `The effect becomes a \\glossterm{condition} on each target.`,
        hit: `Each target is \\glossterm{briefly} \\stunned.`,
        targeting: `
          Make an attack vs. Mental against all \\glossterm{enemies} in a \\medarea radius from you.
        `,
      },
      scaling: "accuracy",
      rank: 6,
      type: "Duration",
    },

    {
      name: "Goading Roar",

      attack: {
        crit: `The effect becomes a \\glossterm{condition} on each target.`,
        hit: `
          Each target is \\glossterm{briefly} \\goaded.
        `,
        targeting: `
          Make an attack vs. Mental against all \\glossterm{enemies} in a \\smallarea radius from you.
        `,
      },
      scaling: "accuracy",
      rank: 1,
      tags: ["Emotion"],
      type: "Duration",
    },

    {
      name: "Greater Goading Roar",

      functionsLike: {
        name: 'goading roar',
        exceptThat: "the area increases to a \\largearea radius.",
      },
      scaling: "accuracy",
      rank: 4,
      tags: ["Emotion"],
      type: "Duration",
    },

    {
      name: "Fearsome Roar",

      attack: {
        crit: `The effect becomes a \\glossterm{condition}.`,
        hit: `
          Each target with remaining \\glossterm{damage resistance} is \\glossterm{briefly} \\shaken by you.
          Each target without remaining damage resistance is \\frightened by you instead of shaken.
        `,
        targeting: `
          Make an attack vs. Mental against all \\glossterm{enemies} in a \\smallarea radius from you.
        `,
      },
      scaling: "accuracy",
      rank: 3,
      tags: ["Emotion"],
      type: "Duration",
    },

    {
      name: "Greater Fearsome Roar",

      functionsLike: {
        name: "fearsome roar",
        exceptThat: "the area increases to a \\largearea radius.",
      },
      scaling: "accuracy",
      rank: 6,
      tags: ["Emotion"],
      type: "Duration",
    },

    {
      name: "Thunderous Shout",

      attack: {
        hit: `Each target takes 2d6 + half \\glossterm{power} sonic damage.`,
        targeting: `
          Make an attack vs. Fortitude against everything in a \\smallarea cone from you.
        `,
      },
      scaling: "damage",
      rank: 3,
      type: "Instant",
    },

    {
      name: "Greater Thunderous Shout",

      attack: {
        hit: `Each target takes 4d6 + half \\glossterm{power} sonic damage.`,
        targeting: `
          Make an attack vs. Fortitude against everything in a \\largearea cone from you.
        `,
      },
      rank: 6,
      type: "Instant",
    },

    {
      name: "Directed Shout",

      attack: {
        hit: `The target takes 2d6 + \\glossterm{power} sonic damage.`,
        targeting: `
          Make an attack vs. Fortitude against anything within \\shortrange of you.
        `,
      },
      scaling: "damage",
      rank: 2,
      type: "Instant",
    },

    {
      name: "Greater Directed Shout",

      attack: {
        hit: `The target takes 4d8 + \\glossterm{power} sonic damage.`,
        targeting: `
          Make an attack vs. Fortitude against anything within \\medrange of you.
        `,
      },
      scaling: "damage",
      rank: 5,
      type: "Instant",
    },

    {
      name: "Rally the Troops",

      effect: `
        You and your \\glossterm{allies} within a \\largearea radius from you can each \\glossterm{briefly} ignore any effects from one \\glossterm{condition} they are already affected by.
        Because this ability has the \\abilitytag{Swift} tag, it removes any relevant penalties from that condition during the current phase.
      `,
      rank: 3,
      scaling: {
        5: "The number of conditions that can be ignored increases to two.",
        7: "The number of conditions that can be ignored increases to three.",
      },
      tags: ["Swift"],
      type: "Duration",
    },

    {
      name: "Challenging Strike",

      effect: `
        Make a \\glossterm{strike}.
        You do not add your \\glossterm{power} to damage with the strike.
        Each creature damaged by the strike is \\glossterm{briefly} \\goaded by you.
      `,
      rank: 1,
      scaling: {
        3: "You gain a +1 accuracy bonus with the strike.",
        5: "The accuracy bonus increases to +2.",
        7: "The accuracy bonus increases to +3.",
      },
      type: "Duration",
    },

    {
      name: "Fearsome Blow",

      effect: `
        Make a \\glossterm{strike}.
        Your \\glossterm{power} with the strike is halved.
        Each creature that loses \\glossterm{hit points} from the strike is \\shaken by you as a \\glossterm{condition}.
      `,
      rank: 1,
      scaling: {
        3: "You gain a +1 accuracy bonus with the strike.",
        5: "The accuracy bonus increases to +2.",
        7: "The accuracy bonus increases to +3.",
      },
      tags: ["Emotion"],
      type: "Duration",
    },

    {
      name: "Greater Fearsome Blow",

      effect: `
        Make a \\glossterm{strike}.
        Your \\glossterm{power} with the strike is halved.
        Each creature that loses \\glossterm{hit points} from the strike is \\frightened by you as a \\glossterm{condition}.
      `,
      rank: 5,
      scaling: {
        7: "You gain a +1 accuracy bonus with the strike.",
      },
      tags: ["Emotion"],
      type: "Duration",
    },

    // The rank here is pretty ambiguous
    {
      name: "Awe-Inspiring Strike",

      effect: `
        Make a melee \\glossterm{strike}.
        Your \\glossterm{power} with the strike is halved.
        In addition, make an attack vs. Mental against each \\glossterm{enemy} other than the target of that strike within a \\smallarea radius from you.
        On a hit, each target is \\glossterm{briefly} \\shaken by you.
      `,
      rank: 3,
      scaling: {
        5: "You gain a +1 accuracy bonus with both the strike and the area attack.",
        7: "The accuracy bonus increases to +2.",
      },
      type: "Duration",
    },

    {
      name: "Inspiring Strike",

      effect: `
        Make a melee \\glossterm{strike}.
        Your \\glossterm{power} with the strike is halved.
        If you deal damage to a creature with the strike, \\glossterm{allies} within a \\largearea radius from you \\glossterm{briefly} gain a +2 bonus to Mental defense.
      `,
      rank: 1,
      scaling: {
        3: "The bonus increases to +3.",
        5: "The bonus increases to +4.",
        7: "The bonus increases to +5.",
      },
      type: "Duration",
    },

    {
      name: "Encouraging Battlecry",

      effect: `
        When you use this ability, you increase your \\glossterm{fatigue level} by one.

        You and each living \\glossterm{ally} in a \\medarea radius from you each regain 1d10 \\glossterm{damage resistance}.
      `,
      rank: 3,
      scaling: { special: "The recovery increases by +1d for each rank beyond 3." },
      tags: ['Emotion'],
      type: "Instant",
    },

    {
      name: "Greater Encouraging Battlecry",

      effect: `
        When you use this ability, you increase your \\glossterm{fatigue level} by one.

        You and each living \\glossterm{ally} in a \\largearea radius from you each regain 4d6 \\glossterm{damage resistance}.
      `,
      rank: 6,
      scaling: { special: "The recovery increases by +1d for each rank beyond 6." },
      tags: ['Emotion'],
      type: "Instant",
    },

    {
      name: "Steadfast Battlecry",

      effect: `
        You and each living \\glossterm{ally} in a \\hugearea radius from you \\glossterm{briefly} gain a +1 bonus to \\glossterm{vital rolls}.
      `,
      rank: 1,
      scaling: {
        3: "The bonus increases to +2.",
        5: "The bonus increases to +3.",
        7: "The bonus increases to +4.",
      },
      tags: ['Emotion'],
      type: "Duration",
    },

    {
      name: "Enraging Roar",

      attack: {
        crit: `The effect becomes a \\glossterm{condition} on each target.`,
        hit: `
          Each target is \\glossterm{briefly} unable to take any \\glossterm{standard actions} that do not cause it to make an attack.
          For example, it could make a \\glossterm{strike} or cast an offensive spell, but it could not heal itself or summon a creature.
        `,
        targeting: `
          Make an attack vs. Mental against all \\glossterm{enemies} in a \\smallarea radius from you.
        `,
      },
      rank: 2,
      scaling: "accuracy",
      tags: ['Emotion'],
      type: "Duration",
    },

    {
      name: "Greater Enraging Roar",

      functionsLike: {
        name: 'enraging roar',
        exceptThat: "the area increases to a \\largearea radius.",
      },
      scaling: "accuracy",
      rank: 5,
      tags: ["Emotion"],
      type: "Duration",
    },
  ],
};
