//! Weather and environment system
//!
//! This module provides a dynamic weather system that affects gameplay through
//! visibility changes and combat modifiers. Different weather types create
//! atmospheric effects and tactical considerations.
//!
//! # Weather Types
//!
//! - **Clear**: Normal conditions with no modifiers
//! - **Rain**: Reduces visibility, extinguishes fire effects, slippery surfaces
//! - **Fog**: Significantly reduces visibility, enemies harder to detect
//! - **Sandstorm**: Reduces visibility and accuracy, causes periodic damage
//!
//! # Example
//!
//! ```rust,no_run
//! use shadowcrypt_core::weather::{WeatherSystem, WeatherType};
//!
//! let mut weather = WeatherSystem::new();
//! weather.set_weather(WeatherType::Fog);
//!
//! // Get visibility modifier (0.0 - 1.0)
//! let vis_mod = weather.visibility_modifier();
//!
//! // Get combat accuracy modifier
//! let acc_mod = weather.accuracy_modifier();
//! ```

use rand::prelude::*;
use serde::{Deserialize, Serialize};

/// The types of weather that can occur in the dungeon
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum WeatherType {
    /// Normal conditions - no effects
    Clear,
    /// Rain - reduces visibility, extinguishes fire, slippery
    Rain,
    /// Dense fog - greatly reduces visibility
    Fog,
    /// Sandstorm - reduces visibility and accuracy, causes damage
    Sandstorm,
}

impl WeatherType {
    /// Returns the display name of the weather
    pub fn name(&self) -> &'static str {
        match self {
            Self::Clear => "Clear",
            Self::Rain => "Rain",
            Self::Fog => "Fog",
            Self::Sandstorm => "Sandstorm",
        }
    }

    /// Returns a description of the weather effects
    pub fn description(&self) -> &'static str {
        match self {
            Self::Clear => "The air is clear and still.",
            Self::Rain => "Heavy rain reduces visibility and extinguishes flames.",
            Self::Fog => "Dense fog shrouds the area, limiting sight.",
            Self::Sandstorm => "Blinding sand tears at exposed flesh.",
        }
    }

    /// Returns a color index for UI display
    /// 0=DarkGrey, 1=Grey, 2=White, 3=Red, 7=Blue, 9=Cyan, 11=Yellow
    pub fn color_index(&self) -> u8 {
        match self {
            Self::Clear => 2,      // White
            Self::Rain => 7,       // Blue
            Self::Fog => 1,        // Grey
            Self::Sandstorm => 11, // Yellow
        }
    }

    /// Returns the glyph character for weather particles (for visual effects)
    pub fn particle_glyph(&self) -> Option<char> {
        match self {
            Self::Clear => None,
            Self::Rain => Some('|'),
            Self::Fog => Some('.'),
            Self::Sandstorm => Some('*'),
        }
    }

    /// Returns the visibility modifier (1.0 = full visibility, 0.0 = no visibility)
    pub fn visibility_modifier(&self) -> f32 {
        match self {
            Self::Clear => 1.0,
            Self::Rain => 0.7,
            Self::Fog => 0.4,
            Self::Sandstorm => 0.5,
        }
    }

    /// Returns the accuracy modifier for ranged attacks and skills
    pub fn accuracy_modifier(&self) -> f32 {
        match self {
            Self::Clear => 1.0,
            Self::Rain => 0.9,
            Self::Fog => 0.7,
            Self::Sandstorm => 0.6,
        }
    }

    /// Returns the movement speed modifier
    pub fn movement_modifier(&self) -> f32 {
        match self {
            Self::Clear => 1.0,
            Self::Rain => 0.9,      // Slippery surfaces
            Self::Fog => 1.0,       // No movement penalty
            Self::Sandstorm => 0.8, // Fighting against wind
        }
    }

    /// Returns damage per turn from environmental effects (0 = no damage)
    pub fn environmental_damage(&self) -> i32 {
        match self {
            Self::Clear => 0,
            Self::Rain => 0,
            Self::Fog => 0,
            Self::Sandstorm => 1, // Sand abrasion
        }
    }

    /// Returns whether this weather extinguishes fire effects
    pub fn extinguishes_fire(&self) -> bool {
        matches!(self, Self::Rain)
    }

    /// Returns whether this weather enhances lightning/electric attacks
    pub fn conducts_electricity(&self) -> bool {
        matches!(self, Self::Rain)
    }

    /// Returns the electric damage multiplier
    pub fn electric_damage_modifier(&self) -> f32 {
        match self {
            Self::Rain => 1.5, // Water conducts electricity
            _ => 1.0,
        }
    }

    /// Returns the fire damage modifier
    pub fn fire_damage_modifier(&self) -> f32 {
        match self {
            Self::Rain => 0.5,       // Rain weakens fire
            Self::Sandstorm => 0.8,  // Sand smothers flames slightly
            _ => 1.0,
        }
    }

    /// Returns the ice damage modifier
    pub fn ice_damage_modifier(&self) -> f32 {
        match self {
            Self::Rain => 1.2,  // Wet targets freeze easier
            Self::Fog => 1.1,   // Moisture in air
            _ => 1.0,
        }
    }
}

impl Default for WeatherType {
    fn default() -> Self {
        Self::Clear
    }
}

/// Intensity level of weather effects
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum WeatherIntensity {
    /// Light weather - reduced effects
    Light,
    /// Normal weather intensity
    Normal,
    /// Heavy/severe weather - enhanced effects
    Heavy,
}

impl WeatherIntensity {
    /// Returns a multiplier for weather effects based on intensity
    pub fn effect_multiplier(&self) -> f32 {
        match self {
            Self::Light => 0.5,
            Self::Normal => 1.0,
            Self::Heavy => 1.5,
        }
    }

    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Light => "Light",
            Self::Normal => "Moderate",
            Self::Heavy => "Heavy",
        }
    }
}

impl Default for WeatherIntensity {
    fn default() -> Self {
        Self::Normal
    }
}

/// The weather system that manages weather state and transitions
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WeatherSystem {
    /// Current weather type
    pub current_weather: WeatherType,
    /// Weather intensity
    pub intensity: WeatherIntensity,
    /// Turns remaining until weather might change
    pub duration_remaining: u32,
    /// Minimum duration for weather effects (in turns)
    pub min_duration: u32,
    /// Maximum duration for weather effects (in turns)
    pub max_duration: u32,
    /// Chance for weather to change each turn after min_duration (0.0 - 1.0)
    pub change_chance: f32,
    /// Whether weather is enabled
    pub enabled: bool,
}

impl WeatherSystem {
    /// Create a new weather system with default settings
    pub fn new() -> Self {
        Self {
            current_weather: WeatherType::Clear,
            intensity: WeatherIntensity::Normal,
            duration_remaining: 50,
            min_duration: 20,
            max_duration: 100,
            change_chance: 0.05,
            enabled: true,
        }
    }

    /// Set the current weather type
    pub fn set_weather(&mut self, weather: WeatherType) {
        self.current_weather = weather;
        self.duration_remaining = self.max_duration;
    }

    /// Set the weather with a specific intensity
    pub fn set_weather_with_intensity(&mut self, weather: WeatherType, intensity: WeatherIntensity) {
        self.current_weather = weather;
        self.intensity = intensity;
        self.duration_remaining = self.max_duration;
    }

    /// Update the weather system (call each turn)
    pub fn update(&mut self, rng: &mut impl Rng) -> Option<WeatherChangeEvent> {
        if !self.enabled {
            return None;
        }

        if self.duration_remaining > 0 {
            self.duration_remaining -= 1;
        }

        // Check for weather change after minimum duration
        if self.duration_remaining < self.min_duration && rng.gen::<f32>() < self.change_chance {
            let old_weather = self.current_weather;
            self.transition_weather(rng);

            if self.current_weather != old_weather {
                return Some(WeatherChangeEvent {
                    from: old_weather,
                    to: self.current_weather,
                    intensity: self.intensity,
                });
            }
        }

        None
    }

    /// Transition to a new random weather type
    fn transition_weather(&mut self, rng: &mut impl Rng) {
        // Weight probabilities based on current weather
        let weights = match self.current_weather {
            WeatherType::Clear => [50, 20, 20, 10],      // Likely to stay clear
            WeatherType::Rain => [30, 30, 30, 10],       // Even chance
            WeatherType::Fog => [40, 20, 20, 20],        // Often clears
            WeatherType::Sandstorm => [40, 10, 20, 30],  // Often continues or clears
        };

        let total: i32 = weights.iter().sum();
        let roll = rng.gen_range(0..total);

        let mut cumulative = 0;
        let weather_types = [
            WeatherType::Clear,
            WeatherType::Rain,
            WeatherType::Fog,
            WeatherType::Sandstorm,
        ];

        for (i, &weight) in weights.iter().enumerate() {
            cumulative += weight;
            if roll < cumulative {
                self.current_weather = weather_types[i];
                break;
            }
        }

        // Randomize intensity
        self.intensity = match rng.gen_range(0..10) {
            0..=2 => WeatherIntensity::Light,
            3..=7 => WeatherIntensity::Normal,
            _ => WeatherIntensity::Heavy,
        };

        // Reset duration
        self.duration_remaining = rng.gen_range(self.min_duration..=self.max_duration);
    }

    /// Force a specific weather for a dungeon theme
    pub fn set_for_theme(&mut self, theme: &str, rng: &mut impl Rng) {
        match theme {
            "Frozen Caverns" | "Ice Cavern" => {
                if rng.gen_bool(0.4) {
                    self.set_weather(WeatherType::Fog);
                }
            }
            "Ancient Ruins" | "Desert" => {
                if rng.gen_bool(0.5) {
                    self.set_weather(WeatherType::Sandstorm);
                }
            }
            "Twisted Caves" | "Cave" => {
                if rng.gen_bool(0.3) {
                    self.set_weather(WeatherType::Fog);
                }
            }
            _ => {
                // Default: small chance of rain
                if rng.gen_bool(0.2) {
                    self.set_weather(WeatherType::Rain);
                }
            }
        }
    }

    /// Get the current visibility modifier (accounting for intensity)
    pub fn visibility_modifier(&self) -> f32 {
        if !self.enabled {
            return 1.0;
        }
        let base = self.current_weather.visibility_modifier();
        let intensity_factor = match self.intensity {
            WeatherIntensity::Light => 0.5,
            WeatherIntensity::Normal => 1.0,
            WeatherIntensity::Heavy => 1.5,
        };
        // Interpolate between 1.0 (no effect) and base modifier
        1.0 - ((1.0 - base) * intensity_factor).min(0.8) // Cap at 80% visibility reduction
    }

    /// Get the current accuracy modifier
    pub fn accuracy_modifier(&self) -> f32 {
        if !self.enabled {
            return 1.0;
        }
        let base = self.current_weather.accuracy_modifier();
        let intensity_factor = self.intensity.effect_multiplier();
        1.0 - ((1.0 - base) * intensity_factor).min(0.6) // Cap at 60% accuracy reduction
    }

    /// Get the current movement modifier
    pub fn movement_modifier(&self) -> f32 {
        if !self.enabled {
            return 1.0;
        }
        let base = self.current_weather.movement_modifier();
        let intensity_factor = self.intensity.effect_multiplier();
        1.0 - ((1.0 - base) * intensity_factor).min(0.4) // Cap at 40% movement reduction
    }

    /// Get environmental damage per turn
    pub fn environmental_damage(&self) -> i32 {
        if !self.enabled {
            return 0;
        }
        let base = self.current_weather.environmental_damage();
        match self.intensity {
            WeatherIntensity::Light => base / 2,
            WeatherIntensity::Normal => base,
            WeatherIntensity::Heavy => base * 2,
        }
    }

    /// Calculate modified view radius based on weather
    pub fn modified_view_radius(&self, base_radius: i32) -> i32 {
        let modifier = self.visibility_modifier();
        ((base_radius as f32) * modifier).max(2.0) as i32
    }

    /// Check if fire effects should be extinguished
    pub fn should_extinguish_fire(&self) -> bool {
        self.enabled && self.current_weather.extinguishes_fire()
    }

    /// Get the damage modifier for a specific damage type
    pub fn damage_modifier(&self, damage_type: DamageType) -> f32 {
        if !self.enabled {
            return 1.0;
        }
        match damage_type {
            DamageType::Fire => self.current_weather.fire_damage_modifier(),
            DamageType::Ice => self.current_weather.ice_damage_modifier(),
            DamageType::Lightning => self.current_weather.electric_damage_modifier(),
            DamageType::Physical => 1.0,
            DamageType::Poison => 1.0,
            DamageType::Holy => 1.0,
            DamageType::Dark => 1.0,
        }
    }

    /// Get a status message describing current weather
    pub fn status_message(&self) -> String {
        if !self.enabled || self.current_weather == WeatherType::Clear {
            return String::new();
        }
        format!(
            "{} {} - {}",
            self.intensity.name(),
            self.current_weather.name(),
            self.current_weather.description()
        )
    }

    /// Check if weather should affect a specific tile coordinate
    /// (Some areas might be sheltered)
    pub fn affects_position(&self, _x: usize, _y: usize, is_indoors: bool) -> bool {
        if !self.enabled {
            return false;
        }
        // Indoor areas are sheltered from weather
        if is_indoors {
            return false;
        }
        true
    }
}

impl Default for WeatherSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Event fired when weather changes
#[derive(Clone, Debug)]
pub struct WeatherChangeEvent {
    /// Previous weather type
    pub from: WeatherType,
    /// New weather type
    pub to: WeatherType,
    /// Intensity of new weather
    pub intensity: WeatherIntensity,
}

impl WeatherChangeEvent {
    /// Get a message describing the weather change
    pub fn message(&self) -> String {
        if self.to == WeatherType::Clear {
            match self.from {
                WeatherType::Rain => "The rain stops and the skies clear.".to_string(),
                WeatherType::Fog => "The fog lifts, revealing your surroundings.".to_string(),
                WeatherType::Sandstorm => "The sandstorm subsides.".to_string(),
                WeatherType::Clear => String::new(),
            }
        } else {
            format!(
                "{} {} begins! {}",
                self.intensity.name(),
                self.to.name().to_lowercase(),
                self.to.description()
            )
        }
    }

    /// Get a color index for the message
    pub fn color_index(&self) -> u8 {
        self.to.color_index()
    }
}

/// Types of damage for weather interaction
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DamageType {
    Physical,
    Fire,
    Ice,
    Lightning,
    Poison,
    Holy,
    Dark,
}

/// Weather effects that can be applied to combat calculations
#[derive(Clone, Debug)]
pub struct WeatherCombatModifiers {
    /// Accuracy modifier for attacks
    pub accuracy: f32,
    /// Damage modifier by type
    pub fire_damage: f32,
    pub ice_damage: f32,
    pub lightning_damage: f32,
    /// Whether burn effects are suppressed
    pub suppress_burn: bool,
    /// Environmental damage to apply
    pub env_damage: i32,
}

impl WeatherCombatModifiers {
    /// Calculate combat modifiers from a weather system
    pub fn from_weather(weather: &WeatherSystem) -> Self {
        Self {
            accuracy: weather.accuracy_modifier(),
            fire_damage: weather.damage_modifier(DamageType::Fire),
            ice_damage: weather.damage_modifier(DamageType::Ice),
            lightning_damage: weather.damage_modifier(DamageType::Lightning),
            suppress_burn: weather.should_extinguish_fire(),
            env_damage: weather.environmental_damage(),
        }
    }

    /// Apply accuracy modifier to a hit chance
    pub fn modify_accuracy(&self, base_accuracy: f32) -> f32 {
        (base_accuracy * self.accuracy).min(1.0)
    }

    /// Apply damage modifier based on damage type
    pub fn modify_damage(&self, base_damage: i32, damage_type: DamageType) -> i32 {
        let modifier = match damage_type {
            DamageType::Fire => self.fire_damage,
            DamageType::Ice => self.ice_damage,
            DamageType::Lightning => self.lightning_damage,
            _ => 1.0,
        };
        ((base_damage as f32) * modifier) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_visibility() {
        let mut weather = WeatherSystem::new();

        weather.set_weather(WeatherType::Clear);
        assert_eq!(weather.visibility_modifier(), 1.0);

        weather.set_weather(WeatherType::Fog);
        assert!(weather.visibility_modifier() < 1.0);
        assert!(weather.visibility_modifier() > 0.0);
    }

    #[test]
    fn test_weather_intensity() {
        let mut weather = WeatherSystem::new();

        weather.set_weather_with_intensity(WeatherType::Rain, WeatherIntensity::Light);
        let light_vis = weather.visibility_modifier();

        weather.set_weather_with_intensity(WeatherType::Rain, WeatherIntensity::Heavy);
        let heavy_vis = weather.visibility_modifier();

        assert!(heavy_vis < light_vis);
    }

    #[test]
    fn test_fire_extinguish() {
        let mut weather = WeatherSystem::new();

        weather.set_weather(WeatherType::Clear);
        assert!(!weather.should_extinguish_fire());

        weather.set_weather(WeatherType::Rain);
        assert!(weather.should_extinguish_fire());
    }

    #[test]
    fn test_damage_modifiers() {
        let mut weather = WeatherSystem::new();

        weather.set_weather(WeatherType::Rain);
        assert!(weather.damage_modifier(DamageType::Fire) < 1.0);
        assert!(weather.damage_modifier(DamageType::Lightning) > 1.0);
    }

    #[test]
    fn test_view_radius_modification() {
        let mut weather = WeatherSystem::new();
        let base_radius = 10;

        weather.set_weather(WeatherType::Clear);
        assert_eq!(weather.modified_view_radius(base_radius), 10);

        weather.set_weather_with_intensity(WeatherType::Fog, WeatherIntensity::Heavy);
        let fog_radius = weather.modified_view_radius(base_radius);
        assert!(fog_radius < base_radius);
        assert!(fog_radius >= 2); // Minimum visibility
    }

    #[test]
    fn test_combat_modifiers() {
        let mut weather = WeatherSystem::new();
        weather.set_weather(WeatherType::Rain);

        let modifiers = WeatherCombatModifiers::from_weather(&weather);

        assert!(modifiers.suppress_burn);
        assert!(modifiers.fire_damage < 1.0);
        assert!(modifiers.lightning_damage > 1.0);
    }

    #[test]
    fn test_sandstorm_damage() {
        let mut weather = WeatherSystem::new();

        weather.set_weather_with_intensity(WeatherType::Sandstorm, WeatherIntensity::Heavy);
        assert!(weather.environmental_damage() > 0);

        weather.set_weather(WeatherType::Clear);
        assert_eq!(weather.environmental_damage(), 0);
    }
}
