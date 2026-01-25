//! Day/Night Cycle System for ShadowCrypt
//!
//! This module provides a comprehensive time system including:
//! - Time progression with hours, minutes, and days
//! - Day/night phases affecting gameplay
//! - Enemy behavior modifications based on time
//! - Visibility adjustments for different times of day
//! - Time-based events and triggers

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Number of game minutes per real turn
pub const MINUTES_PER_TURN: u32 = 5;

/// Total minutes in a game day
pub const MINUTES_PER_DAY: u32 = 24 * 60;

/// Base view radius during daytime
pub const BASE_VIEW_RADIUS: i32 = 10;

/// Minimum view radius during deep night
pub const MIN_VIEW_RADIUS: i32 = 4;

/// Phases of the day cycle
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum TimePhase {
    /// Early morning (5:00 - 7:59) - Transition from night
    Dawn,
    /// Full daylight (8:00 - 17:59) - Best visibility
    Day,
    /// Evening transition (18:00 - 20:59) - Light fading
    Dusk,
    /// Nighttime (21:00 - 4:59) - Reduced visibility, stronger enemies
    Night,
    /// Darkest hour (0:00 - 2:59) - Special events, maximum danger
    Midnight,
}

impl TimePhase {
    /// Returns the display name of this phase
    pub fn name(&self) -> &'static str {
        match self {
            Self::Dawn => "Dawn",
            Self::Day => "Day",
            Self::Dusk => "Dusk",
            Self::Night => "Night",
            Self::Midnight => "Midnight",
        }
    }

    /// Returns a description of the current time phase
    pub fn description(&self) -> &'static str {
        match self {
            Self::Dawn => "The first rays of light pierce the darkness",
            Self::Day => "Bright light illuminates the dungeon",
            Self::Dusk => "Shadows grow longer as darkness approaches",
            Self::Night => "Darkness descends, creatures of the night awaken",
            Self::Midnight => "The witching hour - dark forces are at their peak",
        }
    }

    /// Returns a color index for UI rendering
    pub fn color_index(&self) -> u8 {
        match self {
            Self::Dawn => 11,    // Yellow/Orange
            Self::Day => 2,      // White/Bright
            Self::Dusk => 12,    // Dark Yellow/Orange
            Self::Night => 7,    // Blue/Dark
            Self::Midnight => 4, // Dark Red/Purple
        }
    }

    /// Returns the ambient light level (0.0 to 1.0)
    pub fn light_level(&self) -> f32 {
        match self {
            Self::Dawn => 0.6,
            Self::Day => 1.0,
            Self::Dusk => 0.5,
            Self::Night => 0.3,
            Self::Midnight => 0.15,
        }
    }
}

/// Represents time-based events that can trigger
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum TimeEvent {
    /// Triggered when transitioning to a new phase
    PhaseChange(TimePhase),
    /// New day has begun
    NewDay,
    /// Midnight spawning event
    MidnightSpawn,
    /// Dawn healing bonus
    DawnBlessing,
    /// Dusk warning about approaching night
    DuskWarning,
    /// Solar eclipse (rare event)
    SolarEclipse,
    /// Blood moon (rare event at night)
    BloodMoon,
    /// Witching hour begins
    WitchingHour,
    /// Rooster crow at dawn (restores some hunger)
    RoosterCrow,
}

impl TimeEvent {
    /// Returns the display name of this event
    pub fn name(&self) -> &'static str {
        match self {
            Self::PhaseChange(phase) => phase.name(),
            Self::NewDay => "New Day",
            Self::MidnightSpawn => "Midnight Spawning",
            Self::DawnBlessing => "Dawn's Blessing",
            Self::DuskWarning => "Dusk Falls",
            Self::SolarEclipse => "Solar Eclipse",
            Self::BloodMoon => "Blood Moon",
            Self::WitchingHour => "Witching Hour",
            Self::RoosterCrow => "Rooster's Call",
        }
    }

    /// Returns a message to display when this event triggers
    pub fn message(&self) -> &'static str {
        match self {
            Self::PhaseChange(TimePhase::Dawn) => "Dawn breaks! The darkness recedes.",
            Self::PhaseChange(TimePhase::Day) => "The sun rises fully. Light fills the dungeon.",
            Self::PhaseChange(TimePhase::Dusk) => "Dusk approaches. Shadows lengthen.",
            Self::PhaseChange(TimePhase::Night) => "Night falls! Beware of nocturnal creatures.",
            Self::PhaseChange(TimePhase::Midnight) => "Midnight! Dark forces reach their peak!",
            Self::NewDay => "A new day begins in the dungeon.",
            Self::MidnightSpawn => "Dark energy coalesces... creatures emerge!",
            Self::DawnBlessing => "The morning light restores your vitality!",
            Self::DuskWarning => "Night approaches. Find shelter or prepare for battle!",
            Self::SolarEclipse => "The sun goes dark! An eclipse shrouds the land!",
            Self::BloodMoon => "A blood moon rises! Enemies grow stronger!",
            Self::WitchingHour => "The witching hour begins. Magic is amplified!",
            Self::RoosterCrow => "A distant rooster crows. You feel refreshed.",
        }
    }

    /// Returns a color index for the event message
    pub fn color_index(&self) -> u8 {
        match self {
            Self::PhaseChange(TimePhase::Dawn) => 11,
            Self::PhaseChange(TimePhase::Day) => 2,
            Self::PhaseChange(TimePhase::Dusk) => 12,
            Self::PhaseChange(TimePhase::Night) => 7,
            Self::PhaseChange(TimePhase::Midnight) => 4,
            Self::NewDay => 9,
            Self::MidnightSpawn => 3,
            Self::DawnBlessing => 13,
            Self::DuskWarning => 11,
            Self::SolarEclipse => 0,
            Self::BloodMoon => 3,
            Self::WitchingHour => 14,
            Self::RoosterCrow => 11,
        }
    }
}

/// Categories of creatures based on their activity patterns
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum CreatureActivity {
    /// Active during day, weaker at night
    Diurnal,
    /// Active at night, weaker during day
    Nocturnal,
    /// Active at dawn and dusk
    Crepuscular,
    /// Equally active at all times
    Constant,
    /// Only appears during specific phases
    PhaseLocked(TimePhase),
}

impl CreatureActivity {
    /// Returns the activity multiplier for a given time phase
    /// Values > 1.0 mean stronger, < 1.0 mean weaker
    pub fn get_multiplier(&self, phase: TimePhase) -> f32 {
        match self {
            Self::Diurnal => match phase {
                TimePhase::Day => 1.0,
                TimePhase::Dawn | TimePhase::Dusk => 0.85,
                TimePhase::Night => 0.6,
                TimePhase::Midnight => 0.5,
            },
            Self::Nocturnal => match phase {
                TimePhase::Midnight => 1.5,
                TimePhase::Night => 1.3,
                TimePhase::Dusk | TimePhase::Dawn => 1.0,
                TimePhase::Day => 0.7,
            },
            Self::Crepuscular => match phase {
                TimePhase::Dawn | TimePhase::Dusk => 1.3,
                TimePhase::Day | TimePhase::Night => 0.9,
                TimePhase::Midnight => 0.8,
            },
            Self::Constant => 1.0,
            Self::PhaseLocked(locked_phase) => {
                if phase == *locked_phase {
                    1.5
                } else {
                    0.5
                }
            }
        }
    }

    /// Returns whether a creature is active (can spawn/move) during this phase
    pub fn is_active(&self, phase: TimePhase) -> bool {
        match self {
            Self::Diurnal => matches!(phase, TimePhase::Day | TimePhase::Dawn | TimePhase::Dusk),
            Self::Nocturnal => matches!(
                phase,
                TimePhase::Night | TimePhase::Midnight | TimePhase::Dusk
            ),
            Self::Crepuscular => matches!(phase, TimePhase::Dawn | TimePhase::Dusk),
            Self::Constant => true,
            Self::PhaseLocked(locked_phase) => phase == *locked_phase,
        }
    }
}

/// Modifiers applied based on time of day
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TimeModifiers {
    /// Multiplier for enemy damage
    pub enemy_damage_mult: f32,
    /// Multiplier for enemy health
    pub enemy_health_mult: f32,
    /// Multiplier for enemy detection range
    pub enemy_detection_mult: f32,
    /// View radius modifier (added to base)
    pub view_radius_mod: i32,
    /// Multiplier for XP gained
    pub xp_mult: f32,
    /// Multiplier for item drop rates
    pub drop_rate_mult: f32,
    /// Chance for special spawns (0.0 to 1.0)
    pub special_spawn_chance: f32,
    /// Mana regeneration multiplier
    pub mana_regen_mult: f32,
    /// Healing effectiveness multiplier
    pub healing_mult: f32,
}

impl TimeModifiers {
    /// Create modifiers for a given time phase
    pub fn for_phase(phase: TimePhase) -> Self {
        match phase {
            TimePhase::Dawn => Self {
                enemy_damage_mult: 0.9,
                enemy_health_mult: 0.95,
                enemy_detection_mult: 0.85,
                view_radius_mod: -2,
                xp_mult: 1.0,
                drop_rate_mult: 1.0,
                special_spawn_chance: 0.05,
                mana_regen_mult: 1.2,
                healing_mult: 1.3,
            },
            TimePhase::Day => Self {
                enemy_damage_mult: 0.85,
                enemy_health_mult: 0.9,
                enemy_detection_mult: 1.2,
                view_radius_mod: 0,
                xp_mult: 1.0,
                drop_rate_mult: 1.0,
                special_spawn_chance: 0.02,
                mana_regen_mult: 1.0,
                healing_mult: 1.1,
            },
            TimePhase::Dusk => Self {
                enemy_damage_mult: 1.0,
                enemy_health_mult: 1.0,
                enemy_detection_mult: 1.0,
                view_radius_mod: -3,
                xp_mult: 1.1,
                drop_rate_mult: 1.1,
                special_spawn_chance: 0.08,
                mana_regen_mult: 1.1,
                healing_mult: 1.0,
            },
            TimePhase::Night => Self {
                enemy_damage_mult: 1.2,
                enemy_health_mult: 1.15,
                enemy_detection_mult: 0.8,
                view_radius_mod: -5,
                xp_mult: 1.25,
                drop_rate_mult: 1.2,
                special_spawn_chance: 0.15,
                mana_regen_mult: 1.3,
                healing_mult: 0.9,
            },
            TimePhase::Midnight => Self {
                enemy_damage_mult: 1.4,
                enemy_health_mult: 1.3,
                enemy_detection_mult: 0.7,
                view_radius_mod: -6,
                xp_mult: 1.5,
                drop_rate_mult: 1.4,
                special_spawn_chance: 0.25,
                mana_regen_mult: 1.5,
                healing_mult: 0.8,
            },
        }
    }

    /// Apply blood moon modifiers (multiplies existing values)
    pub fn apply_blood_moon(&mut self) {
        self.enemy_damage_mult *= 1.5;
        self.enemy_health_mult *= 1.5;
        self.xp_mult *= 2.0;
        self.drop_rate_mult *= 2.0;
        self.special_spawn_chance = (self.special_spawn_chance * 3.0).min(1.0);
    }

    /// Apply eclipse modifiers
    pub fn apply_eclipse(&mut self) {
        self.view_radius_mod = -8;
        self.enemy_detection_mult *= 0.5;
        self.mana_regen_mult *= 2.0;
        self.special_spawn_chance = 0.5;
    }
}

impl Default for TimeModifiers {
    fn default() -> Self {
        Self::for_phase(TimePhase::Day)
    }
}

/// The main time system tracking game time
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TimeSystem {
    /// Current minute of the day (0-1439)
    pub current_minute: u32,
    /// Current day number
    pub current_day: u32,
    /// Current time phase
    pub phase: TimePhase,
    /// Previous phase (for detecting transitions)
    previous_phase: TimePhase,
    /// Active time-based effects
    pub active_effects: HashMap<TimeEffect, u32>,
    /// Whether a blood moon is active
    pub blood_moon_active: bool,
    /// Whether an eclipse is active
    pub eclipse_active: bool,
    /// Turns until blood moon ends
    blood_moon_duration: u32,
    /// Turns until eclipse ends
    eclipse_duration: u32,
    /// Random seed for special events
    event_seed: u32,
}

/// Temporary time-based effects
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum TimeEffect {
    /// Torch light bonus to view radius
    TorchLight,
    /// Magical darkness penalty
    MagicalDarkness,
    /// True sight (ignores darkness)
    TrueSight,
    /// Night vision (reduced darkness penalty)
    NightVision,
    /// Sun sensitivity (penalty during day)
    SunSensitivity,
}

impl TimeEffect {
    /// Returns the view radius modifier for this effect
    pub fn view_modifier(&self) -> i32 {
        match self {
            Self::TorchLight => 3,
            Self::MagicalDarkness => -4,
            Self::TrueSight => 10,
            Self::NightVision => 4,
            Self::SunSensitivity => -2,
        }
    }
}

impl TimeSystem {
    /// Create a new time system starting at dawn
    pub fn new() -> Self {
        Self {
            current_minute: 5 * 60, // Start at 5:00 AM (dawn)
            current_day: 1,
            phase: TimePhase::Dawn,
            previous_phase: TimePhase::Night,
            active_effects: HashMap::new(),
            blood_moon_active: false,
            eclipse_active: false,
            blood_moon_duration: 0,
            eclipse_duration: 0,
            event_seed: 42,
        }
    }

    /// Create a time system starting at a specific hour
    pub fn new_at_hour(hour: u32) -> Self {
        let minute = (hour % 24) * 60;
        let mut system = Self::new();
        system.current_minute = minute;
        system.phase = Self::calculate_phase(minute);
        system.previous_phase = system.phase;
        system
    }

    /// Calculate the time phase for a given minute
    fn calculate_phase(minute: u32) -> TimePhase {
        let hour = minute / 60;
        match hour {
            0..=2 => TimePhase::Midnight,
            3..=4 => TimePhase::Night,
            5..=7 => TimePhase::Dawn,
            8..=17 => TimePhase::Day,
            18..=20 => TimePhase::Dusk,
            _ => TimePhase::Night,
        }
    }

    /// Advance time by one turn
    /// Returns a list of events that occurred
    pub fn advance_turn(&mut self) -> Vec<TimeEvent> {
        let mut events = Vec::new();

        self.previous_phase = self.phase;
        self.current_minute = (self.current_minute + MINUTES_PER_TURN) % MINUTES_PER_DAY;

        // Check for new day
        if self.current_minute < MINUTES_PER_TURN {
            self.current_day += 1;
            events.push(TimeEvent::NewDay);
            self.event_seed = self.event_seed.wrapping_mul(1103515245).wrapping_add(12345);
        }

        // Update phase
        self.phase = Self::calculate_phase(self.current_minute);

        // Check for phase transitions
        if self.phase != self.previous_phase {
            events.push(TimeEvent::PhaseChange(self.phase));

            // Phase-specific events
            match self.phase {
                TimePhase::Dawn => {
                    events.push(TimeEvent::DawnBlessing);
                    events.push(TimeEvent::RoosterCrow);
                    self.blood_moon_active = false;
                }
                TimePhase::Dusk => {
                    events.push(TimeEvent::DuskWarning);
                }
                TimePhase::Midnight => {
                    events.push(TimeEvent::WitchingHour);
                    // Check for midnight spawn
                    if self.should_trigger_event(0.3) {
                        events.push(TimeEvent::MidnightSpawn);
                    }
                    // Check for blood moon
                    if self.should_trigger_event(0.1) && !self.blood_moon_active {
                        self.blood_moon_active = true;
                        self.blood_moon_duration = 60; // Lasts 60 turns
                        events.push(TimeEvent::BloodMoon);
                    }
                }
                TimePhase::Day => {
                    // Check for eclipse (rare)
                    if self.current_day > 5
                        && self.should_trigger_event(0.02)
                        && !self.eclipse_active
                    {
                        self.eclipse_active = true;
                        self.eclipse_duration = 30;
                        events.push(TimeEvent::SolarEclipse);
                    }
                }
                _ => {}
            }
        }

        // Tick down special event durations
        if self.blood_moon_active {
            self.blood_moon_duration = self.blood_moon_duration.saturating_sub(1);
            if self.blood_moon_duration == 0 {
                self.blood_moon_active = false;
            }
        }
        if self.eclipse_active {
            self.eclipse_duration = self.eclipse_duration.saturating_sub(1);
            if self.eclipse_duration == 0 {
                self.eclipse_active = false;
            }
        }

        // Tick down active effects
        let mut expired = Vec::new();
        for (effect, duration) in self.active_effects.iter_mut() {
            *duration = duration.saturating_sub(1);
            if *duration == 0 {
                expired.push(*effect);
            }
        }
        for effect in expired {
            self.active_effects.remove(&effect);
        }

        events
    }

    /// Simple deterministic "random" for events based on day and seed
    fn should_trigger_event(&self, base_chance: f32) -> bool {
        let hash = self
            .event_seed
            .wrapping_mul(self.current_day)
            .wrapping_add(self.current_minute);
        let normalized = (hash % 1000) as f32 / 1000.0;
        normalized < base_chance
    }

    /// Get the current time as (hour, minute)
    pub fn get_time(&self) -> (u32, u32) {
        (self.current_minute / 60, self.current_minute % 60)
    }

    /// Get formatted time string (HH:MM)
    pub fn get_time_string(&self) -> String {
        let (hour, minute) = self.get_time();
        format!("{:02}:{:02}", hour, minute)
    }

    /// Get a descriptive time string
    pub fn get_time_description(&self) -> String {
        let (hour, minute) = self.get_time();
        let period = if hour < 12 { "AM" } else { "PM" };
        let display_hour = if hour == 0 {
            12
        } else if hour > 12 {
            hour - 12
        } else {
            hour
        };
        format!(
            "Day {} - {:02}:{:02} {} ({})",
            self.current_day,
            display_hour,
            minute,
            period,
            self.phase.name()
        )
    }

    /// Get the current modifiers based on time and active effects
    pub fn get_modifiers(&self) -> TimeModifiers {
        let mut mods = TimeModifiers::for_phase(self.phase);

        if self.blood_moon_active {
            mods.apply_blood_moon();
        }
        if self.eclipse_active {
            mods.apply_eclipse();
        }

        mods
    }

    /// Calculate the effective view radius
    pub fn get_effective_view_radius(&self) -> i32 {
        let base_mod = self.get_modifiers().view_radius_mod;
        let mut radius = BASE_VIEW_RADIUS + base_mod;

        // Apply active effects
        for (effect, _) in &self.active_effects {
            radius += effect.view_modifier();
        }

        radius.max(MIN_VIEW_RADIUS)
    }

    /// Add a temporary time effect
    pub fn add_effect(&mut self, effect: TimeEffect, duration: u32) {
        self.active_effects.insert(effect, duration);
    }

    /// Remove a time effect
    pub fn remove_effect(&mut self, effect: TimeEffect) {
        self.active_effects.remove(&effect);
    }

    /// Check if an effect is active
    pub fn has_effect(&self, effect: TimeEffect) -> bool {
        self.active_effects.contains_key(&effect)
    }

    /// Get enemy stat multiplier based on current time and creature type
    pub fn get_enemy_multiplier(&self, activity: CreatureActivity) -> f32 {
        let base_mult = activity.get_multiplier(self.phase);
        let mods = self.get_modifiers();

        // Combine with general time modifiers
        base_mult * ((mods.enemy_damage_mult + mods.enemy_health_mult) / 2.0)
    }

    /// Check if a creature type should be active now
    pub fn is_creature_active(&self, activity: CreatureActivity) -> bool {
        activity.is_active(self.phase)
    }

    /// Get the XP multiplier for current time
    pub fn get_xp_multiplier(&self) -> f32 {
        self.get_modifiers().xp_mult
    }

    /// Get the current phase
    pub fn current_phase(&self) -> TimePhase {
        self.phase
    }

    /// Check if it's currently nighttime (Night or Midnight)
    pub fn is_night(&self) -> bool {
        matches!(self.phase, TimePhase::Night | TimePhase::Midnight)
    }

    /// Check if it's currently daytime (Day or Dawn)
    pub fn is_day(&self) -> bool {
        matches!(self.phase, TimePhase::Day | TimePhase::Dawn)
    }

    /// Get special spawn chance for current time
    pub fn get_special_spawn_chance(&self) -> f32 {
        self.get_modifiers().special_spawn_chance
    }
}

impl Default for TimeSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for creature activity classification
pub mod creature_utils {
    use super::CreatureActivity;

    /// Classify an enemy kind string into an activity pattern
    /// This can be used to map existing enemy types to activity patterns
    pub fn classify_creature(kind_name: &str) -> CreatureActivity {
        let name_lower = kind_name.to_lowercase();

        // Undead and shadow creatures are nocturnal
        if name_lower.contains("ghost")
            || name_lower.contains("wraith")
            || name_lower.contains("vampire")
            || name_lower.contains("zombie")
            || name_lower.contains("skeleton")
            || name_lower.contains("lich")
            || name_lower.contains("shadow")
            || name_lower.contains("banshee")
            || name_lower.contains("death")
        {
            return CreatureActivity::Nocturnal;
        }

        // Bat, owl, and night creatures
        if name_lower.contains("bat")
            || name_lower.contains("owl")
            || name_lower.contains("nightstalker")
            || name_lower.contains("moonbeast")
        {
            return CreatureActivity::Nocturnal;
        }

        // Wolf and some predators are crepuscular
        if name_lower.contains("wolf") && !name_lower.contains("dire") {
            return CreatureActivity::Crepuscular;
        }

        // Demons and elementals are constant
        if name_lower.contains("demon")
            || name_lower.contains("elemental")
            || name_lower.contains("golem")
            || name_lower.contains("construct")
        {
            return CreatureActivity::Constant;
        }

        // Forest creatures are typically diurnal
        if name_lower.contains("treant")
            || name_lower.contains("ent")
            || name_lower.contains("druid")
            || name_lower.contains("forest")
            || name_lower.contains("boar")
        {
            return CreatureActivity::Diurnal;
        }

        // Default to constant for dungeon dwellers
        CreatureActivity::Constant
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_system_creation() {
        let time = TimeSystem::new();
        assert_eq!(time.phase, TimePhase::Dawn);
        assert_eq!(time.current_day, 1);
    }

    #[test]
    fn test_time_advancement() {
        let mut time = TimeSystem::new();
        let initial_minute = time.current_minute;

        time.advance_turn();

        assert_eq!(time.current_minute, initial_minute + MINUTES_PER_TURN);
    }

    #[test]
    fn test_phase_transitions() {
        let mut time = TimeSystem::new_at_hour(7); // Late dawn
        assert_eq!(time.phase, TimePhase::Dawn);

        // Advance to day (8:00)
        while time.phase == TimePhase::Dawn {
            time.advance_turn();
        }
        assert_eq!(time.phase, TimePhase::Day);
    }

    #[test]
    fn test_view_radius_calculation() {
        let day_time = TimeSystem::new_at_hour(12); // Noon
        let night_time = TimeSystem::new_at_hour(23); // Night

        assert!(day_time.get_effective_view_radius() > night_time.get_effective_view_radius());
    }

    #[test]
    fn test_creature_activity() {
        let nocturnal = CreatureActivity::Nocturnal;

        // Nocturnal creatures should be stronger at night
        assert!(
            nocturnal.get_multiplier(TimePhase::Midnight)
                > nocturnal.get_multiplier(TimePhase::Day)
        );
    }

    #[test]
    fn test_time_effects() {
        let mut time = TimeSystem::new();

        time.add_effect(TimeEffect::TorchLight, 10);
        assert!(time.has_effect(TimeEffect::TorchLight));

        let base_radius = TimeSystem::new().get_effective_view_radius();
        assert!(time.get_effective_view_radius() > base_radius);
    }

    #[test]
    fn test_time_string_formatting() {
        let time = TimeSystem::new_at_hour(14); // 2:00 PM
        let time_str = time.get_time_string();
        assert_eq!(time_str, "14:00");
    }

    #[test]
    fn test_modifiers() {
        let day_mods = TimeModifiers::for_phase(TimePhase::Day);
        let night_mods = TimeModifiers::for_phase(TimePhase::Night);

        // Night should have higher enemy damage
        assert!(night_mods.enemy_damage_mult > day_mods.enemy_damage_mult);
        // Day should have better view radius
        assert!(day_mods.view_radius_mod > night_mods.view_radius_mod);
    }

    #[test]
    fn test_day_night_checks() {
        let dawn = TimeSystem::new_at_hour(6);
        let noon = TimeSystem::new_at_hour(12);
        let dusk = TimeSystem::new_at_hour(19);
        let night = TimeSystem::new_at_hour(23);
        let midnight = TimeSystem::new_at_hour(1);

        assert!(dawn.is_day());
        assert!(noon.is_day());
        assert!(!dusk.is_day());
        assert!(night.is_night());
        assert!(midnight.is_night());
    }
}
