import { DamageType } from "@src/data";

interface StandardWeaponInput {
  name: StandardWeaponName;
}

interface CustomWeaponInput {
  accuracyBonus?: number;
  damageTypes: DamageType[];
  name: string;
  powerBonus?: number;
}

export type WeaponInput = StandardWeaponInput | CustomWeaponInput;

export type Weapon = Required<CustomWeaponInput>;

export function parseWeaponInput(input: WeaponInput): Weapon {
  return {
    accuracyBonus: 0,
    damageTypes: [],
    powerBonus: 0,
    ...(isStandardWeaponName(input.name) && standardWeapons[input.name]),
    ...input,
  };
}

// This is somewhat cumbersome to write, but it ensure that we specify a damage type for any
// intentionally custom weapons. For the foreseeable future, it's likely that any "custom" weapon is
// actually just a missing standard weapon, so it's worth the duplication.
export type StandardWeaponName =
  | "bite"
  | "claw"
  | "club"
  | "greataxe"
  | "greatclub"
  | "greatsword"
  | "light crossbow"
  | "longsword"
  | "short sword"
  | "slam"
  | "spear"
  | "stinger"
  | "talon"
  | "tentacle";

export function isStandardWeaponName(
  name: StandardWeaponName | string,
): name is StandardWeaponName {
  return Boolean(standardWeapons[name as StandardWeaponName]);
}

export const standardWeapons: Record<StandardWeaponName, Omit<CustomWeaponInput, "name">> = {
  "bite": {
    damageTypes: ["bludgeoning", "piercing"],
  },
  "claw": {
    powerBonus: -2,
    damageTypes: ["slashing", "piercing"],
  },
  "club": {
    damageTypes: ["bludgeoning"],
  },
  "greataxe": {
    powerBonus: 2,
    damageTypes: ["slashing"],
  },
  "greatclub": {
    powerBonus: 2,
    damageTypes: ["bludgeoning"],
  },
  "greatsword": {
    powerBonus: 2,
    damageTypes: ["slashing"],
  },
  "light crossbow": {
    damageTypes: ["piercing"],
  },
  "longsword": {
    damageTypes: ["slashing"],
  },
  "short sword": {
    // TODO: note that this is finessable
    powerBonus: -2,
    damageTypes: ["slashing"],
  },
  "slam": {
    damageTypes: ["bludgeoning"],
  },
  "spear": {
    damageTypes: ["piercing"],
  },
  "tentacle": {
    damageTypes: ["bludgeoning"],
  },
  "talon": {
    damageTypes: ["piercing", "slashing"],
  },
  "stinger": {
    damageTypes: ["piercing"],
    powerBonus: 2,
  },
};
