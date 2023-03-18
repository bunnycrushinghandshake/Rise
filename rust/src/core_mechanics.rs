mod attributes;
pub mod abilities;
pub mod attacks;
mod damage_absorption;
mod damage_dice;
mod dice_pool;
mod damage_types;
mod debuffs;
mod defenses;
mod movement;
mod passive_abilities;
mod resources;
mod senses;
mod sizes;
mod tag;
mod vital_wounds;

pub use attributes::{Attribute, HasAttributes};
pub use damage_absorption::HasDamageAbsorption;
pub use damage_dice::DamageDice;
pub use damage_types::{DamageType, DamageTypeEffect};
pub use debuffs::Debuff;
pub use defenses::{Defense, HasDefenses, SpecialDefenses, SpecialDefenseType};
pub use dice_pool::{DicePool, Die};
pub use movement::{FlightManeuverability, MovementMode, MovementSpeed, SpeedCategory};
pub use passive_abilities::{PassiveAbility, StandardPassiveAbility};
pub use resources::{HasResources, Resource};
pub use senses::Sense;
pub use sizes::{HasSize, Size};
pub use tag::Tag;
pub use vital_wounds::{HasVitalWounds, VitalWound};
