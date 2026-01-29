//! Story and Campaign System for ShadowCrypt
//!
//! This module provides a comprehensive narrative system including:
//! - Main story chapters (10 acts spanning floors 1-100)
//! - Cutscenes and narrative moments
//! - Side stories (companion, faction, romance)
//! - Story NPCs (villains, allies, rivals)
//! - Branching choices affecting endings
//! - Journal and story tracking

use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};

// ============================================================================
// CONSTANTS
// ============================================================================

/// Total number of story acts
pub const TOTAL_ACTS: usize = 10;

/// Floors per act
pub const FLOORS_PER_ACT: u32 = 10;

/// Maximum story choices tracked
pub const MAX_TRACKED_CHOICES: usize = 100;

// ============================================================================
// STORY IDENTIFIERS
// ============================================================================

/// Unique identifier for story acts
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum ActId {
    Act1TheAwakening,
    Act2TheGatheringStorm,
    Act3DescentIntoDarkness,
    Act4TheBurningPath,
    Act5FrozenMemories,
    Act6NaturesWrath,
    Act7CrystalDreams,
    Act8BetweenWorlds,
    Act9DivineConflict,
    Act10TheFinalReckoning,
}

impl ActId {
    /// Returns the act number (1-10)
    pub fn number(&self) -> u32 {
        match self {
            Self::Act1TheAwakening => 1,
            Self::Act2TheGatheringStorm => 2,
            Self::Act3DescentIntoDarkness => 3,
            Self::Act4TheBurningPath => 4,
            Self::Act5FrozenMemories => 5,
            Self::Act6NaturesWrath => 6,
            Self::Act7CrystalDreams => 7,
            Self::Act8BetweenWorlds => 8,
            Self::Act9DivineConflict => 9,
            Self::Act10TheFinalReckoning => 10,
        }
    }

    /// Returns the floor range for this act
    pub fn floor_range(&self) -> (u32, u32) {
        let start = (self.number() - 1) * FLOORS_PER_ACT + 1;
        let end = self.number() * FLOORS_PER_ACT;
        (start, end)
    }

    /// Returns the act for a given floor
    pub fn from_floor(floor: u32) -> Option<Self> {
        match floor {
            1..=10 => Some(Self::Act1TheAwakening),
            11..=20 => Some(Self::Act2TheGatheringStorm),
            21..=30 => Some(Self::Act3DescentIntoDarkness),
            31..=40 => Some(Self::Act4TheBurningPath),
            41..=50 => Some(Self::Act5FrozenMemories),
            51..=60 => Some(Self::Act6NaturesWrath),
            61..=70 => Some(Self::Act7CrystalDreams),
            71..=80 => Some(Self::Act8BetweenWorlds),
            81..=90 => Some(Self::Act9DivineConflict),
            91..=100 => Some(Self::Act10TheFinalReckoning),
            _ => None,
        }
    }

    /// Returns the display name of this act
    pub fn name(&self) -> &'static str {
        match self {
            Self::Act1TheAwakening => "The Awakening",
            Self::Act2TheGatheringStorm => "The Gathering Storm",
            Self::Act3DescentIntoDarkness => "Descent into Darkness",
            Self::Act4TheBurningPath => "The Burning Path",
            Self::Act5FrozenMemories => "Frozen Memories",
            Self::Act6NaturesWrath => "Nature's Wrath",
            Self::Act7CrystalDreams => "Crystal Dreams",
            Self::Act8BetweenWorlds => "Between Worlds",
            Self::Act9DivineConflict => "Divine Conflict",
            Self::Act10TheFinalReckoning => "The Final Reckoning",
        }
    }

    /// Returns a brief description of this act
    pub fn description(&self) -> &'static str {
        match self {
            Self::Act1TheAwakening => "You awaken in the Shadow Crypts with fragmented memories. The journey begins.",
            Self::Act2TheGatheringStorm => "Dark forces stir in the depths. Allies and enemies reveal themselves.",
            Self::Act3DescentIntoDarkness => "The true nature of the crypts emerges. Sanity begins to fray.",
            Self::Act4TheBurningPath => "Through volcanic caverns, past trials of fire. The path demands sacrifice.",
            Self::Act5FrozenMemories => "In frozen halls, memories crystallize. Truths long buried surface.",
            Self::Act6NaturesWrath => "The corrupted wild fights back. Balance must be restored or destroyed.",
            Self::Act7CrystalDreams => "Reality bends in crystalline chambers. Dreams become weapons.",
            Self::Act8BetweenWorlds => "The veil thins. Walk between realms to find the final answers.",
            Self::Act9DivineConflict => "Gods and demons clash. Choose your side in the eternal war.",
            Self::Act10TheFinalReckoning => "The end of all things approaches. Face your destiny.",
        }
    }

    /// Returns all acts in order
    pub fn all() -> Vec<Self> {
        vec![
            Self::Act1TheAwakening,
            Self::Act2TheGatheringStorm,
            Self::Act3DescentIntoDarkness,
            Self::Act4TheBurningPath,
            Self::Act5FrozenMemories,
            Self::Act6NaturesWrath,
            Self::Act7CrystalDreams,
            Self::Act8BetweenWorlds,
            Self::Act9DivineConflict,
            Self::Act10TheFinalReckoning,
        ]
    }
}

/// Unique identifier for cutscenes
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum CutsceneId {
    // Act 1 Cutscenes
    Intro,
    FirstEnemy,
    MeetMentor,
    FirstBoss,
    Act1Complete,

    // Act 2 Cutscenes
    Act2Intro,
    MeetRival,
    DarkOmen,
    ShadowVision,
    Act2Complete,

    // Act 3 Cutscenes
    Act3Intro,
    TrueNatureRevealed,
    BetrayalScene,
    MadnessDescends,
    Act3Complete,

    // Act 4 Cutscenes
    Act4Intro,
    TrialOfFire,
    SacrificeRequired,
    PhoenixRising,
    Act4Complete,

    // Act 5 Cutscenes
    Act5Intro,
    FrozenPast,
    MemoryUnlocked,
    TruthRevealed,
    Act5Complete,

    // Act 6 Cutscenes
    Act6Intro,
    NatureCorrupted,
    GuardianAwakens,
    BalanceChoice,
    Act6Complete,

    // Act 7 Cutscenes
    Act7Intro,
    DreamRealm,
    NightmareManifest,
    CrystalHeart,
    Act7Complete,

    // Act 8 Cutscenes
    Act8Intro,
    VeilTorn,
    ParallelSelf,
    WorldsMerge,
    Act8Complete,

    // Act 9 Cutscenes
    Act9Intro,
    DivineIntervention,
    DemonPact,
    GodsChoice,
    Act9Complete,

    // Act 10 Cutscenes
    Act10Intro,
    FinalPreparation,
    VillainConfrontation,
    UltimateChoice,

    // Endings
    EndingRedemption,
    EndingDamnation,
    EndingAscension,
    EndingBalance,
    EndingTrueEnding,
    EndingSecretEnding,
}

impl CutsceneId {
    /// Returns the act this cutscene belongs to
    pub fn act(&self) -> Option<ActId> {
        match self {
            Self::Intro | Self::FirstEnemy | Self::MeetMentor |
            Self::FirstBoss | Self::Act1Complete => Some(ActId::Act1TheAwakening),

            Self::Act2Intro | Self::MeetRival | Self::DarkOmen |
            Self::ShadowVision | Self::Act2Complete => Some(ActId::Act2TheGatheringStorm),

            Self::Act3Intro | Self::TrueNatureRevealed | Self::BetrayalScene |
            Self::MadnessDescends | Self::Act3Complete => Some(ActId::Act3DescentIntoDarkness),

            Self::Act4Intro | Self::TrialOfFire | Self::SacrificeRequired |
            Self::PhoenixRising | Self::Act4Complete => Some(ActId::Act4TheBurningPath),

            Self::Act5Intro | Self::FrozenPast | Self::MemoryUnlocked |
            Self::TruthRevealed | Self::Act5Complete => Some(ActId::Act5FrozenMemories),

            Self::Act6Intro | Self::NatureCorrupted | Self::GuardianAwakens |
            Self::BalanceChoice | Self::Act6Complete => Some(ActId::Act6NaturesWrath),

            Self::Act7Intro | Self::DreamRealm | Self::NightmareManifest |
            Self::CrystalHeart | Self::Act7Complete => Some(ActId::Act7CrystalDreams),

            Self::Act8Intro | Self::VeilTorn | Self::ParallelSelf |
            Self::WorldsMerge | Self::Act8Complete => Some(ActId::Act8BetweenWorlds),

            Self::Act9Intro | Self::DivineIntervention | Self::DemonPact |
            Self::GodsChoice | Self::Act9Complete => Some(ActId::Act9DivineConflict),

            Self::Act10Intro | Self::FinalPreparation | Self::VillainConfrontation |
            Self::UltimateChoice => Some(ActId::Act10TheFinalReckoning),

            // Endings don't belong to a specific act
            _ => None,
        }
    }
}

/// Unique identifier for story NPCs
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum StoryNPCId {
    // Main Villain
    Malachar,           // The Shadow Lord, main antagonist

    // Allies and Mentors
    ElderSoren,         // Wise mentor who guides the player
    CaptainVex,         // Battle-hardened warrior ally
    PriestessLyanna,    // Holy healer and spiritual guide
    ArchmageTheron,     // Powerful mage with knowledge of the crypts

    // Rival Adventurers
    RavenDark,          // Ambitious rival who seeks the same power
    SilasThorne,        // Treasure hunter with shifting loyalties
    BladeDancer,        // Mysterious assassin with unclear motives

    // Mysterious Figures
    TheWatcher,         // Enigmatic observer throughout the journey
    TheOracleSeer,      // Prophet who speaks in riddles
    ShadowSelf,         // The player's dark reflection
    TheFirstHero,       // Ancient hero trapped in the crypts

    // Faction Representatives
    GuildmasterCrane,   // Leader of the Adventurer's Guild
    HighPriestMarus,    // Head of the Temple of Light
    DarkLordVex,        // Representative of the Shadow Covenant
    NatureGuardianFae,  // Voice of the corrupted wild

    // Act-Specific NPCs
    GoblinKingGrix,     // Act 1 boss turned potential ally
    OrcWarlordKrag,     // Act 2 antagonist
    VampireLordSanguine,// Act 3 boss with complex motivations
    FirelordIgnis,      // Act 4 trial master
    FrostQueenGelida,   // Act 5 keeper of frozen memories
    DruidEldertree,     // Act 6 corrupted guardian
    DreamweaverSomnia,  // Act 7 ruler of crystal dreams
    VoidwalkerNull,     // Act 8 guide between worlds
    CelestialAuriel,    // Act 9 divine representative
    InfernalAzmodeus,   // Act 9 demonic representative
}

impl StoryNPCId {
    /// Returns the display name of this NPC
    pub fn name(&self) -> &'static str {
        match self {
            Self::Malachar => "Malachar, The Shadow Lord",
            Self::ElderSoren => "Elder Soren",
            Self::CaptainVex => "Captain Vex",
            Self::PriestessLyanna => "Priestess Lyanna",
            Self::ArchmageTheron => "Archmage Theron",
            Self::RavenDark => "Raven Dark",
            Self::SilasThorne => "Silas Thorne",
            Self::BladeDancer => "The Blade Dancer",
            Self::TheWatcher => "The Watcher",
            Self::TheOracleSeer => "The Oracle Seer",
            Self::ShadowSelf => "Your Shadow Self",
            Self::TheFirstHero => "The First Hero",
            Self::GuildmasterCrane => "Guildmaster Crane",
            Self::HighPriestMarus => "High Priest Marus",
            Self::DarkLordVex => "Dark Lord Vex",
            Self::NatureGuardianFae => "Nature Guardian Fae",
            Self::GoblinKingGrix => "Goblin King Grix",
            Self::OrcWarlordKrag => "Orc Warlord Krag",
            Self::VampireLordSanguine => "Vampire Lord Sanguine",
            Self::FirelordIgnis => "Firelord Ignis",
            Self::FrostQueenGelida => "Frost Queen Gelida",
            Self::DruidEldertree => "Druid Eldertree",
            Self::DreamweaverSomnia => "Dreamweaver Somnia",
            Self::VoidwalkerNull => "Voidwalker Null",
            Self::CelestialAuriel => "Celestial Auriel",
            Self::InfernalAzmodeus => "Infernal Azmodeus",
        }
    }

    /// Returns a brief description of this NPC
    pub fn description(&self) -> &'static str {
        match self {
            Self::Malachar => "The ancient Shadow Lord who seeks to consume all light. Once a hero who fell to darkness.",
            Self::ElderSoren => "A wise sage who has studied the Shadow Crypts for decades. He knows more than he reveals.",
            Self::CaptainVex => "A grizzled warrior who lost their company to the crypts. Seeks vengeance and redemption.",
            Self::PriestessLyanna => "A devoted healer whose faith is tested by the horrors of the deep.",
            Self::ArchmageTheron => "An ambitious mage whose pursuit of forbidden knowledge led him into the darkness.",
            Self::RavenDark => "Your rival since childhood, always one step ahead, always seeking the same goals.",
            Self::SilasThorne => "A charming rogue whose loyalty changes with the wind and gold.",
            Self::BladeDancer => "A silent killer who appears at crucial moments. Friend or foe remains unclear.",
            Self::TheWatcher => "An entity that observes your journey from the shadows. Its purpose is unknown.",
            Self::TheOracleSeer => "A blind prophet who sees futures that may or may not come to pass.",
            Self::ShadowSelf => "Your darkest reflection, born from the crypts' corrupting influence.",
            Self::TheFirstHero => "The legendary warrior who first sealed the Shadow Lord, now trapped between life and death.",
            Self::GuildmasterCrane => "The pragmatic leader who sends adventurers into the crypts for profit and protection.",
            Self::HighPriestMarus => "Head of the Temple of Light, dedicated to sealing the darkness forever.",
            Self::DarkLordVex => "A cultist leader who believes darkness should be embraced, not fought.",
            Self::NatureGuardianFae => "Once protector of the wild places, now twisted by shadow corruption.",
            Self::GoblinKingGrix => "Cunning ruler of the upper dungeon goblin tribes. May be reasoned with.",
            Self::OrcWarlordKrag => "Brutal leader of the orc horde. Respects only strength.",
            Self::VampireLordSanguine => "Ancient vampire noble who witnessed the world before the Shadowfall.",
            Self::FirelordIgnis => "Elemental lord of flame who tests those who would pass through his domain.",
            Self::FrostQueenGelida => "Keeper of frozen memories and guardian of ancient truths.",
            Self::DruidEldertree => "The last druid, corrupted but not beyond redemption.",
            Self::DreamweaverSomnia => "Ruler of the crystalline dream realm, blurring fantasy and reality.",
            Self::VoidwalkerNull => "A being who exists between dimensions, offering forbidden passage.",
            Self::CelestialAuriel => "Divine messenger sent to aid the worthy and judge the fallen.",
            Self::InfernalAzmodeus => "Demon lord who offers power at a terrible price.",
        }
    }

    /// Returns the NPC's role in the story
    pub fn role(&self) -> NPCRole {
        match self {
            Self::Malachar => NPCRole::MainVillain,
            Self::ElderSoren | Self::CaptainVex | Self::PriestessLyanna |
            Self::ArchmageTheron => NPCRole::AllyMentor,
            Self::RavenDark | Self::SilasThorne | Self::BladeDancer => NPCRole::RivalAdventurer,
            Self::TheWatcher | Self::TheOracleSeer | Self::ShadowSelf |
            Self::TheFirstHero => NPCRole::MysteriousFigure,
            Self::GuildmasterCrane | Self::HighPriestMarus | Self::DarkLordVex |
            Self::NatureGuardianFae => NPCRole::FactionRepresentative,
            _ => NPCRole::ActSpecific,
        }
    }
}

/// Role categories for story NPCs
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum NPCRole {
    MainVillain,
    AllyMentor,
    RivalAdventurer,
    MysteriousFigure,
    FactionRepresentative,
    ActSpecific,
}

impl NPCRole {
    pub fn name(&self) -> &'static str {
        match self {
            Self::MainVillain => "Main Villain",
            Self::AllyMentor => "Ally & Mentor",
            Self::RivalAdventurer => "Rival Adventurer",
            Self::MysteriousFigure => "Mysterious Figure",
            Self::FactionRepresentative => "Faction Representative",
            Self::ActSpecific => "Act Character",
        }
    }
}

// ============================================================================
// STORY CHOICES AND CONSEQUENCES
// ============================================================================

/// Types of story choices
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum StoryChoiceType {
    /// Choices affecting the ending
    EndingDecision,
    /// Moral/ethical choices
    MoralChoice,
    /// Faction alignment choices
    FactionAlignment,
    /// Character relationship choices
    RelationshipChoice,
    /// Side story branching choices
    SideStoryBranch,
}

/// Unique identifier for story choices
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum StoryChoiceId {
    // Act 1 Choices
    SpareGoblinKing,
    TrustMentor,
    AcceptShadowPower,

    // Act 2 Choices
    AllyWithRival,
    WarnTown,
    EmbraceDarkness,

    // Act 3 Choices
    ForgiveBetrayer,
    SacrificeSanity,
    SeekForbiddenKnowledge,

    // Act 4 Choices
    WalkThroughFire,
    SacrificeCompanion,
    AcceptPhoenixBoon,

    // Act 5 Choices
    UnlockAllMemories,
    ForgetThePast,
    ShareTruth,

    // Act 6 Choices
    HealNature,
    ControlNature,
    DestroyCorruption,

    // Act 7 Choices
    MasterDreams,
    RejectIllusions,
    MergeWithNightmare,

    // Act 8 Choices
    UnifySelf,
    AbsorbParallel,
    RejectBothWorlds,

    // Act 9 Choices
    SideDivine,
    SideInfernal,
    RejectBothPowers,
    ForgeOwnPath,

    // Act 10 Choices
    RedeemVillain,
    DestroyVillain,
    BecomeNewVillain,
    TranscendMortality,

    // Faction Choices
    JoinAdventurersGuild,
    JoinTempleOfLight,
    JoinShadowCovenant,
    RemainIndependent,

    // Romance Choices
    RomanceLyanna,
    RomanceVex,
    RomanceRaven,
    RomanceSilas,
    RomanceNone,

    // Companion Choices
    SaveCompanionLife,
    SacrificeForCompanion,
    BetrayCompanion,
}

impl StoryChoiceId {
    /// Returns the type of this choice
    pub fn choice_type(&self) -> StoryChoiceType {
        match self {
            Self::RedeemVillain | Self::DestroyVillain | Self::BecomeNewVillain |
            Self::TranscendMortality | Self::SideDivine | Self::SideInfernal |
            Self::RejectBothPowers | Self::ForgeOwnPath => StoryChoiceType::EndingDecision,

            Self::ForgiveBetrayer | Self::SacrificeCompanion | Self::SacrificeSanity |
            Self::SpareGoblinKing | Self::WarnTown | Self::ShareTruth |
            Self::SaveCompanionLife | Self::SacrificeForCompanion | Self::BetrayCompanion => StoryChoiceType::MoralChoice,

            Self::JoinAdventurersGuild | Self::JoinTempleOfLight | Self::JoinShadowCovenant |
            Self::RemainIndependent => StoryChoiceType::FactionAlignment,

            Self::RomanceLyanna | Self::RomanceVex | Self::RomanceRaven |
            Self::RomanceSilas | Self::RomanceNone | Self::AllyWithRival |
            Self::TrustMentor => StoryChoiceType::RelationshipChoice,

            _ => StoryChoiceType::SideStoryBranch,
        }
    }

    /// Returns a description of this choice
    pub fn description(&self) -> &'static str {
        match self {
            Self::SpareGoblinKing => "Show mercy to the defeated Goblin King",
            Self::TrustMentor => "Place your complete trust in Elder Soren",
            Self::AcceptShadowPower => "Accept the power offered by the shadows",
            Self::AllyWithRival => "Form an alliance with your rival Raven Dark",
            Self::WarnTown => "Warn the surface town of the gathering darkness",
            Self::EmbraceDarkness => "Embrace the darkness within yourself",
            Self::ForgiveBetrayer => "Forgive the one who betrayed you",
            Self::SacrificeSanity => "Sacrifice your sanity to gain forbidden knowledge",
            Self::SeekForbiddenKnowledge => "Seek the forbidden knowledge of the ancients",
            Self::WalkThroughFire => "Walk through the flames without protection",
            Self::SacrificeCompanion => "Allow a companion to sacrifice themselves",
            Self::AcceptPhoenixBoon => "Accept the Phoenix's blessing of rebirth",
            Self::UnlockAllMemories => "Unlock all your forgotten memories",
            Self::ForgetThePast => "Choose to forget your painful past",
            Self::ShareTruth => "Share the terrible truth with your allies",
            Self::HealNature => "Attempt to heal the corrupted nature",
            Self::ControlNature => "Bend the corrupted nature to your will",
            Self::DestroyCorruption => "Destroy the corruption entirely",
            Self::MasterDreams => "Master the power of the crystal dreams",
            Self::RejectIllusions => "Reject all illusions and face harsh reality",
            Self::MergeWithNightmare => "Merge with your nightmare self",
            Self::UnifySelf => "Unify with your parallel self",
            Self::AbsorbParallel => "Absorb your parallel self's power",
            Self::RejectBothWorlds => "Reject both worlds and forge a new path",
            Self::SideDivine => "Side with the divine forces",
            Self::SideInfernal => "Side with the infernal forces",
            Self::RejectBothPowers => "Reject both divine and infernal powers",
            Self::ForgeOwnPath => "Forge your own path beyond gods and demons",
            Self::RedeemVillain => "Attempt to redeem the Shadow Lord",
            Self::DestroyVillain => "Destroy the Shadow Lord completely",
            Self::BecomeNewVillain => "Take the Shadow Lord's place",
            Self::TranscendMortality => "Transcend your mortal form",
            Self::JoinAdventurersGuild => "Join the Adventurer's Guild",
            Self::JoinTempleOfLight => "Join the Temple of Light",
            Self::JoinShadowCovenant => "Join the Shadow Covenant",
            Self::RemainIndependent => "Remain independent of all factions",
            Self::RomanceLyanna => "Pursue a romance with Priestess Lyanna",
            Self::RomanceVex => "Pursue a romance with Captain Vex",
            Self::RomanceRaven => "Pursue a romance with Raven Dark",
            Self::RomanceSilas => "Pursue a romance with Silas Thorne",
            Self::RomanceNone => "Focus on the mission, not romance",
            Self::SaveCompanionLife => "Risk everything to save your companion",
            Self::SacrificeForCompanion => "Sacrifice yourself for your companion",
            Self::BetrayCompanion => "Betray your companion for power",
        }
    }
}

/// A recorded story choice
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StoryChoice {
    pub id: StoryChoiceId,
    pub description: String,
    pub floor_made: u32,
    pub turn_made: u32,
    pub consequences: Vec<String>,
}

// ============================================================================
// ENDINGS
// ============================================================================

/// Possible story endings
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum Ending {
    /// Redeemed the villain and brought peace
    Redemption,
    /// Fell to darkness and became the new villain
    Damnation,
    /// Transcended mortality to become something beyond
    Ascension,
    /// Maintained balance between light and darkness
    Balance,
    /// The true ending - discovered the whole truth
    TrueEnding,
    /// Secret ending - requires specific choices and discoveries
    SecretEnding,
}

impl Ending {
    /// Returns the display name of this ending
    pub fn name(&self) -> &'static str {
        match self {
            Self::Redemption => "Redemption",
            Self::Damnation => "Damnation",
            Self::Ascension => "Ascension",
            Self::Balance => "Balance",
            Self::TrueEnding => "The True Ending",
            Self::SecretEnding => "The Secret Ending",
        }
    }

    /// Returns the ending description
    pub fn description(&self) -> &'static str {
        match self {
            Self::Redemption => "Through compassion and sacrifice, you reached out to the Shadow Lord's buried humanity. Malachar, remembering his days as a hero, helped seal the darkness forever. Peace returns to the world, and you are hailed as a true hero.",
            Self::Damnation => "Power corrupts, and absolute power corrupts absolutely. You claimed the Shadow Lord's throne, becoming the very thing you sought to destroy. The cycle begins anew, and the world trembles.",
            Self::Ascension => "Rejecting the false binary of good and evil, you transcended your mortal form. Neither light nor shadow, you became something new - a guardian existing beyond the veil, watching over both realms.",
            Self::Balance => "Light cannot exist without shadow. You understood this truth and chose to maintain the eternal balance. The crypts remain, but contained. You become the eternal guardian of the threshold.",
            Self::TrueEnding => "You discovered the ultimate truth: the Shadow Crypts are a wound in reality itself, and the Shadow Lord was merely its avatar. By healing the wound at its source, you unmade the crypts entirely and freed countless souls.",
            Self::SecretEnding => "By following the path of the First Hero, collecting all memories, and mastering both dreams and reality, you discovered the hidden history. The Shadow Lord was your future self, and by refusing your destiny, you created a new timeline entirely.",
        }
    }

    /// Returns the requirements for this ending
    pub fn requirements_hint(&self) -> &'static str {
        match self {
            Self::Redemption => "Show mercy throughout your journey and attempt to redeem the Shadow Lord.",
            Self::Damnation => "Embrace darkness and shadow powers at every opportunity.",
            Self::Ascension => "Reject both divine and infernal paths to forge your own way.",
            Self::Balance => "Make balanced choices and heal the corrupted nature.",
            Self::TrueEnding => "Discover all lore entries and unlock all your memories.",
            Self::SecretEnding => "???",
        }
    }
}

// ============================================================================
// CUTSCENES
// ============================================================================

/// A text-based cutscene/narrative moment
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Cutscene {
    pub id: CutsceneId,
    pub title: String,
    pub act: Option<ActId>,
    pub scenes: Vec<CutsceneScene>,
    pub triggers: Vec<CutsceneTrigger>,
    pub requirements: Vec<CutsceneRequirement>,
}

/// A single scene within a cutscene
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CutsceneScene {
    pub speaker: Option<StoryNPCId>,
    pub text: String,
    pub choice: Option<CutsceneChoice>,
}

/// A choice presented during a cutscene
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CutsceneChoice {
    pub prompt: String,
    pub options: Vec<CutsceneOption>,
}

/// An option in a cutscene choice
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CutsceneOption {
    pub text: String,
    pub story_choice: Option<StoryChoiceId>,
    pub consequence_text: String,
}

/// Conditions that trigger a cutscene
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum CutsceneTrigger {
    EnterFloor(u32),
    DefeatBoss(u32),
    FirstEnemyKill,
    ItemFound(String),
    NPCMet(StoryNPCId),
    StoryChoiceMade(StoryChoiceId),
    LoreDiscovered(String),
    CompanionRecruited(String),
}

/// Requirements for a cutscene to be available
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum CutsceneRequirement {
    ActCompleted(ActId),
    CutsceneViewed(CutsceneId),
    ChoiceMade(StoryChoiceId),
    ChoiceNotMade(StoryChoiceId),
    MinFloor(u32),
    MaxFloor(u32),
    NPCRelationship(StoryNPCId, i32),
}

// ============================================================================
// SIDE STORIES
// ============================================================================

/// Categories of side stories
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum SideStoryCategory {
    Companion,
    Faction,
    Romance,
    HiddenLore,
}

impl SideStoryCategory {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Companion => "Companion Stories",
            Self::Faction => "Faction Stories",
            Self::Romance => "Romance Arcs",
            Self::HiddenLore => "Hidden Lore",
        }
    }
}

/// Unique identifier for side stories
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum SideStoryId {
    // Companion Stories
    VexRedemption,
    LyannaCrisis,
    TheronAmbition,
    SorenSecrets,

    // Faction Stories
    GuildConflict,
    TempleSchism,
    CovenantRising,
    NatureBond,

    // Romance Arcs
    LyannaDevotion,
    VexTrust,
    RavenRivalry,
    SilasBetrayal,

    // Hidden Lore
    FirstHeroTale,
    ShadowfallOrigin,
    TrueProphecy,
    LostKingdom,
    ForgottenGods,
}

impl SideStoryId {
    pub fn category(&self) -> SideStoryCategory {
        match self {
            Self::VexRedemption | Self::LyannaCrisis | Self::TheronAmbition |
            Self::SorenSecrets => SideStoryCategory::Companion,

            Self::GuildConflict | Self::TempleSchism | Self::CovenantRising |
            Self::NatureBond => SideStoryCategory::Faction,

            Self::LyannaDevotion | Self::VexTrust | Self::RavenRivalry |
            Self::SilasBetrayal => SideStoryCategory::Romance,

            Self::FirstHeroTale | Self::ShadowfallOrigin | Self::TrueProphecy |
            Self::LostKingdom | Self::ForgottenGods => SideStoryCategory::HiddenLore,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::VexRedemption => "Vex's Redemption",
            Self::LyannaCrisis => "Lyanna's Crisis of Faith",
            Self::TheronAmbition => "Theron's Dangerous Ambition",
            Self::SorenSecrets => "Soren's Hidden Secrets",
            Self::GuildConflict => "Guild Civil War",
            Self::TempleSchism => "The Temple Schism",
            Self::CovenantRising => "Shadow Covenant Rising",
            Self::NatureBond => "Bond with Nature",
            Self::LyannaDevotion => "Path of Devotion",
            Self::VexTrust => "Trust Hard Won",
            Self::RavenRivalry => "Rivalry to Romance",
            Self::SilasBetrayal => "The Charming Betrayer",
            Self::FirstHeroTale => "The First Hero's Tale",
            Self::ShadowfallOrigin => "Origin of the Shadowfall",
            Self::TrueProphecy => "The True Prophecy",
            Self::LostKingdom => "The Lost Kingdom",
            Self::ForgottenGods => "The Forgotten Gods",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::VexRedemption => "Help Captain Vex confront the ghosts of their past and find peace.",
            Self::LyannaCrisis => "Support Priestess Lyanna as her faith is tested to its limits.",
            Self::TheronAmbition => "Witness Archmage Theron's descent into obsession with forbidden power.",
            Self::SorenSecrets => "Discover what Elder Soren has been hiding all along.",
            Self::GuildConflict => "Navigate the political turmoil tearing the Adventurer's Guild apart.",
            Self::TempleSchism => "The Temple of Light divides over how to handle the darkness.",
            Self::CovenantRising => "The Shadow Covenant makes their move for power.",
            Self::NatureBond => "Forge a connection with the spirits of the corrupted wild.",
            Self::LyannaDevotion => "A path of love and faith with Priestess Lyanna.",
            Self::VexTrust => "Earn the trust and heart of the hardened warrior.",
            Self::RavenRivalry => "Transform rivalry into something deeper with Raven Dark.",
            Self::SilasBetrayal => "Navigate the dangerous affections of Silas Thorne.",
            Self::FirstHeroTale => "Uncover the complete history of the legendary First Hero.",
            Self::ShadowfallOrigin => "Learn the true cause of the Shadowfall catastrophe.",
            Self::TrueProphecy => "Discover what the prophecy really foretells.",
            Self::LostKingdom => "Find traces of the kingdom that existed before the crypts.",
            Self::ForgottenGods => "Learn about the gods who were erased from history.",
        }
    }
}

/// A side story with chapters and progress
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SideStory {
    pub id: SideStoryId,
    pub category: SideStoryCategory,
    pub name: String,
    pub description: String,
    pub chapters: Vec<SideStoryChapter>,
    pub current_chapter: usize,
    pub completed: bool,
    pub unlocked: bool,
}

/// A chapter in a side story
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SideStoryChapter {
    pub title: String,
    pub description: String,
    pub objectives: Vec<String>,
    pub completed: bool,
    pub cutscenes: Vec<CutsceneId>,
}

// ============================================================================
// JOURNAL AND TRACKING
// ============================================================================

/// A journal entry recording story events
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct JournalEntry {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub floor: u32,
    pub turn: u32,
    pub category: JournalCategory,
    pub related_npc: Option<StoryNPCId>,
    pub related_act: Option<ActId>,
}

/// Categories for journal entries
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum JournalCategory {
    MainStory,
    SideStory,
    NPCEncounter,
    Discovery,
    PersonalThought,
    ImportantChoice,
}

impl JournalCategory {
    pub fn name(&self) -> &'static str {
        match self {
            Self::MainStory => "Main Story",
            Self::SideStory => "Side Story",
            Self::NPCEncounter => "NPC Encounters",
            Self::Discovery => "Discoveries",
            Self::PersonalThought => "Personal Thoughts",
            Self::ImportantChoice => "Important Choices",
        }
    }
}

/// Lore entry for the codex
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LoreCollectionEntry {
    pub id: String,
    pub title: String,
    pub content: String,
    pub category: LoreCollectionCategory,
    pub discovered: bool,
    pub floor_discovered: Option<u32>,
}

/// Categories for collected lore
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum LoreCollectionCategory {
    WorldHistory,
    Characters,
    Factions,
    Locations,
    Artifacts,
    Prophecies,
    Mysteries,
}

impl LoreCollectionCategory {
    pub fn name(&self) -> &'static str {
        match self {
            Self::WorldHistory => "World History",
            Self::Characters => "Characters",
            Self::Factions => "Factions",
            Self::Locations => "Locations",
            Self::Artifacts => "Artifacts",
            Self::Prophecies => "Prophecies",
            Self::Mysteries => "Mysteries",
        }
    }
}

// ============================================================================
// NPC RELATIONSHIPS
// ============================================================================

/// Relationship status with story NPCs
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NPCRelationship {
    pub npc_id: StoryNPCId,
    pub affinity: i32,       // -100 to 100
    pub trust: i32,          // -100 to 100
    pub interactions: u32,
    pub status: RelationshipStatus,
    pub known_secrets: Vec<String>,
    pub events: Vec<RelationshipEvent>,
}

impl NPCRelationship {
    pub fn new(npc_id: StoryNPCId) -> Self {
        Self {
            npc_id,
            affinity: 0,
            trust: 0,
            interactions: 0,
            status: RelationshipStatus::Stranger,
            known_secrets: Vec::new(),
            events: Vec::new(),
        }
    }

    /// Modify affinity within bounds
    pub fn modify_affinity(&mut self, amount: i32) {
        self.affinity = (self.affinity + amount).clamp(-100, 100);
        self.update_status();
    }

    /// Modify trust within bounds
    pub fn modify_trust(&mut self, amount: i32) {
        self.trust = (self.trust + amount).clamp(-100, 100);
        self.update_status();
    }

    /// Update relationship status based on affinity and trust
    fn update_status(&mut self) {
        self.status = match (self.affinity, self.trust) {
            (a, t) if a >= 75 && t >= 75 => RelationshipStatus::Beloved,
            (a, t) if a >= 50 && t >= 50 => RelationshipStatus::CloseFriend,
            (a, t) if a >= 25 && t >= 25 => RelationshipStatus::Friend,
            (a, _) if a >= 10 => RelationshipStatus::Acquaintance,
            (a, t) if a > -25 && t > -25 => RelationshipStatus::Stranger,
            (a, _) if a > -50 => RelationshipStatus::Disliked,
            (a, t) if a <= -75 || t <= -75 => RelationshipStatus::Nemesis,
            _ => RelationshipStatus::Enemy,
        };
    }

    /// Add a relationship event
    pub fn add_event(&mut self, event: RelationshipEvent) {
        self.events.push(event);
        self.interactions += 1;
    }
}

/// Status levels for NPC relationships
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum RelationshipStatus {
    Nemesis,
    Enemy,
    Disliked,
    Stranger,
    Acquaintance,
    Friend,
    CloseFriend,
    Beloved,
}

impl RelationshipStatus {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Nemesis => "Nemesis",
            Self::Enemy => "Enemy",
            Self::Disliked => "Disliked",
            Self::Stranger => "Stranger",
            Self::Acquaintance => "Acquaintance",
            Self::Friend => "Friend",
            Self::CloseFriend => "Close Friend",
            Self::Beloved => "Beloved",
        }
    }
}

/// An event that affected a relationship
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RelationshipEvent {
    pub description: String,
    pub affinity_change: i32,
    pub trust_change: i32,
    pub floor: u32,
    pub turn: u32,
}

// ============================================================================
// PLOT POINTS AND REVELATIONS
// ============================================================================

/// Major plot points in the story
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum PlotPoint {
    // Act 1 Plot Points
    LearnedOfShadowLord,
    DiscoveredProphecy,
    MetFirstAlly,

    // Act 2 Plot Points
    LearnedOfGatheringDarkness,
    RivalRevealed,
    FirstVision,

    // Act 3 Plot Points
    TrueNatureOfCrypts,
    BetrayalOccurred,
    SanityThreatened,

    // Act 4 Plot Points
    TrialOfFirePassed,
    SacrificeWitnessed,
    PhoenixBlessing,

    // Act 5 Plot Points
    PastRevealed,
    ConnectionToVillain,
    TruthAboutMentor,

    // Act 6 Plot Points
    NatureCorruptionSource,
    DruidicSecret,
    BalanceDiscovered,

    // Act 7 Plot Points
    DreamRealmNature,
    NightmareTruth,
    CrystalPower,

    // Act 8 Plot Points
    VeilExplained,
    ParallelWorldsExist,
    TrueIdentity,

    // Act 9 Plot Points
    GodsWar,
    DemonsOrigin,
    ChoiceOfAllegiance,

    // Act 10 Plot Points
    VillainMotivation,
    FinalTruth,
    DestinyRevealed,
}

impl PlotPoint {
    pub fn name(&self) -> &'static str {
        match self {
            Self::LearnedOfShadowLord => "Learned of the Shadow Lord",
            Self::DiscoveredProphecy => "Discovered the Prophecy",
            Self::MetFirstAlly => "Met First Ally",
            Self::LearnedOfGatheringDarkness => "Learned of Gathering Darkness",
            Self::RivalRevealed => "Rival Revealed",
            Self::FirstVision => "First Vision",
            Self::TrueNatureOfCrypts => "True Nature of Crypts",
            Self::BetrayalOccurred => "Betrayal Occurred",
            Self::SanityThreatened => "Sanity Threatened",
            Self::TrialOfFirePassed => "Trial of Fire Passed",
            Self::SacrificeWitnessed => "Sacrifice Witnessed",
            Self::PhoenixBlessing => "Phoenix Blessing",
            Self::PastRevealed => "Past Revealed",
            Self::ConnectionToVillain => "Connection to Villain",
            Self::TruthAboutMentor => "Truth About Mentor",
            Self::NatureCorruptionSource => "Nature Corruption Source",
            Self::DruidicSecret => "Druidic Secret",
            Self::BalanceDiscovered => "Balance Discovered",
            Self::DreamRealmNature => "Dream Realm Nature",
            Self::NightmareTruth => "Nightmare Truth",
            Self::CrystalPower => "Crystal Power",
            Self::VeilExplained => "Veil Explained",
            Self::ParallelWorldsExist => "Parallel Worlds Exist",
            Self::TrueIdentity => "True Identity",
            Self::GodsWar => "Gods' War",
            Self::DemonsOrigin => "Demons' Origin",
            Self::ChoiceOfAllegiance => "Choice of Allegiance",
            Self::VillainMotivation => "Villain Motivation",
            Self::FinalTruth => "Final Truth",
            Self::DestinyRevealed => "Destiny Revealed",
        }
    }

    pub fn act(&self) -> ActId {
        match self {
            Self::LearnedOfShadowLord | Self::DiscoveredProphecy | Self::MetFirstAlly =>
                ActId::Act1TheAwakening,
            Self::LearnedOfGatheringDarkness | Self::RivalRevealed | Self::FirstVision =>
                ActId::Act2TheGatheringStorm,
            Self::TrueNatureOfCrypts | Self::BetrayalOccurred | Self::SanityThreatened =>
                ActId::Act3DescentIntoDarkness,
            Self::TrialOfFirePassed | Self::SacrificeWitnessed | Self::PhoenixBlessing =>
                ActId::Act4TheBurningPath,
            Self::PastRevealed | Self::ConnectionToVillain | Self::TruthAboutMentor =>
                ActId::Act5FrozenMemories,
            Self::NatureCorruptionSource | Self::DruidicSecret | Self::BalanceDiscovered =>
                ActId::Act6NaturesWrath,
            Self::DreamRealmNature | Self::NightmareTruth | Self::CrystalPower =>
                ActId::Act7CrystalDreams,
            Self::VeilExplained | Self::ParallelWorldsExist | Self::TrueIdentity =>
                ActId::Act8BetweenWorlds,
            Self::GodsWar | Self::DemonsOrigin | Self::ChoiceOfAllegiance =>
                ActId::Act9DivineConflict,
            Self::VillainMotivation | Self::FinalTruth | Self::DestinyRevealed =>
                ActId::Act10TheFinalReckoning,
        }
    }
}

/// A story twist or revelation
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Revelation {
    pub id: String,
    pub title: String,
    pub content: String,
    pub related_plot_point: PlotPoint,
    pub discovered: bool,
    pub floor_discovered: Option<u32>,
}

// ============================================================================
// FACTION ALIGNMENT
// ============================================================================

/// Factions the player can align with
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum StoryFaction {
    AdventurersGuild,
    TempleOfLight,
    ShadowCovenant,
    NatureCircle,
    Independent,
}

impl StoryFaction {
    pub fn name(&self) -> &'static str {
        match self {
            Self::AdventurersGuild => "Adventurer's Guild",
            Self::TempleOfLight => "Temple of Light",
            Self::ShadowCovenant => "Shadow Covenant",
            Self::NatureCircle => "Nature Circle",
            Self::Independent => "Independent",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::AdventurersGuild => "Pragmatic explorers who seek treasure and glory in the crypts.",
            Self::TempleOfLight => "Holy warriors dedicated to sealing the darkness forever.",
            Self::ShadowCovenant => "Those who believe in embracing and controlling the shadow.",
            Self::NatureCircle => "Druids seeking to heal the corruption or adapt to it.",
            Self::Independent => "Beholden to no faction, forging your own path.",
        }
    }
}

/// Tracking standing with factions
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FactionStanding {
    pub faction: StoryFaction,
    pub reputation: i32,  // -100 to 100
    pub rank: FactionRank,
    pub quests_completed: u32,
    pub is_member: bool,
}

impl FactionStanding {
    pub fn new(faction: StoryFaction) -> Self {
        Self {
            faction,
            reputation: 0,
            rank: FactionRank::Outsider,
            quests_completed: 0,
            is_member: false,
        }
    }

    pub fn modify_reputation(&mut self, amount: i32) {
        self.reputation = (self.reputation + amount).clamp(-100, 100);
        self.update_rank();
    }

    fn update_rank(&mut self) {
        self.rank = if !self.is_member {
            match self.reputation {
                r if r >= 50 => FactionRank::Respected,
                r if r >= 0 => FactionRank::Neutral,
                r if r >= -50 => FactionRank::Distrusted,
                _ => FactionRank::Hostile,
            }
        } else {
            match self.reputation {
                r if r >= 90 => FactionRank::Exalted,
                r if r >= 70 => FactionRank::Champion,
                r if r >= 50 => FactionRank::Honored,
                r if r >= 25 => FactionRank::Member,
                _ => FactionRank::Initiate,
            }
        };
    }
}

/// Ranks within factions
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum FactionRank {
    // Non-member ranks
    Hostile,
    Distrusted,
    Neutral,
    Outsider,
    Respected,
    // Member ranks
    Initiate,
    Member,
    Honored,
    Champion,
    Exalted,
}

impl FactionRank {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Hostile => "Hostile",
            Self::Distrusted => "Distrusted",
            Self::Neutral => "Neutral",
            Self::Outsider => "Outsider",
            Self::Respected => "Respected",
            Self::Initiate => "Initiate",
            Self::Member => "Member",
            Self::Honored => "Honored",
            Self::Champion => "Champion",
            Self::Exalted => "Exalted",
        }
    }
}

// ============================================================================
// STORY SYSTEM
// ============================================================================

/// The main story system managing all narrative elements
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StorySystem {
    // Current progress
    pub current_act: ActId,
    pub current_floor: u32,
    pub story_progress_percentage: f32,

    // Act completion tracking
    pub completed_acts: HashSet<ActId>,
    pub act_progress: HashMap<ActId, ActProgress>,

    // Cutscene tracking
    pub viewed_cutscenes: HashSet<CutsceneId>,
    pub pending_cutscenes: Vec<CutsceneId>,
    pub current_cutscene: Option<ActiveCutscene>,

    // Choice tracking
    pub choices_made: HashMap<StoryChoiceId, StoryChoice>,
    pub pending_choices: Vec<PendingChoice>,

    // Ending tracking
    pub achieved_endings: HashSet<Ending>,
    pub current_ending_path: Option<Ending>,

    // NPC relationships
    pub npc_relationships: HashMap<StoryNPCId, NPCRelationship>,
    pub met_npcs: HashSet<StoryNPCId>,

    // Side stories
    pub side_stories: HashMap<SideStoryId, SideStory>,
    pub active_side_stories: Vec<SideStoryId>,

    // Journal
    pub journal_entries: Vec<JournalEntry>,
    pub next_journal_id: u32,

    // Lore collection
    pub lore_collection: HashMap<String, LoreCollectionEntry>,
    pub total_lore_discovered: u32,

    // Plot points and revelations
    pub discovered_plot_points: HashSet<PlotPoint>,
    pub revelations: Vec<Revelation>,

    // Faction standings
    pub faction_standings: HashMap<StoryFaction, FactionStanding>,
    pub current_faction: Option<StoryFaction>,

    // Statistics
    pub total_turns: u32,
    pub bosses_defeated: u32,
    pub npcs_befriended: u32,
    pub npcs_antagonized: u32,
}

impl Default for StorySystem {
    fn default() -> Self {
        Self::new()
    }
}

impl StorySystem {
    /// Create a new story system
    pub fn new() -> Self {
        let mut system = Self {
            current_act: ActId::Act1TheAwakening,
            current_floor: 1,
            story_progress_percentage: 0.0,

            completed_acts: HashSet::new(),
            act_progress: HashMap::new(),

            viewed_cutscenes: HashSet::new(),
            pending_cutscenes: Vec::new(),
            current_cutscene: None,

            choices_made: HashMap::new(),
            pending_choices: Vec::new(),

            achieved_endings: HashSet::new(),
            current_ending_path: None,

            npc_relationships: HashMap::new(),
            met_npcs: HashSet::new(),

            side_stories: HashMap::new(),
            active_side_stories: Vec::new(),

            journal_entries: Vec::new(),
            next_journal_id: 1,

            lore_collection: HashMap::new(),
            total_lore_discovered: 0,

            discovered_plot_points: HashSet::new(),
            revelations: Vec::new(),

            faction_standings: HashMap::new(),
            current_faction: None,

            total_turns: 0,
            bosses_defeated: 0,
            npcs_befriended: 0,
            npcs_antagonized: 0,
        };

        system.initialize();
        system
    }

    /// Initialize story system with default data
    fn initialize(&mut self) {
        // Initialize act progress
        for act in ActId::all() {
            self.act_progress.insert(act, ActProgress::new(act));
        }

        // Initialize faction standings
        for faction in [
            StoryFaction::AdventurersGuild,
            StoryFaction::TempleOfLight,
            StoryFaction::ShadowCovenant,
            StoryFaction::NatureCircle,
        ] {
            self.faction_standings.insert(faction, FactionStanding::new(faction));
        }

        // Initialize side stories
        self.initialize_side_stories();

        // Queue intro cutscene
        self.pending_cutscenes.push(CutsceneId::Intro);
    }

    /// Initialize all side stories
    fn initialize_side_stories(&mut self) {
        let side_story_ids = [
            SideStoryId::VexRedemption,
            SideStoryId::LyannaCrisis,
            SideStoryId::TheronAmbition,
            SideStoryId::SorenSecrets,
            SideStoryId::GuildConflict,
            SideStoryId::TempleSchism,
            SideStoryId::CovenantRising,
            SideStoryId::NatureBond,
            SideStoryId::LyannaDevotion,
            SideStoryId::VexTrust,
            SideStoryId::RavenRivalry,
            SideStoryId::SilasBetrayal,
            SideStoryId::FirstHeroTale,
            SideStoryId::ShadowfallOrigin,
            SideStoryId::TrueProphecy,
            SideStoryId::LostKingdom,
            SideStoryId::ForgottenGods,
        ];

        for id in side_story_ids {
            let story = SideStory {
                id,
                category: id.category(),
                name: id.name().to_string(),
                description: id.description().to_string(),
                chapters: Vec::new(),
                current_chapter: 0,
                completed: false,
                unlocked: false,
            };
            self.side_stories.insert(id, story);
        }
    }

    // ========================================================================
    // FLOOR AND ACT MANAGEMENT
    // ========================================================================

    /// Update when entering a new floor
    pub fn on_floor_enter(&mut self, floor: u32) -> Vec<StoryEvent> {
        let mut events = Vec::new();
        self.current_floor = floor;

        // Update current act
        if let Some(act) = ActId::from_floor(floor) {
            if act != self.current_act {
                events.push(StoryEvent::ActChanged {
                    from: self.current_act,
                    to: act,
                });

                // Mark previous act as complete if moving forward
                if act.number() > self.current_act.number() {
                    self.completed_acts.insert(self.current_act);
                    if let Some(progress) = self.act_progress.get_mut(&self.current_act) {
                        progress.completed = true;
                    }

                    // Queue act completion cutscene
                    let completion_cutscene = match self.current_act {
                        ActId::Act1TheAwakening => CutsceneId::Act1Complete,
                        ActId::Act2TheGatheringStorm => CutsceneId::Act2Complete,
                        ActId::Act3DescentIntoDarkness => CutsceneId::Act3Complete,
                        ActId::Act4TheBurningPath => CutsceneId::Act4Complete,
                        ActId::Act5FrozenMemories => CutsceneId::Act5Complete,
                        ActId::Act6NaturesWrath => CutsceneId::Act6Complete,
                        ActId::Act7CrystalDreams => CutsceneId::Act7Complete,
                        ActId::Act8BetweenWorlds => CutsceneId::Act8Complete,
                        ActId::Act9DivineConflict => CutsceneId::Act9Complete,
                        ActId::Act10TheFinalReckoning => CutsceneId::Act10Intro,
                    };
                    self.queue_cutscene(completion_cutscene);

                    // Queue new act intro cutscene
                    let intro_cutscene = match act {
                        ActId::Act1TheAwakening => CutsceneId::Intro,
                        ActId::Act2TheGatheringStorm => CutsceneId::Act2Intro,
                        ActId::Act3DescentIntoDarkness => CutsceneId::Act3Intro,
                        ActId::Act4TheBurningPath => CutsceneId::Act4Intro,
                        ActId::Act5FrozenMemories => CutsceneId::Act5Intro,
                        ActId::Act6NaturesWrath => CutsceneId::Act6Intro,
                        ActId::Act7CrystalDreams => CutsceneId::Act7Intro,
                        ActId::Act8BetweenWorlds => CutsceneId::Act8Intro,
                        ActId::Act9DivineConflict => CutsceneId::Act9Intro,
                        ActId::Act10TheFinalReckoning => CutsceneId::Act10Intro,
                    };
                    self.queue_cutscene(intro_cutscene);
                }

                self.current_act = act;
            }
        }

        // Update act progress
        if let Some(progress) = self.act_progress.get_mut(&self.current_act) {
            progress.floors_visited.insert(floor);
            progress.current_floor = floor;
        }

        // Update overall progress
        self.update_story_progress();

        // Check for floor-specific cutscenes
        events.extend(self.check_floor_cutscenes(floor));

        events
    }

    /// Update story progress percentage
    fn update_story_progress(&mut self) {
        let total_floors = TOTAL_ACTS as f32 * FLOORS_PER_ACT as f32;
        self.story_progress_percentage = (self.current_floor as f32 / total_floors) * 100.0;
    }

    /// Check for cutscenes triggered by entering a floor
    fn check_floor_cutscenes(&mut self, floor: u32) -> Vec<StoryEvent> {
        let mut events = Vec::new();

        // Floor 1: Intro if not seen
        if floor == 1 && !self.viewed_cutscenes.contains(&CutsceneId::Intro) {
            self.queue_cutscene(CutsceneId::Intro);
            events.push(StoryEvent::CutsceneTriggered(CutsceneId::Intro));
        }

        // Floor 5: Meet mentor
        if floor == 5 && !self.viewed_cutscenes.contains(&CutsceneId::MeetMentor) {
            self.queue_cutscene(CutsceneId::MeetMentor);
            events.push(StoryEvent::CutsceneTriggered(CutsceneId::MeetMentor));
        }

        // Floor 15: Meet rival
        if floor == 15 && !self.viewed_cutscenes.contains(&CutsceneId::MeetRival) {
            self.queue_cutscene(CutsceneId::MeetRival);
            events.push(StoryEvent::CutsceneTriggered(CutsceneId::MeetRival));
        }

        events
    }

    // ========================================================================
    // CUTSCENE MANAGEMENT
    // ========================================================================

    /// Queue a cutscene to be shown
    pub fn queue_cutscene(&mut self, cutscene_id: CutsceneId) {
        if !self.viewed_cutscenes.contains(&cutscene_id)
            && !self.pending_cutscenes.contains(&cutscene_id) {
            self.pending_cutscenes.push(cutscene_id);
        }
    }

    /// Get the next pending cutscene
    pub fn get_pending_cutscene(&mut self) -> Option<CutsceneId> {
        self.pending_cutscenes.pop()
    }

    /// Start a cutscene
    pub fn start_cutscene(&mut self, cutscene_id: CutsceneId) {
        self.current_cutscene = Some(ActiveCutscene {
            cutscene_id,
            current_scene: 0,
            choices_made: Vec::new(),
        });
    }

    /// Advance to the next scene in the current cutscene
    pub fn advance_cutscene(&mut self) -> CutsceneAdvanceResult {
        if let Some(ref mut active) = self.current_cutscene {
            active.current_scene += 1;
            // This would check against actual cutscene data
            // For now, return that it continues
            CutsceneAdvanceResult::Continue
        } else {
            CutsceneAdvanceResult::NoCutscene
        }
    }

    /// End the current cutscene
    pub fn end_cutscene(&mut self) {
        if let Some(active) = self.current_cutscene.take() {
            self.viewed_cutscenes.insert(active.cutscene_id);
        }
    }

    /// Check if a cutscene is active
    pub fn is_cutscene_active(&self) -> bool {
        self.current_cutscene.is_some()
    }

    // ========================================================================
    // CHOICE MANAGEMENT
    // ========================================================================

    /// Record a story choice
    pub fn make_choice(&mut self, choice_id: StoryChoiceId, floor: u32, turn: u32) -> Vec<StoryEvent> {
        let mut events = Vec::new();

        let choice = StoryChoice {
            id: choice_id,
            description: choice_id.description().to_string(),
            floor_made: floor,
            turn_made: turn,
            consequences: Vec::new(),
        };

        self.choices_made.insert(choice_id, choice);

        // Add journal entry
        self.add_journal_entry(
            format!("Choice: {}", choice_id.description()),
            format!("I made an important decision: {}", choice_id.description()),
            floor,
            turn,
            JournalCategory::ImportantChoice,
            None,
            Some(self.current_act),
        );

        events.push(StoryEvent::ChoiceMade(choice_id));

        // Update ending path based on choice
        self.update_ending_path();

        // Check for faction-related choices
        match choice_id {
            StoryChoiceId::JoinAdventurersGuild => {
                self.join_faction(StoryFaction::AdventurersGuild);
                events.push(StoryEvent::FactionJoined(StoryFaction::AdventurersGuild));
            }
            StoryChoiceId::JoinTempleOfLight => {
                self.join_faction(StoryFaction::TempleOfLight);
                events.push(StoryEvent::FactionJoined(StoryFaction::TempleOfLight));
            }
            StoryChoiceId::JoinShadowCovenant => {
                self.join_faction(StoryFaction::ShadowCovenant);
                events.push(StoryEvent::FactionJoined(StoryFaction::ShadowCovenant));
            }
            StoryChoiceId::RemainIndependent => {
                self.current_faction = Some(StoryFaction::Independent);
            }
            _ => {}
        }

        events
    }

    /// Check if a choice has been made
    pub fn has_made_choice(&self, choice_id: StoryChoiceId) -> bool {
        self.choices_made.contains_key(&choice_id)
    }

    /// Update the ending path based on choices made
    fn update_ending_path(&mut self) {
        // Calculate ending based on choices
        let has_darkness = self.has_made_choice(StoryChoiceId::EmbraceDarkness)
            || self.has_made_choice(StoryChoiceId::AcceptShadowPower)
            || self.has_made_choice(StoryChoiceId::SideInfernal);

        let has_light = self.has_made_choice(StoryChoiceId::SideDivine)
            || self.choices_made.contains_key(&StoryChoiceId::ForgiveBetrayer);

        let has_balance = self.has_made_choice(StoryChoiceId::HealNature)
            || self.has_made_choice(StoryChoiceId::RejectBothPowers);

        let has_independence = self.has_made_choice(StoryChoiceId::ForgeOwnPath)
            || self.has_made_choice(StoryChoiceId::RemainIndependent);

        self.current_ending_path = if has_independence && has_balance {
            Some(Ending::Ascension)
        } else if has_darkness && !has_light {
            Some(Ending::Damnation)
        } else if has_light && !has_darkness {
            Some(Ending::Redemption)
        } else if has_balance {
            Some(Ending::Balance)
        } else {
            None
        };
    }

    // ========================================================================
    // NPC RELATIONSHIP MANAGEMENT
    // ========================================================================

    /// Meet an NPC for the first time
    pub fn meet_npc(&mut self, npc_id: StoryNPCId) -> Vec<StoryEvent> {
        let mut events = Vec::new();

        if self.met_npcs.insert(npc_id) {
            // First meeting
            self.npc_relationships.insert(npc_id, NPCRelationship::new(npc_id));
            events.push(StoryEvent::NPCMet(npc_id));

            // Add journal entry
            self.add_journal_entry(
                format!("Met {}", npc_id.name()),
                format!("I encountered {} - {}.", npc_id.name(), npc_id.description()),
                self.current_floor,
                self.total_turns,
                JournalCategory::NPCEncounter,
                Some(npc_id),
                Some(self.current_act),
            );
        }

        events
    }

    /// Modify relationship with an NPC
    pub fn modify_npc_relationship(
        &mut self,
        npc_id: StoryNPCId,
        affinity_change: i32,
        trust_change: i32,
        event_description: &str,
    ) -> Vec<StoryEvent> {
        let mut events = Vec::new();

        // Ensure NPC is met
        if !self.met_npcs.contains(&npc_id) {
            self.meet_npc(npc_id);
        }

        if let Some(relationship) = self.npc_relationships.get_mut(&npc_id) {
            let old_status = relationship.status;

            relationship.modify_affinity(affinity_change);
            relationship.modify_trust(trust_change);

            relationship.add_event(RelationshipEvent {
                description: event_description.to_string(),
                affinity_change,
                trust_change,
                floor: self.current_floor,
                turn: self.total_turns,
            });

            let new_status = relationship.status;

            if new_status != old_status {
                events.push(StoryEvent::RelationshipChanged {
                    npc: npc_id,
                    old_status,
                    new_status,
                });

                // Track befriended/antagonized
                if matches!(new_status, RelationshipStatus::Friend | RelationshipStatus::CloseFriend | RelationshipStatus::Beloved)
                    && !matches!(old_status, RelationshipStatus::Friend | RelationshipStatus::CloseFriend | RelationshipStatus::Beloved)
                {
                    self.npcs_befriended += 1;
                }
                if matches!(new_status, RelationshipStatus::Enemy | RelationshipStatus::Nemesis)
                    && !matches!(old_status, RelationshipStatus::Enemy | RelationshipStatus::Nemesis)
                {
                    self.npcs_antagonized += 1;
                }
            }
        }

        events
    }

    /// Get relationship status with an NPC
    pub fn get_npc_relationship(&self, npc_id: StoryNPCId) -> Option<&NPCRelationship> {
        self.npc_relationships.get(&npc_id)
    }

    // ========================================================================
    // FACTION MANAGEMENT
    // ========================================================================

    /// Join a faction
    pub fn join_faction(&mut self, faction: StoryFaction) {
        // Leave current faction if any
        if let Some(current) = self.current_faction {
            if current != StoryFaction::Independent {
                if let Some(standing) = self.faction_standings.get_mut(&current) {
                    standing.is_member = false;
                    standing.modify_reputation(-20); // Reputation hit for leaving
                }
            }
        }

        self.current_faction = Some(faction);

        if faction != StoryFaction::Independent {
            if let Some(standing) = self.faction_standings.get_mut(&faction) {
                standing.is_member = true;
                standing.rank = FactionRank::Initiate;
                if standing.reputation < 25 {
                    standing.reputation = 25;
                }
            }
        }
    }

    /// Modify faction reputation
    pub fn modify_faction_reputation(&mut self, faction: StoryFaction, amount: i32) {
        if let Some(standing) = self.faction_standings.get_mut(&faction) {
            standing.modify_reputation(amount);
        }
    }

    // ========================================================================
    // SIDE STORY MANAGEMENT
    // ========================================================================

    /// Unlock a side story
    pub fn unlock_side_story(&mut self, story_id: SideStoryId) -> bool {
        if let Some(story) = self.side_stories.get_mut(&story_id) {
            if !story.unlocked {
                story.unlocked = true;
                return true;
            }
        }
        false
    }

    /// Start a side story
    pub fn start_side_story(&mut self, story_id: SideStoryId) -> bool {
        if let Some(story) = self.side_stories.get(&story_id) {
            if story.unlocked && !story.completed && !self.active_side_stories.contains(&story_id) {
                self.active_side_stories.push(story_id);
                return true;
            }
        }
        false
    }

    /// Complete a side story chapter
    pub fn complete_side_story_chapter(&mut self, story_id: SideStoryId) -> bool {
        if let Some(story) = self.side_stories.get_mut(&story_id) {
            if !story.chapters.is_empty() && story.current_chapter < story.chapters.len() {
                story.chapters[story.current_chapter].completed = true;
                story.current_chapter += 1;

                // Check if story is complete
                if story.current_chapter >= story.chapters.len() {
                    story.completed = true;
                    self.active_side_stories.retain(|&id| id != story_id);
                }
                return true;
            }
        }
        false
    }

    // ========================================================================
    // JOURNAL MANAGEMENT
    // ========================================================================

    /// Add a journal entry
    pub fn add_journal_entry(
        &mut self,
        title: impl Into<String>,
        content: impl Into<String>,
        floor: u32,
        turn: u32,
        category: JournalCategory,
        related_npc: Option<StoryNPCId>,
        related_act: Option<ActId>,
    ) {
        let entry = JournalEntry {
            id: self.next_journal_id,
            title: title.into(),
            content: content.into(),
            floor,
            turn,
            category,
            related_npc,
            related_act,
        };

        self.journal_entries.push(entry);
        self.next_journal_id += 1;
    }

    /// Get journal entries by category
    pub fn get_journal_entries(&self, category: Option<JournalCategory>) -> Vec<&JournalEntry> {
        match category {
            Some(cat) => self.journal_entries.iter().filter(|e| e.category == cat).collect(),
            None => self.journal_entries.iter().collect(),
        }
    }

    // ========================================================================
    // LORE COLLECTION
    // ========================================================================

    /// Discover a lore entry
    pub fn discover_lore(&mut self, id: impl Into<String>, title: impl Into<String>, content: impl Into<String>, category: LoreCollectionCategory) -> bool {
        let id_str = id.into();
        if !self.lore_collection.contains_key(&id_str) {
            let entry = LoreCollectionEntry {
                id: id_str.clone(),
                title: title.into(),
                content: content.into(),
                category,
                discovered: true,
                floor_discovered: Some(self.current_floor),
            };
            self.lore_collection.insert(id_str, entry);
            self.total_lore_discovered += 1;
            true
        } else {
            false
        }
    }

    /// Get lore entries by category
    pub fn get_lore_by_category(&self, category: LoreCollectionCategory) -> Vec<&LoreCollectionEntry> {
        self.lore_collection.values().filter(|e| e.category == category && e.discovered).collect()
    }

    /// Get lore completion percentage
    pub fn get_lore_completion(&self) -> f32 {
        // Assume there are 100 total lore entries
        const TOTAL_LORE: f32 = 100.0;
        (self.total_lore_discovered as f32 / TOTAL_LORE) * 100.0
    }

    // ========================================================================
    // PLOT POINTS AND REVELATIONS
    // ========================================================================

    /// Discover a plot point
    pub fn discover_plot_point(&mut self, plot_point: PlotPoint) -> Vec<StoryEvent> {
        let mut events = Vec::new();

        if self.discovered_plot_points.insert(plot_point) {
            events.push(StoryEvent::PlotPointDiscovered(plot_point));

            // Add journal entry
            self.add_journal_entry(
                format!("Discovery: {}", plot_point.name()),
                format!("A major story revelation: {}", plot_point.name()),
                self.current_floor,
                self.total_turns,
                JournalCategory::MainStory,
                None,
                Some(plot_point.act()),
            );
        }

        events
    }

    /// Add a revelation
    pub fn add_revelation(&mut self, id: impl Into<String>, title: impl Into<String>, content: impl Into<String>, plot_point: PlotPoint) {
        let revelation = Revelation {
            id: id.into(),
            title: title.into(),
            content: content.into(),
            related_plot_point: plot_point,
            discovered: true,
            floor_discovered: Some(self.current_floor),
        };
        self.revelations.push(revelation);
        self.discover_plot_point(plot_point);
    }

    // ========================================================================
    // BOSS AND EVENT TRACKING
    // ========================================================================

    /// Record defeating a boss
    pub fn on_boss_defeated(&mut self, floor: u32) -> Vec<StoryEvent> {
        let mut events = Vec::new();
        self.bosses_defeated += 1;

        // Update act progress
        if let Some(act) = ActId::from_floor(floor) {
            if let Some(progress) = self.act_progress.get_mut(&act) {
                progress.bosses_defeated += 1;
            }
        }

        // Trigger boss cutscenes based on floor
        let cutscene = match floor {
            10 => Some(CutsceneId::FirstBoss),
            20 => Some(CutsceneId::Act2Complete),
            30 => Some(CutsceneId::Act3Complete),
            40 => Some(CutsceneId::PhoenixRising),
            50 => Some(CutsceneId::TruthRevealed),
            60 => Some(CutsceneId::GuardianAwakens),
            70 => Some(CutsceneId::CrystalHeart),
            80 => Some(CutsceneId::WorldsMerge),
            90 => Some(CutsceneId::GodsChoice),
            100 => Some(CutsceneId::VillainConfrontation),
            _ => None,
        };

        if let Some(cutscene_id) = cutscene {
            self.queue_cutscene(cutscene_id);
            events.push(StoryEvent::CutsceneTriggered(cutscene_id));
        }

        events
    }

    /// Tick for turn-based updates
    pub fn on_turn(&mut self) {
        self.total_turns += 1;
    }

    // ========================================================================
    // ENDING MANAGEMENT
    // ========================================================================

    /// Check if requirements for an ending are met
    pub fn check_ending_requirements(&self, ending: Ending) -> bool {
        match ending {
            Ending::Redemption => {
                self.has_made_choice(StoryChoiceId::RedeemVillain)
                    && self.has_made_choice(StoryChoiceId::ForgiveBetrayer)
            }
            Ending::Damnation => {
                self.has_made_choice(StoryChoiceId::BecomeNewVillain)
                    || (self.has_made_choice(StoryChoiceId::EmbraceDarkness)
                        && self.has_made_choice(StoryChoiceId::SideInfernal))
            }
            Ending::Ascension => {
                self.has_made_choice(StoryChoiceId::TranscendMortality)
                    || (self.has_made_choice(StoryChoiceId::ForgeOwnPath)
                        && self.has_made_choice(StoryChoiceId::RejectBothPowers))
            }
            Ending::Balance => {
                self.has_made_choice(StoryChoiceId::HealNature)
                    && !self.has_made_choice(StoryChoiceId::EmbraceDarkness)
                    && !self.has_made_choice(StoryChoiceId::SideInfernal)
            }
            Ending::TrueEnding => {
                self.total_lore_discovered >= 90
                    && self.has_made_choice(StoryChoiceId::UnlockAllMemories)
            }
            Ending::SecretEnding => {
                self.total_lore_discovered >= 100
                    && self.has_made_choice(StoryChoiceId::UnlockAllMemories)
                    && self.discovered_plot_points.contains(&PlotPoint::TrueIdentity)
                    && self.has_made_choice(StoryChoiceId::MasterDreams)
            }
        }
    }

    /// Achieve an ending
    pub fn achieve_ending(&mut self, ending: Ending) -> Vec<StoryEvent> {
        let mut events = Vec::new();

        if self.achieved_endings.insert(ending) {
            events.push(StoryEvent::EndingAchieved(ending));

            // Queue ending cutscene
            let cutscene = match ending {
                Ending::Redemption => CutsceneId::EndingRedemption,
                Ending::Damnation => CutsceneId::EndingDamnation,
                Ending::Ascension => CutsceneId::EndingAscension,
                Ending::Balance => CutsceneId::EndingBalance,
                Ending::TrueEnding => CutsceneId::EndingTrueEnding,
                Ending::SecretEnding => CutsceneId::EndingSecretEnding,
            };
            self.queue_cutscene(cutscene);
        }

        events
    }

    /// Get all achieved endings
    pub fn get_achieved_endings(&self) -> Vec<Ending> {
        self.achieved_endings.iter().copied().collect()
    }

    // ========================================================================
    // UTILITY METHODS
    // ========================================================================

    /// Get current act information
    pub fn get_current_act_info(&self) -> ActInfo {
        let (floor_start, floor_end) = self.current_act.floor_range();
        let progress = self.act_progress.get(&self.current_act);

        ActInfo {
            act: self.current_act,
            name: self.current_act.name().to_string(),
            description: self.current_act.description().to_string(),
            floor_start,
            floor_end,
            current_floor: self.current_floor,
            completion_percentage: progress.map_or(0.0, |p| p.completion_percentage()),
            bosses_defeated: progress.map_or(0, |p| p.bosses_defeated),
        }
    }

    /// Get story summary for display
    pub fn get_story_summary(&self) -> StorySummary {
        StorySummary {
            current_act: self.current_act,
            current_floor: self.current_floor,
            story_progress: self.story_progress_percentage,
            acts_completed: self.completed_acts.len(),
            cutscenes_viewed: self.viewed_cutscenes.len(),
            choices_made: self.choices_made.len(),
            npcs_met: self.met_npcs.len(),
            lore_discovered: self.total_lore_discovered,
            side_stories_active: self.active_side_stories.len(),
            current_ending_path: self.current_ending_path,
            current_faction: self.current_faction,
        }
    }

    /// Check if the player has completed the game
    pub fn is_game_complete(&self) -> bool {
        !self.achieved_endings.is_empty()
    }
}

// ============================================================================
// SUPPORTING STRUCTURES
// ============================================================================

/// Progress tracking for an act
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ActProgress {
    pub act: ActId,
    pub floors_visited: HashSet<u32>,
    pub current_floor: u32,
    pub bosses_defeated: u32,
    pub cutscenes_viewed: Vec<CutsceneId>,
    pub choices_made: Vec<StoryChoiceId>,
    pub completed: bool,
}

impl ActProgress {
    pub fn new(act: ActId) -> Self {
        Self {
            act,
            floors_visited: HashSet::new(),
            current_floor: 0,
            bosses_defeated: 0,
            cutscenes_viewed: Vec::new(),
            choices_made: Vec::new(),
            completed: false,
        }
    }

    pub fn completion_percentage(&self) -> f32 {
        let (start, end) = self.act.floor_range();
        let total_floors = end - start + 1;
        (self.floors_visited.len() as f32 / total_floors as f32) * 100.0
    }
}

/// Information about the current act
#[derive(Clone, Debug)]
pub struct ActInfo {
    pub act: ActId,
    pub name: String,
    pub description: String,
    pub floor_start: u32,
    pub floor_end: u32,
    pub current_floor: u32,
    pub completion_percentage: f32,
    pub bosses_defeated: u32,
}

/// Summary of story progress
#[derive(Clone, Debug)]
pub struct StorySummary {
    pub current_act: ActId,
    pub current_floor: u32,
    pub story_progress: f32,
    pub acts_completed: usize,
    pub cutscenes_viewed: usize,
    pub choices_made: usize,
    pub npcs_met: usize,
    pub lore_discovered: u32,
    pub side_stories_active: usize,
    pub current_ending_path: Option<Ending>,
    pub current_faction: Option<StoryFaction>,
}

/// Active cutscene state
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ActiveCutscene {
    pub cutscene_id: CutsceneId,
    pub current_scene: usize,
    pub choices_made: Vec<usize>,
}

/// Result of advancing a cutscene
#[derive(Clone, Debug)]
pub enum CutsceneAdvanceResult {
    Continue,
    ChoiceRequired(CutsceneChoice),
    Complete,
    NoCutscene,
}

/// A pending choice to be presented to the player
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PendingChoice {
    pub prompt: String,
    pub options: Vec<(String, StoryChoiceId)>,
    pub floor: u32,
    pub turn: u32,
}

/// Events generated by the story system
#[derive(Clone, Debug)]
pub enum StoryEvent {
    ActChanged { from: ActId, to: ActId },
    CutsceneTriggered(CutsceneId),
    ChoiceMade(StoryChoiceId),
    NPCMet(StoryNPCId),
    RelationshipChanged {
        npc: StoryNPCId,
        old_status: RelationshipStatus,
        new_status: RelationshipStatus,
    },
    FactionJoined(StoryFaction),
    PlotPointDiscovered(PlotPoint),
    EndingAchieved(Ending),
    SideStoryUnlocked(SideStoryId),
    SideStoryCompleted(SideStoryId),
    LoreDiscovered(String),
}

// ============================================================================
// CUTSCENE DATA
// ============================================================================

/// Get cutscene data by ID
pub fn get_cutscene_data(id: CutsceneId) -> Cutscene {
    match id {
        CutsceneId::Intro => Cutscene {
            id,
            title: "The Awakening".to_string(),
            act: Some(ActId::Act1TheAwakening),
            scenes: vec![
                CutsceneScene {
                    speaker: None,
                    text: "Darkness. Cold stone beneath your fingers. The taste of dust and something metallic in your mouth.".to_string(),
                    choice: None,
                },
                CutsceneScene {
                    speaker: None,
                    text: "You don't remember how you got here. You don't remember much of anything. Just fragments - a name that might be yours, a face that might have been important.".to_string(),
                    choice: None,
                },
                CutsceneScene {
                    speaker: None,
                    text: "The Shadow Crypts. The name surfaces unbidden. A place of legend, where countless adventurers have entered... and few have returned.".to_string(),
                    choice: None,
                },
                CutsceneScene {
                    speaker: None,
                    text: "Above you, impossibly distant, a single point of light. The exit? Or just another illusion in this place of shadows?".to_string(),
                    choice: Some(CutsceneChoice {
                        prompt: "What drives you forward?".to_string(),
                        options: vec![
                            CutsceneOption {
                                text: "I must discover who I am.".to_string(),
                                story_choice: None,
                                consequence_text: "The need to know burns within you. Every answer lies deeper.".to_string(),
                            },
                            CutsceneOption {
                                text: "I sense a great evil here that must be stopped.".to_string(),
                                story_choice: None,
                                consequence_text: "A righteous fury fills your heart. The darkness will not prevail.".to_string(),
                            },
                            CutsceneOption {
                                text: "I will conquer these crypts and claim their power.".to_string(),
                                story_choice: Some(StoryChoiceId::AcceptShadowPower),
                                consequence_text: "Ambition courses through your veins. Power awaits the worthy.".to_string(),
                            },
                        ],
                    }),
                },
                CutsceneScene {
                    speaker: None,
                    text: "You rise. The journey of a hundred floors begins with a single step. The Shadow Crypts await.".to_string(),
                    choice: None,
                },
            ],
            triggers: vec![CutsceneTrigger::EnterFloor(1)],
            requirements: vec![],
        },

        CutsceneId::MeetMentor => Cutscene {
            id,
            title: "The Elder's Wisdom".to_string(),
            act: Some(ActId::Act1TheAwakening),
            scenes: vec![
                CutsceneScene {
                    speaker: None,
                    text: "A soft glow emanates from an alcove ahead, warm and inviting amidst the cold shadows.".to_string(),
                    choice: None,
                },
                CutsceneScene {
                    speaker: Some(StoryNPCId::ElderSoren),
                    text: "Ah, another soul drawn into the crypts' embrace. Come, sit with me a moment. The shadows are patient; we can spare a few words.".to_string(),
                    choice: None,
                },
                CutsceneScene {
                    speaker: Some(StoryNPCId::ElderSoren),
                    text: "I am Soren, once a seeker of truths like yourself. Now I remain here, a keeper of what knowledge the darkness has not yet consumed.".to_string(),
                    choice: None,
                },
                CutsceneScene {
                    speaker: Some(StoryNPCId::ElderSoren),
                    text: "These crypts... they are not merely a dungeon. They are a wound in the world itself, and something ancient festers within.".to_string(),
                    choice: Some(CutsceneChoice {
                        prompt: "How do you respond to Soren's words?".to_string(),
                        options: vec![
                            CutsceneOption {
                                text: "I trust you, Elder. What must I do?".to_string(),
                                story_choice: Some(StoryChoiceId::TrustMentor),
                                consequence_text: "The old man smiles warmly. 'Trust is rare down here. I will help you as I can.'".to_string(),
                            },
                            CutsceneOption {
                                text: "I'll find my own path. But thank you for the warning.".to_string(),
                                story_choice: None,
                                consequence_text: "Soren nods slowly. 'Independence has served many well. May it serve you too.'".to_string(),
                            },
                            CutsceneOption {
                                text: "If there's power here, I intend to claim it.".to_string(),
                                story_choice: Some(StoryChoiceId::AcceptShadowPower),
                                consequence_text: "A shadow passes over Soren's face. 'Many have said the same. The crypts always extract their price.'".to_string(),
                            },
                        ],
                    }),
                },
                CutsceneScene {
                    speaker: Some(StoryNPCId::ElderSoren),
                    text: "Remember this: at the heart of these crypts dwells the Shadow Lord, Malachar. Once, he was like you - a hero who delved too deep and found something that should have remained buried.".to_string(),
                    choice: None,
                },
                CutsceneScene {
                    speaker: Some(StoryNPCId::ElderSoren),
                    text: "Go now. We shall meet again, I think. The crypts have a way of bringing paths together.".to_string(),
                    choice: None,
                },
            ],
            triggers: vec![CutsceneTrigger::EnterFloor(5)],
            requirements: vec![],
        },

        CutsceneId::FirstBoss => Cutscene {
            id,
            title: "The Goblin King's Challenge".to_string(),
            act: Some(ActId::Act1TheAwakening),
            scenes: vec![
                CutsceneScene {
                    speaker: None,
                    text: "A throne of bones and rusted weapons rises before you. Upon it sits the Goblin King, crown askew, eyes gleaming with cunning malice.".to_string(),
                    choice: None,
                },
                CutsceneScene {
                    speaker: Some(StoryNPCId::GoblinKingGrix),
                    text: "So! The surfacer thinks to pass through Grix's domain! Many have tried. Their bones make up my throne!".to_string(),
                    choice: None,
                },
                CutsceneScene {
                    speaker: Some(StoryNPCId::GoblinKingGrix),
                    text: "But Grix is not unreasonable king. Grix knows the deeper dark holds worse things than goblins. Perhaps... we make deal?".to_string(),
                    choice: Some(CutsceneChoice {
                        prompt: "How do you respond to the Goblin King's offer?".to_string(),
                        options: vec![
                            CutsceneOption {
                                text: "What kind of deal do you propose?".to_string(),
                                story_choice: None,
                                consequence_text: "The Goblin King grins, revealing rows of sharp teeth. 'Smart surfacer. Grix likes that.'".to_string(),
                            },
                            CutsceneOption {
                                text: "I don't bargain with monsters. Prepare yourself!".to_string(),
                                story_choice: None,
                                consequence_text: "The Goblin King's eyes narrow. 'Then surfacer dies like all the rest!' The goblins surge forward.".to_string(),
                            },
                        ],
                    }),
                },
            ],
            triggers: vec![CutsceneTrigger::EnterFloor(10)],
            requirements: vec![],
        },

        // Default cutscene for unimplemented ones
        _ => Cutscene {
            id,
            title: format!("Cutscene: {:?}", id),
            act: id.act(),
            scenes: vec![
                CutsceneScene {
                    speaker: None,
                    text: "This cutscene's content is yet to be revealed...".to_string(),
                    choice: None,
                },
            ],
            triggers: vec![],
            requirements: vec![],
        },
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_story_system_creation() {
        let system = StorySystem::new();
        assert_eq!(system.current_act, ActId::Act1TheAwakening);
        assert_eq!(system.current_floor, 1);
        assert!(system.pending_cutscenes.contains(&CutsceneId::Intro));
    }

    #[test]
    fn test_act_from_floor() {
        assert_eq!(ActId::from_floor(1), Some(ActId::Act1TheAwakening));
        assert_eq!(ActId::from_floor(10), Some(ActId::Act1TheAwakening));
        assert_eq!(ActId::from_floor(11), Some(ActId::Act2TheGatheringStorm));
        assert_eq!(ActId::from_floor(50), Some(ActId::Act5FrozenMemories));
        assert_eq!(ActId::from_floor(100), Some(ActId::Act10TheFinalReckoning));
        assert_eq!(ActId::from_floor(101), None);
    }

    #[test]
    fn test_floor_enter() {
        let mut system = StorySystem::new();
        let events = system.on_floor_enter(11);

        assert_eq!(system.current_act, ActId::Act2TheGatheringStorm);
        assert!(system.completed_acts.contains(&ActId::Act1TheAwakening));
        assert!(!events.is_empty());
    }

    #[test]
    fn test_npc_relationship() {
        let mut system = StorySystem::new();
        system.meet_npc(StoryNPCId::ElderSoren);

        assert!(system.met_npcs.contains(&StoryNPCId::ElderSoren));

        system.modify_npc_relationship(
            StoryNPCId::ElderSoren,
            50,
            50,
            "Helped the elder",
        );

        let relationship = system.get_npc_relationship(StoryNPCId::ElderSoren).unwrap();
        assert_eq!(relationship.status, RelationshipStatus::Friend);
    }

    #[test]
    fn test_story_choices() {
        let mut system = StorySystem::new();
        system.make_choice(StoryChoiceId::TrustMentor, 5, 100);

        assert!(system.has_made_choice(StoryChoiceId::TrustMentor));
        assert!(!system.has_made_choice(StoryChoiceId::AcceptShadowPower));
    }

    #[test]
    fn test_faction_joining() {
        let mut system = StorySystem::new();
        system.join_faction(StoryFaction::TempleOfLight);

        assert_eq!(system.current_faction, Some(StoryFaction::TempleOfLight));
        let standing = system.faction_standings.get(&StoryFaction::TempleOfLight).unwrap();
        assert!(standing.is_member);
        assert_eq!(standing.rank, FactionRank::Initiate);
    }

    #[test]
    fn test_ending_path() {
        let mut system = StorySystem::new();

        // Make light-aligned choices
        system.make_choice(StoryChoiceId::ForgiveBetrayer, 30, 500);
        system.make_choice(StoryChoiceId::SideDivine, 90, 1500);

        // Should trend toward redemption
        assert_eq!(system.current_ending_path, Some(Ending::Redemption));
    }

    #[test]
    fn test_journal_entries() {
        let mut system = StorySystem::new();
        system.add_journal_entry(
            "Test Entry",
            "This is test content",
            1,
            10,
            JournalCategory::PersonalThought,
            None,
            None,
        );

        let entries = system.get_journal_entries(Some(JournalCategory::PersonalThought));
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].title, "Test Entry");
    }

    #[test]
    fn test_cutscene_data() {
        let intro = get_cutscene_data(CutsceneId::Intro);
        assert_eq!(intro.title, "The Awakening");
        assert!(!intro.scenes.is_empty());
    }
}
