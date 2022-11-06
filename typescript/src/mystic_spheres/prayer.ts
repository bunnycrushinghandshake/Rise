import { MysticSphere } from '.';

export const prayer: MysticSphere = {
  name: 'Prayer',
  shortDescription: 'Grant divine blessings to aid allies and improve combat prowess.',
  sources: ['divine'],

  cantrips: [
    {
      effect: `
        Choose one adjacent \\glossterm{ally}.
        The target gains a +1 bonus to \\glossterm{accuracy} this round.
      `,
      name: 'Boon of Competence',
      scaling: {
        2: 'The range increases to \\shortrange.',
        4: 'The bonus increases to +2.',
        6: 'The range increases to \\medrange.',
      },
    },
    {
      effect: `
        Choose one adjacent \\glossterm{ally}.
        The target gains a +1 bonus to \\glossterm{defenses} this round.
        Because this ability has the \\abilitytag{Swift} tag, this improves the target's defenses against attacks made against it during the current phase.
      `,
      name: 'Boon of Protection',
      scaling: {
        2: 'The range increases to \\shortrange.',
        4: 'The bonus increases to +2.',
        6: 'The range increases to \\medrange.',
      },
      tags: ['Swift'],
    },
  ],
  spells: [
    {
      name: 'Blessing of Freedom',

      effect: `
        Choose up to five creatures from among yourself and your \\glossterm{allies} within \\medrange. 
        Each target is immune to being \\slowed, \\immobilized, and \\paralyzed.
      `,
      rank: 4,
      type: 'Attune (target)',
    },

    {
      name: 'Blessing of Recovery',

      effect: `
        Choose up to five creatures from among yourself and your \\glossterm{allies} within \\medrange.
        Whenever each target finishes a \\glossterm{long rest}, it removes an additional \\glossterm{vital wound} (see \\pcref{Removing Vital Wounds}).
      `,
      rank: 4,
      scaling: {
        6: `Each target removes two additional vital wounds instead of only one.`,
      },
      type: 'Attune (target)',
    },

    {
      name: 'Blessing of Regeneration',

      effect: `
        Choose up to five creatures from among yourself and your \\glossterm{allies} within \\medrange.
        At the end of each round, each target regains 2d6 \\glossterm{hit points}.
        This healing cannot increase a target's hit points above half its maximum hit points.
      `,
      rank: 5,
      scaling: {
        special: `
          The healing increases by +1d for each rank beyond 5.
        `,
      },
      type: 'Attune (target)',
    },

    {
      name: 'Blessing of Proficiency',

      effect: `
        Choose up to five creatures from among yourself and your \\glossterm{allies} within \\medrange.
        Each target becomes proficient with one additional weapon group, including exotic weapons from that weapon group.
      `,
      rank: 1,
      type: 'Attune (target)',
    },

    {
      name: 'Boon of Precision',

      effect: `
        Choose yourself or one \\glossterm{ally} within \\medrange.
        The next time the target makes a \\glossterm{strike} this round,
          it gains a +2 bonus to \\glossterm{accuracy} and rolls twice and takes the higher result.
        If you cast this spell on yourself, it affects the first strike you make before the end of the next round.
      `,
      rank: 1,
      scaling: {
        3: `The accuracy bonus increases to +3.`,
        5: `The accuracy bonus increases to +4.`,
        7: `The accuracy bonus increases to +5.`,
      },
      tags: [],
    },

    {
      name: 'Boon of Deadly Fortune',

      effect: `
        Choose yourself or one \\glossterm{ally} within \\medrange.
        The target gains a +4 bonus to \\glossterm{accuracy} this round for the purpose of determining if its attacks get a \\glossterm{critical hit}.
        If you cast this spell on yourself, it affects the first attack you make before the end of the next round.
      `,
      rank: 2,
      scaling: {
        4: `The accuracy bonus increases to +5.`,
        6: `The accuracy bonus increases to +6.`,
      },
      tags: [],
    },

    {
      name: 'Boon of Invulnerability',

      effect: `
        Choose yourself or one \\glossterm{ally} within \\medrange.
        The target becomes takes half damage from all sources this round.
        Because this ability has the \\abilitytag{Swift} tag, it affects all damage the target takes during the current phase.
      `,
      rank: 6,
      tags: ['Swift'],
    },

    {
      name: 'Boon of Avoidance',

      effect: `
        Choose yourself or one \\glossterm{ally} within \\medrange.
        The target gains a +2 bonus to Armor and Reflex defenses this round.
        Because this ability has the \\abilitytag{Swift} tag, this improves the target's defenses against attacks made against it during the current phase.
      `,
      rank: 3,
      scaling: {
        5: `The bonus increases to +3.`,
        7: `The bonus increases to +4.`,
      },
      tags: ['Swift'],
    },

    {
      name: 'Blessing of the Purified Body',

      effect: `
        Choose up to five creatures from among yourself and your \\glossterm{allies} within \\medrange.
        Each target is \\glossterm{impervious} to \\glossterm{poisons} and \\glossterm{diseases}.
        In addition, at the end of each round, it automatically gains one success to resist an active poison or disease affecting it that was not applied during that round.
      `,
      rank: 3,
      type: 'Attune (target)',
    },

    {
      name: 'Blessing of Mental Clarity',

      effect: `
        Choose up to five creatures from among yourself and your \\glossterm{allies} within \\medrange.
        Each target is \\glossterm{impervious} to \\abilitytag{Compulsion} and \\abilitytag{Emotion} effects.
        In addition, at the end of each round, it automatically removes one \\glossterm{condition} from a Compulsion or Emotion effect that was not applied during that round.
      `,
      rank: 3,
      type: 'Attune (target)',
    },

    {
      name: 'Blessing of Potency',

      effect: `
        Choose up to five creatures from among yourself and your \\glossterm{allies} within \\medrange.
        Each target gains a +2 \\glossterm{magic bonus} to \\glossterm{power}.
      `,
      rank: 2,
      scaling: {
        4: `The bonus increases to +4.`,
        6: `The bonus increases to +8.`,
      },
      type: 'Attune (target)',
    },

    {
      name: 'Blessing of Perseverance',

      effect: `
        Choose up to five creatures from among yourself and your \\glossterm{allies} within \\medrange.
        Whenever each target would gain a \\glossterm{condition}, it can choose to negate that condition.
        After a creature negates a condition in this way, this spell ends for that creature.
      `,
      rank: 2,
      scaling: {
        4: `The spell can negate two conditions before ending.`,
        6: `The spell can negate three conditions before ending.`,
      },
      type: 'Attune (deep, target)',
    },

    {
      name: 'Boon of Cleansing',

      effect: `
        You or one \\glossterm{ally} within \\medrange can remove a \\glossterm{condition}.
      `,
      rank: 4,
      scaling: {
        6: `The target can remove two effects.`,
      },
    },

    {
      name: 'Cleansing Benediction',

      effect: `
        You and each \\glossterm{ally} within a \\smallarea radius from you can each remove a \\glossterm{condition}.
      `,
      rank: 6,
    },

    {
      name: 'Blessing of Physical Prowess',

      effect: `
        Choose up to five creatures from among yourself and your \\glossterm{allies} within \\medrange.
        When you cast this spell, choose a physical attribute: Strength, Dexterity, or Constitution.
        Each target gains a +2 \\glossterm{magic bonus} to checks using the chosen attribute.
        In addition, if you choose Strength, each target gains a +1 \\glossterm{magic bonus} to Strength for the purpose of determining its weight limits (see \\pcref{Weight Limits}).
      `,
      rank: 3,
      scaling: {
        5: `The bonus increases to +3.`,
        7: `The bonus increases to +4.`,
      },
      type: 'Attune (target)',
    },

    {
      name: 'Blessing of Endurance',

      effect: `
        Choose up to five creatures from among yourself and your \\glossterm{allies} within \\medrange.
        Each target gains a +4 \\glossterm{magic bonus} to its maximum \\glossterm{hit points}.
        In addition, it immediately gains that many hit points.
        When this ability ends, each target loses \\glossterm{hit points} equal to the number of hit points it gained this way.
      `,
      rank: 2,
      scaling: {
        4: `The bonus increases to +8.`,
        6: `The bonus increases to +16.`,
      },
      type: 'Attune (target)',
    },

    {
      name: 'Blessing of Divine Warning',

      effect: `
        Choose up to five creatures from among yourself and your \\glossterm{allies} within \\medrange.
        Each target is never \\unaware or \\partiallyunaware.
      `,
      rank: 6,
      type: 'Attune (target)',
    },

    {
      name: 'Blessing of Cleansing Renewal',

      effect: `
        Choose up to five creatures from among yourself and your \\glossterm{allies} within \\medrange.
        At the end of each round, each target removes one \\glossterm{condition} of its choice affecting it.
      `,
      rank: 7,
      type: 'Attune (target)',
    },

    {
      name: 'Blessing of Mastery',

      effect: `
        Choose up to five creatures from among yourself and your \\glossterm{allies} within \\medrange.
        Each target gains a +4 \\glossterm{magic bonus} to \\glossterm{hit points} and \\glossterm{damage resistance}.
        In addition, each target gains a +2 \\glossterm{magic bonus} to \\glossterm{power}.
      `,
      rank: 2,
      scaling: {
        4: `
          The bonuses to hit points and damage resistance increase to +8.
          In addition, the bonus to power increases to +4.
        `,
        6: `
          The bonuses to hit points and damage resistance increase to +16.
          In addition, the bonus to power increases to +8.
        `,
      },
      type: 'Attune (deep, target)',
    },

    {
      name: 'Blessing of Resilience',

      effect: `
        Choose up to five creatures from among yourself and your \\glossterm{allies} within \\medrange.
        Each target gains a +4 \\glossterm{magic bonus} to its \\glossterm{damage resistance}.
      `,
      rank: 2,
      scaling: {
        4: `The bonus increases to +8.`,
        6: `The bonus increases to +16.`,
      },
      type: 'Attune (target)',
    },

    {
      name: 'Blessing of Vitality',

      effect: `
        Choose up to five creatures from among yourself and your \\glossterm{allies} within \\medrange.
        Whenever each target would gain a \\glossterm{vital wound}, it can choose to negate that vital wound.
        After negating a vital wound for a creature in this way, this spell ends for that creature.
      `,
      rank: 6,
      type: 'Attune (deep, target)',
    },

    {
      name: 'Boon of Shielding',

      effect: `
        Choose yourself or a living \\glossterm{ally} within \\shortrange.
        The target regains 1d6 + \\glossterm{power} \\glossterm{damage resistance} and increases its \\glossterm{fatigue level} by one.
        In addition, it gains a +1 bonus to all \\glossterm{defenses} this round.
      `,
      rank: 1,
      scaling: { special: 'The recovery increases by +1d for each rank beyond 1.' },
      tags: ['Swift'],
    },

    {
      name: 'Empowered Boon of Shielding',

      effect: `
        Choose yourself or a living \\glossterm{ally} within \\shortrange.
        The target regains 2d10 + \\glossterm{power} \\glossterm{damage resistance} and increases its \\glossterm{fatigue level} by one.
        In addition, it gains a +2 bonus to all \\glossterm{defenses} this round.
      `,
      rank: 5,
      scaling: { special: 'The recovery increases by +1d for each rank beyond 4.' },
      tags: ['Swift'],
    },

    {
      name: 'Consecrated Strike',
      effect: `
        Make a \\glossterm{strike}.
        You may use the higher of your Strength and your Willpower to determine your damage with the strike (see \\pcref{Dice Bonuses From Attributes}).
        If you use the \\ability{desperate exertion} ability to affect this strike, you gain an additional +2 accuracy bonus to the reroll.
        This stacks with the normal +2 bonus from the \\textit{desperate exertion} ability.
      `,
      rank: 1,
      scaling: {
        3: 'The accuracy bonus increases to +3.',
        5: 'The accuracy bonus increases to +4.',
        7: 'The accuracy bonus increases to +5.',
      },
    },

    {
      name: 'Exalted Strike',
      effect: `
        Make a \\glossterm{strike}.
        You may use the higher of your Strength and your Willpower to determine your damage with the strike (see \\pcref{Dice Bonuses From Attributes}).
        You gain a +1 \\glossterm{accuracy} bonus with the strike for each spell from the \\sphere{prayer} \\glossterm{mystic sphere} that you are attuned to, to a maximum of +3.
      `,
      rank: 4,
    },

    {
      name: 'Curse of Fugue',

      attack: {
        crit: `The effect lasts until the curse is removed.`,
        hit: `
          The target is \\dazed until it takes a \\glossterm{short rest}.
        `,
        targeting: `
          Make an attack vs. Mental against one creature within \\medrange.
        `,
      },
      rank: 2,
      scaling: 'accuracy',
      tags: ['Curse'],
    },
    {
      name: 'Intense Curse of Fugue',

      attack: {
        crit: `The effect lasts until the curse is removed.`,
        hit: `
          The target is \\stunned until it takes a \\glossterm{short rest}.
        `,
        targeting: `
          Make an attack vs. Mental against one creature within \\medrange.
        `,
      },
      rank: 6,
      scaling: 'accuracy',
      tags: ['Curse'],
    },
    {
      name: 'Curse of Sloth',

      attack: {
        crit: `The effect lasts until the curse is removed.`,
        hit: `
          While the target is below its maximum \\glossterm{hit points}, it is \\slowed.
          This effect lasts until the target takes a \\glossterm{short rest}.
        `,
        targeting: `
          Make an attack vs. Mental against one creature within \\shortrange.
        `,
      },
      rank: 1,
      scaling: 'accuracy',
      tags: ['Curse'],
    },
    {
      name: 'Efficient Curse of Sloth',

      attack: {
        crit: `The effect lasts until the curse is removed.`,
        hit: `
          The target is \\slowed until it takes a \\glossterm{short rest}.
        `,
        targeting: `
          Make an attack vs. Mental against one creature within \\shortrange.
        `,
      },
      rank: 5,
      scaling: 'accuracy',
      tags: ['Curse'],
    },
    {
      name: 'Curse of Blindness',

      attack: {
        crit: `The effect lasts until the curse is removed.`,
        hit: `
          While the target is below its maximum \\glossterm{hit points}, it is \\blinded.
          This effect lasts until the target takes a \\glossterm{short rest}.
        `,
        targeting: `
          Make an attack vs. Mental against one creature within \\shortrange.
        `,
      },
      rank: 5,
      scaling: 'accuracy',
      tags: ['Curse'],
    },
    {
      name: 'Holy Blades',

      effect: `
        Choose up to five creatures from among yourself and your \\glossterm{allies} within \\medrange.
        Each target's weapons shed light like a torch.
        In addition, all damage they deal with \\glossterm{strikes} becomes energy damage in addition to the attack's normal damage types.
      `,
      rank: 1,
      type: 'Attune (target)',
    },
    {
      name: 'Curse of Flammability',

      attack: {
        crit: `The effect lasts until the curse is removed.`,
        hit: `The target is highly flammable until it takes a \\glossterm{short rest}.
        Like dry wood or kindling, it catches on fire whenever it takes any fire damage.
        While ignited in this way, it takes 1d8 + half \\glossterm{power} fire damage during each of your actions.

        It can put out the fire by making a \\glossterm{difficulty value} 10 Dexterity check as a \\glossterm{movement} to put out the flames.
        Dropping \\prone as part of this action gives a +5 bonus to this check.
        Putting out the flames in this way does not remove this curse.`,
        targeting: `
          Make an attack vs. Mental against one creature within \\medrange.
        `,
      },

      rank: 3,
      scaling: 'accuracy',
      tags: ['Curse'],
    },
  ],
  rituals: [
    {
      name: 'Blessing of Fortification',

      castingTime: 'one hour',
      effect: `
        Choose one \\glossterm{unattended}, nonmagical object or part of an object of up to Large size.
        Unlike most abilities, this ritual can affect individual parts of a whole object.

        % How should this affect Strength break difficulty value?
        The target gains a +10 \\glossterm{magic bonus} to its \\glossterm{damage resistance}.
        If the target is moved, this effect ends.
        Otherwise, it lasts for one year.
        `,
      rank: 1,
      type: 'Attune',
    },

    {
      name: 'Enduring Blessing of Fortification',

      castingTime: '24 hours',
      functionsLike: {
        exceptThat: `
          the effect lasts for one hundred years.
        `,
        name: 'blessing of fortification',
      },
      rank: 3,
    },

    {
      name: 'Enduring Empowered Blessing of Fortification',

      castingTime: '24 hours',
      functionsLike: {
        exceptThat: `
          the effect lasts for one hundred years.
        `,
        name: 'empowered fortification',
      },
      rank: 6,
    },

    {
      name: 'Empowered Blessing of Fortification',

      castingTime: 'one hour',
      functionsLike: {
        exceptThat: `
          the bonus to \\glossterm{damage resistance} increases to +20.
        `,
        name: 'blessing of fortification',
      },
      rank: 4,
      type: 'Attune',
    },

    {
      name: 'Bless Water',

      castingTime: 'one minute',
      effect: `
        One pint of \\glossterm{unattended}, nonmagical water within \\shortrange becomes holy water.
        Holy water can be can be thrown as a splash weapon, dealing 1d8 points of damage to a struck \\glossterm{undead} or an evil \\glossterm{planeforged}.
        `,
      rank: 1,
      type: 'Attune',
    },

    {
      name: 'Permanent Bless Water',

      castingTime: 'one hour',
      functionsLike: {
        exceptThat: `
          it loses the \\abilitytag{Attune} (ritual) tag and the effect lasts permanently.
        `,
        name: 'bless water',
      },
      rank: 3,
    },

    {
      name: 'Curse Water',

      castingTime: 'one minute',
      effect: `
        One pint of \\glossterm{unattended}, nonmagical water within \\shortrange becomes unholy water.
        Unholy water can be can be thrown as a splash weapon, dealing 1d8 points of damage to a struck good \\glossterm{planeforged}.
        `,
      rank: 1,
      type: 'Attune',
    },

    {
      name: 'Permanent Curse Water',

      castingTime: 'one hour',
      functionsLike: {
        exceptThat: `
          it loses the \\abilitytag{Attune} (ritual) tag and the effect lasts permanently.
        `,
        name: 'curse water',
      },
      rank: 3,
    },

    {
      name: 'Blessing of Purification',

      castingTime: 'one hour',
      effect: `
        All food and water in a single square within \\shortrange becomes purified.
        Spoiled, rotten, poisonous, or otherwise contaminated food and water becomes pure and suitable for eating and drinking.
        This does not prevent subsequent natural decay or spoiling.
      `,
      rank: 1,
    },
  ],
};