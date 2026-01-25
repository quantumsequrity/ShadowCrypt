//! Magic system: skills and spells

use crate::classes::CharacterClass;
use serde::{Serialize, Deserialize};

/// Available skills in the game
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Skill {
    // Warrior
    Berserk,
    Cleave,
    ShieldBash,
    Whirlwind,

    // Mage
    Fireball,
    IceSpear,
    Lightning,
    Teleport,

    // Rogue
    Backstab,
    ShadowStep,
    PoisonBlade,
    Vanish,

    // Paladin
    HolyLight,
    DivineShield,
    Smite,
    Consecrate,

    // Ranger
    MultiShot,
    PoisonArrow,
    TrapSet,
    EagleEye,

    // Necromancer
    RaiseDead,
    LifeDrain,
    Curse,
    DarkPact,
}

impl Skill {
    /// Returns the display name of the skill
    pub fn name(&self) -> &'static str {
        match self {
            Self::Berserk => "Berserk",
            Self::Cleave => "Cleave",
            Self::ShieldBash => "Shield Bash",
            Self::Whirlwind => "Whirlwind",
            Self::Fireball => "Fireball",
            Self::IceSpear => "Ice Spear",
            Self::Lightning => "Lightning",
            Self::Teleport => "Teleport",
            Self::Backstab => "Backstab",
            Self::ShadowStep => "Shadow Step",
            Self::PoisonBlade => "Poison Blade",
            Self::Vanish => "Vanish",
            Self::HolyLight => "Holy Light",
            Self::DivineShield => "Divine Shield",
            Self::Smite => "Smite",
            Self::Consecrate => "Consecrate",
            Self::MultiShot => "Multi-Shot",
            Self::PoisonArrow => "Poison Arrow",
            Self::TrapSet => "Set Trap",
            Self::EagleEye => "Eagle Eye",
            Self::RaiseDead => "Raise Dead",
            Self::LifeDrain => "Life Drain",
            Self::Curse => "Curse",
            Self::DarkPact => "Dark Pact",
        }
    }

    /// Returns the mana cost of the skill
    pub fn mana_cost(&self) -> i32 {
        match self {
            Self::Berserk | Self::Cleave | Self::ShieldBash => 10,
            Self::Whirlwind => 25,
            Self::Fireball | Self::IceSpear | Self::Lightning => 20,
            Self::Teleport => 30,
            Self::Backstab | Self::ShadowStep | Self::PoisonBlade => 15,
            Self::Vanish => 25,
            Self::HolyLight | Self::DivineShield | Self::Smite => 20,
            Self::Consecrate => 35,
            Self::MultiShot | Self::PoisonArrow | Self::TrapSet => 15,
            Self::EagleEye => 10,
            Self::RaiseDead => 40,
            Self::LifeDrain | Self::Curse => 20,
            Self::DarkPact => 50,
        }
    }

    /// Returns the skills available to a given class
    pub fn for_class(class: CharacterClass) -> Vec<Self> {
        match class {
            CharacterClass::Warrior => vec![Self::Berserk, Self::Cleave, Self::ShieldBash, Self::Whirlwind],
            CharacterClass::Mage => vec![Self::Fireball, Self::IceSpear, Self::Lightning, Self::Teleport],
            CharacterClass::Rogue => vec![Self::Backstab, Self::ShadowStep, Self::PoisonBlade, Self::Vanish],
            CharacterClass::Paladin => vec![Self::HolyLight, Self::DivineShield, Self::Smite, Self::Consecrate],
            CharacterClass::Ranger => vec![Self::MultiShot, Self::PoisonArrow, Self::TrapSet, Self::EagleEye],
            CharacterClass::Necromancer => vec![Self::RaiseDead, Self::LifeDrain, Self::Curse, Self::DarkPact],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_classes_have_skills() {
        for class in CharacterClass::all() {
            let skills = Skill::for_class(class);
            assert!(!skills.is_empty(), "Class {:?} has no skills", class);
            assert_eq!(skills.len(), 4, "Class {:?} should have 4 skills", class);
        }
    }

    #[test]
    fn test_skill_mana_costs_positive() {
        for class in CharacterClass::all() {
            for skill in Skill::for_class(class) {
                assert!(skill.mana_cost() > 0, "Skill {:?} has non-positive mana cost", skill);
            }
        }
    }
}
