//! Magic and skill system for the ShadowCrypt roguelike
//!
//! This module defines the various skills and magical abilities
//! that characters can use in combat and exploration.

use serde::{Serialize, Deserialize};
use crate::classes::CharacterClass;

/// Represents the different skills available in the game
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum Skill {
    // Warrior skills
    /// Increases damage dealt but also damage taken
    Berserk,
    /// Attacks all adjacent enemies
    Cleave,
    /// Stuns an enemy and deals damage
    ShieldBash,
    /// Attacks all enemies in a radius
    Whirlwind,

    // Mage skills
    /// Launches a ball of fire dealing AoE damage
    Fireball,
    /// Launches an ice projectile that can freeze
    IceSpear,
    /// Calls down lightning on enemies
    Lightning,
    /// Instantly moves to a new location
    Teleport,

    // Rogue skills
    /// Deals massive damage from behind
    Backstab,
    /// Teleports behind an enemy
    ShadowStep,
    /// Coats weapon in poison
    PoisonBlade,
    /// Becomes invisible temporarily
    Vanish,

    // Paladin skills
    /// Heals self and damages undead nearby
    HolyLight,
    /// Creates a protective shield
    DivineShield,
    /// Deals holy damage to a single target
    Smite,
    /// Creates holy ground that damages undead
    Consecrate,

    // Ranger skills
    /// Fires multiple arrows at once
    MultiShot,
    /// Fires a poisoned arrow
    PoisonArrow,
    /// Places a trap on the ground
    TrapSet,
    /// Reveals the map in a large radius
    EagleEye,

    // Necromancer skills
    /// Raises a skeleton minion
    RaiseDead,
    /// Drains life from an enemy
    LifeDrain,
    /// Curses an enemy, weakening them
    Curse,
    /// Sacrifices HP for mana and power
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

    /// Returns the skills available for a given character class
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

    /// Returns a description of what the skill does
    pub fn description(&self) -> &'static str {
        match self {
            Self::Berserk => "Enter a berserker rage, dealing double damage but taking 50% more.",
            Self::Cleave => "Swing your weapon in a wide arc, hitting all adjacent enemies.",
            Self::ShieldBash => "Bash an enemy with your shield, stunning them for 2 turns.",
            Self::Whirlwind => "Spin rapidly, attacking all enemies within 2 tiles.",
            Self::Fireball => "Launch a ball of fire that explodes on impact, dealing AoE damage.",
            Self::IceSpear => "Hurl a spear of ice that can freeze enemies solid.",
            Self::Lightning => "Call down a bolt of lightning on a target.",
            Self::Teleport => "Instantly teleport to a visible location.",
            Self::Backstab => "Deal triple damage when attacking from behind.",
            Self::ShadowStep => "Teleport behind an enemy for a devastating strike.",
            Self::PoisonBlade => "Coat your weapon in poison for the next 5 attacks.",
            Self::Vanish => "Become invisible for 5 turns.",
            Self::HolyLight => "Heal yourself and damage nearby undead.",
            Self::DivineShield => "Create a shield that absorbs the next 3 hits.",
            Self::Smite => "Strike a foe with holy energy, extra effective against undead.",
            Self::Consecrate => "Bless the ground, creating a zone that heals allies and hurts undead.",
            Self::MultiShot => "Fire 3 arrows at once at different targets.",
            Self::PoisonArrow => "Fire a poisoned arrow that deals damage over time.",
            Self::TrapSet => "Place a trap that damages and immobilizes enemies.",
            Self::EagleEye => "Greatly increase your vision range and reveal hidden enemies.",
            Self::RaiseDead => "Raise a fallen enemy as your skeletal minion.",
            Self::LifeDrain => "Drain life from an enemy, healing yourself.",
            Self::Curse => "Curse an enemy, reducing their attack and defense.",
            Self::DarkPact => "Sacrifice HP to greatly restore mana and increase spell damage.",
        }
    }

    /// Returns the cooldown in turns (0 means no cooldown)
    pub fn cooldown(&self) -> u32 {
        match self {
            Self::Berserk => 10,
            Self::Cleave => 3,
            Self::ShieldBash => 5,
            Self::Whirlwind => 8,
            Self::Fireball => 4,
            Self::IceSpear => 3,
            Self::Lightning => 5,
            Self::Teleport => 15,
            Self::Backstab => 0, // Always available when behind
            Self::ShadowStep => 6,
            Self::PoisonBlade => 10,
            Self::Vanish => 20,
            Self::HolyLight => 8,
            Self::DivineShield => 15,
            Self::Smite => 4,
            Self::Consecrate => 12,
            Self::MultiShot => 5,
            Self::PoisonArrow => 4,
            Self::TrapSet => 8,
            Self::EagleEye => 10,
            Self::RaiseDead => 20,
            Self::LifeDrain => 6,
            Self::Curse => 8,
            Self::DarkPact => 25,
        }
    }

    /// Returns the range of the skill (0 means melee/self)
    pub fn range(&self) -> i32 {
        match self {
            Self::Berserk | Self::Cleave | Self::ShieldBash | Self::Whirlwind => 0,
            Self::Fireball | Self::Lightning => 8,
            Self::IceSpear => 6,
            Self::Teleport => 10,
            Self::Backstab | Self::ShadowStep => 1,
            Self::PoisonBlade | Self::Vanish => 0,
            Self::HolyLight | Self::DivineShield | Self::Consecrate => 0,
            Self::Smite => 5,
            Self::MultiShot | Self::PoisonArrow => 8,
            Self::TrapSet => 0,
            Self::EagleEye => 0,
            Self::RaiseDead => 3,
            Self::LifeDrain => 4,
            Self::Curse => 6,
            Self::DarkPact => 0,
        }
    }
}

impl std::fmt::Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Represents an element/damage type for spells
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum Element {
    Physical,
    Fire,
    Ice,
    Lightning,
    Holy,
    Dark,
    Poison,
}

impl Element {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Physical => "Physical",
            Self::Fire => "Fire",
            Self::Ice => "Ice",
            Self::Lightning => "Lightning",
            Self::Holy => "Holy",
            Self::Dark => "Dark",
            Self::Poison => "Poison",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_names() {
        assert_eq!(Skill::Fireball.name(), "Fireball");
        assert_eq!(Skill::RaiseDead.name(), "Raise Dead");
    }

    #[test]
    fn test_skill_mana_cost() {
        assert_eq!(Skill::Berserk.mana_cost(), 10);
        assert_eq!(Skill::DarkPact.mana_cost(), 50);
    }

    #[test]
    fn test_skills_for_class() {
        let mage_skills = Skill::for_class(CharacterClass::Mage);
        assert!(mage_skills.contains(&Skill::Fireball));
        assert!(!mage_skills.contains(&Skill::Berserk));
    }
}
