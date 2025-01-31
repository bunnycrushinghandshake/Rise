import { aeromancy } from "./aeromancy";
import { aquamancy } from "./aquamancy";
import { astromancy } from "./astromancy";
import { channelDivinity } from "./channel_divinity";
import { chronomancy } from "./chronomancy";
import { cryomancy } from "./cryomancy";
import { electromancy } from "./electromancy";
import { enchantment } from "./enchantment";
import { fabrication } from "./fabrication";
import { photomancy } from "./photomancy";
import { polymorph } from "./polymorph";
import { prayer } from "./prayer";
import { pyromancy } from "./pyromancy";
import { revelation } from "./revelation";
import { summoning } from "./summoning";
import { telekinesis } from "./telekinesis";
import { terramancy } from "./terramancy";
import { thaumaturgy } from "./thaumaturgy";
import { toxicology } from "./toxicology";
import { umbramancy } from "./umbramancy";
import { verdamancy } from "./verdamancy";
import { vivimancy } from "./vivimancy";
import { universal } from "./universal";

export const mysticSpheres: MysticSphere[] = [
  aeromancy,
  aquamancy,
  astromancy,
  channelDivinity,
  chronomancy,
  cryomancy,
  electromancy,
  enchantment,
  fabrication,
  photomancy,
  polymorph,
  prayer,
  pyromancy,
  revelation,
  summoning,
  telekinesis,
  terramancy,
  thaumaturgy,
  toxicology,
  umbramancy,
  verdamancy,
  vivimancy,
  universal,
];

export type MysticSphereSource = "arcane" | "divine" | "domain" | "nature" | "pact";

export interface MysticSphere {
  cantrips?: Cantrip[];
  hasImage?: boolean;
  name: string;
  rituals?: Ritual[];
  sources: MysticSphereSource[];
  shortDescription: string;
  specialRules?: string;
  spells: Spell[];
}

export interface BaseSpellLike {
  attack?: StandardAttack;
  castingTime?: string;
  cost?: string;
  effect?: string;
  functionsLike?: FunctionsLike;
  name: string;
  narrative?: string;
  scaling?: "accuracy" | "double_accuracy" | "poison" | Record<string, string>;
  tableText?: string;
  tags?: string[];
  type?:
    | "Attune"
    | "Attune (deep)"
    | "Attune (deep, target)"
    | "Attune (target)"
    | "Sustain (free)"
    | "Sustain (minor)"
    | "Sustain (standard)"
    | "Sustain (attuneable, free)"
    | "Sustain (attuneable, minor)"
    | "Sustain (attuneable, standard)";
}

export type Cantrip = BaseSpellLike;

export interface Spell extends BaseSpellLike {
  rank: 1 | 2 | 3 | 4 | 5 | 6 | 7;
}

// TODO: allow customizing individual ritual fatigue costs
export interface Ritual extends Spell {
  castingTime: "24 hours" | "one minute" | "one hour" | "minor action" | "special" | "one week";
  materialCost?: boolean;
}

export type Rank = 1 | 2 | 3 | 4 | 5 | 6 | 7 | null;

export interface SpellLike extends BaseSpellLike {
  castingTime?: string;
  materialCost?: boolean;
  rank?: Rank;
}

export interface StandardAttack {
  crit?: string;
  hit: string;
  missGlance?: boolean;
  miss?: string;
  targeting: string;
}

export interface FunctionsLike {
  abilityType?: "cantrip" | "spell" | "maneuver" | "ability" | "ritual";
  exceptThat?: string;
  mass?: boolean;
  oneYear?: boolean;
  name: string;
}
