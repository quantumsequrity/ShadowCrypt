//! ShadowCrypt Core Library
//!
//! This library contains all the core game logic for the ShadowCrypt roguelike,
//! designed to be shared between CLI and GUI frontends.
//!
//! # Modules
//!
//! - `classes` - Character classes and their properties
//! - `magic` - Skills and spells
//! - `combat` - Status effects and combat calculations
//! - `items` - Items, equipment, and inventory
//! - `entities` - Player and enemy entities
//! - `world` - Map, tiles, and dungeon generation
//! - `ai` - AI decision making for auto-play
//! - `companions` - Companion/pet system with AI, leveling, and abilities
//! - `challenges` - Challenge modes and gameplay modifiers (hardcore, speedrun, pacifist, cursed)
//! - `game` - Main game state and logic
//! - `save` - Save/load functionality
//! - `time` - Day/night cycle system with time-based events
//! - `achievements` - Achievement system and tracking
//! - `runes` - Rune system with socketing, rune words, and crafting
//! - `corruption` - Corruption/sanity system with visual distortions and madness effects
//! - `gambling` - Gambling system with NPCs, mystery items, and risk/reward mechanics
//! - `minigames` - Interactive minigames (lockpicking, fishing, mining, cooking, etc.) with rewards and tournaments
//!
//! # Example Usage
//!
//! ```rust,no_run
//! use shadowcrypt_core::prelude::*;
//!
//! // Create a new game with a Warrior character
//! let mut game = GameState::new(CharacterClass::Warrior);
//!
//! // Move the player
//! game.move_player(1, 0);
//!
//! // Use a skill
//! game.use_skill();
//!
//! // Check game state
//! if game.victory {
//!     println!("You won!");
//! }
//! ```

pub mod classes;
pub mod magic;
pub mod combat;
pub mod items;
pub mod entities;
pub mod world;
pub mod weather;
pub mod armor;
pub mod ai;
pub mod companions;
pub mod challenges;
pub mod enchantments;
pub mod game;
pub mod save;
pub mod quests;
pub mod time;
pub mod achievements;
pub mod runes;
pub mod crafting;
pub mod talents;
pub mod gambling;
pub mod species;
pub mod magic_schools;
pub mod kingdoms;
pub mod guilds;
pub mod party;
pub mod portals;
pub mod tower;
pub mod npcs;
pub mod stealth;
pub mod brewing;
pub mod leaderboard;
pub mod lore;
pub mod mythology;
pub mod cultivation;
pub mod weapon_masters;
pub mod necromancy;
pub mod martial_arts;
pub mod summoning;
pub mod alchemy;
pub mod formations;
pub mod treasures;
pub mod arena;
pub mod trading;
pub mod world_events;
pub mod mounts;
pub mod professions;
pub mod factions;
pub mod skill_trees;
pub mod bosses;
pub mod familiars;
pub mod relationships;
pub mod dungeon_gen;
pub mod inventory;
pub mod dialogue;
pub mod minigames;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::classes::CharacterClass;
    pub use crate::magic::Skill;
    pub use crate::combat::StatusEffect;
    pub use crate::items::{Item, ItemKind, Rarity, EquipSlot};
    pub use crate::entities::{Player, Enemy, EnemyKind};
    pub use crate::world::{Map, Tile, DungeonTheme, Room, SpecialRoomType};
    pub use crate::world::{MAP_WIDTH, MAP_HEIGHT, VIEW_RADIUS, MAX_DUNGEON_LEVEL, BOSS_LEVELS, MINI_BOSS_LEVELS};
    pub use crate::world::{FloorType, FloorModifiers, SpecialMechanic, SecretFloor};
    pub use crate::world::{get_floor_info, secret_floor_available, get_secret_floor_index};
    pub use crate::weather::{WeatherSystem, WeatherType, WeatherIntensity, WeatherCombatModifiers, DamageType};
    pub use crate::ai::{AIAction, AIDecider};
    pub use crate::companions::{
        Companion, CompanionKind, CompanionRarity, CompanionAbility,
        CompanionBehavior, CompanionMorale, CompanionAction, CompanionAI,
        CompanionEncounter, MAX_COMPANIONS,
    };
    pub use crate::challenges::{
        ChallengeMode, ChallengeConfig, ChallengeState, ChallengeEvent,
        GameModifiers, CurseType, ActiveCurse, ChallengeSummary,
        HardcoreData, SpeedrunData, PacifistData,
    };
    pub use crate::game::{GameState, GameMessage};
    pub use crate::save::{
        save_game, load_game, save_exists, delete_save,
        SaveManager, SaveSlotType, SaveMetadata, SaveData, SaveError,
        GameSettings, WorldState, NewGamePlusData, PauseState, PauseMenuOption,
        MAX_SAVE_SLOTS, DEFAULT_AUTOSAVE_INTERVAL,
    };
    pub use crate::quests::{Quest, QuestTracker, QuestReward, QuestCategory, QuestState, ObjectiveType};
    pub use crate::time::{
        TimeSystem, TimePhase, TimeEvent, TimeModifiers, TimeEffect,
        CreatureActivity, MINUTES_PER_TURN, MINUTES_PER_DAY,
        BASE_VIEW_RADIUS, MIN_VIEW_RADIUS,
    };
    pub use crate::achievements::{
        Achievement, AchievementId, AchievementCategory, AchievementTracker,
        AchievementReward, AchievementStats, RunStats, StatType,
    };
    pub use crate::runes::{
        RuneType, Rune, RuneEffect, RuneWordType, SocketedItem,
        RuneInventory, RuneStatBonuses, RuneCrafter, CraftingMaterial,
    };
    pub use crate::gambling::{
        GamblerKind, GamblerRarity, Gambler, GamblingGame, GambleOutcome,
        GamblingStats, GamblingDen, GamblingEvent, MysteryBoxTier, RiskTier,
        MAX_WAGER, MIN_WAGER, STREAK_BONUS_THRESHOLD,
        generate_gambling_encounters,
    };
    pub use crate::minigames::{
        MinigameType, MinigameSystem, MinigameState, MinigameResult, MinigameReward,
        MinigameContext, MinigameSettings, MinigameStats, MinigameSkill,
        DifficultyLevel, InputMechanic, MinigameInput, InputResult,
        Tournament, TournamentParticipant, TournamentPrize, TrophyType,
        MinigameAchievementId, AchievementReward as MinigameAchievementReward,
        MinigameTrigger, FailureConsequence, WaterType, OreType,
        MAX_DIFFICULTY, PERFECT_XP_MULTIPLIER, TOURNAMENT_ROUNDS,
    };
    pub use crate::summoning::{
        SummoningSystem, SummonedEntity, SummonType, SummonTier, SummonCategory,
        SummonerRank, SummonContract, ContractType, ContractTerm,
        SummoningRitual, CircleType, SummoningMaterial, MaterialRarity,
        SummonSkill, SummoningFailure, SummonBonuses, SummoningSummary,
        SummonAlignment, MAX_ACTIVE_SUMMONS, MAX_SUMMON_LEVEL,
        BASE_SUMMON_MANA_COST, TEMPORARY_SUMMON_BASE_DURATION,
    };
}

// Re-export commonly used types at crate root
pub use game::GameState;
pub use classes::CharacterClass;
pub use magic::Skill;
pub use combat::StatusEffect;
pub use items::{Item, ItemKind, Rarity, EquipSlot};
pub use entities::{Player, Enemy, EnemyKind};
pub use world::{Map, Tile, DungeonTheme, Room, SpecialRoomType};
pub use world::{FloorType, FloorModifiers, SpecialMechanic, SecretFloor};
pub use world::{MINI_BOSS_LEVELS, get_floor_info, secret_floor_available, get_secret_floor_index};
pub use weather::{WeatherSystem, WeatherType, WeatherIntensity, WeatherCombatModifiers, DamageType};
pub use ai::{AIAction, AIDecider};
pub use companions::{
    Companion, CompanionKind, CompanionRarity, CompanionAbility,
    CompanionBehavior, CompanionMorale, CompanionAction, CompanionAI,
    CompanionEncounter, MAX_COMPANIONS,
};
pub use challenges::{
    ChallengeMode, ChallengeConfig, ChallengeState, ChallengeEvent,
    GameModifiers, CurseType, ActiveCurse, ChallengeSummary,
};
pub use quests::{Quest, QuestTracker, QuestReward, QuestCategory, QuestState, ObjectiveType};
pub use time::{
    TimeSystem, TimePhase, TimeEvent, TimeModifiers, TimeEffect,
    CreatureActivity, MINUTES_PER_TURN, MINUTES_PER_DAY,
    BASE_VIEW_RADIUS, MIN_VIEW_RADIUS,
};
pub use achievements::{
    Achievement, AchievementId, AchievementCategory, AchievementTracker,
    AchievementReward, AchievementStats, RunStats, StatType,
};

// Talent system re-exports
pub use talents::{
    TalentId, Talent, TalentTree, TalentEffect, PlayerTalents, TalentStatBonuses,
    Element, TALENT_POINTS_PER_LEVEL, BONUS_POINTS_AT_LEVELS,
};

// Gambling system re-exports
pub use gambling::{
    GamblerKind, GamblerRarity, Gambler, GamblingGame, GambleOutcome,
    GamblingStats, GamblingDen, GamblingEvent, MysteryBoxTier, RiskTier,
    MAX_WAGER, MIN_WAGER, STREAK_BONUS_THRESHOLD,
    generate_gambling_encounters,
};

// Minigames system re-exports
pub use minigames::{
    MinigameType, MinigameSystem, MinigameState, MinigameResult, MinigameReward,
    MinigameContext, MinigameSettings, MinigameStats, MinigameSkill,
    DifficultyLevel, InputMechanic, MinigameInput, InputResult,
    Tournament, TournamentParticipant, TournamentPrize, TrophyType,
    MinigameAchievementId, MinigameTrigger, FailureConsequence, WaterType, OreType,
    MAX_DIFFICULTY, PERFECT_XP_MULTIPLIER, TOURNAMENT_ROUNDS,
};

// Summoning system re-exports
pub use summoning::{
    SummoningSystem, SummonedEntity, SummonType, SummonTier, SummonCategory,
    SummonerRank, SummonContract, ContractType, ContractTerm,
    SummoningRitual, CircleType, SummoningMaterial, MaterialRarity,
    SummonSkill, SummoningFailure, SummonBonuses, SummoningSummary,
    SummonAlignment, MAX_ACTIVE_SUMMONS, MAX_SUMMON_LEVEL,
    BASE_SUMMON_MANA_COST, TEMPORARY_SUMMON_BASE_DURATION,
};
