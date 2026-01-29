//! Weather and environment system
//!
//! This module provides a dynamic weather system that affects gameplay through
//! visibility changes, combat modifiers, elemental damage changes, and special spawns.
//! Different weather types create atmospheric effects and tactical considerations.
//!
//! # Weather Types
//!
//! The system includes over 30 weather types:
//! - **Natural Weather**: Clear, Cloudy, Overcast, Rain, Snow, Fog, Storms
//! - **Severe Weather**: Hurricane, Blizzard, Tornado, Earthquake
//! - **Celestial Events**: Solar Eclipse, Lunar Eclipse, Aurora, Meteor Shower
//! - **Magical Weather**: Mana Storm, Spirit Rain, Blood Moon, Void Breach
//!
//! # Seasonal System
//!
//! Weather patterns change based on the current season:
//! - **Spring**: Frequent rain, mild temperatures
//! - **Summer**: Clear skies, heat waves
//! - **Autumn**: Fog, storms, falling leaves
//! - **Winter**: Snow, blizzards, cold
//!
//! # Weather Manipulation
//!
//! Players can influence weather through:
//! - Weather spells (Rain Dance, Summon Storm, Clear Skies)
//! - Artifacts (Storm Orb, Sun Stone, Frost Crown)
//! - Rituals (Weather Binding, Elemental Convergence)
//!
//! # Example
//!
//! ```rust,no_run
//! use shadowcrypt_core::weather::{WeatherSystem, WeatherType, Season};
//!
//! let mut weather = WeatherSystem::new();
//! weather.set_season(Season::Winter);
//! weather.set_weather(WeatherType::HeavySnow);
//!
//! // Get visibility modifier (0.0 - 1.0)
//! let vis_mod = weather.visibility_modifier();
//!
//! // Get combat accuracy modifier
//! let acc_mod = weather.accuracy_modifier();
//!
//! // Check for special spawns
//! let spawns = weather.get_special_spawns();
//!
//! // Get weather forecast
//! let forecast = weather.forecast(3);
//! ```

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Weather Types
// ============================================================================

/// The types of weather that can occur in the game world
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum WeatherType {
    // === Standard Weather ===
    /// Normal conditions - no effects
    Clear,
    /// Partially cloudy - minimal effects
    Cloudy,
    /// Fully overcast skies
    Overcast,

    // === Rain Types ===
    /// Light rain - minor visibility reduction
    LightRain,
    /// Heavy rain - significant visibility and movement effects
    HeavyRain,
    /// Thunderstorm - lightning strikes, heavy rain
    Thunderstorm,
    /// Hurricane - extreme wind and rain
    Hurricane,

    // === Snow Types ===
    /// Light snow - minor cold effects
    LightSnow,
    /// Heavy snow - significant movement penalty
    HeavySnow,
    /// Blizzard - extreme cold and visibility loss
    Blizzard,

    // === Fog Types ===
    /// Light mist - slight visibility reduction
    Mist,
    /// Standard fog - moderate visibility reduction
    Fog,
    /// Dense fog - severe visibility reduction
    DenseFog,

    // === Desert Weather ===
    /// Dust storm - visibility and accuracy penalty
    DustStorm,
    /// Sandstorm - damage and severe penalties
    Sandstorm,

    // === Ice Weather ===
    /// Hail - periodic damage, cold effects
    Hail,
    /// Sleet - ice and rain mix
    Sleet,

    // === Catastrophic Weather ===
    /// Tornado - extreme danger, movement chaos
    Tornado,
    /// Earthquake - ground instability, structural damage
    Earthquake,

    // === Celestial Events ===
    /// Solar eclipse - darkness during day
    SolarEclipse,
    /// Lunar eclipse - empowers dark creatures
    LunarEclipse,
    /// Aurora borealis - magical energy boost
    Aurora,
    /// Meteor shower - falling stars, rare materials
    MeteorShower,

    // === Magical Weather ===
    /// Mana storm - wild magic effects
    ManaStorm,
    /// Spirit rain - ghostly precipitation
    SpiritRain,
    /// Blood moon - empowers undead and dark magic
    BloodMoon,
    /// Void breach - reality tears, void creatures
    VoidBreach,
    /// Arcane wind - magic amplification
    ArcaneWind,
    /// Crystal rain - magical crystals fall
    CrystalRain,
    /// Phoenix fire - healing flames rain down
    PhoenixFire,
    /// Astral storm - planar instability
    AstralStorm,
}

impl WeatherType {
    /// Returns all weather types
    pub fn all() -> &'static [WeatherType] {
        &[
            Self::Clear,
            Self::Cloudy,
            Self::Overcast,
            Self::LightRain,
            Self::HeavyRain,
            Self::Thunderstorm,
            Self::Hurricane,
            Self::LightSnow,
            Self::HeavySnow,
            Self::Blizzard,
            Self::Mist,
            Self::Fog,
            Self::DenseFog,
            Self::DustStorm,
            Self::Sandstorm,
            Self::Hail,
            Self::Sleet,
            Self::Tornado,
            Self::Earthquake,
            Self::SolarEclipse,
            Self::LunarEclipse,
            Self::Aurora,
            Self::MeteorShower,
            Self::ManaStorm,
            Self::SpiritRain,
            Self::BloodMoon,
            Self::VoidBreach,
            Self::ArcaneWind,
            Self::CrystalRain,
            Self::PhoenixFire,
            Self::AstralStorm,
        ]
    }

    /// Returns the display name of the weather
    pub fn name(&self) -> &'static str {
        match self {
            Self::Clear => "Clear",
            Self::Cloudy => "Cloudy",
            Self::Overcast => "Overcast",
            Self::LightRain => "Light Rain",
            Self::HeavyRain => "Heavy Rain",
            Self::Thunderstorm => "Thunderstorm",
            Self::Hurricane => "Hurricane",
            Self::LightSnow => "Light Snow",
            Self::HeavySnow => "Heavy Snow",
            Self::Blizzard => "Blizzard",
            Self::Mist => "Mist",
            Self::Fog => "Fog",
            Self::DenseFog => "Dense Fog",
            Self::DustStorm => "Dust Storm",
            Self::Sandstorm => "Sandstorm",
            Self::Hail => "Hail",
            Self::Sleet => "Sleet",
            Self::Tornado => "Tornado",
            Self::Earthquake => "Earthquake",
            Self::SolarEclipse => "Solar Eclipse",
            Self::LunarEclipse => "Lunar Eclipse",
            Self::Aurora => "Aurora",
            Self::MeteorShower => "Meteor Shower",
            Self::ManaStorm => "Mana Storm",
            Self::SpiritRain => "Spirit Rain",
            Self::BloodMoon => "Blood Moon",
            Self::VoidBreach => "Void Breach",
            Self::ArcaneWind => "Arcane Wind",
            Self::CrystalRain => "Crystal Rain",
            Self::PhoenixFire => "Phoenix Fire",
            Self::AstralStorm => "Astral Storm",
        }
    }

    /// Returns a description of the weather effects
    pub fn description(&self) -> &'static str {
        match self {
            Self::Clear => "The air is clear and still.",
            Self::Cloudy => "Clouds drift lazily across the sky.",
            Self::Overcast => "A thick layer of clouds blocks the sun.",
            Self::LightRain => "A gentle rain falls softly.",
            Self::HeavyRain => "Heavy rain pounds the ground relentlessly.",
            Self::Thunderstorm => "Lightning splits the sky as thunder roars.",
            Self::Hurricane => "Catastrophic winds tear at everything in their path.",
            Self::LightSnow => "Delicate snowflakes drift down peacefully.",
            Self::HeavySnow => "Thick snow blankets the land.",
            Self::Blizzard => "A raging blizzard reduces visibility to nothing.",
            Self::Mist => "A light mist hangs in the air.",
            Self::Fog => "Dense fog shrouds the area, limiting sight.",
            Self::DenseFog => "An impenetrable wall of fog surrounds you.",
            Self::DustStorm => "Choking dust fills the air.",
            Self::Sandstorm => "Blinding sand tears at exposed flesh.",
            Self::Hail => "Ice pellets rain down from above.",
            Self::Sleet => "Freezing rain coats everything in ice.",
            Self::Tornado => "A massive funnel cloud wreaks destruction.",
            Self::Earthquake => "The ground trembles and cracks beneath your feet.",
            Self::SolarEclipse => "The sun is swallowed by darkness.",
            Self::LunarEclipse => "The blood-red moon empowers dark forces.",
            Self::Aurora => "Ethereal lights dance across the sky.",
            Self::MeteorShower => "Brilliant streaks of fire cross the heavens.",
            Self::ManaStorm => "Raw magical energy crackles through the air.",
            Self::SpiritRain => "Ghostly droplets phase through solid matter.",
            Self::BloodMoon => "The crimson moon bathes everything in red light.",
            Self::VoidBreach => "Reality tears open, revealing the void beyond.",
            Self::ArcaneWind => "Winds carrying pure arcane energy sweep through.",
            Self::CrystalRain => "Magical crystals fall from the sky.",
            Self::PhoenixFire => "Golden flames of rebirth rain from above.",
            Self::AstralStorm => "The boundaries between planes weaken and blur.",
        }
    }

    /// Returns the weather category
    pub fn category(&self) -> WeatherCategory {
        match self {
            Self::Clear | Self::Cloudy | Self::Overcast => WeatherCategory::Normal,
            Self::LightRain | Self::HeavyRain | Self::Thunderstorm | Self::Hurricane => {
                WeatherCategory::Rain
            }
            Self::LightSnow | Self::HeavySnow | Self::Blizzard => WeatherCategory::Snow,
            Self::Mist | Self::Fog | Self::DenseFog => WeatherCategory::Fog,
            Self::DustStorm | Self::Sandstorm => WeatherCategory::Desert,
            Self::Hail | Self::Sleet => WeatherCategory::Ice,
            Self::Tornado | Self::Earthquake => WeatherCategory::Catastrophic,
            Self::SolarEclipse | Self::LunarEclipse | Self::Aurora | Self::MeteorShower => {
                WeatherCategory::Celestial
            }
            Self::ManaStorm
            | Self::SpiritRain
            | Self::BloodMoon
            | Self::VoidBreach
            | Self::ArcaneWind
            | Self::CrystalRain
            | Self::PhoenixFire
            | Self::AstralStorm => WeatherCategory::Magical,
        }
    }

    /// Returns whether this is a magical weather type
    pub fn is_magical(&self) -> bool {
        matches!(self.category(), WeatherCategory::Magical)
    }

    /// Returns whether this is a dangerous weather type
    pub fn is_dangerous(&self) -> bool {
        matches!(
            self,
            Self::Hurricane
                | Self::Blizzard
                | Self::Tornado
                | Self::Earthquake
                | Self::VoidBreach
                | Self::ManaStorm
                | Self::AstralStorm
        )
    }

    /// Returns whether this is a celestial event
    pub fn is_celestial(&self) -> bool {
        matches!(self.category(), WeatherCategory::Celestial)
    }

    /// Returns a color index for UI display
    /// 0=Black, 1=DarkGrey, 2=White, 3=Red, 4=Green, 5=Brown,
    /// 6=Magenta, 7=Blue, 8=DarkGreen, 9=Cyan, 10=Pink, 11=Yellow
    pub fn color_index(&self) -> u8 {
        match self {
            Self::Clear => 2,         // White
            Self::Cloudy => 1,        // DarkGrey
            Self::Overcast => 1,      // DarkGrey
            Self::LightRain => 9,     // Cyan
            Self::HeavyRain => 7,     // Blue
            Self::Thunderstorm => 11, // Yellow
            Self::Hurricane => 7,     // Blue
            Self::LightSnow => 2,     // White
            Self::HeavySnow => 2,     // White
            Self::Blizzard => 9,      // Cyan
            Self::Mist => 1,          // DarkGrey
            Self::Fog => 1,           // DarkGrey
            Self::DenseFog => 1,      // DarkGrey
            Self::DustStorm => 5,     // Brown
            Self::Sandstorm => 11,    // Yellow
            Self::Hail => 9,          // Cyan
            Self::Sleet => 9,         // Cyan
            Self::Tornado => 1,       // DarkGrey
            Self::Earthquake => 5,    // Brown
            Self::SolarEclipse => 0,  // Black
            Self::LunarEclipse => 3,  // Red
            Self::Aurora => 4,        // Green
            Self::MeteorShower => 11, // Yellow
            Self::ManaStorm => 6,     // Magenta
            Self::SpiritRain => 9,    // Cyan
            Self::BloodMoon => 3,     // Red
            Self::VoidBreach => 6,    // Magenta
            Self::ArcaneWind => 6,    // Magenta
            Self::CrystalRain => 9,   // Cyan
            Self::PhoenixFire => 11,  // Yellow
            Self::AstralStorm => 6,   // Magenta
        }
    }

    /// Returns the glyph character for weather particles (for visual effects)
    pub fn particle_glyph(&self) -> Option<char> {
        match self {
            Self::Clear | Self::Cloudy | Self::Overcast => None,
            Self::LightRain => Some(','),
            Self::HeavyRain | Self::Thunderstorm | Self::Hurricane => Some('|'),
            Self::LightSnow => Some('.'),
            Self::HeavySnow | Self::Blizzard => Some('*'),
            Self::Mist | Self::Fog | Self::DenseFog => Some('.'),
            Self::DustStorm => Some('.'),
            Self::Sandstorm => Some('*'),
            Self::Hail => Some('o'),
            Self::Sleet => Some('/'),
            Self::Tornado => Some('@'),
            Self::Earthquake => Some('~'),
            Self::SolarEclipse | Self::LunarEclipse => Some('O'),
            Self::Aurora => Some('~'),
            Self::MeteorShower => Some('*'),
            Self::ManaStorm => Some('+'),
            Self::SpiritRain => Some('~'),
            Self::BloodMoon => Some('O'),
            Self::VoidBreach => Some('#'),
            Self::ArcaneWind => Some('~'),
            Self::CrystalRain => Some('^'),
            Self::PhoenixFire => Some('^'),
            Self::AstralStorm => Some('*'),
        }
    }

    /// Returns the visibility modifier (1.0 = full visibility, 0.0 = no visibility)
    pub fn visibility_modifier(&self) -> f32 {
        match self {
            Self::Clear => 1.0,
            Self::Cloudy => 0.95,
            Self::Overcast => 0.85,
            Self::LightRain => 0.8,
            Self::HeavyRain => 0.6,
            Self::Thunderstorm => 0.5,
            Self::Hurricane => 0.3,
            Self::LightSnow => 0.85,
            Self::HeavySnow => 0.5,
            Self::Blizzard => 0.2,
            Self::Mist => 0.7,
            Self::Fog => 0.4,
            Self::DenseFog => 0.2,
            Self::DustStorm => 0.5,
            Self::Sandstorm => 0.3,
            Self::Hail => 0.6,
            Self::Sleet => 0.55,
            Self::Tornado => 0.4,
            Self::Earthquake => 0.7,
            Self::SolarEclipse => 0.3,
            Self::LunarEclipse => 0.5,
            Self::Aurora => 0.85,
            Self::MeteorShower => 0.8,
            Self::ManaStorm => 0.6,
            Self::SpiritRain => 0.7,
            Self::BloodMoon => 0.6,
            Self::VoidBreach => 0.4,
            Self::ArcaneWind => 0.75,
            Self::CrystalRain => 0.65,
            Self::PhoenixFire => 0.7,
            Self::AstralStorm => 0.5,
        }
    }

    /// Returns the accuracy modifier for ranged attacks and skills
    pub fn accuracy_modifier(&self) -> f32 {
        match self {
            Self::Clear => 1.0,
            Self::Cloudy => 1.0,
            Self::Overcast => 0.95,
            Self::LightRain => 0.9,
            Self::HeavyRain => 0.75,
            Self::Thunderstorm => 0.7,
            Self::Hurricane => 0.4,
            Self::LightSnow => 0.95,
            Self::HeavySnow => 0.7,
            Self::Blizzard => 0.4,
            Self::Mist => 0.85,
            Self::Fog => 0.7,
            Self::DenseFog => 0.5,
            Self::DustStorm => 0.6,
            Self::Sandstorm => 0.5,
            Self::Hail => 0.75,
            Self::Sleet => 0.7,
            Self::Tornado => 0.3,
            Self::Earthquake => 0.6,
            Self::SolarEclipse => 0.7,
            Self::LunarEclipse => 0.8,
            Self::Aurora => 1.0,
            Self::MeteorShower => 0.9,
            Self::ManaStorm => 0.7,
            Self::SpiritRain => 0.8,
            Self::BloodMoon => 0.85,
            Self::VoidBreach => 0.6,
            Self::ArcaneWind => 0.8,
            Self::CrystalRain => 0.75,
            Self::PhoenixFire => 0.85,
            Self::AstralStorm => 0.65,
        }
    }

    /// Returns the movement speed modifier
    pub fn movement_modifier(&self) -> f32 {
        match self {
            Self::Clear => 1.0,
            Self::Cloudy => 1.0,
            Self::Overcast => 1.0,
            Self::LightRain => 0.95,
            Self::HeavyRain => 0.85,
            Self::Thunderstorm => 0.8,
            Self::Hurricane => 0.5,
            Self::LightSnow => 0.9,
            Self::HeavySnow => 0.7,
            Self::Blizzard => 0.4,
            Self::Mist => 0.95,
            Self::Fog => 0.9,
            Self::DenseFog => 0.85,
            Self::DustStorm => 0.75,
            Self::Sandstorm => 0.6,
            Self::Hail => 0.8,
            Self::Sleet => 0.7,
            Self::Tornado => 0.3,
            Self::Earthquake => 0.5,
            Self::SolarEclipse => 1.0,
            Self::LunarEclipse => 1.0,
            Self::Aurora => 1.0,
            Self::MeteorShower => 0.9,
            Self::ManaStorm => 0.8,
            Self::SpiritRain => 0.9,
            Self::BloodMoon => 1.0,
            Self::VoidBreach => 0.7,
            Self::ArcaneWind => 0.85,
            Self::CrystalRain => 0.8,
            Self::PhoenixFire => 0.9,
            Self::AstralStorm => 0.75,
        }
    }

    /// Returns damage per turn from environmental effects (0 = no damage)
    pub fn environmental_damage(&self) -> i32 {
        match self {
            Self::Clear
            | Self::Cloudy
            | Self::Overcast
            | Self::LightRain
            | Self::LightSnow
            | Self::Mist
            | Self::Fog
            | Self::SolarEclipse
            | Self::LunarEclipse
            | Self::Aurora
            | Self::ArcaneWind
            | Self::BloodMoon => 0,
            Self::HeavyRain | Self::DenseFog | Self::SpiritRain => 0,
            Self::Thunderstorm => 2,
            Self::Hurricane => 5,
            Self::HeavySnow | Self::Sleet => 1,
            Self::Blizzard => 3,
            Self::DustStorm => 1,
            Self::Sandstorm => 2,
            Self::Hail => 2,
            Self::Tornado => 8,
            Self::Earthquake => 4,
            Self::MeteorShower => 3,
            Self::ManaStorm => 4,
            Self::VoidBreach => 6,
            Self::CrystalRain => 2,
            Self::PhoenixFire => -2, // Healing!
            Self::AstralStorm => 5,
        }
    }

    /// Returns the environmental damage type
    pub fn damage_type(&self) -> Option<DamageType> {
        match self {
            Self::Thunderstorm => Some(DamageType::Lightning),
            Self::Hurricane | Self::Tornado => Some(DamageType::Physical),
            Self::Blizzard | Self::HeavySnow | Self::Hail | Self::Sleet => Some(DamageType::Ice),
            Self::DustStorm | Self::Sandstorm | Self::Earthquake => Some(DamageType::Physical),
            Self::MeteorShower | Self::PhoenixFire => Some(DamageType::Fire),
            Self::ManaStorm | Self::ArcaneWind | Self::CrystalRain | Self::AstralStorm => {
                Some(DamageType::Arcane)
            }
            Self::SpiritRain | Self::BloodMoon => Some(DamageType::Dark),
            Self::VoidBreach => Some(DamageType::Void),
            _ => None,
        }
    }

    /// Returns whether this weather extinguishes fire effects
    pub fn extinguishes_fire(&self) -> bool {
        matches!(
            self,
            Self::LightRain
                | Self::HeavyRain
                | Self::Thunderstorm
                | Self::Hurricane
                | Self::Blizzard
                | Self::Hail
                | Self::Sleet
                | Self::SpiritRain
        )
    }

    /// Returns whether this weather enhances lightning/electric attacks
    pub fn conducts_electricity(&self) -> bool {
        matches!(
            self,
            Self::LightRain
                | Self::HeavyRain
                | Self::Thunderstorm
                | Self::Hurricane
                | Self::Sleet
        )
    }

    /// Returns the electric damage multiplier
    pub fn electric_damage_modifier(&self) -> f32 {
        match self {
            Self::LightRain => 1.25,
            Self::HeavyRain => 1.5,
            Self::Thunderstorm => 2.0,
            Self::Hurricane => 1.75,
            Self::Sleet => 1.3,
            Self::ManaStorm => 1.5,
            _ => 1.0,
        }
    }

    /// Returns the fire damage modifier
    pub fn fire_damage_modifier(&self) -> f32 {
        match self {
            Self::Clear => 1.0,
            Self::LightRain => 0.8,
            Self::HeavyRain => 0.5,
            Self::Thunderstorm => 0.6,
            Self::Hurricane => 0.4,
            Self::LightSnow => 0.9,
            Self::HeavySnow => 0.7,
            Self::Blizzard => 0.5,
            Self::Hail | Self::Sleet => 0.6,
            Self::DustStorm => 0.9,
            Self::Sandstorm => 0.8,
            Self::SpiritRain => 0.7,
            Self::PhoenixFire => 2.0,
            Self::ArcaneWind => 1.2,
            _ => 1.0,
        }
    }

    /// Returns the ice damage modifier
    pub fn ice_damage_modifier(&self) -> f32 {
        match self {
            Self::LightRain => 1.1,
            Self::HeavyRain => 1.2,
            Self::LightSnow => 1.3,
            Self::HeavySnow => 1.5,
            Self::Blizzard => 2.0,
            Self::Fog => 1.1,
            Self::DenseFog => 1.15,
            Self::Hail => 1.5,
            Self::Sleet => 1.4,
            Self::PhoenixFire => 0.5,
            _ => 1.0,
        }
    }

    /// Returns the dark damage modifier
    pub fn dark_damage_modifier(&self) -> f32 {
        match self {
            Self::SolarEclipse => 1.5,
            Self::LunarEclipse => 1.75,
            Self::BloodMoon => 2.0,
            Self::VoidBreach => 1.8,
            Self::Aurora => 0.8,
            Self::PhoenixFire => 0.6,
            _ => 1.0,
        }
    }

    /// Returns the holy damage modifier
    pub fn holy_damage_modifier(&self) -> f32 {
        match self {
            Self::Clear => 1.1,
            Self::Aurora => 1.5,
            Self::SolarEclipse => 0.5,
            Self::LunarEclipse => 0.7,
            Self::BloodMoon => 0.5,
            Self::VoidBreach => 0.6,
            Self::PhoenixFire => 1.5,
            _ => 1.0,
        }
    }

    /// Returns the arcane damage modifier
    pub fn arcane_damage_modifier(&self) -> f32 {
        match self {
            Self::ManaStorm => 2.0,
            Self::Aurora => 1.5,
            Self::ArcaneWind => 1.75,
            Self::CrystalRain => 1.5,
            Self::AstralStorm => 1.8,
            Self::VoidBreach => 1.3,
            _ => 1.0,
        }
    }

    /// Returns the mana regeneration modifier
    pub fn mana_regen_modifier(&self) -> f32 {
        match self {
            Self::ManaStorm => 2.0,
            Self::Aurora => 1.5,
            Self::ArcaneWind => 1.3,
            Self::CrystalRain => 1.25,
            Self::AstralStorm => 1.4,
            Self::BloodMoon => 0.8,
            Self::VoidBreach => 0.7,
            _ => 1.0,
        }
    }

    /// Returns the health regeneration modifier
    pub fn health_regen_modifier(&self) -> f32 {
        match self {
            Self::PhoenixFire => 2.0,
            Self::Aurora => 1.3,
            Self::BloodMoon => 0.7,
            Self::VoidBreach => 0.5,
            Self::Blizzard => 0.8,
            Self::Sandstorm => 0.9,
            _ => 1.0,
        }
    }

    /// Returns the default duration range in turns (min, max)
    pub fn default_duration(&self) -> (u32, u32) {
        match self {
            Self::Clear => (100, 300),
            Self::Cloudy => (50, 150),
            Self::Overcast => (40, 120),
            Self::LightRain => (30, 80),
            Self::HeavyRain => (20, 60),
            Self::Thunderstorm => (15, 45),
            Self::Hurricane => (30, 60),
            Self::LightSnow => (40, 100),
            Self::HeavySnow => (30, 70),
            Self::Blizzard => (20, 50),
            Self::Mist => (20, 60),
            Self::Fog => (30, 90),
            Self::DenseFog => (20, 50),
            Self::DustStorm => (25, 65),
            Self::Sandstorm => (20, 50),
            Self::Hail => (10, 30),
            Self::Sleet => (20, 50),
            Self::Tornado => (5, 15),
            Self::Earthquake => (3, 10),
            Self::SolarEclipse => (10, 20),
            Self::LunarEclipse => (15, 30),
            Self::Aurora => (30, 60),
            Self::MeteorShower => (20, 40),
            Self::ManaStorm => (15, 35),
            Self::SpiritRain => (20, 45),
            Self::BloodMoon => (25, 50),
            Self::VoidBreach => (10, 25),
            Self::ArcaneWind => (25, 55),
            Self::CrystalRain => (15, 35),
            Self::PhoenixFire => (10, 25),
            Self::AstralStorm => (15, 35),
        }
    }

    /// Returns the rarity/probability weight (higher = more common)
    pub fn rarity_weight(&self) -> u32 {
        match self {
            Self::Clear => 100,
            Self::Cloudy => 80,
            Self::Overcast => 60,
            Self::LightRain => 50,
            Self::HeavyRain => 30,
            Self::Thunderstorm => 15,
            Self::Hurricane => 3,
            Self::LightSnow => 40,
            Self::HeavySnow => 20,
            Self::Blizzard => 5,
            Self::Mist => 45,
            Self::Fog => 35,
            Self::DenseFog => 15,
            Self::DustStorm => 25,
            Self::Sandstorm => 12,
            Self::Hail => 10,
            Self::Sleet => 15,
            Self::Tornado => 2,
            Self::Earthquake => 2,
            Self::SolarEclipse => 1,
            Self::LunarEclipse => 2,
            Self::Aurora => 5,
            Self::MeteorShower => 3,
            Self::ManaStorm => 4,
            Self::SpiritRain => 5,
            Self::BloodMoon => 2,
            Self::VoidBreach => 1,
            Self::ArcaneWind => 6,
            Self::CrystalRain => 4,
            Self::PhoenixFire => 2,
            Self::AstralStorm => 3,
        }
    }
}

impl Default for WeatherType {
    fn default() -> Self {
        Self::Clear
    }
}

// ============================================================================
// Weather Categories
// ============================================================================

/// Categories of weather for grouping and seasonal effects
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum WeatherCategory {
    /// Normal/clear weather
    Normal,
    /// Rain-based weather
    Rain,
    /// Snow-based weather
    Snow,
    /// Fog-based weather
    Fog,
    /// Desert weather
    Desert,
    /// Ice weather
    Ice,
    /// Catastrophic events
    Catastrophic,
    /// Celestial events
    Celestial,
    /// Magical weather
    Magical,
}

impl WeatherCategory {
    /// Returns all weather types in this category
    pub fn weather_types(&self) -> Vec<WeatherType> {
        WeatherType::all()
            .iter()
            .filter(|w| w.category() == *self)
            .copied()
            .collect()
    }
}

// ============================================================================
// Weather Intensity
// ============================================================================

/// Intensity level of weather effects
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum WeatherIntensity {
    /// Light weather - reduced effects
    Light,
    /// Normal weather intensity
    Normal,
    /// Heavy/severe weather - enhanced effects
    Heavy,
    /// Extreme weather - maximum effects
    Extreme,
}

impl WeatherIntensity {
    /// Returns a multiplier for weather effects based on intensity
    pub fn effect_multiplier(&self) -> f32 {
        match self {
            Self::Light => 0.5,
            Self::Normal => 1.0,
            Self::Heavy => 1.5,
            Self::Extreme => 2.0,
        }
    }

    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Light => "Light",
            Self::Normal => "Moderate",
            Self::Heavy => "Heavy",
            Self::Extreme => "Extreme",
        }
    }
}

impl Default for WeatherIntensity {
    fn default() -> Self {
        Self::Normal
    }
}

// ============================================================================
// Seasons
// ============================================================================

/// The four seasons that affect weather patterns
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum Season {
    /// Spring - frequent rain, mild temperatures
    Spring,
    /// Summer - clear, hot weather
    Summer,
    /// Autumn - fog, storms, falling leaves
    Autumn,
    /// Winter - snow, cold, blizzards
    Winter,
}

impl Season {
    /// Returns the display name of the season
    pub fn name(&self) -> &'static str {
        match self {
            Self::Spring => "Spring",
            Self::Summer => "Summer",
            Self::Autumn => "Autumn",
            Self::Winter => "Winter",
        }
    }

    /// Returns a description of typical weather
    pub fn description(&self) -> &'static str {
        match self {
            Self::Spring => "A time of renewal with frequent rain showers.",
            Self::Summer => "Long, hot days with clear skies.",
            Self::Autumn => "Cooling temperatures bring fog and storms.",
            Self::Winter => "Cold winds and snow blanket the land.",
        }
    }

    /// Returns the temperature modifier (-1.0 cold to 1.0 hot)
    pub fn temperature_modifier(&self) -> f32 {
        match self {
            Self::Spring => 0.2,
            Self::Summer => 0.8,
            Self::Autumn => -0.1,
            Self::Winter => -0.7,
        }
    }

    /// Returns weather type weights for this season
    pub fn weather_weights(&self) -> HashMap<WeatherType, u32> {
        let mut weights = HashMap::new();

        match self {
            Self::Spring => {
                weights.insert(WeatherType::Clear, 40);
                weights.insert(WeatherType::Cloudy, 60);
                weights.insert(WeatherType::Overcast, 50);
                weights.insert(WeatherType::LightRain, 80);
                weights.insert(WeatherType::HeavyRain, 50);
                weights.insert(WeatherType::Thunderstorm, 30);
                weights.insert(WeatherType::Mist, 40);
                weights.insert(WeatherType::Fog, 30);
                weights.insert(WeatherType::Aurora, 5);
            }
            Self::Summer => {
                weights.insert(WeatherType::Clear, 100);
                weights.insert(WeatherType::Cloudy, 40);
                weights.insert(WeatherType::Overcast, 20);
                weights.insert(WeatherType::LightRain, 15);
                weights.insert(WeatherType::HeavyRain, 10);
                weights.insert(WeatherType::Thunderstorm, 20);
                weights.insert(WeatherType::Hurricane, 5);
                weights.insert(WeatherType::DustStorm, 30);
                weights.insert(WeatherType::Sandstorm, 15);
                weights.insert(WeatherType::SolarEclipse, 2);
                weights.insert(WeatherType::MeteorShower, 5);
            }
            Self::Autumn => {
                weights.insert(WeatherType::Clear, 30);
                weights.insert(WeatherType::Cloudy, 70);
                weights.insert(WeatherType::Overcast, 80);
                weights.insert(WeatherType::LightRain, 50);
                weights.insert(WeatherType::HeavyRain, 40);
                weights.insert(WeatherType::Thunderstorm, 35);
                weights.insert(WeatherType::Mist, 60);
                weights.insert(WeatherType::Fog, 70);
                weights.insert(WeatherType::DenseFog, 40);
                weights.insert(WeatherType::LunarEclipse, 3);
                weights.insert(WeatherType::BloodMoon, 5);
                weights.insert(WeatherType::SpiritRain, 10);
            }
            Self::Winter => {
                weights.insert(WeatherType::Clear, 30);
                weights.insert(WeatherType::Cloudy, 50);
                weights.insert(WeatherType::Overcast, 60);
                weights.insert(WeatherType::LightSnow, 70);
                weights.insert(WeatherType::HeavySnow, 50);
                weights.insert(WeatherType::Blizzard, 20);
                weights.insert(WeatherType::Fog, 30);
                weights.insert(WeatherType::DenseFog, 20);
                weights.insert(WeatherType::Hail, 15);
                weights.insert(WeatherType::Sleet, 25);
                weights.insert(WeatherType::Aurora, 15);
            }
        }

        // Add common magical weather at low rates
        weights.insert(WeatherType::ManaStorm, 3);
        weights.insert(WeatherType::ArcaneWind, 5);

        weights
    }

    /// Returns the next season in cycle
    pub fn next(&self) -> Season {
        match self {
            Self::Spring => Self::Summer,
            Self::Summer => Self::Autumn,
            Self::Autumn => Self::Winter,
            Self::Winter => Self::Spring,
        }
    }

    /// Returns the previous season in cycle
    pub fn previous(&self) -> Season {
        match self {
            Self::Spring => Self::Winter,
            Self::Summer => Self::Spring,
            Self::Autumn => Self::Summer,
            Self::Winter => Self::Autumn,
        }
    }

    /// Gets season from day of year (0-365)
    pub fn from_day(day: u32) -> Season {
        match day % 365 {
            0..=90 => Self::Spring,
            91..=181 => Self::Summer,
            182..=272 => Self::Autumn,
            _ => Self::Winter,
        }
    }
}

impl Default for Season {
    fn default() -> Self {
        Self::Spring
    }
}

// ============================================================================
// Weather Transition
// ============================================================================

/// How weather transitions occur
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum WeatherTransitionType {
    /// Weather changes gradually over time
    Gradual,
    /// Weather changes suddenly (storms, magical events)
    Sudden,
    /// Weather is forced by magic or artifacts
    Forced,
}

/// Tracks the current weather transition state
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WeatherTransition {
    /// The weather we're transitioning from
    pub from_weather: WeatherType,
    /// The weather we're transitioning to
    pub to_weather: WeatherType,
    /// Progress of transition (0.0 to 1.0)
    pub progress: f32,
    /// Type of transition
    pub transition_type: WeatherTransitionType,
    /// Total duration of transition in turns
    pub duration: u32,
    /// Turns elapsed in transition
    pub elapsed: u32,
}

impl WeatherTransition {
    /// Create a new gradual transition
    pub fn gradual(from: WeatherType, to: WeatherType, duration: u32) -> Self {
        Self {
            from_weather: from,
            to_weather: to,
            progress: 0.0,
            transition_type: WeatherTransitionType::Gradual,
            duration,
            elapsed: 0,
        }
    }

    /// Create a sudden transition
    pub fn sudden(from: WeatherType, to: WeatherType) -> Self {
        Self {
            from_weather: from,
            to_weather: to,
            progress: 1.0,
            transition_type: WeatherTransitionType::Sudden,
            duration: 1,
            elapsed: 1,
        }
    }

    /// Create a forced transition (magic/artifacts)
    pub fn forced(from: WeatherType, to: WeatherType, duration: u32) -> Self {
        Self {
            from_weather: from,
            to_weather: to,
            progress: 0.0,
            transition_type: WeatherTransitionType::Forced,
            duration,
            elapsed: 0,
        }
    }

    /// Update transition progress
    pub fn update(&mut self) -> bool {
        if self.elapsed >= self.duration {
            return true;
        }
        self.elapsed += 1;
        self.progress = self.elapsed as f32 / self.duration as f32;
        self.progress >= 1.0
    }

    /// Check if transition is complete
    pub fn is_complete(&self) -> bool {
        self.progress >= 1.0
    }

    /// Get interpolated visibility modifier
    pub fn visibility_modifier(&self) -> f32 {
        let from_vis = self.from_weather.visibility_modifier();
        let to_vis = self.to_weather.visibility_modifier();
        from_vis + (to_vis - from_vis) * self.progress
    }
}

// ============================================================================
// Weather Forecasting
// ============================================================================

/// A forecast entry for upcoming weather
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WeatherForecast {
    /// Predicted weather type
    pub weather: WeatherType,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f32,
    /// Turns until this weather
    pub turns_until: u32,
    /// Expected duration
    pub expected_duration: u32,
}

/// Weather forecasting system
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WeatherForecaster {
    /// Player's forecasting skill level (0-100)
    pub skill_level: u32,
    /// Whether a weather vane is equipped
    pub has_weather_vane: bool,
    /// Bonus from NPCs or items
    pub prediction_bonus: f32,
    /// Cached forecasts
    pub cached_forecasts: Vec<WeatherForecast>,
    /// Turns since last forecast update
    pub turns_since_update: u32,
}

impl WeatherForecaster {
    /// Create a new forecaster
    pub fn new() -> Self {
        Self {
            skill_level: 0,
            has_weather_vane: false,
            prediction_bonus: 0.0,
            cached_forecasts: Vec::new(),
            turns_since_update: 0,
        }
    }

    /// Calculate base accuracy of forecasts
    pub fn accuracy(&self) -> f32 {
        let base = self.skill_level as f32 / 100.0;
        let vane_bonus = if self.has_weather_vane { 0.15 } else { 0.0 };
        (base + vane_bonus + self.prediction_bonus).min(0.95)
    }

    /// Generate forecasts for upcoming weather
    pub fn generate_forecasts(
        &mut self,
        current_weather: WeatherType,
        season: Season,
        count: usize,
        rng: &mut impl Rng,
    ) -> Vec<WeatherForecast> {
        let accuracy = self.accuracy();
        let mut forecasts = Vec::new();
        let weights = season.weather_weights();

        let mut turns_ahead = 0u32;

        for i in 0..count {
            // Accuracy decreases with distance
            let distance_factor = 1.0 - (i as f32 * 0.15);
            let forecast_accuracy = (accuracy * distance_factor).max(0.1);

            // Pick a weather type based on season weights
            let weather = if rng.gen::<f32>() < forecast_accuracy {
                self.pick_weighted_weather(&weights, rng)
            } else {
                // Random weather if prediction fails
                *WeatherType::all().choose(rng).unwrap_or(&WeatherType::Clear)
            };

            let (min_dur, max_dur) = weather.default_duration();
            let duration = rng.gen_range(min_dur..=max_dur);
            turns_ahead += duration;

            forecasts.push(WeatherForecast {
                weather,
                confidence: forecast_accuracy,
                turns_until: turns_ahead,
                expected_duration: duration,
            });
        }

        self.cached_forecasts = forecasts.clone();
        self.turns_since_update = 0;
        forecasts
    }

    /// Pick weather based on weighted probabilities
    fn pick_weighted_weather(
        &self,
        weights: &HashMap<WeatherType, u32>,
        rng: &mut impl Rng,
    ) -> WeatherType {
        let total: u32 = weights.values().sum();
        if total == 0 {
            return WeatherType::Clear;
        }

        let roll = rng.gen_range(0..total);
        let mut cumulative = 0;

        for (weather, weight) in weights {
            cumulative += weight;
            if roll < cumulative {
                return *weather;
            }
        }

        WeatherType::Clear
    }

    /// Get hints about upcoming weather
    pub fn get_hints(&self, rng: &mut impl Rng) -> Vec<String> {
        let mut hints = Vec::new();

        for forecast in &self.cached_forecasts {
            if rng.gen::<f32>() < forecast.confidence {
                let certainty = if forecast.confidence > 0.7 {
                    "certainly"
                } else if forecast.confidence > 0.4 {
                    "likely"
                } else {
                    "possibly"
                };

                hints.push(format!(
                    "{} {} coming in about {} turns.",
                    forecast.weather.name(),
                    certainty,
                    forecast.turns_until
                ));
            }
        }

        hints
    }
}

impl Default for WeatherForecaster {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Weather Manipulation
// ============================================================================

/// Spells that can manipulate weather
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum WeatherSpell {
    /// Clears bad weather
    ClearSkies,
    /// Summons rain
    RainDance,
    /// Summons a thunderstorm
    SummonStorm,
    /// Creates fog
    FogBank,
    /// Summons snow
    Snowfall,
    /// Summons a blizzard
    SummonBlizzard,
    /// Creates a sandstorm
    DesertWrath,
    /// Summons a mana storm
    ArcaneStorm,
    /// Calls down the blood moon
    BloodMoonRitual,
    /// Opens a void breach
    TearReality,
    /// Summons phoenix fire
    PhoenixBlessing,
    /// Calms any weather
    WeatherWard,
}

impl WeatherSpell {
    /// Returns the mana cost
    pub fn mana_cost(&self) -> i32 {
        match self {
            Self::ClearSkies => 20,
            Self::RainDance => 30,
            Self::SummonStorm => 60,
            Self::FogBank => 25,
            Self::Snowfall => 35,
            Self::SummonBlizzard => 80,
            Self::DesertWrath => 50,
            Self::ArcaneStorm => 100,
            Self::BloodMoonRitual => 150,
            Self::TearReality => 200,
            Self::PhoenixBlessing => 120,
            Self::WeatherWard => 40,
        }
    }

    /// Returns the required skill level
    pub fn required_skill(&self) -> u32 {
        match self {
            Self::ClearSkies => 5,
            Self::RainDance => 10,
            Self::SummonStorm => 30,
            Self::FogBank => 15,
            Self::Snowfall => 20,
            Self::SummonBlizzard => 50,
            Self::DesertWrath => 35,
            Self::ArcaneStorm => 60,
            Self::BloodMoonRitual => 80,
            Self::TearReality => 90,
            Self::PhoenixBlessing => 70,
            Self::WeatherWard => 25,
        }
    }

    /// Returns the weather type this spell creates
    pub fn resulting_weather(&self) -> Option<WeatherType> {
        match self {
            Self::ClearSkies => Some(WeatherType::Clear),
            Self::RainDance => Some(WeatherType::HeavyRain),
            Self::SummonStorm => Some(WeatherType::Thunderstorm),
            Self::FogBank => Some(WeatherType::DenseFog),
            Self::Snowfall => Some(WeatherType::HeavySnow),
            Self::SummonBlizzard => Some(WeatherType::Blizzard),
            Self::DesertWrath => Some(WeatherType::Sandstorm),
            Self::ArcaneStorm => Some(WeatherType::ManaStorm),
            Self::BloodMoonRitual => Some(WeatherType::BloodMoon),
            Self::TearReality => Some(WeatherType::VoidBreach),
            Self::PhoenixBlessing => Some(WeatherType::PhoenixFire),
            Self::WeatherWard => None, // Just protects, doesn't change
        }
    }

    /// Returns the duration of the effect
    pub fn duration(&self) -> u32 {
        match self {
            Self::ClearSkies => 100,
            Self::RainDance => 50,
            Self::SummonStorm => 30,
            Self::FogBank => 40,
            Self::Snowfall => 45,
            Self::SummonBlizzard => 25,
            Self::DesertWrath => 35,
            Self::ArcaneStorm => 20,
            Self::BloodMoonRitual => 30,
            Self::TearReality => 15,
            Self::PhoenixBlessing => 20,
            Self::WeatherWard => 60,
        }
    }

    /// Returns the name of the spell
    pub fn name(&self) -> &'static str {
        match self {
            Self::ClearSkies => "Clear Skies",
            Self::RainDance => "Rain Dance",
            Self::SummonStorm => "Summon Storm",
            Self::FogBank => "Fog Bank",
            Self::Snowfall => "Snowfall",
            Self::SummonBlizzard => "Summon Blizzard",
            Self::DesertWrath => "Desert Wrath",
            Self::ArcaneStorm => "Arcane Storm",
            Self::BloodMoonRitual => "Blood Moon Ritual",
            Self::TearReality => "Tear Reality",
            Self::PhoenixBlessing => "Phoenix Blessing",
            Self::WeatherWard => "Weather Ward",
        }
    }

    /// Returns the description
    pub fn description(&self) -> &'static str {
        match self {
            Self::ClearSkies => "Dispels clouds and calms the weather.",
            Self::RainDance => "Calls forth heavy rain from the skies.",
            Self::SummonStorm => "Summons a powerful thunderstorm with lightning.",
            Self::FogBank => "Creates a thick bank of concealing fog.",
            Self::Snowfall => "Brings heavy snowfall to the area.",
            Self::SummonBlizzard => "Unleashes a devastating blizzard.",
            Self::DesertWrath => "Conjures a blinding sandstorm.",
            Self::ArcaneStorm => "Releases wild magical energy into the atmosphere.",
            Self::BloodMoonRitual => "Performs a dark ritual to summon the blood moon.",
            Self::TearReality => "Opens a breach to the void.",
            Self::PhoenixBlessing => "Calls down the healing flames of the phoenix.",
            Self::WeatherWard => "Creates a protective barrier against weather effects.",
        }
    }
}

/// Artifacts that affect weather
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum WeatherArtifact {
    /// Controls storms
    StormOrb,
    /// Brings sunny weather
    SunStone,
    /// Controls cold and ice
    FrostCrown,
    /// Controls rain
    RainScepter,
    /// Controls fog
    MistCloak,
    /// Protects from all weather
    WeatherShield,
    /// Amplifies magical weather
    ArcaneConduit,
    /// Controls the moon phases
    LunarAmulet,
    /// Predicts weather perfectly
    OracleGlass,
    /// Creates localized weather
    ElementalPrism,
}

impl WeatherArtifact {
    /// Returns the name
    pub fn name(&self) -> &'static str {
        match self {
            Self::StormOrb => "Storm Orb",
            Self::SunStone => "Sun Stone",
            Self::FrostCrown => "Frost Crown",
            Self::RainScepter => "Rain Scepter",
            Self::MistCloak => "Mist Cloak",
            Self::WeatherShield => "Weather Shield",
            Self::ArcaneConduit => "Arcane Conduit",
            Self::LunarAmulet => "Lunar Amulet",
            Self::OracleGlass => "Oracle Glass",
            Self::ElementalPrism => "Elemental Prism",
        }
    }

    /// Returns the description
    pub fn description(&self) -> &'static str {
        match self {
            Self::StormOrb => "A crackling sphere that commands the storms.",
            Self::SunStone => "A golden gem that radiates warmth and light.",
            Self::FrostCrown => "An icy diadem that controls winter's fury.",
            Self::RainScepter => "A staff that calls rain from clear skies.",
            Self::MistCloak => "A cloak woven from solidified fog.",
            Self::WeatherShield => "An ancient shield that deflects all weather.",
            Self::ArcaneConduit => "A crystal that channels magical weather.",
            Self::LunarAmulet => "A silver pendant linked to the moon.",
            Self::OracleGlass => "A scrying glass that shows future weather.",
            Self::ElementalPrism => "A prism that refracts weather into elements.",
        }
    }

    /// Returns the weather this artifact can summon
    pub fn summonable_weather(&self) -> Vec<WeatherType> {
        match self {
            Self::StormOrb => vec![
                WeatherType::Thunderstorm,
                WeatherType::Hurricane,
                WeatherType::Tornado,
            ],
            Self::SunStone => vec![WeatherType::Clear],
            Self::FrostCrown => vec![
                WeatherType::LightSnow,
                WeatherType::HeavySnow,
                WeatherType::Blizzard,
                WeatherType::Hail,
            ],
            Self::RainScepter => vec![
                WeatherType::LightRain,
                WeatherType::HeavyRain,
                WeatherType::Thunderstorm,
            ],
            Self::MistCloak => vec![WeatherType::Mist, WeatherType::Fog, WeatherType::DenseFog],
            Self::WeatherShield => vec![],
            Self::ArcaneConduit => vec![
                WeatherType::ManaStorm,
                WeatherType::ArcaneWind,
                WeatherType::AstralStorm,
            ],
            Self::LunarAmulet => vec![
                WeatherType::LunarEclipse,
                WeatherType::BloodMoon,
                WeatherType::Aurora,
            ],
            Self::OracleGlass => vec![],
            Self::ElementalPrism => vec![
                WeatherType::CrystalRain,
                WeatherType::PhoenixFire,
                WeatherType::SpiritRain,
            ],
        }
    }

    /// Returns the immunity this artifact provides
    pub fn immunity(&self) -> Option<WeatherCategory> {
        match self {
            Self::StormOrb => Some(WeatherCategory::Rain),
            Self::FrostCrown => Some(WeatherCategory::Snow),
            Self::MistCloak => Some(WeatherCategory::Fog),
            Self::WeatherShield => None, // Special: all weather immunity
            _ => None,
        }
    }

    /// Returns whether this provides total weather immunity
    pub fn provides_total_immunity(&self) -> bool {
        matches!(self, Self::WeatherShield)
    }

    /// Returns the forecast bonus
    pub fn forecast_bonus(&self) -> f32 {
        match self {
            Self::OracleGlass => 0.5,
            Self::LunarAmulet => 0.1,
            _ => 0.0,
        }
    }
}

/// Rituals for weather control
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WeatherRitual {
    /// Name of the ritual
    pub name: String,
    /// Description
    pub description: String,
    /// Required components (item names)
    pub components: Vec<String>,
    /// Required number of participants
    pub participants_required: u32,
    /// Time to perform (in turns)
    pub cast_time: u32,
    /// Resulting weather
    pub resulting_weather: WeatherType,
    /// Duration of effect
    pub duration: u32,
    /// Whether this is a permanent change
    pub permanent: bool,
}

impl WeatherRitual {
    /// Create a new ritual
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        components: Vec<String>,
        participants: u32,
        cast_time: u32,
        weather: WeatherType,
        duration: u32,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            components,
            participants_required: participants,
            cast_time,
            resulting_weather: weather,
            duration,
            permanent: false,
        }
    }

    /// Creates the Weather Binding ritual
    pub fn weather_binding() -> Self {
        Self::new(
            "Weather Binding",
            "Binds the current weather in place, preventing natural changes.",
            vec![
                "Crystal Anchor".to_string(),
                "Elemental Essence".to_string(),
                "Time Sand".to_string(),
            ],
            1,
            20,
            WeatherType::Clear, // Placeholder - binds current weather
            500,
        )
    }

    /// Creates the Elemental Convergence ritual
    pub fn elemental_convergence() -> Self {
        Self::new(
            "Elemental Convergence",
            "Calls upon all elements to create a powerful magical storm.",
            vec![
                "Fire Crystal".to_string(),
                "Ice Crystal".to_string(),
                "Lightning Crystal".to_string(),
                "Earth Crystal".to_string(),
                "Wind Crystal".to_string(),
            ],
            4,
            50,
            WeatherType::AstralStorm,
            100,
        )
    }

    /// Creates the Void Summoning ritual
    pub fn void_summoning() -> Self {
        let mut ritual = Self::new(
            "Void Summoning",
            "Tears open the fabric of reality to let the void seep through.",
            vec![
                "Void Shard".to_string(),
                "Soul Gem".to_string(),
                "Reality Anchor".to_string(),
                "Blood of the Innocent".to_string(),
            ],
            3,
            100,
            WeatherType::VoidBreach,
            50,
        );
        ritual.permanent = false;
        ritual
    }

    /// Creates the Phoenix Calling ritual
    pub fn phoenix_calling() -> Self {
        Self::new(
            "Phoenix Calling",
            "Calls upon the spirit of the phoenix to rain healing fire.",
            vec![
                "Phoenix Feather".to_string(),
                "Sacred Flame".to_string(),
                "Life Crystal".to_string(),
            ],
            2,
            30,
            WeatherType::PhoenixFire,
            40,
        )
    }

    /// Creates the Eternal Winter ritual
    pub fn eternal_winter() -> Self {
        let mut ritual = Self::new(
            "Eternal Winter",
            "Brings an endless winter to the land.",
            vec![
                "Heart of Ice".to_string(),
                "Frozen Tear".to_string(),
                "Winter's Breath".to_string(),
                "Frost Giant's Blood".to_string(),
            ],
            5,
            200,
            WeatherType::Blizzard,
            1000,
        );
        ritual.permanent = true;
        ritual
    }

    /// Get all predefined rituals
    pub fn all_rituals() -> Vec<Self> {
        vec![
            Self::weather_binding(),
            Self::elemental_convergence(),
            Self::void_summoning(),
            Self::phoenix_calling(),
            Self::eternal_winter(),
        ]
    }
}

// ============================================================================
// Special Spawns
// ============================================================================

/// Creatures that spawn during specific weather
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WeatherSpawn {
    /// Creature type/name
    pub creature: String,
    /// Weather that triggers spawn
    pub weather: WeatherType,
    /// Spawn chance per turn (0.0 - 1.0)
    pub spawn_chance: f32,
    /// Maximum concurrent spawns
    pub max_spawns: u32,
    /// Whether creature despawns when weather ends
    pub weather_dependent: bool,
}

impl WeatherSpawn {
    /// Get default spawns for a weather type
    pub fn for_weather(weather: WeatherType) -> Vec<Self> {
        match weather {
            WeatherType::Thunderstorm => vec![
                Self {
                    creature: "Lightning Elemental".to_string(),
                    weather,
                    spawn_chance: 0.05,
                    max_spawns: 2,
                    weather_dependent: true,
                },
                Self {
                    creature: "Storm Sprite".to_string(),
                    weather,
                    spawn_chance: 0.08,
                    max_spawns: 4,
                    weather_dependent: true,
                },
            ],
            WeatherType::Blizzard => vec![
                Self {
                    creature: "Ice Elemental".to_string(),
                    weather,
                    spawn_chance: 0.04,
                    max_spawns: 2,
                    weather_dependent: true,
                },
                Self {
                    creature: "Frost Wraith".to_string(),
                    weather,
                    spawn_chance: 0.06,
                    max_spawns: 3,
                    weather_dependent: true,
                },
            ],
            WeatherType::Sandstorm => vec![Self {
                creature: "Sand Wurm".to_string(),
                weather,
                spawn_chance: 0.03,
                max_spawns: 1,
                weather_dependent: false,
            }],
            WeatherType::DenseFog => vec![Self {
                creature: "Fog Phantom".to_string(),
                weather,
                spawn_chance: 0.07,
                max_spawns: 3,
                weather_dependent: true,
            }],
            WeatherType::BloodMoon => vec![
                Self {
                    creature: "Blood Fiend".to_string(),
                    weather,
                    spawn_chance: 0.1,
                    max_spawns: 5,
                    weather_dependent: true,
                },
                Self {
                    creature: "Vampiric Bat Swarm".to_string(),
                    weather,
                    spawn_chance: 0.15,
                    max_spawns: 8,
                    weather_dependent: true,
                },
                Self {
                    creature: "Risen Dead".to_string(),
                    weather,
                    spawn_chance: 0.2,
                    max_spawns: 10,
                    weather_dependent: false,
                },
            ],
            WeatherType::ManaStorm => vec![
                Self {
                    creature: "Mana Wisp".to_string(),
                    weather,
                    spawn_chance: 0.12,
                    max_spawns: 6,
                    weather_dependent: true,
                },
                Self {
                    creature: "Arcane Anomaly".to_string(),
                    weather,
                    spawn_chance: 0.05,
                    max_spawns: 2,
                    weather_dependent: true,
                },
            ],
            WeatherType::VoidBreach => vec![
                Self {
                    creature: "Void Spawn".to_string(),
                    weather,
                    spawn_chance: 0.08,
                    max_spawns: 4,
                    weather_dependent: false,
                },
                Self {
                    creature: "Reality Tear".to_string(),
                    weather,
                    spawn_chance: 0.03,
                    max_spawns: 1,
                    weather_dependent: true,
                },
                Self {
                    creature: "Eldritch Horror".to_string(),
                    weather,
                    spawn_chance: 0.01,
                    max_spawns: 1,
                    weather_dependent: false,
                },
            ],
            WeatherType::SpiritRain => vec![
                Self {
                    creature: "Lost Spirit".to_string(),
                    weather,
                    spawn_chance: 0.1,
                    max_spawns: 5,
                    weather_dependent: true,
                },
                Self {
                    creature: "Spectral Guardian".to_string(),
                    weather,
                    spawn_chance: 0.04,
                    max_spawns: 2,
                    weather_dependent: true,
                },
            ],
            WeatherType::MeteorShower => vec![Self {
                creature: "Star Fragment Golem".to_string(),
                weather,
                spawn_chance: 0.02,
                max_spawns: 1,
                weather_dependent: false,
            }],
            WeatherType::Aurora => vec![Self {
                creature: "Aurora Spirit".to_string(),
                weather,
                spawn_chance: 0.05,
                max_spawns: 2,
                weather_dependent: true,
            }],
            WeatherType::AstralStorm => vec![
                Self {
                    creature: "Astral Drifter".to_string(),
                    weather,
                    spawn_chance: 0.06,
                    max_spawns: 3,
                    weather_dependent: true,
                },
                Self {
                    creature: "Planar Rift".to_string(),
                    weather,
                    spawn_chance: 0.02,
                    max_spawns: 1,
                    weather_dependent: true,
                },
            ],
            _ => vec![],
        }
    }
}

// ============================================================================
// Damage Types
// ============================================================================

/// Types of damage for weather interaction
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum DamageType {
    Physical,
    Fire,
    Ice,
    Lightning,
    Poison,
    Holy,
    Dark,
    Arcane,
    Void,
    Nature,
}

// ============================================================================
// Weather Effects
// ============================================================================

/// Complete set of weather effects on gameplay
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WeatherEffects {
    /// Visibility modifier (0.0 - 1.0)
    pub visibility: f32,
    /// Movement speed modifier
    pub movement_speed: f32,
    /// Accuracy modifier
    pub accuracy: f32,
    /// Environmental damage per turn
    pub env_damage: i32,
    /// Type of environmental damage
    pub env_damage_type: Option<DamageType>,
    /// Damage modifiers by type
    pub damage_modifiers: HashMap<DamageType, f32>,
    /// Mana regeneration modifier
    pub mana_regen: f32,
    /// Health regeneration modifier
    pub health_regen: f32,
    /// Whether fire effects are suppressed
    pub suppress_fire: bool,
    /// Whether electricity is enhanced
    pub enhance_electricity: bool,
    /// Special spawns active
    pub active_spawns: Vec<WeatherSpawn>,
}

impl WeatherEffects {
    /// Calculate effects from weather system
    pub fn from_weather_system(system: &WeatherSystem) -> Self {
        let weather = system.current_weather;
        let intensity = system.intensity.effect_multiplier();

        let mut damage_modifiers = HashMap::new();
        damage_modifiers.insert(DamageType::Fire, weather.fire_damage_modifier());
        damage_modifiers.insert(DamageType::Ice, weather.ice_damage_modifier());
        damage_modifiers.insert(DamageType::Lightning, weather.electric_damage_modifier());
        damage_modifiers.insert(DamageType::Dark, weather.dark_damage_modifier());
        damage_modifiers.insert(DamageType::Holy, weather.holy_damage_modifier());
        damage_modifiers.insert(DamageType::Arcane, weather.arcane_damage_modifier());

        Self {
            visibility: system.visibility_modifier(),
            movement_speed: system.movement_modifier(),
            accuracy: system.accuracy_modifier(),
            env_damage: system.environmental_damage(),
            env_damage_type: weather.damage_type(),
            damage_modifiers,
            mana_regen: weather.mana_regen_modifier() * intensity,
            health_regen: weather.health_regen_modifier(),
            suppress_fire: weather.extinguishes_fire(),
            enhance_electricity: weather.conducts_electricity(),
            active_spawns: WeatherSpawn::for_weather(weather),
        }
    }
}

// ============================================================================
// Weather Combat Modifiers
// ============================================================================

/// Weather effects that can be applied to combat calculations
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeatherCombatModifiers {
    /// Accuracy modifier for attacks
    pub accuracy: f32,
    /// Damage modifier by type
    pub fire_damage: f32,
    pub ice_damage: f32,
    pub lightning_damage: f32,
    pub dark_damage: f32,
    pub holy_damage: f32,
    pub arcane_damage: f32,
    /// Whether burn effects are suppressed
    pub suppress_burn: bool,
    /// Environmental damage to apply
    pub env_damage: i32,
    /// Environmental damage type
    pub env_damage_type: Option<DamageType>,
}

impl WeatherCombatModifiers {
    /// Calculate combat modifiers from a weather system
    pub fn from_weather(weather: &WeatherSystem) -> Self {
        Self {
            accuracy: weather.accuracy_modifier(),
            fire_damage: weather.damage_modifier(DamageType::Fire),
            ice_damage: weather.damage_modifier(DamageType::Ice),
            lightning_damage: weather.damage_modifier(DamageType::Lightning),
            dark_damage: weather.damage_modifier(DamageType::Dark),
            holy_damage: weather.damage_modifier(DamageType::Holy),
            arcane_damage: weather.damage_modifier(DamageType::Arcane),
            suppress_burn: weather.should_extinguish_fire(),
            env_damage: weather.environmental_damage(),
            env_damage_type: weather.current_weather.damage_type(),
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
            DamageType::Dark => self.dark_damage,
            DamageType::Holy => self.holy_damage,
            DamageType::Arcane => self.arcane_damage,
            _ => 1.0,
        };
        ((base_damage as f32) * modifier) as i32
    }
}

// ============================================================================
// Weather Change Event
// ============================================================================

/// Event fired when weather changes
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeatherChangeEvent {
    /// Previous weather type
    pub from: WeatherType,
    /// New weather type
    pub to: WeatherType,
    /// Intensity of new weather
    pub intensity: WeatherIntensity,
    /// Type of transition
    pub transition_type: WeatherTransitionType,
    /// Whether this was caused by magic
    pub magical_cause: bool,
}

impl WeatherChangeEvent {
    /// Get a message describing the weather change
    pub fn message(&self) -> String {
        if self.to == WeatherType::Clear {
            match self.from {
                WeatherType::LightRain | WeatherType::HeavyRain => {
                    "The rain stops and the skies clear.".to_string()
                }
                WeatherType::Thunderstorm => {
                    "The thunder fades as the storm passes.".to_string()
                }
                WeatherType::Hurricane => {
                    "The hurricane finally subsides, leaving destruction in its wake.".to_string()
                }
                WeatherType::Fog | WeatherType::DenseFog | WeatherType::Mist => {
                    "The fog lifts, revealing your surroundings.".to_string()
                }
                WeatherType::Sandstorm | WeatherType::DustStorm => {
                    "The sandstorm subsides.".to_string()
                }
                WeatherType::Blizzard | WeatherType::HeavySnow => {
                    "The snow stops falling as the skies clear.".to_string()
                }
                WeatherType::ManaStorm => {
                    "The wild magic settles, returning to normal.".to_string()
                }
                WeatherType::BloodMoon => "The blood moon fades, its crimson light waning.".to_string(),
                WeatherType::VoidBreach => "Reality seals itself, closing the void breach.".to_string(),
                _ => "The weather clears.".to_string(),
            }
        } else if self.magical_cause {
            format!(
                "Magical forces conjure {}! {}",
                self.to.name().to_lowercase(),
                self.to.description()
            )
        } else if matches!(self.transition_type, WeatherTransitionType::Sudden) {
            format!(
                "{} {} strikes suddenly! {}",
                self.intensity.name(),
                self.to.name().to_lowercase(),
                self.to.description()
            )
        } else {
            format!(
                "{} {} begins. {}",
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

// ============================================================================
// Weather System
// ============================================================================

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
    /// Current season
    pub season: Season,
    /// Day of the year (0-364)
    pub day_of_year: u32,
    /// Days per season
    pub days_per_season: u32,
    /// Current weather transition (if any)
    pub transition: Option<WeatherTransition>,
    /// Weather forecaster
    pub forecaster: WeatherForecaster,
    /// Equipped weather artifacts
    pub equipped_artifacts: Vec<WeatherArtifact>,
    /// Active weather ward (turns remaining)
    pub weather_ward_turns: u32,
    /// Weather locked by ritual/spell (cannot change naturally)
    pub weather_locked: bool,
    /// Turns remaining on weather lock
    pub lock_turns_remaining: u32,
    /// History of recent weather
    pub weather_history: Vec<(WeatherType, u32)>,
    /// Maximum history entries
    pub max_history: usize,
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
            season: Season::Spring,
            day_of_year: 0,
            days_per_season: 91,
            transition: None,
            forecaster: WeatherForecaster::new(),
            equipped_artifacts: Vec::new(),
            weather_ward_turns: 0,
            weather_locked: false,
            lock_turns_remaining: 0,
            weather_history: Vec::new(),
            max_history: 20,
        }
    }

    /// Set the current weather type
    pub fn set_weather(&mut self, weather: WeatherType) {
        self.add_to_history(self.current_weather, self.duration_remaining);
        self.current_weather = weather;
        let (min_dur, max_dur) = weather.default_duration();
        self.duration_remaining = (min_dur + max_dur) / 2;
    }

    /// Set the weather with a specific intensity
    pub fn set_weather_with_intensity(&mut self, weather: WeatherType, intensity: WeatherIntensity) {
        self.add_to_history(self.current_weather, self.duration_remaining);
        self.current_weather = weather;
        self.intensity = intensity;
        let (min_dur, max_dur) = weather.default_duration();
        self.duration_remaining = (min_dur + max_dur) / 2;
    }

    /// Set the weather with full control
    pub fn set_weather_full(
        &mut self,
        weather: WeatherType,
        intensity: WeatherIntensity,
        duration: u32,
    ) {
        self.add_to_history(self.current_weather, self.duration_remaining);
        self.current_weather = weather;
        self.intensity = intensity;
        self.duration_remaining = duration;
    }

    /// Set the current season
    pub fn set_season(&mut self, season: Season) {
        self.season = season;
    }

    /// Advance the day counter
    pub fn advance_day(&mut self) {
        self.day_of_year = (self.day_of_year + 1) % 365;
        self.season = Season::from_day(self.day_of_year);
    }

    /// Add weather to history
    fn add_to_history(&mut self, weather: WeatherType, duration: u32) {
        self.weather_history.push((weather, duration));
        while self.weather_history.len() > self.max_history {
            self.weather_history.remove(0);
        }
    }

    /// Update the weather system (call each turn)
    pub fn update(&mut self, rng: &mut impl Rng) -> Option<WeatherChangeEvent> {
        if !self.enabled {
            return None;
        }

        // Update weather ward
        if self.weather_ward_turns > 0 {
            self.weather_ward_turns -= 1;
        }

        // Update weather lock
        if self.weather_locked {
            if self.lock_turns_remaining > 0 {
                self.lock_turns_remaining -= 1;
            } else {
                self.weather_locked = false;
            }
            return None;
        }

        // Update transition if active
        if let Some(ref mut transition) = self.transition {
            if transition.update() {
                let event = WeatherChangeEvent {
                    from: transition.from_weather,
                    to: transition.to_weather,
                    intensity: self.intensity,
                    transition_type: transition.transition_type,
                    magical_cause: matches!(
                        transition.transition_type,
                        WeatherTransitionType::Forced
                    ),
                };
                self.current_weather = transition.to_weather;
                self.transition = None;
                return Some(event);
            }
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
                    transition_type: WeatherTransitionType::Gradual,
                    magical_cause: false,
                });
            }
        }

        // Random chance for sudden weather events
        if rng.gen::<f32>() < 0.001 {
            if let Some(event) = self.trigger_sudden_event(rng) {
                return Some(event);
            }
        }

        None
    }

    /// Transition to a new random weather type based on season
    fn transition_weather(&mut self, rng: &mut impl Rng) {
        let weights = self.season.weather_weights();

        // Add weights for possible transitions from current weather
        let mut adjusted_weights = weights.clone();

        // Increase chance of related weather
        match self.current_weather {
            WeatherType::Cloudy => {
                *adjusted_weights.entry(WeatherType::Overcast).or_insert(0) += 20;
                *adjusted_weights.entry(WeatherType::LightRain).or_insert(0) += 15;
            }
            WeatherType::Overcast => {
                *adjusted_weights.entry(WeatherType::HeavyRain).or_insert(0) += 20;
                *adjusted_weights.entry(WeatherType::Thunderstorm).or_insert(0) += 10;
            }
            WeatherType::LightRain => {
                *adjusted_weights.entry(WeatherType::HeavyRain).or_insert(0) += 25;
                *adjusted_weights.entry(WeatherType::Clear).or_insert(0) += 20;
            }
            WeatherType::HeavyRain => {
                *adjusted_weights.entry(WeatherType::Thunderstorm).or_insert(0) += 20;
                *adjusted_weights.entry(WeatherType::LightRain).or_insert(0) += 25;
            }
            WeatherType::LightSnow => {
                *adjusted_weights.entry(WeatherType::HeavySnow).or_insert(0) += 20;
            }
            WeatherType::HeavySnow => {
                *adjusted_weights.entry(WeatherType::Blizzard).or_insert(0) += 15;
                *adjusted_weights.entry(WeatherType::LightSnow).or_insert(0) += 20;
            }
            WeatherType::Mist => {
                *adjusted_weights.entry(WeatherType::Fog).or_insert(0) += 25;
            }
            WeatherType::Fog => {
                *adjusted_weights.entry(WeatherType::DenseFog).or_insert(0) += 15;
                *adjusted_weights.entry(WeatherType::Clear).or_insert(0) += 20;
            }
            _ => {}
        }

        let total: u32 = adjusted_weights.values().sum();
        if total == 0 {
            self.current_weather = WeatherType::Clear;
            return;
        }

        let roll = rng.gen_range(0..total);
        let mut cumulative = 0;

        for (weather, weight) in &adjusted_weights {
            cumulative += weight;
            if roll < cumulative {
                self.add_to_history(self.current_weather, self.duration_remaining);
                self.current_weather = *weather;
                break;
            }
        }

        // Randomize intensity
        self.intensity = match rng.gen_range(0..100) {
            0..=20 => WeatherIntensity::Light,
            21..=70 => WeatherIntensity::Normal,
            71..=90 => WeatherIntensity::Heavy,
            _ => WeatherIntensity::Extreme,
        };

        // Reset duration based on weather type
        let (min_dur, max_dur) = self.current_weather.default_duration();
        self.duration_remaining = rng.gen_range(min_dur..=max_dur);
    }

    /// Trigger a sudden weather event
    fn trigger_sudden_event(&mut self, rng: &mut impl Rng) -> Option<WeatherChangeEvent> {
        let sudden_events = [
            (WeatherType::Tornado, 1),
            (WeatherType::Earthquake, 1),
            (WeatherType::MeteorShower, 2),
            (WeatherType::VoidBreach, 1),
            (WeatherType::AstralStorm, 2),
        ];

        let total: u32 = sudden_events.iter().map(|(_, w)| w).sum();
        let roll = rng.gen_range(0..total);
        let mut cumulative = 0;

        for (weather, weight) in &sudden_events {
            cumulative += weight;
            if roll < cumulative {
                let old_weather = self.current_weather;
                self.add_to_history(old_weather, self.duration_remaining);
                self.current_weather = *weather;
                self.intensity = WeatherIntensity::Extreme;

                let (min_dur, max_dur) = weather.default_duration();
                self.duration_remaining = rng.gen_range(min_dur..=max_dur);

                return Some(WeatherChangeEvent {
                    from: old_weather,
                    to: *weather,
                    intensity: self.intensity,
                    transition_type: WeatherTransitionType::Sudden,
                    magical_cause: false,
                });
            }
        }

        None
    }

    /// Cast a weather spell
    pub fn cast_spell(
        &mut self,
        spell: WeatherSpell,
        caster_skill: u32,
        rng: &mut impl Rng,
    ) -> Result<Option<WeatherChangeEvent>, &'static str> {
        if caster_skill < spell.required_skill() {
            return Err("Insufficient skill to cast this spell");
        }

        if spell == WeatherSpell::WeatherWard {
            self.weather_ward_turns = spell.duration();
            return Ok(None);
        }

        if let Some(new_weather) = spell.resulting_weather() {
            let old_weather = self.current_weather;

            // Start a forced transition
            self.transition = Some(WeatherTransition::forced(
                old_weather,
                new_weather,
                5, // Quick transition for spells
            ));

            self.intensity = match rng.gen_range(0..100) {
                0..=30 => WeatherIntensity::Normal,
                31..=70 => WeatherIntensity::Heavy,
                _ => WeatherIntensity::Extreme,
            };

            self.duration_remaining = spell.duration();

            return Ok(Some(WeatherChangeEvent {
                from: old_weather,
                to: new_weather,
                intensity: self.intensity,
                transition_type: WeatherTransitionType::Forced,
                magical_cause: true,
            }));
        }

        Ok(None)
    }

    /// Use a weather artifact to change weather
    pub fn use_artifact(
        &mut self,
        artifact: WeatherArtifact,
        rng: &mut impl Rng,
    ) -> Result<Option<WeatherChangeEvent>, &'static str> {
        let summonable = artifact.summonable_weather();
        if summonable.is_empty() {
            return Err("This artifact cannot summon weather");
        }

        let new_weather = *summonable.choose(rng).unwrap();
        let old_weather = self.current_weather;

        self.add_to_history(old_weather, self.duration_remaining);
        self.current_weather = new_weather;
        self.intensity = WeatherIntensity::Heavy;

        let (min_dur, max_dur) = new_weather.default_duration();
        self.duration_remaining = rng.gen_range(min_dur..=max_dur);

        Ok(Some(WeatherChangeEvent {
            from: old_weather,
            to: new_weather,
            intensity: self.intensity,
            transition_type: WeatherTransitionType::Forced,
            magical_cause: true,
        }))
    }

    /// Perform a weather ritual
    pub fn perform_ritual(
        &mut self,
        ritual: &WeatherRitual,
        _rng: &mut impl Rng,
    ) -> Result<WeatherChangeEvent, &'static str> {
        let old_weather = self.current_weather;

        self.add_to_history(old_weather, self.duration_remaining);
        self.current_weather = ritual.resulting_weather;
        self.intensity = WeatherIntensity::Extreme;
        self.duration_remaining = ritual.duration;

        if ritual.permanent {
            self.weather_locked = true;
            self.lock_turns_remaining = u32::MAX;
        }

        Ok(WeatherChangeEvent {
            from: old_weather,
            to: ritual.resulting_weather,
            intensity: self.intensity,
            transition_type: WeatherTransitionType::Forced,
            magical_cause: true,
        })
    }

    /// Lock the current weather for a duration
    pub fn lock_weather(&mut self, duration: u32) {
        self.weather_locked = true;
        self.lock_turns_remaining = duration;
    }

    /// Unlock weather changes
    pub fn unlock_weather(&mut self) {
        self.weather_locked = false;
        self.lock_turns_remaining = 0;
    }

    /// Equip a weather artifact
    pub fn equip_artifact(&mut self, artifact: WeatherArtifact) {
        if !self.equipped_artifacts.contains(&artifact) {
            self.equipped_artifacts.push(artifact);

            // Update forecaster bonus
            self.forecaster.prediction_bonus = self
                .equipped_artifacts
                .iter()
                .map(|a| a.forecast_bonus())
                .sum();
        }
    }

    /// Unequip a weather artifact
    pub fn unequip_artifact(&mut self, artifact: WeatherArtifact) {
        self.equipped_artifacts.retain(|a| *a != artifact);

        // Update forecaster bonus
        self.forecaster.prediction_bonus = self
            .equipped_artifacts
            .iter()
            .map(|a| a.forecast_bonus())
            .sum();
    }

    /// Check if player is immune to current weather
    pub fn is_immune_to_current_weather(&self) -> bool {
        if self.weather_ward_turns > 0 {
            return true;
        }

        for artifact in &self.equipped_artifacts {
            if artifact.provides_total_immunity() {
                return true;
            }
            if let Some(immune_category) = artifact.immunity() {
                if self.current_weather.category() == immune_category {
                    return true;
                }
            }
        }

        false
    }

    /// Force a specific weather for a dungeon theme
    pub fn set_for_theme(&mut self, theme: &str, rng: &mut impl Rng) {
        match theme {
            "Frozen Caverns" | "Ice Cavern" | "Frost Keep" => {
                let weather = *[
                    WeatherType::LightSnow,
                    WeatherType::HeavySnow,
                    WeatherType::Blizzard,
                    WeatherType::Fog,
                ]
                .choose(rng)
                .unwrap();
                self.set_weather(weather);
            }
            "Ancient Ruins" | "Desert" | "Scorched Wastes" => {
                let weather = *[
                    WeatherType::Clear,
                    WeatherType::DustStorm,
                    WeatherType::Sandstorm,
                ]
                .choose(rng)
                .unwrap();
                self.set_weather(weather);
            }
            "Twisted Caves" | "Cave" | "Deep Tunnels" => {
                let weather = *[WeatherType::Fog, WeatherType::DenseFog, WeatherType::Mist]
                    .choose(rng)
                    .unwrap();
                self.set_weather(weather);
            }
            "Haunted Manor" | "Crypts" | "Graveyard" => {
                let weather = *[
                    WeatherType::DenseFog,
                    WeatherType::SpiritRain,
                    WeatherType::BloodMoon,
                ]
                .choose(rng)
                .unwrap();
                self.set_weather(weather);
            }
            "Mage Tower" | "Arcane Sanctum" => {
                let weather = *[
                    WeatherType::ManaStorm,
                    WeatherType::ArcaneWind,
                    WeatherType::Aurora,
                ]
                .choose(rng)
                .unwrap();
                self.set_weather(weather);
            }
            "Void Realm" | "Eldritch Domain" => {
                self.set_weather(WeatherType::VoidBreach);
            }
            _ => {
                // Default: weather based on season
                if rng.gen_bool(0.3) {
                    self.transition_weather(rng);
                }
            }
        }
    }

    /// Get the current visibility modifier (accounting for intensity)
    pub fn visibility_modifier(&self) -> f32 {
        if !self.enabled || self.is_immune_to_current_weather() {
            return 1.0;
        }

        // If transitioning, interpolate
        if let Some(ref transition) = self.transition {
            return transition.visibility_modifier();
        }

        let base = self.current_weather.visibility_modifier();
        let intensity_factor = self.intensity.effect_multiplier();
        // Interpolate between 1.0 (no effect) and base modifier
        1.0 - ((1.0 - base) * intensity_factor).min(0.8) // Cap at 80% visibility reduction
    }

    /// Get the current accuracy modifier
    pub fn accuracy_modifier(&self) -> f32 {
        if !self.enabled || self.is_immune_to_current_weather() {
            return 1.0;
        }
        let base = self.current_weather.accuracy_modifier();
        let intensity_factor = self.intensity.effect_multiplier();
        1.0 - ((1.0 - base) * intensity_factor).min(0.6) // Cap at 60% accuracy reduction
    }

    /// Get the current movement modifier
    pub fn movement_modifier(&self) -> f32 {
        if !self.enabled || self.is_immune_to_current_weather() {
            return 1.0;
        }
        let base = self.current_weather.movement_modifier();
        let intensity_factor = self.intensity.effect_multiplier();
        1.0 - ((1.0 - base) * intensity_factor).min(0.4) // Cap at 40% movement reduction
    }

    /// Get environmental damage per turn
    pub fn environmental_damage(&self) -> i32 {
        if !self.enabled || self.is_immune_to_current_weather() {
            return 0;
        }
        let base = self.current_weather.environmental_damage();
        match self.intensity {
            WeatherIntensity::Light => base / 2,
            WeatherIntensity::Normal => base,
            WeatherIntensity::Heavy => base * 2,
            WeatherIntensity::Extreme => base * 3,
        }
    }

    /// Calculate modified view radius based on weather
    pub fn modified_view_radius(&self, base_radius: i32) -> i32 {
        let modifier = self.visibility_modifier();
        ((base_radius as f32) * modifier).max(2.0) as i32
    }

    /// Check if fire effects should be extinguished
    pub fn should_extinguish_fire(&self) -> bool {
        self.enabled
            && !self.is_immune_to_current_weather()
            && self.current_weather.extinguishes_fire()
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
            DamageType::Dark => self.current_weather.dark_damage_modifier(),
            DamageType::Holy => self.current_weather.holy_damage_modifier(),
            DamageType::Arcane => self.current_weather.arcane_damage_modifier(),
            DamageType::Physical => 1.0,
            DamageType::Poison => 1.0,
            DamageType::Void => 1.0,
            DamageType::Nature => 1.0,
        }
    }

    /// Get mana regeneration modifier
    pub fn mana_regen_modifier(&self) -> f32 {
        if !self.enabled {
            return 1.0;
        }
        self.current_weather.mana_regen_modifier() * self.intensity.effect_multiplier()
    }

    /// Get health regeneration modifier
    pub fn health_regen_modifier(&self) -> f32 {
        if !self.enabled {
            return 1.0;
        }
        self.current_weather.health_regen_modifier()
    }

    /// Get a status message describing current weather
    pub fn status_message(&self) -> String {
        if !self.enabled || self.current_weather == WeatherType::Clear {
            return String::new();
        }

        let immunity_note = if self.is_immune_to_current_weather() {
            " (Protected)"
        } else {
            ""
        };

        format!(
            "{} {} - {}{}",
            self.intensity.name(),
            self.current_weather.name(),
            self.current_weather.description(),
            immunity_note
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

    /// Get special creature spawns for current weather
    pub fn get_special_spawns(&self) -> Vec<WeatherSpawn> {
        WeatherSpawn::for_weather(self.current_weather)
    }

    /// Get weather forecast
    pub fn forecast(&mut self, count: usize, rng: &mut impl Rng) -> Vec<WeatherForecast> {
        self.forecaster
            .generate_forecasts(self.current_weather, self.season, count, rng)
    }

    /// Get NPC hints about weather
    pub fn get_npc_hints(&self, rng: &mut impl Rng) -> Vec<String> {
        let mut hints = self.forecaster.get_hints(rng);

        // Add seasonal hints
        match self.season {
            Season::Spring => hints.push("The spring rains are upon us.".to_string()),
            Season::Summer => hints.push("These hot summer days seem endless.".to_string()),
            Season::Autumn => hints.push("Autumn fog rolls in from the hills.".to_string()),
            Season::Winter => hints.push("Bundle up, winter storms approach.".to_string()),
        }

        // Add magical weather warnings
        if self.current_weather.is_magical() {
            hints.push(format!(
                "Strange magic is afoot - {} fills the air!",
                self.current_weather.name().to_lowercase()
            ));
        }

        hints
    }

    /// Get full weather effects
    pub fn get_effects(&self) -> WeatherEffects {
        WeatherEffects::from_weather_system(self)
    }

    /// Get combat modifiers
    pub fn get_combat_modifiers(&self) -> WeatherCombatModifiers {
        WeatherCombatModifiers::from_weather(self)
    }

    /// Get temperature based on season and weather
    pub fn temperature(&self) -> f32 {
        let base = self.season.temperature_modifier();
        let weather_mod = match self.current_weather {
            WeatherType::Blizzard | WeatherType::HeavySnow => -0.3,
            WeatherType::LightSnow | WeatherType::Hail | WeatherType::Sleet => -0.2,
            WeatherType::LightRain | WeatherType::Fog => -0.1,
            WeatherType::Clear => 0.1,
            WeatherType::Sandstorm | WeatherType::DustStorm => 0.2,
            WeatherType::PhoenixFire => 0.4,
            WeatherType::VoidBreach => -0.4,
            _ => 0.0,
        };
        (base + weather_mod).clamp(-1.0, 1.0)
    }

    /// Check if it's currently raining (any rain type)
    pub fn is_raining(&self) -> bool {
        matches!(
            self.current_weather,
            WeatherType::LightRain
                | WeatherType::HeavyRain
                | WeatherType::Thunderstorm
                | WeatherType::Hurricane
                | WeatherType::SpiritRain
        )
    }

    /// Check if it's currently snowing (any snow type)
    pub fn is_snowing(&self) -> bool {
        matches!(
            self.current_weather,
            WeatherType::LightSnow | WeatherType::HeavySnow | WeatherType::Blizzard
        )
    }

    /// Check if visibility is severely reduced
    pub fn is_low_visibility(&self) -> bool {
        self.visibility_modifier() < 0.5
    }

    /// Check if movement is severely impaired
    pub fn is_movement_impaired(&self) -> bool {
        self.movement_modifier() < 0.7
    }
}

impl Default for WeatherSystem {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

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

        weather.set_weather(WeatherType::DenseFog);
        assert!(weather.visibility_modifier() < 0.5);
    }

    #[test]
    fn test_weather_intensity() {
        let mut weather = WeatherSystem::new();

        weather.set_weather_with_intensity(WeatherType::HeavyRain, WeatherIntensity::Light);
        let light_vis = weather.visibility_modifier();

        weather.set_weather_with_intensity(WeatherType::HeavyRain, WeatherIntensity::Heavy);
        let heavy_vis = weather.visibility_modifier();

        assert!(heavy_vis < light_vis);
    }

    #[test]
    fn test_fire_extinguish() {
        let mut weather = WeatherSystem::new();

        weather.set_weather(WeatherType::Clear);
        assert!(!weather.should_extinguish_fire());

        weather.set_weather(WeatherType::HeavyRain);
        assert!(weather.should_extinguish_fire());

        weather.set_weather(WeatherType::Thunderstorm);
        assert!(weather.should_extinguish_fire());
    }

    #[test]
    fn test_damage_modifiers() {
        let mut weather = WeatherSystem::new();

        weather.set_weather(WeatherType::HeavyRain);
        assert!(weather.damage_modifier(DamageType::Fire) < 1.0);
        assert!(weather.damage_modifier(DamageType::Lightning) > 1.0);

        weather.set_weather(WeatherType::Blizzard);
        assert!(weather.damage_modifier(DamageType::Ice) > 1.0);
    }

    #[test]
    fn test_view_radius_modification() {
        let mut weather = WeatherSystem::new();
        let base_radius = 10;

        weather.set_weather(WeatherType::Clear);
        assert_eq!(weather.modified_view_radius(base_radius), 10);

        weather.set_weather_with_intensity(WeatherType::DenseFog, WeatherIntensity::Heavy);
        let fog_radius = weather.modified_view_radius(base_radius);
        assert!(fog_radius < base_radius);
        assert!(fog_radius >= 2); // Minimum visibility
    }

    #[test]
    fn test_combat_modifiers() {
        let mut weather = WeatherSystem::new();
        weather.set_weather(WeatherType::HeavyRain);

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

    #[test]
    fn test_seasons() {
        assert_eq!(Season::from_day(0), Season::Spring);
        assert_eq!(Season::from_day(91), Season::Summer);
        assert_eq!(Season::from_day(182), Season::Autumn);
        assert_eq!(Season::from_day(273), Season::Winter);
        assert_eq!(Season::from_day(365), Season::Spring); // Wraps around
    }

    #[test]
    fn test_season_weather_weights() {
        let spring_weights = Season::Spring.weather_weights();
        let winter_weights = Season::Winter.weather_weights();

        // Spring should have higher rain weights
        assert!(spring_weights.get(&WeatherType::LightRain).unwrap_or(&0)
            > winter_weights.get(&WeatherType::LightRain).unwrap_or(&0));

        // Winter should have snow
        assert!(winter_weights.get(&WeatherType::HeavySnow).unwrap_or(&0) > &0);
    }

    #[test]
    fn test_weather_categories() {
        assert_eq!(WeatherType::Clear.category(), WeatherCategory::Normal);
        assert_eq!(WeatherType::HeavyRain.category(), WeatherCategory::Rain);
        assert_eq!(WeatherType::Blizzard.category(), WeatherCategory::Snow);
        assert_eq!(WeatherType::ManaStorm.category(), WeatherCategory::Magical);
    }

    #[test]
    fn test_weather_artifact_immunity() {
        let mut weather = WeatherSystem::new();
        weather.set_weather(WeatherType::Blizzard);

        assert!(!weather.is_immune_to_current_weather());

        weather.equip_artifact(WeatherArtifact::FrostCrown);
        assert!(weather.is_immune_to_current_weather());

        weather.unequip_artifact(WeatherArtifact::FrostCrown);
        assert!(!weather.is_immune_to_current_weather());

        weather.equip_artifact(WeatherArtifact::WeatherShield);
        assert!(weather.is_immune_to_current_weather());

        // Should be immune to any weather
        weather.set_weather(WeatherType::Sandstorm);
        assert!(weather.is_immune_to_current_weather());
    }

    #[test]
    fn test_weather_ward() {
        let mut weather = WeatherSystem::new();
        weather.set_weather(WeatherType::Sandstorm);
        weather.weather_ward_turns = 10;

        assert!(weather.is_immune_to_current_weather());
        assert_eq!(weather.environmental_damage(), 0);
    }

    #[test]
    fn test_weather_lock() {
        let mut weather = WeatherSystem::new();
        weather.set_weather(WeatherType::Clear);
        weather.lock_weather(100);

        assert!(weather.weather_locked);
        assert_eq!(weather.lock_turns_remaining, 100);

        weather.unlock_weather();
        assert!(!weather.weather_locked);
    }

    #[test]
    fn test_weather_spells() {
        let spell = WeatherSpell::SummonStorm;

        assert_eq!(spell.name(), "Summon Storm");
        assert_eq!(spell.resulting_weather(), Some(WeatherType::Thunderstorm));
        assert!(spell.mana_cost() > 0);
        assert!(spell.required_skill() > 0);
    }

    #[test]
    fn test_magical_weather() {
        assert!(WeatherType::ManaStorm.is_magical());
        assert!(WeatherType::BloodMoon.is_magical());
        assert!(WeatherType::VoidBreach.is_magical());
        assert!(!WeatherType::Clear.is_magical());
        assert!(!WeatherType::Thunderstorm.is_magical());
    }

    #[test]
    fn test_dangerous_weather() {
        assert!(WeatherType::Hurricane.is_dangerous());
        assert!(WeatherType::Tornado.is_dangerous());
        assert!(WeatherType::VoidBreach.is_dangerous());
        assert!(!WeatherType::Clear.is_dangerous());
        assert!(!WeatherType::LightRain.is_dangerous());
    }

    #[test]
    fn test_special_spawns() {
        let spawns = WeatherSpawn::for_weather(WeatherType::BloodMoon);
        assert!(!spawns.is_empty());

        let spawns = WeatherSpawn::for_weather(WeatherType::Clear);
        assert!(spawns.is_empty());
    }

    #[test]
    fn test_weather_transition() {
        let mut transition =
            WeatherTransition::gradual(WeatherType::Clear, WeatherType::HeavyRain, 10);

        assert!(!transition.is_complete());
        assert_eq!(transition.progress, 0.0);

        for _ in 0..10 {
            transition.update();
        }

        assert!(transition.is_complete());
        assert_eq!(transition.progress, 1.0);
    }

    #[test]
    fn test_temperature() {
        let mut weather = WeatherSystem::new();

        weather.set_season(Season::Summer);
        weather.set_weather(WeatherType::Clear);
        let summer_temp = weather.temperature();

        weather.set_season(Season::Winter);
        weather.set_weather(WeatherType::Blizzard);
        let winter_temp = weather.temperature();

        assert!(summer_temp > winter_temp);
    }

    #[test]
    fn test_weather_history() {
        let mut weather = WeatherSystem::new();

        weather.set_weather(WeatherType::Clear);
        weather.set_weather(WeatherType::LightRain);
        weather.set_weather(WeatherType::HeavyRain);

        assert_eq!(weather.weather_history.len(), 2); // Clear and LightRain
        assert_eq!(weather.weather_history[0].0, WeatherType::Clear);
        assert_eq!(weather.weather_history[1].0, WeatherType::LightRain);
    }

    #[test]
    fn test_weather_type_count() {
        // Verify we have at least 30 weather types as required
        assert!(WeatherType::all().len() >= 30);
    }

    #[test]
    fn test_forecast() {
        let mut weather = WeatherSystem::new();
        weather.forecaster.skill_level = 50;

        let mut rng = rand::thread_rng();
        let forecasts = weather.forecast(3, &mut rng);

        assert_eq!(forecasts.len(), 3);
        for forecast in &forecasts {
            assert!(forecast.confidence > 0.0);
            assert!(forecast.confidence <= 1.0);
        }
    }
}
