export interface PassiveAbility {
  description: string;
  name: string;
}

export function poisonousWeapon(weaponName: string, primaryEffect: string, terminalEffect: string) {
  return `
    Whenever a creature takes damage from the $name's ${weaponName},
      if the attack result beat the target's Fortitude defense,
      the damaged creature becomes \\glossterm{poisoned}.
    The poison's primary effect makes the target ${primaryEffect}, and the terminal effect makes it ${terminalEffect}
  `;
}

export const passiveAbilities: Record<string, PassiveAbility> = {
  incorporeal: {
    description: `
      The $name has no physical body.
      It makes no sound while moving, and may be unaffected by other abilities that only affect corporeal creatures, such as \\glossterm{tremorsense}.
      It is immune to \\glossterm{physical} damage and all \\glossterm{mundane} abilities that do not deal damage.
      Whenever it would take damage, it has a 50\\% chance to take no damage instead.

      The $name can enter or pass through solid objects, but it must remain adjacent to the object's exterior at all times.
      While completely inside a solid object, the object provides \\glossterm{total cover}, so it must emerge from the object to attack or be attacked.
    `,
    name: "incorporeal",
  },
};
