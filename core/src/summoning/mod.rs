//! Summoning System
//!
//! A comprehensive summoning system featuring:
//! - 50+ summon types across elemental, beast, spirit, demonic, celestial, and mythical categories
//! - Summoning circles and rituals with material requirements
//! - Contracts and binding mechanisms (temporary, permanent, familiar bonds)
//! - Summon evolution and growth with experience and skill learning
//! - Summoner ranks from Apprentice to Planar Lord

use std::collections::HashMap;
use rand::prelude::*;
use serde::{Serialize, Deserialize};

// ============================================================================
// Constants
// ============================================================================

/// Maximum number of active summons a player can have
pub const MAX_ACTIVE_SUMMONS: usize = 5;

/// Maximum summon level
pub const MAX_SUMMON_LEVEL: u32 = 50;

/// Experience required per summon level
pub const XP_PER_SUMMON_LEVEL: [u32; 50] = [
    100, 150, 225, 340, 510, 765, 1150, 1725, 2590, 3885,
    4660, 5590, 6710, 8050, 9660, 11590, 13910, 16690, 20030, 24040,
    28850, 34620, 41540, 49850, 59820, 71780, 86140, 103370, 124040, 148850,
    178620, 214340, 257210, 308650, 370380, 444460, 533350, 640020, 768020, 921630,
    1105960, 1327150, 1592580, 1911100, 2293320, 2751980, 3302380, 3962860, 4755430, 5706520,
];

/// Base mana cost multiplier for summoning
pub const BASE_SUMMON_MANA_COST: i32 = 50;

/// Duration multiplier for temporary summons (in turns)
pub const TEMPORARY_SUMMON_BASE_DURATION: u32 = 100;

// ============================================================================
// Summoner Ranks
// ============================================================================

/// Summoner rank progression
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SummonerRank {
    Apprentice,
    Journeyman,
    Summoner,
    MasterSummoner,
    ArchSummoner,
    PlanarLord,
}

impl SummonerRank {
    pub fn all() -> &'static [SummonerRank] {
        &[
            Self::Apprentice,
            Self::Journeyman,
            Self::Summoner,
            Self::MasterSummoner,
            Self::ArchSummoner,
            Self::PlanarLord,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Apprentice => "Apprentice Summoner",
            Self::Journeyman => "Journeyman Summoner",
            Self::Summoner => "Summoner",
            Self::MasterSummoner => "Master Summoner",
            Self::ArchSummoner => "Arch Summoner",
            Self::PlanarLord => "Planar Lord",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Apprentice => "A novice learning the basics of summoning magic.",
            Self::Journeyman => "An experienced practitioner mastering summoning fundamentals.",
            Self::Summoner => "A full-fledged summoner capable of binding powerful entities.",
            Self::MasterSummoner => "A master who commands respect from summoned beings.",
            Self::ArchSummoner => "An elite summoner who can breach planar barriers.",
            Self::PlanarLord => "A legendary summoner who commands entities across all planes.",
        }
    }

    /// Experience threshold to reach this rank
    pub fn xp_threshold(&self) -> u32 {
        match self {
            Self::Apprentice => 0,
            Self::Journeyman => 1000,
            Self::Summoner => 5000,
            Self::MasterSummoner => 25000,
            Self::ArchSummoner => 100000,
            Self::PlanarLord => 500000,
        }
    }

    /// Maximum summons allowed at this rank
    pub fn max_summons(&self) -> usize {
        match self {
            Self::Apprentice => 1,
            Self::Journeyman => 2,
            Self::Summoner => 3,
            Self::MasterSummoner => 4,
            Self::ArchSummoner => 5,
            Self::PlanarLord => 6,
        }
    }

    /// Maximum summon tier allowed at this rank
    pub fn max_summon_tier(&self) -> SummonTier {
        match self {
            Self::Apprentice => SummonTier::Minor,
            Self::Journeyman => SummonTier::Lesser,
            Self::Summoner => SummonTier::Greater,
            Self::MasterSummoner => SummonTier::Lord,
            Self::ArchSummoner => SummonTier::Ancient,
            Self::PlanarLord => SummonTier::Primordial,
        }
    }

    /// Mana cost reduction at this rank (percentage)
    pub fn mana_cost_reduction(&self) -> f32 {
        match self {
            Self::Apprentice => 0.0,
            Self::Journeyman => 0.10,
            Self::Summoner => 0.20,
            Self::MasterSummoner => 0.30,
            Self::ArchSummoner => 0.40,
            Self::PlanarLord => 0.50,
        }
    }

    /// Control power bonus at this rank
    pub fn control_bonus(&self) -> i32 {
        match self {
            Self::Apprentice => 0,
            Self::Journeyman => 5,
            Self::Summoner => 15,
            Self::MasterSummoner => 30,
            Self::ArchSummoner => 50,
            Self::PlanarLord => 100,
        }
    }

    /// Get rank from total summoning experience
    pub fn from_xp(xp: u32) -> Self {
        if xp >= Self::PlanarLord.xp_threshold() {
            Self::PlanarLord
        } else if xp >= Self::ArchSummoner.xp_threshold() {
            Self::ArchSummoner
        } else if xp >= Self::MasterSummoner.xp_threshold() {
            Self::MasterSummoner
        } else if xp >= Self::Summoner.xp_threshold() {
            Self::Summoner
        } else if xp >= Self::Journeyman.xp_threshold() {
            Self::Journeyman
        } else {
            Self::Apprentice
        }
    }

    /// Next rank (if any)
    pub fn next(&self) -> Option<Self> {
        match self {
            Self::Apprentice => Some(Self::Journeyman),
            Self::Journeyman => Some(Self::Summoner),
            Self::Summoner => Some(Self::MasterSummoner),
            Self::MasterSummoner => Some(Self::ArchSummoner),
            Self::ArchSummoner => Some(Self::PlanarLord),
            Self::PlanarLord => None,
        }
    }
}

// ============================================================================
// Summon Tiers
// ============================================================================

/// Power tier of summons
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SummonTier {
    Minor,
    Lesser,
    Greater,
    Lord,
    Ancient,
    Primordial,
}

impl SummonTier {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Minor => "Minor",
            Self::Lesser => "Lesser",
            Self::Greater => "Greater",
            Self::Lord => "Lord",
            Self::Ancient => "Ancient",
            Self::Primordial => "Primordial",
        }
    }

    /// Base stat multiplier for this tier
    pub fn stat_multiplier(&self) -> f32 {
        match self {
            Self::Minor => 1.0,
            Self::Lesser => 1.5,
            Self::Greater => 2.5,
            Self::Lord => 4.0,
            Self::Ancient => 6.5,
            Self::Primordial => 10.0,
        }
    }

    /// Mana cost multiplier for summoning
    pub fn mana_cost_multiplier(&self) -> f32 {
        match self {
            Self::Minor => 1.0,
            Self::Lesser => 2.0,
            Self::Greater => 4.0,
            Self::Lord => 8.0,
            Self::Ancient => 16.0,
            Self::Primordial => 32.0,
        }
    }

    /// Summoning ritual time in turns
    pub fn ritual_time(&self) -> u32 {
        match self {
            Self::Minor => 1,
            Self::Lesser => 3,
            Self::Greater => 5,
            Self::Lord => 10,
            Self::Ancient => 20,
            Self::Primordial => 50,
        }
    }

    /// Base failure chance (percentage)
    pub fn base_failure_chance(&self) -> u32 {
        match self {
            Self::Minor => 5,
            Self::Lesser => 10,
            Self::Greater => 20,
            Self::Lord => 35,
            Self::Ancient => 50,
            Self::Primordial => 70,
        }
    }
}

// ============================================================================
// Summon Categories
// ============================================================================

/// Category of summon
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SummonCategory {
    Elemental,
    Beast,
    Spirit,
    Demonic,
    Celestial,
    Mythical,
    Undead,
    Construct,
    Fae,
    Aberration,
}

impl SummonCategory {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Elemental => "Elemental",
            Self::Beast => "Beast",
            Self::Spirit => "Spirit",
            Self::Demonic => "Demonic",
            Self::Celestial => "Celestial",
            Self::Mythical => "Mythical",
            Self::Undead => "Undead",
            Self::Construct => "Construct",
            Self::Fae => "Fae",
            Self::Aberration => "Aberration",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Elemental => "Beings of pure elemental energy from the Elemental Planes.",
            Self::Beast => "Powerful magical beasts and creatures of the wild.",
            Self::Spirit => "Ethereal beings from the Spirit Realm.",
            Self::Demonic => "Dark entities from the Infernal Planes.",
            Self::Celestial => "Holy beings from the Celestial Spheres.",
            Self::Mythical => "Legendary creatures of immense power.",
            Self::Undead => "Beings that have transcended death.",
            Self::Construct => "Magically animated artificial beings.",
            Self::Fae => "Mystical creatures from the Fae Wilds.",
            Self::Aberration => "Strange beings from beyond reality.",
        }
    }

    /// Returns the alignment tendency of this category
    pub fn alignment(&self) -> SummonAlignment {
        match self {
            Self::Celestial => SummonAlignment::Good,
            Self::Demonic | Self::Undead | Self::Aberration => SummonAlignment::Evil,
            _ => SummonAlignment::Neutral,
        }
    }
}

/// Summon alignment
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SummonAlignment {
    Good,
    Neutral,
    Evil,
}

// ============================================================================
// Summon Types (50+ types)
// ============================================================================

/// All available summon types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SummonType {
    // === Elemental Summons (16 types) ===
    MinorFireElemental,
    GreaterFireElemental,
    FireElementalLord,
    MinorWaterElemental,
    GreaterWaterElemental,
    WaterElementalLord,
    MinorEarthElemental,
    GreaterEarthElemental,
    EarthElementalLord,
    MinorAirElemental,
    GreaterAirElemental,
    AirElementalLord,
    LightningElemental,
    StormElementalLord,
    IceElemental,
    MagmaElemental,

    // === Beast Summons (8 types) ===
    WolfPack,
    DireWolf,
    GreatBear,
    GreatEagle,
    GiantSerpent,
    SabertoothTiger,
    Mammoth,
    AlphaWolfLord,

    // === Spirit Summons (6 types) ===
    GuardianSpirit,
    BattleSpirit,
    AncestralSpirit,
    VengefulSpirit,
    NatureSpirit,
    SpiritLord,

    // === Demonic Summons (10 types) ===
    Imp,
    DemonHound,
    ShadowDemon,
    SuccubusIncubus,
    PitFiend,
    GreaterDemon,
    DemonLord,
    Balor,
    ArchDemon,
    DemonPrince,

    // === Celestial Summons (8 types) ===
    LesserAngel,
    GuardianAngel,
    WarriorAngel,
    Seraph,
    Valkyrie,
    SolarAngel,
    Archangel,
    CelestialLord,

    // === Mythical Summons (10 types) ===
    Drake,
    YoungDragon,
    ElderDragon,
    AncientDragon,
    Phoenix,
    Unicorn,
    Griffin,
    Hydra,
    Chimera,
    Wyrm,

    // === Undead Summons (5 types) ===
    SkeletalWarrior,
    SpecterKnight,
    Wraith,
    DeathKnight,
    Lich,

    // === Construct Summons (4 types) ===
    StoneGolem,
    IronGolem,
    RuneGuardian,
    ColossalGolem,

    // === Fae Summons (4 types) ===
    Sprite,
    Dryad,
    FaeKnight,
    ArchFae,

    // === Aberration Summons (3 types) ===
    VoidTentacle,
    MindFlayer,
    ElderThing,
}

impl SummonType {
    /// Returns all summon types
    pub fn all() -> Vec<Self> {
        vec![
            // Elementals
            Self::MinorFireElemental, Self::GreaterFireElemental, Self::FireElementalLord,
            Self::MinorWaterElemental, Self::GreaterWaterElemental, Self::WaterElementalLord,
            Self::MinorEarthElemental, Self::GreaterEarthElemental, Self::EarthElementalLord,
            Self::MinorAirElemental, Self::GreaterAirElemental, Self::AirElementalLord,
            Self::LightningElemental, Self::StormElementalLord, Self::IceElemental, Self::MagmaElemental,
            // Beasts
            Self::WolfPack, Self::DireWolf, Self::GreatBear, Self::GreatEagle,
            Self::GiantSerpent, Self::SabertoothTiger, Self::Mammoth, Self::AlphaWolfLord,
            // Spirits
            Self::GuardianSpirit, Self::BattleSpirit, Self::AncestralSpirit,
            Self::VengefulSpirit, Self::NatureSpirit, Self::SpiritLord,
            // Demonic
            Self::Imp, Self::DemonHound, Self::ShadowDemon, Self::SuccubusIncubus,
            Self::PitFiend, Self::GreaterDemon, Self::DemonLord, Self::Balor,
            Self::ArchDemon, Self::DemonPrince,
            // Celestial
            Self::LesserAngel, Self::GuardianAngel, Self::WarriorAngel, Self::Seraph,
            Self::Valkyrie, Self::SolarAngel, Self::Archangel, Self::CelestialLord,
            // Mythical
            Self::Drake, Self::YoungDragon, Self::ElderDragon, Self::AncientDragon,
            Self::Phoenix, Self::Unicorn, Self::Griffin, Self::Hydra,
            Self::Chimera, Self::Wyrm,
            // Undead
            Self::SkeletalWarrior, Self::SpecterKnight, Self::Wraith,
            Self::DeathKnight, Self::Lich,
            // Constructs
            Self::StoneGolem, Self::IronGolem, Self::RuneGuardian, Self::ColossalGolem,
            // Fae
            Self::Sprite, Self::Dryad, Self::FaeKnight, Self::ArchFae,
            // Aberrations
            Self::VoidTentacle, Self::MindFlayer, Self::ElderThing,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            // Elementals
            Self::MinorFireElemental => "Minor Fire Elemental",
            Self::GreaterFireElemental => "Greater Fire Elemental",
            Self::FireElementalLord => "Fire Elemental Lord",
            Self::MinorWaterElemental => "Minor Water Elemental",
            Self::GreaterWaterElemental => "Greater Water Elemental",
            Self::WaterElementalLord => "Water Elemental Lord",
            Self::MinorEarthElemental => "Minor Earth Elemental",
            Self::GreaterEarthElemental => "Greater Earth Elemental",
            Self::EarthElementalLord => "Earth Elemental Lord",
            Self::MinorAirElemental => "Minor Air Elemental",
            Self::GreaterAirElemental => "Greater Air Elemental",
            Self::AirElementalLord => "Air Elemental Lord",
            Self::LightningElemental => "Lightning Elemental",
            Self::StormElementalLord => "Storm Elemental Lord",
            Self::IceElemental => "Ice Elemental",
            Self::MagmaElemental => "Magma Elemental",
            // Beasts
            Self::WolfPack => "Wolf Pack",
            Self::DireWolf => "Dire Wolf",
            Self::GreatBear => "Great Bear",
            Self::GreatEagle => "Great Eagle",
            Self::GiantSerpent => "Giant Serpent",
            Self::SabertoothTiger => "Sabertooth Tiger",
            Self::Mammoth => "War Mammoth",
            Self::AlphaWolfLord => "Alpha Wolf Lord",
            // Spirits
            Self::GuardianSpirit => "Guardian Spirit",
            Self::BattleSpirit => "Battle Spirit",
            Self::AncestralSpirit => "Ancestral Spirit",
            Self::VengefulSpirit => "Vengeful Spirit",
            Self::NatureSpirit => "Nature Spirit",
            Self::SpiritLord => "Spirit Lord",
            // Demonic
            Self::Imp => "Imp",
            Self::DemonHound => "Demon Hound",
            Self::ShadowDemon => "Shadow Demon",
            Self::SuccubusIncubus => "Succubus/Incubus",
            Self::PitFiend => "Pit Fiend",
            Self::GreaterDemon => "Greater Demon",
            Self::DemonLord => "Demon Lord",
            Self::Balor => "Balor",
            Self::ArchDemon => "Arch Demon",
            Self::DemonPrince => "Demon Prince",
            // Celestial
            Self::LesserAngel => "Lesser Angel",
            Self::GuardianAngel => "Guardian Angel",
            Self::WarriorAngel => "Warrior Angel",
            Self::Seraph => "Seraph",
            Self::Valkyrie => "Valkyrie",
            Self::SolarAngel => "Solar Angel",
            Self::Archangel => "Archangel",
            Self::CelestialLord => "Celestial Lord",
            // Mythical
            Self::Drake => "Drake",
            Self::YoungDragon => "Young Dragon",
            Self::ElderDragon => "Elder Dragon",
            Self::AncientDragon => "Ancient Dragon",
            Self::Phoenix => "Phoenix",
            Self::Unicorn => "Unicorn",
            Self::Griffin => "Griffin",
            Self::Hydra => "Hydra",
            Self::Chimera => "Chimera",
            Self::Wyrm => "Great Wyrm",
            // Undead
            Self::SkeletalWarrior => "Skeletal Warrior",
            Self::SpecterKnight => "Specter Knight",
            Self::Wraith => "Wraith",
            Self::DeathKnight => "Death Knight",
            Self::Lich => "Lich",
            // Constructs
            Self::StoneGolem => "Stone Golem",
            Self::IronGolem => "Iron Golem",
            Self::RuneGuardian => "Rune Guardian",
            Self::ColossalGolem => "Colossal Golem",
            // Fae
            Self::Sprite => "Sprite",
            Self::Dryad => "Dryad",
            Self::FaeKnight => "Fae Knight",
            Self::ArchFae => "Arch Fae",
            // Aberrations
            Self::VoidTentacle => "Void Tentacle",
            Self::MindFlayer => "Mind Flayer",
            Self::ElderThing => "Elder Thing",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::MinorFireElemental => "A small being of living flame, eager to burn.",
            Self::GreaterFireElemental => "A towering inferno of destructive power.",
            Self::FireElementalLord => "A lord of fire commanding lesser flames.",
            Self::MinorWaterElemental => "A fluid being that flows around obstacles.",
            Self::GreaterWaterElemental => "A massive wave of crushing force.",
            Self::WaterElementalLord => "Master of tides and currents.",
            Self::MinorEarthElemental => "A creature of living stone and soil.",
            Self::GreaterEarthElemental => "A walking mountain of immense strength.",
            Self::EarthElementalLord => "Sovereign of stone and metal.",
            Self::MinorAirElemental => "A whirling zephyr of wind.",
            Self::GreaterAirElemental => "A devastating tornado of power.",
            Self::AirElementalLord => "Lord of winds and storms.",
            Self::LightningElemental => "Pure electrical energy given form.",
            Self::StormElementalLord => "Master of thunder and lightning.",
            Self::IceElemental => "Frozen fury given sentient form.",
            Self::MagmaElemental => "The molten heart of the earth walks.",
            Self::WolfPack => "A pack of loyal magical wolves.",
            Self::DireWolf => "A massive wolf of supernatural size.",
            Self::GreatBear => "An enormous bear of terrible strength.",
            Self::GreatEagle => "A majestic eagle of immense wingspan.",
            Self::GiantSerpent => "A massive snake with venomous fangs.",
            Self::SabertoothTiger => "A prehistoric predator of deadly grace.",
            Self::Mammoth => "A war-trained mammoth of immense power.",
            Self::AlphaWolfLord => "The legendary king of all wolves.",
            Self::GuardianSpirit => "A protective spirit that shields allies.",
            Self::BattleSpirit => "An aggressive spirit seeking combat.",
            Self::AncestralSpirit => "The spirit of a powerful ancestor.",
            Self::VengefulSpirit => "A spirit driven by hatred and vengeance.",
            Self::NatureSpirit => "A spirit embodying nature's essence.",
            Self::SpiritLord => "A powerful spirit commanding others.",
            Self::Imp => "A small, mischievous demon.",
            Self::DemonHound => "A hellish hound of fire and shadow.",
            Self::ShadowDemon => "A demon made of living darkness.",
            Self::SuccubusIncubus => "A seductive demon of charm and deceit.",
            Self::PitFiend => "A powerful demon from the deepest pits.",
            Self::GreaterDemon => "A formidable demon of significant power.",
            Self::DemonLord => "A lord commanding legions of demons.",
            Self::Balor => "A massive demon of fire and destruction.",
            Self::ArchDemon => "One of the ruling demons of the Abyss.",
            Self::DemonPrince => "A prince among demons, nearly godlike in power.",
            Self::LesserAngel => "A minor celestial messenger.",
            Self::GuardianAngel => "An angel devoted to protection.",
            Self::WarriorAngel => "A celestial warrior of righteous fury.",
            Self::Seraph => "A six-winged angel of divine fire.",
            Self::Valkyrie => "A chooser of the slain, warrior maiden.",
            Self::SolarAngel => "An angel of blinding radiance.",
            Self::Archangel => "A leader among the angelic host.",
            Self::CelestialLord => "A supreme celestial being of immense power.",
            Self::Drake => "A young draconic creature.",
            Self::YoungDragon => "An adolescent dragon of growing power.",
            Self::ElderDragon => "A mature dragon of great wisdom and might.",
            Self::AncientDragon => "An ancient dragon of legendary power.",
            Self::Phoenix => "An immortal bird of flame and rebirth.",
            Self::Unicorn => "A noble creature of purity and healing.",
            Self::Griffin => "A majestic beast of eagle and lion.",
            Self::Hydra => "A multi-headed serpent that regenerates.",
            Self::Chimera => "A beast combining lion, goat, and serpent.",
            Self::Wyrm => "The greatest of all dragon kind.",
            Self::SkeletalWarrior => "An animated skeleton in ancient armor.",
            Self::SpecterKnight => "A ghostly knight of ethereal power.",
            Self::Wraith => "A malevolent spirit of pure hatred.",
            Self::DeathKnight => "A fallen knight risen in undeath.",
            Self::Lich => "An undead wizard of tremendous power.",
            Self::StoneGolem => "An animated statue of stone.",
            Self::IronGolem => "A powerful construct of enchanted iron.",
            Self::RuneGuardian => "A golem inscribed with powerful runes.",
            Self::ColossalGolem => "A massive golem of incredible strength.",
            Self::Sprite => "A tiny fae creature of mischief.",
            Self::Dryad => "A tree spirit of the deep forest.",
            Self::FaeKnight => "A warrior of the Fae courts.",
            Self::ArchFae => "A powerful lord or lady of the Fae.",
            Self::VoidTentacle => "A tendril from beyond reality.",
            Self::MindFlayer => "A creature that feeds on thoughts.",
            Self::ElderThing => "An incomprehensible being from beyond.",
        }
    }

    pub fn category(&self) -> SummonCategory {
        match self {
            Self::MinorFireElemental | Self::GreaterFireElemental | Self::FireElementalLord |
            Self::MinorWaterElemental | Self::GreaterWaterElemental | Self::WaterElementalLord |
            Self::MinorEarthElemental | Self::GreaterEarthElemental | Self::EarthElementalLord |
            Self::MinorAirElemental | Self::GreaterAirElemental | Self::AirElementalLord |
            Self::LightningElemental | Self::StormElementalLord | Self::IceElemental |
            Self::MagmaElemental => SummonCategory::Elemental,

            Self::WolfPack | Self::DireWolf | Self::GreatBear | Self::GreatEagle |
            Self::GiantSerpent | Self::SabertoothTiger | Self::Mammoth |
            Self::AlphaWolfLord => SummonCategory::Beast,

            Self::GuardianSpirit | Self::BattleSpirit | Self::AncestralSpirit |
            Self::VengefulSpirit | Self::NatureSpirit | Self::SpiritLord => SummonCategory::Spirit,

            Self::Imp | Self::DemonHound | Self::ShadowDemon | Self::SuccubusIncubus |
            Self::PitFiend | Self::GreaterDemon | Self::DemonLord | Self::Balor |
            Self::ArchDemon | Self::DemonPrince => SummonCategory::Demonic,

            Self::LesserAngel | Self::GuardianAngel | Self::WarriorAngel | Self::Seraph |
            Self::Valkyrie | Self::SolarAngel | Self::Archangel |
            Self::CelestialLord => SummonCategory::Celestial,

            Self::Drake | Self::YoungDragon | Self::ElderDragon | Self::AncientDragon |
            Self::Phoenix | Self::Unicorn | Self::Griffin | Self::Hydra |
            Self::Chimera | Self::Wyrm => SummonCategory::Mythical,

            Self::SkeletalWarrior | Self::SpecterKnight | Self::Wraith |
            Self::DeathKnight | Self::Lich => SummonCategory::Undead,

            Self::StoneGolem | Self::IronGolem | Self::RuneGuardian |
            Self::ColossalGolem => SummonCategory::Construct,

            Self::Sprite | Self::Dryad | Self::FaeKnight | Self::ArchFae => SummonCategory::Fae,

            Self::VoidTentacle | Self::MindFlayer | Self::ElderThing => SummonCategory::Aberration,
        }
    }

    pub fn tier(&self) -> SummonTier {
        match self {
            // Minor tier
            Self::MinorFireElemental | Self::MinorWaterElemental | Self::MinorEarthElemental |
            Self::MinorAirElemental | Self::Imp | Self::Sprite |
            Self::SkeletalWarrior | Self::VoidTentacle => SummonTier::Minor,

            // Lesser tier
            Self::WolfPack | Self::LesserAngel | Self::DemonHound |
            Self::GuardianSpirit | Self::Drake | Self::StoneGolem |
            Self::Dryad | Self::IceElemental => SummonTier::Lesser,

            // Greater tier
            Self::GreaterFireElemental | Self::GreaterWaterElemental |
            Self::GreaterEarthElemental | Self::GreaterAirElemental |
            Self::DireWolf | Self::GreatBear | Self::GreatEagle | Self::GiantSerpent |
            Self::BattleSpirit | Self::AncestralSpirit | Self::VengefulSpirit |
            Self::ShadowDemon | Self::SuccubusIncubus | Self::PitFiend |
            Self::GuardianAngel | Self::WarriorAngel | Self::Valkyrie |
            Self::YoungDragon | Self::Griffin | Self::Unicorn |
            Self::SpecterKnight | Self::Wraith | Self::IronGolem |
            Self::FaeKnight | Self::MindFlayer | Self::SabertoothTiger |
            Self::LightningElemental | Self::MagmaElemental |
            Self::NatureSpirit => SummonTier::Greater,

            // Lord tier
            Self::FireElementalLord | Self::WaterElementalLord |
            Self::EarthElementalLord | Self::AirElementalLord |
            Self::AlphaWolfLord | Self::Mammoth | Self::SpiritLord |
            Self::GreaterDemon | Self::DemonLord |
            Self::Seraph | Self::SolarAngel |
            Self::ElderDragon | Self::Phoenix | Self::Hydra | Self::Chimera |
            Self::DeathKnight | Self::RuneGuardian |
            Self::ArchFae | Self::StormElementalLord => SummonTier::Lord,

            // Ancient tier
            Self::Balor | Self::ArchDemon | Self::Archangel |
            Self::AncientDragon | Self::Lich |
            Self::ColossalGolem | Self::ElderThing => SummonTier::Ancient,

            // Primordial tier
            Self::DemonPrince | Self::CelestialLord | Self::Wyrm => SummonTier::Primordial,
        }
    }

    /// Base stats: (hp, attack, defense, mana, speed)
    pub fn base_stats(&self) -> (i32, i32, i32, i32, i32) {
        let tier_mult = self.tier().stat_multiplier();
        let base = match self {
            // Elementals - balanced with elemental focus
            Self::MinorFireElemental => (20, 12, 5, 15, 10),
            Self::GreaterFireElemental => (45, 25, 12, 30, 12),
            Self::FireElementalLord => (100, 50, 25, 60, 14),
            Self::MinorWaterElemental => (25, 8, 8, 20, 8),
            Self::GreaterWaterElemental => (55, 20, 20, 40, 10),
            Self::WaterElementalLord => (120, 40, 40, 80, 12),
            Self::MinorEarthElemental => (35, 10, 12, 10, 5),
            Self::GreaterEarthElemental => (80, 22, 28, 20, 6),
            Self::EarthElementalLord => (180, 45, 55, 40, 8),
            Self::MinorAirElemental => (15, 10, 3, 15, 15),
            Self::GreaterAirElemental => (35, 22, 8, 30, 18),
            Self::AirElementalLord => (80, 45, 18, 60, 22),
            Self::LightningElemental => (40, 35, 10, 40, 20),
            Self::StormElementalLord => (100, 60, 25, 80, 18),
            Self::IceElemental => (50, 18, 18, 35, 8),
            Self::MagmaElemental => (60, 30, 20, 25, 6),

            // Beasts - high hp/attack, low mana
            Self::WolfPack => (30, 15, 8, 5, 14),
            Self::DireWolf => (50, 25, 12, 8, 12),
            Self::GreatBear => (80, 28, 20, 5, 8),
            Self::GreatEagle => (40, 20, 8, 10, 18),
            Self::GiantSerpent => (55, 22, 10, 15, 10),
            Self::SabertoothTiger => (60, 35, 12, 5, 16),
            Self::Mammoth => (150, 35, 30, 5, 6),
            Self::AlphaWolfLord => (100, 40, 25, 20, 14),

            // Spirits - balanced, good mana
            Self::GuardianSpirit => (40, 10, 25, 40, 10),
            Self::BattleSpirit => (50, 25, 15, 30, 12),
            Self::AncestralSpirit => (60, 20, 20, 50, 10),
            Self::VengefulSpirit => (45, 30, 10, 35, 14),
            Self::NatureSpirit => (55, 15, 18, 45, 12),
            Self::SpiritLord => (120, 40, 35, 80, 12),

            // Demonic - high attack, decent defense
            Self::Imp => (15, 8, 4, 20, 16),
            Self::DemonHound => (35, 22, 10, 15, 14),
            Self::ShadowDemon => (50, 28, 12, 35, 16),
            Self::SuccubusIncubus => (45, 20, 10, 50, 14),
            Self::PitFiend => (70, 35, 25, 40, 10),
            Self::GreaterDemon => (100, 45, 30, 50, 12),
            Self::DemonLord => (150, 60, 40, 70, 14),
            Self::Balor => (200, 75, 50, 60, 12),
            Self::ArchDemon => (250, 90, 60, 80, 14),
            Self::DemonPrince => (400, 120, 80, 120, 16),

            // Celestial - balanced with healing/buffs
            Self::LesserAngel => (30, 15, 15, 40, 12),
            Self::GuardianAngel => (60, 20, 30, 50, 10),
            Self::WarriorAngel => (70, 35, 25, 40, 14),
            Self::Seraph => (100, 45, 35, 70, 16),
            Self::Valkyrie => (80, 40, 28, 45, 14),
            Self::SolarAngel => (130, 55, 45, 80, 14),
            Self::Archangel => (180, 70, 55, 100, 16),
            Self::CelestialLord => (300, 100, 80, 150, 18),

            // Mythical - very high stats
            Self::Drake => (45, 22, 15, 25, 12),
            Self::YoungDragon => (80, 40, 30, 50, 14),
            Self::ElderDragon => (150, 70, 55, 80, 12),
            Self::AncientDragon => (250, 100, 80, 120, 14),
            Self::Phoenix => (100, 50, 25, 80, 18),
            Self::Unicorn => (70, 25, 20, 100, 16),
            Self::Griffin => (75, 35, 25, 30, 16),
            Self::Hydra => (180, 60, 40, 40, 8),
            Self::Chimera => (120, 55, 35, 50, 12),
            Self::Wyrm => (500, 150, 120, 200, 10),

            // Undead - high hp, resistance focus
            Self::SkeletalWarrior => (25, 15, 12, 5, 8),
            Self::SpecterKnight => (50, 25, 20, 20, 10),
            Self::Wraith => (40, 30, 8, 40, 14),
            Self::DeathKnight => (120, 50, 45, 40, 10),
            Self::Lich => (80, 40, 30, 150, 8),

            // Constructs - very high hp/defense, no mana
            Self::StoneGolem => (60, 20, 30, 0, 4),
            Self::IronGolem => (100, 30, 45, 0, 5),
            Self::RuneGuardian => (140, 40, 50, 30, 6),
            Self::ColossalGolem => (300, 60, 80, 0, 3),

            // Fae - low hp, high mana/speed
            Self::Sprite => (10, 5, 3, 30, 20),
            Self::Dryad => (35, 12, 10, 50, 12),
            Self::FaeKnight => (55, 28, 18, 40, 16),
            Self::ArchFae => (100, 45, 30, 120, 18),

            // Aberrations - unusual stat distributions
            Self::VoidTentacle => (20, 18, 5, 25, 12),
            Self::MindFlayer => (60, 25, 15, 100, 10),
            Self::ElderThing => (200, 80, 60, 150, 8),
        };

        (
            (base.0 as f32 * tier_mult) as i32,
            (base.1 as f32 * tier_mult) as i32,
            (base.2 as f32 * tier_mult) as i32,
            (base.3 as f32 * tier_mult) as i32,
            base.4, // Speed doesn't scale with tier
        )
    }

    /// Returns the glyph for display
    pub fn glyph(&self) -> char {
        match self.category() {
            SummonCategory::Elemental => 'E',
            SummonCategory::Beast => 'B',
            SummonCategory::Spirit => 'S',
            SummonCategory::Demonic => 'D',
            SummonCategory::Celestial => 'A',
            SummonCategory::Mythical => 'M',
            SummonCategory::Undead => 'U',
            SummonCategory::Construct => 'G',
            SummonCategory::Fae => 'F',
            SummonCategory::Aberration => 'X',
        }
    }

    /// Returns evolution options for this summon type
    pub fn evolution_options(&self) -> Vec<Self> {
        match self {
            // Elemental evolution chains
            Self::MinorFireElemental => vec![Self::GreaterFireElemental],
            Self::GreaterFireElemental => vec![Self::FireElementalLord, Self::MagmaElemental],
            Self::MinorWaterElemental => vec![Self::GreaterWaterElemental, Self::IceElemental],
            Self::GreaterWaterElemental => vec![Self::WaterElementalLord],
            Self::MinorEarthElemental => vec![Self::GreaterEarthElemental],
            Self::GreaterEarthElemental => vec![Self::EarthElementalLord, Self::MagmaElemental],
            Self::MinorAirElemental => vec![Self::GreaterAirElemental, Self::LightningElemental],
            Self::GreaterAirElemental => vec![Self::AirElementalLord, Self::StormElementalLord],
            Self::LightningElemental => vec![Self::StormElementalLord],

            // Beast evolution chains
            Self::WolfPack => vec![Self::DireWolf],
            Self::DireWolf => vec![Self::AlphaWolfLord],
            Self::Drake => vec![Self::YoungDragon],
            Self::YoungDragon => vec![Self::ElderDragon],
            Self::ElderDragon => vec![Self::AncientDragon],
            Self::AncientDragon => vec![Self::Wyrm],

            // Spirit evolution chains
            Self::GuardianSpirit => vec![Self::AncestralSpirit, Self::NatureSpirit],
            Self::BattleSpirit => vec![Self::VengefulSpirit, Self::SpiritLord],
            Self::AncestralSpirit => vec![Self::SpiritLord],

            // Demonic evolution chains
            Self::Imp => vec![Self::DemonHound, Self::ShadowDemon],
            Self::DemonHound => vec![Self::PitFiend],
            Self::ShadowDemon => vec![Self::GreaterDemon],
            Self::PitFiend => vec![Self::GreaterDemon, Self::Balor],
            Self::GreaterDemon => vec![Self::DemonLord],
            Self::DemonLord => vec![Self::ArchDemon],
            Self::Balor => vec![Self::ArchDemon],
            Self::ArchDemon => vec![Self::DemonPrince],

            // Celestial evolution chains
            Self::LesserAngel => vec![Self::GuardianAngel, Self::WarriorAngel],
            Self::GuardianAngel => vec![Self::Seraph],
            Self::WarriorAngel => vec![Self::Valkyrie, Self::SolarAngel],
            Self::Seraph => vec![Self::Archangel],
            Self::SolarAngel => vec![Self::Archangel],
            Self::Archangel => vec![Self::CelestialLord],

            // Undead evolution chains
            Self::SkeletalWarrior => vec![Self::SpecterKnight, Self::Wraith],
            Self::SpecterKnight => vec![Self::DeathKnight],
            Self::Wraith => vec![Self::Lich],
            Self::DeathKnight => vec![Self::Lich],

            // Construct evolution chains
            Self::StoneGolem => vec![Self::IronGolem],
            Self::IronGolem => vec![Self::RuneGuardian],
            Self::RuneGuardian => vec![Self::ColossalGolem],

            // Fae evolution chains
            Self::Sprite => vec![Self::Dryad],
            Self::Dryad => vec![Self::FaeKnight],
            Self::FaeKnight => vec![Self::ArchFae],

            // Aberration evolution chains
            Self::VoidTentacle => vec![Self::MindFlayer],
            Self::MindFlayer => vec![Self::ElderThing],

            _ => vec![],
        }
    }

    /// Experience required to evolve
    pub fn evolution_xp_required(&self) -> u32 {
        match self.tier() {
            SummonTier::Minor => 500,
            SummonTier::Lesser => 1500,
            SummonTier::Greater => 5000,
            SummonTier::Lord => 15000,
            SummonTier::Ancient => 50000,
            SummonTier::Primordial => 0, // Cannot evolve further
        }
    }
}

// ============================================================================
// Summoning Materials
// ============================================================================

/// Materials required for summoning rituals
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SummoningMaterial {
    // Common materials
    Candle,
    Chalk,
    Incense,
    Salt,
    HolyWater,
    CursedWater,

    // Elemental materials
    FireEssence,
    WaterEssence,
    EarthEssence,
    AirEssence,
    LightningEssence,
    IceEssence,
    MagmaEssence,

    // Rare materials
    DragonScale,
    PhoenixFeather,
    UnicornHair,
    DemonBlood,
    AngelFeather,
    GhostEctoplasm,
    FaeDust,
    VoidFragment,

    // Legendary materials
    DragonHeart,
    PhoenixAsh,
    DemonSoul,
    CelestialEssence,
    AncientRune,
    PrimordialShard,
}

impl SummoningMaterial {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Candle => "Ritual Candle",
            Self::Chalk => "Mystical Chalk",
            Self::Incense => "Sacred Incense",
            Self::Salt => "Purifying Salt",
            Self::HolyWater => "Holy Water",
            Self::CursedWater => "Cursed Water",
            Self::FireEssence => "Fire Essence",
            Self::WaterEssence => "Water Essence",
            Self::EarthEssence => "Earth Essence",
            Self::AirEssence => "Air Essence",
            Self::LightningEssence => "Lightning Essence",
            Self::IceEssence => "Ice Essence",
            Self::MagmaEssence => "Magma Essence",
            Self::DragonScale => "Dragon Scale",
            Self::PhoenixFeather => "Phoenix Feather",
            Self::UnicornHair => "Unicorn Hair",
            Self::DemonBlood => "Demon Blood",
            Self::AngelFeather => "Angel Feather",
            Self::GhostEctoplasm => "Ghost Ectoplasm",
            Self::FaeDust => "Fae Dust",
            Self::VoidFragment => "Void Fragment",
            Self::DragonHeart => "Dragon Heart",
            Self::PhoenixAsh => "Phoenix Ash",
            Self::DemonSoul => "Demon Soul",
            Self::CelestialEssence => "Celestial Essence",
            Self::AncientRune => "Ancient Rune",
            Self::PrimordialShard => "Primordial Shard",
        }
    }

    pub fn rarity(&self) -> MaterialRarity {
        match self {
            Self::Candle | Self::Chalk | Self::Incense | Self::Salt => MaterialRarity::Common,
            Self::HolyWater | Self::CursedWater | Self::FireEssence | Self::WaterEssence |
            Self::EarthEssence | Self::AirEssence => MaterialRarity::Uncommon,
            Self::LightningEssence | Self::IceEssence | Self::MagmaEssence |
            Self::DragonScale | Self::PhoenixFeather | Self::UnicornHair |
            Self::DemonBlood | Self::AngelFeather | Self::GhostEctoplasm |
            Self::FaeDust | Self::VoidFragment => MaterialRarity::Rare,
            Self::DragonHeart | Self::PhoenixAsh | Self::DemonSoul |
            Self::CelestialEssence | Self::AncientRune | Self::PrimordialShard => MaterialRarity::Legendary,
        }
    }

    pub fn gold_value(&self) -> u32 {
        match self.rarity() {
            MaterialRarity::Common => 10,
            MaterialRarity::Uncommon => 50,
            MaterialRarity::Rare => 250,
            MaterialRarity::Legendary => 1000,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MaterialRarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

// ============================================================================
// Summoning Circle
// ============================================================================

/// Types of summoning circles
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CircleType {
    Basic,           // Simple circle for minor summons
    Reinforced,      // Stronger containment
    Elemental,       // Specialized for elementals
    Demonic,         // For demonic summons (pentagram)
    Celestial,       // For celestial summons (heptagram)
    Spirit,          // For spirit summons
    Binding,         // For permanent contracts
    Planar,          // For cross-planar summons
    Primordial,      // For the most powerful summons
}

impl CircleType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Basic => "Basic Circle",
            Self::Reinforced => "Reinforced Circle",
            Self::Elemental => "Elemental Circle",
            Self::Demonic => "Demonic Pentagram",
            Self::Celestial => "Celestial Heptagram",
            Self::Spirit => "Spirit Circle",
            Self::Binding => "Binding Circle",
            Self::Planar => "Planar Gateway",
            Self::Primordial => "Primordial Seal",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Basic => "A simple chalk circle for minor summons.",
            Self::Reinforced => "A strengthened circle with better containment.",
            Self::Elemental => "A circle attuned to elemental energies.",
            Self::Demonic => "A five-pointed star for binding demons.",
            Self::Celestial => "A seven-pointed star for calling celestials.",
            Self::Spirit => "A circle designed for spirit communication.",
            Self::Binding => "A complex circle for permanent contracts.",
            Self::Planar => "A gateway circle for reaching other planes.",
            Self::Primordial => "An ancient seal for the most powerful beings.",
        }
    }

    /// Materials required to create this circle
    pub fn required_materials(&self) -> Vec<(SummoningMaterial, u32)> {
        match self {
            Self::Basic => vec![
                (SummoningMaterial::Chalk, 1),
                (SummoningMaterial::Candle, 4),
            ],
            Self::Reinforced => vec![
                (SummoningMaterial::Chalk, 2),
                (SummoningMaterial::Salt, 1),
                (SummoningMaterial::Candle, 6),
            ],
            Self::Elemental => vec![
                (SummoningMaterial::Chalk, 2),
                (SummoningMaterial::Candle, 4),
                (SummoningMaterial::Incense, 2),
            ],
            Self::Demonic => vec![
                (SummoningMaterial::Chalk, 3),
                (SummoningMaterial::CursedWater, 1),
                (SummoningMaterial::DemonBlood, 1),
                (SummoningMaterial::Candle, 5),
            ],
            Self::Celestial => vec![
                (SummoningMaterial::Chalk, 3),
                (SummoningMaterial::HolyWater, 2),
                (SummoningMaterial::AngelFeather, 1),
                (SummoningMaterial::Candle, 7),
            ],
            Self::Spirit => vec![
                (SummoningMaterial::Chalk, 2),
                (SummoningMaterial::Incense, 3),
                (SummoningMaterial::GhostEctoplasm, 1),
                (SummoningMaterial::Candle, 4),
            ],
            Self::Binding => vec![
                (SummoningMaterial::Chalk, 4),
                (SummoningMaterial::Salt, 2),
                (SummoningMaterial::AncientRune, 1),
                (SummoningMaterial::Candle, 8),
            ],
            Self::Planar => vec![
                (SummoningMaterial::Chalk, 5),
                (SummoningMaterial::VoidFragment, 2),
                (SummoningMaterial::AncientRune, 2),
                (SummoningMaterial::Candle, 12),
            ],
            Self::Primordial => vec![
                (SummoningMaterial::PrimordialShard, 3),
                (SummoningMaterial::AncientRune, 4),
                (SummoningMaterial::DragonHeart, 1),
                (SummoningMaterial::CelestialEssence, 1),
                (SummoningMaterial::DemonSoul, 1),
                (SummoningMaterial::Candle, 16),
            ],
        }
    }

    /// Maximum summon tier this circle can contain
    pub fn max_tier(&self) -> SummonTier {
        match self {
            Self::Basic => SummonTier::Minor,
            Self::Reinforced => SummonTier::Lesser,
            Self::Elemental | Self::Spirit => SummonTier::Greater,
            Self::Demonic | Self::Celestial => SummonTier::Lord,
            Self::Binding | Self::Planar => SummonTier::Ancient,
            Self::Primordial => SummonTier::Primordial,
        }
    }

    /// Failure chance reduction (percentage)
    pub fn failure_reduction(&self) -> u32 {
        match self {
            Self::Basic => 0,
            Self::Reinforced => 5,
            Self::Elemental | Self::Spirit => 10,
            Self::Demonic | Self::Celestial => 15,
            Self::Binding => 20,
            Self::Planar => 25,
            Self::Primordial => 35,
        }
    }
}

// ============================================================================
// Summoning Ritual
// ============================================================================

/// Represents an active summoning ritual
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SummoningRitual {
    pub summon_type: SummonType,
    pub circle_type: CircleType,
    pub turns_remaining: u32,
    pub total_turns: u32,
    pub mana_invested: i32,
    pub materials_consumed: Vec<(SummoningMaterial, u32)>,
    pub interrupted: bool,
}

impl SummoningRitual {
    pub fn new(summon_type: SummonType, circle_type: CircleType, summoner_rank: SummonerRank) -> Self {
        let base_time = summon_type.tier().ritual_time();
        let rank_reduction = match summoner_rank {
            SummonerRank::Apprentice => 0,
            SummonerRank::Journeyman => 1,
            SummonerRank::Summoner => 2,
            SummonerRank::MasterSummoner => 3,
            SummonerRank::ArchSummoner => 5,
            SummonerRank::PlanarLord => 8,
        };
        let turns = base_time.saturating_sub(rank_reduction).max(1);

        Self {
            summon_type,
            circle_type,
            turns_remaining: turns,
            total_turns: turns,
            mana_invested: 0,
            materials_consumed: Vec::new(),
            interrupted: false,
        }
    }

    /// Advance the ritual by one turn
    pub fn tick(&mut self) -> bool {
        if self.turns_remaining > 0 {
            self.turns_remaining -= 1;
        }
        self.turns_remaining == 0
    }

    /// Check if the ritual is complete
    pub fn is_complete(&self) -> bool {
        self.turns_remaining == 0 && !self.interrupted
    }

    /// Interrupt the ritual (e.g., from taking damage)
    pub fn interrupt(&mut self) {
        self.interrupted = true;
    }

    /// Progress percentage
    pub fn progress(&self) -> f32 {
        if self.total_turns == 0 {
            return 1.0;
        }
        1.0 - (self.turns_remaining as f32 / self.total_turns as f32)
    }
}

// ============================================================================
// Contracts and Binding
// ============================================================================

/// Type of contract binding a summon
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractType {
    Temporary,    // Lasts for a set duration
    Permanent,    // Lasts until dismissed or destroyed
    Familiar,     // Deep bond with permanent connection
    Forced,       // Bound against will (risky)
    Willing,      // Mutual agreement (most stable)
}

impl ContractType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Temporary => "Temporary Contract",
            Self::Permanent => "Permanent Contract",
            Self::Familiar => "Familiar Bond",
            Self::Forced => "Forced Binding",
            Self::Willing => "Willing Pact",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Temporary => "A short-term summoning that fades with time.",
            Self::Permanent => "A lasting contract binding the entity indefinitely.",
            Self::Familiar => "A deep spiritual bond between summoner and summon.",
            Self::Forced => "The entity is bound against its will. Risky.",
            Self::Willing => "A mutual agreement between summoner and entity.",
        }
    }

    /// Control bonus or penalty
    pub fn control_modifier(&self) -> i32 {
        match self {
            Self::Temporary => 0,
            Self::Permanent => 5,
            Self::Familiar => 20,
            Self::Forced => -15,
            Self::Willing => 10,
        }
    }

    /// Power bonus or penalty for the summon
    pub fn power_modifier(&self) -> f32 {
        match self {
            Self::Temporary => 1.0,
            Self::Permanent => 1.1,
            Self::Familiar => 1.25,
            Self::Forced => 0.8,
            Self::Willing => 1.15,
        }
    }

    /// Duration multiplier (for temporary contracts)
    pub fn duration_multiplier(&self) -> f32 {
        match self {
            Self::Temporary => 1.0,
            Self::Permanent => f32::INFINITY,
            Self::Familiar => f32::INFINITY,
            Self::Forced => 0.5,
            Self::Willing => 2.0,
        }
    }

    /// Risk of rebellion
    pub fn rebellion_risk(&self) -> u32 {
        match self {
            Self::Temporary => 5,
            Self::Permanent => 10,
            Self::Familiar => 0,
            Self::Forced => 40,
            Self::Willing => 2,
        }
    }
}

/// A contract binding a summon to the summoner
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SummonContract {
    pub contract_type: ContractType,
    pub signed_at: u64,         // Game turn when contract was made
    pub duration: Option<u32>,  // None for permanent
    pub loyalty: i32,           // -100 to 100
    pub broken: bool,
    pub terms: Vec<ContractTerm>,
}

impl SummonContract {
    pub fn new_temporary(duration: u32, current_turn: u64) -> Self {
        Self {
            contract_type: ContractType::Temporary,
            signed_at: current_turn,
            duration: Some(duration),
            loyalty: 50,
            broken: false,
            terms: vec![ContractTerm::ProtectSummoner, ContractTerm::ObeyCommands],
        }
    }

    pub fn new_permanent(current_turn: u64) -> Self {
        Self {
            contract_type: ContractType::Permanent,
            signed_at: current_turn,
            duration: None,
            loyalty: 60,
            broken: false,
            terms: vec![
                ContractTerm::ProtectSummoner,
                ContractTerm::ObeyCommands,
                ContractTerm::NoHarmToSummoner,
            ],
        }
    }

    pub fn new_familiar(current_turn: u64) -> Self {
        Self {
            contract_type: ContractType::Familiar,
            signed_at: current_turn,
            duration: None,
            loyalty: 100,
            broken: false,
            terms: vec![
                ContractTerm::ProtectSummoner,
                ContractTerm::ObeyCommands,
                ContractTerm::NoHarmToSummoner,
                ContractTerm::ShareSenses,
                ContractTerm::LifeLink,
            ],
        }
    }

    pub fn new_forced(current_turn: u64) -> Self {
        Self {
            contract_type: ContractType::Forced,
            signed_at: current_turn,
            duration: Some(TEMPORARY_SUMMON_BASE_DURATION / 2),
            loyalty: -20,
            broken: false,
            terms: vec![ContractTerm::ObeyCommands],
        }
    }

    pub fn new_willing(current_turn: u64) -> Self {
        Self {
            contract_type: ContractType::Willing,
            signed_at: current_turn,
            duration: None,
            loyalty: 80,
            broken: false,
            terms: vec![
                ContractTerm::ProtectSummoner,
                ContractTerm::ObeyCommands,
                ContractTerm::NoHarmToSummoner,
                ContractTerm::MutualRespect,
            ],
        }
    }

    /// Check if contract has expired
    pub fn is_expired(&self, current_turn: u64) -> bool {
        if let Some(duration) = self.duration {
            current_turn >= self.signed_at + duration as u64
        } else {
            false
        }
    }

    /// Modify loyalty
    pub fn modify_loyalty(&mut self, amount: i32) {
        self.loyalty = (self.loyalty + amount).clamp(-100, 100);
        if self.loyalty <= -80 {
            self.broken = true;
        }
    }

    /// Check for rebellion based on loyalty and contract type
    pub fn check_rebellion(&self, rng: &mut impl Rng) -> bool {
        if self.broken {
            return true;
        }

        let base_risk = self.contract_type.rebellion_risk();
        let loyalty_modifier = if self.loyalty < 0 {
            (-self.loyalty) as u32
        } else {
            0
        };

        let total_risk = base_risk + loyalty_modifier;
        rng.gen_range(0..100) < total_risk
    }
}

/// Terms that can be part of a contract
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractTerm {
    ProtectSummoner,    // Will defend the summoner
    ObeyCommands,       // Will follow commands
    NoHarmToSummoner,   // Cannot harm the summoner
    ShareSenses,        // Summoner can see through summon's senses
    LifeLink,           // HP shared between summoner and summon
    MutualRespect,      // Both parties respect each other
    NoKilling,          // Summon won't kill
    GoldTribute,        // Requires gold payment
    SoulTribute,        // Requires soul energy
    TimeLimited,        // Only active during certain conditions
}

// ============================================================================
// Summon Skills
// ============================================================================

/// Skills that summons can learn
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SummonSkill {
    // Combat skills
    PowerAttack,
    MultiStrike,
    Cleave,
    ArmorBreak,
    CriticalStrike,
    Ferocity,

    // Defensive skills
    Shield,
    Regeneration,
    Evasion,
    Fortitude,
    ElementalResist,
    MagicResist,

    // Elemental skills
    FireBlast,
    IceStorm,
    LightningBolt,
    Earthquake,
    Whirlwind,
    VoidRay,

    // Support skills
    Heal,
    Buff,
    Debuff,
    Taunt,
    Aura,
    Summon, // Some summons can summon minions

    // Special skills
    Flight,
    Teleport,
    Invisibility,
    Rebirth,
    SoulDrain,
    MindControl,
}

impl SummonSkill {
    pub fn name(&self) -> &'static str {
        match self {
            Self::PowerAttack => "Power Attack",
            Self::MultiStrike => "Multi-Strike",
            Self::Cleave => "Cleave",
            Self::ArmorBreak => "Armor Break",
            Self::CriticalStrike => "Critical Strike",
            Self::Ferocity => "Ferocity",
            Self::Shield => "Shield",
            Self::Regeneration => "Regeneration",
            Self::Evasion => "Evasion",
            Self::Fortitude => "Fortitude",
            Self::ElementalResist => "Elemental Resist",
            Self::MagicResist => "Magic Resist",
            Self::FireBlast => "Fire Blast",
            Self::IceStorm => "Ice Storm",
            Self::LightningBolt => "Lightning Bolt",
            Self::Earthquake => "Earthquake",
            Self::Whirlwind => "Whirlwind",
            Self::VoidRay => "Void Ray",
            Self::Heal => "Heal",
            Self::Buff => "Empower",
            Self::Debuff => "Weaken",
            Self::Taunt => "Taunt",
            Self::Aura => "Aura",
            Self::Summon => "Summon Minion",
            Self::Flight => "Flight",
            Self::Teleport => "Teleport",
            Self::Invisibility => "Invisibility",
            Self::Rebirth => "Rebirth",
            Self::SoulDrain => "Soul Drain",
            Self::MindControl => "Mind Control",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::PowerAttack => "A devastating attack dealing 200% damage.",
            Self::MultiStrike => "Attack multiple times in rapid succession.",
            Self::Cleave => "Attack hits all adjacent enemies.",
            Self::ArmorBreak => "Reduces enemy defense significantly.",
            Self::CriticalStrike => "Increased critical hit chance and damage.",
            Self::Ferocity => "Attack speed increased when HP is low.",
            Self::Shield => "Creates a protective barrier.",
            Self::Regeneration => "Slowly regenerates health over time.",
            Self::Evasion => "Chance to completely avoid attacks.",
            Self::Fortitude => "Reduces all incoming damage.",
            Self::ElementalResist => "Resists elemental damage.",
            Self::MagicResist => "Resists magical attacks.",
            Self::FireBlast => "Launches a ball of fire.",
            Self::IceStorm => "Creates a storm of ice shards.",
            Self::LightningBolt => "Strikes with lightning.",
            Self::Earthquake => "Shakes the ground, damaging all nearby.",
            Self::Whirlwind => "Creates a powerful wind attack.",
            Self::VoidRay => "Fires a beam of void energy.",
            Self::Heal => "Restores health to allies.",
            Self::Buff => "Increases ally stats temporarily.",
            Self::Debuff => "Decreases enemy stats temporarily.",
            Self::Taunt => "Forces enemies to attack this summon.",
            Self::Aura => "Provides passive bonuses to nearby allies.",
            Self::Summon => "Summons additional minions.",
            Self::Flight => "Can fly over obstacles.",
            Self::Teleport => "Can teleport short distances.",
            Self::Invisibility => "Can become invisible.",
            Self::Rebirth => "Resurrects once upon death.",
            Self::SoulDrain => "Drains life force from enemies.",
            Self::MindControl => "Can control enemy minds.",
        }
    }

    pub fn mana_cost(&self) -> i32 {
        match self {
            Self::PowerAttack => 15,
            Self::MultiStrike => 20,
            Self::Cleave => 18,
            Self::ArmorBreak => 12,
            Self::CriticalStrike => 10,
            Self::Ferocity => 0, // Passive
            Self::Shield => 25,
            Self::Regeneration => 0, // Passive
            Self::Evasion => 0, // Passive
            Self::Fortitude => 0, // Passive
            Self::ElementalResist => 0, // Passive
            Self::MagicResist => 0, // Passive
            Self::FireBlast => 20,
            Self::IceStorm => 30,
            Self::LightningBolt => 25,
            Self::Earthquake => 40,
            Self::Whirlwind => 22,
            Self::VoidRay => 50,
            Self::Heal => 30,
            Self::Buff => 20,
            Self::Debuff => 18,
            Self::Taunt => 10,
            Self::Aura => 0, // Passive
            Self::Summon => 60,
            Self::Flight => 5,
            Self::Teleport => 35,
            Self::Invisibility => 25,
            Self::Rebirth => 0, // Auto-trigger
            Self::SoulDrain => 40,
            Self::MindControl => 80,
        }
    }

    pub fn cooldown(&self) -> u32 {
        match self {
            Self::PowerAttack => 3,
            Self::MultiStrike => 4,
            Self::Cleave => 2,
            Self::ArmorBreak => 5,
            Self::CriticalStrike => 0, // Passive
            Self::Ferocity => 0, // Passive
            Self::Shield => 10,
            Self::Regeneration => 0, // Passive
            Self::Evasion => 0, // Passive
            Self::Fortitude => 0, // Passive
            Self::ElementalResist => 0, // Passive
            Self::MagicResist => 0, // Passive
            Self::FireBlast => 3,
            Self::IceStorm => 5,
            Self::LightningBolt => 4,
            Self::Earthquake => 8,
            Self::Whirlwind => 4,
            Self::VoidRay => 10,
            Self::Heal => 6,
            Self::Buff => 8,
            Self::Debuff => 6,
            Self::Taunt => 5,
            Self::Aura => 0, // Passive
            Self::Summon => 20,
            Self::Flight => 1,
            Self::Teleport => 8,
            Self::Invisibility => 12,
            Self::Rebirth => 100, // Once per battle
            Self::SoulDrain => 7,
            Self::MindControl => 30,
        }
    }

    /// Level required to learn this skill
    pub fn level_required(&self) -> u32 {
        match self {
            Self::PowerAttack | Self::Shield | Self::FireBlast => 1,
            Self::MultiStrike | Self::Heal | Self::IceStorm => 5,
            Self::Cleave | Self::Evasion | Self::LightningBolt | Self::Taunt => 10,
            Self::ArmorBreak | Self::Regeneration | Self::Earthquake | Self::Buff => 15,
            Self::CriticalStrike | Self::Fortitude | Self::Whirlwind | Self::Debuff => 20,
            Self::Ferocity | Self::ElementalResist | Self::VoidRay | Self::Flight => 25,
            Self::MagicResist | Self::Aura | Self::Teleport => 30,
            Self::Invisibility | Self::Summon => 35,
            Self::Rebirth | Self::SoulDrain => 40,
            Self::MindControl => 45,
        }
    }
}

// ============================================================================
// Summoned Entity
// ============================================================================

/// A summoned entity instance
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SummonedEntity {
    pub id: u64,
    pub summon_type: SummonType,
    pub name: String,
    pub x: usize,
    pub y: usize,

    // Stats
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub mana: i32,
    pub max_mana: i32,
    pub speed: i32,

    // Progression
    pub level: u32,
    pub xp: u32,
    pub xp_to_level: u32,

    // Contract and control
    pub contract: SummonContract,
    pub control_level: i32,  // Higher = better control

    // Skills
    pub skills: Vec<SummonSkill>,
    pub skill_cooldowns: HashMap<SummonSkill, u32>,

    // State
    pub is_active: bool,
    pub turns_remaining: Option<u32>,  // For temporary summons
    pub has_rebirth: bool,

    // Combat tracking
    pub kills: u32,
    pub damage_dealt: u64,
    pub damage_taken: u64,
}

impl SummonedEntity {
    pub fn new(
        id: u64,
        summon_type: SummonType,
        contract: SummonContract,
        summoner_rank: SummonerRank,
        x: usize,
        y: usize,
    ) -> Self {
        let (base_hp, base_atk, base_def, base_mana, speed) = summon_type.base_stats();
        let power_mod = contract.contract_type.power_modifier();
        let rank_bonus = 1.0 + (summoner_rank as u32 as f32 * 0.1);

        let max_hp = ((base_hp as f32) * power_mod * rank_bonus) as i32;
        let max_mana = ((base_mana as f32) * power_mod * rank_bonus) as i32;

        let duration = if contract.duration.is_some() {
            Some((TEMPORARY_SUMMON_BASE_DURATION as f32 * contract.contract_type.duration_multiplier()) as u32)
        } else {
            None
        };

        let mut skills = Vec::new();
        // Grant starting skills based on summon type
        match summon_type.category() {
            SummonCategory::Elemental => {
                skills.push(SummonSkill::FireBlast);
                skills.push(SummonSkill::ElementalResist);
            }
            SummonCategory::Beast => {
                skills.push(SummonSkill::PowerAttack);
                skills.push(SummonSkill::Ferocity);
            }
            SummonCategory::Spirit => {
                skills.push(SummonSkill::Shield);
                skills.push(SummonSkill::Heal);
            }
            SummonCategory::Demonic => {
                skills.push(SummonSkill::PowerAttack);
                skills.push(SummonSkill::SoulDrain);
            }
            SummonCategory::Celestial => {
                skills.push(SummonSkill::Heal);
                skills.push(SummonSkill::Shield);
                skills.push(SummonSkill::Aura);
            }
            SummonCategory::Mythical => {
                skills.push(SummonSkill::PowerAttack);
                skills.push(SummonSkill::FireBlast);
                skills.push(SummonSkill::Flight);
            }
            SummonCategory::Undead => {
                skills.push(SummonSkill::SoulDrain);
                skills.push(SummonSkill::Fortitude);
            }
            SummonCategory::Construct => {
                skills.push(SummonSkill::Fortitude);
                skills.push(SummonSkill::Taunt);
            }
            SummonCategory::Fae => {
                skills.push(SummonSkill::Heal);
                skills.push(SummonSkill::Invisibility);
            }
            SummonCategory::Aberration => {
                skills.push(SummonSkill::VoidRay);
                skills.push(SummonSkill::MindControl);
            }
        }

        Self {
            id,
            summon_type,
            name: summon_type.name().to_string(),
            x,
            y,
            hp: max_hp,
            max_hp,
            attack: ((base_atk as f32) * power_mod * rank_bonus) as i32,
            defense: ((base_def as f32) * power_mod * rank_bonus) as i32,
            mana: max_mana,
            max_mana,
            speed,
            level: 1,
            xp: 0,
            xp_to_level: XP_PER_SUMMON_LEVEL[0],
            contract,
            control_level: summoner_rank.control_bonus(),
            skills,
            skill_cooldowns: HashMap::new(),
            is_active: true,
            turns_remaining: duration,
            has_rebirth: summon_type == SummonType::Phoenix,
            kills: 0,
            damage_dealt: 0,
            damage_taken: 0,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    /// Take damage, returns actual damage taken
    pub fn take_damage(&mut self, amount: i32) -> i32 {
        let actual = (amount - self.defense).max(1);
        self.hp -= actual;
        self.damage_taken += actual as u64;

        // Phoenix rebirth
        if self.hp <= 0 && self.has_rebirth {
            self.hp = self.max_hp / 2;
            self.has_rebirth = false;
            return -1; // Signal rebirth
        }

        actual
    }

    /// Heal the summon
    pub fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }

    /// Restore mana
    pub fn restore_mana(&mut self, amount: i32) {
        self.mana = (self.mana + amount).min(self.max_mana);
    }

    /// Use mana, returns true if successful
    pub fn use_mana(&mut self, amount: i32) -> bool {
        if self.mana >= amount {
            self.mana -= amount;
            true
        } else {
            false
        }
    }

    /// Gain XP and return true if leveled up
    pub fn gain_xp(&mut self, amount: u32) -> bool {
        self.xp += amount;
        if self.xp >= self.xp_to_level && self.level < MAX_SUMMON_LEVEL {
            self.xp -= self.xp_to_level;
            self.level += 1;

            if (self.level as usize) < XP_PER_SUMMON_LEVEL.len() {
                self.xp_to_level = XP_PER_SUMMON_LEVEL[self.level as usize - 1];
            }

            // Stat increases on level up
            let hp_gain = 5 + (self.level / 5) as i32;
            self.max_hp += hp_gain;
            self.hp += hp_gain;
            self.attack += 2;
            self.defense += 1;
            self.max_mana += 3;

            // Improve loyalty slightly on level up
            self.contract.modify_loyalty(2);

            return true;
        }
        false
    }

    /// Check if summon can use a skill
    pub fn can_use_skill(&self, skill: SummonSkill) -> bool {
        if !self.skills.contains(&skill) {
            return false;
        }
        if self.mana < skill.mana_cost() {
            return false;
        }
        if let Some(&cooldown) = self.skill_cooldowns.get(&skill) {
            if cooldown > 0 {
                return false;
            }
        }
        true
    }

    /// Use a skill
    pub fn use_skill(&mut self, skill: SummonSkill) -> bool {
        if !self.can_use_skill(skill) {
            return false;
        }

        self.mana -= skill.mana_cost();
        self.skill_cooldowns.insert(skill, skill.cooldown());
        true
    }

    /// Tick cooldowns and duration
    pub fn tick(&mut self) {
        // Reduce skill cooldowns
        for (_, cooldown) in self.skill_cooldowns.iter_mut() {
            *cooldown = cooldown.saturating_sub(1);
        }

        // Reduce duration for temporary summons
        if let Some(ref mut duration) = self.turns_remaining {
            *duration = duration.saturating_sub(1);
            if *duration == 0 {
                self.is_active = false;
            }
        }

        // Passive regeneration
        if self.skills.contains(&SummonSkill::Regeneration) {
            self.heal(self.max_hp / 50); // 2% regen per turn
        }

        // Mana regeneration
        self.restore_mana(self.max_mana / 20); // 5% mana regen per turn
    }

    /// Check if summon can evolve
    pub fn can_evolve(&self) -> bool {
        let options = self.summon_type.evolution_options();
        !options.is_empty() && self.xp >= self.summon_type.evolution_xp_required()
    }

    /// Get available evolution options
    pub fn get_evolution_options(&self) -> Vec<SummonType> {
        if self.can_evolve() {
            self.summon_type.evolution_options()
        } else {
            vec![]
        }
    }

    /// Evolve the summon to a new type
    pub fn evolve(&mut self, new_type: SummonType) -> bool {
        let options = self.summon_type.evolution_options();
        if !options.contains(&new_type) {
            return false;
        }
        if self.xp < self.summon_type.evolution_xp_required() {
            return false;
        }

        // Deduct evolution XP
        self.xp -= self.summon_type.evolution_xp_required();

        // Get new base stats
        let (base_hp, base_atk, base_def, base_mana, speed) = new_type.base_stats();

        // Calculate stat bonus from levels gained
        let level_hp_bonus = (5 + self.level as i32 / 5) * self.level as i32;
        let level_atk_bonus = 2 * self.level as i32;
        let level_def_bonus = self.level as i32;
        let level_mana_bonus = 3 * self.level as i32;

        // Apply new stats
        self.max_hp = base_hp + level_hp_bonus;
        self.hp = self.max_hp;
        self.attack = base_atk + level_atk_bonus;
        self.defense = base_def + level_def_bonus;
        self.max_mana = base_mana + level_mana_bonus;
        self.mana = self.max_mana;
        self.speed = speed;

        // Update type and name
        self.summon_type = new_type;
        self.name = new_type.name().to_string();

        // Phoenix rebirth resets on evolution
        if new_type == SummonType::Phoenix {
            self.has_rebirth = true;
        }

        true
    }

    /// Learn a new skill if eligible
    pub fn try_learn_skill(&mut self, skill: SummonSkill) -> bool {
        if self.skills.contains(&skill) {
            return false;
        }
        if self.level < skill.level_required() {
            return false;
        }
        self.skills.push(skill);
        true
    }

    /// Display name with level
    pub fn display_name(&self) -> String {
        format!("{} Lv.{}", self.name, self.level)
    }

    /// Stats summary
    pub fn stats_summary(&self) -> String {
        format!(
            "{}: HP {}/{} MP {}/{} ATK {} DEF {} SPD {}",
            self.display_name(),
            self.hp, self.max_hp,
            self.mana, self.max_mana,
            self.attack, self.defense, self.speed
        )
    }
}

// ============================================================================
// Failure and Risks
// ============================================================================

/// What happens when summoning fails
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SummoningFailure {
    Fizzle,              // Nothing happens, mana wasted
    Backlash(i32),       // Summoner takes damage
    WildSummon,          // Random creature appears
    HostileSummon,       // Intended creature appears but hostile
    DimensionalRift,     // Opens unstable portal (special event)
    SummonerCorruption,  // Summoner gains corruption
    MaterialsLost,       // Materials consumed but nothing happens
}

impl SummoningFailure {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Fizzle => "Fizzle",
            Self::Backlash(_) => "Magical Backlash",
            Self::WildSummon => "Wild Summon",
            Self::HostileSummon => "Hostile Summon",
            Self::DimensionalRift => "Dimensional Rift",
            Self::SummonerCorruption => "Summoner Corruption",
            Self::MaterialsLost => "Materials Lost",
        }
    }

    pub fn description(&self) -> String {
        match self {
            Self::Fizzle => "The ritual fizzles. Mana is wasted.".to_string(),
            Self::Backlash(dmg) => format!("Magical backlash! Take {} damage.", dmg),
            Self::WildSummon => "A random creature answers the call!".to_string(),
            Self::HostileSummon => "The creature appears but refuses to obey!".to_string(),
            Self::DimensionalRift => "A rift tears through reality!".to_string(),
            Self::SummonerCorruption => "Dark energies seep into your soul.".to_string(),
            Self::MaterialsLost => "The materials are consumed but nothing happens.".to_string(),
        }
    }

    /// Generate a random failure based on summon tier
    pub fn random_for_tier(tier: SummonTier, rng: &mut impl Rng) -> Self {
        let roll = rng.gen_range(0..100);
        match tier {
            SummonTier::Minor | SummonTier::Lesser => {
                if roll < 50 { Self::Fizzle }
                else if roll < 80 { Self::MaterialsLost }
                else { Self::Backlash(rng.gen_range(5..15)) }
            }
            SummonTier::Greater => {
                if roll < 30 { Self::Fizzle }
                else if roll < 50 { Self::MaterialsLost }
                else if roll < 70 { Self::Backlash(rng.gen_range(15..35)) }
                else if roll < 90 { Self::WildSummon }
                else { Self::HostileSummon }
            }
            SummonTier::Lord | SummonTier::Ancient => {
                if roll < 20 { Self::Fizzle }
                else if roll < 35 { Self::Backlash(rng.gen_range(30..60)) }
                else if roll < 55 { Self::WildSummon }
                else if roll < 75 { Self::HostileSummon }
                else if roll < 90 { Self::SummonerCorruption }
                else { Self::DimensionalRift }
            }
            SummonTier::Primordial => {
                if roll < 10 { Self::Fizzle }
                else if roll < 25 { Self::Backlash(rng.gen_range(50..100)) }
                else if roll < 40 { Self::WildSummon }
                else if roll < 60 { Self::HostileSummon }
                else if roll < 80 { Self::SummonerCorruption }
                else { Self::DimensionalRift }
            }
        }
    }
}

// ============================================================================
// Summoning System
// ============================================================================

/// Main summoning system manager
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SummoningSystem {
    // Summoner stats
    pub rank: SummonerRank,
    pub total_xp: u32,
    pub summons_performed: u32,
    pub successful_summons: u32,
    pub failed_summons: u32,

    // Active summons
    pub active_summons: Vec<SummonedEntity>,
    pub familiar: Option<SummonedEntity>,

    // Materials inventory
    pub materials: HashMap<SummoningMaterial, u32>,

    // Current ritual (if any)
    pub current_ritual: Option<SummoningRitual>,

    // Known summon types (unlocked for summoning)
    pub known_summons: Vec<SummonType>,

    // Statistics
    pub total_summon_kills: u32,
    pub total_summon_damage: u64,
    pub summons_lost: u32,

    // ID counter for summons
    next_summon_id: u64,
}

impl Default for SummoningSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl SummoningSystem {
    pub fn new() -> Self {
        // Start with basic summons known
        let known_summons = vec![
            SummonType::MinorFireElemental,
            SummonType::MinorWaterElemental,
            SummonType::MinorEarthElemental,
            SummonType::MinorAirElemental,
            SummonType::Imp,
            SummonType::WolfPack,
            SummonType::GuardianSpirit,
            SummonType::SkeletalWarrior,
        ];

        // Start with some basic materials
        let mut materials = HashMap::new();
        materials.insert(SummoningMaterial::Chalk, 10);
        materials.insert(SummoningMaterial::Candle, 20);
        materials.insert(SummoningMaterial::Incense, 5);
        materials.insert(SummoningMaterial::Salt, 5);

        Self {
            rank: SummonerRank::Apprentice,
            total_xp: 0,
            summons_performed: 0,
            successful_summons: 0,
            failed_summons: 0,
            active_summons: Vec::new(),
            familiar: None,
            materials,
            current_ritual: None,
            known_summons,
            total_summon_kills: 0,
            total_summon_damage: 0,
            summons_lost: 0,
            next_summon_id: 1,
        }
    }

    /// Get the next unique summon ID
    fn next_id(&mut self) -> u64 {
        let id = self.next_summon_id;
        self.next_summon_id += 1;
        id
    }

    /// Add summoning XP and check for rank up
    pub fn add_xp(&mut self, amount: u32) -> Option<SummonerRank> {
        let old_rank = self.rank;
        self.total_xp += amount;
        self.rank = SummonerRank::from_xp(self.total_xp);

        if self.rank != old_rank {
            Some(self.rank)
        } else {
            None
        }
    }

    /// Check if a summon type is known
    pub fn knows_summon(&self, summon_type: SummonType) -> bool {
        self.known_summons.contains(&summon_type)
    }

    /// Learn a new summon type
    pub fn learn_summon(&mut self, summon_type: SummonType) -> bool {
        if self.knows_summon(summon_type) {
            return false;
        }
        self.known_summons.push(summon_type);
        true
    }

    /// Check if player can summon the given type
    pub fn can_summon(&self, summon_type: SummonType) -> Result<(), String> {
        // Check if known
        if !self.knows_summon(summon_type) {
            return Err("You don't know how to summon this entity.".to_string());
        }

        // Check rank
        if summon_type.tier() > self.rank.max_summon_tier() {
            return Err(format!(
                "Your rank is too low. Need {} to summon {} tier.",
                self.rank.next().map(|r| r.name()).unwrap_or("higher rank"),
                summon_type.tier().name()
            ));
        }

        // Check active summon limit
        if self.active_summons.len() >= self.rank.max_summons() {
            return Err("You have too many active summons.".to_string());
        }

        // Check if ritual already in progress
        if self.current_ritual.is_some() {
            return Err("A summoning ritual is already in progress.".to_string());
        }

        Ok(())
    }

    /// Check if player has required materials for a circle
    pub fn has_materials_for_circle(&self, circle_type: CircleType) -> bool {
        for (material, required) in circle_type.required_materials() {
            let have = self.materials.get(&material).copied().unwrap_or(0);
            if have < required {
                return false;
            }
        }
        true
    }

    /// Calculate mana cost for summoning
    pub fn calculate_mana_cost(&self, summon_type: SummonType) -> i32 {
        let base = BASE_SUMMON_MANA_COST;
        let tier_mult = summon_type.tier().mana_cost_multiplier();
        let rank_reduction = self.rank.mana_cost_reduction();

        let cost = (base as f32 * tier_mult * (1.0 - rank_reduction)) as i32;
        cost.max(10) // Minimum cost
    }

    /// Start a summoning ritual
    pub fn start_ritual(
        &mut self,
        summon_type: SummonType,
        circle_type: CircleType,
    ) -> Result<(), String> {
        self.can_summon(summon_type)?;

        // Check circle can contain this tier
        if summon_type.tier() > circle_type.max_tier() {
            return Err(format!(
                "This circle cannot contain {} tier summons.",
                summon_type.tier().name()
            ));
        }

        // Check and consume materials
        if !self.has_materials_for_circle(circle_type) {
            return Err("You don't have the required materials.".to_string());
        }

        // Consume materials
        let materials_consumed: Vec<_> = circle_type.required_materials();
        for (material, amount) in &materials_consumed {
            if let Some(count) = self.materials.get_mut(material) {
                *count -= amount;
            }
        }

        // Create ritual
        let mut ritual = SummoningRitual::new(summon_type, circle_type, self.rank);
        ritual.materials_consumed = materials_consumed;

        self.current_ritual = Some(ritual);
        Ok(())
    }

    /// Advance the current ritual by one turn
    pub fn tick_ritual(&mut self) -> Option<bool> {
        if let Some(ref mut ritual) = self.current_ritual {
            Some(ritual.tick())
        } else {
            None
        }
    }

    /// Complete the ritual and attempt to summon
    pub fn complete_ritual(
        &mut self,
        player_mana: i32,
        current_turn: u64,
        x: usize,
        y: usize,
        rng: &mut impl Rng,
    ) -> Result<SummonedEntity, SummoningFailure> {
        let ritual = self.current_ritual.take()
            .ok_or(SummoningFailure::Fizzle)?;

        if ritual.interrupted {
            self.failed_summons += 1;
            return Err(SummoningFailure::Fizzle);
        }

        if !ritual.is_complete() {
            self.current_ritual = Some(ritual);
            return Err(SummoningFailure::Fizzle);
        }

        // Check mana
        let mana_cost = self.calculate_mana_cost(ritual.summon_type);
        if player_mana < mana_cost {
            self.failed_summons += 1;
            return Err(SummoningFailure::Backlash(mana_cost / 2));
        }

        self.summons_performed += 1;

        // Calculate failure chance
        let base_failure = ritual.summon_type.tier().base_failure_chance();
        let circle_reduction = ritual.circle_type.failure_reduction();
        let rank_reduction = self.rank as u32 * 3;
        let final_failure = base_failure.saturating_sub(circle_reduction).saturating_sub(rank_reduction);

        // Check for failure
        if rng.gen_range(0..100) < final_failure {
            self.failed_summons += 1;
            return Err(SummoningFailure::random_for_tier(ritual.summon_type.tier(), rng));
        }

        // Success!
        self.successful_summons += 1;

        // Create contract based on tier and alignment
        let contract = match ritual.summon_type.category().alignment() {
            SummonAlignment::Good => SummonContract::new_willing(current_turn),
            SummonAlignment::Neutral => SummonContract::new_temporary(
                TEMPORARY_SUMMON_BASE_DURATION,
                current_turn
            ),
            SummonAlignment::Evil => SummonContract::new_forced(current_turn),
        };

        // Create the summoned entity
        let id = self.next_id();
        let summon = SummonedEntity::new(
            id,
            ritual.summon_type,
            contract,
            self.rank,
            x,
            y,
        );

        // Award summoning XP
        let xp_gain = match ritual.summon_type.tier() {
            SummonTier::Minor => 10,
            SummonTier::Lesser => 25,
            SummonTier::Greater => 50,
            SummonTier::Lord => 100,
            SummonTier::Ancient => 250,
            SummonTier::Primordial => 500,
        };
        self.add_xp(xp_gain);

        // Add to active summons
        self.active_summons.push(summon.clone());

        Ok(summon)
    }

    /// Dismiss a summon
    pub fn dismiss_summon(&mut self, summon_id: u64) -> bool {
        if let Some(pos) = self.active_summons.iter().position(|s| s.id == summon_id) {
            self.active_summons.remove(pos);
            true
        } else {
            false
        }
    }

    /// Make a summon into a familiar
    pub fn make_familiar(&mut self, summon_id: u64, current_turn: u64) -> Result<(), String> {
        if self.familiar.is_some() {
            return Err("You already have a familiar.".to_string());
        }

        let pos = self.active_summons.iter().position(|s| s.id == summon_id)
            .ok_or("Summon not found.")?;

        let mut summon = self.active_summons.remove(pos);

        // Upgrade contract to familiar bond
        summon.contract = SummonContract::new_familiar(current_turn);
        summon.turns_remaining = None; // Permanent
        summon.control_level += 20;

        self.familiar = Some(summon);
        Ok(())
    }

    /// Release familiar
    pub fn release_familiar(&mut self) -> Option<SummonedEntity> {
        self.familiar.take()
    }

    /// Add material to inventory
    pub fn add_material(&mut self, material: SummoningMaterial, amount: u32) {
        *self.materials.entry(material).or_insert(0) += amount;
    }

    /// Remove material from inventory
    pub fn remove_material(&mut self, material: SummoningMaterial, amount: u32) -> bool {
        if let Some(count) = self.materials.get_mut(&material) {
            if *count >= amount {
                *count -= amount;
                return true;
            }
        }
        false
    }

    /// Tick all active summons
    pub fn tick_summons(&mut self, current_turn: u64) {
        // Tick each summon
        for summon in &mut self.active_summons {
            summon.tick();

            // Check contract expiration
            if summon.contract.is_expired(current_turn) {
                summon.is_active = false;
            }
        }

        // Tick familiar
        if let Some(ref mut familiar) = self.familiar {
            familiar.tick();
        }

        // Remove inactive summons
        let lost_count = self.active_summons.iter().filter(|s| !s.is_active || !s.is_alive()).count();
        self.summons_lost += lost_count as u32;
        self.active_summons.retain(|s| s.is_active && s.is_alive());
    }

    /// Get total number of summons (including familiar)
    pub fn total_summons(&self) -> usize {
        self.active_summons.len() + if self.familiar.is_some() { 1 } else { 0 }
    }

    /// Get all active summons as references (including familiar)
    pub fn all_summons(&self) -> Vec<&SummonedEntity> {
        let mut summons: Vec<&SummonedEntity> = self.active_summons.iter().collect();
        if let Some(ref familiar) = self.familiar {
            summons.push(familiar);
        }
        summons
    }

    /// Get mutable reference to a summon by ID
    pub fn get_summon_mut(&mut self, id: u64) -> Option<&mut SummonedEntity> {
        if let Some(ref mut familiar) = self.familiar {
            if familiar.id == id {
                return Some(familiar);
            }
        }
        self.active_summons.iter_mut().find(|s| s.id == id)
    }

    /// Calculate total stat bonus from summons for player
    pub fn get_summon_bonuses(&self) -> SummonBonuses {
        let mut bonuses = SummonBonuses::default();

        for summon in self.all_summons() {
            // Familiars provide direct bonuses
            if summon.contract.contract_type == ContractType::Familiar {
                bonuses.hp_bonus += summon.max_hp / 10;
                bonuses.mana_bonus += summon.max_mana / 5;
                bonuses.attack_bonus += summon.attack / 10;
                bonuses.defense_bonus += summon.defense / 10;
            }

            // Aura skill provides bonuses
            if summon.skills.contains(&SummonSkill::Aura) {
                bonuses.attack_bonus += 5;
                bonuses.defense_bonus += 3;
            }
        }

        bonuses
    }

    /// Get summary of the summoning system state
    pub fn get_summary(&self) -> SummoningSummary {
        SummoningSummary {
            rank: self.rank,
            total_xp: self.total_xp,
            xp_to_next_rank: self.rank.next().map(|r| r.xp_threshold()),
            active_summons: self.active_summons.len(),
            max_summons: self.rank.max_summons(),
            has_familiar: self.familiar.is_some(),
            known_summons_count: self.known_summons.len(),
            summons_performed: self.summons_performed,
            success_rate: if self.summons_performed > 0 {
                (self.successful_summons as f32 / self.summons_performed as f32) * 100.0
            } else {
                0.0
            },
        }
    }
}

/// Bonuses provided by summons to the player
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SummonBonuses {
    pub hp_bonus: i32,
    pub mana_bonus: i32,
    pub attack_bonus: i32,
    pub defense_bonus: i32,
}

/// Summary of summoning system state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SummoningSummary {
    pub rank: SummonerRank,
    pub total_xp: u32,
    pub xp_to_next_rank: Option<u32>,
    pub active_summons: usize,
    pub max_summons: usize,
    pub has_familiar: bool,
    pub known_summons_count: usize,
    pub summons_performed: u32,
    pub success_rate: f32,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summoner_rank_progression() {
        assert_eq!(SummonerRank::from_xp(0), SummonerRank::Apprentice);
        assert_eq!(SummonerRank::from_xp(1000), SummonerRank::Journeyman);
        assert_eq!(SummonerRank::from_xp(5000), SummonerRank::Summoner);
        assert_eq!(SummonerRank::from_xp(500000), SummonerRank::PlanarLord);
    }

    #[test]
    fn test_summon_type_count() {
        let all_types = SummonType::all();
        assert!(all_types.len() >= 50, "Should have at least 50 summon types");
    }

    #[test]
    fn test_summoning_system_creation() {
        let system = SummoningSystem::new();
        assert_eq!(system.rank, SummonerRank::Apprentice);
        assert!(!system.known_summons.is_empty());
        assert!(system.active_summons.is_empty());
    }

    #[test]
    fn test_material_management() {
        let mut system = SummoningSystem::new();
        system.add_material(SummoningMaterial::DragonScale, 5);
        assert_eq!(system.materials.get(&SummoningMaterial::DragonScale), Some(&5));

        assert!(system.remove_material(SummoningMaterial::DragonScale, 3));
        assert_eq!(system.materials.get(&SummoningMaterial::DragonScale), Some(&2));

        assert!(!system.remove_material(SummoningMaterial::DragonScale, 5));
    }

    #[test]
    fn test_summon_evolution() {
        let summon_type = SummonType::MinorFireElemental;
        let options = summon_type.evolution_options();
        assert!(!options.is_empty());
        assert!(options.contains(&SummonType::GreaterFireElemental));
    }

    #[test]
    fn test_circle_requirements() {
        let basic = CircleType::Basic;
        let materials = basic.required_materials();
        assert!(!materials.is_empty());
        assert!(materials.iter().any(|(m, _)| *m == SummoningMaterial::Chalk));
    }

    #[test]
    fn test_summoned_entity_creation() {
        let contract = SummonContract::new_temporary(100, 0);
        let entity = SummonedEntity::new(
            1,
            SummonType::MinorFireElemental,
            contract,
            SummonerRank::Apprentice,
            5,
            5,
        );
        assert!(entity.is_alive());
        assert_eq!(entity.level, 1);
        assert!(!entity.skills.is_empty());
    }

    #[test]
    fn test_contract_types() {
        let temp = ContractType::Temporary;
        let familiar = ContractType::Familiar;

        assert!(familiar.control_modifier() > temp.control_modifier());
        assert!(familiar.power_modifier() > temp.power_modifier());
        assert!(temp.rebellion_risk() > familiar.rebellion_risk());
    }

    #[test]
    fn test_summon_xp_and_levelup() {
        let contract = SummonContract::new_temporary(100, 0);
        let mut entity = SummonedEntity::new(
            1,
            SummonType::MinorFireElemental,
            contract,
            SummonerRank::Apprentice,
            5,
            5,
        );

        let initial_level = entity.level;
        entity.xp = entity.xp_to_level - 1;
        let leveled = entity.gain_xp(10);

        assert!(leveled);
        assert_eq!(entity.level, initial_level + 1);
    }

    #[test]
    fn test_skill_usage() {
        let contract = SummonContract::new_temporary(100, 0);
        let mut entity = SummonedEntity::new(
            1,
            SummonType::MinorFireElemental,
            contract,
            SummonerRank::Apprentice,
            5,
            5,
        );

        // Elementals start with FireBlast
        assert!(entity.skills.contains(&SummonSkill::FireBlast));
        assert!(entity.can_use_skill(SummonSkill::FireBlast));

        entity.use_skill(SummonSkill::FireBlast);
        assert!(!entity.can_use_skill(SummonSkill::FireBlast)); // On cooldown
    }

    #[test]
    fn test_tier_ordering() {
        assert!(SummonTier::Minor < SummonTier::Lesser);
        assert!(SummonTier::Lesser < SummonTier::Greater);
        assert!(SummonTier::Greater < SummonTier::Lord);
        assert!(SummonTier::Lord < SummonTier::Ancient);
        assert!(SummonTier::Ancient < SummonTier::Primordial);
    }

    #[test]
    fn test_all_summons_have_stats() {
        for summon_type in SummonType::all() {
            let stats = summon_type.base_stats();
            assert!(stats.0 > 0, "{:?} should have HP", summon_type);
            assert!(stats.1 > 0, "{:?} should have attack", summon_type);
        }
    }

    #[test]
    fn test_summoning_failure_generation() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let failure = SummoningFailure::random_for_tier(SummonTier::Greater, &mut rng);
            // Should not panic
            let _ = failure.name();
            let _ = failure.description();
        }
    }
}
