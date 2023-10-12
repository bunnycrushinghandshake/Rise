import { CombatStyle } from '.';

export const ripAndTear: CombatStyle = {
  name: 'Rip and Tear',
  shortDescription: 'Rip foes apart with slashing weapons.',

  maneuvers: [
    {
      name: 'Strip the Flesh',

      effect: `
        Make a \\glossterm{strike} with double \\glossterm{weapon damage} using a slashing weapon.
        If the target loses hit points, it becomes \\vulnerable to all damage as a \\glossterm{condition}.
      `,
      rank: 7,
    },

    {
      name: 'Rend the Hide',

      effect: `
        Make a \\glossterm{strike} using a slashing weapon.
        The strike deals 1d6 \\glossterm{extra damage} per 4 power (minimum 1d6) if the target is not wearing metal body armor.
      `,
      rank: 3,
    },

    {
      name: 'Rend the Hide+',

      effect: `
        Make a \\glossterm{strike} using a slashing weapon.
        The strike deals 1d10 \\glossterm{extra damage} per 3 power if the target is not wearing metal body armor.
      `,
      rank: 7,
    },

    {
      name: 'Brow Gash',

      effect: `
        Make a \\glossterm{strike} using a slashing weapon.
        If the target loses hit points, it is \\dazzled as a \\glossterm{condition}.
      `,
      rank: 1,
    },

    {
      name: 'Blinding Brow Gash',

      effect: `
        Make a \\glossterm{strike} with double \\glossterm{weapon damage} using a slashing weapon.
        If the target loses hit points, it is \\blinded as a \\glossterm{condition}.
      `,
      rank: 7,
    },

    {
      name: 'Hamstring',

      effect: `
        Make a melee \\glossterm{weak strike} using a slashing weapon.
        If the target loses hit points, it is \\slowed as a \\glossterm{condition}.
      `,
      rank: 1,
    },

    {
      name: 'Two-Weapon Rend',

      // 1d6 +1 per 2 power is about the same damage as an extra strike, so about 50%
      // more damage compared to hitting with two weapons.
      effect: `
        Make a melee strike using two slashing weapons (see \\pcref{Dual Wielding}).
        If the target takes damage from both weapons, it bleeds.
        A bleeding creature takes 1d6 slashing damage +1 per two \\glossterm{power} during your next action.
      `,
      rank: 1,
    },

    {
      name: 'Two-Weapon Rend+',

      // Unclear if correct damage
      effect: `
        Make a melee strike using two slashing weapons (see \\pcref{Dual Wielding}).
        Each target that takes damage from both weapons bleeds.
        A bleeding creature takes 1d6 slashing damage per 3 \\glossterm{power} during your next action.
      `,
      rank: 5,
    },

    {
      name: 'Flintspark Strike',

      effect: `
        Make a strike using a slashing weapon.
        Damage dealt by the strike is fire damage in addition to its normal damage types.
        If the target takes damage and your attack result beats its Reflex defense, it takes 1d6 fire damage per 4 power (minimum 1d6) during your next action.
      `,
      rank: 3,
    },

    {
      name: 'Flintspark Strike+',

      effect: `
        Make a strike with double \\glossterm{weapon damage} using a slashing weapon.
        Damage dealt by the strike is fire damage in addition to its normal damage types.
        If your attack result beats a target's Reflex defense, the target takes 1d10 fire damage per 3 power during your next action.
      `,
      rank: 7,
    },

    {
      name: 'Sweeping Strike',

      effect: `
        Make a melee \\glossterm{strike} using a slashing weapon.
        The strike gains the \\glossterm{Sweeping} (1) tag, or you gain a +1 bonus to the Sweeping value if it already had that tag.
        This allows the strike to hit an additional target (see \\pcref{Weapon Tags}).
      `,
      rank: 1,
    },

    {
      name: 'Sweeping Throw',

      effect: `
        Make a thrown \\glossterm{strike} using a slashing weapon.
        The strike also targets an additional creature or object within 10 feet of the strike's primary target.
      `,
      rank: 1,
    },

    {
      name: 'Ricochet',

      effect: `
        Make a thrown \\glossterm{strike} using a slashing or bludgeoning weapon against up to three creatures or objects in a \\smallarea radius within \\medrange of you.
        Each target must be within your maximum \\glossterm{range limit} with your weapon, and you take the normal longshot penalty for attacking a creature at long range (see \\pcref{Weapon Range Limits}).
        If you choose yourself as one of the targets, you can catch the weapon instead of taking damage from it.
      `,
      rank: 3,
    },

    {
      name: 'Ricochet+',

      effect: `
        Make a thrown \\glossterm{strike} using a slashing or bludgeoning weapon against up to five creatures or objects within \\medrange of you.
        You can choose the same target multiple times, but not twice in a row, and no more than twice total.
        If you choose the same target twice, it takes double damage from the attack.

        Each target must be within your maximum \\glossterm{range limit} with your weapon, and you take the normal longshot penalty for attacking a creature at long range (see \\pcref{Weapon Range Limits}).
        If you choose yourself as one of the targets, you can catch the weapon instead of taking damage from it.
      `,
      rank: 7,
    },

    {
      name: 'Desperate Bloodwhirl',

      effect: `
        After you use this ability, you increase your \\glossterm{fatigue level} by one.

        Make a melee \\glossterm{strike} using a slashing weapon.
        The strike targets all \\glossterm{enemies} adjacent to you.
        On a miss, you get a \\glossterm{glancing blow}.
        Each creature that loses \\glossterm{hit points} takes damage from the strike again during your next action.
        You cannot use the \\textit{desperate exertion} ability to affect this strike.
      `,
      rank: 1,
    },

    {
      name: 'Bloodletting Strike',

      effect: `
        Make a \\glossterm{weak strike} using a slashing weapon.
        If the target loses hit points, it takes damage from the strike again during your next action.
      `,
      rank: 1,
    },

    {
      name: 'Bloodletting Strike+',

      effect: `
        Make a strike using a slashing weapon.
        If the target loses hit points, it takes damage from the strike again during each of your next two actions.
      `,
      rank: 5,
    },

    {
      name: 'Spinning Slash',

      effect: `
        Make a melee \\glossterm{strike} using a slashing weapon.
        The strike targets all \\glossterm{enemies} adjacent to you.
        On a miss, you get a \\glossterm{glancing blow}.
      `,
      rank: 3,
    },

    {
      name: 'Slash and Return',

      effect: `
        Make two \\glossterm{strikes} using a slashing weapon.
        You gain a +1 accuracy bonus with the first strike, and a -3 accuracy penalty with the second strike.
      `,
      rank: 5,
    },

    {
      name: 'Tear Exposed Flesh',

      effect: `
        Make a \\glossterm{strike} using a slashing weapon.
        If the target has no remaining \\glossterm{damage resistance}, the strike deals 1d8 +1 per two power \\glossterm{extra damage}.
      `,
      // narrative: '',
      rank: 3,
    },

    {
      name: 'Tear Exposed Flesh+',

      effect: `
        Make a \\glossterm{strike} with double \\glossterm{weapon damage} using a slashing weapon.
        If the target has no remaining \\glossterm{damage resistance}, the strike deals 1d10 \\glossterm{extra damage} per 3 power.
      `,
      // narrative: '',
      rank: 7,
    },
  ],
};
