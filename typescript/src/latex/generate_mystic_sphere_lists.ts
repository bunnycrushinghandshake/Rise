import { MysticSphere, mysticSpheres, Source } from "@src/mystic_spheres";
import { titleCase } from "change-case";

export function generateMysticSphereLists(): string {
  const sources: Source[] = ["arcane", "divine", "nature", "pact"];
  return sources.map(generateSourceList).join("\n");
}

function generateSourceList(source: Source): string {
  const sourceSpheres = mysticSpheres.filter((sphere) => sphere.sources.includes(source));
  return `
    {
    \\RaggedRight
    \\subsection{${titleCase(source)}}\\label{${titleCase(source)}}

    ${sourceSpheres.map(formatSphere).join("\n")}
    ${
      source === "divine"
        ? `
          \\subsubsection{Domain Spheres}
          ${mysticSpheres
            .filter((sphere) => sphere.sources.includes("domain"))
            .map(formatSphere)
            .join("\n")}
        `
        : ""
    }
  `;
}

function formatSphere(sphere: MysticSphere): string {
  return `\\par\\noindent \\sphere{${sphere.name}}: ${sphere.shortDescription}`;
}
