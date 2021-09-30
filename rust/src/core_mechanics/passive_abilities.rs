#[derive(Clone)]
pub struct PassiveAbility {
    pub description: String,
    pub is_magical: bool,
    pub name: String,
}

impl PassiveAbility {
    pub fn to_latex(&self) -> String {
        return format!(
            "
                \\parhead<{name}>{magical} {description}
            ",
            description = self.description,
            magical = if self.is_magical {
                "[\\glossterm<Magical>]"
            } else {
                ""
            },
            name = self.name,
        );
    }
}

pub enum StandardPassiveAbility {
    Amphibious,
    Incorporeal,
    TwoActions,
    ThreeActions,
    Undead,
}

impl StandardPassiveAbility {
    pub fn ability(&self) -> PassiveAbility {
        match self {
            Self::Amphibious => PassiveAbility {
                description: "The $name can hold its breath for ten times the normal length of time.".to_string(),
                is_magical: false,
                name: "Amphibious".to_string(),
            },
            Self::Incorporeal => PassiveAbility {
                description: r"
                  The $name is \glossterm{incorporeal}.
                  It does not have a tangible body, and is \glossterm{impervious} to \glossterm{physical damage}.
                  It can enter or pass through solid objects.
                ".to_string(),
                is_magical: false,
                name: "Incorporeal".to_string(),
            },
            Self::ThreeActions => PassiveAbility {
                description: "The $name can take three standard actions each round. It cannot use the same ability or weapon twice in the same round.".to_string(),
                is_magical: false,
                name: "Multiple Actions".to_string(),
            },
            Self::TwoActions => PassiveAbility {
                description: "The $name can take two standard actions each round. It cannot use the same ability or weapon twice in the same round.".to_string(),
                is_magical: false,
                name: "Multiple Actions".to_string(),
            },
            Self::Undead => PassiveAbility {
                description: r"
                  The $name is \glossterm{undead}.
                  It is not a \glossterm{living} creature, and it is immune to \abilitytag{Compulsion} and \abilitytag{Emotion} abilities.
                  However, it is affected in a special way by spells from the \sphere{vivimancy} \glossterm{mystic sphere} (see \pcref{Vivimancy}).
                ".to_string(),
                is_magical: false,
                name: "Undead".to_string(),
            },
        }
    }
}
