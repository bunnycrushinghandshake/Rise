import { CombatStyle } from '.';

export const mobileAssault: CombatStyle = {
  name: 'Mobile Assault',
  shortDescription: 'Move around the battlefield with ease to avoid threats or hunt weak foes.',

  maneuvers: [
    {
      name: 'Reaping Harvest',

      effect: `
        Move up to half your movement speed in a straight line.
        You can make a melee \\glossterm{strike} using a slashing or bludgeoning weapon.
        The strike targets all \\glossterm{enemies} adjacent to you at any point during your movement.
        On a miss, you get a \\glossterm{glancing blow}.
      `,
      rank: 3,
    },

    {
      name: 'Reaping Harvest+',

      effect: `
        Move up to your movement speed in a straight line.
        You can make a melee \\glossterm{strike} using a slashing or bludgeoning weapon.
        The strike deals double \\glossterm{weapon damage}, and it targets all \\glossterm{enemies} adjacent to you at any point during your movement.
        On a miss, you get a \\glossterm{glancing blow}.
      `,
      rank: 7,
    },

    {
      name: 'Spring Attack',

      effect: `
        Move up to half your movement speed and make a \\glossterm{strike} with a -2 accuracy penalty.
        After making the strike, you can use the other half of your movement.
      `,
      rank: 3,
    },

    {
      name: "Wanderer's Strike",

      effect: `
        You can move up to half your speed and make a \\glossterm{weak strike}, in either order.
      `,
      rank: 1,
    },

    {
      name: "Wanderer's Strike+",

      effect: `
        You can move up your speed and make a \\glossterm{strike} with a +1 accuracy bonus, in either order.
      `,
      rank: 5,
    },

    {
      name: 'Push Through',

      effect: `
        Make a melee \\glossterm{strike}.
        You can \\glossterm{briefly} move through the space of each creature damaged by the strike.
      `,
      rank: 1,
    },

    {
      name: 'Sprinting Charge',

      functionsLike: {
        abilityType: 'ability',
        exceptThat: `
          you can move up to twice your speed instead of up to your speed, and the defense penalty is removed.
          When you use this ability, you increase your \\glossterm{fatigue level} by one.
        `,
        name: 'charge',
      },
      rank: 1,
    },

    {
      name: 'Prepared Sprint',

      effect: `
        Your movement speed is \\glossterm{briefly} doubled.
        However, you also cannot use the \\textit{sprint} ability during that time.
      `,
      rank: 1,
    },

    {
      name: 'Retreating Strike',

      effect: `
        Make a melee \\glossterm{strike} with a -2 accuracy penalty.
        In addition, you can move up to your movement speed in a straight line away from one target of the strike.
      `,
      rank: 3,
    },

    {
      name: 'Flash Sweep',

      effect: `
        You \\glossterm{teleport} horizontally to a location within \\shortrange.
        In addition, you can make a melee \\glossterm{strike} that targets all \\glossterm{enemies} within a 5 ft.\\ wide line between your starting location and your ending location.
        You cannot use this ability if you have any \\glossterm{encumbrance}.
      `,
      rank: 5,
    },

    {
      name: 'Flash Flurry',

      effect: `
        You \\glossterm{teleport} horizontally to a location within \\shortrange.
        In addition, you can two melee \\glossterm{strikes}.
        Each strike targets one creature within a 5 ft.\\ wide line between your starting location and your ending location.
        You can target the same creature with both strikes.
        You cannot use this ability if you have any \\glossterm{encumbrance}.
      `,
      rank: 7,
    },

    {
      name: 'Flash Charge',

      effect: `
        You \\glossterm{teleport} horizontally to a location within \\shortrange.
        In addition, you can make a melee \\glossterm{strike} at your destination.
        You cannot use this ability if you have any \\glossterm{encumbrance}.
      `,
      rank: 3,
    },

    {
      name: 'Leaping Strike',

      effect: `
        You make a long jump or high jump and move as normal for the jump (see \\pcref{Jump}).
        You can make a melee \\glossterm{strike} from any location you occupy during the motion, including both your initial leap and your fall afterwards (if any).
      `,
      rank: 3,
    },

    {
      name: 'Leaping Impact Strike',

      effect: `
        You make a long jump or high jump and move as normal for the jump (see \\pcref{Jump}).
        You can make a melee \\glossterm{strike} from any location you occupy during the motion, including both your initial leap and your fall afterwards (if any).
        Your \\glossterm{weapon damage} with the strike is doubled.
        If you hit with the strike, the target takes half of the \\glossterm{falling damage} that you would normally take based on the height of the jump, ignoring any of your abilities that reduce that damage.
      `,
      rank: 7,
    },

    {
      name: 'Vault Over',

      effect: `
        Make an attack vs. Reflex against one creature adjacent to you that is no more than one size category larger than you.
        Your \\glossterm{accuracy} with this attack is equal to your Jump skill.
        If you hit, you leap up over the target's body, using its body as a springboard if necessary, and land in any space adjacent to it.
        % TODO: wording
        Your final destination cannot be more distant from your starting location than half your \\glossterm{land speed}.
        If this attack succeeds, you can make a \\glossterm{strike} from any location you occupy during the leap.
      `,
      rank: 1,
    },

    {
      name: 'Passing Splitstrike',

      effect: `
        Make a melee \\glossterm{strike} with a -1 accuracy penalty, then move up to 10 feet and make another melee \\glossterm{strike} with a -1 accuracy penalty.
        You cannot include the same creature or object as a target of both strikes.
      `,
      rank: 3,
    },

    {
      name: 'Unbalancing Backstep',

      effect: `
        Choose a creature you can see.
        You can move up to 5 feet away from that creature.
        In addition, you gain a +2 bonus to your Armor and Reflex defenses this round.
        Whenever that creature misses or \\glossterm{glances} you with a melee \\glossterm{strike} this round, it takes a -2 penalty to Armor defense during the next round.
        As normal, this bonus does not stack with itself, even if the same creature misses you with multiple melee attacks.

        The defense bonus and reactive penalties are \\abilitytag{Swift} effects, but not the movement.
      `,
      rank: 1,
      tags: ['Swift (see text)'],
    },

    {
      name: 'Fearsome Pounce',

      effect: `
        Move up to half your movement speed and make a melee \\glossterm{strike}.
        Each creature that loses \\glossterm{hit points} from the strike is \\shaken by you as a \\glossterm{condition}.
      `,
      rank: 3,
      tags: ['Emotion'],
    },

    {
      name: 'Fearsome Pounce+',

      effect: `
        Move up to half your movement speed and make a melee \\glossterm{strike} with double \\glossterm{weapon damage}.
        Each creature that loses \\glossterm{hit points} from the strike is \\frightened by you as a \\glossterm{condition}.
      `,
      rank: 7,
      tags: ['Emotion'],
    },

    {
      name: 'Frenzied Charge',

      functionsLike: {
        abilityType: 'ability',
        exceptThat: `
          you gain a +2 accuracy bonus with the strike if you moved at least 15 feet during the charge.
          However, the penalty to your defenses increases to \\minus4.
        `,
        name: 'charge',
      },
      rank: 3,
    },
  ],
};
