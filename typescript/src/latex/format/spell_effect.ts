import { Spell } from "@src/mystic_spheres";

function assertEndsWithPeriod(text: string | null | undefined) {
  if (text && !text.trim().endsWith(".")) {
    throw new Error(`Text should end with period: ${text}`);
  }
}

export function spellEffect(
  spell: Pick<Spell, "attack" | "castingTime" | "effect" | "functionsLike" | "name">,
): string | null {
  try {
    if (spell.attack) {
      assertEndsWithPeriod(spell.attack.targeting);
      assertEndsWithPeriod(spell.attack.hit);
      assertEndsWithPeriod(spell.attack.glance);
      assertEndsWithPeriod(spell.attack.crit);
      // The terminal % prevents a double-space in weird edge cases
      return `
        ${spell.attack.targeting.trim()}%
        \\vspace{0.25em}
        \\hit ${spell.attack.hit.trim()}
        ${spell.attack.glance ? `\\glance ${spell.attack.glance.trim()}` : ""}
        ${spell.attack.crit ? `\\crit ${spell.attack.crit.trim()}` : ""}
      `;
    } else if (spell.effect) {
      return spell.effect;
    } else if (spell.functionsLike) {
      const exceptThat = spell.functionsLike.mass
        ? "it affects up to five creatures of your choice from among yourself and your \\glossterm{allies} within \\medrange."
        : spell.functionsLike.exceptThat;
      assertEndsWithPeriod(exceptThat);
      if (!exceptThat) {
        throw new Error(`Must have a defined 'exceptThat' in a 'functionsLike'`);
      }

      const spellCategory =
        spell.castingTime && spell.castingTime !== "minor action" ? "ritual" : "spell";

      return `
        This ${spellCategory} functions like the \\${spellCategory}{${
        spell.functionsLike.spell
      }} ${spellCategory}, except that ${exceptThat.trim()}
      `;
    } else {
      return null;
    }
  } catch (err) {
    err.message += `Error converting spell ${spell.name} to LaTeX: ${err.message}`;
    throw err;
  }
}