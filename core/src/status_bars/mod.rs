//! Status Bars Rendering Module
//!
//! This module provides comprehensive status bar rendering for both CLI (crossterm)
//! and GUI (egui) frontends. It supports various bar types, styles, and color gradients
//! for health, mana, stamina, experience, and custom resource displays.
//!
//! # Features
//!
//! - Multiple bar types (health, mana, stamina, experience, energy, rage, custom)
//! - Various visual styles (block, line, dots, arrows, custom characters)
//! - Dynamic color gradients based on fill percentage
//! - Special bars for bosses, buffs/debuffs, cooldowns, and cast times
//! - Both CLI and GUI rendering support
//!
//! # Example
//!
//! ```rust,no_run
//! use shadowcrypt_core::status_bars::{StatusBar, BarType, BarStyle};
//!
//! let health_bar = StatusBar::new(75, 100, BarType::Health)
//!     .with_width(20)
//!     .with_text(true)
//!     .with_percentage(true);
//!
//! let rendered = health_bar.render_ascii();
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

// ============================================================================
// Constants
// ============================================================================

/// Default bar width in characters
pub const DEFAULT_BAR_WIDTH: u16 = 20;

/// Minimum bar width
pub const MIN_BAR_WIDTH: u16 = 5;

/// Maximum bar width
pub const MAX_BAR_WIDTH: u16 = 100;

/// Boss bar default width (wider for visibility)
pub const BOSS_BAR_WIDTH: u16 = 40;

/// Cooldown bar default width
pub const COOLDOWN_BAR_WIDTH: u16 = 10;

// ============================================================================
// Color Types
// ============================================================================

/// RGB color representation for both CLI and GUI
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Create a new color from RGB values
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Create color from hex value
    pub const fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
        }
    }

    /// Interpolate between two colors
    pub fn lerp(&self, other: &Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        Color {
            r: (self.r as f32 + (other.r as f32 - self.r as f32) * t) as u8,
            g: (self.g as f32 + (other.g as f32 - self.g as f32) * t) as u8,
            b: (self.b as f32 + (other.b as f32 - self.b as f32) * t) as u8,
        }
    }

    /// Convert to crossterm color
    #[cfg(feature = "crossterm")]
    pub fn to_crossterm(&self) -> crossterm::style::Color {
        crossterm::style::Color::Rgb {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }

    /// Convert to egui color
    #[cfg(feature = "egui")]
    pub fn to_egui(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.r, self.g, self.b)
    }

    /// Convert to ANSI escape sequence
    pub fn to_ansi_fg(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.r, self.g, self.b)
    }

    /// Convert to ANSI background escape sequence
    pub fn to_ansi_bg(&self) -> String {
        format!("\x1b[48;2;{};{};{}m", self.r, self.g, self.b)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}

// Predefined colors
impl Color {
    pub const WHITE: Color = Color::new(255, 255, 255);
    pub const BLACK: Color = Color::new(0, 0, 0);
    pub const GRAY: Color = Color::new(128, 128, 128);
    pub const DARK_GRAY: Color = Color::new(64, 64, 64);

    // Health colors
    pub const HEALTH_FULL: Color = Color::new(0, 255, 0);        // Bright green
    pub const HEALTH_HIGH: Color = Color::new(144, 238, 144);    // Light green
    pub const HEALTH_MEDIUM: Color = Color::new(255, 255, 0);    // Yellow
    pub const HEALTH_LOW: Color = Color::new(255, 69, 0);        // Red-orange
    pub const HEALTH_CRITICAL: Color = Color::new(139, 0, 0);    // Dark red

    // Mana colors
    pub const MANA_FULL: Color = Color::new(135, 206, 250);      // Light sky blue
    pub const MANA_HIGH: Color = Color::new(100, 149, 237);      // Cornflower blue
    pub const MANA_MEDIUM: Color = Color::new(65, 105, 225);     // Royal blue
    pub const MANA_LOW: Color = Color::new(0, 0, 139);           // Dark blue

    // Stamina colors
    pub const STAMINA_FULL: Color = Color::new(144, 238, 144);   // Light green
    pub const STAMINA_MEDIUM: Color = Color::new(34, 139, 34);   // Forest green
    pub const STAMINA_LOW: Color = Color::new(0, 100, 0);        // Dark green

    // Experience colors
    pub const XP_HIGH: Color = Color::new(218, 112, 214);        // Light purple
    pub const XP_LOW: Color = Color::new(128, 0, 128);           // Purple

    // Energy colors
    pub const ENERGY_FULL: Color = Color::new(255, 255, 0);      // Yellow
    pub const ENERGY_LOW: Color = Color::new(255, 165, 0);       // Orange

    // Rage colors
    pub const RAGE_FULL: Color = Color::new(255, 0, 0);          // Red
    pub const RAGE_LOW: Color = Color::new(139, 0, 0);           // Dark red

    // Special colors
    pub const BOSS_COLOR: Color = Color::new(255, 0, 255);       // Magenta
    pub const BUFF_COLOR: Color = Color::new(0, 255, 255);       // Cyan
    pub const DEBUFF_COLOR: Color = Color::new(128, 0, 128);     // Purple
    pub const COOLDOWN_COLOR: Color = Color::new(169, 169, 169); // Dark gray
    pub const CAST_COLOR: Color = Color::new(255, 215, 0);       // Gold
}

// ============================================================================
// Bar Style Characters
// ============================================================================

/// Bar style defining the visual appearance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BarStyle {
    /// Block style: [][][]
    Block,
    /// Line style: ----
    Line,
    /// Dots style:
    Dots,
    /// Arrows style: >>>>
    Arrows,
    /// Solid blocks:
    SolidBlock,
    /// Half blocks for smoother gradients:
    HalfBlock,
    /// Custom characters
    Custom {
        filled: char,
        empty: char,
        left_cap: char,
        right_cap: char,
    },
}

impl Default for BarStyle {
    fn default() -> Self {
        Self::SolidBlock
    }
}

impl BarStyle {
    /// Get the filled character for this style
    pub fn filled_char(&self) -> char {
        match self {
            BarStyle::Block => '\u{2588}',      // Full block
            BarStyle::Line => '\u{2500}',       // Light horizontal
            BarStyle::Dots => '\u{25CF}',       // Black circle
            BarStyle::Arrows => '>',
            BarStyle::SolidBlock => '\u{2588}', // Full block
            BarStyle::HalfBlock => '\u{2588}',  // Full block
            BarStyle::Custom { filled, .. } => *filled,
        }
    }

    /// Get the empty character for this style
    pub fn empty_char(&self) -> char {
        match self {
            BarStyle::Block => '\u{2591}',      // Light shade
            BarStyle::Line => '\u{2508}',       // Light quadruple dash
            BarStyle::Dots => '\u{25CB}',       // White circle
            BarStyle::Arrows => '-',
            BarStyle::SolidBlock => '\u{2591}', // Light shade
            BarStyle::HalfBlock => '\u{2591}',  // Light shade
            BarStyle::Custom { empty, .. } => *empty,
        }
    }

    /// Get the left cap character
    pub fn left_cap(&self) -> char {
        match self {
            BarStyle::Custom { left_cap, .. } => *left_cap,
            _ => '[',
        }
    }

    /// Get the right cap character
    pub fn right_cap(&self) -> char {
        match self {
            BarStyle::Custom { right_cap, .. } => *right_cap,
            _ => ']',
        }
    }
}

// ============================================================================
// Bar Types
// ============================================================================

/// Type of status bar with associated color scheme
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BarType {
    /// Health bar with green-yellow-red gradient
    Health,
    /// Mana bar with light blue to dark blue gradient
    Mana,
    /// Stamina bar with green gradient
    Stamina,
    /// Experience bar with purple gradient
    Experience,
    /// Energy bar with yellow-orange gradient
    Energy,
    /// Rage bar with red gradient
    Rage,
    /// Custom bar with specified colors
    Custom {
        filled_char: char,
        empty_char: char,
        color: Color,
    },
}

impl Default for BarType {
    fn default() -> Self {
        Self::Health
    }
}

impl BarType {
    /// Get the color for a given percentage based on bar type
    pub fn get_color(&self, percentage: f32) -> Color {
        match self {
            BarType::Health => Self::health_gradient(percentage),
            BarType::Mana => Self::mana_gradient(percentage),
            BarType::Stamina => Self::stamina_gradient(percentage),
            BarType::Experience => Self::xp_gradient(percentage),
            BarType::Energy => Self::energy_gradient(percentage),
            BarType::Rage => Self::rage_gradient(percentage),
            BarType::Custom { color, .. } => *color,
        }
    }

    /// Health gradient: Green (100%) -> Yellow (50%) -> Red (25%) -> Dark Red (10%)
    fn health_gradient(percentage: f32) -> Color {
        if percentage >= 75.0 {
            // Green to light green
            Color::HEALTH_FULL.lerp(&Color::HEALTH_HIGH, (100.0 - percentage) / 25.0)
        } else if percentage >= 50.0 {
            // Light green to yellow
            Color::HEALTH_HIGH.lerp(&Color::HEALTH_MEDIUM, (75.0 - percentage) / 25.0)
        } else if percentage >= 25.0 {
            // Yellow to red-orange
            Color::HEALTH_MEDIUM.lerp(&Color::HEALTH_LOW, (50.0 - percentage) / 25.0)
        } else if percentage >= 10.0 {
            // Red-orange to dark red
            Color::HEALTH_LOW.lerp(&Color::HEALTH_CRITICAL, (25.0 - percentage) / 15.0)
        } else {
            Color::HEALTH_CRITICAL
        }
    }

    /// Mana gradient: Light Blue (100%) -> Blue (50%) -> Dark Blue (25%)
    fn mana_gradient(percentage: f32) -> Color {
        if percentage >= 75.0 {
            Color::MANA_FULL.lerp(&Color::MANA_HIGH, (100.0 - percentage) / 25.0)
        } else if percentage >= 50.0 {
            Color::MANA_HIGH.lerp(&Color::MANA_MEDIUM, (75.0 - percentage) / 25.0)
        } else {
            Color::MANA_MEDIUM.lerp(&Color::MANA_LOW, (50.0 - percentage) / 50.0)
        }
    }

    /// Stamina gradient: Light Green -> Green -> Dark Green
    fn stamina_gradient(percentage: f32) -> Color {
        if percentage >= 66.0 {
            Color::STAMINA_FULL.lerp(&Color::STAMINA_MEDIUM, (100.0 - percentage) / 34.0)
        } else if percentage >= 33.0 {
            Color::STAMINA_MEDIUM.lerp(&Color::STAMINA_LOW, (66.0 - percentage) / 33.0)
        } else {
            Color::STAMINA_LOW
        }
    }

    /// XP gradient: Light Purple -> Purple
    fn xp_gradient(percentage: f32) -> Color {
        Color::XP_LOW.lerp(&Color::XP_HIGH, percentage / 100.0)
    }

    /// Energy gradient: Yellow -> Orange
    fn energy_gradient(percentage: f32) -> Color {
        Color::ENERGY_LOW.lerp(&Color::ENERGY_FULL, percentage / 100.0)
    }

    /// Rage gradient: Dark Red -> Red
    fn rage_gradient(percentage: f32) -> Color {
        Color::RAGE_LOW.lerp(&Color::RAGE_FULL, percentage / 100.0)
    }
}

// ============================================================================
// Main StatusBar Struct
// ============================================================================

/// A status bar that can be rendered in various styles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusBar {
    /// Current value
    pub current: i32,
    /// Maximum value
    pub maximum: i32,
    /// Type of bar (determines color scheme)
    pub bar_type: BarType,
    /// Width of the bar in characters
    pub width: u16,
    /// Whether to show numeric text (e.g., "80/100")
    pub show_text: bool,
    /// Whether to show percentage
    pub show_percentage: bool,
    /// Visual style of the bar
    pub style: BarStyle,
    /// Optional label for the bar
    pub label: Option<String>,
    /// Whether to show left/right caps
    pub show_caps: bool,
    /// Empty bar color override
    pub empty_color: Option<Color>,
}

impl Default for StatusBar {
    fn default() -> Self {
        Self {
            current: 100,
            maximum: 100,
            bar_type: BarType::Health,
            width: DEFAULT_BAR_WIDTH,
            show_text: true,
            show_percentage: false,
            style: BarStyle::SolidBlock,
            label: None,
            show_caps: true,
            empty_color: None,
        }
    }
}

impl StatusBar {
    /// Create a new status bar
    pub fn new(current: i32, maximum: i32, bar_type: BarType) -> Self {
        Self {
            current: current.max(0),
            maximum: maximum.max(1),
            bar_type,
            ..Default::default()
        }
    }

    /// Set the bar width
    pub fn with_width(mut self, width: u16) -> Self {
        self.width = width.clamp(MIN_BAR_WIDTH, MAX_BAR_WIDTH);
        self
    }

    /// Set whether to show numeric text
    pub fn with_text(mut self, show: bool) -> Self {
        self.show_text = show;
        self
    }

    /// Set whether to show percentage
    pub fn with_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }

    /// Set the bar style
    pub fn with_style(mut self, style: BarStyle) -> Self {
        self.style = style;
        self
    }

    /// Set a label for the bar
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set whether to show caps
    pub fn with_caps(mut self, show: bool) -> Self {
        self.show_caps = show;
        self
    }

    /// Set empty bar color
    pub fn with_empty_color(mut self, color: Color) -> Self {
        self.empty_color = Some(color);
        self
    }

    /// Calculate the fill percentage (0.0 - 100.0)
    pub fn percentage(&self) -> f32 {
        if self.maximum <= 0 {
            return 0.0;
        }
        (self.current as f32 / self.maximum as f32 * 100.0).clamp(0.0, 100.0)
    }

    /// Get the current color based on fill percentage
    pub fn current_color(&self) -> Color {
        self.bar_type.get_color(self.percentage())
    }

    /// Get the number of filled characters
    pub fn filled_count(&self) -> u16 {
        let pct = self.percentage() / 100.0;
        (self.width as f32 * pct).round() as u16
    }

    /// Update the current value
    pub fn set_current(&mut self, value: i32) {
        self.current = value.clamp(0, self.maximum);
    }

    /// Update the maximum value
    pub fn set_maximum(&mut self, value: i32) {
        self.maximum = value.max(1);
        self.current = self.current.min(self.maximum);
    }

    /// Render to plain ASCII string (no colors)
    pub fn render_plain(&self) -> String {
        let filled = self.filled_count() as usize;
        let empty = self.width as usize - filled;

        let filled_char = self.style.filled_char();
        let empty_char = self.style.empty_char();

        let bar: String = std::iter::repeat(filled_char)
            .take(filled)
            .chain(std::iter::repeat(empty_char).take(empty))
            .collect();

        let mut result = String::new();

        // Add label if present
        if let Some(ref label) = self.label {
            result.push_str(label);
            result.push_str(": ");
        }

        // Add caps and bar
        if self.show_caps {
            result.push(self.style.left_cap());
        }
        result.push_str(&bar);
        if self.show_caps {
            result.push(self.style.right_cap());
        }

        // Add text display
        if self.show_text {
            result.push_str(&format!(" {}/{}", self.current, self.maximum));
        }

        // Add percentage display
        if self.show_percentage {
            result.push_str(&format!(" ({:.0}%)", self.percentage()));
        }

        result
    }

    /// Render to ASCII string with ANSI color codes
    pub fn render_ascii(&self) -> String {
        let filled = self.filled_count() as usize;
        let empty = self.width as usize - filled;

        let filled_char = self.style.filled_char();
        let empty_char = self.style.empty_char();
        let color = self.current_color();
        let empty_color = self.empty_color.unwrap_or(Color::DARK_GRAY);

        let filled_str: String = std::iter::repeat(filled_char).take(filled).collect();
        let empty_str: String = std::iter::repeat(empty_char).take(empty).collect();

        let mut result = String::new();

        // Add label if present
        if let Some(ref label) = self.label {
            result.push_str(label);
            result.push_str(": ");
        }

        // Add caps and bar with colors
        if self.show_caps {
            result.push(self.style.left_cap());
        }

        // Colored filled portion
        result.push_str(&color.to_ansi_fg());
        result.push_str(&filled_str);

        // Colored empty portion
        result.push_str(&empty_color.to_ansi_fg());
        result.push_str(&empty_str);

        // Reset color
        result.push_str("\x1b[0m");

        if self.show_caps {
            result.push(self.style.right_cap());
        }

        // Add text display
        if self.show_text {
            result.push_str(&format!(" {}/{}", self.current, self.maximum));
        }

        // Add percentage display
        if self.show_percentage {
            result.push_str(&format!(" ({:.0}%)", self.percentage()));
        }

        result
    }

    /// Render with gradient colors (each character can have different color)
    pub fn render_gradient_ascii(&self) -> String {
        let filled = self.filled_count() as usize;
        let empty = self.width as usize - filled;

        let filled_char = self.style.filled_char();
        let empty_char = self.style.empty_char();
        let empty_color = self.empty_color.unwrap_or(Color::DARK_GRAY);

        let mut result = String::new();

        // Add label if present
        if let Some(ref label) = self.label {
            result.push_str(label);
            result.push_str(": ");
        }

        if self.show_caps {
            result.push(self.style.left_cap());
        }

        // Render each filled character with gradient
        for i in 0..filled {
            let segment_pct = (i as f32 / self.width as f32) * 100.0;
            let color = self.bar_type.get_color(100.0 - segment_pct * 0.5);
            result.push_str(&color.to_ansi_fg());
            result.push(filled_char);
        }

        // Render empty portion
        result.push_str(&empty_color.to_ansi_fg());
        for _ in 0..empty {
            result.push(empty_char);
        }

        result.push_str("\x1b[0m");

        if self.show_caps {
            result.push(self.style.right_cap());
        }

        if self.show_text {
            result.push_str(&format!(" {}/{}", self.current, self.maximum));
        }

        if self.show_percentage {
            result.push_str(&format!(" ({:.0}%)", self.percentage()));
        }

        result
    }

    /// Get rendering data for egui
    pub fn to_gui_data(&self) -> StatusBarGuiData {
        StatusBarGuiData {
            percentage: self.percentage() / 100.0,
            current: self.current,
            maximum: self.maximum,
            color: self.current_color(),
            empty_color: self.empty_color.unwrap_or(Color::DARK_GRAY),
            label: self.label.clone(),
            show_text: self.show_text,
            show_percentage: self.show_percentage,
        }
    }
}

impl fmt::Display for StatusBar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render_plain())
    }
}

// ============================================================================
// GUI Data Struct
// ============================================================================

/// Data structure for GUI rendering
#[derive(Debug, Clone)]
pub struct StatusBarGuiData {
    /// Fill percentage (0.0 - 1.0)
    pub percentage: f32,
    /// Current value
    pub current: i32,
    /// Maximum value
    pub maximum: i32,
    /// Fill color
    pub color: Color,
    /// Empty/background color
    pub empty_color: Color,
    /// Optional label
    pub label: Option<String>,
    /// Whether to show numeric text
    pub show_text: bool,
    /// Whether to show percentage
    pub show_percentage: bool,
}

// ============================================================================
// Special Bar Types
// ============================================================================

/// Boss health bar with multiple segments and special styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossHealthBar {
    /// Boss name
    pub name: String,
    /// Current health
    pub current_hp: i32,
    /// Maximum health
    pub max_hp: i32,
    /// Number of segments (for multi-phase bosses)
    pub segments: u8,
    /// Current segment
    pub current_segment: u8,
    /// Bar width
    pub width: u16,
    /// Whether the boss is enraged
    pub enraged: bool,
    /// Phase name
    pub phase_name: Option<String>,
}

impl BossHealthBar {
    /// Create a new boss health bar
    pub fn new(name: impl Into<String>, current_hp: i32, max_hp: i32) -> Self {
        Self {
            name: name.into(),
            current_hp: current_hp.max(0),
            max_hp: max_hp.max(1),
            segments: 1,
            current_segment: 1,
            width: BOSS_BAR_WIDTH,
            enraged: false,
            phase_name: None,
        }
    }

    /// Set the number of segments
    pub fn with_segments(mut self, segments: u8, current: u8) -> Self {
        self.segments = segments.max(1);
        self.current_segment = current.clamp(1, self.segments);
        self
    }

    /// Set enraged state
    pub fn with_enraged(mut self, enraged: bool) -> Self {
        self.enraged = enraged;
        self
    }

    /// Set phase name
    pub fn with_phase(mut self, phase: impl Into<String>) -> Self {
        self.phase_name = Some(phase.into());
        self
    }

    /// Get health percentage
    pub fn percentage(&self) -> f32 {
        (self.current_hp as f32 / self.max_hp as f32 * 100.0).clamp(0.0, 100.0)
    }

    /// Get the current color
    pub fn current_color(&self) -> Color {
        if self.enraged {
            Color::new(255, 50, 50) // Bright red when enraged
        } else {
            // Boss uses a special magenta-red gradient
            let pct = self.percentage();
            if pct > 50.0 {
                Color::BOSS_COLOR.lerp(&Color::new(255, 100, 100), (100.0 - pct) / 50.0)
            } else {
                Color::new(255, 100, 100).lerp(&Color::HEALTH_CRITICAL, (50.0 - pct) / 50.0)
            }
        }
    }

    /// Render to ASCII
    pub fn render_ascii(&self) -> String {
        let filled = (self.width as f32 * self.percentage() / 100.0).round() as usize;
        let empty = self.width as usize - filled;

        let filled_char = '\u{2588}'; // Full block
        let empty_char = '\u{2591}';  // Light shade
        let color = self.current_color();

        let mut result = String::new();

        // Boss name header
        result.push_str(&format!("=== {} ===\n", self.name));

        // Phase name if present
        if let Some(ref phase) = self.phase_name {
            result.push_str(&format!("[ {} ]\n", phase));
        }

        // Segment indicators
        if self.segments > 1 {
            result.push('[');
            for i in 1..=self.segments {
                if i <= self.current_segment {
                    result.push_str(&Color::BOSS_COLOR.to_ansi_fg());
                    result.push('\u{25A0}'); // Filled square
                } else {
                    result.push_str(&Color::DARK_GRAY.to_ansi_fg());
                    result.push('\u{25A1}'); // Empty square
                }
            }
            result.push_str("\x1b[0m");
            result.push_str("]\n");
        }

        // Health bar
        result.push('[');
        result.push_str(&color.to_ansi_fg());
        for _ in 0..filled {
            result.push(filled_char);
        }
        result.push_str(&Color::DARK_GRAY.to_ansi_fg());
        for _ in 0..empty {
            result.push(empty_char);
        }
        result.push_str("\x1b[0m");
        result.push_str(&format!("] {}/{} ({:.0}%)",
            self.current_hp, self.max_hp, self.percentage()));

        // Enraged indicator
        if self.enraged {
            result.push_str(&format!(" {}", Color::RAGE_FULL.to_ansi_fg()));
            result.push_str("ENRAGED!");
            result.push_str("\x1b[0m");
        }

        result
    }

    /// Get GUI data
    pub fn to_gui_data(&self) -> BossBarGuiData {
        BossBarGuiData {
            name: self.name.clone(),
            percentage: self.percentage() / 100.0,
            current_hp: self.current_hp,
            max_hp: self.max_hp,
            color: self.current_color(),
            segments: self.segments,
            current_segment: self.current_segment,
            enraged: self.enraged,
            phase_name: self.phase_name.clone(),
        }
    }
}

/// GUI data for boss health bar
#[derive(Debug, Clone)]
pub struct BossBarGuiData {
    pub name: String,
    pub percentage: f32,
    pub current_hp: i32,
    pub max_hp: i32,
    pub color: Color,
    pub segments: u8,
    pub current_segment: u8,
    pub enraged: bool,
    pub phase_name: Option<String>,
}

// ============================================================================
// Buff/Debuff Duration Bar
// ============================================================================

/// Duration bar for buffs and debuffs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurationBar {
    /// Effect name
    pub name: String,
    /// Remaining duration
    pub remaining: f32,
    /// Total duration
    pub total: f32,
    /// Whether this is a buff (true) or debuff (false)
    pub is_buff: bool,
    /// Bar width
    pub width: u16,
    /// Stack count (if applicable)
    pub stacks: Option<u8>,
}

impl DurationBar {
    /// Create a new duration bar
    pub fn new(name: impl Into<String>, remaining: f32, total: f32, is_buff: bool) -> Self {
        Self {
            name: name.into(),
            remaining: remaining.max(0.0),
            total: total.max(0.1),
            is_buff,
            width: 10,
            stacks: None,
        }
    }

    /// Set stack count
    pub fn with_stacks(mut self, stacks: u8) -> Self {
        self.stacks = Some(stacks);
        self
    }

    /// Get percentage remaining
    pub fn percentage(&self) -> f32 {
        (self.remaining / self.total * 100.0).clamp(0.0, 100.0)
    }

    /// Get the color
    pub fn color(&self) -> Color {
        if self.is_buff {
            // Buff: cyan fading to gray
            Color::BUFF_COLOR.lerp(&Color::GRAY, 1.0 - self.percentage() / 100.0)
        } else {
            // Debuff: purple fading to gray
            Color::DEBUFF_COLOR.lerp(&Color::GRAY, 1.0 - self.percentage() / 100.0)
        }
    }

    /// Render to ASCII
    pub fn render_ascii(&self) -> String {
        let filled = (self.width as f32 * self.percentage() / 100.0).round() as usize;
        let empty = self.width as usize - filled;
        let color = self.color();

        let mut result = String::new();

        // Name with optional stacks
        result.push_str(&self.name);
        if let Some(stacks) = self.stacks {
            result.push_str(&format!(" x{}", stacks));
        }
        result.push_str(": ");

        // Bar
        result.push('[');
        result.push_str(&color.to_ansi_fg());
        for _ in 0..filled {
            result.push('\u{2588}');
        }
        result.push_str(&Color::DARK_GRAY.to_ansi_fg());
        for _ in 0..empty {
            result.push('\u{2591}');
        }
        result.push_str("\x1b[0m");
        result.push_str(&format!("] {:.1}s", self.remaining));

        result
    }
}

// ============================================================================
// Cooldown Timer Bar
// ============================================================================

/// Cooldown timer bar for abilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CooldownBar {
    /// Ability name
    pub name: String,
    /// Remaining cooldown
    pub remaining: f32,
    /// Total cooldown
    pub total: f32,
    /// Whether the ability is ready
    pub ready: bool,
    /// Bar width
    pub width: u16,
}

impl CooldownBar {
    /// Create a new cooldown bar
    pub fn new(name: impl Into<String>, remaining: f32, total: f32) -> Self {
        Self {
            name: name.into(),
            remaining: remaining.max(0.0),
            total: total.max(0.1),
            ready: remaining <= 0.0,
            width: COOLDOWN_BAR_WIDTH,
        }
    }

    /// Get percentage of cooldown elapsed (inverted for display)
    pub fn percentage(&self) -> f32 {
        if self.ready {
            100.0
        } else {
            (1.0 - self.remaining / self.total) * 100.0
        }
    }

    /// Get the color
    pub fn color(&self) -> Color {
        if self.ready {
            Color::new(0, 255, 0) // Green when ready
        } else {
            // Gray to yellow as it gets closer to ready
            Color::COOLDOWN_COLOR.lerp(&Color::ENERGY_FULL, self.percentage() / 100.0)
        }
    }

    /// Render to ASCII
    pub fn render_ascii(&self) -> String {
        let filled = (self.width as f32 * self.percentage() / 100.0).round() as usize;
        let empty = self.width as usize - filled;
        let color = self.color();

        let mut result = String::new();

        result.push_str(&self.name);
        result.push_str(": ");

        result.push('[');
        result.push_str(&color.to_ansi_fg());
        for _ in 0..filled {
            result.push('\u{2588}');
        }
        result.push_str(&Color::DARK_GRAY.to_ansi_fg());
        for _ in 0..empty {
            result.push('\u{2591}');
        }
        result.push_str("\x1b[0m");
        result.push(']');

        if self.ready {
            result.push_str(&format!(" {}", Color::new(0, 255, 0).to_ansi_fg()));
            result.push_str("READY");
            result.push_str("\x1b[0m");
        } else {
            result.push_str(&format!(" {:.1}s", self.remaining));
        }

        result
    }
}

// ============================================================================
// Cast Bar
// ============================================================================

/// Cast bar for abilities being channeled or cast
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastBar {
    /// Ability being cast
    pub ability_name: String,
    /// Current cast progress
    pub progress: f32,
    /// Total cast time
    pub total: f32,
    /// Whether cast can be interrupted
    pub interruptible: bool,
    /// Bar width
    pub width: u16,
}

impl CastBar {
    /// Create a new cast bar
    pub fn new(ability_name: impl Into<String>, progress: f32, total: f32) -> Self {
        Self {
            ability_name: ability_name.into(),
            progress: progress.max(0.0),
            total: total.max(0.1),
            interruptible: true,
            width: 20,
        }
    }

    /// Set interruptible state
    pub fn with_interruptible(mut self, interruptible: bool) -> Self {
        self.interruptible = interruptible;
        self
    }

    /// Get percentage complete
    pub fn percentage(&self) -> f32 {
        (self.progress / self.total * 100.0).clamp(0.0, 100.0)
    }

    /// Check if cast is complete
    pub fn is_complete(&self) -> bool {
        self.progress >= self.total
    }

    /// Get the color
    pub fn color(&self) -> Color {
        if self.is_complete() {
            Color::new(0, 255, 0) // Green when complete
        } else if !self.interruptible {
            Color::CAST_COLOR // Gold for uninterruptible
        } else {
            // Regular cast: blend from orange to gold
            Color::ENERGY_LOW.lerp(&Color::CAST_COLOR, self.percentage() / 100.0)
        }
    }

    /// Render to ASCII
    pub fn render_ascii(&self) -> String {
        let filled = (self.width as f32 * self.percentage() / 100.0).round() as usize;
        let empty = self.width as usize - filled;
        let color = self.color();

        let mut result = String::new();

        // Casting indicator
        if !self.is_complete() {
            result.push_str("Casting: ");
        }
        result.push_str(&self.ability_name);

        if !self.interruptible {
            result.push_str(" [UNINTERRUPTIBLE]");
        }

        result.push('\n');

        result.push_str("<");
        result.push_str(&color.to_ansi_fg());
        for _ in 0..filled {
            result.push('=');
        }
        result.push_str(&Color::DARK_GRAY.to_ansi_fg());
        for _ in 0..empty {
            result.push('-');
        }
        result.push_str("\x1b[0m");
        result.push_str(&format!("> {:.1}/{:.1}s ({:.0}%)",
            self.progress, self.total, self.percentage()));

        result
    }
}

// ============================================================================
// Composite Bar (Multiple segments)
// ============================================================================

/// A bar with multiple segments (e.g., combo meter, multi-resource)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeBar {
    /// Bar segments
    pub segments: Vec<BarSegment>,
    /// Bar width
    pub width: u16,
    /// Label
    pub label: Option<String>,
}

/// A single segment of a composite bar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarSegment {
    /// Segment value (0.0 - 1.0)
    pub value: f32,
    /// Segment color
    pub color: Color,
    /// Segment label
    pub label: Option<String>,
}

impl CompositeBar {
    /// Create a new composite bar
    pub fn new(segments: Vec<BarSegment>, width: u16) -> Self {
        Self {
            segments,
            width,
            label: None,
        }
    }

    /// Set label
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Render to ASCII
    pub fn render_ascii(&self) -> String {
        let mut result = String::new();

        if let Some(ref label) = self.label {
            result.push_str(label);
            result.push_str(": ");
        }

        result.push('[');

        let total_chars = self.width as usize;
        let mut used = 0;

        for segment in &self.segments {
            let chars = ((segment.value * total_chars as f32).round() as usize)
                .min(total_chars - used);
            result.push_str(&segment.color.to_ansi_fg());
            for _ in 0..chars {
                result.push('\u{2588}');
            }
            used += chars;
        }

        // Fill remaining with empty
        result.push_str(&Color::DARK_GRAY.to_ansi_fg());
        for _ in used..total_chars {
            result.push('\u{2591}');
        }

        result.push_str("\x1b[0m");
        result.push(']');

        result
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Render a health bar with default settings
pub fn render_health_bar(current: i32, maximum: i32, width: u16) -> String {
    StatusBar::new(current, maximum, BarType::Health)
        .with_width(width)
        .with_text(true)
        .render_ascii()
}

/// Render a mana bar with default settings
pub fn render_mana_bar(current: i32, maximum: i32, width: u16) -> String {
    StatusBar::new(current, maximum, BarType::Mana)
        .with_width(width)
        .with_text(true)
        .render_ascii()
}

/// Render a stamina bar with default settings
pub fn render_stamina_bar(current: i32, maximum: i32, width: u16) -> String {
    StatusBar::new(current, maximum, BarType::Stamina)
        .with_width(width)
        .with_text(true)
        .render_ascii()
}

/// Render an XP bar with default settings
pub fn render_xp_bar(current: i32, maximum: i32, width: u16) -> String {
    StatusBar::new(current, maximum, BarType::Experience)
        .with_width(width)
        .with_text(true)
        .with_percentage(true)
        .with_label("XP")
        .render_ascii()
}

/// Render an energy bar with default settings
pub fn render_energy_bar(current: i32, maximum: i32, width: u16) -> String {
    StatusBar::new(current, maximum, BarType::Energy)
        .with_width(width)
        .with_text(true)
        .render_ascii()
}

/// Render a rage bar with default settings
pub fn render_rage_bar(current: i32, maximum: i32, width: u16) -> String {
    StatusBar::new(current, maximum, BarType::Rage)
        .with_width(width)
        .with_text(true)
        .render_ascii()
}

/// Render a boss health bar
pub fn render_boss_bar(name: &str, current: i32, maximum: i32, enraged: bool) -> String {
    BossHealthBar::new(name, current, maximum)
        .with_enraged(enraged)
        .render_ascii()
}

/// Render a boss health bar with phases
pub fn render_boss_bar_with_phases(
    name: &str,
    current: i32,
    maximum: i32,
    total_phases: u8,
    current_phase: u8,
    phase_name: Option<&str>,
    enraged: bool,
) -> String {
    let mut bar = BossHealthBar::new(name, current, maximum)
        .with_segments(total_phases, current_phase)
        .with_enraged(enraged);

    if let Some(phase) = phase_name {
        bar = bar.with_phase(phase);
    }

    bar.render_ascii()
}

/// Render a cooldown bar
pub fn render_cooldown(name: &str, remaining: f32, total: f32) -> String {
    CooldownBar::new(name, remaining, total).render_ascii()
}

/// Render a cast bar
pub fn render_cast_bar(ability: &str, progress: f32, total: f32, interruptible: bool) -> String {
    CastBar::new(ability, progress, total)
        .with_interruptible(interruptible)
        .render_ascii()
}

/// Render a buff duration bar
pub fn render_buff_bar(name: &str, remaining: f32, total: f32, stacks: Option<u8>) -> String {
    let mut bar = DurationBar::new(name, remaining, total, true);
    if let Some(s) = stacks {
        bar = bar.with_stacks(s);
    }
    bar.render_ascii()
}

/// Render a debuff duration bar
pub fn render_debuff_bar(name: &str, remaining: f32, total: f32, stacks: Option<u8>) -> String {
    let mut bar = DurationBar::new(name, remaining, total, false);
    if let Some(s) = stacks {
        bar = bar.with_stacks(s);
    }
    bar.render_ascii()
}

/// Render a mini health bar (compact, no text)
pub fn render_mini_bar(current: i32, maximum: i32, width: u16) -> String {
    StatusBar::new(current, maximum, BarType::Health)
        .with_width(width)
        .with_text(false)
        .with_caps(false)
        .render_ascii()
}

/// Render multiple resource bars as a compact display
pub fn render_resource_panel(
    health: (i32, i32),
    mana: (i32, i32),
    stamina: Option<(i32, i32)>,
    width: u16,
) -> String {
    let mut result = String::new();

    result.push_str("HP: ");
    result.push_str(&StatusBar::new(health.0, health.1, BarType::Health)
        .with_width(width)
        .with_text(true)
        .with_caps(false)
        .render_ascii());
    result.push('\n');

    result.push_str("MP: ");
    result.push_str(&StatusBar::new(mana.0, mana.1, BarType::Mana)
        .with_width(width)
        .with_text(true)
        .with_caps(false)
        .render_ascii());

    if let Some((cur, max)) = stamina {
        result.push('\n');
        result.push_str("SP: ");
        result.push_str(&StatusBar::new(cur, max, BarType::Stamina)
            .with_width(width)
            .with_text(true)
            .with_caps(false)
            .render_ascii());
    }

    result
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_bar_creation() {
        let bar = StatusBar::new(75, 100, BarType::Health);
        assert_eq!(bar.current, 75);
        assert_eq!(bar.maximum, 100);
        assert!((bar.percentage() - 75.0).abs() < 0.01);
    }

    #[test]
    fn test_status_bar_percentage() {
        let bar = StatusBar::new(50, 100, BarType::Health);
        assert!((bar.percentage() - 50.0).abs() < 0.01);

        let bar = StatusBar::new(0, 100, BarType::Health);
        assert!((bar.percentage() - 0.0).abs() < 0.01);

        let bar = StatusBar::new(100, 100, BarType::Health);
        assert!((bar.percentage() - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_status_bar_clamping() {
        let bar = StatusBar::new(-10, 100, BarType::Health);
        assert_eq!(bar.current, 0);

        let bar = StatusBar::new(150, 100, BarType::Health);
        assert_eq!(bar.current, 150); // Not clamped on creation, only on set
    }

    #[test]
    fn test_color_interpolation() {
        let c1 = Color::new(0, 0, 0);
        let c2 = Color::new(255, 255, 255);

        let mid = c1.lerp(&c2, 0.5);
        assert_eq!(mid.r, 127);
        assert_eq!(mid.g, 127);
        assert_eq!(mid.b, 127);
    }

    #[test]
    fn test_health_gradient() {
        let color_full = BarType::health_gradient(100.0);
        assert_eq!(color_full.r, Color::HEALTH_FULL.r);

        let color_critical = BarType::health_gradient(5.0);
        assert_eq!(color_critical.r, Color::HEALTH_CRITICAL.r);
    }

    #[test]
    fn test_bar_styles() {
        assert_eq!(BarStyle::Block.filled_char(), '\u{2588}');
        assert_eq!(BarStyle::Dots.filled_char(), '\u{25CF}');
        assert_eq!(BarStyle::Arrows.filled_char(), '>');
    }

    #[test]
    fn test_boss_bar() {
        let bar = BossHealthBar::new("Dragon", 5000, 10000);
        assert!((bar.percentage() - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_cooldown_bar() {
        let bar = CooldownBar::new("Fireball", 0.0, 5.0);
        assert!(bar.ready);

        let bar = CooldownBar::new("Fireball", 2.5, 5.0);
        assert!(!bar.ready);
        assert!((bar.percentage() - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_cast_bar() {
        let bar = CastBar::new("Heal", 1.5, 3.0);
        assert!((bar.percentage() - 50.0).abs() < 0.01);
        assert!(!bar.is_complete());

        let bar = CastBar::new("Heal", 3.0, 3.0);
        assert!(bar.is_complete());
    }

    #[test]
    fn test_render_plain() {
        let bar = StatusBar::new(50, 100, BarType::Health)
            .with_width(10)
            .with_text(true)
            .with_caps(true);

        let rendered = bar.render_plain();
        assert!(rendered.contains('['));
        assert!(rendered.contains(']'));
        assert!(rendered.contains("50/100"));
    }

    #[test]
    fn test_helper_functions() {
        let health = render_health_bar(75, 100, 10);
        assert!(health.contains("75/100"));

        let mana = render_mana_bar(30, 50, 10);
        assert!(mana.contains("30/50"));
    }
}
