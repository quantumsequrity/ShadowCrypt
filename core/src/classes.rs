//! Character classes and their properties

use serde::{Serialize, Deserialize};

/// Available character classes in the game
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum CharacterClass {
    Warrior,
    Mage,
    Rogue,
    Paladin,
    Ranger,
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

    /// Returns base stats: (hp, attack, defense, mana, speed)
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

    /// Returns the description of the class's special ability
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

    /// Returns an iterator over all available classes
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::Warrior,
            Self::Mage,
            Self::Rogue,
            Self::Paladin,
            Self::Ranger,
            Self::Necromancer,
        ].into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_classes_have_names() {
        for class in CharacterClass::all() {
            assert!(!class.name().is_empty());
        }
    }

    #[test]
    fn test_base_stats_positive() {
        for class in CharacterClass::all() {
            let (hp, atk, def, mana, spd) = class.base_stats();
            assert!(hp > 0);
            assert!(atk > 0);
            assert!(def >= 0);
            assert!(mana >= 0);
            assert!(spd > 0);
        }
    }
}
