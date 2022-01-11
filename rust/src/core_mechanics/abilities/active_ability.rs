use super::latex::latex_ability_block;
use crate::core_mechanics::abilities::{AbilityTag, AbilityType, Cooldown, UsageTime};

#[derive(Clone)]
pub struct ActiveAbility {
    pub ability_type: AbilityType,
    pub cooldown: Option<Cooldown>,
    pub effect: String,
    pub is_magical: bool,
    pub name: String,
    pub tags: Option<Vec<AbilityTag>>,
    pub usage_time: Option<UsageTime>,
}

// LaTeX generation functions
impl ActiveAbility {
    pub fn latex_ability_block(self) -> String {
        let mut tags = self.tags.unwrap_or(vec![]);
        if self.is_magical {
            tags.push(AbilityTag::Magical);
        };
        return latex_ability_block(
            self.ability_type,
            self.effect,
            tags.iter().map(|t| t.latex()).collect(),
            self.name,
            self.usage_time,
        );
    }
}
