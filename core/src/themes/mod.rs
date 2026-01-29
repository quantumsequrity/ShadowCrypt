//! Color Themes System
//!
//! Comprehensive visual theming system with 10+ built-in themes,
//! colorblind accessibility modes, and custom theme support.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Core Color Types
// ============================================================================

/// RGBA Color representation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Create a new color with full opacity
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Create a new color with specified alpha
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create color from hex value (e.g., 0xFF5733)
    pub const fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
            a: 255,
        }
    }

    /// Create color from hex string (e.g., "#FF5733" or "FF5733")
    pub fn from_hex_str(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Self::rgb(r, g, b))
    }

    /// Convert to hex string
    pub fn to_hex_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// Convert to tuple (r, g, b)
    pub const fn to_tuple(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    /// Convert to tuple with alpha (r, g, b, a)
    pub const fn to_tuple_rgba(&self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    /// Blend two colors together
    pub fn blend(&self, other: &Color, factor: f32) -> Self {
        let factor = factor.clamp(0.0, 1.0);
        let inv = 1.0 - factor;
        Self {
            r: (self.r as f32 * inv + other.r as f32 * factor) as u8,
            g: (self.g as f32 * inv + other.g as f32 * factor) as u8,
            b: (self.b as f32 * inv + other.b as f32 * factor) as u8,
            a: (self.a as f32 * inv + other.a as f32 * factor) as u8,
        }
    }

    /// Lighten the color
    pub fn lighten(&self, amount: f32) -> Self {
        self.blend(&Color::rgb(255, 255, 255), amount)
    }

    /// Darken the color
    pub fn darken(&self, amount: f32) -> Self {
        self.blend(&Color::rgb(0, 0, 0), amount)
    }

    /// Get luminance (perceived brightness)
    pub fn luminance(&self) -> f32 {
        0.299 * (self.r as f32 / 255.0)
            + 0.587 * (self.g as f32 / 255.0)
            + 0.114 * (self.b as f32 / 255.0)
    }

    /// Check if color is considered "light"
    pub fn is_light(&self) -> bool {
        self.luminance() > 0.5
    }

    /// Get contrasting color (black or white)
    pub fn contrasting(&self) -> Self {
        if self.is_light() {
            Color::rgb(0, 0, 0)
        } else {
            Color::rgb(255, 255, 255)
        }
    }

    /// Apply colorblind simulation
    pub fn apply_colorblind_filter(&self, mode: ColorblindMode) -> Self {
        match mode {
            ColorblindMode::None => *self,
            ColorblindMode::Deuteranopia => self.simulate_deuteranopia(),
            ColorblindMode::Protanopia => self.simulate_protanopia(),
            ColorblindMode::Tritanopia => self.simulate_tritanopia(),
        }
    }

    fn simulate_deuteranopia(&self) -> Self {
        // Green-blind simulation (most common)
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let new_r = 0.625 * r + 0.375 * g;
        let new_g = 0.700 * r + 0.300 * g;
        let new_b = 0.300 * g + 0.700 * b;

        Self::rgb(
            (new_r * 255.0).clamp(0.0, 255.0) as u8,
            (new_g * 255.0).clamp(0.0, 255.0) as u8,
            (new_b * 255.0).clamp(0.0, 255.0) as u8,
        )
    }

    fn simulate_protanopia(&self) -> Self {
        // Red-blind simulation
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let new_r = 0.567 * r + 0.433 * g;
        let new_g = 0.558 * r + 0.442 * g;
        let new_b = 0.242 * g + 0.758 * b;

        Self::rgb(
            (new_r * 255.0).clamp(0.0, 255.0) as u8,
            (new_g * 255.0).clamp(0.0, 255.0) as u8,
            (new_b * 255.0).clamp(0.0, 255.0) as u8,
        )
    }

    fn simulate_tritanopia(&self) -> Self {
        // Blue-blind simulation
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let new_r = 0.950 * r + 0.050 * g;
        let new_g = 0.433 * g + 0.567 * b;
        let new_b = 0.475 * g + 0.525 * b;

        Self::rgb(
            (new_r * 255.0).clamp(0.0, 255.0) as u8,
            (new_g * 255.0).clamp(0.0, 255.0) as u8,
            (new_b * 255.0).clamp(0.0, 255.0) as u8,
        )
    }

    // Common color constants
    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const WHITE: Color = Color::rgb(255, 255, 255);
    pub const RED: Color = Color::rgb(255, 0, 0);
    pub const GREEN: Color = Color::rgb(0, 255, 0);
    pub const BLUE: Color = Color::rgb(0, 0, 255);
    pub const YELLOW: Color = Color::rgb(255, 255, 0);
    pub const CYAN: Color = Color::rgb(0, 255, 255);
    pub const MAGENTA: Color = Color::rgb(255, 0, 255);
    pub const ORANGE: Color = Color::rgb(255, 165, 0);
    pub const PURPLE: Color = Color::rgb(128, 0, 128);
    pub const GRAY: Color = Color::rgb(128, 128, 128);
    pub const DARK_GRAY: Color = Color::rgb(64, 64, 64);
    pub const LIGHT_GRAY: Color = Color::rgb(192, 192, 192);
    pub const TRANSPARENT: Color = Color::rgba(0, 0, 0, 0);
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex_string())
    }
}

// ============================================================================
// Colorblind Modes
// ============================================================================

/// Colorblind accessibility modes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ColorblindMode {
    /// Normal vision
    #[default]
    None,
    /// Green-blind (most common, ~6% of males)
    Deuteranopia,
    /// Red-blind (~1% of males)
    Protanopia,
    /// Blue-blind (rare, <0.01%)
    Tritanopia,
}

impl ColorblindMode {
    pub fn all() -> &'static [ColorblindMode] {
        &[
            Self::None,
            Self::Deuteranopia,
            Self::Protanopia,
            Self::Tritanopia,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::None => "Normal Vision",
            Self::Deuteranopia => "Deuteranopia (Green-Blind)",
            Self::Protanopia => "Protanopia (Red-Blind)",
            Self::Tritanopia => "Tritanopia (Blue-Blind)",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::None => "Standard color vision with no adjustments.",
            Self::Deuteranopia => "Adjusts colors for green color blindness, the most common type.",
            Self::Protanopia => "Adjusts colors for red color blindness.",
            Self::Tritanopia => "Adjusts colors for blue color blindness, a rare condition.",
        }
    }
}

// ============================================================================
// Theme Identifiers
// ============================================================================

/// Built-in theme identifiers
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ThemeId {
    /// Traditional roguelike colors (white on black with primary colors)
    #[default]
    Classic,
    /// Dark with purple and violet accents
    DarkFantasy,
    /// High contrast for accessibility (bright colors, clear distinctions)
    HighContrast,
    /// Classic green terminal aesthetic
    RetroGreen,
    /// Red and black dark theme
    Crimson,
    /// Blue oceanic tones
    Ocean,
    /// Green natural tones
    Forest,
    /// Purple and black void theme
    Void,
    /// Warm gold and brown tones
    Golden,
    /// Blue and white icy theme
    Ice,
    /// Custom user-defined theme
    Custom,
}

impl ThemeId {
    pub fn all_builtin() -> &'static [ThemeId] {
        &[
            Self::Classic,
            Self::DarkFantasy,
            Self::HighContrast,
            Self::RetroGreen,
            Self::Crimson,
            Self::Ocean,
            Self::Forest,
            Self::Void,
            Self::Golden,
            Self::Ice,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Classic => "Classic",
            Self::DarkFantasy => "Dark Fantasy",
            Self::HighContrast => "High Contrast",
            Self::RetroGreen => "Retro Green",
            Self::Crimson => "Crimson",
            Self::Ocean => "Ocean",
            Self::Forest => "Forest",
            Self::Void => "Void",
            Self::Golden => "Golden",
            Self::Ice => "Ice",
            Self::Custom => "Custom",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Classic => "Traditional roguelike colors with clean, readable text.",
            Self::DarkFantasy => "Dark backgrounds with mystical purple and violet accents.",
            Self::HighContrast => "Maximum contrast for accessibility and visibility.",
            Self::RetroGreen => "Classic CRT terminal aesthetic with green phosphor glow.",
            Self::Crimson => "Dark and brooding with blood red accents.",
            Self::Ocean => "Calming blue tones inspired by deep ocean waters.",
            Self::Forest => "Natural greens and earth tones of an ancient forest.",
            Self::Void => "Cosmic purple and black, echoes of the void between stars.",
            Self::Golden => "Warm, inviting gold and amber tones of treasure.",
            Self::Ice => "Cold blue and white of frozen wastelands.",
            Self::Custom => "User-defined custom color scheme.",
        }
    }
}

// ============================================================================
// Color Theme Definition
// ============================================================================

/// Complete color theme definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColorTheme {
    /// Theme identifier
    pub id: ThemeId,
    /// Theme display name
    pub name: String,
    /// Theme description
    pub description: String,

    // === UI Colors ===
    /// Main background color
    pub background: Color,
    /// Primary text/foreground color
    pub foreground: Color,
    /// Secondary/dimmed text color
    pub foreground_dim: Color,
    /// Border/frame color
    pub border: Color,
    /// Highlight/selection color
    pub highlight: Color,
    /// Accent color for important UI elements
    pub accent: Color,
    /// Warning/alert color
    pub warning: Color,
    /// Error/danger color
    pub error: Color,
    /// Success/confirmation color
    pub success: Color,

    // === Game Entity Colors ===
    /// Player character color
    pub player: Color,
    /// Weak enemy color (low level)
    pub enemy_weak: Color,
    /// Normal enemy color (medium level)
    pub enemy_normal: Color,
    /// Strong enemy color (high level)
    pub enemy_strong: Color,
    /// Boss enemy color
    pub enemy_boss: Color,
    /// NPC color
    pub npc: Color,
    /// Companion/ally color
    pub companion: Color,
    /// Neutral creature color
    pub neutral: Color,

    // === Terrain Colors ===
    /// Wall/obstacle color
    pub wall: Color,
    /// Floor/walkable tile color
    pub floor: Color,
    /// Door color
    pub door: Color,
    /// Stairs going up color
    pub stairs_up: Color,
    /// Stairs going down color
    pub stairs_down: Color,
    /// Water/liquid color
    pub water: Color,
    /// Lava/dangerous liquid color
    pub lava: Color,
    /// Grass/vegetation color
    pub grass: Color,
    /// Trap color
    pub trap: Color,
    /// Secret/hidden area color
    pub secret: Color,
    /// Fog of war (unexplored) color
    pub fog_unexplored: Color,
    /// Fog of war (explored but not visible) color
    pub fog_explored: Color,

    // === Item Rarity Colors ===
    /// Common item color
    pub item_common: Color,
    /// Uncommon item color
    pub item_uncommon: Color,
    /// Rare item color
    pub item_rare: Color,
    /// Epic item color
    pub item_epic: Color,
    /// Legendary item color
    pub item_legendary: Color,
    /// Artifact/mythic item color
    pub item_artifact: Color,
    /// Cursed item color
    pub item_cursed: Color,

    // === Status Bar Colors ===
    /// HP bar - high health
    pub hp_high: Color,
    /// HP bar - medium health
    pub hp_medium: Color,
    /// HP bar - low health (critical)
    pub hp_low: Color,
    /// Mana/MP bar color
    pub mp_color: Color,
    /// Experience/XP bar color
    pub xp_color: Color,
    /// Stamina bar color
    pub stamina_color: Color,
    /// Shield/armor bar color
    pub shield_color: Color,

    // === Effect Colors ===
    /// Fire effect color
    pub effect_fire: Color,
    /// Ice/cold effect color
    pub effect_ice: Color,
    /// Lightning/electric effect color
    pub effect_lightning: Color,
    /// Poison effect color
    pub effect_poison: Color,
    /// Healing effect color
    pub effect_heal: Color,
    /// Magic/arcane effect color
    pub effect_magic: Color,
    /// Shadow/dark effect color
    pub effect_shadow: Color,
    /// Holy/light effect color
    pub effect_holy: Color,

    // === Message Log Colors ===
    /// Normal message color
    pub msg_normal: Color,
    /// Combat message color
    pub msg_combat: Color,
    /// Item-related message color
    pub msg_item: Color,
    /// System message color
    pub msg_system: Color,
    /// Story/lore message color
    pub msg_story: Color,
}

impl Default for ColorTheme {
    fn default() -> Self {
        Self::classic()
    }
}

impl ColorTheme {
    // ========================================================================
    // Built-in Theme Constructors
    // ========================================================================

    /// Classic roguelike theme - traditional colors
    pub fn classic() -> Self {
        Self {
            id: ThemeId::Classic,
            name: "Classic".to_string(),
            description: "Traditional roguelike colors with clean, readable text.".to_string(),

            // UI Colors
            background: Color::rgb(0, 0, 0),
            foreground: Color::rgb(255, 255, 255),
            foreground_dim: Color::rgb(170, 170, 170),
            border: Color::rgb(128, 128, 128),
            highlight: Color::rgb(255, 255, 0),
            accent: Color::rgb(0, 170, 255),
            warning: Color::rgb(255, 170, 0),
            error: Color::rgb(255, 85, 85),
            success: Color::rgb(85, 255, 85),

            // Entity Colors
            player: Color::rgb(255, 255, 255),
            enemy_weak: Color::rgb(85, 255, 85),
            enemy_normal: Color::rgb(255, 255, 85),
            enemy_strong: Color::rgb(255, 85, 85),
            enemy_boss: Color::rgb(255, 0, 255),
            npc: Color::rgb(85, 255, 255),
            companion: Color::rgb(85, 170, 255),
            neutral: Color::rgb(170, 170, 170),

            // Terrain Colors
            wall: Color::rgb(128, 128, 128),
            floor: Color::rgb(64, 64, 64),
            door: Color::rgb(139, 90, 43),
            stairs_up: Color::rgb(255, 255, 255),
            stairs_down: Color::rgb(200, 200, 200),
            water: Color::rgb(0, 100, 200),
            lava: Color::rgb(255, 100, 0),
            grass: Color::rgb(34, 139, 34),
            trap: Color::rgb(255, 0, 128),
            secret: Color::rgb(128, 0, 128),
            fog_unexplored: Color::rgb(0, 0, 0),
            fog_explored: Color::rgb(32, 32, 48),

            // Item Rarity Colors
            item_common: Color::rgb(200, 200, 200),
            item_uncommon: Color::rgb(85, 255, 85),
            item_rare: Color::rgb(85, 85, 255),
            item_epic: Color::rgb(170, 85, 255),
            item_legendary: Color::rgb(255, 170, 0),
            item_artifact: Color::rgb(255, 215, 0),
            item_cursed: Color::rgb(139, 0, 0),

            // Status Colors
            hp_high: Color::rgb(0, 200, 0),
            hp_medium: Color::rgb(255, 200, 0),
            hp_low: Color::rgb(200, 0, 0),
            mp_color: Color::rgb(0, 100, 255),
            xp_color: Color::rgb(200, 0, 200),
            stamina_color: Color::rgb(255, 200, 100),
            shield_color: Color::rgb(100, 150, 255),

            // Effect Colors
            effect_fire: Color::rgb(255, 100, 0),
            effect_ice: Color::rgb(100, 200, 255),
            effect_lightning: Color::rgb(255, 255, 100),
            effect_poison: Color::rgb(100, 255, 100),
            effect_heal: Color::rgb(100, 255, 150),
            effect_magic: Color::rgb(200, 100, 255),
            effect_shadow: Color::rgb(80, 0, 120),
            effect_holy: Color::rgb(255, 255, 200),

            // Message Colors
            msg_normal: Color::rgb(200, 200, 200),
            msg_combat: Color::rgb(255, 100, 100),
            msg_item: Color::rgb(100, 200, 255),
            msg_system: Color::rgb(255, 255, 100),
            msg_story: Color::rgb(200, 150, 255),
        }
    }

    /// Dark Fantasy theme - mystical purple and dark tones
    pub fn dark_fantasy() -> Self {
        Self {
            id: ThemeId::DarkFantasy,
            name: "Dark Fantasy".to_string(),
            description: "Dark backgrounds with mystical purple and violet accents.".to_string(),

            // UI Colors
            background: Color::rgb(15, 10, 25),
            foreground: Color::rgb(220, 210, 240),
            foreground_dim: Color::rgb(140, 130, 160),
            border: Color::rgb(80, 60, 100),
            highlight: Color::rgb(200, 150, 255),
            accent: Color::rgb(150, 80, 200),
            warning: Color::rgb(255, 180, 80),
            error: Color::rgb(220, 60, 80),
            success: Color::rgb(80, 200, 120),

            // Entity Colors
            player: Color::rgb(200, 180, 255),
            enemy_weak: Color::rgb(100, 180, 100),
            enemy_normal: Color::rgb(200, 180, 100),
            enemy_strong: Color::rgb(200, 80, 80),
            enemy_boss: Color::rgb(255, 50, 200),
            npc: Color::rgb(100, 200, 220),
            companion: Color::rgb(120, 160, 255),
            neutral: Color::rgb(130, 120, 140),

            // Terrain Colors
            wall: Color::rgb(60, 50, 80),
            floor: Color::rgb(30, 25, 45),
            door: Color::rgb(100, 60, 40),
            stairs_up: Color::rgb(200, 180, 255),
            stairs_down: Color::rgb(120, 100, 160),
            water: Color::rgb(40, 60, 120),
            lava: Color::rgb(200, 60, 20),
            grass: Color::rgb(40, 80, 50),
            trap: Color::rgb(180, 40, 100),
            secret: Color::rgb(100, 40, 140),
            fog_unexplored: Color::rgb(10, 5, 15),
            fog_explored: Color::rgb(25, 20, 40),

            // Item Rarity Colors
            item_common: Color::rgb(160, 150, 180),
            item_uncommon: Color::rgb(80, 200, 120),
            item_rare: Color::rgb(80, 120, 220),
            item_epic: Color::rgb(180, 80, 255),
            item_legendary: Color::rgb(255, 180, 50),
            item_artifact: Color::rgb(255, 220, 100),
            item_cursed: Color::rgb(120, 20, 40),

            // Status Colors
            hp_high: Color::rgb(60, 180, 80),
            hp_medium: Color::rgb(220, 180, 60),
            hp_low: Color::rgb(180, 40, 60),
            mp_color: Color::rgb(100, 80, 200),
            xp_color: Color::rgb(180, 80, 180),
            stamina_color: Color::rgb(220, 180, 100),
            shield_color: Color::rgb(100, 140, 200),

            // Effect Colors
            effect_fire: Color::rgb(220, 80, 20),
            effect_ice: Color::rgb(100, 180, 220),
            effect_lightning: Color::rgb(220, 220, 100),
            effect_poison: Color::rgb(100, 200, 80),
            effect_heal: Color::rgb(100, 220, 150),
            effect_magic: Color::rgb(180, 100, 255),
            effect_shadow: Color::rgb(60, 20, 100),
            effect_holy: Color::rgb(255, 240, 200),

            // Message Colors
            msg_normal: Color::rgb(180, 170, 200),
            msg_combat: Color::rgb(220, 100, 100),
            msg_item: Color::rgb(100, 180, 220),
            msg_system: Color::rgb(220, 200, 100),
            msg_story: Color::rgb(180, 140, 220),
        }
    }

    /// High Contrast theme - maximum accessibility
    pub fn high_contrast() -> Self {
        Self {
            id: ThemeId::HighContrast,
            name: "High Contrast".to_string(),
            description: "Maximum contrast for accessibility and visibility.".to_string(),

            // UI Colors
            background: Color::rgb(0, 0, 0),
            foreground: Color::rgb(255, 255, 255),
            foreground_dim: Color::rgb(200, 200, 200),
            border: Color::rgb(255, 255, 255),
            highlight: Color::rgb(255, 255, 0),
            accent: Color::rgb(0, 255, 255),
            warning: Color::rgb(255, 200, 0),
            error: Color::rgb(255, 0, 0),
            success: Color::rgb(0, 255, 0),

            // Entity Colors - distinct shapes and patterns should supplement these
            player: Color::rgb(255, 255, 255),
            enemy_weak: Color::rgb(0, 255, 0),
            enemy_normal: Color::rgb(255, 255, 0),
            enemy_strong: Color::rgb(255, 128, 0),
            enemy_boss: Color::rgb(255, 0, 0),
            npc: Color::rgb(0, 255, 255),
            companion: Color::rgb(128, 128, 255),
            neutral: Color::rgb(200, 200, 200),

            // Terrain Colors
            wall: Color::rgb(180, 180, 180),
            floor: Color::rgb(60, 60, 60),
            door: Color::rgb(255, 200, 100),
            stairs_up: Color::rgb(255, 255, 255),
            stairs_down: Color::rgb(200, 200, 200),
            water: Color::rgb(0, 128, 255),
            lava: Color::rgb(255, 64, 0),
            grass: Color::rgb(0, 200, 0),
            trap: Color::rgb(255, 0, 128),
            secret: Color::rgb(255, 0, 255),
            fog_unexplored: Color::rgb(0, 0, 0),
            fog_explored: Color::rgb(40, 40, 40),

            // Item Rarity Colors - very distinct
            item_common: Color::rgb(200, 200, 200),
            item_uncommon: Color::rgb(0, 255, 0),
            item_rare: Color::rgb(0, 128, 255),
            item_epic: Color::rgb(255, 0, 255),
            item_legendary: Color::rgb(255, 200, 0),
            item_artifact: Color::rgb(255, 255, 0),
            item_cursed: Color::rgb(255, 0, 0),

            // Status Colors
            hp_high: Color::rgb(0, 255, 0),
            hp_medium: Color::rgb(255, 255, 0),
            hp_low: Color::rgb(255, 0, 0),
            mp_color: Color::rgb(0, 128, 255),
            xp_color: Color::rgb(255, 0, 255),
            stamina_color: Color::rgb(255, 200, 0),
            shield_color: Color::rgb(0, 255, 255),

            // Effect Colors
            effect_fire: Color::rgb(255, 128, 0),
            effect_ice: Color::rgb(128, 200, 255),
            effect_lightning: Color::rgb(255, 255, 0),
            effect_poison: Color::rgb(0, 255, 0),
            effect_heal: Color::rgb(0, 255, 128),
            effect_magic: Color::rgb(255, 0, 255),
            effect_shadow: Color::rgb(128, 0, 128),
            effect_holy: Color::rgb(255, 255, 200),

            // Message Colors
            msg_normal: Color::rgb(255, 255, 255),
            msg_combat: Color::rgb(255, 128, 128),
            msg_item: Color::rgb(128, 200, 255),
            msg_system: Color::rgb(255, 255, 0),
            msg_story: Color::rgb(255, 128, 255),
        }
    }

    /// Retro Green theme - classic CRT terminal
    pub fn retro_green() -> Self {
        Self {
            id: ThemeId::RetroGreen,
            name: "Retro Green".to_string(),
            description: "Classic CRT terminal aesthetic with green phosphor glow.".to_string(),

            // UI Colors
            background: Color::rgb(0, 10, 0),
            foreground: Color::rgb(0, 255, 0),
            foreground_dim: Color::rgb(0, 150, 0),
            border: Color::rgb(0, 180, 0),
            highlight: Color::rgb(100, 255, 100),
            accent: Color::rgb(150, 255, 150),
            warning: Color::rgb(200, 255, 0),
            error: Color::rgb(255, 100, 100),
            success: Color::rgb(0, 255, 100),

            // Entity Colors - various shades of green
            player: Color::rgb(200, 255, 200),
            enemy_weak: Color::rgb(0, 150, 0),
            enemy_normal: Color::rgb(0, 200, 0),
            enemy_strong: Color::rgb(100, 255, 0),
            enemy_boss: Color::rgb(255, 255, 0),
            npc: Color::rgb(0, 200, 150),
            companion: Color::rgb(100, 200, 200),
            neutral: Color::rgb(0, 120, 0),

            // Terrain Colors
            wall: Color::rgb(0, 100, 0),
            floor: Color::rgb(0, 40, 0),
            door: Color::rgb(100, 150, 0),
            stairs_up: Color::rgb(150, 255, 150),
            stairs_down: Color::rgb(80, 180, 80),
            water: Color::rgb(0, 100, 100),
            lava: Color::rgb(200, 150, 0),
            grass: Color::rgb(0, 150, 50),
            trap: Color::rgb(200, 200, 0),
            secret: Color::rgb(0, 150, 150),
            fog_unexplored: Color::rgb(0, 5, 0),
            fog_explored: Color::rgb(0, 25, 0),

            // Item Rarity Colors
            item_common: Color::rgb(0, 150, 0),
            item_uncommon: Color::rgb(50, 200, 50),
            item_rare: Color::rgb(100, 255, 100),
            item_epic: Color::rgb(150, 255, 150),
            item_legendary: Color::rgb(200, 255, 100),
            item_artifact: Color::rgb(255, 255, 150),
            item_cursed: Color::rgb(150, 100, 0),

            // Status Colors
            hp_high: Color::rgb(0, 200, 0),
            hp_medium: Color::rgb(150, 200, 0),
            hp_low: Color::rgb(200, 100, 0),
            mp_color: Color::rgb(0, 150, 150),
            xp_color: Color::rgb(150, 255, 0),
            stamina_color: Color::rgb(200, 200, 0),
            shield_color: Color::rgb(100, 200, 150),

            // Effect Colors
            effect_fire: Color::rgb(255, 200, 0),
            effect_ice: Color::rgb(100, 200, 200),
            effect_lightning: Color::rgb(200, 255, 100),
            effect_poison: Color::rgb(0, 255, 100),
            effect_heal: Color::rgb(100, 255, 150),
            effect_magic: Color::rgb(150, 255, 200),
            effect_shadow: Color::rgb(0, 80, 0),
            effect_holy: Color::rgb(200, 255, 200),

            // Message Colors
            msg_normal: Color::rgb(0, 200, 0),
            msg_combat: Color::rgb(200, 255, 0),
            msg_item: Color::rgb(100, 255, 150),
            msg_system: Color::rgb(150, 200, 0),
            msg_story: Color::rgb(100, 200, 150),
        }
    }

    /// Crimson theme - dark and brooding with red accents
    pub fn crimson() -> Self {
        Self {
            id: ThemeId::Crimson,
            name: "Crimson".to_string(),
            description: "Dark and brooding with blood red accents.".to_string(),

            // UI Colors
            background: Color::rgb(15, 5, 5),
            foreground: Color::rgb(240, 220, 220),
            foreground_dim: Color::rgb(160, 140, 140),
            border: Color::rgb(100, 40, 40),
            highlight: Color::rgb(255, 100, 100),
            accent: Color::rgb(200, 50, 50),
            warning: Color::rgb(255, 180, 80),
            error: Color::rgb(255, 50, 50),
            success: Color::rgb(100, 200, 100),

            // Entity Colors
            player: Color::rgb(255, 220, 220),
            enemy_weak: Color::rgb(150, 100, 100),
            enemy_normal: Color::rgb(200, 100, 100),
            enemy_strong: Color::rgb(255, 80, 80),
            enemy_boss: Color::rgb(255, 0, 50),
            npc: Color::rgb(150, 200, 200),
            companion: Color::rgb(150, 150, 220),
            neutral: Color::rgb(140, 130, 130),

            // Terrain Colors
            wall: Color::rgb(80, 50, 50),
            floor: Color::rgb(40, 25, 25),
            door: Color::rgb(120, 60, 40),
            stairs_up: Color::rgb(220, 180, 180),
            stairs_down: Color::rgb(140, 100, 100),
            water: Color::rgb(60, 40, 100),
            lava: Color::rgb(255, 80, 0),
            grass: Color::rgb(60, 80, 50),
            trap: Color::rgb(255, 50, 100),
            secret: Color::rgb(150, 50, 100),
            fog_unexplored: Color::rgb(10, 5, 5),
            fog_explored: Color::rgb(30, 20, 20),

            // Item Rarity Colors
            item_common: Color::rgb(180, 160, 160),
            item_uncommon: Color::rgb(100, 180, 100),
            item_rare: Color::rgb(100, 100, 200),
            item_epic: Color::rgb(200, 80, 180),
            item_legendary: Color::rgb(255, 180, 50),
            item_artifact: Color::rgb(255, 200, 100),
            item_cursed: Color::rgb(150, 0, 30),

            // Status Colors
            hp_high: Color::rgb(180, 50, 50),
            hp_medium: Color::rgb(200, 150, 50),
            hp_low: Color::rgb(200, 0, 50),
            mp_color: Color::rgb(80, 60, 180),
            xp_color: Color::rgb(180, 60, 120),
            stamina_color: Color::rgb(200, 150, 80),
            shield_color: Color::rgb(120, 100, 180),

            // Effect Colors
            effect_fire: Color::rgb(255, 100, 20),
            effect_ice: Color::rgb(100, 150, 220),
            effect_lightning: Color::rgb(255, 220, 100),
            effect_poison: Color::rgb(100, 180, 80),
            effect_heal: Color::rgb(180, 100, 120),
            effect_magic: Color::rgb(200, 80, 180),
            effect_shadow: Color::rgb(80, 20, 40),
            effect_holy: Color::rgb(255, 230, 200),

            // Message Colors
            msg_normal: Color::rgb(200, 180, 180),
            msg_combat: Color::rgb(255, 100, 100),
            msg_item: Color::rgb(150, 180, 220),
            msg_system: Color::rgb(220, 200, 100),
            msg_story: Color::rgb(200, 150, 180),
        }
    }

    /// Ocean theme - calming blue tones
    pub fn ocean() -> Self {
        Self {
            id: ThemeId::Ocean,
            name: "Ocean".to_string(),
            description: "Calming blue tones inspired by deep ocean waters.".to_string(),

            // UI Colors
            background: Color::rgb(5, 15, 30),
            foreground: Color::rgb(220, 240, 255),
            foreground_dim: Color::rgb(140, 170, 200),
            border: Color::rgb(40, 80, 120),
            highlight: Color::rgb(100, 200, 255),
            accent: Color::rgb(50, 150, 200),
            warning: Color::rgb(255, 200, 100),
            error: Color::rgb(255, 100, 120),
            success: Color::rgb(100, 220, 150),

            // Entity Colors
            player: Color::rgb(200, 230, 255),
            enemy_weak: Color::rgb(100, 180, 150),
            enemy_normal: Color::rgb(150, 200, 180),
            enemy_strong: Color::rgb(200, 150, 150),
            enemy_boss: Color::rgb(255, 100, 150),
            npc: Color::rgb(100, 200, 220),
            companion: Color::rgb(150, 180, 255),
            neutral: Color::rgb(130, 150, 170),

            // Terrain Colors
            wall: Color::rgb(40, 60, 90),
            floor: Color::rgb(20, 35, 55),
            door: Color::rgb(100, 80, 60),
            stairs_up: Color::rgb(180, 210, 240),
            stairs_down: Color::rgb(100, 130, 160),
            water: Color::rgb(30, 80, 140),
            lava: Color::rgb(255, 120, 50),
            grass: Color::rgb(50, 120, 80),
            trap: Color::rgb(200, 80, 150),
            secret: Color::rgb(80, 100, 180),
            fog_unexplored: Color::rgb(5, 10, 20),
            fog_explored: Color::rgb(15, 30, 50),

            // Item Rarity Colors
            item_common: Color::rgb(160, 180, 200),
            item_uncommon: Color::rgb(80, 200, 150),
            item_rare: Color::rgb(80, 150, 255),
            item_epic: Color::rgb(180, 100, 220),
            item_legendary: Color::rgb(255, 200, 80),
            item_artifact: Color::rgb(255, 220, 120),
            item_cursed: Color::rgb(120, 40, 80),

            // Status Colors
            hp_high: Color::rgb(80, 180, 120),
            hp_medium: Color::rgb(200, 200, 80),
            hp_low: Color::rgb(200, 80, 100),
            mp_color: Color::rgb(80, 120, 220),
            xp_color: Color::rgb(150, 100, 200),
            stamina_color: Color::rgb(200, 180, 100),
            shield_color: Color::rgb(100, 180, 220),

            // Effect Colors
            effect_fire: Color::rgb(255, 150, 50),
            effect_ice: Color::rgb(150, 220, 255),
            effect_lightning: Color::rgb(255, 255, 150),
            effect_poison: Color::rgb(100, 200, 100),
            effect_heal: Color::rgb(100, 220, 180),
            effect_magic: Color::rgb(150, 120, 255),
            effect_shadow: Color::rgb(30, 50, 100),
            effect_holy: Color::rgb(255, 250, 220),

            // Message Colors
            msg_normal: Color::rgb(180, 200, 220),
            msg_combat: Color::rgb(255, 150, 150),
            msg_item: Color::rgb(100, 200, 255),
            msg_system: Color::rgb(255, 230, 150),
            msg_story: Color::rgb(180, 200, 255),
        }
    }

    /// Forest theme - natural greens and earth tones
    pub fn forest() -> Self {
        Self {
            id: ThemeId::Forest,
            name: "Forest".to_string(),
            description: "Natural greens and earth tones of an ancient forest.".to_string(),

            // UI Colors
            background: Color::rgb(10, 20, 10),
            foreground: Color::rgb(230, 240, 220),
            foreground_dim: Color::rgb(150, 170, 140),
            border: Color::rgb(60, 90, 50),
            highlight: Color::rgb(150, 255, 100),
            accent: Color::rgb(100, 180, 80),
            warning: Color::rgb(255, 200, 80),
            error: Color::rgb(220, 80, 80),
            success: Color::rgb(100, 220, 100),

            // Entity Colors
            player: Color::rgb(220, 240, 200),
            enemy_weak: Color::rgb(140, 180, 100),
            enemy_normal: Color::rgb(180, 200, 100),
            enemy_strong: Color::rgb(220, 150, 80),
            enemy_boss: Color::rgb(255, 100, 100),
            npc: Color::rgb(100, 180, 200),
            companion: Color::rgb(120, 180, 150),
            neutral: Color::rgb(140, 150, 130),

            // Terrain Colors
            wall: Color::rgb(60, 80, 50),
            floor: Color::rgb(35, 50, 30),
            door: Color::rgb(120, 80, 50),
            stairs_up: Color::rgb(200, 220, 180),
            stairs_down: Color::rgb(120, 140, 100),
            water: Color::rgb(40, 100, 120),
            lava: Color::rgb(255, 120, 40),
            grass: Color::rgb(60, 140, 60),
            trap: Color::rgb(200, 100, 80),
            secret: Color::rgb(100, 150, 100),
            fog_unexplored: Color::rgb(5, 15, 5),
            fog_explored: Color::rgb(20, 35, 20),

            // Item Rarity Colors
            item_common: Color::rgb(170, 180, 160),
            item_uncommon: Color::rgb(100, 200, 100),
            item_rare: Color::rgb(80, 150, 220),
            item_epic: Color::rgb(180, 100, 200),
            item_legendary: Color::rgb(255, 200, 80),
            item_artifact: Color::rgb(255, 220, 100),
            item_cursed: Color::rgb(120, 60, 60),

            // Status Colors
            hp_high: Color::rgb(80, 180, 80),
            hp_medium: Color::rgb(200, 180, 60),
            hp_low: Color::rgb(180, 60, 60),
            mp_color: Color::rgb(80, 140, 180),
            xp_color: Color::rgb(180, 150, 80),
            stamina_color: Color::rgb(200, 180, 100),
            shield_color: Color::rgb(120, 160, 140),

            // Effect Colors
            effect_fire: Color::rgb(255, 140, 40),
            effect_ice: Color::rgb(150, 200, 220),
            effect_lightning: Color::rgb(255, 255, 120),
            effect_poison: Color::rgb(120, 200, 80),
            effect_heal: Color::rgb(120, 220, 140),
            effect_magic: Color::rgb(160, 140, 220),
            effect_shadow: Color::rgb(40, 60, 40),
            effect_holy: Color::rgb(255, 250, 200),

            // Message Colors
            msg_normal: Color::rgb(190, 200, 180),
            msg_combat: Color::rgb(220, 140, 100),
            msg_item: Color::rgb(140, 200, 180),
            msg_system: Color::rgb(220, 220, 140),
            msg_story: Color::rgb(180, 200, 160),
        }
    }

    /// Void theme - cosmic purple and black
    pub fn void() -> Self {
        Self {
            id: ThemeId::Void,
            name: "Void".to_string(),
            description: "Cosmic purple and black, echoes of the void between stars.".to_string(),

            // UI Colors
            background: Color::rgb(5, 0, 15),
            foreground: Color::rgb(230, 220, 255),
            foreground_dim: Color::rgb(150, 140, 180),
            border: Color::rgb(80, 50, 120),
            highlight: Color::rgb(200, 150, 255),
            accent: Color::rgb(150, 80, 200),
            warning: Color::rgb(255, 180, 100),
            error: Color::rgb(255, 80, 120),
            success: Color::rgb(120, 200, 180),

            // Entity Colors
            player: Color::rgb(220, 200, 255),
            enemy_weak: Color::rgb(120, 100, 180),
            enemy_normal: Color::rgb(160, 100, 200),
            enemy_strong: Color::rgb(200, 80, 180),
            enemy_boss: Color::rgb(255, 50, 200),
            npc: Color::rgb(100, 180, 220),
            companion: Color::rgb(150, 150, 220),
            neutral: Color::rgb(130, 120, 150),

            // Terrain Colors
            wall: Color::rgb(50, 30, 80),
            floor: Color::rgb(25, 15, 45),
            door: Color::rgb(100, 60, 80),
            stairs_up: Color::rgb(200, 180, 255),
            stairs_down: Color::rgb(120, 100, 160),
            water: Color::rgb(40, 40, 120),
            lava: Color::rgb(200, 50, 100),
            grass: Color::rgb(50, 80, 80),
            trap: Color::rgb(255, 80, 150),
            secret: Color::rgb(120, 60, 180),
            fog_unexplored: Color::rgb(3, 0, 10),
            fog_explored: Color::rgb(15, 10, 35),

            // Item Rarity Colors
            item_common: Color::rgb(160, 150, 180),
            item_uncommon: Color::rgb(100, 180, 160),
            item_rare: Color::rgb(100, 120, 220),
            item_epic: Color::rgb(200, 80, 220),
            item_legendary: Color::rgb(255, 180, 100),
            item_artifact: Color::rgb(255, 200, 150),
            item_cursed: Color::rgb(120, 30, 80),

            // Status Colors
            hp_high: Color::rgb(100, 180, 150),
            hp_medium: Color::rgb(200, 180, 100),
            hp_low: Color::rgb(200, 60, 100),
            mp_color: Color::rgb(120, 80, 200),
            xp_color: Color::rgb(200, 100, 180),
            stamina_color: Color::rgb(180, 160, 120),
            shield_color: Color::rgb(120, 120, 200),

            // Effect Colors
            effect_fire: Color::rgb(255, 100, 80),
            effect_ice: Color::rgb(120, 180, 220),
            effect_lightning: Color::rgb(220, 200, 255),
            effect_poison: Color::rgb(80, 200, 120),
            effect_heal: Color::rgb(150, 200, 200),
            effect_magic: Color::rgb(200, 100, 255),
            effect_shadow: Color::rgb(60, 30, 100),
            effect_holy: Color::rgb(255, 240, 220),

            // Message Colors
            msg_normal: Color::rgb(180, 170, 200),
            msg_combat: Color::rgb(220, 120, 150),
            msg_item: Color::rgb(150, 180, 220),
            msg_system: Color::rgb(200, 180, 255),
            msg_story: Color::rgb(180, 150, 220),
        }
    }

    /// Golden theme - warm gold and amber tones
    pub fn golden() -> Self {
        Self {
            id: ThemeId::Golden,
            name: "Golden".to_string(),
            description: "Warm, inviting gold and amber tones of treasure.".to_string(),

            // UI Colors
            background: Color::rgb(20, 15, 5),
            foreground: Color::rgb(255, 240, 200),
            foreground_dim: Color::rgb(180, 160, 120),
            border: Color::rgb(150, 120, 60),
            highlight: Color::rgb(255, 220, 100),
            accent: Color::rgb(220, 180, 80),
            warning: Color::rgb(255, 180, 80),
            error: Color::rgb(220, 80, 80),
            success: Color::rgb(150, 200, 100),

            // Entity Colors
            player: Color::rgb(255, 240, 200),
            enemy_weak: Color::rgb(180, 160, 100),
            enemy_normal: Color::rgb(200, 180, 100),
            enemy_strong: Color::rgb(220, 150, 80),
            enemy_boss: Color::rgb(255, 100, 80),
            npc: Color::rgb(150, 200, 200),
            companion: Color::rgb(180, 180, 150),
            neutral: Color::rgb(160, 150, 120),

            // Terrain Colors
            wall: Color::rgb(100, 80, 50),
            floor: Color::rgb(50, 40, 25),
            door: Color::rgb(150, 100, 50),
            stairs_up: Color::rgb(255, 230, 180),
            stairs_down: Color::rgb(180, 150, 100),
            water: Color::rgb(60, 80, 120),
            lava: Color::rgb(255, 150, 50),
            grass: Color::rgb(100, 120, 50),
            trap: Color::rgb(200, 80, 80),
            secret: Color::rgb(180, 150, 80),
            fog_unexplored: Color::rgb(10, 8, 3),
            fog_explored: Color::rgb(35, 28, 15),

            // Item Rarity Colors
            item_common: Color::rgb(180, 170, 140),
            item_uncommon: Color::rgb(150, 200, 100),
            item_rare: Color::rgb(100, 150, 220),
            item_epic: Color::rgb(200, 120, 180),
            item_legendary: Color::rgb(255, 215, 0),
            item_artifact: Color::rgb(255, 230, 100),
            item_cursed: Color::rgb(120, 60, 40),

            // Status Colors
            hp_high: Color::rgb(150, 180, 80),
            hp_medium: Color::rgb(220, 180, 60),
            hp_low: Color::rgb(200, 80, 60),
            mp_color: Color::rgb(100, 140, 200),
            xp_color: Color::rgb(220, 180, 80),
            stamina_color: Color::rgb(220, 200, 100),
            shield_color: Color::rgb(180, 160, 120),

            // Effect Colors
            effect_fire: Color::rgb(255, 180, 50),
            effect_ice: Color::rgb(150, 200, 220),
            effect_lightning: Color::rgb(255, 255, 150),
            effect_poison: Color::rgb(120, 180, 80),
            effect_heal: Color::rgb(180, 220, 150),
            effect_magic: Color::rgb(200, 150, 220),
            effect_shadow: Color::rgb(80, 60, 40),
            effect_holy: Color::rgb(255, 250, 200),

            // Message Colors
            msg_normal: Color::rgb(220, 200, 160),
            msg_combat: Color::rgb(255, 150, 100),
            msg_item: Color::rgb(255, 220, 120),
            msg_system: Color::rgb(220, 200, 100),
            msg_story: Color::rgb(220, 180, 140),
        }
    }

    /// Ice theme - cold blue and white
    pub fn ice() -> Self {
        Self {
            id: ThemeId::Ice,
            name: "Ice".to_string(),
            description: "Cold blue and white of frozen wastelands.".to_string(),

            // UI Colors
            background: Color::rgb(10, 20, 30),
            foreground: Color::rgb(230, 245, 255),
            foreground_dim: Color::rgb(160, 190, 210),
            border: Color::rgb(80, 120, 160),
            highlight: Color::rgb(180, 230, 255),
            accent: Color::rgb(100, 180, 220),
            warning: Color::rgb(255, 200, 100),
            error: Color::rgb(255, 120, 140),
            success: Color::rgb(100, 220, 180),

            // Entity Colors
            player: Color::rgb(240, 250, 255),
            enemy_weak: Color::rgb(140, 180, 200),
            enemy_normal: Color::rgb(160, 200, 220),
            enemy_strong: Color::rgb(180, 150, 200),
            enemy_boss: Color::rgb(200, 100, 180),
            npc: Color::rgb(150, 220, 220),
            companion: Color::rgb(180, 200, 240),
            neutral: Color::rgb(160, 180, 200),

            // Terrain Colors
            wall: Color::rgb(80, 100, 130),
            floor: Color::rgb(40, 55, 75),
            door: Color::rgb(120, 100, 80),
            stairs_up: Color::rgb(220, 240, 255),
            stairs_down: Color::rgb(140, 170, 200),
            water: Color::rgb(40, 80, 140),
            lava: Color::rgb(255, 140, 80),
            grass: Color::rgb(80, 140, 120),
            trap: Color::rgb(200, 100, 150),
            secret: Color::rgb(140, 160, 200),
            fog_unexplored: Color::rgb(5, 12, 20),
            fog_explored: Color::rgb(25, 40, 55),

            // Item Rarity Colors
            item_common: Color::rgb(180, 200, 210),
            item_uncommon: Color::rgb(100, 200, 180),
            item_rare: Color::rgb(100, 160, 255),
            item_epic: Color::rgb(180, 120, 220),
            item_legendary: Color::rgb(255, 220, 100),
            item_artifact: Color::rgb(255, 240, 180),
            item_cursed: Color::rgb(100, 60, 100),

            // Status Colors
            hp_high: Color::rgb(100, 200, 180),
            hp_medium: Color::rgb(200, 200, 120),
            hp_low: Color::rgb(200, 100, 120),
            mp_color: Color::rgb(100, 150, 220),
            xp_color: Color::rgb(180, 150, 220),
            stamina_color: Color::rgb(200, 200, 150),
            shield_color: Color::rgb(150, 200, 240),

            // Effect Colors
            effect_fire: Color::rgb(255, 160, 80),
            effect_ice: Color::rgb(180, 230, 255),
            effect_lightning: Color::rgb(220, 240, 255),
            effect_poison: Color::rgb(100, 200, 150),
            effect_heal: Color::rgb(150, 230, 220),
            effect_magic: Color::rgb(180, 160, 255),
            effect_shadow: Color::rgb(50, 70, 100),
            effect_holy: Color::rgb(255, 255, 240),

            // Message Colors
            msg_normal: Color::rgb(200, 220, 240),
            msg_combat: Color::rgb(255, 160, 160),
            msg_item: Color::rgb(150, 220, 255),
            msg_system: Color::rgb(255, 240, 180),
            msg_story: Color::rgb(200, 210, 255),
        }
    }

    /// Create a custom theme with the given name
    pub fn custom(name: &str) -> Self {
        let mut theme = Self::classic();
        theme.id = ThemeId::Custom;
        theme.name = name.to_string();
        theme.description = "User-defined custom color scheme.".to_string();
        theme
    }

    /// Get a theme by its ID
    pub fn from_id(id: ThemeId) -> Self {
        match id {
            ThemeId::Classic => Self::classic(),
            ThemeId::DarkFantasy => Self::dark_fantasy(),
            ThemeId::HighContrast => Self::high_contrast(),
            ThemeId::RetroGreen => Self::retro_green(),
            ThemeId::Crimson => Self::crimson(),
            ThemeId::Ocean => Self::ocean(),
            ThemeId::Forest => Self::forest(),
            ThemeId::Void => Self::void(),
            ThemeId::Golden => Self::golden(),
            ThemeId::Ice => Self::ice(),
            ThemeId::Custom => Self::custom("Custom"),
        }
    }

    // ========================================================================
    // Theme Modification Methods
    // ========================================================================

    /// Apply colorblind filter to all colors in the theme
    pub fn apply_colorblind_filter(&mut self, mode: ColorblindMode) {
        if mode == ColorblindMode::None {
            return;
        }

        // Apply filter to all colors
        self.background = self.background.apply_colorblind_filter(mode);
        self.foreground = self.foreground.apply_colorblind_filter(mode);
        self.foreground_dim = self.foreground_dim.apply_colorblind_filter(mode);
        self.border = self.border.apply_colorblind_filter(mode);
        self.highlight = self.highlight.apply_colorblind_filter(mode);
        self.accent = self.accent.apply_colorblind_filter(mode);
        self.warning = self.warning.apply_colorblind_filter(mode);
        self.error = self.error.apply_colorblind_filter(mode);
        self.success = self.success.apply_colorblind_filter(mode);

        self.player = self.player.apply_colorblind_filter(mode);
        self.enemy_weak = self.enemy_weak.apply_colorblind_filter(mode);
        self.enemy_normal = self.enemy_normal.apply_colorblind_filter(mode);
        self.enemy_strong = self.enemy_strong.apply_colorblind_filter(mode);
        self.enemy_boss = self.enemy_boss.apply_colorblind_filter(mode);
        self.npc = self.npc.apply_colorblind_filter(mode);
        self.companion = self.companion.apply_colorblind_filter(mode);
        self.neutral = self.neutral.apply_colorblind_filter(mode);

        self.wall = self.wall.apply_colorblind_filter(mode);
        self.floor = self.floor.apply_colorblind_filter(mode);
        self.door = self.door.apply_colorblind_filter(mode);
        self.stairs_up = self.stairs_up.apply_colorblind_filter(mode);
        self.stairs_down = self.stairs_down.apply_colorblind_filter(mode);
        self.water = self.water.apply_colorblind_filter(mode);
        self.lava = self.lava.apply_colorblind_filter(mode);
        self.grass = self.grass.apply_colorblind_filter(mode);
        self.trap = self.trap.apply_colorblind_filter(mode);
        self.secret = self.secret.apply_colorblind_filter(mode);
        self.fog_unexplored = self.fog_unexplored.apply_colorblind_filter(mode);
        self.fog_explored = self.fog_explored.apply_colorblind_filter(mode);

        self.item_common = self.item_common.apply_colorblind_filter(mode);
        self.item_uncommon = self.item_uncommon.apply_colorblind_filter(mode);
        self.item_rare = self.item_rare.apply_colorblind_filter(mode);
        self.item_epic = self.item_epic.apply_colorblind_filter(mode);
        self.item_legendary = self.item_legendary.apply_colorblind_filter(mode);
        self.item_artifact = self.item_artifact.apply_colorblind_filter(mode);
        self.item_cursed = self.item_cursed.apply_colorblind_filter(mode);

        self.hp_high = self.hp_high.apply_colorblind_filter(mode);
        self.hp_medium = self.hp_medium.apply_colorblind_filter(mode);
        self.hp_low = self.hp_low.apply_colorblind_filter(mode);
        self.mp_color = self.mp_color.apply_colorblind_filter(mode);
        self.xp_color = self.xp_color.apply_colorblind_filter(mode);
        self.stamina_color = self.stamina_color.apply_colorblind_filter(mode);
        self.shield_color = self.shield_color.apply_colorblind_filter(mode);

        self.effect_fire = self.effect_fire.apply_colorblind_filter(mode);
        self.effect_ice = self.effect_ice.apply_colorblind_filter(mode);
        self.effect_lightning = self.effect_lightning.apply_colorblind_filter(mode);
        self.effect_poison = self.effect_poison.apply_colorblind_filter(mode);
        self.effect_heal = self.effect_heal.apply_colorblind_filter(mode);
        self.effect_magic = self.effect_magic.apply_colorblind_filter(mode);
        self.effect_shadow = self.effect_shadow.apply_colorblind_filter(mode);
        self.effect_holy = self.effect_holy.apply_colorblind_filter(mode);

        self.msg_normal = self.msg_normal.apply_colorblind_filter(mode);
        self.msg_combat = self.msg_combat.apply_colorblind_filter(mode);
        self.msg_item = self.msg_item.apply_colorblind_filter(mode);
        self.msg_system = self.msg_system.apply_colorblind_filter(mode);
        self.msg_story = self.msg_story.apply_colorblind_filter(mode);
    }

    /// Create a copy with colorblind filter applied
    pub fn with_colorblind_filter(&self, mode: ColorblindMode) -> Self {
        let mut theme = self.clone();
        theme.apply_colorblind_filter(mode);
        theme
    }

    /// Generate a preview palette of key colors
    pub fn preview_palette(&self) -> Vec<(&'static str, Color)> {
        vec![
            ("Background", self.background),
            ("Foreground", self.foreground),
            ("Highlight", self.highlight),
            ("Accent", self.accent),
            ("Player", self.player),
            ("Enemy", self.enemy_normal),
            ("Boss", self.enemy_boss),
            ("NPC", self.npc),
            ("Wall", self.wall),
            ("Floor", self.floor),
            ("Water", self.water),
            ("Lava", self.lava),
            ("Common Item", self.item_common),
            ("Rare Item", self.item_rare),
            ("Legendary", self.item_legendary),
            ("HP High", self.hp_high),
            ("HP Low", self.hp_low),
            ("Mana", self.mp_color),
        ]
    }
}

// ============================================================================
// Theme System
// ============================================================================

/// Theme management system
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThemeSystem {
    /// Currently active theme
    pub active_theme: ColorTheme,
    /// Active colorblind mode
    pub colorblind_mode: ColorblindMode,
    /// Custom themes created by the user
    pub custom_themes: HashMap<String, ColorTheme>,
    /// Theme history for undo
    theme_history: Vec<ThemeId>,
    /// Maximum history size
    max_history: usize,
}

impl Default for ThemeSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl ThemeSystem {
    /// Create a new theme system with default settings
    pub fn new() -> Self {
        Self {
            active_theme: ColorTheme::classic(),
            colorblind_mode: ColorblindMode::None,
            custom_themes: HashMap::new(),
            theme_history: vec![ThemeId::Classic],
            max_history: 10,
        }
    }

    /// Get the current active theme
    pub fn current_theme(&self) -> &ColorTheme {
        &self.active_theme
    }

    /// Get a mutable reference to the current theme
    pub fn current_theme_mut(&mut self) -> &mut ColorTheme {
        &mut self.active_theme
    }

    /// Load a built-in theme by ID
    pub fn load_theme(&mut self, id: ThemeId) {
        self.theme_history.push(id);
        if self.theme_history.len() > self.max_history {
            self.theme_history.remove(0);
        }

        let mut theme = ColorTheme::from_id(id);
        if self.colorblind_mode != ColorblindMode::None {
            theme.apply_colorblind_filter(self.colorblind_mode);
        }
        self.active_theme = theme;
    }

    /// Apply a theme directly
    pub fn apply_theme(&mut self, theme: ColorTheme) {
        self.theme_history.push(theme.id);
        if self.theme_history.len() > self.max_history {
            self.theme_history.remove(0);
        }

        let mut theme = theme;
        if self.colorblind_mode != ColorblindMode::None {
            theme.apply_colorblind_filter(self.colorblind_mode);
        }
        self.active_theme = theme;
    }

    /// Load a custom theme by name
    pub fn load_custom_theme(&mut self, name: &str) -> bool {
        if let Some(theme) = self.custom_themes.get(name).cloned() {
            self.apply_theme(theme);
            true
        } else {
            false
        }
    }

    /// Create and save a custom theme
    pub fn create_custom_theme(&mut self, name: &str, base_theme: ThemeId) -> &mut ColorTheme {
        let mut theme = ColorTheme::from_id(base_theme);
        theme.id = ThemeId::Custom;
        theme.name = name.to_string();
        theme.description = format!("Custom theme based on {}", base_theme.name());
        self.custom_themes.insert(name.to_string(), theme);
        self.custom_themes.get_mut(name).unwrap()
    }

    /// Save the current theme as a custom theme
    pub fn save_current_as_custom(&mut self, name: &str) {
        let mut theme = self.active_theme.clone();
        theme.id = ThemeId::Custom;
        theme.name = name.to_string();
        self.custom_themes.insert(name.to_string(), theme);
    }

    /// Delete a custom theme
    pub fn delete_custom_theme(&mut self, name: &str) -> bool {
        self.custom_themes.remove(name).is_some()
    }

    /// Get list of all custom theme names
    pub fn custom_theme_names(&self) -> Vec<&String> {
        self.custom_themes.keys().collect()
    }

    /// Set colorblind mode
    pub fn set_colorblind_mode(&mut self, mode: ColorblindMode) {
        self.colorblind_mode = mode;
        // Reload current theme with new colorblind settings
        let current_id = self.active_theme.id;
        self.load_theme(current_id);
    }

    /// Get current colorblind mode
    pub fn get_colorblind_mode(&self) -> ColorblindMode {
        self.colorblind_mode
    }

    /// Revert to previous theme
    pub fn revert_theme(&mut self) -> bool {
        if self.theme_history.len() > 1 {
            self.theme_history.pop(); // Remove current
            if let Some(&prev_id) = self.theme_history.last() {
                let mut theme = ColorTheme::from_id(prev_id);
                if self.colorblind_mode != ColorblindMode::None {
                    theme.apply_colorblind_filter(self.colorblind_mode);
                }
                self.active_theme = theme;
                return true;
            }
        }
        false
    }

    /// Get all available built-in themes
    pub fn available_themes() -> Vec<ThemeId> {
        ThemeId::all_builtin().to_vec()
    }

    /// Generate theme preview data
    pub fn preview_theme(&self, id: ThemeId) -> ThemePreview {
        let theme = ColorTheme::from_id(id);
        ThemePreview {
            id,
            name: theme.name.clone(),
            description: theme.description.clone(),
            palette: theme.preview_palette().iter()
                .map(|(name, color)| (name.to_string(), *color))
                .collect(),
        }
    }

    /// Preview all available themes
    pub fn preview_all_themes(&self) -> Vec<ThemePreview> {
        ThemeId::all_builtin()
            .iter()
            .map(|&id| self.preview_theme(id))
            .collect()
    }

    /// Export theme to JSON string
    pub fn export_theme(&self, theme: &ColorTheme) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(theme)
    }

    /// Import theme from JSON string
    pub fn import_theme(&mut self, json: &str) -> Result<ColorTheme, serde_json::Error> {
        let theme: ColorTheme = serde_json::from_str(json)?;
        Ok(theme)
    }

    /// Import and save a custom theme from JSON
    pub fn import_custom_theme(&mut self, json: &str) -> Result<String, serde_json::Error> {
        let theme = self.import_theme(json)?;
        let name = theme.name.clone();
        self.custom_themes.insert(name.clone(), theme);
        Ok(name)
    }
}

/// Theme preview information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThemePreview {
    pub id: ThemeId,
    pub name: String,
    pub description: String,
    pub palette: Vec<(String, Color)>,
}

// ============================================================================
// Color Utility Functions
// ============================================================================

/// Get HP color based on percentage
pub fn hp_color(theme: &ColorTheme, current: i32, max: i32) -> Color {
    if max <= 0 {
        return theme.hp_low;
    }
    let percentage = (current as f32 / max as f32) * 100.0;
    if percentage > 60.0 {
        theme.hp_high
    } else if percentage > 25.0 {
        theme.hp_medium
    } else {
        theme.hp_low
    }
}

/// Get item rarity color
pub fn rarity_color(theme: &ColorTheme, rarity: ItemRarity) -> Color {
    match rarity {
        ItemRarity::Common => theme.item_common,
        ItemRarity::Uncommon => theme.item_uncommon,
        ItemRarity::Rare => theme.item_rare,
        ItemRarity::Epic => theme.item_epic,
        ItemRarity::Legendary => theme.item_legendary,
        ItemRarity::Artifact => theme.item_artifact,
        ItemRarity::Cursed => theme.item_cursed,
    }
}

/// Get enemy color based on threat level
pub fn enemy_color(theme: &ColorTheme, threat: EnemyThreat) -> Color {
    match threat {
        EnemyThreat::Weak => theme.enemy_weak,
        EnemyThreat::Normal => theme.enemy_normal,
        EnemyThreat::Strong => theme.enemy_strong,
        EnemyThreat::Boss => theme.enemy_boss,
    }
}

/// Item rarity levels for color selection
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Artifact,
    Cursed,
}

/// Enemy threat levels for color selection
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnemyThreat {
    Weak,
    Normal,
    Strong,
    Boss,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color::rgb(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.a, 255);

        let hex_color = Color::from_hex(0xFF8040);
        assert_eq!(hex_color.r, 255);
        assert_eq!(hex_color.g, 128);
        assert_eq!(hex_color.b, 64);
    }

    #[test]
    fn test_hex_string_conversion() {
        let color = Color::rgb(255, 128, 64);
        assert_eq!(color.to_hex_string(), "#FF8040");

        let parsed = Color::from_hex_str("#FF8040").unwrap();
        assert_eq!(parsed, color);

        let parsed_no_hash = Color::from_hex_str("FF8040").unwrap();
        assert_eq!(parsed_no_hash, color);
    }

    #[test]
    fn test_color_blend() {
        let white = Color::WHITE;
        let black = Color::BLACK;
        let gray = white.blend(&black, 0.5);
        assert_eq!(gray.r, 127);
        assert_eq!(gray.g, 127);
        assert_eq!(gray.b, 127);
    }

    #[test]
    fn test_all_builtin_themes() {
        for &id in ThemeId::all_builtin() {
            let theme = ColorTheme::from_id(id);
            assert!(!theme.name.is_empty());
            assert!(!theme.description.is_empty());
        }
    }

    #[test]
    fn test_theme_system() {
        let mut system = ThemeSystem::new();
        assert_eq!(system.active_theme.id, ThemeId::Classic);

        system.load_theme(ThemeId::DarkFantasy);
        assert_eq!(system.active_theme.id, ThemeId::DarkFantasy);

        system.set_colorblind_mode(ColorblindMode::Deuteranopia);
        assert_eq!(system.colorblind_mode, ColorblindMode::Deuteranopia);
    }

    #[test]
    fn test_custom_theme() {
        let mut system = ThemeSystem::new();
        system.create_custom_theme("MyTheme", ThemeId::Classic);
        assert!(system.custom_themes.contains_key("MyTheme"));
        assert!(system.load_custom_theme("MyTheme"));
        assert_eq!(system.active_theme.name, "MyTheme");
    }

    #[test]
    fn test_colorblind_filter() {
        let color = Color::rgb(255, 0, 0); // Pure red
        let filtered = color.apply_colorblind_filter(ColorblindMode::Deuteranopia);
        // Deuteranopia should alter red-green perception
        assert_ne!(color, filtered);
    }

    #[test]
    fn test_hp_color() {
        let theme = ColorTheme::classic();
        assert_eq!(hp_color(&theme, 100, 100), theme.hp_high);
        assert_eq!(hp_color(&theme, 50, 100), theme.hp_medium);
        assert_eq!(hp_color(&theme, 10, 100), theme.hp_low);
    }

    #[test]
    fn test_theme_preview() {
        let system = ThemeSystem::new();
        let preview = system.preview_theme(ThemeId::Classic);
        assert!(!preview.palette.is_empty());
        assert_eq!(preview.name, "Classic");
    }

    #[test]
    fn test_theme_export_import() {
        let mut system = ThemeSystem::new();
        let theme = ColorTheme::dark_fantasy();
        let json = system.export_theme(&theme).unwrap();
        let imported = system.import_theme(&json).unwrap();
        assert_eq!(imported.name, theme.name);
    }
}
