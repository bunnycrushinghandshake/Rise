mod attributes;
mod damage_absorption;
mod damage_dice;
mod damage_types;
mod debuffs;
mod defenses;
mod movement_modes;
mod passive_abilities;
mod resources;
mod senses;
mod sizes;
mod vital_wounds;

pub use attributes::{Attribute, HasAttributes};
pub use damage_absorption::HasDamageAbsorption;
pub use damage_dice::DamageDice;
pub use damage_types::{DamageType, DamageTypeEffect};
pub use debuffs::Debuff;
pub use defenses::{Defense, HasDefenses, SpecialDefenseModifier, SpecialDefenseType};
pub use movement_modes::{FlightManeuverability, MovementMode, SpeedCategory};
pub use passive_abilities::{PassiveAbility, StandardPassiveAbility};
pub use resources::{HasResources, Resource};
pub use senses::Sense;
pub use sizes::Size;
pub use vital_wounds::{HasVitalWounds, VitalWound};
