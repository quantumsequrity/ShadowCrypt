//! World Events System
//!
//! Comprehensive world events including natural disasters, celestial events, invasions,
//! discoveries, and social events. Events have duration, affected areas, difficulty levels,
//! rewards, and consequences. Events can chain into story arcs and permanently change the world.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// ============================================================================
// Constants
// ============================================================================

/// Maximum concurrent active events
pub const MAX_ACTIVE_EVENTS: usize = 10;

/// Event announcement duration in turns
pub const EVENT_ANNOUNCEMENT_DURATION: u32 = 5;

/// Minimum turns between similar events
pub const EVENT_COOLDOWN: u32 = 50;

// ============================================================================
// Event Categories and Types
// ============================================================================

/// Main event categories
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventCategory {
    NaturalDisaster,
    CelestialEvent,
    Invasion,
    Discovery,
    SocialEvent,
}

impl EventCategory {
    pub fn all() -> &'static [EventCategory] {
        &[
            Self::NaturalDisaster,
            Self::CelestialEvent,
            Self::Invasion,
            Self::Discovery,
            Self::SocialEvent,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::NaturalDisaster => "Natural Disaster",
            Self::CelestialEvent => "Celestial Event",
            Self::Invasion => "Invasion",
            Self::Discovery => "Discovery",
            Self::SocialEvent => "Social Event",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::NaturalDisaster => "Catastrophic natural phenomena threatening the land.",
            Self::CelestialEvent => "Rare astronomical events affecting cultivation and magic.",
            Self::Invasion => "Hostile forces attacking from beyond.",
            Self::Discovery => "Newly revealed secrets and treasures.",
            Self::SocialEvent => "Grand gatherings and competitions.",
        }
    }

    pub fn event_types(&self) -> Vec<EventType> {
        match self {
            Self::NaturalDisaster => vec![
                EventType::Earthquake,
                EventType::VolcanicEruption,
                EventType::Flood,
                EventType::MeteorShower,
            ],
            Self::CelestialEvent => vec![
                EventType::SolarEclipse,
                EventType::LunarEclipse,
                EventType::SpiritRain,
                EventType::HeavenlyTribulation,
            ],
            Self::Invasion => vec![
                EventType::DemonInvasion,
                EventType::BeastTide,
                EventType::UndeadUprising,
                EventType::ForeignRealmBreach,
            ],
            Self::Discovery => vec![
                EventType::SecretRealmOpened,
                EventType::AncientTombDiscovered,
                EventType::TreasureAppears,
                EventType::LegendaryBeastSighted,
            ],
            Self::SocialEvent => vec![
                EventType::SectTournament,
                EventType::KingdomFestival,
                EventType::GrandAuction,
                EventType::MartialArtsCompetition,
            ],
        }
    }
}

/// Specific event types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventType {
    // Natural Disasters
    Earthquake,
    VolcanicEruption,
    Flood,
    MeteorShower,

    // Celestial Events
    SolarEclipse,
    LunarEclipse,
    SpiritRain,
    HeavenlyTribulation,

    // Invasions
    DemonInvasion,
    BeastTide,
    UndeadUprising,
    ForeignRealmBreach,

    // Discoveries
    SecretRealmOpened,
    AncientTombDiscovered,
    TreasureAppears,
    LegendaryBeastSighted,

    // Social Events
    SectTournament,
    KingdomFestival,
    GrandAuction,
    MartialArtsCompetition,
}

impl EventType {
    pub fn all() -> &'static [EventType] {
        &[
            // Natural Disasters
            Self::Earthquake,
            Self::VolcanicEruption,
            Self::Flood,
            Self::MeteorShower,
            // Celestial Events
            Self::SolarEclipse,
            Self::LunarEclipse,
            Self::SpiritRain,
            Self::HeavenlyTribulation,
            // Invasions
            Self::DemonInvasion,
            Self::BeastTide,
            Self::UndeadUprising,
            Self::ForeignRealmBreach,
            // Discoveries
            Self::SecretRealmOpened,
            Self::AncientTombDiscovered,
            Self::TreasureAppears,
            Self::LegendaryBeastSighted,
            // Social Events
            Self::SectTournament,
            Self::KingdomFestival,
            Self::GrandAuction,
            Self::MartialArtsCompetition,
        ]
    }

    pub fn category(&self) -> EventCategory {
        match self {
            Self::Earthquake | Self::VolcanicEruption | Self::Flood | Self::MeteorShower => {
                EventCategory::NaturalDisaster
            }
            Self::SolarEclipse | Self::LunarEclipse | Self::SpiritRain | Self::HeavenlyTribulation => {
                EventCategory::CelestialEvent
            }
            Self::DemonInvasion | Self::BeastTide | Self::UndeadUprising | Self::ForeignRealmBreach => {
                EventCategory::Invasion
            }
            Self::SecretRealmOpened | Self::AncientTombDiscovered | Self::TreasureAppears | Self::LegendaryBeastSighted => {
                EventCategory::Discovery
            }
            Self::SectTournament | Self::KingdomFestival | Self::GrandAuction | Self::MartialArtsCompetition => {
                EventCategory::SocialEvent
            }
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Earthquake => "Earthquake",
            Self::VolcanicEruption => "Volcanic Eruption",
            Self::Flood => "Great Flood",
            Self::MeteorShower => "Meteor Shower",
            Self::SolarEclipse => "Solar Eclipse",
            Self::LunarEclipse => "Lunar Eclipse",
            Self::SpiritRain => "Spirit Rain",
            Self::HeavenlyTribulation => "Heavenly Tribulation",
            Self::DemonInvasion => "Demon Invasion",
            Self::BeastTide => "Beast Tide",
            Self::UndeadUprising => "Undead Uprising",
            Self::ForeignRealmBreach => "Foreign Realm Breach",
            Self::SecretRealmOpened => "Secret Realm Opened",
            Self::AncientTombDiscovered => "Ancient Tomb Discovered",
            Self::TreasureAppears => "Treasure Appears",
            Self::LegendaryBeastSighted => "Legendary Beast Sighted",
            Self::SectTournament => "Sect Tournament",
            Self::KingdomFestival => "Kingdom Festival",
            Self::GrandAuction => "Grand Auction",
            Self::MartialArtsCompetition => "Martial Arts Competition",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Earthquake => "The ground shakes violently, collapsing structures and opening fissures. Dungeons may reveal hidden chambers.",
            Self::VolcanicEruption => "A volcano awakens, spewing lava and ash across the land. Fire elementals emerge from the molten depths.",
            Self::Flood => "Torrential rains cause massive flooding. Water creatures become more active and treasures wash ashore.",
            Self::MeteorShower => "Celestial stones rain from the sky, carrying rare materials and awakening dormant powers.",
            Self::SolarEclipse => "Darkness covers the land as the sun is blocked. Dark cultivators gain immense power during this time.",
            Self::LunarEclipse => "The blood moon rises, empowering light cultivators and weakening creatures of darkness.",
            Self::SpiritRain => "Spiritual energy falls from the heavens, greatly accelerating cultivation for all beings.",
            Self::HeavenlyTribulation => "Lightning strikes randomly across the land, testing the worthy and destroying the unworthy.",
            Self::DemonInvasion => "Portals to the demon realm open, unleashing hordes of demons upon the mortal world.",
            Self::BeastTide => "Magical beasts surge forth from the wilderness in a frenzied migration, destroying everything in their path.",
            Self::UndeadUprising => "The dead rise from their graves, led by a powerful necromancer seeking to conquer the living.",
            Self::ForeignRealmBreach => "A rift opens to another dimension, and alien creatures pour through seeking to colonize.",
            Self::SecretRealmOpened => "An ancient secret realm becomes accessible, filled with forgotten treasures and dangers.",
            Self::AncientTombDiscovered => "An ancient cultivator's tomb is discovered, containing their legacy and guardian beasts.",
            Self::TreasureAppears => "A legendary treasure manifests in the world, drawing cultivators from all corners.",
            Self::LegendaryBeastSighted => "A legendary beast appears, offering great rewards to those who can defeat or befriend it.",
            Self::SectTournament => "Sects gather for a grand tournament to determine the strongest young cultivators.",
            Self::KingdomFestival => "A kingdom celebrates with festivities, offering special quests and discounted goods.",
            Self::GrandAuction => "A grand auction is held, featuring rare items and opportunities for the wealthy.",
            Self::MartialArtsCompetition => "A martial arts competition draws fighters from across the land to test their skills.",
        }
    }

    pub fn base_duration(&self) -> u32 {
        match self {
            Self::Earthquake => 10,
            Self::VolcanicEruption => 50,
            Self::Flood => 30,
            Self::MeteorShower => 5,
            Self::SolarEclipse => 15,
            Self::LunarEclipse => 15,
            Self::SpiritRain => 20,
            Self::HeavenlyTribulation => 25,
            Self::DemonInvasion => 100,
            Self::BeastTide => 75,
            Self::UndeadUprising => 80,
            Self::ForeignRealmBreach => 120,
            Self::SecretRealmOpened => 200,
            Self::AncientTombDiscovered => 150,
            Self::TreasureAppears => 50,
            Self::LegendaryBeastSighted => 100,
            Self::SectTournament => 30,
            Self::KingdomFestival => 50,
            Self::GrandAuction => 20,
            Self::MartialArtsCompetition => 25,
        }
    }

    pub fn base_difficulty(&self) -> EventDifficulty {
        match self {
            Self::Earthquake => EventDifficulty::Medium,
            Self::VolcanicEruption => EventDifficulty::Hard,
            Self::Flood => EventDifficulty::Medium,
            Self::MeteorShower => EventDifficulty::Easy,
            Self::SolarEclipse => EventDifficulty::Medium,
            Self::LunarEclipse => EventDifficulty::Medium,
            Self::SpiritRain => EventDifficulty::Easy,
            Self::HeavenlyTribulation => EventDifficulty::Extreme,
            Self::DemonInvasion => EventDifficulty::Extreme,
            Self::BeastTide => EventDifficulty::Hard,
            Self::UndeadUprising => EventDifficulty::Hard,
            Self::ForeignRealmBreach => EventDifficulty::Legendary,
            Self::SecretRealmOpened => EventDifficulty::Hard,
            Self::AncientTombDiscovered => EventDifficulty::Hard,
            Self::TreasureAppears => EventDifficulty::Medium,
            Self::LegendaryBeastSighted => EventDifficulty::Legendary,
            Self::SectTournament => EventDifficulty::Medium,
            Self::KingdomFestival => EventDifficulty::Easy,
            Self::GrandAuction => EventDifficulty::Easy,
            Self::MartialArtsCompetition => EventDifficulty::Medium,
        }
    }

    pub fn can_participate(&self) -> bool {
        match self {
            Self::Earthquake | Self::VolcanicEruption | Self::Flood => false,
            Self::MeteorShower => true,
            Self::SolarEclipse | Self::LunarEclipse | Self::SpiritRain => true,
            Self::HeavenlyTribulation => true,
            Self::DemonInvasion | Self::BeastTide | Self::UndeadUprising | Self::ForeignRealmBreach => true,
            Self::SecretRealmOpened | Self::AncientTombDiscovered | Self::TreasureAppears | Self::LegendaryBeastSighted => true,
            Self::SectTournament | Self::KingdomFestival | Self::GrandAuction | Self::MartialArtsCompetition => true,
        }
    }

    pub fn possible_chain_events(&self) -> Vec<EventType> {
        match self {
            Self::Earthquake => vec![Self::AncientTombDiscovered, Self::SecretRealmOpened],
            Self::VolcanicEruption => vec![Self::Earthquake, Self::MeteorShower],
            Self::Flood => vec![Self::TreasureAppears],
            Self::MeteorShower => vec![Self::SpiritRain, Self::TreasureAppears],
            Self::SolarEclipse => vec![Self::DemonInvasion, Self::UndeadUprising],
            Self::LunarEclipse => vec![Self::BeastTide, Self::LegendaryBeastSighted],
            Self::SpiritRain => vec![Self::HeavenlyTribulation],
            Self::HeavenlyTribulation => vec![Self::SecretRealmOpened],
            Self::DemonInvasion => vec![Self::ForeignRealmBreach, Self::HeavenlyTribulation],
            Self::BeastTide => vec![Self::LegendaryBeastSighted],
            Self::UndeadUprising => vec![Self::AncientTombDiscovered],
            Self::ForeignRealmBreach => vec![Self::SecretRealmOpened, Self::LegendaryBeastSighted],
            Self::SecretRealmOpened => vec![Self::TreasureAppears, Self::LegendaryBeastSighted],
            Self::AncientTombDiscovered => vec![Self::UndeadUprising, Self::TreasureAppears],
            Self::TreasureAppears => vec![Self::MartialArtsCompetition],
            Self::LegendaryBeastSighted => vec![Self::BeastTide],
            Self::SectTournament => vec![Self::MartialArtsCompetition],
            Self::KingdomFestival => vec![Self::GrandAuction],
            Self::GrandAuction => vec![Self::TreasureAppears],
            Self::MartialArtsCompetition => vec![Self::SectTournament],
        }
    }
}

// ============================================================================
// Event Difficulty and Scale
// ============================================================================

/// Event difficulty levels
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum EventDifficulty {
    Trivial = 0,
    Easy = 1,
    Medium = 2,
    Hard = 3,
    Extreme = 4,
    Legendary = 5,
    Apocalyptic = 6,
}

impl EventDifficulty {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Trivial => "Trivial",
            Self::Easy => "Easy",
            Self::Medium => "Medium",
            Self::Hard => "Hard",
            Self::Extreme => "Extreme",
            Self::Legendary => "Legendary",
            Self::Apocalyptic => "Apocalyptic",
        }
    }

    pub fn min_level(&self) -> u32 {
        match self {
            Self::Trivial => 1,
            Self::Easy => 5,
            Self::Medium => 15,
            Self::Hard => 30,
            Self::Extreme => 50,
            Self::Legendary => 75,
            Self::Apocalyptic => 100,
        }
    }

    pub fn reward_multiplier(&self) -> f32 {
        match self {
            Self::Trivial => 0.5,
            Self::Easy => 1.0,
            Self::Medium => 1.5,
            Self::Hard => 2.5,
            Self::Extreme => 4.0,
            Self::Legendary => 7.0,
            Self::Apocalyptic => 15.0,
        }
    }
}

/// Event scale - how large the affected area is
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventScale {
    /// Single location (one room/area)
    Local,
    /// Multiple adjacent areas
    Regional,
    /// Entire dungeon floor or town
    Zonal,
    /// Multiple zones
    Continental,
    /// Entire world
    Global,
}

impl EventScale {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Local => "Local",
            Self::Regional => "Regional",
            Self::Zonal => "Zonal",
            Self::Continental => "Continental",
            Self::Global => "Global",
        }
    }

    pub fn radius(&self) -> u32 {
        match self {
            Self::Local => 5,
            Self::Regional => 20,
            Self::Zonal => 50,
            Self::Continental => 200,
            Self::Global => u32::MAX,
        }
    }
}

// ============================================================================
// Affected Areas
// ============================================================================

/// An area affected by an event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AffectedArea {
    pub center_x: i32,
    pub center_y: i32,
    pub floor: Option<u32>,
    pub dimension: Option<String>,
    pub radius: u32,
    pub intensity: f32,
    pub modifiers: AreaModifiers,
}

impl AffectedArea {
    pub fn new(center_x: i32, center_y: i32, radius: u32) -> Self {
        Self {
            center_x,
            center_y,
            floor: None,
            dimension: None,
            radius,
            intensity: 1.0,
            modifiers: AreaModifiers::default(),
        }
    }

    pub fn with_floor(mut self, floor: u32) -> Self {
        self.floor = Some(floor);
        self
    }

    pub fn with_dimension(mut self, dimension: String) -> Self {
        self.dimension = Some(dimension);
        self
    }

    pub fn with_intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity.clamp(0.0, 2.0);
        self
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        let dx = (x - self.center_x) as f32;
        let dy = (y - self.center_y) as f32;
        let distance = (dx * dx + dy * dy).sqrt();
        distance <= self.radius as f32
    }

    pub fn intensity_at(&self, x: i32, y: i32) -> f32 {
        if !self.contains(x, y) {
            return 0.0;
        }
        let dx = (x - self.center_x) as f32;
        let dy = (y - self.center_y) as f32;
        let distance = (dx * dx + dy * dy).sqrt();
        let falloff = 1.0 - (distance / self.radius as f32);
        self.intensity * falloff
    }
}

/// Modifiers applied to areas during events
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AreaModifiers {
    pub damage_multiplier: f32,
    pub spawn_rate_multiplier: f32,
    pub loot_multiplier: f32,
    pub xp_multiplier: f32,
    pub cultivation_multiplier: f32,
    pub visibility_modifier: i32,
    pub movement_speed_modifier: f32,
    pub environmental_damage: u32,
    pub environmental_damage_type: Option<DamageType>,
}

impl AreaModifiers {
    pub fn new() -> Self {
        Self {
            damage_multiplier: 1.0,
            spawn_rate_multiplier: 1.0,
            loot_multiplier: 1.0,
            xp_multiplier: 1.0,
            cultivation_multiplier: 1.0,
            visibility_modifier: 0,
            movement_speed_modifier: 1.0,
            environmental_damage: 0,
            environmental_damage_type: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageType {
    Physical,
    Fire,
    Cold,
    Lightning,
    Poison,
    Necrotic,
    Radiant,
    Shadow,
    Arcane,
    Void,
}

// ============================================================================
// Event Rewards and Consequences
// ============================================================================

/// Rewards for participating in events
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventReward {
    pub gold: u32,
    pub experience: u32,
    pub cultivation_points: u32,
    pub items: Vec<EventRewardItem>,
    pub reputation_gains: HashMap<String, i32>,
    pub skill_unlock: Option<String>,
    pub title_unlock: Option<String>,
    pub exclusive_reward: Option<ExclusiveReward>,
}

impl Default for EventReward {
    fn default() -> Self {
        Self {
            gold: 0,
            experience: 0,
            cultivation_points: 0,
            items: Vec::new(),
            reputation_gains: HashMap::new(),
            skill_unlock: None,
            title_unlock: None,
            exclusive_reward: None,
        }
    }
}

impl EventReward {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_gold(mut self, gold: u32) -> Self {
        self.gold = gold;
        self
    }

    pub fn with_experience(mut self, xp: u32) -> Self {
        self.experience = xp;
        self
    }

    pub fn with_cultivation(mut self, points: u32) -> Self {
        self.cultivation_points = points;
        self
    }

    pub fn with_item(mut self, item: EventRewardItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn apply_multiplier(&mut self, multiplier: f32) {
        self.gold = (self.gold as f32 * multiplier) as u32;
        self.experience = (self.experience as f32 * multiplier) as u32;
        self.cultivation_points = (self.cultivation_points as f32 * multiplier) as u32;
    }
}

/// Item reward from events
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventRewardItem {
    pub item_id: String,
    pub quantity: u32,
    pub drop_chance: f32,
    pub min_rarity: ItemRarity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
    Divine,
}

/// Exclusive rewards only available from specific events
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExclusiveReward {
    UniqueItem { item_id: String, name: String },
    SpecialMount { mount_id: String, name: String },
    PetCompanion { pet_id: String, name: String },
    SecretTechnique { technique_id: String, name: String },
    BloodlineAwakening { bloodline_id: String, name: String },
    DimensionKey { dimension_id: String, name: String },
    AncientLegacy { legacy_id: String, description: String },
}

/// Consequences for ignoring events
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventConsequence {
    pub description: String,
    pub severity: ConsequenceSeverity,
    pub effects: Vec<ConsequenceEffect>,
    pub permanent: bool,
    pub reversible: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConsequenceSeverity {
    Minor,
    Moderate,
    Major,
    Severe,
    Catastrophic,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConsequenceEffect {
    AreaDestroyed { area_id: String },
    NpcDeath { npc_id: String, npc_name: String },
    ShopClosed { shop_id: String, duration: u32 },
    ReputationLoss { faction: String, amount: i32 },
    ResourceDepletion { resource: String, amount: u32 },
    EnemyStrengthened { enemy_type: String, multiplier: f32 },
    PermanentEnvironmentChange { change_type: EnvironmentChange },
    WorldStateChange { state_key: String, new_value: String },
    NewEnemyType { enemy_id: String, spawn_area: String },
    QuestFailed { quest_id: String },
    TerritoryLost { territory_id: String, new_owner: String },
    PowerBalanceShift { faction_a: String, faction_b: String, shift: i32 },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EnvironmentChange {
    TerrainAltered { from: String, to: String },
    NewHazard { hazard_type: String },
    ResourceSpawn { resource_type: String },
    PortalOpened { destination: String },
    AreaSealed { until_event: Option<String> },
    ClimateChange { new_climate: String },
}

// ============================================================================
// Event Phases
// ============================================================================

/// Phase of an event
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventPhase {
    /// Event is announced but not yet active
    Announced,
    /// Event is building up
    Brewing,
    /// Event is at full intensity
    Active,
    /// Event is winding down
    Declining,
    /// Event has ended
    Concluded,
    /// Event was resolved by players
    Resolved,
    /// Event failed/was ignored
    Failed,
}

impl EventPhase {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Brewing | Self::Active | Self::Declining)
    }

    pub fn is_concluded(&self) -> bool {
        matches!(self, Self::Concluded | Self::Resolved | Self::Failed)
    }
}

// ============================================================================
// World Event
// ============================================================================

/// A world event instance
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldEvent {
    pub id: u64,
    pub event_type: EventType,
    pub name: String,
    pub description: String,
    pub phase: EventPhase,
    pub difficulty: EventDifficulty,
    pub scale: EventScale,
    pub affected_areas: Vec<AffectedArea>,
    pub start_turn: u32,
    pub duration: u32,
    pub turns_elapsed: u32,
    pub intensity: f32,
    pub rewards: EventReward,
    pub consequences: Vec<EventConsequence>,
    pub participation_rewards: EventReward,
    pub completion_rewards: EventReward,
    pub chain_event_id: Option<u64>,
    pub story_arc_id: Option<u64>,
    pub player_participation: PlayerParticipation,
    pub world_changes: Vec<WorldStateChange>,
    pub spawned_entities: Vec<SpawnedEntity>,
    pub objectives: Vec<EventObjective>,
}

impl WorldEvent {
    pub fn new(id: u64, event_type: EventType, start_turn: u32) -> Self {
        let difficulty = event_type.base_difficulty();
        let duration = event_type.base_duration();

        Self {
            id,
            event_type,
            name: event_type.name().to_string(),
            description: event_type.description().to_string(),
            phase: EventPhase::Announced,
            difficulty,
            scale: EventScale::Regional,
            affected_areas: Vec::new(),
            start_turn,
            duration,
            turns_elapsed: 0,
            intensity: 1.0,
            rewards: Self::generate_base_rewards(event_type, difficulty),
            consequences: Vec::new(),
            participation_rewards: EventReward::default(),
            completion_rewards: EventReward::default(),
            chain_event_id: None,
            story_arc_id: None,
            player_participation: PlayerParticipation::default(),
            world_changes: Vec::new(),
            spawned_entities: Vec::new(),
            objectives: Vec::new(),
        }
    }

    fn generate_base_rewards(event_type: EventType, difficulty: EventDifficulty) -> EventReward {
        let multiplier = difficulty.reward_multiplier();
        let base_gold = match event_type.category() {
            EventCategory::NaturalDisaster => 100,
            EventCategory::CelestialEvent => 200,
            EventCategory::Invasion => 500,
            EventCategory::Discovery => 1000,
            EventCategory::SocialEvent => 300,
        };
        let base_xp = match event_type.category() {
            EventCategory::NaturalDisaster => 50,
            EventCategory::CelestialEvent => 150,
            EventCategory::Invasion => 300,
            EventCategory::Discovery => 200,
            EventCategory::SocialEvent => 100,
        };

        EventReward {
            gold: (base_gold as f32 * multiplier) as u32,
            experience: (base_xp as f32 * multiplier) as u32,
            cultivation_points: (50.0 * multiplier) as u32,
            items: Vec::new(),
            reputation_gains: HashMap::new(),
            skill_unlock: None,
            title_unlock: None,
            exclusive_reward: None,
        }
    }

    pub fn tick(&mut self, current_turn: u32) -> Vec<EventUpdate> {
        let mut updates = Vec::new();
        self.turns_elapsed = current_turn.saturating_sub(self.start_turn);

        // Phase transitions
        let announcement_end = EVENT_ANNOUNCEMENT_DURATION;
        let brewing_end = announcement_end + self.duration / 4;
        let active_end = brewing_end + self.duration / 2;
        let declining_end = active_end + self.duration / 4;

        let new_phase = if self.turns_elapsed < announcement_end {
            EventPhase::Announced
        } else if self.turns_elapsed < brewing_end {
            EventPhase::Brewing
        } else if self.turns_elapsed < active_end {
            EventPhase::Active
        } else if self.turns_elapsed < declining_end {
            EventPhase::Declining
        } else {
            EventPhase::Concluded
        };

        if new_phase != self.phase {
            let old_phase = self.phase;
            self.phase = new_phase;
            updates.push(EventUpdate::PhaseChange {
                event_id: self.id,
                old_phase,
                new_phase
            });
        }

        // Update intensity based on phase
        self.intensity = match self.phase {
            EventPhase::Announced => 0.0,
            EventPhase::Brewing => 0.5,
            EventPhase::Active => 1.0,
            EventPhase::Declining => 0.5,
            _ => 0.0,
        };

        updates
    }

    pub fn is_complete(&self) -> bool {
        self.phase.is_concluded()
    }

    pub fn can_participate(&self) -> bool {
        self.event_type.can_participate() && self.phase.is_active()
    }

    pub fn add_affected_area(&mut self, area: AffectedArea) {
        self.affected_areas.push(area);
    }

    pub fn add_objective(&mut self, objective: EventObjective) {
        self.objectives.push(objective);
    }

    pub fn check_objectives(&self) -> f32 {
        if self.objectives.is_empty() {
            return 1.0;
        }
        let completed = self.objectives.iter().filter(|o| o.completed).count();
        completed as f32 / self.objectives.len() as f32
    }

    pub fn get_active_modifiers(&self) -> AreaModifiers {
        let mut modifiers = AreaModifiers::new();

        match self.event_type {
            EventType::Earthquake => {
                modifiers.movement_speed_modifier = 0.7;
                modifiers.environmental_damage = 5;
                modifiers.environmental_damage_type = Some(DamageType::Physical);
            }
            EventType::VolcanicEruption => {
                modifiers.environmental_damage = 15;
                modifiers.environmental_damage_type = Some(DamageType::Fire);
                modifiers.visibility_modifier = -5;
            }
            EventType::Flood => {
                modifiers.movement_speed_modifier = 0.5;
                modifiers.environmental_damage = 3;
                modifiers.environmental_damage_type = Some(DamageType::Cold);
            }
            EventType::MeteorShower => {
                modifiers.loot_multiplier = 1.5;
                modifiers.environmental_damage = 20;
                modifiers.environmental_damage_type = Some(DamageType::Fire);
            }
            EventType::SolarEclipse => {
                modifiers.visibility_modifier = -10;
                modifiers.damage_multiplier = 1.3; // Dark cultivators boosted
            }
            EventType::LunarEclipse => {
                modifiers.visibility_modifier = -3;
                modifiers.cultivation_multiplier = 1.2;
            }
            EventType::SpiritRain => {
                modifiers.cultivation_multiplier = 2.0;
                modifiers.xp_multiplier = 1.5;
            }
            EventType::HeavenlyTribulation => {
                modifiers.environmental_damage = 50;
                modifiers.environmental_damage_type = Some(DamageType::Lightning);
                modifiers.xp_multiplier = 3.0;
            }
            EventType::DemonInvasion => {
                modifiers.spawn_rate_multiplier = 3.0;
                modifiers.damage_multiplier = 1.5;
                modifiers.loot_multiplier = 2.0;
            }
            EventType::BeastTide => {
                modifiers.spawn_rate_multiplier = 5.0;
                modifiers.loot_multiplier = 1.5;
            }
            EventType::UndeadUprising => {
                modifiers.spawn_rate_multiplier = 4.0;
                modifiers.visibility_modifier = -3;
            }
            EventType::ForeignRealmBreach => {
                modifiers.spawn_rate_multiplier = 2.0;
                modifiers.damage_multiplier = 2.0;
                modifiers.loot_multiplier = 3.0;
                modifiers.xp_multiplier = 2.5;
            }
            EventType::SecretRealmOpened => {
                modifiers.loot_multiplier = 3.0;
                modifiers.xp_multiplier = 2.0;
            }
            EventType::AncientTombDiscovered => {
                modifiers.loot_multiplier = 2.5;
                modifiers.spawn_rate_multiplier = 1.5;
            }
            EventType::TreasureAppears => {
                modifiers.loot_multiplier = 5.0;
            }
            EventType::LegendaryBeastSighted => {
                modifiers.xp_multiplier = 3.0;
                modifiers.loot_multiplier = 2.0;
            }
            EventType::SectTournament => {
                modifiers.xp_multiplier = 1.5;
            }
            EventType::KingdomFestival => {
                modifiers.loot_multiplier = 1.2;
            }
            EventType::GrandAuction => {
                modifiers.loot_multiplier = 1.5;
            }
            EventType::MartialArtsCompetition => {
                modifiers.xp_multiplier = 2.0;
            }
        }

        // Scale by intensity
        modifiers.damage_multiplier = 1.0 + (modifiers.damage_multiplier - 1.0) * self.intensity;
        modifiers.spawn_rate_multiplier = 1.0 + (modifiers.spawn_rate_multiplier - 1.0) * self.intensity;
        modifiers.loot_multiplier = 1.0 + (modifiers.loot_multiplier - 1.0) * self.intensity;
        modifiers.xp_multiplier = 1.0 + (modifiers.xp_multiplier - 1.0) * self.intensity;
        modifiers.cultivation_multiplier = 1.0 + (modifiers.cultivation_multiplier - 1.0) * self.intensity;
        modifiers.environmental_damage = (modifiers.environmental_damage as f32 * self.intensity) as u32;

        modifiers
    }
}

// ============================================================================
// Player Participation
// ============================================================================

/// Tracks player participation in an event
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PlayerParticipation {
    pub joined: bool,
    pub join_turn: Option<u32>,
    pub contribution_score: u32,
    pub kills: u32,
    pub objectives_completed: u32,
    pub items_collected: u32,
    pub damage_dealt: u64,
    pub damage_taken: u64,
    pub times_died: u32,
    pub rank: Option<u32>,
    pub rewards_claimed: bool,
}

impl PlayerParticipation {
    pub fn join(&mut self, turn: u32) {
        self.joined = true;
        self.join_turn = Some(turn);
    }

    pub fn add_contribution(&mut self, amount: u32) {
        self.contribution_score += amount;
    }

    pub fn record_kill(&mut self) {
        self.kills += 1;
        self.contribution_score += 10;
    }

    pub fn complete_objective(&mut self) {
        self.objectives_completed += 1;
        self.contribution_score += 50;
    }

    pub fn collect_item(&mut self) {
        self.items_collected += 1;
        self.contribution_score += 5;
    }

    pub fn record_damage(&mut self, dealt: u64, taken: u64) {
        self.damage_dealt += dealt;
        self.damage_taken += taken;
        self.contribution_score += (dealt / 100) as u32;
    }

    pub fn record_death(&mut self) {
        self.times_died += 1;
    }

    pub fn calculate_rank(&mut self, total_participants: u32) {
        // Simplified ranking based on contribution
        if total_participants == 0 {
            self.rank = Some(1);
            return;
        }
        let percentile = (self.contribution_score as f32 / (total_participants as f32 * 100.0)).min(1.0);
        self.rank = Some((total_participants as f32 * (1.0 - percentile)).max(1.0) as u32);
    }
}

// ============================================================================
// Event Objectives
// ============================================================================

/// An objective within an event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventObjective {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub objective_type: ObjectiveType,
    pub progress: u32,
    pub target: u32,
    pub completed: bool,
    pub reward: Option<EventReward>,
    pub required: bool,
    pub hidden: bool,
}

impl EventObjective {
    pub fn new(id: u64, name: &str, objective_type: ObjectiveType, target: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: String::new(),
            objective_type,
            progress: 0,
            target,
            completed: false,
            reward: None,
            required: false,
            hidden: false,
        }
    }

    pub fn update_progress(&mut self, amount: u32) -> bool {
        self.progress += amount;
        if self.progress >= self.target && !self.completed {
            self.completed = true;
            return true;
        }
        false
    }

    pub fn progress_percentage(&self) -> f32 {
        (self.progress as f32 / self.target as f32).min(1.0) * 100.0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ObjectiveType {
    KillEnemies { enemy_type: Option<String> },
    CollectItems { item_type: Option<String> },
    SurviveTurns,
    ReachLocation { x: i32, y: i32 },
    DefeatBoss { boss_id: String },
    ProtectNpc { npc_id: String },
    DestroyStructure { structure_id: String },
    ActivateDevice { device_id: String },
    EscortNpc { npc_id: String, destination_x: i32, destination_y: i32 },
    GatherResources { resource_type: String },
    SolveRiddle { riddle_id: String },
    Custom { check_fn_id: String },
}

// ============================================================================
// World State Changes
// ============================================================================

/// Permanent changes to the world caused by events
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldStateChange {
    pub change_id: u64,
    pub event_id: u64,
    pub change_type: WorldChangeType,
    pub turn_occurred: u32,
    pub reversible: bool,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WorldChangeType {
    AreaDestroyed { area_id: String, reconstruction_cost: u32 },
    AreaCreated { area_id: String, area_type: String },
    TerrainChanged { location: (i32, i32), old_terrain: String, new_terrain: String },
    NpcDied { npc_id: String, npc_name: String, cause: String },
    NpcAppeared { npc_id: String, npc_name: String, location: String },
    FactionPowerChanged { faction_id: String, power_delta: i32 },
    NewEnemyType { enemy_type_id: String, spawn_locations: Vec<String> },
    ResourceDiscovered { resource_id: String, location: String },
    PortalOpened { portal_id: String, source: String, destination: String },
    PortalClosed { portal_id: String },
    ShopStatusChanged { shop_id: String, open: bool },
    TerritoryOwnerChanged { territory_id: String, old_owner: String, new_owner: String },
    ClimateChanged { region_id: String, old_climate: String, new_climate: String },
    MagicLevelChanged { region_id: String, magic_delta: i32 },
    QuestUnlocked { quest_id: String },
    QuestFailed { quest_id: String },
    AchievementUnlocked { achievement_id: String },
    LoreDiscovered { lore_id: String },
}

/// Entities spawned by events
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpawnedEntity {
    pub entity_id: u64,
    pub entity_type: SpawnedEntityType,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub floor: Option<u32>,
    pub spawn_turn: u32,
    pub despawn_turn: Option<u32>,
    pub is_boss: bool,
    pub loot_table: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SpawnedEntityType {
    Enemy { enemy_id: String, level: u32 },
    Boss { boss_id: String, level: u32 },
    Npc { npc_id: String },
    Creature { creature_id: String },
    Object { object_id: String },
    Portal { destination: String },
    TreasureChest { rarity: ItemRarity },
}

// ============================================================================
// Event Chains and Story Arcs
// ============================================================================

/// A chain of related events
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventChain {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub events: Vec<ChainedEvent>,
    pub current_stage: usize,
    pub started: bool,
    pub completed: bool,
    pub failed: bool,
    pub branching_points: Vec<BranchingPoint>,
    pub final_rewards: EventReward,
}

impl EventChain {
    pub fn new(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: String::new(),
            events: Vec::new(),
            current_stage: 0,
            started: false,
            completed: false,
            failed: false,
            branching_points: Vec::new(),
            final_rewards: EventReward::default(),
        }
    }

    pub fn add_event(&mut self, event: ChainedEvent) {
        self.events.push(event);
    }

    pub fn advance(&mut self) -> Option<&ChainedEvent> {
        if self.current_stage < self.events.len() {
            let event = &self.events[self.current_stage];
            self.current_stage += 1;
            Some(event)
        } else {
            self.completed = true;
            None
        }
    }

    pub fn current_event(&self) -> Option<&ChainedEvent> {
        self.events.get(self.current_stage)
    }
}

/// An event within a chain
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChainedEvent {
    pub event_type: EventType,
    pub delay_turns: u32,
    pub trigger_condition: TriggerCondition,
    pub required: bool,
    pub on_success: ChainOutcome,
    pub on_failure: ChainOutcome,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TriggerCondition {
    Immediate,
    AfterDelay { turns: u32 },
    OnEventComplete { event_id: u64 },
    OnObjectiveComplete { objective_id: u64 },
    OnPlayerAction { action_type: String },
    Conditional { condition_id: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ChainOutcome {
    Continue,
    Skip { stages: usize },
    Branch { branch_id: u64 },
    End,
    Fail,
}

/// A branching point in an event chain
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BranchingPoint {
    pub id: u64,
    pub stage: usize,
    pub condition: String,
    pub branches: Vec<EventBranch>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventBranch {
    pub id: u64,
    pub name: String,
    pub events: Vec<ChainedEvent>,
    pub requirements: Vec<BranchRequirement>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BranchRequirement {
    MinLevel(u32),
    MinReputation { faction: String, amount: i32 },
    HasItem { item_id: String },
    QuestComplete { quest_id: String },
    ChoiceMade { choice_id: String },
    Custom { requirement_id: String },
}

/// A story arc spanning multiple event chains
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoryArc {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub chapters: Vec<StoryChapter>,
    pub current_chapter: usize,
    pub world_impact: WorldImpact,
    pub started: bool,
    pub completed: bool,
}

impl StoryArc {
    pub fn new(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: String::new(),
            chapters: Vec::new(),
            current_chapter: 0,
            world_impact: WorldImpact::default(),
            started: false,
            completed: false,
        }
    }

    pub fn add_chapter(&mut self, chapter: StoryChapter) {
        self.chapters.push(chapter);
    }

    pub fn advance_chapter(&mut self) -> bool {
        if self.current_chapter < self.chapters.len() - 1 {
            self.current_chapter += 1;
            true
        } else {
            self.completed = true;
            false
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoryChapter {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub event_chains: Vec<u64>,
    pub completion_requirements: Vec<ChapterRequirement>,
    pub rewards: EventReward,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ChapterRequirement {
    CompleteEventChain { chain_id: u64 },
    CompleteAllChains,
    CompleteAnyChain { count: usize },
    ReachLevel { level: u32 },
    DefeatBoss { boss_id: String },
    Custom { requirement_id: String },
}

/// Long-term impact of story arcs on the world
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WorldImpact {
    pub power_shifts: HashMap<String, i32>,
    pub territory_changes: Vec<(String, String, String)>, // territory, old_owner, new_owner
    pub permanent_changes: Vec<WorldStateChange>,
    pub unlocked_content: Vec<String>,
    pub new_factions: Vec<String>,
    pub destroyed_factions: Vec<String>,
}

// ============================================================================
// Event Updates and Notifications
// ============================================================================

/// Updates generated by the event system
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventUpdate {
    EventStarted { event_id: u64, event_type: EventType },
    EventEnded { event_id: u64, outcome: EventOutcome },
    PhaseChange { event_id: u64, old_phase: EventPhase, new_phase: EventPhase },
    ObjectiveCompleted { event_id: u64, objective_id: u64 },
    RewardEarned { event_id: u64, reward: EventReward },
    ConsequenceTriggered { event_id: u64, consequence: EventConsequence },
    ChainEventTriggered { chain_id: u64, event_type: EventType },
    WorldStateChanged { change: WorldStateChange },
    EntitySpawned { entity: SpawnedEntity },
    EntityDespawned { entity_id: u64 },
    PlayerRankUpdate { event_id: u64, new_rank: u32 },
    StoryArcProgressed { arc_id: u64, chapter: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventOutcome {
    Success,
    PartialSuccess,
    Failure,
    Ignored,
    Cancelled,
}

// ============================================================================
// World Event System
// ============================================================================

/// Main world event system manager
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldEventSystem {
    pub active_events: HashMap<u64, WorldEvent>,
    pub completed_events: Vec<u64>,
    pub event_history: Vec<EventHistoryEntry>,
    pub event_chains: HashMap<u64, EventChain>,
    pub story_arcs: HashMap<u64, StoryArc>,
    pub world_state_changes: Vec<WorldStateChange>,
    pub spawned_entities: HashMap<u64, SpawnedEntity>,
    pub event_cooldowns: HashMap<EventType, u32>,
    pub current_turn: u32,
    pub next_event_id: u64,
    pub next_chain_id: u64,
    pub next_arc_id: u64,
    pub next_entity_id: u64,
    pub next_change_id: u64,
    pub global_modifiers: AreaModifiers,
    pub scheduled_events: Vec<ScheduledEvent>,
    pub event_statistics: EventStatistics,
}

impl Default for WorldEventSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl WorldEventSystem {
    pub fn new() -> Self {
        Self {
            active_events: HashMap::new(),
            completed_events: Vec::new(),
            event_history: Vec::new(),
            event_chains: HashMap::new(),
            story_arcs: HashMap::new(),
            world_state_changes: Vec::new(),
            spawned_entities: HashMap::new(),
            event_cooldowns: HashMap::new(),
            current_turn: 0,
            next_event_id: 1,
            next_chain_id: 1,
            next_arc_id: 1,
            next_entity_id: 1,
            next_change_id: 1,
            global_modifiers: AreaModifiers::new(),
            scheduled_events: Vec::new(),
            event_statistics: EventStatistics::default(),
        }
    }

    /// Advance the event system by one turn
    pub fn tick(&mut self) -> Vec<EventUpdate> {
        self.current_turn += 1;
        let mut updates = Vec::new();

        // Process scheduled events
        let scheduled: Vec<_> = self.scheduled_events
            .iter()
            .filter(|e| e.trigger_turn <= self.current_turn)
            .cloned()
            .collect();

        for scheduled_event in scheduled {
            if let Some(update) = self.trigger_event(scheduled_event.event_type) {
                updates.push(update);
            }
        }
        self.scheduled_events.retain(|e| e.trigger_turn > self.current_turn);

        // Update active events
        let event_ids: Vec<_> = self.active_events.keys().cloned().collect();
        let mut chain_triggers = Vec::new();
        for event_id in event_ids {
            if let Some(event) = self.active_events.get_mut(&event_id) {
                let event_updates = event.tick(self.current_turn);
                updates.extend(event_updates);

                // Collect chain event info for later processing
                if event.is_complete() && !event.phase.is_concluded() {
                    chain_triggers.push(event.event_type);
                }
            }
        }
        // Process chain triggers after releasing mutable borrow
        for event_type in chain_triggers {
            if let Some(chain_event) = self.check_chain_trigger(event_type) {
                updates.push(EventUpdate::ChainEventTriggered {
                    chain_id: 0,
                    event_type: chain_event,
                });
                self.schedule_event(chain_event, self.current_turn + 10);
            }
        }

        // Move completed events
        let completed: Vec<_> = self.active_events
            .iter()
            .filter(|(_, e)| e.is_complete())
            .map(|(id, _)| *id)
            .collect();

        for event_id in completed {
            if let Some(event) = self.active_events.remove(&event_id) {
                self.finalize_event(event, &mut updates);
            }
        }

        // Update cooldowns
        for cooldown in self.event_cooldowns.values_mut() {
            *cooldown = cooldown.saturating_sub(1);
        }

        // Recalculate global modifiers
        self.recalculate_global_modifiers();

        // Despawn expired entities
        self.despawn_expired_entities(&mut updates);

        updates
    }

    /// Trigger a new event
    pub fn trigger_event(&mut self, event_type: EventType) -> Option<EventUpdate> {
        // Check cooldown
        if let Some(&cooldown) = self.event_cooldowns.get(&event_type) {
            if cooldown > 0 {
                return None;
            }
        }

        // Check max active events
        if self.active_events.len() >= MAX_ACTIVE_EVENTS {
            return None;
        }

        let event_id = self.next_event_id;
        self.next_event_id += 1;

        let event = WorldEvent::new(event_id, event_type, self.current_turn);
        self.active_events.insert(event_id, event);
        self.event_cooldowns.insert(event_type, EVENT_COOLDOWN);
        self.event_statistics.events_started += 1;

        Some(EventUpdate::EventStarted { event_id, event_type })
    }

    /// Schedule an event for a future turn
    pub fn schedule_event(&mut self, event_type: EventType, trigger_turn: u32) {
        self.scheduled_events.push(ScheduledEvent {
            event_type,
            trigger_turn,
            priority: 0,
        });
    }

    /// Get an active event by ID
    pub fn get_event(&self, event_id: u64) -> Option<&WorldEvent> {
        self.active_events.get(&event_id)
    }

    /// Get a mutable reference to an active event
    pub fn get_event_mut(&mut self, event_id: u64) -> Option<&mut WorldEvent> {
        self.active_events.get_mut(&event_id)
    }

    /// Join an event as a participant
    pub fn join_event(&mut self, event_id: u64) -> bool {
        if let Some(event) = self.active_events.get_mut(&event_id) {
            if event.can_participate() {
                event.player_participation.join(self.current_turn);
                return true;
            }
        }
        false
    }

    /// Record player contribution to an event
    pub fn record_contribution(&mut self, event_id: u64, contribution: u32) {
        if let Some(event) = self.active_events.get_mut(&event_id) {
            event.player_participation.add_contribution(contribution);
        }
    }

    /// Record a kill in an event
    pub fn record_kill(&mut self, event_id: u64) {
        if let Some(event) = self.active_events.get_mut(&event_id) {
            event.player_participation.record_kill();
        }
    }

    /// Complete an objective in an event
    pub fn complete_objective(&mut self, event_id: u64, objective_id: u64) -> Option<EventUpdate> {
        if let Some(event) = self.active_events.get_mut(&event_id) {
            for objective in &mut event.objectives {
                if objective.id == objective_id && !objective.completed {
                    objective.completed = true;
                    event.player_participation.complete_objective();
                    return Some(EventUpdate::ObjectiveCompleted { event_id, objective_id });
                }
            }
        }
        None
    }

    /// Update objective progress
    pub fn update_objective_progress(&mut self, event_id: u64, objective_id: u64, amount: u32) -> Option<EventUpdate> {
        if let Some(event) = self.active_events.get_mut(&event_id) {
            for objective in &mut event.objectives {
                if objective.id == objective_id {
                    if objective.update_progress(amount) {
                        event.player_participation.complete_objective();
                        return Some(EventUpdate::ObjectiveCompleted { event_id, objective_id });
                    }
                }
            }
        }
        None
    }

    /// Claim rewards from a completed event
    pub fn claim_rewards(&mut self, event_id: u64) -> Option<EventReward> {
        // Check completed events
        if let Some(event) = self.active_events.get_mut(&event_id) {
            if event.is_complete() && event.player_participation.joined && !event.player_participation.rewards_claimed {
                event.player_participation.rewards_claimed = true;
                let mut reward = event.rewards.clone();
                reward.apply_multiplier(event.difficulty.reward_multiplier());
                return Some(reward);
            }
        }
        None
    }

    /// Create an event chain
    pub fn create_event_chain(&mut self, name: &str) -> u64 {
        let chain_id = self.next_chain_id;
        self.next_chain_id += 1;
        self.event_chains.insert(chain_id, EventChain::new(chain_id, name));
        chain_id
    }

    /// Add an event to a chain
    pub fn add_to_chain(&mut self, chain_id: u64, chained_event: ChainedEvent) {
        if let Some(chain) = self.event_chains.get_mut(&chain_id) {
            chain.add_event(chained_event);
        }
    }

    /// Start an event chain
    pub fn start_chain(&mut self, chain_id: u64) -> Option<EventUpdate> {
        if let Some(chain) = self.event_chains.get_mut(&chain_id) {
            if !chain.started {
                chain.started = true;
                if let Some(first_event) = chain.current_event() {
                    let event_type = first_event.event_type;
                    return self.trigger_event(event_type);
                }
            }
        }
        None
    }

    /// Create a story arc
    pub fn create_story_arc(&mut self, name: &str) -> u64 {
        let arc_id = self.next_arc_id;
        self.next_arc_id += 1;
        self.story_arcs.insert(arc_id, StoryArc::new(arc_id, name));
        arc_id
    }

    /// Spawn an entity for an event
    pub fn spawn_entity(&mut self, event_id: u64, entity_type: SpawnedEntityType, name: &str, x: i32, y: i32) -> u64 {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;

        let entity = SpawnedEntity {
            entity_id,
            entity_type,
            name: name.to_string(),
            x,
            y,
            floor: None,
            spawn_turn: self.current_turn,
            despawn_turn: None,
            is_boss: false,
            loot_table: None,
        };

        self.spawned_entities.insert(entity_id, entity.clone());

        if let Some(event) = self.active_events.get_mut(&event_id) {
            event.spawned_entities.push(entity);
        }

        entity_id
    }

    /// Record a world state change
    pub fn record_world_change(&mut self, event_id: u64, change_type: WorldChangeType, description: &str) -> u64 {
        let change_id = self.next_change_id;
        self.next_change_id += 1;

        let change = WorldStateChange {
            change_id,
            event_id,
            change_type,
            turn_occurred: self.current_turn,
            reversible: false,
            description: description.to_string(),
        };

        self.world_state_changes.push(change.clone());

        if let Some(event) = self.active_events.get_mut(&event_id) {
            event.world_changes.push(change);
        }

        change_id
    }

    /// Get all events affecting a location
    pub fn events_at_location(&self, x: i32, y: i32) -> Vec<&WorldEvent> {
        self.active_events
            .values()
            .filter(|e| {
                e.affected_areas.iter().any(|a| a.contains(x, y))
            })
            .collect()
    }

    /// Get combined modifiers at a location
    pub fn modifiers_at_location(&self, x: i32, y: i32) -> AreaModifiers {
        let mut modifiers = self.global_modifiers.clone();

        for event in self.events_at_location(x, y) {
            let event_mods = event.get_active_modifiers();
            modifiers.damage_multiplier *= event_mods.damage_multiplier;
            modifiers.spawn_rate_multiplier *= event_mods.spawn_rate_multiplier;
            modifiers.loot_multiplier *= event_mods.loot_multiplier;
            modifiers.xp_multiplier *= event_mods.xp_multiplier;
            modifiers.cultivation_multiplier *= event_mods.cultivation_multiplier;
            modifiers.visibility_modifier += event_mods.visibility_modifier;
            modifiers.movement_speed_modifier *= event_mods.movement_speed_modifier;
            modifiers.environmental_damage = modifiers.environmental_damage.max(event_mods.environmental_damage);
            if event_mods.environmental_damage_type.is_some() {
                modifiers.environmental_damage_type = event_mods.environmental_damage_type;
            }
        }

        modifiers
    }

    /// Get all active events of a category
    pub fn events_by_category(&self, category: EventCategory) -> Vec<&WorldEvent> {
        self.active_events
            .values()
            .filter(|e| e.event_type.category() == category)
            .collect()
    }

    /// Get event history
    pub fn get_history(&self, limit: usize) -> &[EventHistoryEntry] {
        let start = self.event_history.len().saturating_sub(limit);
        &self.event_history[start..]
    }

    /// Check if a specific event type is active
    pub fn is_event_type_active(&self, event_type: EventType) -> bool {
        self.active_events.values().any(|e| e.event_type == event_type)
    }

    /// Get statistics
    pub fn get_statistics(&self) -> &EventStatistics {
        &self.event_statistics
    }

    // Private helper methods

    fn check_chain_trigger(&self, completed_type: EventType) -> Option<EventType> {
        let possible = completed_type.possible_chain_events();
        if possible.is_empty() {
            return None;
        }
        // Simple random selection (in a real implementation, use proper RNG)
        let index = (self.current_turn as usize) % possible.len();
        Some(possible[index])
    }

    fn finalize_event(&mut self, event: WorldEvent, updates: &mut Vec<EventUpdate>) {
        let outcome = if event.player_participation.joined {
            if event.check_objectives() >= 0.5 {
                EventOutcome::Success
            } else {
                EventOutcome::PartialSuccess
            }
        } else if event.objectives.iter().all(|o| !o.required || o.completed) {
            EventOutcome::Ignored
        } else {
            EventOutcome::Failure
        };

        // Apply consequences if event was ignored or failed
        if matches!(outcome, EventOutcome::Failure | EventOutcome::Ignored) {
            for consequence in &event.consequences {
                updates.push(EventUpdate::ConsequenceTriggered {
                    event_id: event.id,
                    consequence: consequence.clone(),
                });
            }
        }

        // Record in history
        self.event_history.push(EventHistoryEntry {
            event_id: event.id,
            event_type: event.event_type,
            start_turn: event.start_turn,
            end_turn: self.current_turn,
            outcome,
            player_participated: event.player_participation.joined,
            contribution_score: event.player_participation.contribution_score,
        });

        self.completed_events.push(event.id);
        self.event_statistics.events_completed += 1;

        if event.player_participation.joined {
            self.event_statistics.events_participated += 1;
        }

        updates.push(EventUpdate::EventEnded { event_id: event.id, outcome });
    }

    fn recalculate_global_modifiers(&mut self) {
        self.global_modifiers = AreaModifiers::new();

        // Apply global effects from celestial events
        for event in self.active_events.values() {
            match event.event_type {
                EventType::SpiritRain => {
                    self.global_modifiers.cultivation_multiplier *= 1.5;
                }
                EventType::SolarEclipse => {
                    self.global_modifiers.visibility_modifier -= 5;
                }
                EventType::LunarEclipse => {
                    self.global_modifiers.cultivation_multiplier *= 1.2;
                }
                _ => {}
            }
        }
    }

    fn despawn_expired_entities(&mut self, updates: &mut Vec<EventUpdate>) {
        let expired: Vec<_> = self.spawned_entities
            .iter()
            .filter(|(_, e)| e.despawn_turn.map(|t| t <= self.current_turn).unwrap_or(false))
            .map(|(id, _)| *id)
            .collect();

        for entity_id in expired {
            self.spawned_entities.remove(&entity_id);
            updates.push(EventUpdate::EntityDespawned { entity_id });
        }
    }
}

// ============================================================================
// Supporting Types
// ============================================================================

/// A scheduled event waiting to trigger
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScheduledEvent {
    pub event_type: EventType,
    pub trigger_turn: u32,
    pub priority: i32,
}

/// Historical record of an event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventHistoryEntry {
    pub event_id: u64,
    pub event_type: EventType,
    pub start_turn: u32,
    pub end_turn: u32,
    pub outcome: EventOutcome,
    pub player_participated: bool,
    pub contribution_score: u32,
}

/// Statistics about events
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EventStatistics {
    pub events_started: u32,
    pub events_completed: u32,
    pub events_participated: u32,
    pub total_contribution: u64,
    pub highest_contribution: u32,
    pub bosses_defeated: u32,
    pub objectives_completed: u32,
    pub chains_completed: u32,
    pub arcs_completed: u32,
    pub rewards_claimed: u32,
    pub consequences_suffered: u32,
}

// ============================================================================
// Event Generators
// ============================================================================

/// Utility for generating random events
pub struct EventGenerator;

impl EventGenerator {
    /// Generate a random natural disaster
    pub fn random_natural_disaster() -> EventType {
        let disasters = [
            EventType::Earthquake,
            EventType::VolcanicEruption,
            EventType::Flood,
            EventType::MeteorShower,
        ];
        disasters[0] // In real implementation, use proper RNG
    }

    /// Generate a random celestial event
    pub fn random_celestial_event() -> EventType {
        let events = [
            EventType::SolarEclipse,
            EventType::LunarEclipse,
            EventType::SpiritRain,
            EventType::HeavenlyTribulation,
        ];
        events[0]
    }

    /// Generate a random invasion event
    pub fn random_invasion() -> EventType {
        let invasions = [
            EventType::DemonInvasion,
            EventType::BeastTide,
            EventType::UndeadUprising,
            EventType::ForeignRealmBreach,
        ];
        invasions[0]
    }

    /// Generate a random discovery event
    pub fn random_discovery() -> EventType {
        let discoveries = [
            EventType::SecretRealmOpened,
            EventType::AncientTombDiscovered,
            EventType::TreasureAppears,
            EventType::LegendaryBeastSighted,
        ];
        discoveries[0]
    }

    /// Generate a random social event
    pub fn random_social_event() -> EventType {
        let events = [
            EventType::SectTournament,
            EventType::KingdomFestival,
            EventType::GrandAuction,
            EventType::MartialArtsCompetition,
        ];
        events[0]
    }

    /// Create a pre-configured event chain
    pub fn create_demon_invasion_chain() -> EventChain {
        let mut chain = EventChain::new(0, "The Demon Lord's Return");
        chain.description = "A series of events leading to a full demon invasion.".to_string();

        chain.add_event(ChainedEvent {
            event_type: EventType::SolarEclipse,
            delay_turns: 0,
            trigger_condition: TriggerCondition::Immediate,
            required: true,
            on_success: ChainOutcome::Continue,
            on_failure: ChainOutcome::Fail,
        });

        chain.add_event(ChainedEvent {
            event_type: EventType::DemonInvasion,
            delay_turns: 20,
            trigger_condition: TriggerCondition::AfterDelay { turns: 20 },
            required: true,
            on_success: ChainOutcome::Continue,
            on_failure: ChainOutcome::Fail,
        });

        chain.add_event(ChainedEvent {
            event_type: EventType::ForeignRealmBreach,
            delay_turns: 50,
            trigger_condition: TriggerCondition::AfterDelay { turns: 50 },
            required: true,
            on_success: ChainOutcome::End,
            on_failure: ChainOutcome::Fail,
        });

        chain
    }

    /// Create a secret realm discovery chain
    pub fn create_secret_realm_chain() -> EventChain {
        let mut chain = EventChain::new(0, "Secrets of the Ancients");
        chain.description = "Discover the secrets of an ancient civilization.".to_string();

        chain.add_event(ChainedEvent {
            event_type: EventType::Earthquake,
            delay_turns: 0,
            trigger_condition: TriggerCondition::Immediate,
            required: false,
            on_success: ChainOutcome::Continue,
            on_failure: ChainOutcome::Continue,
        });

        chain.add_event(ChainedEvent {
            event_type: EventType::AncientTombDiscovered,
            delay_turns: 10,
            trigger_condition: TriggerCondition::AfterDelay { turns: 10 },
            required: true,
            on_success: ChainOutcome::Continue,
            on_failure: ChainOutcome::Fail,
        });

        chain.add_event(ChainedEvent {
            event_type: EventType::SecretRealmOpened,
            delay_turns: 30,
            trigger_condition: TriggerCondition::AfterDelay { turns: 30 },
            required: true,
            on_success: ChainOutcome::End,
            on_failure: ChainOutcome::Fail,
        });

        chain
    }

    /// Create a martial tournament arc
    pub fn create_tournament_arc() -> StoryArc {
        let mut arc = StoryArc::new(0, "The Grand Martial Tournament");
        arc.description = "Rise through the ranks to become the supreme martial artist.".to_string();

        arc.add_chapter(StoryChapter {
            id: 1,
            name: "Qualifier Rounds".to_string(),
            description: "Prove your worth in the preliminary rounds.".to_string(),
            event_chains: Vec::new(),
            completion_requirements: vec![ChapterRequirement::ReachLevel { level: 20 }],
            rewards: EventReward::new().with_gold(1000).with_experience(500),
        });

        arc.add_chapter(StoryChapter {
            id: 2,
            name: "Main Tournament".to_string(),
            description: "Face the strongest fighters from all sects.".to_string(),
            event_chains: Vec::new(),
            completion_requirements: vec![ChapterRequirement::DefeatBoss { boss_id: "tournament_champion".to_string() }],
            rewards: EventReward::new().with_gold(5000).with_experience(2000),
        });

        arc
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_categories() {
        for category in EventCategory::all() {
            assert!(!category.event_types().is_empty());
            for event_type in category.event_types() {
                assert_eq!(event_type.category(), *category);
            }
        }
    }

    #[test]
    fn test_event_creation() {
        let mut system = WorldEventSystem::new();
        let update = system.trigger_event(EventType::Earthquake);
        assert!(update.is_some());
        assert_eq!(system.active_events.len(), 1);
    }

    #[test]
    fn test_event_tick() {
        let mut system = WorldEventSystem::new();
        system.trigger_event(EventType::SpiritRain);

        for _ in 0..50 {
            system.tick();
        }

        assert!(system.completed_events.len() > 0 || system.active_events.len() > 0);
    }

    #[test]
    fn test_player_participation() {
        let mut system = WorldEventSystem::new();
        if let Some(EventUpdate::EventStarted { event_id, .. }) = system.trigger_event(EventType::SectTournament) {
            assert!(system.join_event(event_id));
            system.record_kill(event_id);
            system.record_contribution(event_id, 100);

            if let Some(event) = system.get_event(event_id) {
                assert!(event.player_participation.joined);
                assert_eq!(event.player_participation.kills, 1);
                assert!(event.player_participation.contribution_score > 100);
            }
        }
    }

    #[test]
    fn test_affected_area() {
        let area = AffectedArea::new(0, 0, 10);
        assert!(area.contains(0, 0));
        assert!(area.contains(5, 5));
        assert!(!area.contains(15, 15));

        let intensity = area.intensity_at(0, 0);
        assert!((intensity - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_event_modifiers() {
        let event = WorldEvent::new(1, EventType::SpiritRain, 0);
        let modifiers = event.get_active_modifiers();
        assert!(modifiers.cultivation_multiplier >= 1.0);
    }

    #[test]
    fn test_event_chain() {
        let chain = EventGenerator::create_demon_invasion_chain();
        assert!(!chain.events.is_empty());
        assert_eq!(chain.events.len(), 3);
    }

    #[test]
    fn test_story_arc() {
        let arc = EventGenerator::create_tournament_arc();
        assert!(!arc.chapters.is_empty());
        assert_eq!(arc.chapters.len(), 2);
    }

    #[test]
    fn test_event_cooldown() {
        let mut system = WorldEventSystem::new();
        system.trigger_event(EventType::Earthquake);

        // Should fail due to cooldown
        let second = system.trigger_event(EventType::Earthquake);
        assert!(second.is_none());
    }

    #[test]
    fn test_event_difficulty() {
        for difficulty in [EventDifficulty::Easy, EventDifficulty::Medium, EventDifficulty::Hard] {
            assert!(difficulty.reward_multiplier() > 0.0);
            assert!(difficulty.min_level() > 0);
        }
    }

    #[test]
    fn test_objective_progress() {
        let mut objective = EventObjective::new(1, "Kill 10 enemies", ObjectiveType::KillEnemies { enemy_type: None }, 10);
        assert!(!objective.completed);

        objective.update_progress(5);
        assert!(!objective.completed);
        assert!((objective.progress_percentage() - 50.0).abs() < 0.01);

        objective.update_progress(5);
        assert!(objective.completed);
    }
}
