//! Environmental agent implementations
//!
//! These agents control the game world environment:
//! weather, time, seasons, wildlife, and special events.

use super::{Agent, AgentId, AgentKind, AgentState};
use crate::behavior::{Goal, GoalKind, GoalPriority};
use rand::Rng;
use serde::{Serialize, Deserialize};

/// Environmental agent behaviors
pub struct EnvironmentalBehaviors;

impl EnvironmentalBehaviors {
    /// Creates default goals for environmental agents
    pub fn default_goals(kind: AgentKind) -> Vec<Goal> {
        match kind {
            AgentKind::WeatherController => vec![
                Goal::new(GoalKind::ManageWeather, GoalPriority::High),
                Goal::new(GoalKind::CreateAtmosphere, GoalPriority::Medium),
            ],
            AgentKind::DayNightCycle => vec![
                Goal::new(GoalKind::ManageTime, GoalPriority::Critical),
                Goal::new(GoalKind::TriggerEvents, GoalPriority::Medium),
            ],
            AgentKind::SeasonManager => vec![
                Goal::new(GoalKind::ManageSeason, GoalPriority::High),
                Goal::new(GoalKind::AdjustEnvironment, GoalPriority::Medium),
            ],
            AgentKind::WildlifeSpawner => vec![
                Goal::new(GoalKind::SpawnCreatures, GoalPriority::Medium),
                Goal::new(GoalKind::ManagePopulation, GoalPriority::Low),
            ],
            AgentKind::EventSpawner => vec![
                Goal::new(GoalKind::TriggerEvents, GoalPriority::High),
                Goal::new(GoalKind::CreateChallenges, GoalPriority::Medium),
            ],
            AgentKind::Storm => vec![
                Goal::new(GoalKind::CreateHazard, GoalPriority::High),
                Goal::new(GoalKind::Move, GoalPriority::Medium),
            ],
            AgentKind::Earthquake => vec![
                Goal::new(GoalKind::CreateHazard, GoalPriority::Critical),
                Goal::new(GoalKind::DamageArea, GoalPriority::High),
            ],
            _ => vec![
                Goal::new(GoalKind::ManageEnvironment, GoalPriority::Medium),
            ],
        }
    }
}

/// Weather types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Weather {
    Clear,
    Cloudy,
    Rainy,
    Stormy,
    Snowy,
    Foggy,
    Windy,
    Hail,
    Sandstorm,
    Blizzard,
    Thunderstorm,
    AcidRain,
    MagicStorm,
    Eclipse,
}

impl Weather {
    /// Returns effects on gameplay
    pub fn effects(&self) -> WeatherEffects {
        match self {
            Self::Clear => WeatherEffects {
                visibility_modifier: 1.0,
                speed_modifier: 1.0,
                damage_modifier: 1.0,
                special_effect: None,
            },
            Self::Cloudy => WeatherEffects {
                visibility_modifier: 0.9,
                speed_modifier: 1.0,
                damage_modifier: 1.0,
                special_effect: None,
            },
            Self::Rainy => WeatherEffects {
                visibility_modifier: 0.7,
                speed_modifier: 0.9,
                damage_modifier: 1.0,
                special_effect: Some(WeatherSpecialEffect::SlipperyGround),
            },
            Self::Stormy => WeatherEffects {
                visibility_modifier: 0.5,
                speed_modifier: 0.8,
                damage_modifier: 1.1,
                special_effect: Some(WeatherSpecialEffect::Lightning),
            },
            Self::Snowy => WeatherEffects {
                visibility_modifier: 0.6,
                speed_modifier: 0.7,
                damage_modifier: 0.9,
                special_effect: Some(WeatherSpecialEffect::Cold),
            },
            Self::Foggy => WeatherEffects {
                visibility_modifier: 0.3,
                speed_modifier: 1.0,
                damage_modifier: 1.0,
                special_effect: Some(WeatherSpecialEffect::Concealment),
            },
            Self::Windy => WeatherEffects {
                visibility_modifier: 0.9,
                speed_modifier: 0.9,
                damage_modifier: 1.0,
                special_effect: Some(WeatherSpecialEffect::RangedPenalty),
            },
            Self::Blizzard => WeatherEffects {
                visibility_modifier: 0.2,
                speed_modifier: 0.5,
                damage_modifier: 0.8,
                special_effect: Some(WeatherSpecialEffect::Freezing),
            },
            Self::Thunderstorm => WeatherEffects {
                visibility_modifier: 0.4,
                speed_modifier: 0.7,
                damage_modifier: 1.2,
                special_effect: Some(WeatherSpecialEffect::Lightning),
            },
            Self::AcidRain => WeatherEffects {
                visibility_modifier: 0.6,
                speed_modifier: 0.8,
                damage_modifier: 1.0,
                special_effect: Some(WeatherSpecialEffect::AcidDamage),
            },
            Self::MagicStorm => WeatherEffects {
                visibility_modifier: 0.5,
                speed_modifier: 0.9,
                damage_modifier: 1.3,
                special_effect: Some(WeatherSpecialEffect::WildMagic),
            },
            Self::Eclipse => WeatherEffects {
                visibility_modifier: 0.4,
                speed_modifier: 1.0,
                damage_modifier: 1.2,
                special_effect: Some(WeatherSpecialEffect::UndeadPower),
            },
            _ => WeatherEffects::default(),
        }
    }

    /// Returns a random weather based on weights
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        let weights = [
            (Self::Clear, 30),
            (Self::Cloudy, 25),
            (Self::Rainy, 15),
            (Self::Foggy, 10),
            (Self::Windy, 10),
            (Self::Stormy, 5),
            (Self::Snowy, 3),
            (Self::Thunderstorm, 2),
        ];

        let total: u32 = weights.iter().map(|(_, w)| w).sum();
        let roll = rng.gen_range(0..total);
        let mut sum = 0;

        for (weather, weight) in weights {
            sum += weight;
            if roll < sum {
                return weather;
            }
        }

        Self::Clear
    }
}

#[derive(Clone, Debug, Default)]
pub struct WeatherEffects {
    pub visibility_modifier: f32,
    pub speed_modifier: f32,
    pub damage_modifier: f32,
    pub special_effect: Option<WeatherSpecialEffect>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeatherSpecialEffect {
    SlipperyGround,
    Lightning,
    Cold,
    Freezing,
    Concealment,
    RangedPenalty,
    AcidDamage,
    WildMagic,
    UndeadPower,
}

/// Time of day
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimeOfDay {
    Dawn,
    Morning,
    Noon,
    Afternoon,
    Dusk,
    Evening,
    Night,
    Midnight,
}

impl TimeOfDay {
    /// Gets time from hour (0-23)
    pub fn from_hour(hour: u32) -> Self {
        match hour {
            5..=6 => Self::Dawn,
            7..=10 => Self::Morning,
            11..=13 => Self::Noon,
            14..=16 => Self::Afternoon,
            17..=18 => Self::Dusk,
            19..=21 => Self::Evening,
            22..=23 | 0..=2 => Self::Night,
            3..=4 => Self::Midnight,
            _ => Self::Noon,
        }
    }

    /// Returns light level (0.0 to 1.0)
    pub fn light_level(&self) -> f32 {
        match self {
            Self::Dawn => 0.4,
            Self::Morning => 0.8,
            Self::Noon => 1.0,
            Self::Afternoon => 0.9,
            Self::Dusk => 0.5,
            Self::Evening => 0.3,
            Self::Night => 0.1,
            Self::Midnight => 0.05,
        }
    }

    /// Returns if it's dark
    pub fn is_dark(&self) -> bool {
        matches!(self, Self::Night | Self::Midnight | Self::Dusk)
    }
}

/// Seasons
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

impl Season {
    /// Gets the season for a given day (0-365)
    pub fn from_day(day: u32) -> Self {
        match day % 365 {
            0..=91 => Self::Spring,
            92..=182 => Self::Summer,
            183..=273 => Self::Autumn,
            274..=365 => Self::Winter,
            _ => Self::Spring,
        }
    }

    /// Returns environmental effects
    pub fn effects(&self) -> SeasonEffects {
        match self {
            Self::Spring => SeasonEffects {
                growth_rate: 1.5,
                spawn_rate: 1.2,
                weather_bias: vec![Weather::Rainy, Weather::Cloudy],
            },
            Self::Summer => SeasonEffects {
                growth_rate: 1.0,
                spawn_rate: 1.0,
                weather_bias: vec![Weather::Clear, Weather::Stormy],
            },
            Self::Autumn => SeasonEffects {
                growth_rate: 0.5,
                spawn_rate: 0.8,
                weather_bias: vec![Weather::Windy, Weather::Foggy],
            },
            Self::Winter => SeasonEffects {
                growth_rate: 0.1,
                spawn_rate: 0.5,
                weather_bias: vec![Weather::Snowy, Weather::Blizzard],
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct SeasonEffects {
    pub growth_rate: f32,
    pub spawn_rate: f32,
    pub weather_bias: Vec<Weather>,
}

/// World time state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldTime {
    /// Current tick (game time unit)
    pub tick: u64,
    /// Ticks per hour
    pub ticks_per_hour: u32,
    /// Current hour (0-23)
    pub hour: u32,
    /// Current day (0-365)
    pub day: u32,
    /// Current year
    pub year: u32,
}

impl Default for WorldTime {
    fn default() -> Self {
        Self {
            tick: 0,
            ticks_per_hour: 60,
            hour: 8,
            day: 1,
            year: 1,
        }
    }
}

impl WorldTime {
    /// Advances time by one tick
    pub fn tick(&mut self) -> Vec<TimeEvent> {
        let mut events = Vec::new();
        self.tick += 1;

        let old_hour = self.hour;
        let old_day = self.day;
        let old_time = TimeOfDay::from_hour(old_hour);

        // Update hour
        if self.tick % self.ticks_per_hour as u64 == 0 {
            self.hour = (self.hour + 1) % 24;

            // Check for time of day change
            let new_time = TimeOfDay::from_hour(self.hour);
            if new_time != old_time {
                events.push(TimeEvent::TimeOfDayChanged(new_time));
            }

            // Check for new day
            if self.hour == 0 {
                self.day = (self.day + 1) % 365;
                events.push(TimeEvent::NewDay(self.day));

                // Check for new season
                let old_season = Season::from_day(old_day);
                let new_season = Season::from_day(self.day);
                if new_season != old_season {
                    events.push(TimeEvent::SeasonChanged(new_season));
                }

                // Check for new year
                if self.day == 0 {
                    self.year += 1;
                    events.push(TimeEvent::NewYear(self.year));
                }
            }
        }

        events
    }

    /// Gets current time of day
    pub fn time_of_day(&self) -> TimeOfDay {
        TimeOfDay::from_hour(self.hour)
    }

    /// Gets current season
    pub fn season(&self) -> Season {
        Season::from_day(self.day)
    }
}

/// Events triggered by time passing
#[derive(Clone, Debug)]
pub enum TimeEvent {
    TimeOfDayChanged(TimeOfDay),
    NewDay(u32),
    SeasonChanged(Season),
    NewYear(u32),
}

/// World state managed by environmental agents
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldState {
    pub time: WorldTime,
    pub weather: Weather,
    pub weather_duration: u32,
    pub ambient_effects: Vec<AmbientEffect>,
}

impl Default for WorldState {
    fn default() -> Self {
        Self {
            time: WorldTime::default(),
            weather: Weather::Clear,
            weather_duration: 100,
            ambient_effects: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AmbientEffect {
    pub kind: AmbientEffectKind,
    pub x: usize,
    pub y: usize,
    pub radius: usize,
    pub duration: u32,
    pub intensity: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AmbientEffectKind {
    Fire,
    Water,
    Ice,
    Poison,
    Magic,
    Light,
    Darkness,
    Smoke,
    Steam,
}

/// Special world events
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldEvent {
    pub id: u32,
    pub kind: WorldEventKind,
    pub name: String,
    pub description: String,
    pub start_turn: u32,
    pub duration: u32,
    pub effects: Vec<WorldEventEffect>,
    pub active: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorldEventKind {
    /// Monster invasion
    MonsterInvasion,
    /// Merchant caravan arrives
    MerchantCaravan,
    /// Festival/celebration
    Festival,
    /// Plague outbreak
    Plague,
    /// Portal opens
    PortalOpening,
    /// Ancient evil awakens
    AncientEvil,
    /// Celestial event
    CelestialEvent,
    /// Natural disaster
    NaturalDisaster,
    /// War begins
    War,
    /// Treasure appears
    TreasureDiscovery,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WorldEventEffect {
    SpawnEnemies { kind: String, count: u32 },
    SpawnNpcs { kind: String, count: u32 },
    ModifyPrices { multiplier: f32 },
    ModifySpawnRates { multiplier: f32 },
    ChangeWeather { weather: Weather },
    DamageArea { x: usize, y: usize, radius: usize, damage: i32 },
    BuffAllies { stat: String, amount: i32 },
    DebuffEnemies { stat: String, amount: i32 },
    RevealMap { radius: usize },
    SpawnTreasure { x: usize, y: usize },
}
