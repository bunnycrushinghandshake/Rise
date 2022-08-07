import { CombatStyle } from ".";

export const ripAndTear: CombatStyle = {
  name: "Rip and Tear",
  shortDescription: "Rip foes apart with slashing weapons.",

  maneuvers: [
    {
      name: "Strip the Flesh",

      effect: `
        Make a \\glossterm{strike} using a slashing weapon.
        You do not add your \\glossterm{power} to damage with the strike.
        Each creature that loses \\glossterm{hit points} from the strike becomes \\glossterm{briefly} \\glossterm{vulnerable} to all damage.
      `,
      rank: 3,
      scaling: {
        5: "You gain a +1 accuracy bonus with the strike.",
        7: "The accuracy bonus increases to +2.",
      },
    },

    {
      name: "Greater Strip the Flesh",

      effect: `
        Make a \\glossterm{strike} using a slashing weapon.
        You do not add your \\glossterm{power} to damage with the strike.
        Each creature that loses \\glossterm{hit points} from the strike becomes \\glossterm{vulnerable} to all damage as a \\glossterm{condition}.
      `,
      rank: 7,
    },

    {
      name: "Strip the Armor",

      effect: `
        Make a \\glossterm{strike} using a slashing weapon.
        Each creature that resists all damage from the strike \\glossterm{briefly} takes a -2 penalty to Armor defense.
      `,
      rank: 3,
      scaling: {
        5: "You gain a +1 accuracy bonus with the strike.",
        7: "The accuracy bonus increases to +2.",
      },
    },

    {
      name: "Rend the Hide",

      effect: `
        Make a \\glossterm{strike} using a slashing weapon.
        You gain a +1 \\glossterm{accuracy} bonus with the strike against creatures that are not wearing armor.
      `,
      rank: 2,
      scaling: {
        4: "The accuracy bonus increases to +2.",
        6: "The accuracy bonus increases to +3.",
      },
    },

    {
      name: "Brow Gash",

      effect: `
        Make a \\glossterm{strike} using a slashing weapon.
        Your \\glossterm{power} with the strike is halved.
        Each creature that loses \\glossterm{hit points} from the strike is \\dazzled as a \\glossterm{condition}.
      `,
      rank: 1,
      scaling: {
        3: "You gain a +1 accuracy bonus with the strike.",
        5: "The accuracy bonus increases to +2.",
        7: "The accuracy bonus increases to +3.",
      },
    },

    {
      name: "Bloody Brow Gash",

      effect: `
        Make a \\glossterm{strike} using a slashing weapon.
        Your \\glossterm{power} with the strike is halved.
        Each creature that loses \\glossterm{hit points} from the strike is \\dazzled as a \\glossterm{condition}.
        In addition, it \\glossterm{briefly} takes damage equal to your \\glossterm{power} at the end of each round.
      `,
      rank: 5,
      scaling: {
        7: "You gain a +1 accuracy bonus with the strike.",
      },
    },

    {
      name: "Blinding Brow Gash",

      effect: `
        Make a \\glossterm{strike} using a slashing weapon.
        You do not add your \\glossterm{power} to damage with the strike.
        Each creature that loses \\glossterm{hit points} from the strike is \\blinded as a \\glossterm{condition}.
      `,
      rank: 7,
    },

    {
      name: "Hamstring",

      effect: `
        Make a \\glossterm{strike} using a slashing weapon.
        Your \\glossterm{power} with the strike is halved.
        Each creature that loses \\glossterm{hit points} from the strike is \\slowed as a \\glossterm{condition}.
      `,
      rank: 1,
      scaling: {
        3: "You gain a +1 accuracy bonus with the strike.",
        5: "The accuracy bonus increases to +2.",
        7: "The accuracy bonus increases to +3.",
      },
    },

    {
      name: "Greater Hamstring",

      effect: `
        Make a \\glossterm{strike} using a slashing weapon.
        Your \\glossterm{power} with the strike is halved.
        Each creature damaged by the strike is \\slowed as a \\glossterm{condition}.
      `,
      rank: 5,
    },

    {
      name: "Two-Weapon Rend",

      effect: `
        Make a melee strike using a slashing weapon.
        At the end of this phase, each creature that you hit during this phase with both this strike and the \\textit{offhand strike} ability takes slashing damage equal to half your \\glossterm{power} (minimum 1).
      `,
      rank: 1,
      scaling: {
        3: "You gain a +1 accuracy bonus with the strike.",
        5: "The accuracy bonus increases to +2.",
        7: "The accuracy bonus increases to +3.",
      },
    },

    {
      name: "Greater Two-Weapon Rend",

      effect: `
        Make a melee strike using a slashing weapon.
        At the end of this phase, each creature that you hit during this phase with both this strike and the \\textit{offhand strike} ability takes slashing damage equal to your \\glossterm{power}.
      `,
      rank: 4,
      scaling: {
        6: "You gain a +1 accuracy bonus with the strike.",
      },
    },

    {
      name: "Flintspark Strike",

      effect: `
        Make a strike with a +2 damage bonus using a slashing weapon.
        If the target is wearing metal armor or is significantly composed of metal, damage dealt by the strike is fire damage in addition to its normal damage types.
      `,
      rank: 3,
      scaling: {
        5: "The damage bonus increases to +4.",
        7: "The damage bonus increases to +8.",
      },
    },

    {
      name: "Sweeping Strike",

      effect: `
        Make a melee \\glossterm{strike} using a slashing weapon.
        The strike gains the \\glossterm{Sweeping} (1) tag, or you gain a +1 bonus to the Sweeping value if it already had that tag (see \\pcref{Weapon Tags}).
      `,
      rank: 2,
      scaling: {
        4: "You gain a +2 damage bonus with the strike.",
        6: "The damage bonus increases to +4.",
      },
    },

    {
      name: "Sweeping Throw",

      effect: `
        Make a thrown \\glossterm{strike} using a slashing weapon.
        The strike also targets an additional creature or object within 10 feet of the strike's primary target.
      `,
      rank: 2,
      scaling: {
        4: "You gain a +2 damage bonus with the strike.",
        6: "The damage bonus increases to +4.",
      },
    },

    {
      name: "Greater Sweeping Throw",

      effect: `
        Make a thrown \\glossterm{strike} using a slashing weapon.
        The strike also targets up to two additional creatures or object within 10 feet of the strike's primary target.
      `,
      rank: 5,
      scaling: {
        7: "You gain a +4 damage bonus with the strike.",
      },
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
    },

    // is 2x power equal to +5-10 damage? sometimes better, sometimes worse, especially with the
    // delay and limited stacking, so seems fine
    {
      name: "Bloodletting Strike",

      effect: `
        Make a \\glossterm{strike} using a slashing weapon.
        Each creature damaged by the strike \\glossterm{briefly} bleeds from the wound.
        It takes physical damage equal to half your \\glossterm{power} with that strike at the end of each round.
        This damage does not stack if you use this strike on the same creature again before it stops bleeding.
      `,
      rank: 5,
      scaling: {
        7: "You gain a +1 accuracy bonus with the strike.",
      },
    },

    {
      name: "Bloodletting Sweep",

      functionsLike: {
        name: 'bloodletting strike',
        exceptThat: 'the strike gains the \\glossterm{Sweeping} (1) tag, or you gain a +1 bonus to the Sweeping value if it already had that tag (see \\pcref{Weapon Tags}).',
      },
      rank: 7,
    },

    {
      name: "Spinning Slash",

      effect: `
        Make a melee \\glossterm{strike} using a slashing weapon.
        The strike targets any number of \\glossterm{enemies} within your \\glossterm{reach} with that weapon.
        Your \\glossterm{power} with the strike is halved.
      `,
      rank: 1,
      scaling: {
        3: "You gain a +1 accuracy bonus with the strike.",
        5: "The accuracy bonus increases to +2.",
        7: "The accuracy bonus increases to +3.",
      },
    },

    {
      name: "Greater Spinning Slash",

      effect: `
        Make a melee \\glossterm{strike} using a slashing weapon.
        The strike targets any number of \\glossterm{enemies} within your \\glossterm{reach} with that weapon.
      `,
      rank: 4,
      scaling: {
        6: "You gain a +1 accuracy bonus with the strike.",
      },
    },

    {
      name: "Twinslash",

      effect: `
        Make two melee \\glossterm{strikes} using a slashing weapon.
        You do not add your \\glossterm{power} to damage with either strike.
      `,
      rank: 6,
    },

    {
      name: "Tear Exposed Flesh",

      effect: `
        Make a \\glossterm{strike} using a slashing weapon.
        If the target does not have any remaining \\glossterm{damage resistance}, you gain a +2 damage bonus with the strike.
      `,
      rank: 1,
      scaling: {
        3: "The damage bonus increases to +4.",
        5: "The damage bonus increases to +8.",
        7: "The damage bonus increases to +16.",
      },
    },

    {
      name: "Agonizing Flay",

      effect: `
        Make a \\glossterm{strike} using a slashing weapon.
        Your \\glossterm{power} with the strike is halved.
        Each creature damaged by the strike is \\glossterm{briefly} \\dazed.
      `,
      rank: 3,
      scaling: {
        5: "You gain a +1 \\glossterm{accuracy} bonus with the strike.",
        7: "The accuracy bonus increases to +2.",
      },
    },

    {
      name: "Greater Agonizing Flay",

      effect: `
        Make a \\glossterm{strike} using a slashing weapon.
        Your \\glossterm{power} with the strike is halved.
        Each creature damaged by the strike is \\glossterm{briefly} \\stunned.
      `,
      rank: 7,
    },
  ],
};
