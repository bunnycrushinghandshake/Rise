import { CombatStyle } from ".";

export const bluntForce: CombatStyle = {
  name: "Blunt Force",
  shortDescription: "Smash foes with bludgeoning weapons and raw power.",
  sources: ["martial", "primal"],

  maneuvers: [
    {
      name: "Liver Shot",

      effect: `
        Make a \\glossterm{strike} using a bludgeoning weapon.
        The attack is made against each subject's Fortitude defense instead of its Armor defense.
        You take a -2d penalty to damage with the strike, and your \\glossterm{power} is halved.
        Each creature that loses hit points from the strike is \\glossterm{sickened} as a \\glossterm{condition}.
        `,

      rank: 1,
      type: "Duration",
    },

    {
      name: "Nauseating Liver Shot",

      effect: `
        Make a \\glossterm{strike} using a bludgeoning weapon.
        The attack is made against each subject's Fortitude defense instead of its Armor defense.
        You take a -2d penalty to damage with the strike, and your \\glossterm{power} is halved.
        Each creature that loses hit points from the strike is \\glossterm{nauseated} as a \\glossterm{condition}.
        `,

      rank: 3,
      type: "Duration",
    },
    {
      name: "Greater Nauseating Liver Shot",

      effect: `
        Make a \\glossterm{strike} using a bludgeoning weapon.
        The attack is made against each subject's Fortitude defense instead of its Armor defense.
        Your \\glossterm{power} with the strike is halved.
        Each creature that loses hit points from the strike is \\glossterm{nauseated} as a \\glossterm{condition}.
        `,

      rank: 6,
      type: "Duration",
    },

    {
      name: "Pulverizing Smash",

      effect: `
        Make a \\glossterm{strike} using a bludgeoning weapon.
        The attack is made against each subject's Fortitude defense instead of its Armor defense.
        You take -1d penalty to damage with the strike.
        `,

      rank: 1,
      type: "Instant",
    },

    {
      name: "Pulverizing Power Smash",

      effect: `
        Make a \\glossterm{strike} using a bludgeoning weapon with a -2 penalty to accuracy.
        The attack is made against each subject's Fortitude defense instead of its Armor defense.
        You gain a +1d bonus to damage with the strike.
        `,

      rank: 3,
      type: "Instant",
    },

    {
      name: "Ground Stomp",

      // original targets: Everything adjacent to you that is on solid ground
      attack: {
        glance: `Half damage.

        \\rankline
        The damage increases by +1d for each rank beyond 3.`,
        hit: `Each subject takes 2d6 + half \\glossterm{power} bludgeoning damage
        Each Large or smaller creature that loses \\glossterm{hit points} from this damage is knocked \\glossterm{prone}..`,
        targeting: `
        Make an attack vs. Reflex against each subject.
        `,
      },

      rank: 3,
      type: "Instant",
    },

    {
      name: "Earthshatter Stomp",

      // original targets: Everything in a \areamed radius from you that is on solid ground
      attack: {
        glance: `Half damage.`,
        hit: `Each subject takes 4d6 + half \\glossterm{power} bludgeoning damage
        Each Large or smaller creature that loses \\glossterm{hit points} from this damage is knocked \\glossterm{prone}..`,
        targeting: `
        Make an attack vs. Reflex against each subject.
        You take a -4 penalty to accuracy with this attack against your \\glossterm{allies}.
        `,
      },

      rank: 6,
      type: "Instant",
    },

    {
      name: "Ground Slam",

      // original targets: Everything on the ground in a \areasmall, 10 ft. wide line from you
      effect: `
        Make a melee \\glossterm{strike} using a bludgeoning weapon against each subject.
        Your \\glossterm{power} with the strike is halved.
        All damage dealt by this attack is bludgeoning damage instead of its normal types.
        `,

      rank: 3,
      type: "Instant",
    },

    {
      name: "Titanic Slam",

      // original targets: Everything on the ground in a \arealarge, 10 ft. wide line from you
      effect: `
        Make a melee \\glossterm{strike} using a bludgeoning weapon against each subject.
        Your \\glossterm{power} with the strike is halved.
        All damage dealt by this attack is bludgeoning damage instead of its normal types.
        `,

      rank: 5,
      type: "Instant",
    },

    {
      name: "Resonating Strike",

      effect: `
        Make a strike using a bludgeoning weapon.
        Damage dealt by the strike is sonic damage in addition to its normal damage types.
        `,

      rank: 3,
      type: "Instant",
    },

    {
      name: "Resonating Crush",

      effect: `
        Make a strike using a bludgeoning weapon.
        The attack is made against each subject's Fortitude defense instead of its Armor defense.
        You take -1d penalty to damage with the strike.
        Damage dealt by the strike is sonic damage in addition to its normal damage types.
        `,

      rank: 4,
      type: "Instant",
    },

    {
      name: "Headshot",

      effect: `
        Make a \\glossterm{strike} using a bludgeoning weapon.
        You take a -2d penalty to damage with the strike, and your \\glossterm{power} is halved.
        If a creature loses hit points from the strike, it is \\glossterm{dazed} as a \\glossterm{condition}.
        `,

      rank: 1,
      type: "Duration",
    },

    {
      name: "Stunning Headshot",

      effect: `
        Make a \\glossterm{strike} using a bludgeoning weapon.
        You take a -2d penalty to damage with the strike, and your \\glossterm{power} is halved.
        If a creature loses hit points from the strike, it is \\glossterm{stunned} as a \\glossterm{condition}.
        `,

      rank: 3,
      type: "Duration",
    },

    {
      name: "Confusing Headshot",

      effect: `
        Make a \\glossterm{strike} using a bludgeoning weapon.
        You take a -2d penalty to damage with the strike, and your \\glossterm{power} is halved.
        If a creature loses hit points from the strike, it is \\glossterm{confused} as a \\glossterm{condition}.
        `,

      rank: 6,
      type: "Duration",
    },

    {
      name: "Leap Slam",

      // original targets: Everything adjacent to you that is on earth or unworked stone (see text)
      attack: {
        glance: `Half damage.

        \\rankline
        The damage increases by +1d for each rank beyond 4.`,
        hit: `Each subject takes 2d8 + half \\glossterm{power} bludgeoning damage.`,
        targeting: `
        You make a Jump check to leap and move as normal for the leap, up to a maximum distance equal to your \\glossterm{base speed} (see \\pcref{Leap}).
        When you land, if the vertical distance in feet between the highest point of your leap and your landing point was at least ten feet, make an attack vs. Reflex against each subject.
        `,
      },

      rank: 4,
      type: "Instant",
    },

    {
      name: "Ricochet",

      // original targets: Up to three creatures or objects in a \areasmall radius within \shortrange (see text)
      effect: `
        Make a thrown \\glossterm{strike} using a bludgeoning weapon against each subject.
        Your \\glossterm{power} with the strike is halved.
        If you choose yourself as one of the subjects, you can catch the weapon instead of taking damage from it.
      `,
      rank: 2,
      type: "Instant",
    },
  ],
};
