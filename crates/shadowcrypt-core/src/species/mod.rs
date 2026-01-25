//! Species system for the ShadowCrypt roguelike
//!
//! This module defines different species that characters (players and NPCs) can be.
//! Species affect base stats, abilities, and available classes.

use serde::{Serialize, Deserialize};

/// Represents the different species available in the game
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum Species {
    /// Standard human - balanced stats, versatile class options
    Human,
    /// Elven - higher mana and dexterity, lower HP
    Elf,
    /// Dwarven - higher HP and defense, lower mana
    Dwarf,
    /// Orcish - higher attack and HP, lower mana and defense
    Orc,
    /// Undead - resistant to poison, weak to holy
    Undead,
    /// Demon - high attack and mana, weak to holy
    Demon,
}

impl Species {
    /// Returns the display name of the species
    pub fn name(&self) -> &'static str {
        match self {
            Self::Human => "Human",
            Self::Elf => "Elf",
            Self::Dwarf => "Dwarf",
            Self::Orc => "Orc",
            Self::Undead => "Undead",
            Self::Demon => "Demon",
        }
    }

    /// Returns the base stat modifiers (hp_mod, attack_mod, defense_mod, mana_mod, speed_mod)
    /// These are percentage multipliers (100 = no change)
    pub fn stat_modifiers(&self) -> (i32, i32, i32, i32, i32) {
        match self {
            Self::Human => (100, 100, 100, 100, 100),
            Self::Elf => (85, 95, 90, 130, 115),
            Self::Dwarf => (120, 105, 115, 80, 90),
            Self::Orc => (115, 120, 95, 70, 100),
            Self::Undead => (100, 100, 100, 90, 85),
            Self::Demon => (95, 115, 90, 120, 100),
        }
    }

    /// Returns a description of the species
    pub fn description(&self) -> &'static str {
        match self {
            Self::Human => "Adaptable and versatile, humans excel at all classes.",
            Self::Elf => "Graceful and magical, elves have enhanced mana and speed.",
            Self::Dwarf => "Sturdy and resilient, dwarves have high HP and defense.",
            Self::Orc => "Fierce and powerful, orcs deal devastating damage.",
            Self::Undead => "Risen from death, immune to poison but weak to holy.",
            Self::Demon => "Infernal beings with great power and magical affinity.",
        }
    }

    /// Check if this species is immune to a given damage type
    pub fn is_immune_to(&self, damage_type: &str) -> bool {
        match (self, damage_type) {
            (Self::Undead, "poison") => true,
            (Self::Demon, "fire") => true,
            _ => false,
        }
    }

    /// Check if this species is vulnerable to a given damage type
    pub fn is_vulnerable_to(&self, damage_type: &str) -> bool {
        match (self, damage_type) {
            (Self::Undead, "holy") => true,
            (Self::Demon, "holy") => true,
            _ => false,
        }
    }

    /// Returns all available species
    pub fn all() -> &'static [Species] {
        &[
            Species::Human,
            Species::Elf,
            Species::Dwarf,
            Species::Orc,
            Species::Undead,
            Species::Demon,
        ]
    }
}

impl Default for Species {
    fn default() -> Self {
        Self::Human
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_species_names() {
        assert_eq!(Species::Human.name(), "Human");
        assert_eq!(Species::Elf.name(), "Elf");
    }

    #[test]
    fn test_species_modifiers() {
        let (hp, _, _, _, _) = Species::Dwarf.stat_modifiers();
        assert!(hp > 100); // Dwarves should have more HP
    }

    #[test]
    fn test_species_immunities() {
        assert!(Species::Undead.is_immune_to("poison"));
        assert!(!Species::Human.is_immune_to("poison"));
    }
}
