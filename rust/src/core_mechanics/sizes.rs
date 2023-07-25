use crate::creatures::Creature;

#[derive(Clone, Debug)]
pub enum Size {
    Fine,
    Diminuitive,
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
    Colossal,
}

impl Size {
    pub fn base_speed(&self) -> i32 {
        match self {
            Size::Fine => 5,
            Size::Diminuitive => 10,
            Size::Tiny => 15,
            Size::Small => 20,
            Size::Medium => 30,
            Size::Large => 40,
            Size::Huge => 50,
            Size::Gargantuan => 60,
            Size::Colossal => 80,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Size::Fine => "Fine",
            Size::Diminuitive => "Diminuitive",
            Size::Tiny => "Tiny",
            Size::Small => "Small",
            Size::Medium => "Medium",
            Size::Large => "Large",
            Size::Huge => "Huge",
            Size::Gargantuan => "Gargantuan",
            Size::Colossal => "Colossal",
        }
    }

    pub fn reach(&self, is_long: bool) -> i32 {
        let tall_reach = self.space().floor() as i32;
        if is_long && tall_reach >= 10 {
            tall_reach / 2
        } else {
            tall_reach
        }
    }

    pub fn reflex_modifier(&self) -> i32 {
        match self {
            Size::Fine => 4,
            Size::Diminuitive => 3,
            Size::Tiny => 2,
            Size::Small => 1,
            Size::Medium => 0,
            Size::Large => -1,
            Size::Huge => -2,
            Size::Gargantuan => -3,
            Size::Colossal => -4,
        }
    }

    pub fn space(&self) -> f64 {
        match self {
            Size::Fine => 0.25,
            Size::Diminuitive => 0.5,
            Size::Tiny => 1.0,
            Size::Small => 2.5,
            Size::Medium => 5.0,
            Size::Large => 10.0,
            Size::Huge => 20.0,
            Size::Gargantuan => 40.0,
            Size::Colossal => 80.0,
        }
    }
}

pub trait HasSize {
    fn set_size(&mut self, size: Size);
    fn get_size(&self) -> &Size;
}

impl HasSize for Creature {
    fn set_size(&mut self, size: Size) {
        self.size = size;
    }
    fn get_size(&self) -> &Size {
        &self.size
    }
}
