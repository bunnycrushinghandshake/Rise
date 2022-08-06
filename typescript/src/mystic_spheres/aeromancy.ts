import { MysticSphere } from ".";

export const aeromancy: MysticSphere = {
  name: "Aeromancy",
  shortDescription: "Command air to protect allies and blast foes.",
  sources: ["nature", "domain"],

  cantrips: [
    {
      name: "Airborne Leap",

      effect: `
        You \\glossterm{briefly} gain a +3 \\glossterm{magic bonus} to the Jump skill.
      `,
      narrative: `
        The air rises beneath you and pushes you up, propelling you farther when you leap.
      `,
      scaling: {
        2: "The bonus increases to +4.",
        4: "The bonus increases to +5.",
        6: "The bonus increases to +6.",
      },
    },
    {
      name: "Manipulate Air",

      effect: `
        You change the wind speed within a \\medarea radius \\glossterm{emanation} from you by up to 10 miles per hour.
        If you decrease the wind's speed to 0, you can increase it again with the remainder of your speed change and choose any direction for it to travel.
        You choose the speed change and direction when you cast this spell, and that choice persists for the duration of this effect.
      `,
      narrative: `
        The wind around you waxes and wanes at your command, softening the force of a tempest or creating one to harass your foes.
      `,
      scaling: {
        2: "The area increases to a \\largearea radius, and the maximum speed change increases to 20 miles per hour.",
        4: "The area increases to a \\hugearea radius, and the maximum speed change increases to 40 miles per hour.",
        6: "The area increases to a \\gargarea radius, and the maximum speed change increases to 80 miles per hour.",
      },
      type: "Sustain (minor)",
    },
    {
      name: "Feather Fall",

      effect: `
        Choose yourself or an \\glossterm{ally} within \\longrange.
        Until the end of the round, that creature treats all falls as if they were 20 feet shorter for the purpose of determining \\glossterm{falling damage}.
      `,
      narrative: `
        The air beneath you suddenly accelerates into a great wind, softening the force of your unexpected fall.
      `,
      scaling: {
        2: "The distance reduction increases to 50 feet.",
        4: "The distance reduction increases to 100 feet.",
        6: "The distance reduction increases to 200 feet.",
      },
    },
  ],
  spells: [
    // Treat arrow attraction as r1 -1 level
    {
      name: "Curse of Arrow Attraction",

      attack: {
        crit: "The effect lasts until this curse is removed.",
        hit: `
          The target takes a -2 penalty to defenses against ranged \\glossterm{strikes} until it takes a \\glossterm{short rest}.
        `,
        targeting: "Make an attack vs. Mental against anything within \\medrange.",
      },
      narrative: `
        The air around your foe ripples with hidden air currents that seem to guide the flight of arrows, ensuring that they strike true.
      `,
      rank: 2,
      scaling: "accuracy",
      tags: ["Curse"],
    },
    // Treat greater arrow attraction as r2 -2 levels
    {
      name: "Greater Curse of Arrow Attraction",

      attack: {
        crit: `
          The effect lasts until this curse is removed.
        `,
        hit: `
          The target takes a -4 penalty to defenses against ranged \\glossterm{strikes} until it takes a \\glossterm{short rest}.
        `,
        targeting: "Make an attack vs. Mental against anything within \\medrange.",
      },
      narrative: `
        The air around your foe ripples with hidden air currents that seem to guide the flight of arrows, ensuring that they strike true.
      `,
      rank: 5,
      scaling: "accuracy",
      tags: ["Curse"],
    },
    {
      name: "Propulsion",

      effect: `
        Choose yourself or one Large or smaller \\glossterm{ally} within \\medrange.
        You \\glossterm{push} the target up to 60 feet in any direction.
        You cannot change the direction of the movement partway through.
        Moving the target upwards costs twice the normal movement cost.
      `,
      rank: 1,
      scaling: {
        3: "The distance increases to 120 feet.",
        5: "The distance increases to 240 feet.",
        7: "The distance increases to 480 feet.",
      },
    },
    {
      name: "Mass Propulsion",

      effect: `
        Choose up to five creatures from among yourself and your Large or smaller \\glossterm{allies} within \\medrange.
        You \\glossterm{push} each target up to 60 feet in any direction.
        Each target must be pushed in the same direction.
        You cannot change the direction of the movement partway through.
        Moving a target upwards costs twice the normal movement cost.
      `,
      rank: 3,
      scaling: {
        5: "The distance increases to 120 feet.",
        7: "The distance increases to 240 feet.",
      },
    },
    {
      name: "Wind Screen",

      effect: `
        You gain a +2 bonus to your defenses against ranged \\glossterm{strikes}.
      `,
      // narrative: "",
      rank: 2,
      scaling: {
        4: `The bonus increases to +3.`,
        6: `The bonus increases to +4.`,
      },
      type: "Attune",
    },
    {
      name: "Mass Wind Screen",

      functionsLike: {
        mass: true,
        name: "wind screen",
      },
      // narrative: "",
      rank: 3,
      scaling: {
        5: "The bonus to Armor defense increases to +2.",
        7: "The bonus to Armor defense increases to +3.",
      },
      type: "Attune (target)",
    },
    {
      name: "Windsnipe",

      attack: {
        // crit: '',
        hit: "The target takes 2d6 \\add \\glossterm{power} bludgeoning damage.",
        targeting: "Make an attack vs. Armor against anything within \\distrange.",
      },
      // narrative: '',
      rank: 3,
      scaling: "damage",
    },
    {
      name: "Greater Windsnipe",

      attack: {
        // crit: '',
        hit: "The target takes 4d8 \\add \\glossterm{power} bludgeoning damage.",
        targeting: "Make an attack vs. Armor against anything within \\extrange.",
      },
      // narrative: '',
      rank: 6,
      scaling: "damage",
    },
    {
      name: "Buffet",

      attack: {
        crit: "Double damage, and you can knockback the target 60 feet instead of 30 feet.",
        hit: `
          The target takes 1d6 bludgeoning damage.
          If it loses \\glossterm{hit points} from this damage, you \\glossterm{knockback} it up to 30 feet in any direction (see \\pcref{Knockback Effects}).
          Moving the target upwards costs twice the normal movement cost.
        `,
        targeting:
          "Make an attack vs. Fortitude against anything Large or smaller within \\medrange.",
      },
      // narrative: '',
      rank: 1,
      scaling: "accuracy",
    },
    {
      name: "Greater Buffet",

      functionsLike: {
        name: 'buffet',
        // This deals an immediate 6d6 if you smash someone against a barrier, which is a lot of damage.
        exceptThat: "the damage increases to 1d10. In addition, the knockback distance increases to 60 feet, or 120 feet on a critical hit.",
      },
      // narrative: '',
      rank: 4,
      scaling: "accuracy",
    },
    {
      name: "Supreme Buffet",

      functionsLike: {
        name: 'buffet',
        // This deals an immediate 12d6 if you smash someone against a barrier, which is a lot of damage.
        exceptThat: "the damage increases to 2d10. In addition, the knockback distance increases to 120 feet, or 240 feet on a critical hit.",
      },
      // narrative: '',
      rank: 7,
      scaling: "accuracy",
    },
    {
      name: "Gentle Descent",

      effect: "You gain a 30 foot \\glossterm{glide speed} (see \\pcref{Gliding}).",
      // narrative: '',
      rank: 2,
      scaling: {
        4: "You are immune to \\glossterm{falling damage} even if you do not glide.",
        6: "You can reduce your \\glossterm{glide speed} to 20 feet or increase it to 60 feet during each phase that you glide.",
      },
      type: "Attune",
    },
    {
      name: "Mass Gentle Descent",

      functionsLike: {
        mass: true,
        name: "gentle descent",
      },
      // narrative: '',
      rank: 4,
      scaling: {
        6: "The target is immune to \\glossterm{falling damage} even if it does not glide.",
      },
      type: "Attune (target)",
    },
    {
      name: "Flight",

      effect: `
        You gain a 30 foot \\glossterm{fly speed} with a maximum height of 30 feet (see \\pcref{Flying}).
        If you are above that height, you gain a 30 foot \\glossterm{glide speed} instead.
      `,
      // narrative: '',
      rank: 4,
      scaling: {
        6: "The maximum height increases to 60 feet.",
      },
      type: "Attune",
    },
    {
      name: "Agile Flight",

      effect: `
        You gain a 30 foot \\glossterm{fly speed} with a maximum height of 30 feet (see \\pcref{Flying}).
        Your \\glossterm{maneuverability} with this fly speed is perfect (see \\pcref{Flying Maneuverability}).
      `,
      // narrative: '',
      rank: 6,
      type: "Attune",
    },
    {
      name: "Soaring Flight",

      effect: `
        You gain a 30 foot \\glossterm{fly speed} with a maximum height of 120 feet (see \\pcref{Flying}).
        If you are above that height, you gain a 30 foot \\glossterm{glide speed} instead.
        Your \\glossterm{maneuverability} with this fly speed is poor (see \\pcref{Flying Maneuverability}).
      `,
      // narrative: '',
      rank: 7,
      type: "Attune",
    },
    {
      name: "Air Walk",

      effect: `
        You can walk on air as if it were solid ground.
        This only functions as long as you are no more than 60 feet above an object at least two size categories larger than you that is free-standing and capable of supporting your weight.
      `,
      // narrative: '',
      rank: 5,
      scaling: {
        7: "The maximum height increases to 120 feet.",
      },
      type: "Attune",
    },
    // +2 levels for push, -2 levels for no power
    {
      name: "Gust of Wind",

      attack: {
        // crit: '',
        hit: `
          Each target takes 1d6 bludgeoning damage.
          In addition, each target damaged by the attack is \\glossterm{pushed} 30 feet in the direction the line points away from you.
          Once a target leaves the area, it stops being moved and blocks any other targets from being pushed.
        `,
        targeting: `
          Make an attack vs. Fortitude against everything in a \\largearealong, 5 ft. wide line from you.
        `,
      },
      // effect: '',
      // narrative: '',
      rank: 1,
      scaling: "damage",
    },
    {
      name: "Greater Gust of Wind",

      attack: {
        // crit: '',
        // +1 level for farther push, +2 levels for area
        hit: `
          Each target takes 2d6 bludgeoning damage.
          In addition, each target damaged by the attack is \\glossterm{pushed} 60 feet in the direction the line points away from you.
          Once a target leaves the area, it stops being moved and blocks any other targets from being pushed.
        `,
        targeting: `
          Make an attack vs. Fortitude against everything in a \\hugearealong, 10 ft. wide line from you.
        `,
      },
      // narrative: '',
      rank: 4,
      scaling: "damage",
    },
    {
      name: "Supreme Gust of Wind",

      attack: {
        // crit: '',
        // +2 levels for half power, +1 level for more area
        hit: `
          Each target takes 4d6 + half \\glossterm{power} bludgeoning damage.
          In addition, each target damaged by the attack is \\glossterm{pushed} 60 feet in the direction the line points away from you.
          Once a target leaves the area, it stops being moved and blocks any other targets from being pushed.
        `,
        targeting: `
          Make an attack vs. Fortitude against everything in a \\hugearealong, 15 ft. wide line from you.
        `,
      },
      // narrative: '',
      rank: 7,
      // scaling: "damage",
    },
    {
      name: "Windblade",

      effect: `
        You gain a +5 foot \\glossterm{magic bonus} to your \\glossterm{reach} with melee \\glossterm{strikes}.
        Attacks that hit because of this reach deal bludgeoning damage instead of any other damage types.
        This has no effect on ranged attacks you make.
      `,
      // narrative: '',
      rank: 3,
      scaling: {
        5: "The bonus increases to +10 feet.",
        7: "The bonus increases to +15 feet.",
      },
      type: "Attune",
    },
    {
      name: "Mass Windblade",

      functionsLike: {
        mass: true,
        name: "windblade",
      },
      // narrative: '',
      rank: 5,
      scaling: {
        7: "The bonus increases to +10 feet.",
      },
      type: "Attune (target)",
    },
    {
      name: "Retributive Winds",

      attack: {
        // crit: '',
        hit: `
          Each target takes 2d10 bludgeoning damage.
        `,
        targeting: `
          At the end of each phase, make an attack vs. Armor against each creature within \\shortrange of you that attacked you during that phase.
          Any effect which increases this spell's range increases the range of this retaliation by the same amount.
        `,
      },
      // effect: '',
      // narrative: '',
      rank: 5,
      scaling: "damage",
      type: "Attune (deep)",
    },
    {
      name: "Control Weather",

      effect: `
        When you cast this spell, you choose a new weather pattern.
        You can only choose weather which would be reasonably probable in the climate and season of the area you are in.
        For example, you can normally create a thunderstorm, but not if you are in a desert.

        When you complete the spell, the weather begins to take effect in a two mile radius cylinder-shaped \\glossterm{zone} from your location.
        After five minutes, your chosen weather pattern fully takes effect.
        % TODO: define weather intensities
        You cannot change the intensity of the weather beyond what would be possible without magic during this time frame.
        For example, you can change a clear sky into a light thunderstorm, but you cannot create a hurricane or tornado from untroubled air.

        You can control the general tendencies of the weather, such as the direction and intensity of the wind.
        You cannot control specific applications of the weather, such as the location of lightning strikes.
        Contradictory weather conditions are not possible simultaneously.

        After the spell's effect ends, the weather continues on its natural course, which may cause your chosen weather pattern to end.
        % TODO: This should be redundant with generic spell mechanics
        If another ability would magically manipulate the weather in the same area, the most recently used ability takes precedence.
      `,
      // narrative: '',
      rank: 4,
      scaling: {
        6: "You can shape the weather for up to fifteen minutes before it takes effect, increasing the intensity of the changes you can make.",
      },
      type: "Attune (deep)",
    },
    {
      name: "Cyclone",

      attack: {
        // crit: '',
        hit: `
          Each target takes 1d10 \\add half \\glossterm{power} bludgeoning damage.
        `,
        targeting: `
          Make an attack vs. Fortitude against everything in a \\smallarea radius within \\medrange.
        `,
      },
      // narrative: '',
      rank: 3,
      scaling: "damage",
    },
    // 2 levels for push
    {
      name: "Hurricane",

      attack: {
        // crit: '',
        hit: `
          Each target takes 2d10 \\add half \\glossterm{power} bludgeoning damage.
          In addition, each target is \\glossterm{pushed} 30 feet clockwise around you.
          Each target's final position should be the same distance from you as its starting position.
        `,
        targeting: `
          Make an attack vs. Fortitude against all \\glossterm{enemies} in a \\largearea radius from you.
        `,
      },
      // narrative: '',
      rank: 6,
      scaling: "damage",
    },
    {
      name: "Windtheft",

      attack: {
        // crit: '',
        // No relevant glance effect
        hit: `
          You \\glossterm{knockback} the object up to 60 feet towards you.
          You can use a \\glossterm{free hand} to catch the object if it reaches you.
        `,
        targeting: `
          Make an attack vs. Reflex against one Small or smaller object within \\medrange.
          If the object is attended by a creature, the attack must also beat the attending creature's Reflex defense.
          If it is held in two hands or well secured, this attack automatically fails.

          After you successfully steal an item from a creature with this spell, it gains a +5 bonus to its defenses against this spell until it takes a \\glossterm{short rest}.
        `,
      },
      // narrative: '',
      rank: 2,
      scaling: "accuracy",
    },
    {
      name: "Dust Cloud",

      attack: {
        crit: `
          The effect becomes a \\glossterm{condition}.
        `,
        hit: `
          Each target is \\glossterm{briefly} \\dazzled.
        `,
        targeting: `
          Make an attack vs. Reflex against all creatures in a \\smallarea radius within \\longrange from you.
        `,
      },
      // narrative: '',
      rank: 1,
      scaling: "accuracy",
    },
    {
      name: "Massive Dust Cloud",

      attack: {
        crit: `
          The effect becomes a \\glossterm{condition}.
        `,
        hit: `
          Each target is \\glossterm{briefly} \\dazzled.
        `,
        targeting: `
          Make an attack vs. Reflex against all creatures in a \\hugearea radius within \\longrange from you.
        `,
      },
      // narrative: '',
      rank: 5,
      scaling: "accuracy",
    },
    {
      name: "Blinding Dust Cloud",

      attack: {
        crit: `
          The effect becomes a \\glossterm{condition}.
        `,
        hit: `
          Each target with no remaining \\glossterm{damage resistance} is \\glossterm{briefly} \\blinded.
        `,
        targeting: `
          Make an attack vs. Reflex against all creatures in a \\smallarea radius within \\medrange from you.
        `,
      },
      // narrative: '',
      rank: 4,
      scaling: "accuracy",
    },
    {
      name: "Dustblind",

      attack: {
        hit: `
          The target takes 2d8 physical damage.
          If it loses \\glossterm{hit points} from this damage, it is \\blinded as a \\glossterm{condition}.
        `,
        targeting: `
          Make an attack vs. Reflex against one creature within \\medrange.
          If there is dirt, dust, or a collection of loose objects of similar size within 60 foot \\glossterm{range} of the target's eyes, you gain a +2 accuracy bonus with this attack.
        `,
      },
      // narrative: '',
      rank: 6,
      scaling: "accuracy",
    },
    {
      name: "Piercing Windblast",

      attack: {
        // crit: '',
        hit: "The target takes 2d6 \\add \\glossterm{power} piercing damage.",
        targeting: "Make an attack vs. Reflex against anything within \\medrange.",
      },
      narrative: `
        A rush of wind flows rapidly through the gaps in your foe's armor to pierce its heart.
      `,
      rank: 3,
      scaling: "damage",
    },
    {
      name: "Greater Piercing Windblast",

      attack: {
        // crit: '',
        hit: "The target takes 4d8 \\add \\glossterm{power} piercing damage.",
        targeting: "Make an attack vs. Reflex against anything within \\longrange.",
      },
      narrative: `
        A rush of wind flows instantly through the gaps in your foe's armor to pierce its heart.
      `,
      rank: 6,
      scaling: "damage",
    },
    {
      name: "Misty Step",

      effect: `
        You can move through creatures freely.
        This does not allow you to move through inanimate objects.
        In addition, you gain a 30 foot \\glossterm{glide speed}.
      `,
      narrative: `
        Your body is partially transformed into mist.
        This allows you to drift through enemies and even the air with ease.
      `,
      rank: 3,
      scaling: {
        5: "You also ignore all sources of \\glossterm{difficult terrain}.",
        7: "You also gain a 30 foot \\glossterm{fly speed} with a maximum height of 30 feet (see \\pcref{Flying}).",
      },
      type: "Attune",
    },
    {
      name: "Airborne Jumper",

      effect: `
        If you are \\glossterm{trained} with the Jump skill, you gain a +3 \\glossterm{magic bonus} to it.
        Otherwise, you are treated as being \\glossterm{trained} in that skill.
      `,
      rank: 1,
      scaling: {
        3: `The bonus increases to +4.`,
        5: `The bonus increases to +5.`,
        7: `The bonus increases to +6.`,
      },
      type: "Attune",
    },
    {
      name: "Wall of Wind",

      effect: `
        You create a wall of wind in a 20 ft.\\ high, \\medarea \\glossterm{wall} within \\medrange.
        It does not block passage or significantly obstruct sight.
        However, ranged \\glossterm{strikes} that pass through the wall take a -2 \\glossterm{accuracy} penalty.

        After using this ability, you \\glossterm{briefly} cannot use it or any other \\abilitytag{Barrier} ability.
      `,
      rank: 1,
      scaling: {
        3: "The penalty increases to -3.",
        5: "The penalty increases to -4.",
        7: "The penalty increases to -5.",
      },
      tags: ['Barrier'],
      type: "Sustain (attuneable, minor)",
    },
    {
      name: "Massive Wall of Wind",

      functionsLike: {
        name: "wall of wind",
        exceptThat: "the area increases to a 30 ft. high, \\hugearea \\glossterm{wall} within \\longrange.",
      },
      rank: 4,
      scaling: {
        6: "The penalty increases to -3.",
      },
      tags: ['Barrier'],
      type: "Sustain (attuneable, minor)",
    },
  ],

  rituals: [
    {
      name: "Air Bubble",

      castingTime: "one minute",
      effect: `
        One ritual participant gains the ability to breathe clear, clean air regardless of its surroundings.
        This can allow it to breathe underwater and avoid air-based poisons.
      `,
      // narrative: '',
      rank: 3,
      type: "Attune (target)",
    },
    {
      name: "Detect Air",

      castingTime: "one minute",
      effect: `
        You learn the approximate distance and direction to any air within \\rnglong \\glossterm{range} of you.
        Since this is a \\abilitytag{Detection} ability, its range can penetrate some solid objects (see \\pcref{Detection}).
        This ritual can detect air pockets with a minimum size of Fine.
      `,
      // narrative: '',
      rank: 1,
      tags: ["Detection"],
    },
    {
      name: "Greater Detect Air",

      castingTime: "one minute",
      functionsLike: {
        exceptThat: "the range increases to \\extrange.",
        name: "detect air",
      },
      // narrative: '',
      rank: 3,
    },
    {
      name: "Supreme Detect Air",

      castingTime: "one minute",
      functionsLike: {
        exceptThat: "the range increases to 2,000 feet.",
        name: "detect air",
      },
      // narrative: '',
      rank: 5,
    },
  ],
};
