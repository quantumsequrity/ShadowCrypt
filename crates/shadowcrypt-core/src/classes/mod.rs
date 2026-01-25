//! Character class system for the ShadowCrypt roguelike
//!
//! This module defines the different character classes available to players,
//! including their base stats and special abilities.

use serde::{Serialize, Deserialize};

/// Represents the different character classes available in the game
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum CharacterClass {
    /// Melee-focused fighter with high HP and attack
    Warrior,
    /// Magic user with high mana and powerful spells
    Mage,
    /// Stealthy class with high speed and critical damage
    Rogue,
    /// Holy warrior with healing and anti-undead abilities
    Paladin,
    /// Ranged fighter with bows and nature magic
    Ranger,
    /// Dark magic user who can raise and control the dead
    Necromancer,
}

impl CharacterClass {
    /// Returns the display name of the class
    pub fn name(&self) -> &'static str {
        match self {
            Self::Warrior => "Warrior",
            Self::Mage => "Mage",
            Self::Rogue => "Rogue",
            Self::Paladin => "Paladin",
            Self::Ranger => "Ranger",
            Self::Necromancer => "Necromancer",
        }
    }

    /// Returns the base stats for this class
    /// Returns (hp, attack, defense, mana, speed)
    pub fn base_stats(&self) -> (i32, i32, i32, i32, i32) {
        match self {
            Self::Warrior => (50, 8, 5, 10, 10),
            Self::Mage => (30, 3, 2, 50, 10),
            Self::Rogue => (35, 6, 3, 20, 15),
            Self::Paladin => (45, 6, 6, 30, 8),
            Self::Ranger => (38, 7, 3, 25, 12),
            Self::Necromancer => (32, 4, 2, 45, 9),
        }
    }

    /// Returns a description of the class's special ability
    pub fn special_ability(&self) -> &'static str {
        match self {
            Self::Warrior => "Berserk (2x damage, take 50% more)",
            Self::Mage => "Fireball (AoE damage)",
            Self::Rogue => "Backstab (3x damage from behind)",
            Self::Paladin => "Holy Light (heal + damage undead)",
            Self::Ranger => "Multi-shot (hit 3 enemies)",
            Self::Necromancer => "Raise Dead (summon skeleton)",
        }
    }

    /// Returns a detailed description of the class
    pub fn description(&self) -> &'static str {
        match self {
            Self::Warrior => "A stalwart fighter who excels in close combat. \
                              Warriors have the highest HP and strong melee attacks.",
            Self::Mage => "A master of arcane arts. Mages have low HP but \
                          tremendous magical power and a large mana pool.",
            Self::Rogue => "A swift and cunning fighter. Rogues are fast \
                           and deal devastating critical hits from the shadows.",
            Self::Paladin => "A holy warrior blessed with divine power. \
                             Paladins can heal and are especially effective against undead.",
            Self::Ranger => "A skilled archer and tracker. Rangers excel at \
                            ranged combat and have useful nature-based abilities.",
            Self::Necromancer => "A dark mage who commands the dead. Necromancers \
                                 can raise fallen enemies to fight alongside them.",
        }
    }

    /// Returns all available character classes
    pub fn all() -> &'static [CharacterClass] {
        &[
            CharacterClass::Warrior,
            CharacterClass::Mage,
            CharacterClass::Rogue,
            CharacterClass::Paladin,
            CharacterClass::Ranger,
            CharacterClass::Necromancer,
        ]
    }

    /// Returns the recommended playstyle for this class
    pub fn playstyle(&self) -> &'static str {
        match self {
            Self::Warrior => "Aggressive melee combat",
            Self::Mage => "Ranged magic, avoid close combat",
            Self::Rogue => "Hit and run, use stealth",
            Self::Paladin => "Balanced combat with healing",
            Self::Ranger => "Maintain distance, use traps",
            Self::Necromancer => "Summon minions, stay back",
        }
    }
}

impl Default for CharacterClass {
    fn default() -> Self {
        Self::Warrior
    }
}

impl std::fmt::Display for CharacterClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_names() {
        assert_eq!(CharacterClass::Warrior.name(), "Warrior");
        assert_eq!(CharacterClass::Necromancer.name(), "Necromancer");
    }

    #[test]
    fn test_class_stats() {
        let (hp, attack, defense, mana, speed) = CharacterClass::Warrior.base_stats();
        assert_eq!(hp, 50);
        assert_eq!(attack, 8);
        assert_eq!(defense, 5);
        assert_eq!(mana, 10);
        assert_eq!(speed, 10);
    }

    #[test]
    fn test_mage_has_high_mana() {
        let (_, _, _, mana, _) = CharacterClass::Mage.base_stats();
        let (_, _, _, warrior_mana, _) = CharacterClass::Warrior.base_stats();
        assert!(mana > warrior_mana);
    }

    #[test]
    fn test_all_classes() {
        let classes = CharacterClass::all();
        assert_eq!(classes.len(), 6);
    }
}
