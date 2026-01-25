//! Combat system: status effects and combat calculations

use serde::{Serialize, Deserialize};

/// Status effects that can affect entities
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum StatusEffect {
    Poison,
    Burn,
    Freeze,
    Bleed,
    Stun,
    Blind,
    Haste,
    Shield,
    Regeneration,
    Strength,
    Weakness,
    Invisibility,
    Confusion,
}

impl StatusEffect {
    /// Returns the display name of the status effect
    pub fn name(&self) -> &'static str {
        match self {
            Self::Poison => "Poisoned",
            Self::Burn => "Burning",
            Self::Freeze => "Frozen",
            Self::Bleed => "Bleeding",
            Self::Stun => "Stunned",
            Self::Blind => "Blind",
            Self::Haste => "Haste",
            Self::Shield => "Shielded",
            Self::Regeneration => "Regenerating",
            Self::Strength => "Strengthened",
            Self::Weakness => "Weakened",
            Self::Invisibility => "Invisible",
            Self::Confusion => "Confused",
        }
    }

    /// Returns a color index for the status effect (for UI rendering)
    /// 0=DarkGrey, 1=Grey, 2=White, 3=Red, 4=DarkRed, 5=Green, 6=DarkGreen,
    /// 7=Blue, 8=DarkBlue, 9=Cyan, 10=DarkCyan, 11=Yellow, 12=DarkYellow,
    /// 13=Magenta, 14=DarkMagenta
    pub fn color_index(&self) -> u8 {
        match self {
            Self::Poison => 5,       // Green
            Self::Burn => 3,         // Red
            Self::Freeze => 9,       // Cyan
            Self::Bleed => 4,        // DarkRed
            Self::Stun => 11,        // Yellow
            Self::Blind => 0,        // DarkGrey
            Self::Haste => 7,        // Blue
            Self::Shield => 2,       // White
            Self::Regeneration => 13, // Magenta
            Self::Strength => 11,    // Yellow
            Self::Weakness => 14,    // DarkMagenta
            Self::Invisibility => 1, // Grey
            Self::Confusion => 12,   // DarkYellow
        }
    }

    /// Returns whether this is a harmful effect
    pub fn is_harmful(&self) -> bool {
        matches!(
            self,
            Self::Poison | Self::Burn | Self::Freeze | Self::Bleed |
            Self::Stun | Self::Blind | Self::Weakness | Self::Confusion
        )
    }

    /// Returns whether this is a beneficial effect
    pub fn is_beneficial(&self) -> bool {
        matches!(
            self,
            Self::Haste | Self::Shield | Self::Regeneration |
            Self::Strength | Self::Invisibility
        )
    }

    /// Returns damage per tick for damage-over-time effects
    pub fn damage_per_tick(&self) -> i32 {
        match self {
            Self::Poison => 2,
            Self::Burn => 3,
            Self::Bleed => 1,
            _ => 0,
        }
    }

    /// Returns healing per tick for healing-over-time effects
    pub fn healing_per_tick(&self) -> i32 {
        match self {
            Self::Regeneration => 3,
            _ => 0,
        }
    }
}

/// Calculate damage after applying defense
pub fn calculate_damage(attack: i32, defense: i32) -> i32 {
    (attack - defense).max(1)
}

/// Calculate damage with strength modifier
pub fn calculate_damage_with_strength(base_attack: i32, has_strength: bool, has_weakness: bool) -> i32 {
    let mut attack = base_attack;
    if has_strength {
        attack = (attack as f32 * 1.5) as i32;
    }
    if has_weakness {
        attack = (attack as f32 * 0.5) as i32;
    }
    attack.max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_damage() {
        assert_eq!(calculate_damage(5, 100), 1);
        assert_eq!(calculate_damage(0, 10), 1);
    }

    #[test]
    fn test_normal_damage() {
        assert_eq!(calculate_damage(10, 3), 7);
        assert_eq!(calculate_damage(20, 5), 15);
    }

    #[test]
    fn test_strength_modifier() {
        let base = 10;
        assert_eq!(calculate_damage_with_strength(base, true, false), 15);
        assert_eq!(calculate_damage_with_strength(base, false, true), 5);
        assert_eq!(calculate_damage_with_strength(base, true, true), 7); // 10 * 1.5 * 0.5 = 7.5 -> 7
    }
}
