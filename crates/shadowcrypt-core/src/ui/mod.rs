//! UI utilities for the ShadowCrypt roguelike
//!
//! This module provides color definitions and rendering utilities
//! that can be used by both CLI and GUI frontends.

use crate::items::Rarity;
use crate::combat::{StatusEffect, EnemyKind};
use crate::world::Tile;

/// RGB color representation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    // Standard colors
    pub const BLACK: Color = Color::new(0, 0, 0);
    pub const WHITE: Color = Color::new(255, 255, 255);
    pub const GREY: Color = Color::new(128, 128, 128);
    pub const DARK_GREY: Color = Color::new(64, 64, 64);
    pub const RED: Color = Color::new(255, 0, 0);
    pub const DARK_RED: Color = Color::new(139, 0, 0);
    pub const GREEN: Color = Color::new(0, 255, 0);
    pub const DARK_GREEN: Color = Color::new(0, 100, 0);
    pub const BLUE: Color = Color::new(0, 0, 255);
    pub const DARK_BLUE: Color = Color::new(0, 0, 139);
    pub const YELLOW: Color = Color::new(255, 255, 0);
    pub const DARK_YELLOW: Color = Color::new(139, 139, 0);
    pub const CYAN: Color = Color::new(0, 255, 255);
    pub const DARK_CYAN: Color = Color::new(0, 139, 139);
    pub const MAGENTA: Color = Color::new(255, 0, 255);
    pub const DARK_MAGENTA: Color = Color::new(139, 0, 139);
    pub const ORANGE: Color = Color::new(255, 165, 0);
    pub const BROWN: Color = Color::new(139, 69, 19);
    pub const GOLD: Color = Color::new(255, 215, 0);
    pub const PINK: Color = Color::new(255, 192, 203);
    pub const PURPLE: Color = Color::new(128, 0, 128);

    /// Converts to a u32 for certain rendering systems (0xRRGGBB)
    pub fn to_u32(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    /// Creates from a u32 (0xRRGGBB)
    pub fn from_u32(value: u32) -> Self {
        Self {
            r: ((value >> 16) & 0xFF) as u8,
            g: ((value >> 8) & 0xFF) as u8,
            b: (value & 0xFF) as u8,
        }
    }

    /// Blends this color with another
    pub fn blend(&self, other: Color, factor: f32) -> Color {
        let factor = factor.clamp(0.0, 1.0);
        Color {
            r: (self.r as f32 * (1.0 - factor) + other.r as f32 * factor) as u8,
            g: (self.g as f32 * (1.0 - factor) + other.g as f32 * factor) as u8,
            b: (self.b as f32 * (1.0 - factor) + other.b as f32 * factor) as u8,
        }
    }

    /// Darkens the color by a factor (0.0 = no change, 1.0 = black)
    pub fn darken(&self, factor: f32) -> Color {
        self.blend(Color::BLACK, factor)
    }

    /// Lightens the color by a factor (0.0 = no change, 1.0 = white)
    pub fn lighten(&self, factor: f32) -> Color {
        self.blend(Color::WHITE, factor)
    }
}

/// Returns the color for a tile
pub fn tile_color(tile: Tile) -> Color {
    match tile {
        Tile::Wall => Color::DARK_GREY,
        Tile::Floor => Color::GREY,
        Tile::StairsDown => Color::CYAN,
        Tile::StairsUp => Color::CYAN,
        Tile::Door => Color::YELLOW,
        Tile::OpenDoor => Color::DARK_YELLOW,
        Tile::Trap => Color::RED,
        Tile::DisarmedTrap => Color::DARK_GREY,
        Tile::Water => Color::BLUE,
        Tile::Lava => Color::RED,
        Tile::Chest => Color::YELLOW,
        Tile::OpenChest => Color::DARK_YELLOW,
        Tile::Shrine => Color::MAGENTA,
        Tile::UsedShrine => Color::DARK_MAGENTA,
        Tile::Pillar => Color::WHITE,
        Tile::Grass => Color::GREEN,
        Tile::Ice => Color::CYAN,
        Tile::Sand => Color::YELLOW,
        Tile::BossGate => Color::RED,
    }
}

/// Returns the color for a status effect
pub fn status_effect_color(effect: StatusEffect) -> Color {
    match effect {
        StatusEffect::Poison => Color::GREEN,
        StatusEffect::Burn => Color::RED,
        StatusEffect::Freeze => Color::CYAN,
        StatusEffect::Bleed => Color::DARK_RED,
        StatusEffect::Stun => Color::YELLOW,
        StatusEffect::Blind => Color::DARK_GREY,
        StatusEffect::Haste => Color::BLUE,
        StatusEffect::Shield => Color::WHITE,
        StatusEffect::Regeneration => Color::MAGENTA,
        StatusEffect::Strength => Color::YELLOW,
        StatusEffect::Weakness => Color::DARK_MAGENTA,
        StatusEffect::Invisibility => Color::GREY,
        StatusEffect::Confusion => Color::DARK_YELLOW,
    }
}

/// Returns the color for a rarity level
pub fn rarity_color(rarity: Rarity) -> Color {
    match rarity {
        Rarity::Common => Color::GREY,
        Rarity::Uncommon => Color::GREEN,
        Rarity::Rare => Color::BLUE,
        Rarity::Epic => Color::MAGENTA,
        Rarity::Legendary => Color::YELLOW,
        Rarity::Mythic => Color::RED,
    }
}

/// Returns the color for an enemy type
pub fn enemy_color(kind: EnemyKind) -> Color {
    match kind {
        EnemyKind::Rat | EnemyKind::Bat | EnemyKind::GiantRat | EnemyKind::CaveCrawler => Color::DARK_GREY,
        EnemyKind::Spider | EnemyKind::GiantSpider | EnemyKind::IceSpider => Color::DARK_YELLOW,
        EnemyKind::Goblin | EnemyKind::BossGoblinKing | EnemyKind::GoblinChampion | EnemyKind::Kobold => Color::GREEN,
        EnemyKind::Skeleton | EnemyKind::Mummy | EnemyKind::BoneGolem => Color::WHITE,
        EnemyKind::Orc | EnemyKind::BossOrcWarlord | EnemyKind::OrcBerserker | EnemyKind::Hobgoblin => Color::DARK_GREEN,
        EnemyKind::Troll | EnemyKind::ForestTroll => Color::DARK_CYAN,
        EnemyKind::CaveOgre | EnemyKind::CaveBear => Color::DARK_YELLOW,
        EnemyKind::Slime | EnemyKind::Mushroom => Color::GREEN,
        EnemyKind::RockElemental => Color::GREY,
        EnemyKind::Zombie | EnemyKind::Ghoul => Color::DARK_GREEN,
        EnemyKind::Ghost | EnemyKind::Wraith | EnemyKind::IceWraith | EnemyKind::Banshee => Color::GREY,
        EnemyKind::Vampire | EnemyKind::BossVampireLord | EnemyKind::VampireElite => Color::DARK_RED,
        EnemyKind::DeathKnight => Color::DARK_MAGENTA,
        EnemyKind::Wolf | EnemyKind::DireWolf | EnemyKind::FrostWolf => Color::GREY,
        EnemyKind::TreeEnt | EnemyKind::BossForestGuardian | EnemyKind::VenomousVine => Color::GREEN,
        EnemyKind::Druid | EnemyKind::ForestSpirit => Color::DARK_GREEN,
        EnemyKind::WildBoar => Color::DARK_YELLOW,
        EnemyKind::GiantWasp => Color::YELLOW,
        EnemyKind::IceElemental | EnemyKind::FrostGiant | EnemyKind::YetiWarrior | EnemyKind::BossIceDragon
        | EnemyKind::FrozenKnight | EnemyKind::Wendigo | EnemyKind::FrostLord => Color::CYAN,
        EnemyKind::FireElemental | EnemyKind::LavaGolem | EnemyKind::Hellhound | EnemyKind::FireDrake
        | EnemyKind::MagmaSlime | EnemyKind::Salamander | EnemyKind::CinderWraith | EnemyKind::InfernalImp
        | EnemyKind::InfernalLord => Color::RED,
        EnemyKind::Golem | EnemyKind::AncientGuardian | EnemyKind::CursedStatue => Color::YELLOW,
        EnemyKind::Sphinx | EnemyKind::Gargoyle => Color::YELLOW,
        EnemyKind::Lich | EnemyKind::MummyLord => Color::MAGENTA,
        EnemyKind::ShadowAssassin => Color::DARK_GREY,
        EnemyKind::Demon | EnemyKind::DemonLord | EnemyKind::Balrog | EnemyKind::BossDemonKing
        | EnemyKind::PitFiend | EnemyKind::DoomGuard => Color::RED,
        EnemyKind::Succubus | EnemyKind::ShadowDemon => Color::MAGENTA,
        EnemyKind::AbyssalHorror => Color::DARK_RED,
        EnemyKind::AncientWyrm => Color::GREEN,
    }
}

/// Message types for the game log
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MessageType {
    Normal,
    Combat,
    Damage,
    Heal,
    LevelUp,
    Death,
    Item,
    System,
    Warning,
    Boss,
}

impl MessageType {
    /// Returns the color for this message type
    pub fn color(&self) -> Color {
        match self {
            Self::Normal => Color::WHITE,
            Self::Combat => Color::YELLOW,
            Self::Damage => Color::RED,
            Self::Heal => Color::GREEN,
            Self::LevelUp => Color::CYAN,
            Self::Death => Color::DARK_RED,
            Self::Item => Color::BLUE,
            Self::System => Color::GREY,
            Self::Warning => Color::ORANGE,
            Self::Boss => Color::MAGENTA,
        }
    }
}

/// A game message with color information
#[derive(Clone, Debug)]
pub struct GameMessage {
    pub text: String,
    pub message_type: MessageType,
}

impl GameMessage {
    pub fn new(text: impl Into<String>, message_type: MessageType) -> Self {
        Self {
            text: text.into(),
            message_type,
        }
    }

    pub fn color(&self) -> Color {
        self.message_type.color()
    }
}

/// Renders a health bar as a string
pub fn health_bar(current: i32, max: i32, width: usize) -> String {
    let filled = if max > 0 {
        (current as f32 / max as f32 * width as f32) as usize
    } else {
        0
    };
    let filled = filled.min(width);
    let empty = width - filled;

    format!(
        "[{}{}] {}/{}",
        "#".repeat(filled),
        "-".repeat(empty),
        current,
        max
    )
}

/// Formats a stat with optional bonus/malus coloring indicator
pub fn format_stat(base: i32, current: i32) -> String {
    if current > base {
        format!("{} (+{})", current, current - base)
    } else if current < base {
        format!("{} ({})", current, current - base)
    } else {
        current.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_blend() {
        let white = Color::WHITE;
        let black = Color::BLACK;
        let grey = white.blend(black, 0.5);
        assert!(grey.r > 120 && grey.r < 136);
    }

    #[test]
    fn test_health_bar() {
        let bar = health_bar(50, 100, 10);
        assert!(bar.contains("#####"));
        assert!(bar.contains("50/100"));
    }

    #[test]
    fn test_format_stat() {
        assert_eq!(format_stat(10, 15), "15 (+5)");
        assert_eq!(format_stat(10, 8), "8 (-2)");
        assert_eq!(format_stat(10, 10), "10");
    }
}
