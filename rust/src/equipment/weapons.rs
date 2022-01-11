use crate::core_mechanics::abilities::{Attack, AttackEffect, Targeting};
use crate::core_mechanics::{DamageDice, DamageType, Defense};
use std::fmt;
use titlecase::titlecase;

#[derive(Clone)]
pub struct Weapon {
    pub accuracy: i32,
    pub damage_dice: DamageDice,
    pub damage_types: Vec<DamageType>,
    pub name: String,
    pub tags: Vec<WeaponTag>,
}

#[derive(Clone)]
pub enum WeaponTag {
    Ammunition,
    Compact,
    Disarming,
    Forceful,
    Grappling,
    Impact,Keen,
    Long,
    Mounted,
    Parrying,
    Projectile(i32, i32),
    Stealthy,
    Sweeping(i32),
    Subdual,
    Thrown(i32, i32),
    Tripping,
    Unarmed,
    VersatileGrip,
}

// Local alias to make weapon definitions shorter, since "Tag" is unambiguous in this file
use WeaponTag as Tag;

#[derive(Copy, Clone)]
pub enum StandardWeapon {
    ArmorSpikes,
    Battleaxe,
    Bite,
    Broadsword,
    Claw,
    Club,
    GiantBoulder,
    Greataxe,
    Greatclub,
    Greatsword,
    HeavyFlail,
    Javelin,
    Longbow,
    MonsterBite,
    MonsterClaws,
    MonsterGore,
    MonsterStinger,
    MonsterTalons,
    Sap,
    Scimitar,
    Sickle,
    Slam,
    Sledgehammer,
    Spear,
    Totokia,
    Warhammer,
}

impl StandardWeapon {
    pub fn weapon(&self) -> Weapon {
        match self {
            Self::ArmorSpikes => Weapon {
                accuracy: 0,
                damage_dice: DamageDice::d4(),
                damage_types: vec![DamageType::Piercing],
                name: "armor spike".to_string(),
                tags: vec![],
            },
            Self::Battleaxe => Weapon {
                accuracy: 0,
                damage_dice: DamageDice::d6(),
                damage_types: vec![DamageType::Slashing],
                name: "battleaxe".to_string(),
                tags: vec![Tag::Sweeping(1), Tag::VersatileGrip],
            },
            Self::Bite => Weapon {
                accuracy: 0,
                damage_dice: DamageDice::d6(),
                damage_types: vec![DamageType::Physical],
                name: "bite".to_string(),
                tags: vec![Tag::Grappling],
            },
            Self::Broadsword => Weapon {
                accuracy: 0,
                damage_dice: DamageDice::d6(),
                damage_types: vec![DamageType::Slashing],
                name: "broadsword".to_string(),
                tags: vec![Tag::Sweeping(1), Tag::VersatileGrip],
            },
            Self::Claw => Weapon {
                accuracy: 2,
                damage_dice: DamageDice::d4(),
                damage_types: vec![DamageType::Slashing],
                name: "claw".to_string(),
                tags: vec![Tag::Grappling],
            },
            Self::Club => Weapon {
                accuracy: 0,
                damage_dice: DamageDice::d6(),
                damage_types: vec![DamageType::Bludgeoning],
                name: "club".to_string(),
                tags: vec![Tag::VersatileGrip],
            },
            Self::GiantBoulder => Weapon {
                // TODO: add Forceful and thrown range limits
                // individual giants can customize these range limits
                accuracy: 0,
                damage_dice: DamageDice::d8(),
                damage_types: vec![DamageType::Bludgeoning],
                name: "boulder".to_string(),
                tags: vec![Tag::Forceful, Tag::Thrown(120, 480)],
            },
            Self::Greataxe => Weapon {
                accuracy: 0,
                damage_dice: DamageDice::d10(),
                damage_types: vec![DamageType::Slashing],
                name: "greataxe".to_string(),
                tags: vec![Tag::Sweeping(1)],
            },
            Self::Greatclub => Weapon {
                accuracy: 0,
                damage_dice: DamageDice::d10(),
                damage_types: vec![DamageType::Bludgeoning],
                name: "greatclub".to_string(),
                tags: vec![Tag::Forceful],
            },
            Self::Greatsword => Weapon {
                accuracy: 0,
                damage_dice: DamageDice::d8(),
                damage_types: vec![DamageType::Slashing],
                name: "greatsword".to_string(),
                tags: vec![Tag::Sweeping(2)],
            },
            Self::HeavyFlail => Weapon {
                accuracy: 0,
                damage_dice: DamageDice::d10(),
                damage_types: vec![DamageType::Bludgeoning],
                name: "heavy flail".to_string(),
                tags: vec![Tag::Tripping],
            },
            Self::Javelin => Weapon {
                accuracy: 1,
                damage_dice: DamageDice::d4(),
                damage_types: vec![DamageType::Piercing],
                name: "javelin".to_string(),
                tags: vec![Tag::Thrown(60, 120)],
            },
            Self::Longbow => Weapon {
                accuracy: 0,
                damage_dice: DamageDice::d6(),
                damage_types: vec![DamageType::Piercing],
                name: "longbow".to_string(),
                tags: vec![Tag::Projectile(120, 480)],
            },
            Self::MonsterBite => Weapon {
                accuracy: 0,
                damage_dice: DamageDice::d8(),
                damage_types: vec![DamageType::Physical],
                name: "bite".to_string(),
                tags: vec![Tag::Grappling],
            },
            Self::MonsterClaws => Weapon {
                accuracy: 3,
                damage_dice: DamageDice::d4(),
                damage_types: vec![DamageType::Slashing],
                name: "claws".to_string(),
                tags: vec![Tag::Grappling],
            },
            Self::MonsterGore => Weapon {
                accuracy: 0,
                damage_dice: DamageDice::d8(),
                damage_types: vec![DamageType::Piercing],
                name: "gore".to_string(),
                tags: vec![Tag::Impact],
            },
            Self::MonsterStinger => Weapon {
                accuracy: 1,
                damage_dice: DamageDice::d8(),
                damage_types: vec![DamageType::Piercing],
                name: "stinger".to_string().to_string(),
                tags: vec![],
            },
            Self::MonsterTalons => Weapon {
                accuracy: 3,
                damage_dice: DamageDice::d4(),
                damage_types: vec![DamageType::Piercing],
                name: "talons".to_string(),
                tags: vec![],
            },
            Self::Sap => Weapon {
                accuracy: 2,
                damage_dice: DamageDice::d4(),
                damage_types: vec![DamageType::Bludgeoning],
                name: "sap".to_string(),
                tags: vec![Tag::Stealthy, Tag::Subdual],
            },
            Self::Scimitar => Weapon {
                accuracy: 0,
                damage_dice: DamageDice::d6(),
                damage_types: vec![DamageType::Slashing],
                name: "scimitar".to_string(),
                tags: vec![Tag::Keen, Tag::Mounted],
            },
            Self::Sickle => Weapon {
                accuracy: 1,
                damage_dice: DamageDice::d4(),
                damage_types: vec![DamageType::Slashing],
                name: "sickle".to_string(),
                tags: vec![Tag::Tripping],
            },
            Self::Slam => Weapon {
                accuracy: 0,
                damage_dice: DamageDice::d8(),
                damage_types: vec![DamageType::Bludgeoning],
                name: "slam".to_string(),
                tags: vec![],
            },
            Self::Sledgehammer => Weapon {
                accuracy: 0,
                damage_dice: DamageDice::d8(),
                damage_types: vec![DamageType::Bludgeoning],
                name: "sledgehammer".to_string(),
                tags: vec![Tag::Forceful],
            },
            Self::Spear => Weapon {
                accuracy: 0,
                damage_dice: DamageDice::d6(),
                damage_types: vec![DamageType::Piercing],
                name: "spear".to_string(),
                tags: vec![Tag::Thrown(30, 60), Tag::VersatileGrip],
            },
            Self::Totokia => Weapon {
                accuracy: 0,
                damage_dice: DamageDice::d8(),
                damage_types: vec![DamageType::Bludgeoning],
                name: "totokia".to_string(),
                tags: vec![Tag::Impact, Tag::VersatileGrip],
            },
            Self::Warhammer => Weapon {
                accuracy: 0,
                damage_dice: DamageDice::d6(),
                damage_types: vec![DamageType::Bludgeoning],
                name: "warhammer".to_string(),
                tags: vec![Tag::Forceful, Tag::VersatileGrip],
            },
        }
    }

    // TODO: add weapon tags
}

impl Weapon {
    pub fn attack(&self) -> Attack {
        return Attack {
            accuracy: self.accuracy,
            cooldown: None,
            crit: None,
            defense: Defense::Armor,
            hit: AttackEffect::from_weapon(self.clone()),
            movement: None,
            name: titlecase(&self.name),
            is_magical: false,
            is_strike: true,
            replaces_weapon: Some(self.clone()),
            targeting: Targeting::Strike,
        };
    }

    pub fn except<F: FnOnce(&mut Self)>(mut self, f: F) -> Self {
        f(&mut self);
        return self;
    }

    pub fn plural_name(&self) -> String {
        return format!("{}s", self.name);
    }
}

impl fmt::Display for Weapon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub enum WeaponGroup {
    Armor,
    Axes,
    Blades,
    Bows,
    Crossbows,
    Clublike,
    Monk,
    Thrown,
}

impl WeaponGroup {
    pub fn name_plural(&self) -> &str {
        match self {
            Self::Armor => "armor",
            Self::Axes => "axes",
            Self::Blades => "blades",
            Self::Bows => "bows",
            Self::Crossbows => "crossbows",
            Self::Clublike => "clublike weapons",
            Self::Monk => "monk weapons",
            Self::Thrown => "thrown weapons",
        }
    }
}

#[derive(Clone)]
pub enum WeaponMaterial {
    Adamantine,
    PureAdamantine,
    ColdIron,
    Normal,
    Diamondsteel,
    PureDiamondsteel,
    Dragonfang(String),
    AncientDragonfang(String),
    Mithral,
    PureMithral,
}

impl WeaponMaterial {
    pub fn name(&self) -> String {
        match self {
            Self::Adamantine => "adamantine".to_string(),
            Self::PureAdamantine => "pure adamantine".to_string(),
            Self::ColdIron => "cold iron".to_string(),
            Self::Normal => "normal".to_string(),
            Self::Diamondsteel => "diamondsteel".to_string(),
            Self::PureDiamondsteel => "pure diamondsteel".to_string(),
            Self::Dragonfang(color) => format!("{} dragonfang", color),
            Self::AncientDragonfang(color) => format!("{} ancient dragonfang", color),
            Self::Mithral => "mithral".to_string(),
            Self::PureMithral => "pure mithral".to_string(),
        }
    }
}

// impl WeaponGroup {
//     fn weapons(&self) -> Vec<Weapon> {
//         match self {
//             Self::Armor => vec![StandardWeapon::ArmorSpikes],
//             Self::Axes => vec![StandardWeapon::Battleaxe, StandardWeapon::Greataxe],
//             Self::Blades => vec![StandardWeapon::Broadsword, StandardWeapon::Greataxe],
//             Self::Bows => vec![StandardWeapon::Longbow],
//             Self::Clublike => vec![StandardWeapon::Sap],
//         }
//     }
// }
