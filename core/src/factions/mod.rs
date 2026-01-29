//! Comprehensive Faction and Reputation System
//!
//! A detailed faction system featuring 30+ major factions across kingdoms, cultivation sects,
//! guilds, races, secret societies, and monster factions. Includes reputation tracking,
//! faction relationships, territory control, and faction wars.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// Link to other modules
use crate::cultivation::CultivationRealm;
use crate::guilds::Guild;
use crate::species::Species;
use crate::kingdoms::Kingdom;

// =============================================================================
// CONSTANTS
// =============================================================================

/// Reputation thresholds
pub const REPUTATION_HATED_MIN: i32 = -1000;
pub const REPUTATION_HATED_MAX: i32 = -750;
pub const REPUTATION_HOSTILE_MIN: i32 = -749;
pub const REPUTATION_HOSTILE_MAX: i32 = -500;
pub const REPUTATION_UNFRIENDLY_MIN: i32 = -499;
pub const REPUTATION_UNFRIENDLY_MAX: i32 = -250;
pub const REPUTATION_NEUTRAL_MIN: i32 = -249;
pub const REPUTATION_NEUTRAL_MAX: i32 = 249;
pub const REPUTATION_FRIENDLY_MIN: i32 = 250;
pub const REPUTATION_FRIENDLY_MAX: i32 = 499;
pub const REPUTATION_HONORED_MIN: i32 = 500;
pub const REPUTATION_HONORED_MAX: i32 = 749;
pub const REPUTATION_REVERED_MIN: i32 = 750;
pub const REPUTATION_REVERED_MAX: i32 = 999;
pub const REPUTATION_EXALTED_MIN: i32 = 1000;

/// Maximum reputation value
pub const MAX_REPUTATION: i32 = 2000;
/// Minimum reputation value
pub const MIN_REPUTATION: i32 = -1000;

/// War contribution thresholds
pub const WAR_CONTRIBUTION_BRONZE: u32 = 100;
pub const WAR_CONTRIBUTION_SILVER: u32 = 500;
pub const WAR_CONTRIBUTION_GOLD: u32 = 1000;
pub const WAR_CONTRIBUTION_PLATINUM: u32 = 5000;
pub const WAR_CONTRIBUTION_LEGENDARY: u32 = 10000;

// =============================================================================
// FACTION CATEGORIES
// =============================================================================

/// Major faction categories in the game world
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FactionCategory {
    /// Political kingdoms and empires
    Kingdom,
    /// Cultivation sects for spiritual advancement
    CultivationSect,
    /// Professional guilds
    Guild,
    /// Racial factions
    Race,
    /// Hidden organizations
    SecretSociety,
    /// Monster kingdoms and tribes
    MonsterFaction,
    /// Religious orders
    ReligiousOrder,
    /// Mercenary companies
    MercenaryCompany,
}

impl FactionCategory {
    pub fn all() -> &'static [FactionCategory] {
        &[
            Self::Kingdom,
            Self::CultivationSect,
            Self::Guild,
            Self::Race,
            Self::SecretSociety,
            Self::MonsterFaction,
            Self::ReligiousOrder,
            Self::MercenaryCompany,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Kingdom => "Kingdom",
            Self::CultivationSect => "Cultivation Sect",
            Self::Guild => "Guild",
            Self::Race => "Racial Faction",
            Self::SecretSociety => "Secret Society",
            Self::MonsterFaction => "Monster Faction",
            Self::ReligiousOrder => "Religious Order",
            Self::MercenaryCompany => "Mercenary Company",
        }
    }
}

// =============================================================================
// FACTION IDENTIFIERS (30+ FACTIONS)
// =============================================================================

/// All faction identifiers in the game
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FactionId {
    // =========================================================================
    // KINGDOMS (3)
    // =========================================================================
    /// The holy Kingdom of Light - bastion of righteousness
    KingdomOfLight,
    /// The malevolent Dark Empire - realm of shadows
    DarkEmpire,
    /// The Neutral Territories - free cities and independent states
    NeutralTerritories,

    // =========================================================================
    // CULTIVATION SECTS (6)
    // =========================================================================
    // Orthodox Sects
    /// Heavenly Sword Sect - righteous sword cultivators
    HeavenlySwordSect,
    /// Jade Phoenix Pavilion - healing and support cultivation
    JadePhoenixPavilion,
    /// Azure Cloud Monastery - scholarly cultivation
    AzureCloudMonastery,

    // Demonic Sects
    /// Blood Moon Cult - demonic cultivation through sacrifice
    BloodMoonCult,
    /// Shadow Viper Sect - assassination and poison cultivation
    ShadowViperSect,
    /// Soul Devouring Palace - soul cultivation through dark arts
    SoulDevouringPalace,

    // Buddhist Temples
    /// Temple of Enlightenment - Buddhist cultivation
    TempleOfEnlightenment,
    /// Thousand Buddha Monastery - martial Buddhist monks
    ThousandBuddhaMonastery,

    // =========================================================================
    // GUILDS (12 - linked to Guild enum)
    // =========================================================================
    /// Blades Brotherhood - warriors guild
    GuildBladesBrotherhood,
    /// Arcanum Circle - mages guild
    GuildArcanumCircle,
    /// Shadow Syndicate - thieves guild
    GuildShadowSyndicate,
    /// Silent Blade - assassins guild
    GuildSilentBlade,
    /// Golden Coin - merchants guild
    GuildGoldenCoin,
    /// Master Artisans - crafters guild
    GuildMasterArtisans,
    /// Pathfinders Union - adventurers guild
    GuildPathfindersUnion,
    /// White Rose - healers guild
    GuildWhiteRose,
    /// Bone Collectors - necromancers guild
    GuildBoneCollectors,
    /// Beast Slayers - monster hunters
    GuildBeastSlayers,
    /// Philosophers Stone - alchemists guild
    GuildPhilosophersStone,
    /// Cartographers Society - explorers guild
    GuildCartographersSociety,

    // =========================================================================
    // RACES (12 - linked to Species enum)
    // =========================================================================
    /// Human Alliance - united human kingdoms
    HumanAlliance,
    /// Elven Courts - united elven nations
    ElvenCourts,
    /// Dwarven Clans - mountain dwarf confederation
    DwarvenClans,
    /// Orcish Horde - united orc tribes
    OrcishHorde,
    /// Undead Collective - organized undead society
    UndeadCollective,
    /// Demonic Legion - organized demon hierarchy
    DemonicLegion,
    /// Beastkin Tribes - united beastfolk
    BeastkinTribes,
    /// Dragonborn Council - dragon-descended peoples
    DragonbornCouncil,
    /// Fae Courts - seelie and unseelie combined
    FaeCourts,
    /// Construct Assembly - sentient construct collective
    ConstructAssembly,
    /// Elemental Conclave - elemental beings council
    ElementalConclave,
    /// Celestial Host - divine beings
    CelestialHost,

    // =========================================================================
    // SECRET SOCIETIES (6)
    // =========================================================================
    /// The Illuminated - shadow council controlling world events
    TheIlluminated,
    /// Order of the Eternal Eye - ancient order of seers
    OrderOfTheEternalEye,
    /// Brotherhood of Shadows - assassin brotherhood
    BrotherhoodOfShadows,
    /// The Veiled Hand - puppet masters of politics
    TheVeiledHand,
    /// Circle of Whispers - information brokers network
    CircleOfWhispers,
    /// Keepers of the Void - guardians of forbidden knowledge
    KeepersOfTheVoid,

    // =========================================================================
    // MONSTER FACTIONS (6)
    // =========================================================================
    // Beast Kingdoms
    /// Primal Beast Kingdom - intelligent beast civilization
    PrimalBeastKingdom,
    /// Serpent Empire - snake-like creatures
    SerpentEmpire,

    // Undead Empire
    /// Necropolis Dominion - organized undead empire
    NecropolisDominion,
    /// Vampire Courts - vampire noble houses
    VampireCourts,

    // Demon Realm
    /// Infernal Legions - demon army
    InfernalLegions,
    /// Abyssal Horde - chaotic demon swarm
    AbyssalHorde,
}

impl FactionId {
    /// Returns all faction IDs
    pub fn all() -> &'static [FactionId] {
        &[
            // Kingdoms
            Self::KingdomOfLight,
            Self::DarkEmpire,
            Self::NeutralTerritories,
            // Cultivation Sects
            Self::HeavenlySwordSect,
            Self::JadePhoenixPavilion,
            Self::AzureCloudMonastery,
            Self::BloodMoonCult,
            Self::ShadowViperSect,
            Self::SoulDevouringPalace,
            Self::TempleOfEnlightenment,
            Self::ThousandBuddhaMonastery,
            // Guilds
            Self::GuildBladesBrotherhood,
            Self::GuildArcanumCircle,
            Self::GuildShadowSyndicate,
            Self::GuildSilentBlade,
            Self::GuildGoldenCoin,
            Self::GuildMasterArtisans,
            Self::GuildPathfindersUnion,
            Self::GuildWhiteRose,
            Self::GuildBoneCollectors,
            Self::GuildBeastSlayers,
            Self::GuildPhilosophersStone,
            Self::GuildCartographersSociety,
            // Races
            Self::HumanAlliance,
            Self::ElvenCourts,
            Self::DwarvenClans,
            Self::OrcishHorde,
            Self::UndeadCollective,
            Self::DemonicLegion,
            Self::BeastkinTribes,
            Self::DragonbornCouncil,
            Self::FaeCourts,
            Self::ConstructAssembly,
            Self::ElementalConclave,
            Self::CelestialHost,
            // Secret Societies
            Self::TheIlluminated,
            Self::OrderOfTheEternalEye,
            Self::BrotherhoodOfShadows,
            Self::TheVeiledHand,
            Self::CircleOfWhispers,
            Self::KeepersOfTheVoid,
            // Monster Factions
            Self::PrimalBeastKingdom,
            Self::SerpentEmpire,
            Self::NecropolisDominion,
            Self::VampireCourts,
            Self::InfernalLegions,
            Self::AbyssalHorde,
        ]
    }

    /// Returns the category this faction belongs to
    pub fn category(&self) -> FactionCategory {
        match self {
            Self::KingdomOfLight | Self::DarkEmpire | Self::NeutralTerritories => {
                FactionCategory::Kingdom
            }
            Self::HeavenlySwordSect
            | Self::JadePhoenixPavilion
            | Self::AzureCloudMonastery
            | Self::BloodMoonCult
            | Self::ShadowViperSect
            | Self::SoulDevouringPalace
            | Self::TempleOfEnlightenment
            | Self::ThousandBuddhaMonastery => FactionCategory::CultivationSect,
            Self::GuildBladesBrotherhood
            | Self::GuildArcanumCircle
            | Self::GuildShadowSyndicate
            | Self::GuildSilentBlade
            | Self::GuildGoldenCoin
            | Self::GuildMasterArtisans
            | Self::GuildPathfindersUnion
            | Self::GuildWhiteRose
            | Self::GuildBoneCollectors
            | Self::GuildBeastSlayers
            | Self::GuildPhilosophersStone
            | Self::GuildCartographersSociety => FactionCategory::Guild,
            Self::HumanAlliance
            | Self::ElvenCourts
            | Self::DwarvenClans
            | Self::OrcishHorde
            | Self::UndeadCollective
            | Self::DemonicLegion
            | Self::BeastkinTribes
            | Self::DragonbornCouncil
            | Self::FaeCourts
            | Self::ConstructAssembly
            | Self::ElementalConclave
            | Self::CelestialHost => FactionCategory::Race,
            Self::TheIlluminated
            | Self::OrderOfTheEternalEye
            | Self::BrotherhoodOfShadows
            | Self::TheVeiledHand
            | Self::CircleOfWhispers
            | Self::KeepersOfTheVoid => FactionCategory::SecretSociety,
            Self::PrimalBeastKingdom
            | Self::SerpentEmpire
            | Self::NecropolisDominion
            | Self::VampireCourts
            | Self::InfernalLegions
            | Self::AbyssalHorde => FactionCategory::MonsterFaction,
        }
    }

    /// Returns the faction name
    pub fn name(&self) -> &'static str {
        match self {
            // Kingdoms
            Self::KingdomOfLight => "Kingdom of Light",
            Self::DarkEmpire => "Dark Empire",
            Self::NeutralTerritories => "Neutral Territories",
            // Cultivation Sects - Orthodox
            Self::HeavenlySwordSect => "Heavenly Sword Sect",
            Self::JadePhoenixPavilion => "Jade Phoenix Pavilion",
            Self::AzureCloudMonastery => "Azure Cloud Monastery",
            // Cultivation Sects - Demonic
            Self::BloodMoonCult => "Blood Moon Cult",
            Self::ShadowViperSect => "Shadow Viper Sect",
            Self::SoulDevouringPalace => "Soul Devouring Palace",
            // Cultivation Sects - Buddhist
            Self::TempleOfEnlightenment => "Temple of Enlightenment",
            Self::ThousandBuddhaMonastery => "Thousand Buddha Monastery",
            // Guilds
            Self::GuildBladesBrotherhood => "The Blades Brotherhood",
            Self::GuildArcanumCircle => "Arcanum Circle",
            Self::GuildShadowSyndicate => "Shadow Syndicate",
            Self::GuildSilentBlade => "The Silent Blade",
            Self::GuildGoldenCoin => "Golden Coin Trading Co.",
            Self::GuildMasterArtisans => "Master Artisans Guild",
            Self::GuildPathfindersUnion => "Pathfinders Union",
            Self::GuildWhiteRose => "Order of the White Rose",
            Self::GuildBoneCollectors => "The Bone Collectors",
            Self::GuildBeastSlayers => "Beast Slayers League",
            Self::GuildPhilosophersStone => "Philosopher's Stone Society",
            Self::GuildCartographersSociety => "Cartographers Society",
            // Races
            Self::HumanAlliance => "Human Alliance",
            Self::ElvenCourts => "Elven Courts",
            Self::DwarvenClans => "Dwarven Clans",
            Self::OrcishHorde => "Orcish Horde",
            Self::UndeadCollective => "Undead Collective",
            Self::DemonicLegion => "Demonic Legion",
            Self::BeastkinTribes => "Beastkin Tribes",
            Self::DragonbornCouncil => "Dragonborn Council",
            Self::FaeCourts => "Fae Courts",
            Self::ConstructAssembly => "Construct Assembly",
            Self::ElementalConclave => "Elemental Conclave",
            Self::CelestialHost => "Celestial Host",
            // Secret Societies
            Self::TheIlluminated => "The Illuminated",
            Self::OrderOfTheEternalEye => "Order of the Eternal Eye",
            Self::BrotherhoodOfShadows => "Brotherhood of Shadows",
            Self::TheVeiledHand => "The Veiled Hand",
            Self::CircleOfWhispers => "Circle of Whispers",
            Self::KeepersOfTheVoid => "Keepers of the Void",
            // Monster Factions
            Self::PrimalBeastKingdom => "Primal Beast Kingdom",
            Self::SerpentEmpire => "Serpent Empire",
            Self::NecropolisDominion => "Necropolis Dominion",
            Self::VampireCourts => "Vampire Courts",
            Self::InfernalLegions => "Infernal Legions",
            Self::AbyssalHorde => "Abyssal Horde",
        }
    }

    /// Returns a description of the faction
    pub fn description(&self) -> &'static str {
        match self {
            // Kingdoms
            Self::KingdomOfLight => "A bastion of righteousness and holy power, the Kingdom of Light stands against darkness in all its forms. Its paladins and priests are renowned throughout the land.",
            Self::DarkEmpire => "A malevolent empire ruled by shadow lords and dark sorcerers. The Dark Empire seeks to shroud the world in eternal night.",
            Self::NeutralTerritories => "A confederation of free cities, independent states, and neutral zones. They maintain balance between light and darkness through diplomacy and trade.",
            // Cultivation Sects - Orthodox
            Self::HeavenlySwordSect => "Righteous sword cultivators who follow the path of justice. Their disciples are known for their unwavering moral code and devastating sword techniques.",
            Self::JadePhoenixPavilion => "A sect focused on healing arts and support cultivation. Their disciples are welcomed everywhere for their ability to cure the incurable.",
            Self::AzureCloudMonastery => "Scholarly cultivators who seek enlightenment through knowledge. Their vast libraries contain secrets from ancient times.",
            // Cultivation Sects - Demonic
            Self::BloodMoonCult => "Demonic cultivators who advance through blood sacrifice and dark rituals. Their power is great but their path leads to corruption.",
            Self::ShadowViperSect => "Masters of assassination and poison cultivation. They are feared throughout the cultivation world for their deadly efficiency.",
            Self::SoulDevouringPalace => "Practitioners of forbidden soul cultivation arts. They grow stronger by consuming the souls of their victims.",
            // Cultivation Sects - Buddhist
            Self::TempleOfEnlightenment => "Buddhist cultivators who seek to transcend worldly desires. Their monks are known for their inner peace and powerful defensive techniques.",
            Self::ThousandBuddhaMonastery => "Martial Buddhist monks who combine combat prowess with spiritual cultivation. Their monastery houses countless Buddha statues, each containing a fragment of enlightenment.",
            // Guilds
            Self::GuildBladesBrotherhood => "A brotherhood of warriors dedicated to martial excellence and honorable combat.",
            Self::GuildArcanumCircle => "Mages studying the arcane arts and uncovering magical secrets of the universe.",
            Self::GuildShadowSyndicate => "Thieves and rogues who operate in the shadows, taking what others cannot protect.",
            Self::GuildSilentBlade => "Professional assassins who eliminate targets with deadly precision and absolute discretion.",
            Self::GuildGoldenCoin => "Merchants and traders who believe gold is the true power in any realm.",
            Self::GuildMasterArtisans => "Master crafters creating the finest weapons, armor, and magical items.",
            Self::GuildPathfindersUnion => "Adventurers who explore dungeons, ruins, and unknown territories for glory and treasure.",
            Self::GuildWhiteRose => "Healers and priests dedicated to saving lives and combating disease.",
            Self::GuildBoneCollectors => "Necromancers studying the secrets of death and the power of undeath.",
            Self::GuildBeastSlayers => "Elite monster hunters specializing in tracking and slaying dangerous creatures.",
            Self::GuildPhilosophersStone => "Alchemists seeking the secrets of transmutation and the legendary Philosopher's Stone.",
            Self::GuildCartographersSociety => "Explorers and mapmakers documenting the unknown reaches of the world.",
            // Races
            Self::HumanAlliance => "The united human kingdoms, versatile and adaptable, striving to carve their place in a world of magic and monsters.",
            Self::ElvenCourts => "Ancient elven nations unified under the high courts, masters of magic and keepers of millennia of knowledge.",
            Self::DwarvenClans => "Mountain dwarf confederation known for their craftsmanship, resilience, and deep connection to stone and metal.",
            Self::OrcishHorde => "United orc tribes that value strength, honor in battle, and the glory of conquest.",
            Self::UndeadCollective => "Organized undead society led by intelligent undead who seek to expand their eternal domain.",
            Self::DemonicLegion => "The organized hierarchy of demons, from imps to demon lords, united in their desire to conquer the mortal realm.",
            Self::BeastkinTribes => "United beastfolk tribes representing wolves, cats, bears, and other beast-human hybrids.",
            Self::DragonbornCouncil => "Dragon-descended peoples governed by an ancient council that preserves draconic traditions.",
            Self::FaeCourts => "The combined seelie and unseelie courts of the fae realm, mysterious and capricious.",
            Self::ConstructAssembly => "A collective of sentient constructs - golems, automatons, and warforged - seeking recognition as living beings.",
            Self::ElementalConclave => "A council of elemental beings representing fire, water, earth, air, and void.",
            Self::CelestialHost => "Divine beings and their mortal servants who work to maintain cosmic order.",
            // Secret Societies
            Self::TheIlluminated => "A shadow council of the world's most powerful individuals who secretly control major events across all nations.",
            Self::OrderOfTheEternalEye => "An ancient order of seers and prophets who have watched over the world since the dawn of civilization.",
            Self::BrotherhoodOfShadows => "A legendary assassin brotherhood whose members are bound by blood oaths and ancient rituals.",
            Self::TheVeiledHand => "Puppet masters who manipulate politics, trade, and war from behind the scenes.",
            Self::CircleOfWhispers => "An information broker network that knows every secret worth knowing.",
            Self::KeepersOfTheVoid => "Guardians of forbidden knowledge and artifacts too dangerous for the world to know.",
            // Monster Factions
            Self::PrimalBeastKingdom => "An intelligent beast civilization ruled by awakened animals who have gained sentience and power.",
            Self::SerpentEmpire => "An empire of serpentine creatures - nagas, lamias, and serpent-folk - ruled by ancient snake gods.",
            Self::NecropolisDominion => "A vast undead empire ruled by lich kings and death knights from their city of the dead.",
            Self::VampireCourts => "Noble vampire houses engaged in eternal political intrigue while feeding on the living.",
            Self::InfernalLegions => "Organized demon armies under the command of archdevils, seeking to conquer through military might.",
            Self::AbyssalHorde => "Chaotic demon swarms that spread destruction and corruption wherever they go.",
        }
    }

    /// Returns linked Guild if this faction represents a guild
    pub fn linked_guild(&self) -> Option<Guild> {
        match self {
            Self::GuildBladesBrotherhood => Some(Guild::BladesBrotherhood),
            Self::GuildArcanumCircle => Some(Guild::ArcanumCircle),
            Self::GuildShadowSyndicate => Some(Guild::ShadowSyndicate),
            Self::GuildSilentBlade => Some(Guild::SilentBlade),
            Self::GuildGoldenCoin => Some(Guild::GoldenCoin),
            Self::GuildMasterArtisans => Some(Guild::MasterArtisans),
            Self::GuildPathfindersUnion => Some(Guild::PathfindersUnion),
            Self::GuildWhiteRose => Some(Guild::WhiteRose),
            Self::GuildBoneCollectors => Some(Guild::BoneCollectors),
            Self::GuildBeastSlayers => Some(Guild::BeastSlayers),
            Self::GuildPhilosophersStone => Some(Guild::PhilosophersStone),
            Self::GuildCartographersSociety => Some(Guild::CartographersSociety),
            _ => None,
        }
    }

    /// Returns linked Species if this faction represents a racial faction
    pub fn linked_species(&self) -> Option<Species> {
        match self {
            Self::HumanAlliance => Some(Species::Human),
            Self::ElvenCourts => Some(Species::Elf),
            Self::DwarvenClans => Some(Species::Dwarf),
            Self::OrcishHorde => Some(Species::Orc),
            Self::UndeadCollective => Some(Species::Undead),
            Self::DemonicLegion => Some(Species::Demon),
            Self::BeastkinTribes => Some(Species::Beastkin),
            Self::DragonbornCouncil => Some(Species::Dragonborn),
            Self::FaeCourts => Some(Species::Fae),
            Self::ConstructAssembly => Some(Species::Construct),
            Self::ElementalConclave => Some(Species::Elemental),
            Self::CelestialHost => Some(Species::Celestial),
            _ => None,
        }
    }

    /// Returns linked Kingdom if applicable
    pub fn linked_kingdom(&self) -> Option<Kingdom> {
        match self {
            Self::KingdomOfLight => Some(Kingdom::Valdoria),
            Self::DarkEmpire => Some(Kingdom::Necropolis),
            Self::HumanAlliance => Some(Kingdom::Valdoria),
            Self::ElvenCourts => Some(Kingdom::Sylvaneth),
            Self::DwarvenClans => Some(Kingdom::Ironhold),
            Self::OrcishHorde => Some(Kingdom::Grommash),
            Self::UndeadCollective => Some(Kingdom::Necropolis),
            Self::DemonicLegion => Some(Kingdom::Infernium),
            Self::BeastkinTribes => Some(Kingdom::Wildlands),
            Self::CelestialHost => Some(Kingdom::Celestia),
            _ => None,
        }
    }

    /// Returns the minimum cultivation realm required to join (for cultivation sects)
    pub fn required_cultivation_realm(&self) -> Option<CultivationRealm> {
        match self {
            Self::HeavenlySwordSect
            | Self::JadePhoenixPavilion
            | Self::AzureCloudMonastery
            | Self::TempleOfEnlightenment
            | Self::ThousandBuddhaMonastery => Some(CultivationRealm::QiCondensation),
            Self::BloodMoonCult | Self::ShadowViperSect => Some(CultivationRealm::FoundationEstablishment),
            Self::SoulDevouringPalace => Some(CultivationRealm::CoreFormation),
            _ => None,
        }
    }

    /// Returns whether this faction is considered "evil" or villainous
    pub fn is_evil(&self) -> bool {
        matches!(
            self,
            Self::DarkEmpire
                | Self::BloodMoonCult
                | Self::ShadowViperSect
                | Self::SoulDevouringPalace
                | Self::NecropolisDominion
                | Self::VampireCourts
                | Self::InfernalLegions
                | Self::AbyssalHorde
                | Self::BrotherhoodOfShadows
        )
    }

    /// Returns whether this faction is hidden/secret
    pub fn is_secret(&self) -> bool {
        matches!(
            self,
            Self::TheIlluminated
                | Self::OrderOfTheEternalEye
                | Self::BrotherhoodOfShadows
                | Self::TheVeiledHand
                | Self::CircleOfWhispers
                | Self::KeepersOfTheVoid
        )
    }

    /// Returns the difficulty to join this faction (0-100)
    pub fn join_difficulty(&self) -> u32 {
        match self {
            // Easy to join
            Self::NeutralTerritories
            | Self::GuildPathfindersUnion
            | Self::GuildGoldenCoin => 10,
            // Moderate difficulty
            Self::KingdomOfLight
            | Self::HumanAlliance
            | Self::ElvenCourts
            | Self::DwarvenClans
            | Self::GuildBladesBrotherhood
            | Self::GuildArcanumCircle
            | Self::GuildWhiteRose
            | Self::GuildMasterArtisans
            | Self::GuildPhilosophersStone
            | Self::GuildCartographersSociety
            | Self::GuildBeastSlayers
            | Self::TempleOfEnlightenment
            | Self::ThousandBuddhaMonastery => 30,
            // Hard to join
            Self::HeavenlySwordSect
            | Self::JadePhoenixPavilion
            | Self::AzureCloudMonastery
            | Self::OrcishHorde
            | Self::BeastkinTribes
            | Self::DragonbornCouncil
            | Self::FaeCourts
            | Self::ElementalConclave
            | Self::CelestialHost
            | Self::PrimalBeastKingdom => 50,
            // Very hard to join
            Self::DarkEmpire
            | Self::BloodMoonCult
            | Self::ShadowViperSect
            | Self::GuildShadowSyndicate
            | Self::GuildSilentBlade
            | Self::GuildBoneCollectors
            | Self::UndeadCollective
            | Self::DemonicLegion
            | Self::ConstructAssembly
            | Self::SerpentEmpire => 70,
            // Nearly impossible to join
            Self::SoulDevouringPalace
            | Self::TheIlluminated
            | Self::OrderOfTheEternalEye
            | Self::BrotherhoodOfShadows
            | Self::TheVeiledHand
            | Self::CircleOfWhispers
            | Self::KeepersOfTheVoid
            | Self::NecropolisDominion
            | Self::VampireCourts
            | Self::InfernalLegions
            | Self::AbyssalHorde => 90,
        }
    }
}

// =============================================================================
// REPUTATION LEVELS
// =============================================================================

/// Reputation level with a faction
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ReputationLevel {
    /// -1000 to -750: Kill on sight
    Hated = 0,
    /// -749 to -500: Attack on sight
    Hostile = 1,
    /// -499 to -250: No services, higher prices
    Unfriendly = 2,
    /// -249 to 249: Standard treatment
    Neutral = 3,
    /// 250 to 499: Minor discounts, some quests
    Friendly = 4,
    /// 500 to 749: Good discounts, special quests
    Honored = 5,
    /// 750 to 999: Great discounts, exclusive content
    Revered = 6,
    /// 1000+: Maximum benefits, legendary status
    Exalted = 7,
}

impl ReputationLevel {
    /// Get reputation level from raw reputation value
    pub fn from_reputation(rep: i32) -> Self {
        match rep {
            r if r <= REPUTATION_HATED_MAX => Self::Hated,
            r if r <= REPUTATION_HOSTILE_MAX => Self::Hostile,
            r if r <= REPUTATION_UNFRIENDLY_MAX => Self::Unfriendly,
            r if r <= REPUTATION_NEUTRAL_MAX => Self::Neutral,
            r if r <= REPUTATION_FRIENDLY_MAX => Self::Friendly,
            r if r <= REPUTATION_HONORED_MAX => Self::Honored,
            r if r <= REPUTATION_REVERED_MAX => Self::Revered,
            _ => Self::Exalted,
        }
    }

    /// Get the minimum reputation value for this level
    pub fn min_reputation(&self) -> i32 {
        match self {
            Self::Hated => REPUTATION_HATED_MIN,
            Self::Hostile => REPUTATION_HOSTILE_MIN,
            Self::Unfriendly => REPUTATION_UNFRIENDLY_MIN,
            Self::Neutral => REPUTATION_NEUTRAL_MIN,
            Self::Friendly => REPUTATION_FRIENDLY_MIN,
            Self::Honored => REPUTATION_HONORED_MIN,
            Self::Revered => REPUTATION_REVERED_MIN,
            Self::Exalted => REPUTATION_EXALTED_MIN,
        }
    }

    /// Get the maximum reputation value for this level
    pub fn max_reputation(&self) -> i32 {
        match self {
            Self::Hated => REPUTATION_HATED_MAX,
            Self::Hostile => REPUTATION_HOSTILE_MAX,
            Self::Unfriendly => REPUTATION_UNFRIENDLY_MAX,
            Self::Neutral => REPUTATION_NEUTRAL_MAX,
            Self::Friendly => REPUTATION_FRIENDLY_MAX,
            Self::Honored => REPUTATION_HONORED_MAX,
            Self::Revered => REPUTATION_REVERED_MAX,
            Self::Exalted => MAX_REPUTATION,
        }
    }

    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Hated => "Hated",
            Self::Hostile => "Hostile",
            Self::Unfriendly => "Unfriendly",
            Self::Neutral => "Neutral",
            Self::Friendly => "Friendly",
            Self::Honored => "Honored",
            Self::Revered => "Revered",
            Self::Exalted => "Exalted",
        }
    }

    /// Get color code for UI display
    pub fn color(&self) -> &'static str {
        match self {
            Self::Hated => "#8B0000",      // Dark Red
            Self::Hostile => "#FF0000",    // Red
            Self::Unfriendly => "#FF6600", // Orange
            Self::Neutral => "#FFFF00",    // Yellow
            Self::Friendly => "#00FF00",   // Green
            Self::Honored => "#00BFFF",    // Deep Sky Blue
            Self::Revered => "#9400D3",    // Dark Violet
            Self::Exalted => "#FFD700",    // Gold
        }
    }

    /// Get all reputation levels
    pub fn all() -> &'static [ReputationLevel] {
        &[
            Self::Hated,
            Self::Hostile,
            Self::Unfriendly,
            Self::Neutral,
            Self::Friendly,
            Self::Honored,
            Self::Revered,
            Self::Exalted,
        ]
    }
}

impl Default for ReputationLevel {
    fn default() -> Self {
        Self::Neutral
    }
}

// =============================================================================
// REPUTATION EFFECTS
// =============================================================================

/// Effects granted based on reputation level
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReputationEffects {
    /// Shop price modifier (-0.5 = 50% off, 0.5 = 50% markup)
    pub shop_price_modifier: f32,
    /// Whether faction shops are accessible
    pub shop_access: bool,
    /// Whether faction quests are available
    pub quest_access: bool,
    /// Whether faction territories are accessible
    pub territory_access: bool,
    /// Whether NPCs will interact positively
    pub npc_friendly: bool,
    /// Whether NPCs will attack on sight
    pub npc_hostile: bool,
    /// Special abilities unlocked
    pub special_abilities: Vec<FactionAbility>,
    /// Stat bonuses
    pub stat_bonuses: FactionStatBonuses,
    /// Title granted
    pub title: Option<String>,
    /// Mount access
    pub mount_access: bool,
    /// Teleport access to faction locations
    pub teleport_access: bool,
    /// Can participate in faction wars
    pub war_participation: bool,
}

impl ReputationEffects {
    /// Calculate effects for a given reputation level and faction
    pub fn for_level(level: ReputationLevel, faction: FactionId) -> Self {
        let base_effects = match level {
            ReputationLevel::Hated => Self {
                shop_price_modifier: 0.0,
                shop_access: false,
                quest_access: false,
                territory_access: false,
                npc_friendly: false,
                npc_hostile: true,
                special_abilities: vec![],
                stat_bonuses: FactionStatBonuses::default(),
                title: Some("Enemy".to_string()),
                mount_access: false,
                teleport_access: false,
                war_participation: false,
            },
            ReputationLevel::Hostile => Self {
                shop_price_modifier: 0.0,
                shop_access: false,
                quest_access: false,
                territory_access: false,
                npc_friendly: false,
                npc_hostile: true,
                special_abilities: vec![],
                stat_bonuses: FactionStatBonuses::default(),
                title: Some("Outlaw".to_string()),
                mount_access: false,
                teleport_access: false,
                war_participation: false,
            },
            ReputationLevel::Unfriendly => Self {
                shop_price_modifier: 0.25,
                shop_access: true,
                quest_access: false,
                territory_access: true,
                npc_friendly: false,
                npc_hostile: false,
                special_abilities: vec![],
                stat_bonuses: FactionStatBonuses::default(),
                title: None,
                mount_access: false,
                teleport_access: false,
                war_participation: false,
            },
            ReputationLevel::Neutral => Self {
                shop_price_modifier: 0.0,
                shop_access: true,
                quest_access: true,
                territory_access: true,
                npc_friendly: true,
                npc_hostile: false,
                special_abilities: vec![],
                stat_bonuses: FactionStatBonuses::default(),
                title: None,
                mount_access: false,
                teleport_access: false,
                war_participation: false,
            },
            ReputationLevel::Friendly => Self {
                shop_price_modifier: -0.10,
                shop_access: true,
                quest_access: true,
                territory_access: true,
                npc_friendly: true,
                npc_hostile: false,
                special_abilities: vec![FactionAbility::BasicSupport],
                stat_bonuses: FactionStatBonuses {
                    hp_bonus: 5,
                    ..Default::default()
                },
                title: Some("Friend".to_string()),
                mount_access: false,
                teleport_access: false,
                war_participation: true,
            },
            ReputationLevel::Honored => Self {
                shop_price_modifier: -0.15,
                shop_access: true,
                quest_access: true,
                territory_access: true,
                npc_friendly: true,
                npc_hostile: false,
                special_abilities: vec![FactionAbility::BasicSupport, FactionAbility::FactionBuff],
                stat_bonuses: FactionStatBonuses {
                    hp_bonus: 10,
                    attack_bonus: 2,
                    defense_bonus: 2,
                    ..Default::default()
                },
                title: Some("Honored".to_string()),
                mount_access: true,
                teleport_access: false,
                war_participation: true,
            },
            ReputationLevel::Revered => Self {
                shop_price_modifier: -0.20,
                shop_access: true,
                quest_access: true,
                territory_access: true,
                npc_friendly: true,
                npc_hostile: false,
                special_abilities: vec![
                    FactionAbility::BasicSupport,
                    FactionAbility::FactionBuff,
                    FactionAbility::EmergencyAid,
                ],
                stat_bonuses: FactionStatBonuses {
                    hp_bonus: 20,
                    attack_bonus: 5,
                    defense_bonus: 5,
                    mana_bonus: 10,
                    ..Default::default()
                },
                title: Some("Revered".to_string()),
                mount_access: true,
                teleport_access: true,
                war_participation: true,
            },
            ReputationLevel::Exalted => Self {
                shop_price_modifier: -0.30,
                shop_access: true,
                quest_access: true,
                territory_access: true,
                npc_friendly: true,
                npc_hostile: false,
                special_abilities: vec![
                    FactionAbility::BasicSupport,
                    FactionAbility::FactionBuff,
                    FactionAbility::EmergencyAid,
                    FactionAbility::FactionSummon,
                    FactionAbility::LeaderAudience,
                ],
                stat_bonuses: FactionStatBonuses {
                    hp_bonus: 30,
                    attack_bonus: 10,
                    defense_bonus: 10,
                    mana_bonus: 20,
                    speed_bonus: 5,
                    xp_bonus_percent: 10,
                },
                title: Some("Exalted".to_string()),
                mount_access: true,
                teleport_access: true,
                war_participation: true,
            },
        };

        // Apply faction-specific modifications
        Self::apply_faction_bonuses(base_effects, faction, level)
    }

    /// Apply faction-specific bonuses to base effects
    fn apply_faction_bonuses(mut effects: Self, faction: FactionId, level: ReputationLevel) -> Self {
        if level < ReputationLevel::Friendly {
            return effects;
        }

        // Faction-specific bonuses at higher reputation levels
        match faction {
            FactionId::KingdomOfLight => {
                effects.special_abilities.push(FactionAbility::HolyBlessing);
                effects.stat_bonuses.light_damage_bonus = 10;
            }
            FactionId::DarkEmpire => {
                effects.special_abilities.push(FactionAbility::ShadowCloak);
                effects.stat_bonuses.dark_damage_bonus = 10;
            }
            FactionId::HeavenlySwordSect | FactionId::GuildBladesBrotherhood => {
                effects.stat_bonuses.attack_bonus += 5;
                effects.special_abilities.push(FactionAbility::SwordMastery);
            }
            FactionId::JadePhoenixPavilion | FactionId::GuildWhiteRose => {
                effects.stat_bonuses.hp_bonus += 10;
                effects.special_abilities.push(FactionAbility::HealingAura);
            }
            FactionId::GuildShadowSyndicate | FactionId::BrotherhoodOfShadows => {
                effects.special_abilities.push(FactionAbility::ShadowStep);
            }
            FactionId::GuildGoldenCoin => {
                effects.shop_price_modifier -= 0.10; // Extra discount
            }
            FactionId::CircleOfWhispers => {
                effects.special_abilities.push(FactionAbility::InformationNetwork);
            }
            FactionId::VampireCourts => {
                effects.special_abilities.push(FactionAbility::BloodDrain);
            }
            _ => {}
        }

        effects
    }
}

/// Stat bonuses from faction reputation
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FactionStatBonuses {
    pub hp_bonus: i32,
    pub attack_bonus: i32,
    pub defense_bonus: i32,
    pub mana_bonus: i32,
    pub speed_bonus: i32,
    pub xp_bonus_percent: i32,
    pub light_damage_bonus: i32,
    pub dark_damage_bonus: i32,
    pub fire_damage_bonus: i32,
    pub ice_damage_bonus: i32,
}

impl FactionStatBonuses {
    /// Combine multiple stat bonuses
    pub fn combine(&self, other: &FactionStatBonuses) -> FactionStatBonuses {
        FactionStatBonuses {
            hp_bonus: self.hp_bonus + other.hp_bonus,
            attack_bonus: self.attack_bonus + other.attack_bonus,
            defense_bonus: self.defense_bonus + other.defense_bonus,
            mana_bonus: self.mana_bonus + other.mana_bonus,
            speed_bonus: self.speed_bonus + other.speed_bonus,
            xp_bonus_percent: self.xp_bonus_percent + other.xp_bonus_percent,
            light_damage_bonus: self.light_damage_bonus + other.light_damage_bonus,
            dark_damage_bonus: self.dark_damage_bonus + other.dark_damage_bonus,
            fire_damage_bonus: self.fire_damage_bonus + other.fire_damage_bonus,
            ice_damage_bonus: self.ice_damage_bonus + other.ice_damage_bonus,
        }
    }
}

/// Special abilities that can be unlocked through faction reputation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FactionAbility {
    /// Basic support from faction (information, shelter)
    BasicSupport,
    /// Receive faction buff when in territory
    FactionBuff,
    /// Emergency healing/rescue by faction
    EmergencyAid,
    /// Summon faction members to aid in combat
    FactionSummon,
    /// Audience with faction leader
    LeaderAudience,
    /// Holy blessing ability (Kingdom of Light)
    HolyBlessing,
    /// Shadow cloak invisibility (Dark Empire)
    ShadowCloak,
    /// Enhanced sword techniques
    SwordMastery,
    /// Passive healing aura
    HealingAura,
    /// Teleport short distances
    ShadowStep,
    /// Access to information network
    InformationNetwork,
    /// Drain life from enemies
    BloodDrain,
    /// Summon undead minions
    RaiseUndead,
    /// Elemental mastery
    ElementalControl,
    /// Beast companion bonding
    BeastBond,
    /// Divine protection
    DivineShield,
    /// Demonic transformation
    DemonicForm,
}

impl FactionAbility {
    pub fn name(&self) -> &'static str {
        match self {
            Self::BasicSupport => "Basic Support",
            Self::FactionBuff => "Faction Buff",
            Self::EmergencyAid => "Emergency Aid",
            Self::FactionSummon => "Faction Summon",
            Self::LeaderAudience => "Leader Audience",
            Self::HolyBlessing => "Holy Blessing",
            Self::ShadowCloak => "Shadow Cloak",
            Self::SwordMastery => "Sword Mastery",
            Self::HealingAura => "Healing Aura",
            Self::ShadowStep => "Shadow Step",
            Self::InformationNetwork => "Information Network",
            Self::BloodDrain => "Blood Drain",
            Self::RaiseUndead => "Raise Undead",
            Self::ElementalControl => "Elemental Control",
            Self::BeastBond => "Beast Bond",
            Self::DivineShield => "Divine Shield",
            Self::DemonicForm => "Demonic Form",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::BasicSupport => "Receive basic information, shelter, and minor assistance from faction members.",
            Self::FactionBuff => "Gain a stat buff while in faction-controlled territory.",
            Self::EmergencyAid => "Once per day, call for emergency healing or rescue.",
            Self::FactionSummon => "Summon faction members to aid you in combat.",
            Self::LeaderAudience => "Request a personal audience with faction leadership.",
            Self::HolyBlessing => "Receive a holy blessing that increases light damage and heals over time.",
            Self::ShadowCloak => "Become invisible in shadows for a short duration.",
            Self::SwordMastery => "Enhanced sword techniques grant bonus damage and critical chance.",
            Self::HealingAura => "Passively heal yourself and nearby allies over time.",
            Self::ShadowStep => "Teleport a short distance through shadows.",
            Self::InformationNetwork => "Access to secret information about quests, treasures, and enemies.",
            Self::BloodDrain => "Drain life from enemies to heal yourself.",
            Self::RaiseUndead => "Summon undead minions from fallen enemies.",
            Self::ElementalControl => "Enhanced control over elemental magic.",
            Self::BeastBond => "Form a stronger bond with beast companions.",
            Self::DivineShield => "Create a protective barrier that blocks damage.",
            Self::DemonicForm => "Transform into a powerful demonic form temporarily.",
        }
    }

    pub fn cooldown_turns(&self) -> u32 {
        match self {
            Self::BasicSupport => 0,
            Self::FactionBuff => 0,
            Self::EmergencyAid => 100,
            Self::FactionSummon => 50,
            Self::LeaderAudience => 200,
            Self::HolyBlessing => 30,
            Self::ShadowCloak => 20,
            Self::SwordMastery => 0,
            Self::HealingAura => 0,
            Self::ShadowStep => 10,
            Self::InformationNetwork => 0,
            Self::BloodDrain => 15,
            Self::RaiseUndead => 25,
            Self::ElementalControl => 0,
            Self::BeastBond => 0,
            Self::DivineShield => 20,
            Self::DemonicForm => 100,
        }
    }
}

// =============================================================================
// FACTION RELATIONSHIPS
// =============================================================================

/// Relationship between two factions
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FactionRelationship {
    /// Factions are formal allies
    Allied,
    /// Factions are friendly but not formally allied
    Friendly,
    /// Factions have no strong feelings either way
    Neutral,
    /// Factions dislike each other
    Unfriendly,
    /// Factions are hostile but not at war
    Hostile,
    /// Factions are actively at war
    AtWar,
}

impl FactionRelationship {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Allied => "Allied",
            Self::Friendly => "Friendly",
            Self::Neutral => "Neutral",
            Self::Unfriendly => "Unfriendly",
            Self::Hostile => "Hostile",
            Self::AtWar => "At War",
        }
    }

    /// Returns how actions with one faction affect reputation with related faction
    pub fn reputation_spillover(&self) -> f32 {
        match self {
            Self::Allied => 0.5,      // 50% positive spillover
            Self::Friendly => 0.25,   // 25% positive spillover
            Self::Neutral => 0.0,     // No spillover
            Self::Unfriendly => -0.1, // 10% negative spillover
            Self::Hostile => -0.25,   // 25% negative spillover
            Self::AtWar => -0.5,      // 50% negative spillover (gain with enemy = lose with ally)
        }
    }
}

impl Default for FactionRelationship {
    fn default() -> Self {
        Self::Neutral
    }
}

/// A dynamic alliance or conflict between factions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactionPact {
    pub id: u32,
    pub pact_type: PactType,
    pub factions: Vec<FactionId>,
    pub formed_turn: u32,
    pub duration: Option<u32>, // None = permanent until broken
    pub terms: Vec<PactTerm>,
    pub active: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PactType {
    Alliance,
    MutualDefense,
    NonAggression,
    TradeAgreement,
    WarDeclaration,
    Ceasefire,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PactTerm {
    MutualAid,
    SharedIntelligence,
    TerritoryAccess,
    TradeRights,
    WarSupport,
    Tribute { amount: u32, per_turns: u32 },
}

// =============================================================================
// FACTION STATE
// =============================================================================

/// Complete state of a faction
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Faction {
    pub id: FactionId,
    pub name: String,
    pub description: String,
    pub category: FactionCategory,
    /// Leader name
    pub leader: String,
    /// Faction headquarters location
    pub headquarters: String,
    /// Current power level (0-100)
    pub power_level: u32,
    /// Controlled territories
    pub territories: Vec<TerritoryId>,
    /// Total member count
    pub member_count: u64,
    /// Treasury/resources
    pub treasury: u64,
    /// Military strength
    pub military_strength: u32,
    /// Relationships with other factions
    pub relationships: HashMap<FactionId, FactionRelationship>,
    /// Active wars
    pub active_wars: Vec<FactionId>,
    /// Active alliances
    pub active_alliances: Vec<FactionId>,
    /// Available quests for members
    pub available_quests: Vec<FactionQuestId>,
    /// Ranks within the faction
    pub ranks: Vec<FactionRank>,
    /// Special items available for purchase
    pub shop_items: Vec<FactionShopItem>,
}

impl Faction {
    /// Create a new faction with default state
    pub fn new(id: FactionId) -> Self {
        Self {
            id,
            name: id.name().to_string(),
            description: id.description().to_string(),
            category: id.category(),
            leader: Self::generate_leader_name(id),
            headquarters: Self::generate_headquarters(id),
            power_level: 50,
            territories: vec![],
            member_count: Self::initial_member_count(id),
            treasury: Self::initial_treasury(id),
            military_strength: Self::initial_military(id),
            relationships: Self::default_relationships(id),
            active_wars: Self::initial_wars(id),
            active_alliances: Self::initial_alliances(id),
            available_quests: vec![],
            ranks: Self::generate_ranks(id),
            shop_items: vec![],
        }
    }

    fn generate_leader_name(id: FactionId) -> String {
        match id {
            FactionId::KingdomOfLight => "High King Aurelius the Radiant".to_string(),
            FactionId::DarkEmpire => "Shadow Emperor Malachar".to_string(),
            FactionId::NeutralTerritories => "Council of Five".to_string(),
            FactionId::HeavenlySwordSect => "Sect Master Jian Tianming".to_string(),
            FactionId::JadePhoenixPavilion => "Pavilion Mistress Feng Luan".to_string(),
            FactionId::AzureCloudMonastery => "Grand Elder Yun Qing".to_string(),
            FactionId::BloodMoonCult => "Blood Patriarch Xue Yue".to_string(),
            FactionId::ShadowViperSect => "Viper Queen Du Ying".to_string(),
            FactionId::SoulDevouringPalace => "Palace Lord Hun Shi".to_string(),
            FactionId::TempleOfEnlightenment => "Abbot Kongming".to_string(),
            FactionId::ThousandBuddhaMonastery => "Grand Master Wukong".to_string(),
            FactionId::TheIlluminated => "The Unseen One".to_string(),
            FactionId::OrderOfTheEternalEye => "Oracle Prime".to_string(),
            FactionId::BrotherhoodOfShadows => "The Faceless".to_string(),
            FactionId::TheVeiledHand => "The Puppeteer".to_string(),
            FactionId::CircleOfWhispers => "The Listener".to_string(),
            FactionId::KeepersOfTheVoid => "Void Walker".to_string(),
            FactionId::PrimalBeastKingdom => "Beast King Fenrir".to_string(),
            FactionId::SerpentEmpire => "Serpent Empress Nagini".to_string(),
            FactionId::NecropolisDominion => "Lich King Mortis".to_string(),
            FactionId::VampireCourts => "Blood Queen Carmilla".to_string(),
            FactionId::InfernalLegions => "Arch-Devil Belial".to_string(),
            FactionId::AbyssalHorde => "Chaos Lord Azathoth".to_string(),
            FactionId::HumanAlliance => "High Chancellor Marcus".to_string(),
            FactionId::ElvenCourts => "High Queen Aelindra".to_string(),
            FactionId::DwarvenClans => "High Thane Borin Steelbeard".to_string(),
            FactionId::OrcishHorde => "Warchief Grom'thar".to_string(),
            FactionId::DragonbornCouncil => "Elder Dragon Tiamat".to_string(),
            FactionId::CelestialHost => "Archon Seraphiel".to_string(),
            _ => "Unknown Leader".to_string(),
        }
    }

    fn generate_headquarters(id: FactionId) -> String {
        match id {
            FactionId::KingdomOfLight => "Radiant Citadel".to_string(),
            FactionId::DarkEmpire => "Obsidian Fortress".to_string(),
            FactionId::NeutralTerritories => "Free City of Crossroads".to_string(),
            FactionId::HeavenlySwordSect => "Sword Peak Mountain".to_string(),
            FactionId::JadePhoenixPavilion => "Phoenix Valley".to_string(),
            FactionId::AzureCloudMonastery => "Cloud Peak".to_string(),
            FactionId::BloodMoonCult => "Crimson Temple".to_string(),
            FactionId::ShadowViperSect => "Viper's Den".to_string(),
            FactionId::SoulDevouringPalace => "Soul Abyss".to_string(),
            FactionId::TempleOfEnlightenment => "Enlightenment Peak".to_string(),
            FactionId::ThousandBuddhaMonastery => "Buddha Mountain".to_string(),
            FactionId::TheIlluminated => "Unknown".to_string(),
            FactionId::OrderOfTheEternalEye => "The All-Seeing Tower".to_string(),
            FactionId::BrotherhoodOfShadows => "The Black Lodge".to_string(),
            FactionId::NecropolisDominion => "City of the Dead".to_string(),
            FactionId::VampireCourts => "Crimson Castle".to_string(),
            FactionId::InfernalLegions => "The Nine Hells".to_string(),
            FactionId::AbyssalHorde => "The Abyss".to_string(),
            _ => "Main Headquarters".to_string(),
        }
    }

    fn initial_member_count(id: FactionId) -> u64 {
        match id.category() {
            FactionCategory::Kingdom => 1_000_000,
            FactionCategory::Race => 500_000,
            FactionCategory::CultivationSect => 10_000,
            FactionCategory::Guild => 5_000,
            FactionCategory::SecretSociety => 500,
            FactionCategory::MonsterFaction => 100_000,
            _ => 10_000,
        }
    }

    fn initial_treasury(id: FactionId) -> u64 {
        match id.category() {
            FactionCategory::Kingdom => 10_000_000,
            FactionCategory::Race => 5_000_000,
            FactionCategory::Guild => 1_000_000,
            FactionCategory::SecretSociety => 5_000_000, // Secret societies are wealthy
            FactionCategory::MonsterFaction => 500_000,
            _ => 100_000,
        }
    }

    fn initial_military(id: FactionId) -> u32 {
        match id.category() {
            FactionCategory::Kingdom => 100_000,
            FactionCategory::Race => 50_000,
            FactionCategory::MonsterFaction => 75_000,
            FactionCategory::CultivationSect => 5_000,
            FactionCategory::Guild => 1_000,
            FactionCategory::SecretSociety => 500,
            _ => 1_000,
        }
    }

    fn default_relationships(id: FactionId) -> HashMap<FactionId, FactionRelationship> {
        let mut relationships = HashMap::new();

        for other in FactionId::all() {
            if *other == id {
                continue;
            }

            let relationship = Self::calculate_default_relationship(id, *other);
            relationships.insert(*other, relationship);
        }

        relationships
    }

    fn calculate_default_relationship(a: FactionId, b: FactionId) -> FactionRelationship {
        // Kingdom of Light vs Dark Empire - At War
        if (a == FactionId::KingdomOfLight && b == FactionId::DarkEmpire)
            || (a == FactionId::DarkEmpire && b == FactionId::KingdomOfLight)
        {
            return FactionRelationship::AtWar;
        }

        // Light faction relationships
        if a == FactionId::KingdomOfLight || b == FactionId::KingdomOfLight {
            let other = if a == FactionId::KingdomOfLight { b } else { a };
            if other.is_evil() {
                return FactionRelationship::Hostile;
            }
            if matches!(other, FactionId::CelestialHost | FactionId::GuildWhiteRose) {
                return FactionRelationship::Allied;
            }
        }

        // Dark faction relationships
        if a == FactionId::DarkEmpire || b == FactionId::DarkEmpire {
            let other = if a == FactionId::DarkEmpire { b } else { a };
            if !other.is_evil() && !other.is_secret() {
                return FactionRelationship::Unfriendly;
            }
            if matches!(
                other,
                FactionId::NecropolisDominion | FactionId::InfernalLegions
            ) {
                return FactionRelationship::Allied;
            }
        }

        // Orthodox vs Demonic cultivation sects
        let orthodox_sects = [
            FactionId::HeavenlySwordSect,
            FactionId::JadePhoenixPavilion,
            FactionId::AzureCloudMonastery,
            FactionId::TempleOfEnlightenment,
            FactionId::ThousandBuddhaMonastery,
        ];
        let demonic_sects = [
            FactionId::BloodMoonCult,
            FactionId::ShadowViperSect,
            FactionId::SoulDevouringPalace,
        ];

        if orthodox_sects.contains(&a) && demonic_sects.contains(&b)
            || orthodox_sects.contains(&b) && demonic_sects.contains(&a)
        {
            return FactionRelationship::Hostile;
        }

        // Orthodox sects are friendly with each other
        if orthodox_sects.contains(&a) && orthodox_sects.contains(&b) {
            return FactionRelationship::Friendly;
        }

        // Demonic sects are unfriendly even with each other (competition)
        if demonic_sects.contains(&a) && demonic_sects.contains(&b) {
            return FactionRelationship::Unfriendly;
        }

        // Celestial vs Demonic/Undead
        if a == FactionId::CelestialHost || b == FactionId::CelestialHost {
            let other = if a == FactionId::CelestialHost { b } else { a };
            if matches!(
                other,
                FactionId::DemonicLegion
                    | FactionId::InfernalLegions
                    | FactionId::AbyssalHorde
                    | FactionId::NecropolisDominion
            ) {
                return FactionRelationship::AtWar;
            }
        }

        // Racial faction relationships based on species
        if let (Some(species_a), Some(species_b)) = (a.linked_species(), b.linked_species()) {
            // Traditional racial conflicts
            if (species_a == Species::Elf && species_b == Species::Orc)
                || (species_a == Species::Orc && species_b == Species::Elf)
            {
                return FactionRelationship::Hostile;
            }
            if (species_a == Species::Human && species_b == Species::Dwarf)
                || (species_a == Species::Dwarf && species_b == Species::Human)
            {
                return FactionRelationship::Friendly;
            }
            if (species_a == Species::Elf && species_b == Species::Human)
                || (species_a == Species::Human && species_b == Species::Elf)
            {
                return FactionRelationship::Friendly;
            }
        }

        // Secret societies are neutral with everyone (they work in the shadows)
        if a.is_secret() || b.is_secret() {
            return FactionRelationship::Neutral;
        }

        // Default neutral
        FactionRelationship::Neutral
    }

    fn initial_wars(id: FactionId) -> Vec<FactionId> {
        match id {
            FactionId::KingdomOfLight => vec![FactionId::DarkEmpire],
            FactionId::DarkEmpire => vec![FactionId::KingdomOfLight],
            FactionId::CelestialHost => vec![
                FactionId::InfernalLegions,
                FactionId::AbyssalHorde,
            ],
            FactionId::InfernalLegions => vec![FactionId::CelestialHost],
            FactionId::AbyssalHorde => vec![FactionId::CelestialHost],
            _ => vec![],
        }
    }

    fn initial_alliances(id: FactionId) -> Vec<FactionId> {
        match id {
            FactionId::KingdomOfLight => vec![FactionId::CelestialHost, FactionId::GuildWhiteRose],
            FactionId::DarkEmpire => vec![FactionId::NecropolisDominion],
            FactionId::CelestialHost => vec![FactionId::KingdomOfLight],
            FactionId::HeavenlySwordSect => vec![
                FactionId::JadePhoenixPavilion,
                FactionId::AzureCloudMonastery,
            ],
            FactionId::HumanAlliance => vec![FactionId::DwarvenClans, FactionId::ElvenCourts],
            FactionId::DwarvenClans => vec![FactionId::HumanAlliance],
            FactionId::ElvenCourts => vec![FactionId::HumanAlliance, FactionId::FaeCourts],
            _ => vec![],
        }
    }

    fn generate_ranks(id: FactionId) -> Vec<FactionRank> {
        match id.category() {
            FactionCategory::CultivationSect => vec![
                FactionRank::new(0, "Outer Disciple", ReputationLevel::Neutral),
                FactionRank::new(1, "Inner Disciple", ReputationLevel::Friendly),
                FactionRank::new(2, "Core Disciple", ReputationLevel::Honored),
                FactionRank::new(3, "Elder", ReputationLevel::Revered),
                FactionRank::new(4, "Grand Elder", ReputationLevel::Exalted),
            ],
            FactionCategory::Guild => vec![
                FactionRank::new(0, "Initiate", ReputationLevel::Neutral),
                FactionRank::new(1, "Apprentice", ReputationLevel::Friendly),
                FactionRank::new(2, "Journeyman", ReputationLevel::Friendly),
                FactionRank::new(3, "Adept", ReputationLevel::Honored),
                FactionRank::new(4, "Expert", ReputationLevel::Honored),
                FactionRank::new(5, "Master", ReputationLevel::Revered),
                FactionRank::new(6, "Grand Master", ReputationLevel::Exalted),
            ],
            FactionCategory::Kingdom => vec![
                FactionRank::new(0, "Commoner", ReputationLevel::Neutral),
                FactionRank::new(1, "Citizen", ReputationLevel::Friendly),
                FactionRank::new(2, "Knight", ReputationLevel::Honored),
                FactionRank::new(3, "Baron", ReputationLevel::Revered),
                FactionRank::new(4, "Count", ReputationLevel::Revered),
                FactionRank::new(5, "Duke", ReputationLevel::Exalted),
            ],
            FactionCategory::SecretSociety => vec![
                FactionRank::new(0, "Initiate", ReputationLevel::Neutral),
                FactionRank::new(1, "Acolyte", ReputationLevel::Friendly),
                FactionRank::new(2, "Adept", ReputationLevel::Honored),
                FactionRank::new(3, "Master", ReputationLevel::Revered),
                FactionRank::new(4, "Inner Circle", ReputationLevel::Exalted),
            ],
            FactionCategory::MonsterFaction => vec![
                FactionRank::new(0, "Minion", ReputationLevel::Neutral),
                FactionRank::new(1, "Soldier", ReputationLevel::Friendly),
                FactionRank::new(2, "Elite", ReputationLevel::Honored),
                FactionRank::new(3, "Champion", ReputationLevel::Revered),
                FactionRank::new(4, "Warlord", ReputationLevel::Exalted),
            ],
            _ => vec![
                FactionRank::new(0, "Member", ReputationLevel::Neutral),
                FactionRank::new(1, "Veteran", ReputationLevel::Friendly),
                FactionRank::new(2, "Elite", ReputationLevel::Honored),
                FactionRank::new(3, "Champion", ReputationLevel::Revered),
                FactionRank::new(4, "Legend", ReputationLevel::Exalted),
            ],
        }
    }

    /// Get relationship with another faction
    pub fn get_relationship(&self, other: FactionId) -> FactionRelationship {
        self.relationships
            .get(&other)
            .copied()
            .unwrap_or(FactionRelationship::Neutral)
    }

    /// Check if at war with another faction
    pub fn is_at_war_with(&self, other: FactionId) -> bool {
        self.active_wars.contains(&other)
    }

    /// Check if allied with another faction
    pub fn is_allied_with(&self, other: FactionId) -> bool {
        self.active_alliances.contains(&other)
    }
}

/// Rank within a faction
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactionRank {
    pub level: u32,
    pub name: String,
    pub required_reputation: ReputationLevel,
}

impl FactionRank {
    pub fn new(level: u32, name: &str, required_reputation: ReputationLevel) -> Self {
        Self {
            level,
            name: name.to_string(),
            required_reputation,
        }
    }
}

/// Item available in faction shop
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactionShopItem {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub cost: u32,
    pub required_reputation: ReputationLevel,
    pub item_type: FactionItemType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactionItemType {
    Weapon,
    Armor,
    Consumable,
    Recipe,
    Mount,
    Cosmetic,
    UniqueAbility,
}

/// Quest ID for faction quests
pub type FactionQuestId = u32;

/// Territory ID
pub type TerritoryId = u32;

// =============================================================================
// FACTION WARS
// =============================================================================

/// A war between factions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactionWar {
    pub id: u32,
    pub name: String,
    pub attackers: Vec<FactionId>,
    pub defenders: Vec<FactionId>,
    pub started_turn: u32,
    pub ended_turn: Option<u32>,
    pub status: WarStatus,
    pub contested_territories: Vec<TerritoryId>,
    pub attacker_score: u32,
    pub defender_score: u32,
    pub battles: Vec<WarBattle>,
    pub phase: WarPhase,
}

impl FactionWar {
    pub fn new(
        id: u32,
        name: &str,
        attackers: Vec<FactionId>,
        defenders: Vec<FactionId>,
        turn: u32,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            attackers,
            defenders,
            started_turn: turn,
            ended_turn: None,
            status: WarStatus::Active,
            contested_territories: vec![],
            attacker_score: 0,
            defender_score: 0,
            battles: vec![],
            phase: WarPhase::Mobilization,
        }
    }

    /// Check if a faction is an attacker
    pub fn is_attacker(&self, faction: FactionId) -> bool {
        self.attackers.contains(&faction)
    }

    /// Check if a faction is a defender
    pub fn is_defender(&self, faction: FactionId) -> bool {
        self.defenders.contains(&faction)
    }

    /// Check if a faction is involved in the war
    pub fn is_involved(&self, faction: FactionId) -> bool {
        self.is_attacker(faction) || self.is_defender(faction)
    }

    /// Get the winning side (if war is over)
    pub fn winner(&self) -> Option<WarSide> {
        if self.status != WarStatus::Ended {
            return None;
        }
        if self.attacker_score > self.defender_score {
            Some(WarSide::Attackers)
        } else if self.defender_score > self.attacker_score {
            Some(WarSide::Defenders)
        } else {
            None // Draw
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WarStatus {
    Preparing,
    Active,
    Ceasefire,
    Ended,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WarPhase {
    Mobilization,
    Skirmishes,
    OpenBattle,
    Siege,
    FinalAssault,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WarSide {
    Attackers,
    Defenders,
}

/// A battle within a war
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WarBattle {
    pub id: u32,
    pub name: String,
    pub turn: u32,
    pub location: TerritoryId,
    pub attacker_forces: u32,
    pub defender_forces: u32,
    pub attacker_casualties: u32,
    pub defender_casualties: u32,
    pub winner: Option<WarSide>,
    pub player_participated: bool,
    pub player_contribution: u32,
}

/// Player's war contribution tracking
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WarContribution {
    pub war_id: u32,
    pub faction_side: Option<WarSide>,
    pub total_contribution: u32,
    pub enemies_killed: u32,
    pub objectives_completed: u32,
    pub resources_donated: u32,
    pub battles_participated: u32,
    pub rank: ContributionRank,
}

impl WarContribution {
    pub fn new(war_id: u32) -> Self {
        Self {
            war_id,
            ..Default::default()
        }
    }

    /// Add contribution points
    pub fn add_contribution(&mut self, amount: u32) {
        self.total_contribution += amount;
        self.update_rank();
    }

    /// Update rank based on contribution
    fn update_rank(&mut self) {
        self.rank = ContributionRank::from_points(self.total_contribution);
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ContributionRank {
    #[default]
    None,
    Bronze,
    Silver,
    Gold,
    Platinum,
    Legendary,
}

impl ContributionRank {
    pub fn from_points(points: u32) -> Self {
        match points {
            0 => Self::None,
            p if p < WAR_CONTRIBUTION_BRONZE => Self::None,
            p if p < WAR_CONTRIBUTION_SILVER => Self::Bronze,
            p if p < WAR_CONTRIBUTION_GOLD => Self::Silver,
            p if p < WAR_CONTRIBUTION_PLATINUM => Self::Gold,
            p if p < WAR_CONTRIBUTION_LEGENDARY => Self::Platinum,
            _ => Self::Legendary,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Bronze => "Bronze Contributor",
            Self::Silver => "Silver Contributor",
            Self::Gold => "Gold Contributor",
            Self::Platinum => "Platinum Contributor",
            Self::Legendary => "Legendary Contributor",
        }
    }

    /// Reputation bonus for this contribution rank
    pub fn reputation_bonus(&self) -> i32 {
        match self {
            Self::None => 0,
            Self::Bronze => 25,
            Self::Silver => 50,
            Self::Gold => 100,
            Self::Platinum => 200,
            Self::Legendary => 500,
        }
    }

    /// Gold reward multiplier
    pub fn gold_multiplier(&self) -> f32 {
        match self {
            Self::None => 0.0,
            Self::Bronze => 1.0,
            Self::Silver => 1.5,
            Self::Gold => 2.0,
            Self::Platinum => 3.0,
            Self::Legendary => 5.0,
        }
    }
}

/// War rewards based on contribution
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WarReward {
    pub gold: u32,
    pub reputation_bonus: i32,
    pub items: Vec<String>,
    pub titles: Vec<String>,
    pub special_abilities: Vec<FactionAbility>,
}

impl WarReward {
    /// Calculate rewards based on contribution rank and war outcome
    pub fn calculate(contribution: &WarContribution, won: bool) -> Self {
        let base_gold = match contribution.rank {
            ContributionRank::None => 0,
            ContributionRank::Bronze => 500,
            ContributionRank::Silver => 1500,
            ContributionRank::Gold => 5000,
            ContributionRank::Platinum => 15000,
            ContributionRank::Legendary => 50000,
        };

        let gold_multiplier = if won { 1.5 } else { 0.5 };
        let gold = (base_gold as f32 * gold_multiplier) as u32;

        let reputation_bonus = if won {
            contribution.rank.reputation_bonus()
        } else {
            contribution.rank.reputation_bonus() / 2
        };

        let mut items = vec![];
        let mut titles = vec![];
        let mut special_abilities = vec![];

        if won {
            match contribution.rank {
                ContributionRank::Gold => {
                    items.push("War Hero's Medal".to_string());
                }
                ContributionRank::Platinum => {
                    items.push("Champion's Insignia".to_string());
                    titles.push("War Champion".to_string());
                }
                ContributionRank::Legendary => {
                    items.push("Legendary War Trophy".to_string());
                    titles.push("Legendary War Hero".to_string());
                    special_abilities.push(FactionAbility::FactionSummon);
                }
                _ => {}
            }
        }

        Self {
            gold,
            reputation_bonus,
            items,
            titles,
            special_abilities,
        }
    }
}

// =============================================================================
// TERRITORIES
// =============================================================================

/// A territory that can be controlled by factions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Territory {
    pub id: TerritoryId,
    pub name: String,
    pub description: String,
    pub controlling_faction: Option<FactionId>,
    pub contested: bool,
    pub defense_level: u32,
    pub resource_production: TerritoryResources,
    pub buildings: Vec<TerritoryBuilding>,
    pub adjacent_territories: Vec<TerritoryId>,
}

impl Territory {
    pub fn new(id: TerritoryId, name: &str, description: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            controlling_faction: None,
            contested: false,
            defense_level: 50,
            resource_production: TerritoryResources::default(),
            buildings: vec![],
            adjacent_territories: vec![],
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TerritoryResources {
    pub gold_per_turn: u32,
    pub food_per_turn: u32,
    pub materials_per_turn: u32,
    pub mana_crystals_per_turn: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TerritoryBuilding {
    pub name: String,
    pub building_type: BuildingType,
    pub level: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildingType {
    Fortress,
    Barracks,
    Market,
    Temple,
    Workshop,
    Farm,
    Mine,
    MageTower,
    TrainingGround,
}

// =============================================================================
// PLAYER FACTION STATE
// =============================================================================

/// Player's standing with all factions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerFactionStanding {
    pub faction: FactionId,
    pub reputation: i32,
    pub level: ReputationLevel,
    pub rank: Option<FactionRank>,
    pub is_member: bool,
    pub joined_turn: Option<u32>,
    pub quests_completed: u32,
    pub total_reputation_earned: i32,
    pub total_reputation_lost: i32,
    pub kills_for_faction: u32,
    pub kills_against_faction: u32,
}

impl PlayerFactionStanding {
    pub fn new(faction: FactionId) -> Self {
        Self {
            faction,
            reputation: 0,
            level: ReputationLevel::Neutral,
            rank: None,
            is_member: false,
            joined_turn: None,
            quests_completed: 0,
            total_reputation_earned: 0,
            total_reputation_lost: 0,
            kills_for_faction: 0,
            kills_against_faction: 0,
        }
    }

    /// Modify reputation by a given amount
    pub fn modify_reputation(&mut self, amount: i32) {
        let old_reputation = self.reputation;
        self.reputation = (self.reputation + amount).clamp(MIN_REPUTATION, MAX_REPUTATION);

        if amount > 0 {
            self.total_reputation_earned += amount;
        } else {
            self.total_reputation_lost += amount.abs();
        }

        let new_level = ReputationLevel::from_reputation(self.reputation);
        if new_level != self.level {
            self.level = new_level;
        }
    }

    /// Get reputation effects for current level
    pub fn get_effects(&self) -> ReputationEffects {
        ReputationEffects::for_level(self.level, self.faction)
    }

    /// Calculate reputation needed for next level
    pub fn reputation_to_next_level(&self) -> Option<i32> {
        let current_level_idx = self.level as usize;
        if current_level_idx >= 7 {
            // Already Exalted
            return None;
        }

        let next_level = ReputationLevel::all()[current_level_idx + 1];
        Some(next_level.min_reputation() - self.reputation)
    }

    /// Calculate percentage progress to next level
    pub fn progress_to_next_level(&self) -> f32 {
        let min = self.level.min_reputation();
        let max = self.level.max_reputation();
        let range = max - min;

        if range == 0 {
            return 1.0;
        }

        let progress = self.reputation - min;
        (progress as f32 / range as f32).clamp(0.0, 1.0)
    }
}

// =============================================================================
// FACTION EVENTS
// =============================================================================

/// Events that affect faction standings
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FactionEvent {
    /// Player joined a faction
    Joined {
        faction: FactionId,
        turn: u32,
    },
    /// Player left a faction
    Left {
        faction: FactionId,
        turn: u32,
        reason: LeaveReason,
    },
    /// Reputation changed
    ReputationChanged {
        faction: FactionId,
        amount: i32,
        reason: ReputationChangeReason,
        turn: u32,
    },
    /// Player promoted in faction
    Promoted {
        faction: FactionId,
        new_rank: String,
        turn: u32,
    },
    /// Player demoted in faction
    Demoted {
        faction: FactionId,
        new_rank: String,
        turn: u32,
    },
    /// Faction relationship changed
    RelationshipChanged {
        faction_a: FactionId,
        faction_b: FactionId,
        old_relationship: FactionRelationship,
        new_relationship: FactionRelationship,
        turn: u32,
    },
    /// War declared
    WarDeclared {
        attacker: FactionId,
        defender: FactionId,
        turn: u32,
    },
    /// War ended
    WarEnded {
        war_id: u32,
        winner: Option<WarSide>,
        turn: u32,
    },
    /// Alliance formed
    AllianceFormed {
        factions: Vec<FactionId>,
        turn: u32,
    },
    /// Territory changed hands
    TerritoryCapture {
        territory: TerritoryId,
        old_owner: Option<FactionId>,
        new_owner: FactionId,
        turn: u32,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeaveReason {
    Voluntary,
    Expelled,
    Betrayal,
    FactionDestroyed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ReputationChangeReason {
    QuestCompleted { quest_name: String },
    QuestFailed { quest_name: String },
    EnemyKilled { enemy_name: String },
    AlliedKilled { ally_name: String },
    Donation { amount: u32 },
    Theft,
    WarContribution,
    TerritoryDefense,
    TerritoryCapture,
    Diplomacy,
    Gift,
    Betrayal,
    RescueMission,
    SpilloverEffect { source_faction: FactionId },
    Other { description: String },
}

// =============================================================================
// FACTION SYSTEM (Main System Struct)
// =============================================================================

/// The main faction system managing all faction-related gameplay
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactionSystem {
    /// All factions in the game
    pub factions: HashMap<FactionId, Faction>,
    /// Player's standing with each faction
    pub player_standings: HashMap<FactionId, PlayerFactionStanding>,
    /// All active faction pacts
    pub pacts: Vec<FactionPact>,
    /// All territories
    pub territories: HashMap<TerritoryId, Territory>,
    /// Active wars
    pub active_wars: Vec<FactionWar>,
    /// Historical wars (ended)
    pub war_history: Vec<FactionWar>,
    /// Player's war contributions
    pub player_war_contributions: HashMap<u32, WarContribution>,
    /// Faction events history
    pub events: Vec<FactionEvent>,
    /// Current game turn
    pub current_turn: u32,
    /// Next pact ID
    next_pact_id: u32,
    /// Next war ID
    next_war_id: u32,
    /// Next territory ID
    next_territory_id: u32,
}

impl Default for FactionSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl FactionSystem {
    /// Create a new faction system with all factions initialized
    pub fn new() -> Self {
        let mut factions = HashMap::new();
        let mut player_standings = HashMap::new();

        // Initialize all factions
        for faction_id in FactionId::all() {
            factions.insert(*faction_id, Faction::new(*faction_id));
            player_standings.insert(*faction_id, PlayerFactionStanding::new(*faction_id));
        }

        let mut system = Self {
            factions,
            player_standings,
            pacts: vec![],
            territories: HashMap::new(),
            active_wars: vec![],
            war_history: vec![],
            player_war_contributions: HashMap::new(),
            events: vec![],
            current_turn: 0,
            next_pact_id: 1,
            next_war_id: 1,
            next_territory_id: 1,
        };

        // Initialize starting wars
        system.initialize_starting_wars();
        // Initialize territories
        system.initialize_territories();

        system
    }

    /// Initialize starting wars based on faction relationships
    fn initialize_starting_wars(&mut self) {
        // Kingdom of Light vs Dark Empire
        self.declare_war(
            FactionId::KingdomOfLight,
            FactionId::DarkEmpire,
            "The Eternal Conflict",
        );

        // Celestial Host vs Infernal Legions
        self.declare_war(
            FactionId::CelestialHost,
            FactionId::InfernalLegions,
            "War of Heaven and Hell",
        );
    }

    /// Initialize basic territories
    fn initialize_territories(&mut self) {
        let territories_data = [
            (FactionId::KingdomOfLight, "Radiant Plains", "Fertile lands blessed by light"),
            (FactionId::KingdomOfLight, "Holy Citadel Region", "The heart of the Kingdom of Light"),
            (FactionId::DarkEmpire, "Shadow Wastes", "Dark corrupted lands"),
            (FactionId::DarkEmpire, "Obsidian Fortress Territory", "The seat of dark power"),
            (FactionId::NeutralTerritories, "Free Crossroads", "Trading hub for all factions"),
            (FactionId::NeutralTerritories, "Merchant's Coast", "Neutral trading ports"),
            (FactionId::HeavenlySwordSect, "Sword Peak Mountains", "Mountain range of sword cultivators"),
            (FactionId::CelestialHost, "Heavenly Domain", "Divine realm on the mortal plane"),
            (FactionId::NecropolisDominion, "Deathlands", "Cursed lands of the undead"),
            (FactionId::InfernalLegions, "Infernal Rift", "Tear between worlds"),
        ];

        for (faction, name, desc) in territories_data {
            let id = self.next_territory_id;
            self.next_territory_id += 1;

            let mut territory = Territory::new(id, name, desc);
            territory.controlling_faction = Some(faction);

            // Add territory to faction
            if let Some(f) = self.factions.get_mut(&faction) {
                f.territories.push(id);
            }

            self.territories.insert(id, territory);
        }
    }

    /// Get a faction by ID
    pub fn get_faction(&self, id: FactionId) -> Option<&Faction> {
        self.factions.get(&id)
    }

    /// Get a mutable faction by ID
    pub fn get_faction_mut(&mut self, id: FactionId) -> Option<&mut Faction> {
        self.factions.get_mut(&id)
    }

    /// Get player standing with a faction
    pub fn get_standing(&self, faction: FactionId) -> Option<&PlayerFactionStanding> {
        self.player_standings.get(&faction)
    }

    /// Get mutable player standing
    pub fn get_standing_mut(&mut self, faction: FactionId) -> Option<&mut PlayerFactionStanding> {
        self.player_standings.get_mut(&faction)
    }

    /// Get player's reputation with a faction
    pub fn get_reputation(&self, faction: FactionId) -> i32 {
        self.player_standings
            .get(&faction)
            .map(|s| s.reputation)
            .unwrap_or(0)
    }

    /// Get player's reputation level with a faction
    pub fn get_reputation_level(&self, faction: FactionId) -> ReputationLevel {
        self.player_standings
            .get(&faction)
            .map(|s| s.level)
            .unwrap_or(ReputationLevel::Neutral)
    }

    /// Modify player's reputation with a faction
    pub fn modify_reputation(
        &mut self,
        faction: FactionId,
        amount: i32,
        reason: ReputationChangeReason,
    ) {
        // Modify main faction reputation
        if let Some(standing) = self.player_standings.get_mut(&faction) {
            standing.modify_reputation(amount);

            // Record event
            self.events.push(FactionEvent::ReputationChanged {
                faction,
                amount,
                reason: reason.clone(),
                turn: self.current_turn,
            });
        }

        // Apply spillover to related factions
        self.apply_reputation_spillover(faction, amount, reason);
    }

    /// Apply reputation spillover to related factions
    fn apply_reputation_spillover(
        &mut self,
        source_faction: FactionId,
        amount: i32,
        _reason: ReputationChangeReason,
    ) {
        let relationships: Vec<(FactionId, FactionRelationship)> = self
            .factions
            .get(&source_faction)
            .map(|f| f.relationships.iter().map(|(k, v)| (*k, *v)).collect())
            .unwrap_or_default();

        for (related_faction, relationship) in relationships {
            let spillover_rate = relationship.reputation_spillover();
            if spillover_rate.abs() < 0.01 {
                continue;
            }

            let spillover_amount = (amount as f32 * spillover_rate) as i32;
            if spillover_amount == 0 {
                continue;
            }

            if let Some(standing) = self.player_standings.get_mut(&related_faction) {
                standing.modify_reputation(spillover_amount);

                // Record spillover event
                self.events.push(FactionEvent::ReputationChanged {
                    faction: related_faction,
                    amount: spillover_amount,
                    reason: ReputationChangeReason::SpilloverEffect {
                        source_faction,
                    },
                    turn: self.current_turn,
                });
            }
        }
    }

    /// Join a faction
    pub fn join_faction(&mut self, faction: FactionId) -> Result<(), String> {
        // Check if already a member
        if let Some(standing) = self.player_standings.get(&faction) {
            if standing.is_member {
                return Err("Already a member of this faction".to_string());
            }
        }

        // Check reputation requirement (must be at least Neutral)
        let rep_level = self.get_reputation_level(faction);
        if rep_level < ReputationLevel::Neutral {
            return Err(format!(
                "Reputation too low to join. Current: {:?}, Required: Neutral or higher",
                rep_level
            ));
        }

        // Check if faction is exclusive and conflicts with current memberships
        let current_memberships: Vec<FactionId> = self
            .player_standings
            .iter()
            .filter(|(_, s)| s.is_member)
            .map(|(id, _)| *id)
            .collect();

        for membership in &current_memberships {
            if let Some(member_faction) = self.factions.get(membership) {
                if member_faction.is_at_war_with(faction) {
                    return Err(format!(
                        "Cannot join {} - at war with your faction {}",
                        faction.name(),
                        membership.name()
                    ));
                }
            }
        }

        // Join the faction
        if let Some(standing) = self.player_standings.get_mut(&faction) {
            standing.is_member = true;
            standing.joined_turn = Some(self.current_turn);

            // Assign initial rank
            if let Some(f) = self.factions.get(&faction) {
                if let Some(rank) = f.ranks.first() {
                    standing.rank = Some(rank.clone());
                }
            }
        }

        // Record event
        self.events.push(FactionEvent::Joined {
            faction,
            turn: self.current_turn,
        });

        // Reputation bonus for joining
        self.modify_reputation(
            faction,
            50,
            ReputationChangeReason::Other {
                description: "Joined faction".to_string(),
            },
        );

        Ok(())
    }

    /// Leave a faction
    pub fn leave_faction(&mut self, faction: FactionId, reason: LeaveReason) -> Result<(), String> {
        if let Some(standing) = self.player_standings.get_mut(&faction) {
            if !standing.is_member {
                return Err("Not a member of this faction".to_string());
            }

            standing.is_member = false;
            standing.rank = None;

            // Reputation penalty for leaving
            let penalty = match reason {
                LeaveReason::Voluntary => -100,
                LeaveReason::Betrayal => -500,
                LeaveReason::Expelled => -200,
                LeaveReason::FactionDestroyed => 0,
            };

            standing.modify_reputation(penalty);

            self.events.push(FactionEvent::Left {
                faction,
                turn: self.current_turn,
                reason,
            });

            Ok(())
        } else {
            Err("Faction not found".to_string())
        }
    }

    /// Check if player is member of a faction
    pub fn is_member(&self, faction: FactionId) -> bool {
        self.player_standings
            .get(&faction)
            .map(|s| s.is_member)
            .unwrap_or(false)
    }

    /// Get all factions player is a member of
    pub fn get_memberships(&self) -> Vec<FactionId> {
        self.player_standings
            .iter()
            .filter(|(_, s)| s.is_member)
            .map(|(id, _)| *id)
            .collect()
    }

    /// Get reputation effects for a faction
    pub fn get_effects(&self, faction: FactionId) -> ReputationEffects {
        let level = self.get_reputation_level(faction);
        ReputationEffects::for_level(level, faction)
    }

    /// Get combined stat bonuses from all factions
    pub fn get_combined_stat_bonuses(&self) -> FactionStatBonuses {
        let mut combined = FactionStatBonuses::default();

        for (faction_id, standing) in &self.player_standings {
            if standing.level >= ReputationLevel::Friendly {
                let effects = ReputationEffects::for_level(standing.level, *faction_id);
                combined = combined.combine(&effects.stat_bonuses);
            }
        }

        combined
    }

    /// Get shop price modifier for a faction
    pub fn get_shop_modifier(&self, faction: FactionId) -> f32 {
        self.get_effects(faction).shop_price_modifier
    }

    /// Check if player can access faction territory
    pub fn can_access_territory(&self, faction: FactionId) -> bool {
        self.get_effects(faction).territory_access
    }

    /// Check if player can access faction quests
    pub fn can_access_quests(&self, faction: FactionId) -> bool {
        self.get_effects(faction).quest_access
    }

    /// Declare war between factions
    pub fn declare_war(
        &mut self,
        attacker: FactionId,
        defender: FactionId,
        war_name: &str,
    ) -> u32 {
        let war_id = self.next_war_id;
        self.next_war_id += 1;

        let war = FactionWar::new(war_id, war_name, vec![attacker], vec![defender], self.current_turn);
        self.active_wars.push(war);

        // Update faction relationships
        if let Some(f) = self.factions.get_mut(&attacker) {
            f.relationships.insert(defender, FactionRelationship::AtWar);
            if !f.active_wars.contains(&defender) {
                f.active_wars.push(defender);
            }
        }
        if let Some(f) = self.factions.get_mut(&defender) {
            f.relationships.insert(attacker, FactionRelationship::AtWar);
            if !f.active_wars.contains(&attacker) {
                f.active_wars.push(attacker);
            }
        }

        // Record event
        self.events.push(FactionEvent::WarDeclared {
            attacker,
            defender,
            turn: self.current_turn,
        });

        war_id
    }

    /// End a war
    pub fn end_war(&mut self, war_id: u32) -> Option<WarReward> {
        let war_idx = self.active_wars.iter().position(|w| w.id == war_id)?;
        let mut war = self.active_wars.remove(war_idx);

        war.status = WarStatus::Ended;
        war.ended_turn = Some(self.current_turn);

        let winner = war.winner();

        // Update faction relationships
        for attacker in &war.attackers {
            if let Some(f) = self.factions.get_mut(attacker) {
                for defender in &war.defenders {
                    f.active_wars.retain(|w| w != defender);
                    f.relationships.insert(*defender, FactionRelationship::Hostile);
                }
            }
        }
        for defender in &war.defenders {
            if let Some(f) = self.factions.get_mut(defender) {
                for attacker in &war.attackers {
                    f.active_wars.retain(|w| w != attacker);
                    f.relationships.insert(*attacker, FactionRelationship::Hostile);
                }
            }
        }

        // Calculate player rewards if they participated
        let player_reward = self.player_war_contributions.get(&war_id).map(|contrib| {
            let player_side = contrib.faction_side;
            let player_won = match (winner, player_side) {
                (Some(WarSide::Attackers), Some(WarSide::Attackers)) => true,
                (Some(WarSide::Defenders), Some(WarSide::Defenders)) => true,
                _ => false,
            };
            WarReward::calculate(contrib, player_won)
        });

        // Record event
        self.events.push(FactionEvent::WarEnded {
            war_id,
            winner,
            turn: self.current_turn,
        });

        // Move to history
        self.war_history.push(war);

        player_reward
    }

    /// Add player war contribution
    pub fn add_war_contribution(&mut self, war_id: u32, amount: u32, side: WarSide) {
        let contrib = self
            .player_war_contributions
            .entry(war_id)
            .or_insert_with(|| WarContribution::new(war_id));

        contrib.faction_side = Some(side);
        contrib.add_contribution(amount);
    }

    /// Get active war between two factions
    pub fn get_war_between(&self, faction_a: FactionId, faction_b: FactionId) -> Option<&FactionWar> {
        self.active_wars.iter().find(|w| {
            (w.is_attacker(faction_a) && w.is_defender(faction_b))
                || (w.is_attacker(faction_b) && w.is_defender(faction_a))
        })
    }

    /// Get all factions in a category
    pub fn get_factions_by_category(&self, category: FactionCategory) -> Vec<FactionId> {
        FactionId::all()
            .iter()
            .filter(|f| f.category() == category)
            .copied()
            .collect()
    }

    /// Get factions at war with given faction
    pub fn get_enemies(&self, faction: FactionId) -> Vec<FactionId> {
        self.factions
            .get(&faction)
            .map(|f| f.active_wars.clone())
            .unwrap_or_default()
    }

    /// Get allied factions
    pub fn get_allies(&self, faction: FactionId) -> Vec<FactionId> {
        self.factions
            .get(&faction)
            .map(|f| f.active_alliances.clone())
            .unwrap_or_default()
    }

    /// Tick the faction system (call each game turn)
    pub fn tick(&mut self) {
        self.current_turn += 1;

        // Process wars
        for war in &mut self.active_wars {
            // Progress war phases
            if self.current_turn - war.started_turn > 10 && war.phase == WarPhase::Mobilization {
                war.phase = WarPhase::Skirmishes;
            }
            if self.current_turn - war.started_turn > 30 && war.phase == WarPhase::Skirmishes {
                war.phase = WarPhase::OpenBattle;
            }
        }

        // Check for pact expirations
        self.pacts.retain(|pact| {
            if let Some(duration) = pact.duration {
                pact.active && (self.current_turn - pact.formed_turn) < duration
            } else {
                pact.active
            }
        });
    }

    /// Get faction summary for UI display
    pub fn get_faction_summary(&self, faction: FactionId) -> FactionSummary {
        let standing = self.player_standings.get(&faction);
        let faction_data = self.factions.get(&faction);

        FactionSummary {
            id: faction,
            name: faction.name().to_string(),
            category: faction.category(),
            reputation: standing.map(|s| s.reputation).unwrap_or(0),
            level: standing.map(|s| s.level).unwrap_or(ReputationLevel::Neutral),
            is_member: standing.map(|s| s.is_member).unwrap_or(false),
            rank_name: standing.and_then(|s| s.rank.as_ref().map(|r| r.name.clone())),
            at_war: faction_data.map(|f| !f.active_wars.is_empty()).unwrap_or(false),
            enemies: faction_data.map(|f| f.active_wars.len()).unwrap_or(0),
            allies: faction_data.map(|f| f.active_alliances.len()).unwrap_or(0),
        }
    }

    /// Get all faction summaries
    pub fn get_all_summaries(&self) -> Vec<FactionSummary> {
        FactionId::all()
            .iter()
            .map(|id| self.get_faction_summary(*id))
            .collect()
    }

    /// Get factions player can interact with positively
    pub fn get_accessible_factions(&self) -> Vec<FactionId> {
        self.player_standings
            .iter()
            .filter(|(_, s)| s.level >= ReputationLevel::Unfriendly)
            .map(|(id, _)| *id)
            .collect()
    }

    /// Get factions that are hostile to player
    pub fn get_hostile_factions(&self) -> Vec<FactionId> {
        self.player_standings
            .iter()
            .filter(|(_, s)| s.level <= ReputationLevel::Hostile)
            .map(|(id, _)| *id)
            .collect()
    }

    /// Get all special abilities player has unlocked through faction reputation
    pub fn get_unlocked_abilities(&self) -> HashSet<FactionAbility> {
        let mut abilities = HashSet::new();

        for (faction_id, standing) in &self.player_standings {
            if standing.level >= ReputationLevel::Friendly {
                let effects = ReputationEffects::for_level(standing.level, *faction_id);
                for ability in effects.special_abilities {
                    abilities.insert(ability);
                }
            }
        }

        abilities
    }
}

/// Summary of faction status for UI
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactionSummary {
    pub id: FactionId,
    pub name: String,
    pub category: FactionCategory,
    pub reputation: i32,
    pub level: ReputationLevel,
    pub is_member: bool,
    pub rank_name: Option<String>,
    pub at_war: bool,
    pub enemies: usize,
    pub allies: usize,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_faction_count() {
        // Verify we have at least 30 factions
        assert!(FactionId::all().len() >= 30);
    }

    #[test]
    fn test_reputation_levels() {
        assert_eq!(ReputationLevel::from_reputation(-1000), ReputationLevel::Hated);
        assert_eq!(ReputationLevel::from_reputation(-750), ReputationLevel::Hated);
        assert_eq!(ReputationLevel::from_reputation(-749), ReputationLevel::Hostile);
        assert_eq!(ReputationLevel::from_reputation(-500), ReputationLevel::Hostile);
        assert_eq!(ReputationLevel::from_reputation(-499), ReputationLevel::Unfriendly);
        assert_eq!(ReputationLevel::from_reputation(-250), ReputationLevel::Unfriendly);
        assert_eq!(ReputationLevel::from_reputation(-249), ReputationLevel::Neutral);
        assert_eq!(ReputationLevel::from_reputation(0), ReputationLevel::Neutral);
        assert_eq!(ReputationLevel::from_reputation(249), ReputationLevel::Neutral);
        assert_eq!(ReputationLevel::from_reputation(250), ReputationLevel::Friendly);
        assert_eq!(ReputationLevel::from_reputation(499), ReputationLevel::Friendly);
        assert_eq!(ReputationLevel::from_reputation(500), ReputationLevel::Honored);
        assert_eq!(ReputationLevel::from_reputation(749), ReputationLevel::Honored);
        assert_eq!(ReputationLevel::from_reputation(750), ReputationLevel::Revered);
        assert_eq!(ReputationLevel::from_reputation(999), ReputationLevel::Revered);
        assert_eq!(ReputationLevel::from_reputation(1000), ReputationLevel::Exalted);
        assert_eq!(ReputationLevel::from_reputation(2000), ReputationLevel::Exalted);
    }

    #[test]
    fn test_faction_system_creation() {
        let system = FactionSystem::new();

        // Check all factions are initialized
        for faction_id in FactionId::all() {
            assert!(system.factions.contains_key(faction_id));
            assert!(system.player_standings.contains_key(faction_id));
        }
    }

    #[test]
    fn test_reputation_modification() {
        let mut system = FactionSystem::new();

        let initial = system.get_reputation(FactionId::KingdomOfLight);
        system.modify_reputation(
            FactionId::KingdomOfLight,
            100,
            ReputationChangeReason::QuestCompleted {
                quest_name: "Test Quest".to_string(),
            },
        );

        assert_eq!(
            system.get_reputation(FactionId::KingdomOfLight),
            initial + 100
        );
    }

    #[test]
    fn test_faction_joining() {
        let mut system = FactionSystem::new();

        // Should be able to join a neutral faction
        let result = system.join_faction(FactionId::NeutralTerritories);
        assert!(result.is_ok());
        assert!(system.is_member(FactionId::NeutralTerritories));
    }

    #[test]
    fn test_reputation_spillover() {
        let mut system = FactionSystem::new();

        // Gain reputation with Kingdom of Light
        let initial_celestial = system.get_reputation(FactionId::CelestialHost);
        system.modify_reputation(
            FactionId::KingdomOfLight,
            100,
            ReputationChangeReason::QuestCompleted {
                quest_name: "Test".to_string(),
            },
        );

        // Celestial Host should gain spillover (they're allied)
        let new_celestial = system.get_reputation(FactionId::CelestialHost);
        assert!(new_celestial > initial_celestial);
    }

    #[test]
    fn test_faction_categories() {
        // Test that each faction has a valid category
        for faction_id in FactionId::all() {
            let category = faction_id.category();
            assert!(FactionCategory::all().contains(&category));
        }
    }

    #[test]
    fn test_linked_modules() {
        // Test Guild links
        assert_eq!(
            FactionId::GuildBladesBrotherhood.linked_guild(),
            Some(Guild::BladesBrotherhood)
        );

        // Test Species links
        assert_eq!(
            FactionId::HumanAlliance.linked_species(),
            Some(Species::Human)
        );

        // Test Kingdom links
        assert_eq!(
            FactionId::KingdomOfLight.linked_kingdom(),
            Some(Kingdom::Valdoria)
        );
    }

    #[test]
    fn test_war_system() {
        let mut system = FactionSystem::new();

        // Check initial wars exist
        assert!(!system.active_wars.is_empty());

        // Check Kingdom of Light is at war with Dark Empire
        let war = system.get_war_between(FactionId::KingdomOfLight, FactionId::DarkEmpire);
        assert!(war.is_some());
    }

    #[test]
    fn test_reputation_effects() {
        let effects_hated = ReputationEffects::for_level(ReputationLevel::Hated, FactionId::KingdomOfLight);
        assert!(!effects_hated.shop_access);
        assert!(effects_hated.npc_hostile);

        let effects_exalted = ReputationEffects::for_level(ReputationLevel::Exalted, FactionId::KingdomOfLight);
        assert!(effects_exalted.shop_access);
        assert!(!effects_exalted.npc_hostile);
        assert!(effects_exalted.shop_price_modifier < 0.0); // Discount
    }

    #[test]
    fn test_war_contribution() {
        let mut contrib = WarContribution::new(1);

        contrib.add_contribution(150);
        assert_eq!(contrib.rank, ContributionRank::Bronze);

        contrib.add_contribution(400);
        assert_eq!(contrib.rank, ContributionRank::Silver);

        contrib.add_contribution(500);
        assert_eq!(contrib.rank, ContributionRank::Gold);
    }
}
