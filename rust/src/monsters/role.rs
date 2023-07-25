use crate::core_mechanics::Defense;
use crate::creatures::{Creature, HasModifiers, Modifier};

#[derive(Copy, Clone, Debug, Hash)]
pub enum Role {
    Brute,      // +str, melee HP-heavy damage sponge, like barbarian or any heavy weapon user
    Skirmisher, // +dex, high mobility mixed range, like rogue/monk/ranger
    Warrior, // +con, melee or short range defense tank, like a typical sword and board fighter/paladin
    Sniper,  // +per, low mobility long range, like an archer
    Mystic,  // +wil, low HP, high DR, typically a caster
    Leader,  // average in all respects
}

// No clear balancing. Hoping that the role differentiation makes them hard to directly compare.
impl Role {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Brute,
            Self::Skirmisher,
            Self::Warrior,
            Self::Sniper,
            Self::Mystic,
            Self::Leader,
        ]
    }

    // Shorthand to avoid specifying the name and priority every time
    fn add_modifier(&self, creature: &mut Creature, modifier: Modifier) {
        creature.add_modifier(modifier, Some(self.name()), None);
    }

    pub fn set_core_statistics(&self, creature: &mut Creature) {
        for defense in Defense::all() {
            self.add_modifier(creature, Modifier::Defense(defense, self.defense(&defense)))
        }
        self.add_modifier(
            creature,
            Modifier::DamageResistanceFromLevel(self.damage_resistance()),
        );
        self.add_modifier(creature, Modifier::HitPointsFromLevel(self.hit_points()));
    }

    pub fn defense(&self, defense: &Defense) -> i32 {
        // order: Armor, Fort, Ref, Ment
        let defenses = match self {
            Role::Brute => [4, 4, 4, 4],
            Role::Skirmisher => [4, 4, 6, 4],
            Role::Warrior => [6, 4, 4, 4],
            Role::Sniper => [3, 4, 4, 4],
            Role::Mystic => [2, 4, 4, 6],
            Role::Leader => [4, 4, 4, 4],
        };

        let i = match defense {
            Defense::Armor => 0,
            Defense::Fortitude => 1,
            Defense::Reflex => 2,
            Defense::Mental => 3,
        };
        defenses[i]
    }

    pub fn damage_resistance(&self) -> i32 {
        match self {
            Role::Brute => 0,
            Role::Skirmisher => 2,
            Role::Warrior => 4,
            Role::Sniper => 0,
            Role::Mystic => 4,
            Role::Leader => 2,
        }
    }

    pub fn hit_points(&self) -> i32 {
        match self {
            Role::Brute => 4,
            Role::Skirmisher => 0,
            Role::Warrior => 2,
            Role::Sniper => 0,
            Role::Mystic => 0,
            Role::Leader => 2,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Role::Brute => "Brute",
            Role::Skirmisher => "Skirmisher",
            Role::Warrior => "Warrior",
            Role::Sniper => "Sniper",
            Role::Mystic => "Mystic",
            Role::Leader => "Leader",
        }
    }
}
