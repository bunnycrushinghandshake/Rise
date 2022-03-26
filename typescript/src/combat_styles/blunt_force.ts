import { CombatStyle } from ".";

export const bluntForce: CombatStyle = {
  name: "Blunt Force",
  shortDescription: "Smash foes with bludgeoning weapons and raw power.",

  maneuvers: [
    {
      name: "Crushing Strike",

      effect: `
        Make a \\glossterm{strike} using a bludgeoning weapon.
        Your \\glossterm{power} with the strike is halved.
        The attack is made against each target's Fortitude defense instead of its Armor defense.
      `,
      rank: 1,
      scaling: {
        3: "You gain a +2 damage bonus with the strike.",
        5: "The damage bonus increases to +4.",
        7: "The damage bonus increases to +8.",
      },
      type: "Instant",
    },

    {
      name: "Ground Stomp",

      attack: {
        hit: `Each target takes 1d8 + half \\glossterm{power} bludgeoning damage.`,
        targeting: `
          Make an attack vs. Reflex against everything adjacent to you that is on the same stable surface as you.
        `,
      },
      scaling: "damage",
      rank: 2,
      type: "Instant",
    },

    {
      name: "Greater Ground Stomp",

      attack: {
        hit: `
          Each target takes 2d6 + half \\glossterm{power} bludgeoning damage.
        `,
        targeting: `
          Make an attack vs. Reflex against everything in a \\medarea radius from you that is on the same stable surface as you.
        `,
      },
      rank: 4,
      scaling: "damage",
      type: "Instant",
    },

    {
      name: "Supreme Ground Stomp",

      attack: {
        hit: `
          Each target takes 2d10 + half \\glossterm{power} bludgeoning damage.
          Each creature that loses \\glossterm{hit points} from this damage is knocked \\prone.
        `,
        targeting: `
          Make an attack vs. Reflex against everything in a \\medarea radius from you that is on the same stable surface as you.
        `,
      },
      rank: 6,
      scaling: "damage",
      type: "Instant",
    },

    {
      name: "Ground Slam",

      effect: `
        Make a melee \\glossterm{strike} using a bludgeoning weapon against a stable surface.
        The strike targets everything supported by that surface in a \\smallarealong, 5 ft. wide line from you.
        All damage dealt by this attack is bludgeoning damage instead of its normal types.
      `,
      rank: 3,
      scaling: {
        5: "You gain a +1 accuracy bonus with the strike.",
        7: "The accuracy bonus increases to +2.",
      },
      type: "Instant",
    },

    {
      name: "Greater Ground Slam",

      effect: `
        Make a melee \\glossterm{strike} using a bludgeoning weapon against a stable surface.
        The strike targets everything on that surface in a \\medarealong, 10 ft. wide line from you.
        All damage dealt by this attack is bludgeoning damage instead of its normal types.
      `,
      rank: 6,
      type: "Instant",
    },

    {
      name: "Resonating Strike",

      effect: `
        Make a strike with a +2 damage bonus using a bludgeoning weapon.
        Damage dealt by the strike is sonic damage in addition to its normal damage types.
      `,
      rank: 3,
      scaling: {
        5: "The damage bonus increases to +4.",
        7: "The damage bonus increases to +8.",
      },
      type: "Instant",
    },

    {
      name: "Headshot",

      effect: `
        Make a \\glossterm{strike} using a bludgeoning weapon.
        Your \\glossterm{power} with the strike is halved.
        Each creature that loses \\glossterm{hit points} from the strike is \\dazed as a \\glossterm{condition}.
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
      name: "Greater Headshot",

      effect: `
        Make a \\glossterm{strike} using a bludgeoning weapon.
        Your \\glossterm{power} with the strike is halved.
        Each creature damaged by the strike is \\dazed as a \\glossterm{condition}.
      `,
      rank: 5,
      scaling: {
        7: "You gain a +1 accuracy bonus with the strike.",
      },
      type: "Duration",
    },

    {
      name: "Stunning Smash",

      effect: `
        Make a \\glossterm{strike} using a bludgeoning weapon.
        You do not add your \\glossterm{power} to damage with the strike.
        Each creature that loses \\glossterm{hit points} from the strike is \\stunned as a \\glossterm{condition}.
      `,
      rank: 3,
      scaling: {
        5: "You gain a +1 accuracy bonus with the strike.",
        7: "The accuracy bonus increases to +2.",
      },
      type: "Duration",
    },

    {
      name: "Head-Spinning Smash",

      effect: `
        Make a \\glossterm{strike} using a bludgeoning weapon.
        You do not add your \\glossterm{power} to damage with the strike.
        Each creature that loses \\glossterm{hit points} from the strike is \\confused as a \\glossterm{condition}.
      `,
      rank: 7,
      type: "Duration",
    },

    {
      name: "Leap Slam",

      attack: {
        hit: `Each target takes 2d6 + half \\glossterm{power} bludgeoning damage.`,
        targeting: `
          You make a long jump or high jump and move as normal for the jump (see \\pcref{Jump}).
          When you land, if the vertical distance in feet between the highest point of your leap and your landing point was at least ten feet, you emit a small shockwave.
          If you do, make an attack vs. Reflex against everything adjacent to you that is on the same stable surface as you.
        `,
      },
      scaling: "damage",
      rank: 4,
      type: "Instant",
    },

    {
      name: "Ricochet",

      effect: `
        Make a thrown \\glossterm{strike} using a slashing or bludgeoning weapon against up to three creatures or objects within \\shortrange of you.
        Each target must be within your maximum \\glossterm{range limit} with your weapon, and you take the normal longshot penalty for attacking a creature at long range (see \\pcref{Weapon Range Limits}).
        If you choose yourself as one of the targets, you can catch the weapon instead of taking damage from it.
      `,
      rank: 4,
      scaling: {
        6: "The maximum range increases to \\medrange.",
      },
      type: "Instant",
    },

    {
      name: "Knockdown",

      effect: `
        Make a \\glossterm{strike} using a bludgeoning weapon.
        The attack is made against each target's Fortitude defense instead of its Armor defense.
        Your \\glossterm{power} with the strike is halved.
        Each creature damaged by the strike falls \\prone if it is no larger than one size category larger than you.
        A creature knocked prone in this way cannot be knocked prone by this effect again until it takes a \\glossterm{short rest}.
      `,
      rank: 1,
      scaling: {
        3: "You gain a +1 accuracy bonus with the strike.",
        5: "The accuracy bonus increases to +2.",
        7: "The accuracy bonus increases to +3.",
      },
      type: "Instant",
    },

    {
      name: "Forceful Strike",

      effect: `
        Make a \\glossterm{strike} using a bludgeoning weapon.
        The strike gains the \\glossterm{Forceful} weapon tag (see \\pcref{Weapon Tags}).
        If it already has that weapon tag, the maximum size category of creature that you can \\glossterm{knockback} with that tag increases by one size category, and the knockback distance increases by 10 feet.
      `,
      rank: 2,
      scaling: {
        4: "The bonus knockback distance increases to 20 feet.",
        6: "The bonus knockback distance increases to 30 feet.",
      },
      type: "Instant",
    },

    {
      name: "Press Forward",

      effect: `
        Make a melee \\glossterm{strike} using a bludgeoning weapon.
        After you make the strike, you can move up to 10 feet.
        During this movement, you can move into space occupied by creatures that you damaged with the strike.
        When you do, you \\glossterm{push} that creature up to 5 feet in any direction.
        You cannot push creatures two or more size categories larger than you with this ability.
      `,
      rank: 2,
      scaling: {
        4: "You gain a +1 accuracy bonus with the strike.",
        6: "The accuracy bonus increases to +2.",
      },
      type: "Instant",
    },

    {
      name: "Greater Press Forward",

      functionsLike: {
        exceptThat: "you can move up to your full movement speed instead of only 10 feet.",
        name: "press forward",
      },
      rank: 5,
      type: "Instant",
    },

    {
      name: "Desperate Smash",

      effect: `
        After you use this ability, you increase your \\glossterm{fatigue level} by one.

        Make a \\glossterm{strike} with a +4 damage bonus using a bludgeoning weapon.
        You cannot use the \\textit{desperate exertion} ability to affect this strike.
      `,
      rank: 1,
      scaling: {
        3: "The damage bonus increases to +8.",
        5: "The damage bonus increases to +16.",
        7: "The damage bonus increases to +24.",
      },
      type: "Instant",
    },

    {
      name: "Desperate Crush",

      effect: `
        After you use this ability, you increase your \\glossterm{fatigue level} by one.

        Make a \\glossterm{strike} with a +4 damage bonus using a bludgeoning weapon.
        The attack is made against each target's Fortitude defense instead of its Armor defense.
        You cannot use the \\textit{desperate exertion} ability to affect this strike.
      `,
      rank: 2,
      scaling: {
        4: "The damage bonus increases to +8.",
        6: "The damage bonus increases to +16.",
      },
      type: "Instant",
    },

    {
      name: "Tenderizing Smash",

      effect: `
        Make a \\glossterm{strike} using a bludgeoning weapon.
        Each creature damaged by the strike is \\glossterm{briefly} \\dazed.
        After it stops being dazed, it is immune to being dazed in this way until it takes a \\glossterm{short rest}.
      `,
      rank: 3,
      scaling: {
        5: "You gain a +1 accuracy bonus with the strike.",
        7: "The accuracy bonus increases to +2.",
      },
      type: "Duration",
    },

    {
      name: "Greater Tenderizing Smash",

      effect: `
        Make a \\glossterm{strike} using a bludgeoning weapon.
        Each creature damaged by the strike is \\glossterm{briefly} \\stunned.
        After it stops being stunned, it is immune to being stunned in this way until it takes a \\glossterm{short rest}.
      `,
      rank: 7,
      type: "Duration",
    },

    {
      name: "Boneshatter",

      // baseline is +2d at this level, could be +3d due to delay
      effect: `
        Make a \\glossterm{strike} using a bludgeoning weapon.
        Your \\glossterm{power} with the strike is halved.
        Each creature damaged by the strike \\glossterm{briefly} takes half the damage from the strike again at the end of each round.
      `,
      rank: 5,
      scaling: {
        7: "You gain a +4 damage bonus with the strike.",
      },
      type: "Duration",
    },

    {
      name: "Stomach-Churning Strike",

      effect: `
        Make a \\glossterm{strike}.
        Your \\glossterm{power} with the strike is halved.
        Each creature damaged by the strike \\glossterm{briefly} takes a -2 penalty to Fortitude defense.
      `,
      rank: 1,
      scaling: {
        3: "The penalty increases to -3.",
        5: "The penalty increases to -4.",
        7: "The penalty increases to -3.",
      },
      type: "Instant",
    },

    {
      name: "Resonant Smash",

      // -1 rank for self-deafen
      effect: `
        Make a melee \\glossterm{strike}.
        Your \\glossterm{power} with the strike is halved.
        You and each creature damaged by the strike are \\glossterm{briefly} \\deafened.
        This ability does not have the \\abilitytag{Swift} tag, so it does not affect abilities used by you or any damaged creatures during the current phase.
      `,
      rank: 2,
      scaling: {
        4: "You gain a +1 accuracy bonus with the strike.",
        6: "The accuracy bonus increases to +2.",
      },
      type: "Duration",
    },
  ],
};
