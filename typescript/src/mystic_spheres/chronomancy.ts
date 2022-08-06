import { MysticSphere } from ".";

export const chronomancy: MysticSphere = {
  name: "Chronomancy",
  shortDescription: "Manipulate the passage of time to inhibit foes and aid allies.",
  sources: ["arcane", "pact"],

  cantrips: [
    {
      name: "Accelerated Reading",

      effect: `
        You can read at twice your normal speed.
        However, the mental effort imposes a -4 penalty to Mental defense.
      `,
      scaling: {
        2: "You can read at four times your normal speed.",
        4: "You can read at six times your normal speed.",
        6: "You can read at ten times your normal speed.",
      },
      type: "Sustain (free)",
    },
    {
      name: "Accelerated Search",

      effect: `
        Make an Awareness check to search everything in a \\smallarea radius from you (see \\pcref{Search}).
        
      `,
      scaling: {
        2: "You gain a +2 bonus to this check in addition to the normal +5 bonus for searching carefully.",
        4: "The bonus increases to +3.",
        6: "The bonus increases to +4.",
      },
    },
    {
      name: "Rapid Aging",

      effect: `
        Choose one Large or smaller \\glossterm{unattended}, nonmagical object within \\medrange.
        In addition, choose any number of hours, up to 24 hours.
        The target ages as if that much time had passed.
        When this spell ends, the object returns to its original state.
      `,
      scaling: {
        2: "You can choose to age the target by up to a week.",
        4: "You can choose to age the target by up to a month.",
        6: "You can choose to age the target by up to three months.",
      },
      type: "Sustain (minor)",
    },
  ],
  spells: [
    {
      name: "Accelerated Twinstrike",

      effect: `
        This spell has no \\glossterm{somatic components}.

        Make a \\glossterm{strike}.
        You may reroll the accuracy roll and take the highest result.
        However, you do not add your \\glossterm{power} to damage with the strike.
      `,
      rank: 1,
      scaling: {
        3: "You gain a +1 accuracy bonus with the strike.",
        5: "The accuracy bonus increases to +2.",
        7: "The accuracy bonus increases to +3.",
      },
      tags: [],
    },
    {
      name: "Accelerated Triplestrike",

      effect: `
        This spell has no \\glossterm{somatic components}.

        Make a \\glossterm{strike}.
        You may reroll the accuracy roll twice and take the highest result.
        However, you do not add your \\glossterm{power} to damage with the strike.
      `,
      rank: 5,
      scaling: {
        7: "You gain a +1 accuracy bonus with the strike.",
      },
    },

    {
      name: "Slowing Curse",

      attack: {
        crit: `The effect lasts until the curse is removed.`,
        hit: `The target is \\slowed until it takes a \\glossterm{short rest}.`,
        targeting: `
          Make an attack vs. Mental against one creature within \\medrange.
        `,
      },
      rank: 3,
      scaling: "accuracy",
      tags: ["Curse"],
    },

    // This is a very unique combination of triggers and effects, so correct rank is hard
    {
      name: "Curse of Temporal Dislocation",

      attack: {
        crit: `The effect lasts until the curse is removed.`,
        hit: `At the end of each round, if the target lost hit points that round, it has a 50\\% chance to be sent forward in time by one round.
        During the next round, it ceases to exist.
        At the end of the next round, it returns to its original location, or the closest open space if that location is occupied.
        This effect lasts until the target takes a \\glossterm{short rest}.`,
        targeting: `
        Make an attack vs. Mental against one creature within \\medrange.
        `,
      },
      rank: 5,
      scaling: "accuracy",
      tags: ["Curse"],
    },

    {
      name: "Slow",

      attack: {
        crit: `The condition must be removed twice before the effect ends.`,
        hit: `The target is \\slowed as a \\glossterm{condition}.`,
        targeting: `
          Make an attack vs. Mental against one creature within \\medrange.
        `,
      },
      rank: 1,
      scaling: "accuracy",
    },

    {
      name: "Mass Slow",

      attack: {
        crit: `The effect becomes a \\glossterm{condition} on each target.`,
        hit: `Each target is \\glossterm{briefly} \\slowed.`,
        targeting: `
          Make an attack vs. Mental with a +1 \\glossterm{accuracy} bonus against all creatures in a \\smallarea radius within \\medrange.
        `,
      },
      rank: 1,
      scaling: "accuracy",
    },

    {
      name: "Greater Mass Slow",

      attack: {
        crit: `The condition must be removed twice before the effect ends.`,
        hit: `Each target is \\slowed as a \\glossterm{condition}.`,
        targeting: `
          Make an attack vs. Mental against all creatures in a \\smallarea radius within \\medrange.
        `,
      },
      rank: 4,
      scaling: "accuracy",
    },

    {
      name: "Greater Slow",

      attack: {
        crit: `The target is \\immobilized instead of slowed.`,
        hit: `
          The target is \\slowed as a \\glossterm{condition}.
          The condition must be removed twice before the effect ends.
        `,
        targeting: `
          Make an attack vs. Mental against one creature within \\medrange.
        `,
      },
      rank: 5,
    },

    // -2 levels for 50% chance of activation
    {
      name: "Stutterstop",

      attack: {
        crit: `The target is immobilized every round.`,
        hit: `As a \\glossterm{condition}, the target is \\slowed and randomly immobilized.
        At the start of each round, it has a 50\\% chance to be \\immobilized during that round.`,
        targeting: `
          Make an attack vs. Mental against one creature within \\medrange.
        `,
      },

      rank: 7,
      scaling: "accuracy",
    },

    {
      name: "Haste",

      effect: `
        You gain a +5 foot \\glossterm{magic bonus} to your speed with all of your \\glossterm{movement modes}.
      `,
      rank: 2,
      scaling: {
        4: `The speed bonus increases to +10 feet.`,
        6: `The speed bonus increases to +15 feet.`,
      },
      type: "Attune",
    },

    {
      name: "Accelerated Strike",

      effect: `
        As a \\glossterm{minor action}, you can make a \\glossterm{strike} with a -3 penalty to \\glossterm{accuracy}.
        You do not add your \\glossterm{power} to damage with the strike.
      `,
      rank: 4,
      scaling: { 6: `The accuracy penalty is reduced to -2.` },
      type: "Attune (deep)",
    },

    {
      name: "Temporal Duplicate",

      castingTime: "minor action",
      effect: `
        Choose yourself or one \\glossterm{ally} within \\medrange.
        You reach into a possible future and create a duplicate of the target.
        The duplicate is identical in all ways to the target when the spell resolves.

        The target and its duplicate can act during the next round.
        At the end of that round, the target and its duplicate cease to exist.
        During that round, time does not pass for the target.
        At the end of the following round, the target reappears in the place where it ceased to exist.
        If that space is occupied, it appears in the closest unoccupied space.
        When the target reappears, its condition is unchanged from when it left.
        Its \\glossterm{hit points}, conditions, and all other statistics are unaffected, regardless of any damage or other negative effects suffered by the duplicate.

        The duplicate is fragile, and its actions are limited.
        It cannot use actions that would cause it to increase its \\glossterm{fatigue level}, lose \\glossterm{hit points}, or otherwise suffer negative consequences as a cost of the action.
        If it loses any \\glossterm{hit points}, it ceases to exist.
      `,
      rank: 5,
      scaling: {
        7: `If you cast this spell as a standard action, you can choose to have the duplicate persist for two rounds instead of one.
                If you do, the target disappears for two rounds at the same time as the duplicate.`,
      },
    },

    {
      name: "Time Hop",

      castingTime: "minor action",
      effect: `
        Choose either yourself or one Medium or smaller \\glossterm{ally} or \\glossterm{unattended} object within \\medrange.
        You send the target into the future, causing it to temporarily cease to exist.
        When you cast this spell, you choose how many rounds the target ceases to exist for, up to a maximum of five rounds.
        At the end of the last round, it reappears in the same location where it disappeared.

        The area the target occupied can be physically crossed, but it is treated as an invalid destination for teleportation and other similar magic.
        When the target reappears, all of its surroundings are adjusted as if the object had retroactively always existed in its space.
        For example, if the location is occupied by a creature that walked into the area, the creature is relocated to the closest unoccupied space along the path it took to reach the target.
      `,
      rank: 2,
      scaling: {
        4: `The maximum size of the target increases to Large.`,
        6: `The maximum size of the target increases to Huge.`,
      },
    },

    {
      name: "Temporal Stasis",

      castingTime: "minor action",
      effect: `
        Choose yourself or one Medium or smaller \\glossterm{ally} within \\medrange.
        The target is placed into stasis, rendering it unconscious.
        While in stasis, it cannot take any actions and cannot be targeted, moved, damaged, or otherwise affected by outside forces in any way.

        % TODO: wording
        This effect normally lasts as long as you \\glossterm{attune} to it, and until the end of the round when you release the attunement.
        If you use this ability on yourself, it instead lasts for a number of rounds you choose when you cast the spell, up to a maximum of five rounds.
      `,
      rank: 4,
      scaling: { 6: `The maximum size of the target increases to Large.` },
      type: "Attune",
    },

    {
      name: "Time Lock -- Mind",

      effect: `
        Choose yourself or an \\glossterm{ally} within \\medrange.
        You lock the state of the target's body in time.
        Note the target's current \\glossterm{conditions}.
        If the target dies, this effect ends immediately.

        As a \\glossterm{standard action}, you can reach through time to restore the target's state.
        If you do, the target's \\glossterm{conditions} become identical to what they were when you cast this spell.
        This restoration is a \\abilitytag{Swift} effect, so it happens before any incoming attacks.
        It does not affect any other properties of the target, such as any vital wounds gained or resources expended.
        After you restore the target's state in this way, it increases its \\glossterm{fatigue level} by two, and the spell ends.
      `,
      rank: 1,
      scaling: {
        3: `
          The target's fatigue level only increases by one instead of two.
        `,
        5: `
          The target's fatigue level does not increase.
        `,
        7: `
          You can target an additional \\glossterm{ally} with this spell.
        `,
      },
      type: "Sustain (minor)",
    },

    {
      name: "Time Lock -- Health",

      effect: `
        This spell functions like the \\spell{time lock -- mind} spell, except that you lock and restore the target's \\glossterm{hit points} instead of its conditions.
      `,
      rank: 4,
      scaling: {
        6: `
          The target's fatigue level only increases by one instead of two.
        `,
      },
      type: "Sustain (minor)",
    },

    {
      name: "Time Lock -- Vitality",

      effect: `
        This spell functions like the \\spell{time lock -- mind} spell, except that you lock and restore the target's \\glossterm{vital wounds} instead of its conditions.
        In addition, the target's fatigue level increases by four when it is restored instead of only increasing by two.
      `,
      rank: 7,
      type: "Sustain (minor)",
    },

    {
      name: "Time Stop",

      effect: `
        You can take two full rounds of actions immediately.
        During this time, all other creatures and objects are fixed in time, and cannot be targeted, moved, damaged, or otherwise affected by outside forces in any way.
        You can still affect yourself and create areas or new effects.
        When this effect ends, you are \\stunned as a \\glossterm{condition}.

        You are still vulnerable to danger, such as from heat or dangerous gases.
        However, you cannot be detected by any means while you travel.

        After you cast this spell, you cannot cast it again until you take a \\glossterm{short rest}.
      `,
      rank: 7,
    },

    {
      name: "Evasion",

      effect: `
        You take half damage from abilities that affect an area and attack your Armor or Reflex defense.
        This does not protect you from any non-damaging effects of those abilities, or from abilities that affect multiple specific targets without affecting an area.
      `,
      rank: 4,
      scaling: {
        6: `This effect also protects you from area attacks against your Fortitude and Mental defenses.`
      },
      type: "Attune",
    },

    {
      name: "Minor Acceleration",

      effect: `
        You can take two \\glossterm{minor actions} each round instead of one.
        You cannot take the same minor action twice in the same round.
      `,
      rank: 6,
      type: "Attune",
    },

    {
      name: "Instant Analysis",

      effect: `
        You accelerate your mind to incredible speed, allowing you to process information quickly.
        From your perspective, you freeze time for five rounds.
        During this time, all creatures and objects are fixed in time, and cannot be targeted, moved, damaged, or otherwise affected by outside forces in any way.
        Your mind remains active during this time, and you are the only one aware of the stoppage of time.
        You cannot move or take any actions other than to observe your surroundings.
        In addition, you can release the time freeze as a \\glossterm{free action}.
        When this spell ends, time resumes in the same phase that it was originally frozen.

        After casting this spell, you cannot cast it again until you take a \\glossterm{short rest}.
      `,
      rank: 3,
      scaling: {
        5: `You can cast this spell as a \\glossterm{minor action}.`,
        7: `The length of frozen time increases to five minutes.`,
      },
    },

    {
      name: "Timeseal",

      // effect: '',
      // narrative: '',
      attack: {
        // crit: '',
        hit: `
          The target takes 1d10 + half \\glossterm{power} energy damage.
          If it loses \\glossterm{hit points} from this damage, it is \\glossterm{briefly} frozen in time.
          It becomes completely immune to all damage, attacks, and effects of any kind.
          In addition, it is \\unconscious and cannot act in any way.
          At the end of the next round, it returns to normal, with no awareness of the intervening time.
          After it returns to normal, it becomes immune to being frozen in time in this way until it takes a \\glossterm{short rest}.
        `,
        targeting: `
          Make an attack vs. Mental against one creature within \\medrange.
        `,
      },
      rank: 3,
      scaling: "damage",
    },
    {
      name: "Certain Timeseal",

      functionsLike: {
        name: "timeseal",
        exceptThat: "you gain a +3 accuracy bonus with the attack, and the damage increases to 2d10 + half \\glossterm{power}.",
      },
      rank: 6,
      scaling: "damage",
    },

    {
      name: "Accelerated Draw",

      effect: `
        You draw one or two weapons into your \\glossterm{free hands}.
        Then, you can make a \\glossterm{mundane} \\glossterm{strike}.
      `,
      narrative: `
        This spell seeks to mimic with time-altering magic what some skilled warriors can do naturally.
      `,
      rank: 1,
      scaling: {
        3: "You gain a +1 accuracy bonus with the strike.",
        5: "The accuracy bonus increases to +2.",
        7: "The accuracy bonus increases to +3.",
      },
    },

    {
      name: "Quickchange",

      effect: `
        You can change your appearance or equipment with superhuman speed.
        This has no effect on any creatures other than yourself.
        This can have any one of the following effects, which are completed at the end of the current phase regardless of the time they would normally take:
        \\begin{itemize}
          \\item You can take off your body armor or clothing, along with any weapons or shields you have equipped.
            You can leave the items on the ground in your square or stow them in an available location, such as in a backpack you wear.
          \\item You can don a new set of body armor or clothing and equip any weapons or shields.
            All of the items you equip this way must be unattended and in your square, but they can be in a hidden location, such as in a backpack.
          \\item You can use the \\ability{disguise creature} ability to affect yourself with a \\minus2 penalty (see \\pcref{Disguise Creature}.
        \\end{itemize}
      `,
      narrative: `
        You become a blur of motion as you quickly don your armor, readying yourself against an unexpected attack.
      `,
      rank: 2,
      scaling: {
        4: "You can perform any two of the listed actions.",
        6: "You can perform any combination of the listed actions.",
      },
    },

    {
      name: "Expeditious Retreat",

      effect: `
        You can immediately use the \\textit{sprint} ability without increasing your \\glossterm{fatigue level}.
        After you cast this spell, you \\glossterm{briefly} cannot cast it again.
      `,
      narrative: `
        You accelerate your body to flee from combat with incredible alacrity.
      `,
      rank: 2,
      scaling: {
        4: "You also gain a +5 foot bonus to your movement speed while you sprint.",
        6: "The speed bonus increases to +10 feet.",
      },
    },

    {
      name: "Disjointed Expiration",

      effect: `
        You or one \\glossterm{ally} within \\medrange can remove a \\glossterm{condition}.
        This cannot remove an effect applied during the current round.
        For each effect removed this way, you deal the target 4 energy damage.
      `,
      rank: 4,
      narrative: `
        You twist time to let your ally's nausea run its natural course in mere seconds.
        It is painful to undergo such a selective temporal acceleration, but the consequences of such distraction on the battlefield would be far worse.
      `,
      scaling: {
        6: `The target can remove two effects.`,
      },
    },

    {
      name: "Accelerated Legerdemain",

      effect: `
        If you are \\glossterm{trained} with the Sleight of Hand skill, you gain a +3 \\glossterm{magic bonus} to it.
        Otherwise, you are treated as being \\glossterm{trained} in that skill.
      `,
      rank: 1,
      narrative: `
        You speed up your fine motions, allowing you to lift pockets and perform subtle feats with ease.
      `,
      scaling: {
        3: `The bonus increases to +4.`,
        5: `The bonus increases to +5.`,
        7: `The bonus increases to +6.`,
      },
      type: "Attune",
    },

    {
      name: 'Rewind Damage',

      effect: `
        Chose yourself or one \\glossterm{ally} within \\shortrange.
        The target regains 1d6 + \\glossterm{power} \\glossterm{damage resistance} and increases its \\glossterm{fatigue level} by one.
        This recovery is doubled for each consecutive round that you have cast this spell on the same target.
      `,
      rank: 1,
      scaling: { special: "The recovery increases by +1d for each rank beyond 1." },
      tags: ['Swift'],
    },

    {
      name: 'Greater Rewind Damage',

      functionsLike: {
        name: 'rewind damage',
        exceptThat: "The recovery increases to 4d6 + \\glossterm{power}.",
      },
      rank: 5,
      scaling: { special: "The recovery increases by +1d for each rank beyond 5." },
    },
  ],
  rituals: [
    {
      name: "Gentle Repose",

      castingTime: "one minute",
      effect: `
        Choose one \\glossterm{unattended}, nonmagical object within \\shortrange.
        Time does not pass for the target, preventing it from decaying or spoiling.
        This can extend the time a poison or similar item lasts before becoming inert.
        % What effects have an explicit time limit?
        If used on a corpse, this effectively extends the time limit for effects that require a fresh or intact body.
        Additionally, this can make transporting a fallen comrade more pleasant.

        % Does this need to explicitly clarify that it doesn't stop time from passing for the creature's soul?
      `,
      rank: 3,
      type: "Attune",
    },
  ],
};
