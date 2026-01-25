use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::io::{stdout, Write, Read as IoRead};
use std::path::PathBuf;
use std::time::Duration;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;

mod achievements;
mod crafting;
mod npc;
mod quest;
mod recipes;

use achievements::AchievementTracker;
use crafting::{CraftingSkill, CraftingProgress, Recipe, CraftingStation, MaterialQuality, CraftingSystem, CraftingResult, Enchantment, FoodBuff, SalvageResult};
use npc::{NPC, NPCType, DialogueAction, DialogueCondition};
use quest::{Quest, QuestJournal, QuestType, QuestObjective, ObjectiveType, QuestReward, QuestStatus, QuestDifficulty, ProceduralQuestGenerator};

// ============================================================================
// CONSTANTS
// ============================================================================

const MAP_WIDTH: usize = 100;
const MAP_HEIGHT: usize = 45;
const VIEW_RADIUS: i32 = 10;
const MAX_ROOMS: usize = 20;
const MIN_ROOM_SIZE: usize = 5;
const MAX_ROOM_SIZE: usize = 15;
const MAX_DUNGEON_LEVEL: u32 = 30;
const BOSS_LEVELS: [u32; 6] = [5, 10, 15, 20, 25, 30];

// ============================================================================
// STATUS EFFECTS
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum StatusEffect {
    Poison,
    Burn,
    Freeze,
    Bleed,
    Stun,
    Blind,
    Haste,
    Shield,
    Regeneration,
    Strength,
    Weakness,
    Invisibility,
    Confusion,
}

impl StatusEffect {
    fn name(&self) -> &'static str {
        match self {
            Self::Poison => "Poisoned",
            Self::Burn => "Burning",
            Self::Freeze => "Frozen",
            Self::Bleed => "Bleeding",
            Self::Stun => "Stunned",
            Self::Blind => "Blind",
            Self::Haste => "Haste",
            Self::Shield => "Shielded",
            Self::Regeneration => "Regenerating",
            Self::Strength => "Strengthened",
            Self::Weakness => "Weakened",
            Self::Invisibility => "Invisible",
            Self::Confusion => "Confused",
        }
    }

    fn color(&self) -> Color {
        match self {
            Self::Poison => Color::Green,
            Self::Burn => Color::Red,
            Self::Freeze => Color::Cyan,
            Self::Bleed => Color::DarkRed,
            Self::Stun => Color::Yellow,
            Self::Blind => Color::DarkGrey,
            Self::Haste => Color::Blue,
            Self::Shield => Color::White,
            Self::Regeneration => Color::Magenta,
            Self::Strength => Color::Yellow,
            Self::Weakness => Color::DarkMagenta,
            Self::Invisibility => Color::Grey,
            Self::Confusion => Color::DarkYellow,
        }
    }
}

// ============================================================================
// CHARACTER CLASS
// ============================================================================

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
enum CharacterClass {
    Warrior,
    Mage,
    Rogue,
    Paladin,
    Ranger,
    Necromancer,
}

impl CharacterClass {
    fn name(&self) -> &'static str {
        match self {
            Self::Warrior => "Warrior",
            Self::Mage => "Mage",
            Self::Rogue => "Rogue",
            Self::Paladin => "Paladin",
            Self::Ranger => "Ranger",
            Self::Necromancer => "Necromancer",
        }
    }

    fn base_stats(&self) -> (i32, i32, i32, i32, i32) {
        // (hp, attack, defense, mana, speed)
        match self {
            Self::Warrior => (50, 8, 5, 10, 10),
            Self::Mage => (30, 3, 2, 50, 10),
            Self::Rogue => (35, 6, 3, 20, 15),
            Self::Paladin => (45, 6, 6, 30, 8),
            Self::Ranger => (38, 7, 3, 25, 12),
            Self::Necromancer => (32, 4, 2, 45, 9),
        }
    }

    fn special_ability(&self) -> &'static str {
        match self {
            Self::Warrior => "Berserk (2x damage, take 50% more)",
            Self::Mage => "Fireball (AoE damage)",
            Self::Rogue => "Backstab (3x damage from behind)",
            Self::Paladin => "Holy Light (heal + damage undead)",
            Self::Ranger => "Multi-shot (hit 3 enemies)",
            Self::Necromancer => "Raise Dead (summon skeleton)",
        }
    }
    fn available_subclasses(&self) -> Vec<Subclass> {
        match self {
            Self::Warrior => vec![Subclass::Berserker, Subclass::Knight, Subclass::Gladiator],
            Self::Mage => vec![Subclass::Elementalist, Subclass::Necromancer, Subclass::Enchanter],
            Self::Rogue => vec![Subclass::Assassin, Subclass::Shadow, Subclass::Trickster],
            Self::Paladin => vec![Subclass::Priest, Subclass::Inquisitor, Subclass::Monk],
            Self::Ranger => vec![Subclass::Beastmaster, Subclass::Archer, Subclass::Druid],
            Self::Necromancer => vec![Subclass::Demonologist, Subclass::BloodMage, Subclass::Hexer],
        }
    }
    fn subclass_upgrade_level() -> u32 { 10 }
    fn advanced_subclass_level() -> u32 { 25 }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
enum Subclass { Berserker, Knight, Gladiator, Warlord, Paladin, DarkKnight, Champion, Elementalist, Necromancer, Enchanter, Archmage, LichLord, Sage, Assassin, Shadow, Trickster, ShadowBlade, Nightstalker, MasterThief, Priest, Inquisitor, Monk, HighPriest, Templar, Grandmaster, Beastmaster, Archer, Druid, Alpha, Marksman, Archdruid, Demonologist, BloodMage, Hexer, DemonLord, BloodLord, CurseWeaver }
impl Subclass {
    fn name(&self) -> &'static str { match self { Self::Berserker => "Berserker", Self::Knight => "Knight", Self::Gladiator => "Gladiator", Self::Warlord => "Warlord", Self::Paladin => "Paladin", Self::DarkKnight => "Dark Knight", Self::Champion => "Champion", Self::Elementalist => "Elementalist", Self::Necromancer => "Necromancer", Self::Enchanter => "Enchanter", Self::Archmage => "Archmage", Self::LichLord => "Lich Lord", Self::Sage => "Sage", Self::Assassin => "Assassin", Self::Shadow => "Shadow", Self::Trickster => "Trickster", Self::ShadowBlade => "Shadow Blade", Self::Nightstalker => "Nightstalker", Self::MasterThief => "Master Thief", Self::Priest => "Priest", Self::Inquisitor => "Inquisitor", Self::Monk => "Monk", Self::HighPriest => "High Priest", Self::Templar => "Templar", Self::Grandmaster => "Grandmaster", Self::Beastmaster => "Beastmaster", Self::Archer => "Archer", Self::Druid => "Druid", Self::Alpha => "Alpha", Self::Marksman => "Marksman", Self::Archdruid => "Archdruid", Self::Demonologist => "Demonologist", Self::BloodMage => "Blood Mage", Self::Hexer => "Hexer", Self::DemonLord => "Demon Lord", Self::BloodLord => "Blood Lord", Self::CurseWeaver => "Curse Weaver" } }
    fn description(&self) -> &'static str { match self { Self::Berserker => "Unleash primal rage", Self::Knight => "Stalwart defender", Self::Gladiator => "Swift duelist", Self::Warlord => "Command armies", Self::Paladin => "Holy warrior", Self::DarkKnight => "Dark powers", Self::Champion => "Arena master", Self::Elementalist => "Master elements", Self::Necromancer => "Death magic", Self::Enchanter => "Powerful buffs", Self::Archmage => "Supreme mage", Self::LichLord => "Undead lord", Self::Sage => "Ultimate wisdom", Self::Assassin => "Deadly strikes", Self::Shadow => "One with darkness", Self::Trickster => "Traps and deception", Self::ShadowBlade => "Shadow weapons", Self::Nightstalker => "Invisible killer", Self::MasterThief => "Steal anything", Self::Priest => "Divine healing", Self::Inquisitor => "Holy wrath", Self::Monk => "Martial mastery", Self::HighPriest => "Miracle worker", Self::Templar => "Holy crusader", Self::Grandmaster => "Martial perfection", Self::Beastmaster => "Animal bond", Self::Archer => "Ranged precision", Self::Druid => "Nature's forces", Self::Alpha => "Pack leader", Self::Marksman => "Perfect accuracy", Self::Archdruid => "Nature incarnate", Self::Demonologist => "Summon demons", Self::BloodMage => "Life for power", Self::Hexer => "Devastating curses", Self::DemonLord => "Demonic legions", Self::BloodLord => "Crimson arts", Self::CurseWeaver => "Inescapable curses" } }
    fn is_advanced(&self) -> bool { matches!(self, Self::Warlord | Self::Paladin | Self::DarkKnight | Self::Champion | Self::Archmage | Self::LichLord | Self::Sage | Self::ShadowBlade | Self::Nightstalker | Self::MasterThief | Self::HighPriest | Self::Templar | Self::Grandmaster | Self::Alpha | Self::Marksman | Self::Archdruid | Self::DemonLord | Self::BloodLord | Self::CurseWeaver) }
    fn advanced_form(&self) -> Option<Subclass> { match self { Self::Berserker => Some(Self::Warlord), Self::Knight => Some(Self::Paladin), Self::Gladiator => Some(Self::Champion), Self::Elementalist => Some(Self::Archmage), Self::Necromancer => Some(Self::LichLord), Self::Enchanter => Some(Self::Sage), Self::Assassin => Some(Self::ShadowBlade), Self::Shadow => Some(Self::Nightstalker), Self::Trickster => Some(Self::MasterThief), Self::Priest => Some(Self::HighPriest), Self::Inquisitor => Some(Self::Templar), Self::Monk => Some(Self::Grandmaster), Self::Beastmaster => Some(Self::Alpha), Self::Archer => Some(Self::Marksman), Self::Druid => Some(Self::Archdruid), Self::Demonologist => Some(Self::DemonLord), Self::BloodMage => Some(Self::BloodLord), Self::Hexer => Some(Self::CurseWeaver), _ => None } }
    fn alternative_advanced(&self) -> Option<Subclass> { match self { Self::Knight => Some(Self::DarkKnight), _ => None } }
    fn available_advanced_forms(&self) -> Vec<Subclass> { let mut f = Vec::new(); if let Some(p) = self.advanced_form() { f.push(p); } if let Some(a) = self.alternative_advanced() { f.push(a); } f }
    fn stat_bonuses(&self) -> (i32, i32, i32, i32, i32) { match self { Self::Berserker => (20, 30, 0, 10, 5), Self::Knight => (30, 15, 25, 0, 0), Self::Gladiator => (15, 25, 10, 5, 20), Self::Warlord => (40, 50, 20, 15, 10), Self::Paladin => (50, 30, 40, 30, 5), Self::DarkKnight => (35, 45, 30, 25, 10), Self::Champion => (30, 40, 20, 10, 35), Self::Elementalist => (10, 20, 5, 40, 5), Self::Necromancer => (15, 15, 10, 35, 5), Self::Enchanter => (10, 10, 15, 45, 5), Self::Archmage => (20, 35, 15, 80, 10), Self::LichLord => (50, 30, 25, 60, 5), Self::Sage => (25, 20, 30, 90, 10), Self::Assassin => (10, 35, 5, 15, 20), Self::Shadow => (15, 20, 10, 20, 25), Self::Trickster => (15, 25, 10, 25, 15), Self::ShadowBlade => (20, 55, 15, 30, 30), Self::Nightstalker => (25, 40, 20, 35, 40), Self::MasterThief => (30, 45, 25, 40, 35), Self::Priest => (25, 10, 15, 40, 5), Self::Inquisitor => (20, 25, 20, 25, 5), Self::Monk => (20, 30, 20, 15, 25), Self::HighPriest => (50, 20, 30, 80, 10), Self::Templar => (40, 45, 40, 40, 15), Self::Grandmaster => (40, 50, 35, 25, 45), Self::Beastmaster => (20, 20, 15, 20, 15), Self::Archer => (15, 30, 10, 15, 20), Self::Druid => (20, 15, 15, 35, 10), Self::Alpha => (40, 35, 30, 30, 25), Self::Marksman => (25, 55, 20, 25, 30), Self::Archdruid => (45, 30, 30, 70, 15), Self::Demonologist => (15, 25, 10, 35, 5), Self::BloodMage => (30, 30, 5, 30, 5), Self::Hexer => (15, 20, 15, 40, 5), Self::DemonLord => (35, 45, 25, 60, 15), Self::BloodLord => (60, 50, 15, 50, 10), Self::CurseWeaver => (30, 35, 30, 80, 10) } }
    fn unique_skills(&self) -> Vec<Skill> { match self { Self::Berserker => vec![Skill::Rage, Skill::Reckless, Skill::BloodFrenzy], Self::Knight => vec![Skill::ShieldWall, Skill::Fortify, Skill::Rally], Self::Gladiator => vec![Skill::DualStrike, Skill::Riposte, Skill::FlurryOfBlows], Self::Warlord => vec![Skill::Rage, Skill::BattleCry, Skill::Devastate, Skill::Conqueror], Self::Paladin => vec![Skill::ShieldWall, Skill::HolySmite, Skill::DivineAura, Skill::Redemption], Self::DarkKnight => vec![Skill::ShieldWall, Skill::DarkSlash, Skill::SoulReap, Skill::Corruption], Self::Champion => vec![Skill::DualStrike, Skill::Execute, Skill::GloryStrike, Skill::Unstoppable], Self::Elementalist => vec![Skill::FireBlast, Skill::FrostNova, Skill::ChainLightning], Self::Necromancer => vec![Skill::RaiseSkeleton, Skill::DeathCoil, Skill::BoneArmor], Self::Enchanter => vec![Skill::Empower, Skill::MagicShield, Skill::Haste], Self::Archmage => vec![Skill::FireBlast, Skill::Meteor, Skill::ElementalMastery, Skill::ArcaneNova], Self::LichLord => vec![Skill::RaiseSkeleton, Skill::ArmyOfDead, Skill::DeathGrip, Skill::Lichform], Self::Sage => vec![Skill::Empower, Skill::TimeWarp, Skill::Omniscience, Skill::Transcendence], Self::Assassin => vec![Skill::DeadlyStrike, Skill::Ambush, Skill::PoisonDagger], Self::Shadow => vec![Skill::ShadowMeld, Skill::Vanish, Skill::ShadowStrike], Self::Trickster => vec![Skill::ThrowKnife, Skill::SmokeScreen, Skill::TrapMaster], Self::ShadowBlade => vec![Skill::DeadlyStrike, Skill::ShadowDance, Skill::DeathMark, Skill::Eviscerate], Self::Nightstalker => vec![Skill::ShadowMeld, Skill::PhantomStrike, Skill::Assassination, Skill::VoidStep], Self::MasterThief => vec![Skill::ThrowKnife, Skill::Pickpocket, Skill::GrandHeist, Skill::LuckOfThief], Self::Priest => vec![Skill::Heal, Skill::Blessing, Skill::Purify], Self::Inquisitor => vec![Skill::HolyStrike, Skill::Judgment, Skill::Exorcism], Self::Monk => vec![Skill::PalmStrike, Skill::FlurryOfBlows, Skill::InnerPeace], Self::HighPriest => vec![Skill::Heal, Skill::Miracle, Skill::DivineIntervention, Skill::Resurrection], Self::Templar => vec![Skill::HolyStrike, Skill::CrusaderStrike, Skill::HolyWrath, Skill::Zealot], Self::Grandmaster => vec![Skill::PalmStrike, Skill::QuiveringPalm, Skill::Enlightenment, Skill::PerfectForm], Self::Beastmaster => vec![Skill::CallPet, Skill::BeastBond, Skill::PackTactics], Self::Archer => vec![Skill::AimedShot, Skill::MultiShot, Skill::PiercingArrow], Self::Druid => vec![Skill::Entangle, Skill::NatureFury, Skill::Rejuvenate], Self::Alpha => vec![Skill::CallPet, Skill::AlphaRoar, Skill::BeastMaster, Skill::Stampede], Self::Marksman => vec![Skill::AimedShot, Skill::Headshot, Skill::RapidFire, Skill::KillShot], Self::Archdruid => vec![Skill::Entangle, Skill::NatureWrath, Skill::TreeForm, Skill::ForceOfNature], Self::Demonologist => vec![Skill::SummonImp, Skill::DemonBolt, Skill::Hellfire], Self::BloodMage => vec![Skill::BloodBolt, Skill::LifeTap, Skill::BloodShield], Self::Hexer => vec![Skill::Hex, Skill::DoomCurse, Skill::Weakness], Self::DemonLord => vec![Skill::SummonImp, Skill::SummonDemon, Skill::InfernalPact, Skill::DemonicForm], Self::BloodLord => vec![Skill::BloodBolt, Skill::Exsanguinate, Skill::CrimsonPact, Skill::BloodNova], Self::CurseWeaver => vec![Skill::Hex, Skill::Doom, Skill::CurseOfAgony, Skill::Pandemonium] } }
    fn color(&self) -> Color { match self { Self::Berserker | Self::BloodMage | Self::BloodLord => Color::Red, Self::Knight | Self::Sage | Self::Priest | Self::HighPriest => Color::White, Self::Gladiator | Self::Champion | Self::Paladin | Self::Inquisitor | Self::Templar => Color::Yellow, Self::Warlord | Self::Demonologist | Self::DemonLord => Color::DarkRed, Self::DarkKnight | Self::ShadowBlade | Self::Hexer | Self::CurseWeaver => Color::DarkMagenta, Self::Elementalist | Self::Grandmaster => Color::Cyan, Self::Necromancer | Self::LichLord | Self::Assassin | Self::Nightstalker => Color::DarkGrey, Self::Enchanter => Color::Magenta, Self::Archmage => Color::Blue, Self::Shadow => Color::Grey, Self::Trickster | Self::MasterThief | Self::Monk => Color::DarkYellow, Self::Beastmaster | Self::Alpha => Color::DarkGreen, Self::Archer | Self::Druid | Self::Marksman | Self::Archdruid => Color::Green } }
}

// ============================================================================
// ============================================================================
// ADVANCED CLASSES (CLASS EVOLUTION)
// ============================================================================

/// Advanced class tiers for class evolution system
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
enum ClassTier {
    Base,       // Starting class
    Tier2,      // First evolution (level 15+)
    Tier3,      // Final evolution (level 30+)
}

/// Advanced classes that characters can evolve into
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
enum AdvancedClass {
    // Warrior evolutions
    Berserker,      // Warrior T2 - offense focused, rage mechanics
    Guardian,       // Warrior T2 - defense focused, shield abilities
    Warlord,        // Warrior T3 - supreme commander, army buffs

    // Mage evolutions
    Archmage,       // Mage T2 - all magic mastery
    Elementalist,   // Mage T2 - elemental magic specialist
    Sage,           // Mage T3 - ultimate wisdom, reality manipulation

    // Rogue evolutions
    Assassin,       // Rogue T2 - burst damage, poison mastery
    Shadowdancer,   // Rogue T2 - shadow magic, teleportation
    Phantom,        // Rogue T3 - ethereal form, death incarnate

    // Paladin evolutions
    Crusader,       // Paladin T2 - offensive holy warrior
    Templar,        // Paladin T2 - defensive holy guardian
    Avatar,         // Paladin T3 - divine vessel, godly power

    // Ranger evolutions
    Beastmaster,    // Ranger T2 - pet mastery, animal companions
    Marksman,       // Ranger T2 - ranged expertise, precision shots
    Warden,         // Ranger T3 - nature's champion, elemental beasts

    // Necromancer evolutions
    Lich,           // Necro T2 - become undead, phylactery
    Reaper,         // Necro T2 - death magic, soul harvest
    DeathLord,      // Necro T3 - master of death, undead army
}

/// Requirements needed to evolve to an advanced class
#[derive(Clone, Debug)]
struct ClassRequirements {
    min_level: u32,
    min_kills: u32,
    required_item: Option<&'static str>,
    required_boss_kills: u32,
    stat_requirements: (i32, i32, i32, i32, i32), // (hp, atk, def, mana, spd)
}

impl ClassRequirements {
    fn new(
        min_level: u32,
        min_kills: u32,
        required_item: Option<&'static str>,
        required_boss_kills: u32,
        stat_requirements: (i32, i32, i32, i32, i32),
    ) -> Self {
        Self {
            min_level,
            min_kills,
            required_item,
            required_boss_kills,
            stat_requirements,
        }
    }
}

impl AdvancedClass {
    /// Get the tier of this advanced class
    fn tier(&self) -> ClassTier {
        match self {
            // Tier 2 classes
            Self::Berserker | Self::Guardian |
            Self::Archmage | Self::Elementalist |
            Self::Assassin | Self::Shadowdancer |
            Self::Crusader | Self::Templar |
            Self::Beastmaster | Self::Marksman |
            Self::Lich | Self::Reaper => ClassTier::Tier2,

            // Tier 3 classes
            Self::Warlord | Self::Sage | Self::Phantom |
            Self::Avatar | Self::Warden | Self::DeathLord => ClassTier::Tier3,
        }
    }

    /// Get the base class this advanced class evolves from
    fn base_class(&self) -> CharacterClass {
        match self {
            Self::Berserker | Self::Guardian | Self::Warlord => CharacterClass::Warrior,
            Self::Archmage | Self::Elementalist | Self::Sage => CharacterClass::Mage,
            Self::Assassin | Self::Shadowdancer | Self::Phantom => CharacterClass::Rogue,
            Self::Crusader | Self::Templar | Self::Avatar => CharacterClass::Paladin,
            Self::Beastmaster | Self::Marksman | Self::Warden => CharacterClass::Ranger,
            Self::Lich | Self::Reaper | Self::DeathLord => CharacterClass::Necromancer,
        }
    }

    /// Get the name of this advanced class
    fn name(&self) -> &'static str {
        match self {
            Self::Berserker => "Berserker",
            Self::Guardian => "Guardian",
            Self::Warlord => "Warlord",
            Self::Archmage => "Archmage",
            Self::Elementalist => "Elementalist",
            Self::Sage => "Sage",
            Self::Assassin => "Assassin",
            Self::Shadowdancer => "Shadowdancer",
            Self::Phantom => "Phantom",
            Self::Crusader => "Crusader",
            Self::Templar => "Templar",
            Self::Avatar => "Avatar",
            Self::Beastmaster => "Beastmaster",
            Self::Marksman => "Marksman",
            Self::Warden => "Warden",
            Self::Lich => "Lich",
            Self::Reaper => "Reaper",
            Self::DeathLord => "Death Lord",
        }
    }

    /// Get the description of this advanced class
    fn description(&self) -> &'static str {
        match self {
            Self::Berserker => "Fury incarnate. Massive damage, rage abilities, low defense",
            Self::Guardian => "Unbreakable wall. High defense, party protection, taunts",
            Self::Warlord => "Supreme commander. Army buffs, devastating charges, fear aura",
            Self::Archmage => "Master of all magic. Reduced mana costs, spell amplification",
            Self::Elementalist => "Elemental fury. Fire, ice, lightning mastery, combos",
            Self::Sage => "Ultimate wisdom. Reality manipulation, time magic, omniscience",
            Self::Assassin => "Death from shadows. Instant kills, poison mastery, stealth",
            Self::Shadowdancer => "Shadow and magic. Teleportation, illusions, dark magic",
            Self::Phantom => "Beyond death. Ethereal form, soul rend, untouchable",
            Self::Crusader => "Holy warrior. Smite evil, divine strikes, zealous charge",
            Self::Templar => "Divine shield. Holy barriers, healing aura, sanctuary",
            Self::Avatar => "Divine vessel. Godly power, resurrection, judgment",
            Self::Beastmaster => "Beast lord. Multiple pets, animal forms, pack tactics",
            Self::Marksman => "Perfect aim. Critical shots, piercing arrows, sniper",
            Self::Warden => "Nature's champion. Elemental beasts, terrain control, primal fury",
            Self::Lich => "Undead mage. Immortal phylactery, ice magic, curse mastery",
            Self::Reaper => "Death's hand. Soul harvest, instant death, fear aura",
            Self::DeathLord => "Master of death. Massive undead army, death knight form",
        }
    }

    /// Get the stat bonuses for this advanced class
    fn stat_bonuses(&self) -> (i32, i32, i32, i32, i32) {
        // (hp, attack, defense, mana, speed)
        match self {
            // Warrior evolutions
            Self::Berserker => (30, 25, -5, 0, 10),
            Self::Guardian => (50, 5, 30, 10, -5),
            Self::Warlord => (60, 30, 25, 20, 15),

            // Mage evolutions
            Self::Archmage => (10, 10, 5, 60, 5),
            Self::Elementalist => (15, 20, 5, 45, 10),
            Self::Sage => (30, 25, 20, 80, 20),

            // Rogue evolutions
            Self::Assassin => (10, 30, 0, 15, 25),
            Self::Shadowdancer => (15, 20, 5, 35, 20),
            Self::Phantom => (25, 35, 15, 40, 35),

            // Paladin evolutions
            Self::Crusader => (35, 25, 15, 30, 10),
            Self::Templar => (45, 15, 30, 35, 5),
            Self::Avatar => (60, 30, 35, 50, 20),

            // Ranger evolutions
            Self::Beastmaster => (30, 20, 15, 25, 15),
            Self::Marksman => (15, 35, 5, 20, 20),
            Self::Warden => (45, 30, 25, 35, 25),

            // Necromancer evolutions
            Self::Lich => (0, 15, 10, 70, 5),  // Lich trades HP for power
            Self::Reaper => (20, 30, 5, 50, 15),
            Self::DeathLord => (40, 35, 25, 65, 20),
        }
    }

    /// Get the requirements to evolve to this class
    fn requirements(&self) -> ClassRequirements {
        match self {
            // Tier 2 Warrior
            Self::Berserker => ClassRequirements::new(
                15, 100, Some("Rage Crystal"), 2, (80, 15, 0, 0, 0)
            ),
            Self::Guardian => ClassRequirements::new(
                15, 75, Some("Aegis Shield"), 2, (100, 0, 15, 0, 0)
            ),
            // Tier 3 Warrior
            Self::Warlord => ClassRequirements::new(
                30, 300, Some("Conqueror's Crown"), 10, (150, 25, 20, 0, 0)
            ),

            // Tier 2 Mage
            Self::Archmage => ClassRequirements::new(
                15, 60, Some("Tome of Infinite Knowledge"), 2, (0, 0, 0, 100, 0)
            ),
            Self::Elementalist => ClassRequirements::new(
                15, 80, Some("Elemental Core"), 2, (0, 10, 0, 80, 0)
            ),
            // Tier 3 Mage
            Self::Sage => ClassRequirements::new(
                30, 200, Some("Orb of Omniscience"), 10, (0, 15, 0, 200, 0)
            ),

            // Tier 2 Rogue
            Self::Assassin => ClassRequirements::new(
                15, 120, Some("Venom Fang Dagger"), 2, (0, 15, 0, 0, 20)
            ),
            Self::Shadowdancer => ClassRequirements::new(
                15, 90, Some("Shadow Essence"), 2, (0, 10, 0, 40, 20)
            ),
            // Tier 3 Rogue
            Self::Phantom => ClassRequirements::new(
                30, 350, Some("Soul of the Void"), 10, (0, 30, 0, 60, 30)
            ),

            // Tier 2 Paladin
            Self::Crusader => ClassRequirements::new(
                15, 100, Some("Holy Avenger Sword"), 3, (80, 15, 10, 50, 0)
            ),
            Self::Templar => ClassRequirements::new(
                15, 80, Some("Divine Relic"), 3, (100, 10, 15, 60, 0)
            ),
            // Tier 3 Paladin
            Self::Avatar => ClassRequirements::new(
                30, 250, Some("Blessing of the Gods"), 12, (150, 25, 25, 100, 0)
            ),

            // Tier 2 Ranger
            Self::Beastmaster => ClassRequirements::new(
                15, 90, Some("Beast Soul Totem"), 2, (60, 15, 10, 40, 15)
            ),
            Self::Marksman => ClassRequirements::new(
                15, 100, Some("Legendary Bow"), 2, (50, 20, 5, 30, 18)
            ),
            // Tier 3 Ranger
            Self::Warden => ClassRequirements::new(
                30, 280, Some("Heart of the Wild"), 10, (100, 30, 20, 60, 25)
            ),

            // Tier 2 Necromancer
            Self::Lich => ClassRequirements::new(
                15, 80, Some("Phylactery"), 3, (0, 10, 5, 100, 0)
            ),
            Self::Reaper => ClassRequirements::new(
                15, 100, Some("Death's Scythe"), 3, (40, 15, 5, 80, 10)
            ),
            // Tier 3 Necromancer
            Self::DeathLord => ClassRequirements::new(
                30, 300, Some("Crown of the Dead King"), 12, (80, 25, 15, 150, 15)
            ),
        }
    }

    /// Get the unique ability for this advanced class
    fn unique_ability(&self) -> &'static str {
        match self {
            Self::Berserker => "Blood Rage - Enter berserk state: +100% damage, lifesteal, but lose 5 HP/turn",
            Self::Guardian => "Fortress Stance - Become immovable: 75% damage reduction, taunt all enemies",
            Self::Warlord => "Rally the Troops - All allies gain +50% stats, enemies feared for 5 turns",

            Self::Archmage => "Arcane Overload - Next 3 spells cost no mana and deal double damage",
            Self::Elementalist => "Elemental Convergence - Unleash all elements at once, massive AoE",
            Self::Sage => "Time Stop - Freeze time for 3 turns, act freely while enemies frozen",

            Self::Assassin => "Death Mark - Mark target for instant kill on next hit from behind",
            Self::Shadowdancer => "Shadow Realm - Enter shadow plane, invisible and can phase through walls",
            Self::Phantom => "Soul Rend - Become ethereal, attacks hit souls directly ignoring armor",

            Self::Crusader => "Divine Judgment - Smite all enemies in sight, bonus damage to undead/demons",
            Self::Templar => "Sanctuary - Create holy barrier, party invulnerable for 3 turns",
            Self::Avatar => "Divine Descent - Transform into godly form, +200% all stats for 10 turns",

            Self::Beastmaster => "Pack Alpha - Summon 3 legendary beasts to fight alongside you",
            Self::Marksman => "Perfect Shot - Guaranteed critical hit that pierces all enemies in line",
            Self::Warden => "Primal Storm - Summon nature's wrath, elemental beasts rain destruction",

            Self::Lich => "Phylactery Bond - On death, resurrect at full power after 3 turns",
            Self::Reaper => "Soul Harvest - Kill all enemies below 25% HP, heal for their max HP",
            Self::DeathLord => "Army of the Damned - Raise all corpses on floor as permanent undead army",
        }
    }

    /// Get available evolution paths from a base class
    fn evolution_paths(base: CharacterClass) -> Vec<AdvancedClass> {
        match base {
            CharacterClass::Warrior => vec![Self::Berserker, Self::Guardian],
            CharacterClass::Mage => vec![Self::Archmage, Self::Elementalist],
            CharacterClass::Rogue => vec![Self::Assassin, Self::Shadowdancer],
            CharacterClass::Paladin => vec![Self::Crusader, Self::Templar],
            CharacterClass::Ranger => vec![Self::Beastmaster, Self::Marksman],
            CharacterClass::Necromancer => vec![Self::Lich, Self::Reaper],
        }
    }

    /// Get tier 3 evolution from tier 2 classes
    fn tier3_evolution(&self) -> Option<AdvancedClass> {
        match self {
            Self::Berserker | Self::Guardian => Some(Self::Warlord),
            Self::Archmage | Self::Elementalist => Some(Self::Sage),
            Self::Assassin | Self::Shadowdancer => Some(Self::Phantom),
            Self::Crusader | Self::Templar => Some(Self::Avatar),
            Self::Beastmaster | Self::Marksman => Some(Self::Warden),
            Self::Lich | Self::Reaper => Some(Self::DeathLord),
            // Tier 3 classes have no further evolution
            _ => None,
        }
    }

    /// Check if a player meets the requirements for this class
    fn can_evolve(&self, player_level: u32, player_kills: u32, boss_kills: u32,
                  player_stats: (i32, i32, i32, i32, i32), has_item: bool) -> bool {
        let req = self.requirements();
        player_level >= req.min_level
            && player_kills >= req.min_kills
            && boss_kills >= req.required_boss_kills
            && player_stats.0 >= req.stat_requirements.0  // HP
            && player_stats.1 >= req.stat_requirements.1  // ATK
            && player_stats.2 >= req.stat_requirements.2  // DEF
            && player_stats.3 >= req.stat_requirements.3  // Mana
            && player_stats.4 >= req.stat_requirements.4  // Speed
            && (req.required_item.is_none() || has_item)
    }

    /// Get all tier 2 classes
    fn all_tier2() -> Vec<AdvancedClass> {
        vec![
            Self::Berserker, Self::Guardian,
            Self::Archmage, Self::Elementalist,
            Self::Assassin, Self::Shadowdancer,
            Self::Crusader, Self::Templar,
            Self::Beastmaster, Self::Marksman,
            Self::Lich, Self::Reaper,
        ]
    }

    /// Get all tier 3 classes
    fn all_tier3() -> Vec<AdvancedClass> {
        vec![
            Self::Warlord, Self::Sage, Self::Phantom,
            Self::Avatar, Self::Warden, Self::DeathLord,
        ]
    }
}

// SPECIES AND SUBSPECIES
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Species {
    Human(HumanSubspecies),
    Elf(ElfSubspecies),
    Dwarf(DwarfSubspecies),
    Dragonian(DragonianSubspecies),
    Demon(DemonSubspecies),
    Undead(UndeadSubspecies),
    Beastkin(BeastkinSubspecies),
    Orc(OrcSubspecies),
    Goblin(GoblinSubspecies),
    Fairy(FairySubspecies),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum HumanSubspecies {
    Northerner,  // +10 STR, +5 DEF, cold resist
    Southerner,  // +10 DEX, +5 SPD, heat resist
    Imperial,    // +5 all stats, +gold find
    Nomad,       // +15 SPD, survival bonuses
    Islander,    // +10 DEX, water breathing
}

impl HumanSubspecies {
    fn stat_bonuses(&self) -> (i32, i32, i32, i32, i32) { // (hp, atk, def, spd, mana)
        match self {
            Self::Northerner => (20, 10, 5, 0, 0),
            Self::Southerner => (0, 5, 0, 10, 5),
            Self::Imperial => (10, 5, 5, 5, 10),
            Self::Nomad => (5, 5, 0, 15, 0),
            Self::Islander => (10, 5, 5, 10, 0),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Northerner => "Northerner",
            Self::Southerner => "Southerner",
            Self::Imperial => "Imperial",
            Self::Nomad => "Nomad",
            Self::Islander => "Islander",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::Northerner => "Hardy folk from frozen lands. +STR, +DEF, Cold Resist",
            Self::Southerner => "Agile desert dwellers. +DEX, +SPD, Heat Resist",
            Self::Imperial => "Noble bloodline. Balanced stats, +Gold Find",
            Self::Nomad => "Wandering survivors. High speed, survival skills",
            Self::Islander => "Sea-faring people. +DEX, can breathe underwater",
        }
    }
}

impl Species {
    fn name(&self) -> &'static str {
        match self {
            Self::Human(sub) => sub.name(),
            Self::Elf(sub) => sub.name(),
            Self::Dwarf(sub) => sub.name(),
            Self::Dragonian(sub) => sub.name(),
            Self::Demon(sub) => sub.name(),
            Self::Undead(sub) => sub.name(),
            Self::Beastkin(sub) => sub.name(),
            Self::Orc(sub) => sub.name(),
            Self::Goblin(sub) => sub.name(),
            Self::Fairy(sub) => sub.name(),
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::Human(sub) => sub.description(),
            Self::Elf(sub) => sub.description(),
            Self::Dwarf(sub) => sub.description(),
            Self::Dragonian(sub) => sub.description(),
            Self::Demon(sub) => sub.description(),
            Self::Undead(sub) => sub.description(),
            Self::Beastkin(sub) => sub.description(),
            Self::Orc(sub) => sub.description(),
            Self::Goblin(sub) => sub.description(),
            Self::Fairy(sub) => sub.description(),
        }
    }

    fn stat_bonuses(&self) -> (i32, i32, i32, i32, i32) { // (hp, atk, def, spd, mana)
        match self {
            Self::Human(sub) => sub.stat_bonuses(),
            Self::Elf(sub) => sub.stat_bonuses(),
            Self::Dwarf(sub) => sub.stat_bonuses(),
            Self::Dragonian(sub) => sub.stat_bonuses(),
            Self::Demon(sub) => sub.stat_bonuses(),
            Self::Undead(sub) => sub.stat_bonuses(),
            Self::Beastkin(sub) => sub.stat_bonuses(),
            Self::Orc(sub) => sub.stat_bonuses(),
            Self::Goblin(sub) => sub.stat_bonuses(),
            Self::Fairy(sub) => sub.stat_bonuses(),
        }
    }

    fn base_species_name(&self) -> &'static str {
        match self {
            Self::Human(_) => "Human",
            Self::Elf(_) => "Elf",
            Self::Dwarf(_) => "Dwarf",
            Self::Dragonian(_) => "Dragonian",
            Self::Demon(_) => "Demon",
            Self::Undead(_) => "Undead",
            Self::Beastkin(_) => "Beastkin",
            Self::Orc(_) => "Orc",
            Self::Goblin(_) => "Goblin",
            Self::Fairy(_) => "Fairy",
        }
    }
}

// ============================================================================
// ELF SUBSPECIES
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
enum ElfSubspecies {
    HighElf,      // +20 INT, +30 mana, arcane magic
    WoodElf,      // +15 DEX, nature magic, forest bonus
    DarkElf,      // +10 DEX, +10 INT, dark magic, stealth
    BloodElf,     // +15 INT, blood magic, lifesteal
    SeaElf,       // +10 DEX, water magic, swim
    MoonElf,      // +20 INT, lunar magic, night vision
    SunElf,       // +15 ATK, +10 INT, holy magic
    WildElf,      // +20 DEX, beast taming, savage
}

impl ElfSubspecies {
    fn stat_bonuses(&self) -> (i32, i32, i32, i32, i32) {
        match self {
            Self::HighElf => (0, 5, 0, 5, 30),
            Self::WoodElf => (10, 10, 5, 15, 10),
            Self::DarkElf => (0, 10, 5, 10, 15),
            Self::BloodElf => (-10, 15, 0, 5, 20),
            Self::SeaElf => (15, 5, 10, 10, 10),
            Self::MoonElf => (5, 5, 5, 5, 25),
            Self::SunElf => (10, 15, 5, 5, 15),
            Self::WildElf => (15, 15, 0, 20, 0),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::HighElf => "High Elf",
            Self::WoodElf => "Wood Elf",
            Self::DarkElf => "Dark Elf",
            Self::BloodElf => "Blood Elf",
            Self::SeaElf => "Sea Elf",
            Self::MoonElf => "Moon Elf",
            Self::SunElf => "Sun Elf",
            Self::WildElf => "Wild Elf",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::HighElf => "Ancient arcane masters. High mana, magic affinity",
            Self::WoodElf => "Forest guardians. Agile, nature magic",
            Self::DarkElf => "Shadow dwellers. Strong attacks, stealth",
            Self::BloodElf => "Blood mages. Lifesteal, powerful magic",
            Self::SeaElf => "Ocean dwellers. Water breathing, swim mastery",
            Self::MoonElf => "Lunar elves. Stronger at night, magic power",
            Self::SunElf => "Light wielders. Holy magic, day strength",
            Self::WildElf => "Feral elves. Beast taming, savage combat",
        }
    }

    fn special_ability(&self) -> &'static str {
        match self {
            Self::HighElf => "Arcane Mastery - Spells cost 20% less mana",
            Self::WoodElf => "Nature's Grace - Heal in forests, tame beasts",
            Self::DarkElf => "Shadow Step - Short range teleport",
            Self::BloodElf => "Blood Drain - Steal HP with attacks",
            Self::SeaElf => "Aquatic - Breathe underwater, water magic boost",
            Self::MoonElf => "Lunar Power - Stronger at night",
            Self::SunElf => "Solar Blessing - Stronger during day, holy resist",
            Self::WildElf => "Feral Rage - Berserk mode when low HP",
        }
    }
}

// ============================================================================
// DWARF SUBSPECIES
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
enum DwarfSubspecies {
    MountainDwarf,  // +25 STR, +20 DEF, mining bonus
    DeepDwarf,      // +15 DEF, dark vision, poison resist
    GoldDwarf,      // +10 all, +50% gold find
    IronDwarf,      // +30 DEF, slow, immune to knockback
    RuneDwarf,      // +20 INT, rune magic, enchanting
    FrostDwarf,     // +20 DEF, ice resist, frost weapons
    FireDwarf,      // +20 ATK, fire resist, forge master
    HillDwarf,      // +30 HP, +10 DEF, nature resist
}

impl DwarfSubspecies {
    fn stat_bonuses(&self) -> (i32, i32, i32, i32, i32) {
        // (hp, attack, defense, speed, mana)
        match self {
            Self::MountainDwarf => (25, 20, 20, -5, 0),
            Self::DeepDwarf => (20, 10, 15, 0, 5),
            Self::GoldDwarf => (15, 10, 10, 5, 10),
            Self::IronDwarf => (30, 15, 30, -10, 0),
            Self::RuneDwarf => (10, 5, 10, 0, 25),
            Self::FrostDwarf => (20, 15, 20, 0, 5),
            Self::FireDwarf => (15, 25, 15, 0, 5),
            Self::HillDwarf => (40, 10, 15, 5, 0),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::MountainDwarf => "Mountain Dwarf",
            Self::DeepDwarf => "Deep Dwarf",
            Self::GoldDwarf => "Gold Dwarf",
            Self::IronDwarf => "Iron Dwarf",
            Self::RuneDwarf => "Rune Dwarf",
            Self::FrostDwarf => "Frost Dwarf",
            Self::FireDwarf => "Fire Dwarf",
            Self::HillDwarf => "Hill Dwarf",
        }
    }

    fn special_ability(&self) -> &'static str {
        match self {
            Self::MountainDwarf => "Stone Skin - Take 25% less physical damage",
            Self::DeepDwarf => "Dark Vision - See in complete darkness",
            Self::GoldDwarf => "Treasure Sense - Find hidden gold and items",
            Self::IronDwarf => "Immovable - Cannot be pushed or knocked back",
            Self::RuneDwarf => "Rune Craft - Enchant weapons and armor",
            Self::FrostDwarf => "Frost Aura - Slow nearby enemies",
            Self::FireDwarf => "Forge Master - Weapons deal fire damage",
            Self::HillDwarf => "Dwarven Resilience - Poison/Disease immunity",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::MountainDwarf => "Hardy dwarf from the mountain halls",
            Self::DeepDwarf => "Dwarf adapted to deep underground living",
            Self::GoldDwarf => "Dwarf with innate gold-finding abilities",
            Self::IronDwarf => "Dwarf with unbreakable resolve",
            Self::RuneDwarf => "Dwarf with ancient rune magic",
            Self::FrostDwarf => "Dwarf from the frozen peaks",
            Self::FireDwarf => "Dwarf master of forge and flame",
            Self::HillDwarf => "Sturdy dwarf from the hills",
        }
    }
}

// ============================================================================
// FAIRY SUBSPECIES
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
enum FairySubspecies {
    Pixie,          // Tiny, fast, nature magic
    Sprite,         // Light magic, healing
    Dryad,          // Tree spirit, nature power
    Nymph,          // Water/nature, charm
    Sylph,          // Air spirit, wind magic
    WillOWisp,      // Ghost light, confusion
    DarkFairy,      // Shadow magic, curses
    Leprechaun,     // Luck, gold, tricks
}

impl FairySubspecies {
    fn name(&self) -> &'static str {
        match self {
            Self::Pixie => "Pixie",
            Self::Sprite => "Sprite",
            Self::Dryad => "Dryad",
            Self::Nymph => "Nymph",
            Self::Sylph => "Sylph",
            Self::WillOWisp => "Will-o'-Wisp",
            Self::DarkFairy => "Dark Fairy",
            Self::Leprechaun => "Leprechaun",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::Pixie => "Tiny fairy with incredible speed",
            Self::Sprite => "Light fairy with healing powers",
            Self::Dryad => "Tree spirit with nature magic",
            Self::Nymph => "Water fairy with charm abilities",
            Self::Sylph => "Air spirit with wind magic",
            Self::WillOWisp => "Ghost light with confusion powers",
            Self::DarkFairy => "Corrupted fairy with curse magic",
            Self::Leprechaun => "Lucky fairy with gold-finding abilities",
        }
    }

    fn stat_bonuses(&self) -> (i32, i32, i32, i32, i32) {
        match self {
            Self::Pixie => (-30, 5, -20, 40, 30),
            Self::Sprite => (-20, 5, -15, 35, 40),
            Self::Dryad => (10, 10, 15, 10, 30),
            Self::Nymph => (0, 10, 5, 25, 35),
            Self::Sylph => (-15, 10, -10, 45, 25),
            Self::WillOWisp => (-25, 15, -20, 30, 40),
            Self::DarkFairy => (-20, 20, -10, 35, 35),
            Self::Leprechaun => (-10, 15, 0, 30, 25),
        }
    }

    fn fairy_magic(&self) -> &'static str {
        match self {
            Self::Pixie => "Shrink - Become tiny, dodge everything",
            Self::Sprite => "Healing Light - Powerful healing aura",
            Self::Dryad => "Tree Form - Root in place, massive regen",
            Self::Nymph => "Allure - Charm enemies to fight for you",
            Self::Sylph => "Wind Walk - Fly over all terrain",
            Self::WillOWisp => "Hypnotic Light - Confuse all enemies",
            Self::DarkFairy => "Fairy Curse - Powerful debuffs",
            Self::Leprechaun => "Pot of Gold - Massive luck bonus",
        }
    }
}

// ============================================================================
// ANGEL SUBSPECIES
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
enum AngelSubspecies {
    Seraph,         // Fire, purification, 6 wings
    Cherub,         // Guardian, protection magic
    Throne,         // Justice, smite evil
    Dominion,       // Leadership, buff allies
    Virtue,         // Miracles, healing
    Power,          // Warrior angel, holy weapons
    FallenAngel,    // Corrupted, dark+holy mix
    Nephilim,       // Half-angel, balanced
}

impl AngelSubspecies {
    fn stat_bonuses(&self) -> (i32, i32, i32, i32, i32) {
        match self {
            Self::Seraph => (30, 30, 20, 20, 50),
            Self::Cherub => (40, 15, 35, 15, 30),
            Self::Throne => (25, 35, 25, 15, 35),
            Self::Dominion => (30, 20, 25, 20, 40),
            Self::Virtue => (20, 10, 20, 20, 60),
            Self::Power => (35, 40, 30, 15, 25),
            Self::FallenAngel => (25, 35, 20, 25, 45),
            Self::Nephilim => (30, 25, 25, 20, 30),
        }
    }

    fn divine_power(&self) -> &'static str {
        match self {
            Self::Seraph => "Holy Fire - Burn evil with divine flames",
            Self::Cherub => "Divine Shield - Protect self and allies",
            Self::Throne => "Judgment - Execute low HP enemies",
            Self::Dominion => "Command - Buff all allies significantly",
            Self::Virtue => "Miracle - Full heal and cure all",
            Self::Power => "Holy Weapon - Summon divine armaments",
            Self::FallenAngel => "Twilight - Mix of holy and dark powers",
            Self::Nephilim => "Hybrid Power - Use both mortal and divine abilities",
        }
    }
}

// ============================================================================
// DRAGONIAN SPECIES & EVOLUTION
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
enum DragonianSubspecies {
    RedDragonian,    // Fire type - fire breath, fire immune
    BlackDragonian,  // Dark/Acid - acid spit, dark magic
    BlueDragonian,   // Lightning - electric breath, storm magic
    WhiteDragonian,  // Ice - frost breath, cold immune
    GoldDragonian,   // Holy - holy breath, all resists
    GreenDragonian,  // Poison - poison gas, nature magic
    SilverDragonian, // Wind - wind breath, flight
    BronzeDragonian, // Earth - quake, high defense
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
enum DragonForm {
    Dragonian,      // Starting form - humanoid with dragon features
    Drake,          // Level 10 - larger, wings
    Wyrm,           // Level 20 - serpentine, powerful magic
    TrueDragon,     // Level 30 - full dragon form, ultimate power
    ElderDragon,    // Level 40 - ancient power, reality warping
    DragonGod,      // Level 50 - godlike, near invincible
}

impl DragonianSubspecies {
    fn name(&self) -> &'static str {
        match self {
            Self::RedDragonian => "Red Dragonian",
            Self::BlackDragonian => "Black Dragonian",
            Self::BlueDragonian => "Blue Dragonian",
            Self::WhiteDragonian => "White Dragonian",
            Self::GoldDragonian => "Gold Dragonian",
            Self::GreenDragonian => "Green Dragonian",
            Self::SilverDragonian => "Silver Dragonian",
            Self::BronzeDragonian => "Bronze Dragonian",
        }
    }

    fn stat_bonuses(&self) -> (i32, i32, i32, i32, i32) {
        // (hp, attack, defense, speed, mana)
        match self {
            Self::RedDragonian => (20, 25, 10, 5, 15),
            Self::BlackDragonian => (15, 20, 15, 10, 20),
            Self::BlueDragonian => (10, 20, 10, 15, 25),
            Self::WhiteDragonian => (25, 15, 20, 5, 15),
            Self::GoldDragonian => (20, 20, 20, 10, 30),
            Self::GreenDragonian => (15, 15, 15, 15, 20),
            Self::SilverDragonian => (10, 15, 10, 25, 20),
            Self::BronzeDragonian => (30, 20, 30, -5, 10),
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::RedDragonian => "Flame-scaled warriors. Fire breath, immune to fire",
            Self::BlackDragonian => "Acid dragons. Corrosive attacks, dark magic",
            Self::BlueDragonian => "Storm dragons. Lightning attacks, high speed",
            Self::WhiteDragonian => "Frost dragons. Ice magic, immune to cold",
            Self::GoldDragonian => "Holy dragons. Divine power, all resists",
            Self::GreenDragonian => "Poison dragons. Toxic attacks, nature magic",
            Self::SilverDragonian => "Wind dragons. Fast, flight mastery",
            Self::BronzeDragonian => "Earth dragons. Incredible defense, earthquakes",
        }
    }

    fn breath_weapon(&self) -> &'static str {
        match self {
            Self::RedDragonian => "Fire Breath - Cone of flames",
            Self::BlackDragonian => "Acid Spray - Melts armor",
            Self::BlueDragonian => "Lightning Bolt - Chain lightning",
            Self::WhiteDragonian => "Frost Breath - Freezes enemies",
            Self::GoldDragonian => "Holy Fire - Burns evil",
            Self::GreenDragonian => "Poison Cloud - AoE poison",
            Self::SilverDragonian => "Wind Blast - Knockback",
            Self::BronzeDragonian => "Earth Spike - Stuns",
        }
    }
}

impl DragonForm {
    fn name(&self) -> &'static str {
        match self {
            Self::Dragonian => "Dragonian",
            Self::Drake => "Drake",
            Self::Wyrm => "Wyrm",
            Self::TrueDragon => "True Dragon",
            Self::ElderDragon => "Elder Dragon",
            Self::DragonGod => "Dragon God",
        }
    }

    fn can_evolve(level: u32, current_form: DragonForm) -> Option<DragonForm> {
        match (current_form, level) {
            (DragonForm::Dragonian, l) if l >= 10 => Some(DragonForm::Drake),
            (DragonForm::Drake, l) if l >= 20 => Some(DragonForm::Wyrm),
            (DragonForm::Wyrm, l) if l >= 30 => Some(DragonForm::TrueDragon),
            (DragonForm::TrueDragon, l) if l >= 40 => Some(DragonForm::ElderDragon),
            (DragonForm::ElderDragon, l) if l >= 50 => Some(DragonForm::DragonGod),
            _ => None,
        }
    }

    fn evolution_bonus(&self) -> (i32, i32, i32, i32, i32) {
        // (hp, attack, defense, speed, mana)
        match self {
            DragonForm::Dragonian => (0, 0, 0, 0, 0),
            DragonForm::Drake => (50, 20, 20, 10, 30),
            DragonForm::Wyrm => (100, 40, 40, 20, 60),
            DragonForm::TrueDragon => (200, 80, 80, 30, 100),
            DragonForm::ElderDragon => (400, 150, 150, 50, 200),
            DragonForm::DragonGod => (1000, 300, 300, 100, 500),
        }
    }
}

// ============================================================================
// UNDEAD SUBSPECIES
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
enum UndeadSubspecies {
    Skeleton,       // No hunger, pierce resist, fragile
    Zombie,         // High HP, regen from corpses, slow
    Vampire,        // Lifesteal, night power, sun weakness
    Lich,           // Powerful magic, phylactery (extra life)
    Wraith,         // Incorporeal, drain life, phase through walls
    DeathKnight,    // Heavy armor, dark aura, summon undead
    Mummy,          // Curse enemies, sand magic, ancient power
    Revenant,       // Vengeance power, cannot die until revenge
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
enum UndeadTier {
    Risen,          // Freshly undead
    Greater,        // Level 15 - stronger undead powers
    Ancient,        // Level 30 - powerful undead lord
    Eternal,        // Level 45 - death incarnate
}

impl UndeadSubspecies {
    fn stat_bonuses(&self) -> (i32, i32, i32, i32, i32) {
        match self {
            Self::Skeleton => (0, 15, 5, 15, 10),
            Self::Zombie => (60, 10, 15, -15, 0),
            Self::Vampire => (20, 20, 10, 20, 30),
            Self::Lich => (-20, 10, 0, 5, 80),
            Self::Wraith => (-30, 15, -10, 25, 40),
            Self::DeathKnight => (40, 25, 30, 0, 20),
            Self::Mummy => (30, 15, 20, -5, 25),
            Self::Revenant => (20, 30, 20, 10, 10),
        }
    }

    fn undead_trait(&self) -> &'static str {
        match self {
            Self::Skeleton => "Boneless - No hunger, immune to poison/bleed",
            Self::Zombie => "Relentless - Regen HP from killing",
            Self::Vampire => "Blood Thirst - Must drain blood, powerful at night",
            Self::Lich => "Phylactery - Respawn once per floor if killed",
            Self::Wraith => "Incorporeal - Phase through walls, 50% physical resist",
            Self::DeathKnight => "Death Aura - Weaken nearby living enemies",
            Self::Mummy => "Ancient Curse - Curse attackers",
            Self::Revenant => "Undying Rage - Cannot die while enemies remain",
        }
    }

    fn holy_vulnerability(&self) -> i32 {
        match self {
            Self::Skeleton | Self::Zombie => 150,  // 1.5x holy damage
            Self::Vampire | Self::Lich => 200,     // 2x holy damage
            Self::Wraith => 300,                   // 3x holy damage
            Self::DeathKnight | Self::Mummy => 150,
            Self::Revenant => 100,                 // No extra holy damage
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::Skeleton => "Animated bones. Immune to poison and bleeding",
            Self::Zombie => "Risen dead. Very tough, regenerates, but slow",
            Self::Vampire => "Blood drinkers. Lifesteal, weak to sunlight",
            Self::Lich => "Undead mages. Immense magic power, phylactery",
            Self::Wraith => "Spectral beings. Phase through walls, ethereal",
            Self::DeathKnight => "Undead warriors. Heavy armor, dark aura",
            Self::Mummy => "Ancient dead. Curses, sand magic",
            Self::Revenant => "Vengeful spirits. Cannot die until revenge",
        }
    }

    #[allow(dead_code)]
    fn name(&self) -> &'static str {
        match self {
            Self::Skeleton => "Skeleton",
            Self::Zombie => "Zombie",
            Self::Vampire => "Vampire",
            Self::Lich => "Lich",
            Self::Wraith => "Wraith",
            Self::DeathKnight => "Death Knight",
            Self::Mummy => "Mummy",
            Self::Revenant => "Revenant",
        }
    }
}

impl UndeadTier {
    #[allow(dead_code)]
    fn from_level(level: u32) -> Self {
        match level {
            0..=14 => Self::Risen,
            15..=29 => Self::Greater,
            30..=44 => Self::Ancient,
            _ => Self::Eternal,
        }
    }

    #[allow(dead_code)]
    fn name(&self) -> &'static str {
        match self {
            Self::Risen => "Risen",
            Self::Greater => "Greater",
            Self::Ancient => "Ancient",
            Self::Eternal => "Eternal",
        }
    }

    #[allow(dead_code)]
    fn power_multiplier(&self) -> f32 {
        match self {
            Self::Risen => 1.0,
            Self::Greater => 1.5,
            Self::Ancient => 2.0,
            Self::Eternal => 3.0,
        }
    }
}

// ============================================================================
// DEMON SUBSPECIES
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum DemonSubspecies {
    Infernal,    // +20 ATK, +10 MANA, fire magic
    Abyssal,     // +15 HP, +15 DEF, darkness immune
    Succubus,    // +15 MANA, +10 SPD, charm ability
    Imp,         // +20 SPD, +5 ATK, flight
    Balor,       // +25 ATK, +15 HP, -10 SPD, berserker
}

impl DemonSubspecies {
    fn stat_bonuses(&self) -> (i32, i32, i32, i32, i32) { // (hp, atk, def, spd, mana)
        match self {
            Self::Infernal => (10, 25, 5, 0, 15),
            Self::Abyssal => (20, 10, 20, -5, 0),
            Self::Succubus => (0, 5, 0, 15, 25),
            Self::Imp => (0, 10, 0, 25, 10),
            Self::Balor => (25, 30, 10, -15, 0),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Infernal => "Infernal Demon",
            Self::Abyssal => "Abyssal Demon",
            Self::Succubus => "Succubus",
            Self::Imp => "Imp",
            Self::Balor => "Balor",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::Infernal => "Hell-born destroyers. Devastating fire magic",
            Self::Abyssal => "Void creatures. Tough, immune to darkness",
            Self::Succubus => "Seductive demons. Charm magic, high mana",
            Self::Imp => "Minor demons. Fast, can fly",
            Self::Balor => "Greater demons. Extreme power, slow berserker",
        }
    }
}

// ============================================================================
// BEASTKIN SUBSPECIES
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
enum BeastkinSubspecies {
    Wolfkin,        // Pack tactics, speed, howl
    Catkin,         // Night vision, agility, stealth
    Bearkin,        // High HP/STR, rage, hibernation heal
    Foxkin,         // Illusion magic, charm, cunning
    Lionkin,        // Leadership, roar fear, pride bonus
    Tigerkin,       // Ambush, critical strikes, solo hunter
    Serpentkin,     // Poison, constrict, heat sense
    Eaglekin,       // Flight, keen sight, dive attack
    Ratkin,         // Disease, swarm, scavenger
    Sharkkin,       // Blood frenzy, water combat, bite
}

impl BeastkinSubspecies {
    fn name(&self) -> &'static str {
        match self {
            Self::Wolfkin => "Wolfkin",
            Self::Catkin => "Catkin",
            Self::Bearkin => "Bearkin",
            Self::Foxkin => "Foxkin",
            Self::Lionkin => "Lionkin",
            Self::Tigerkin => "Tigerkin",
            Self::Serpentkin => "Serpentkin",
            Self::Eaglekin => "Eaglekin",
            Self::Ratkin => "Ratkin",
            Self::Sharkkin => "Sharkkin",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::Wolfkin => "Wolf-blooded beastkin with pack instincts",
            Self::Catkin => "Feline beastkin with nine lives",
            Self::Bearkin => "Bear-blooded beastkin with immense strength",
            Self::Foxkin => "Fox-blooded beastkin with illusion magic",
            Self::Lionkin => "Lion-blooded beastkin with royal presence",
            Self::Tigerkin => "Tiger-blooded beastkin with deadly ambush",
            Self::Serpentkin => "Serpent-blooded beastkin with constricting grip",
            Self::Eaglekin => "Eagle-blooded beastkin with flight abilities",
            Self::Ratkin => "Rat-blooded beastkin with plague abilities",
            Self::Sharkkin => "Shark-blooded beastkin with blood frenzy",
        }
    }

    fn stat_bonuses(&self) -> (i32, i32, i32, i32, i32) {
        match self {
            Self::Wolfkin => (15, 20, 10, 20, 5),
            Self::Catkin => (5, 15, 5, 30, 10),
            Self::Bearkin => (50, 30, 25, -10, 0),
            Self::Foxkin => (0, 10, 5, 20, 30),
            Self::Lionkin => (25, 25, 15, 10, 10),
            Self::Tigerkin => (20, 35, 10, 20, 5),
            Self::Serpentkin => (15, 20, 15, 15, 15),
            Self::Eaglekin => (10, 20, 5, 25, 10),
            Self::Ratkin => (10, 15, 5, 25, 15),
            Self::Sharkkin => (30, 35, 20, 15, 0),
        }
    }

    fn beast_ability(&self) -> &'static str {
        match self {
            Self::Wolfkin => "Pack Howl - Buff allies, fear enemies",
            Self::Catkin => "Nine Lives - Survive fatal blow once per floor",
            Self::Bearkin => "Hibernate - Full heal but skip turns",
            Self::Foxkin => "Fox Fire - Create illusion decoys",
            Self::Lionkin => "Roar - Stun and fear all enemies",
            Self::Tigerkin => "Ambush - First attack deals 3x damage",
            Self::Serpentkin => "Constrict - Immobilize and crush enemy",
            Self::Eaglekin => "Dive - Flying attack, ignore terrain",
            Self::Ratkin => "Plague Carrier - Spread disease",
            Self::Sharkkin => "Blood Frenzy - +damage vs wounded enemies",
        }
    }

    fn transformation(&self, level: u32) -> Option<&'static str> {
        if level >= 25 {
            Some(match self {
                Self::Wolfkin => "Alpha Wolf - Lead pack of wolves",
                Self::Catkin => "Shadow Cat - Permanent stealth",
                Self::Bearkin => "Werebear - Massive size and power",
                Self::Foxkin => "Nine-Tail - Ultimate illusion mastery",
                Self::Lionkin => "King of Beasts - Command all beasts",
                Self::Tigerkin => "White Tiger - Divine beast form",
                Self::Serpentkin => "Naga - Half-serpent spellcaster",
                Self::Eaglekin => "Thunderbird - Storm powers",
                Self::Ratkin => "Rat King - Control rat swarms",
                Self::Sharkkin => "Megalodon Form - Massive power",
            })
        } else {
            None
        }
    }
}

// ============================================================================
// TILE TYPES
// ============================================================================

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
enum Tile {
    Wall,
    Floor,
    StairsDown,
    StairsUp,
    Door,
    OpenDoor,
    Trap,
    DisarmedTrap,
    Water,
    Lava,
    Chest,
    OpenChest,
    Shrine,
    UsedShrine,
    Pillar,
    Grass,
    Ice,
    Sand,
    BossGate,
}

impl Tile {
    fn glyph(&self) -> char {
        match self {
            Self::Wall => '#',
            Self::Floor => '.',
            Self::StairsDown => '>',
            Self::StairsUp => '<',
            Self::Door => '+',
            Self::OpenDoor => '\'',
            Self::Trap => '^',
            Self::DisarmedTrap => '_',
            Self::Water => '~',
            Self::Lava => '~',
            Self::Chest => '=',
            Self::OpenChest => '-',
            Self::Shrine => '&',
            Self::UsedShrine => '.',
            Self::Pillar => 'O',
            Self::Grass => '"',
            Self::Ice => '.',
            Self::Sand => '.',
            Self::BossGate => '8',
        }
    }

    fn color(&self) -> Color {
        match self {
            Self::Wall => Color::DarkGrey,
            Self::Floor => Color::Grey,
            Self::StairsDown => Color::Cyan,
            Self::StairsUp => Color::Cyan,
            Self::Door => Color::Yellow,
            Self::OpenDoor => Color::DarkYellow,
            Self::Trap => Color::Red,
            Self::DisarmedTrap => Color::DarkGrey,
            Self::Water => Color::Blue,
            Self::Lava => Color::Red,
            Self::Chest => Color::Yellow,
            Self::OpenChest => Color::DarkYellow,
            Self::Shrine => Color::Magenta,
            Self::UsedShrine => Color::DarkMagenta,
            Self::Pillar => Color::White,
            Self::Grass => Color::Green,
            Self::Ice => Color::Cyan,
            Self::Sand => Color::Yellow,
            Self::BossGate => Color::Red,
        }
    }

    fn walkable(&self) -> bool {
        matches!(
            self,
            Self::Floor
                | Self::StairsDown
                | Self::StairsUp
                | Self::OpenDoor
                | Self::Trap
                | Self::DisarmedTrap
                | Self::Water
                | Self::Grass
                | Self::Ice
                | Self::Sand
                | Self::UsedShrine
                | Self::OpenChest
                | Self::BossGate
        )
    }

    fn blocks_sight(&self) -> bool {
        matches!(self, Self::Wall | Self::Door | Self::Pillar)
    }
}

// ============================================================================
// DUNGEON THEMES
// ============================================================================

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
enum DungeonTheme {
    Dungeon,
    Cave,
    Crypt,
    Forest,
    IceCavern,
    VolcanicLair,
    AncientRuins,
    DemonRealm,
}

impl DungeonTheme {
    fn from_level(level: u32) -> Self {
        match level {
            1..=4 => Self::Dungeon,
            5..=8 => Self::Cave,
            9..=12 => Self::Crypt,
            13..=16 => Self::Forest,
            17..=20 => Self::IceCavern,
            21..=24 => Self::VolcanicLair,
            25..=28 => Self::AncientRuins,
            _ => Self::DemonRealm,
        }
    }

    fn floor_tile(&self) -> Tile {
        match self {
            Self::Dungeon | Self::Cave | Self::Crypt => Tile::Floor,
            Self::Forest => Tile::Grass,
            Self::IceCavern => Tile::Ice,
            Self::AncientRuins => Tile::Sand,
            Self::VolcanicLair | Self::DemonRealm => Tile::Floor,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Dungeon => "Dark Dungeon",
            Self::Cave => "Twisted Caves",
            Self::Crypt => "Haunted Crypt",
            Self::Forest => "Cursed Forest",
            Self::IceCavern => "Frozen Caverns",
            Self::VolcanicLair => "Volcanic Depths",
            Self::AncientRuins => "Ancient Ruins",
            Self::DemonRealm => "Demon Realm",
        }
    }
}

// ============================================================================
// EQUIPMENT SLOTS
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum EquipSlot {
    Weapon,
    Shield,
    Helmet,
    Armor,
    Gloves,
    Boots,
    Ring1,
    Ring2,
    Amulet,
}

// ============================================================================
// ITEM RARITY
// ============================================================================

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

impl Rarity {
    fn color(&self) -> Color {
        match self {
            Self::Common => Color::Grey,
            Self::Uncommon => Color::Green,
            Self::Rare => Color::Blue,
            Self::Epic => Color::Magenta,
            Self::Legendary => Color::Yellow,
            Self::Mythic => Color::Red,
        }
    }

    fn prefix(&self) -> &'static str {
        match self {
            Self::Common => "",
            Self::Uncommon => "Fine ",
            Self::Rare => "Superior ",
            Self::Epic => "Epic ",
            Self::Legendary => "Legendary ",
            Self::Mythic => "Mythic ",
        }
    }

    fn stat_bonus(&self) -> f32 {
        match self {
            Self::Common => 1.0,
            Self::Uncommon => 1.25,
            Self::Rare => 1.5,
            Self::Epic => 2.0,
            Self::Legendary => 3.0,
            Self::Mythic => 5.0,
        }
    }

    fn rarity_rank(&self) -> u8 {
        match self {
            Self::Common => 0,
            Self::Uncommon => 1,
            Self::Rare => 2,
            Self::Epic => 3,
            Self::Legendary => 4,
            Self::Mythic => 5,
        }
    }
}

// ============================================================================
// ITEM TYPES
// ============================================================================

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
enum ItemKind {
    // Potions (20)
    HealthPotion,
    ManaPotion,
    StrengthPotion,
    DefensePotion,
    SpeedPotion,
    InvisibilityPotion,
    FireResistPotion,
    IceResistPotion,
    PoisonResistPotion,
    RegenerationPotion,
    BerserkPotion,
    GiantPotion,
    LevitationPotion,
    XPPotion,
    FullRestorePotion,
    LuckPotion,
    CriticalPotion,
    VisionPotion,
    CureAllPotion,
    UltimatePowerPotion,

    // Scrolls (18)
    ScrollTeleport,
    ScrollFireball,
    ScrollIceStorm,
    ScrollLightning,
    ScrollMapping,
    ScrollIdentify,
    ScrollEnchant,
    ScrollSummon,
    ScrollBanish,
    ScrollTimeStop,
    ScrollMassHeal,
    ScrollDeath,
    ScrollEarthquake,
    ScrollMeteor,
    ScrollBlizzard,
    ScrollChainLightning,
    ScrollDivineWrath,
    ScrollDarkness,

    // Weapons (25)
    Dagger,
    ShortSword,
    LongSword,
    Greatsword,
    Axe,
    BattleAxe,
    Mace,
    WarHammer,
    Spear,
    Halberd,
    Staff,
    Bow,
    Crossbow,
    Wand,
    Scythe,
    Katana,
    Rapier,
    Flail,
    Morningstar,
    Trident,
    FlameSword,
    FrostBlade,
    ThunderAxe,
    VoidStaff,
    DemonSlayer,

    // Shields (10)
    Buckler,
    WoodenShield,
    IronShield,
    TowerShield,
    MagicShield,
    DragonShield,
    SpikedShield,
    MirrorShield,
    PhoenixShield,
    AbyssalShield,

    // Armor (12)
    LeatherArmor,
    ChainMail,
    ScaleMail,
    PlateMail,
    DragonArmor,
    MageRobes,
    AssassinGarb,
    HolyArmor,
    DemonArmor,
    CrystalArmor,
    ShadowCloak,
    TitanPlate,

    // Helmets (10)
    LeatherCap,
    IronHelm,
    SteelHelm,
    CrownOfKings,
    WizardHat,
    DemonSkull,
    DragonHelm,
    CrystalCrown,
    HoodOfShadows,
    HelmOfValor,

    // Gloves (8)
    LeatherGloves,
    IronGauntlets,
    GlovesOfPower,
    ThievesGloves,
    DragonGauntlets,
    FrostGauntlets,
    FlameGauntlets,
    GauntletsOfMight,

    // Boots (8)
    LeatherBoots,
    IronBoots,
    BootsOfSpeed,
    BootsOfLeaping,
    WingedBoots,
    ShadowBoots,
    LavaWalkers,
    BootsOfTheWind,

    // Rings (15)
    RingOfStrength,
    RingOfProtection,
    RingOfSpeed,
    RingOfRegeneration,
    RingOfFireball,
    RingOfInvisibility,
    RingOfTheVampire,
    RingOfMana,
    RingOfLuck,
    RingOfDeath,
    RingOfFrost,
    RingOfFlame,
    RingOfThunder,
    RingOfShadows,
    RingOfTheAncients,

    // Amulets (12)
    AmuletOfHealth,
    AmuletOfMana,
    AmuletOfProtection,
    AmuletOfPower,
    AmuletOfWisdom,
    AmuletOfLife,
    AmuletOfDeath,
    AmuletOfTheGods,
    AmuletOfDragons,
    AmuletOfChaos,
    AmuletOfOrder,
    AmuletOfBalance,

    // Food (8)
    Bread,
    Meat,
    Apple,
    Cheese,
    Feast,
    DragonFruit,
    AncientWine,
    GoldenApple,

    // Special (10)
    Gold,
    Key,
    Bomb,
    Torch,
    Compass,
    TeleportCrystal,
    SoulGem,
    AncientRelic,
    DragonScale,
    DemonHeart,

    // Crafting Materials (25)
    IronOre,
    SteelIngot,
    LeatherStrip,
    DragonBlood,
    RedHerb,
    EmptyVial,
    MoonFlower,
    PhoenixFeather,
    UnicornHorn,
    ElixirOfLife,
    ManacrystalI,
    ManacrystalII,
    ManacrystalIII,
    BlankScroll,
    RawMeat,
    GoldBar,
    SilverBar,
    MithrilOre,
    RuneStone,
    EnchantedGem,
    FrostEssence,
    FireEssence,
    VoidEssence,
    AncientBone,
    CursedFabric,
}

impl ItemKind {
    fn glyph(&self) -> char {
        match self {
            // Potions
            Self::HealthPotion | Self::ManaPotion | Self::StrengthPotion
            | Self::DefensePotion | Self::SpeedPotion | Self::InvisibilityPotion
            | Self::FireResistPotion | Self::IceResistPotion | Self::PoisonResistPotion
            | Self::RegenerationPotion | Self::BerserkPotion | Self::GiantPotion
            | Self::LevitationPotion | Self::XPPotion | Self::FullRestorePotion
            | Self::LuckPotion | Self::CriticalPotion | Self::VisionPotion
            | Self::CureAllPotion | Self::UltimatePowerPotion => '!',

            // Scrolls
            Self::ScrollTeleport | Self::ScrollFireball | Self::ScrollIceStorm
            | Self::ScrollLightning | Self::ScrollMapping | Self::ScrollIdentify
            | Self::ScrollEnchant | Self::ScrollSummon | Self::ScrollBanish
            | Self::ScrollTimeStop | Self::ScrollMassHeal | Self::ScrollDeath
            | Self::ScrollEarthquake | Self::ScrollMeteor | Self::ScrollBlizzard
            | Self::ScrollChainLightning | Self::ScrollDivineWrath | Self::ScrollDarkness => '?',

            // Weapons
            Self::Dagger | Self::Rapier => '|',
            Self::ShortSword | Self::LongSword | Self::Greatsword | Self::Katana => '/',
            Self::Axe | Self::BattleAxe | Self::ThunderAxe => 'P',
            Self::Mace | Self::WarHammer | Self::Flail | Self::Morningstar => 'T',
            Self::Spear | Self::Halberd | Self::Trident => '|',
            Self::Staff | Self::Wand | Self::VoidStaff => '/',
            Self::Bow | Self::Crossbow => '}',
            Self::Scythe | Self::DemonSlayer => '7',
            Self::FlameSword | Self::FrostBlade => '/',

            // Shields
            Self::Buckler | Self::WoodenShield | Self::IronShield | Self::TowerShield
            | Self::MagicShield | Self::DragonShield | Self::SpikedShield
            | Self::MirrorShield | Self::PhoenixShield | Self::AbyssalShield => ')',

            // Armor
            Self::LeatherArmor | Self::ChainMail | Self::ScaleMail | Self::PlateMail
            | Self::DragonArmor | Self::MageRobes | Self::AssassinGarb | Self::HolyArmor
            | Self::DemonArmor | Self::CrystalArmor | Self::ShadowCloak | Self::TitanPlate => '[',

            // Helmets
            Self::LeatherCap | Self::IronHelm | Self::SteelHelm | Self::CrownOfKings
            | Self::WizardHat | Self::DemonSkull | Self::DragonHelm | Self::CrystalCrown
            | Self::HoodOfShadows | Self::HelmOfValor => '^',

            // Gloves
            Self::LeatherGloves | Self::IronGauntlets | Self::GlovesOfPower
            | Self::ThievesGloves | Self::DragonGauntlets | Self::FrostGauntlets
            | Self::FlameGauntlets | Self::GauntletsOfMight => '{',

            // Boots
            Self::LeatherBoots | Self::IronBoots | Self::BootsOfSpeed
            | Self::BootsOfLeaping | Self::WingedBoots | Self::ShadowBoots
            | Self::LavaWalkers | Self::BootsOfTheWind => '}',

            // Rings
            Self::RingOfStrength | Self::RingOfProtection | Self::RingOfSpeed
            | Self::RingOfRegeneration | Self::RingOfFireball | Self::RingOfInvisibility
            | Self::RingOfTheVampire | Self::RingOfMana | Self::RingOfLuck | Self::RingOfDeath
            | Self::RingOfFrost | Self::RingOfFlame | Self::RingOfThunder
            | Self::RingOfShadows | Self::RingOfTheAncients => 'o',

            // Amulets
            Self::AmuletOfHealth | Self::AmuletOfMana | Self::AmuletOfProtection
            | Self::AmuletOfPower | Self::AmuletOfWisdom | Self::AmuletOfLife
            | Self::AmuletOfDeath | Self::AmuletOfTheGods | Self::AmuletOfDragons
            | Self::AmuletOfChaos | Self::AmuletOfOrder | Self::AmuletOfBalance => '"',

            // Food
            Self::Bread | Self::Meat | Self::Apple | Self::Cheese | Self::Feast
            | Self::DragonFruit | Self::AncientWine | Self::GoldenApple => '%',

            // Special
            Self::Gold => '$',
            Self::Key => 'k',
            Self::Bomb => '*',
            Self::Torch => '(',
            Self::Compass => 'c',
            Self::TeleportCrystal => '+',
            Self::SoulGem => 'o',
            Self::AncientRelic => '*',
            Self::DragonScale => 's',
            Self::DemonHeart => 'h',

            // Crafting materials and other items
            _ => '*',
        }
    }

    #[allow(dead_code)]
    fn color(&self) -> Color {
        match self {
            Self::HealthPotion | Self::FullRestorePotion => Color::Red,
            Self::ManaPotion => Color::Blue,
            Self::StrengthPotion | Self::BerserkPotion => Color::Yellow,
            Self::DefensePotion => Color::Cyan,
            Self::SpeedPotion => Color::Magenta,
            Self::InvisibilityPotion => Color::Grey,
            Self::FireResistPotion => Color::DarkRed,
            Self::IceResistPotion => Color::DarkCyan,
            Self::PoisonResistPotion => Color::Green,
            Self::RegenerationPotion => Color::Magenta,
            Self::GiantPotion => Color::Yellow,
            Self::LevitationPotion => Color::White,
            Self::XPPotion => Color::Cyan,

            Self::ScrollTeleport => Color::Blue,
            Self::ScrollFireball => Color::Red,
            Self::ScrollIceStorm => Color::Cyan,
            Self::ScrollLightning => Color::Yellow,
            Self::ScrollMapping => Color::White,
            Self::ScrollIdentify => Color::Grey,
            Self::ScrollEnchant => Color::Magenta,
            Self::ScrollSummon => Color::Green,
            Self::ScrollBanish => Color::DarkMagenta,
            Self::ScrollTimeStop => Color::DarkCyan,
            Self::ScrollMassHeal => Color::Red,
            Self::ScrollDeath => Color::DarkRed,

            Self::Gold => Color::Yellow,
            Self::Key => Color::Yellow,
            Self::Bomb => Color::Red,
            Self::Torch => Color::Yellow,
            Self::Compass => Color::Cyan,

            _ => Color::White,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::HealthPotion => "Health Potion",
            Self::ManaPotion => "Mana Potion",
            Self::StrengthPotion => "Strength Potion",
            Self::DefensePotion => "Defense Potion",
            Self::SpeedPotion => "Speed Potion",
            Self::InvisibilityPotion => "Invisibility Potion",
            Self::FireResistPotion => "Fire Resist Potion",
            Self::IceResistPotion => "Ice Resist Potion",
            Self::PoisonResistPotion => "Antidote",
            Self::RegenerationPotion => "Regeneration Potion",
            Self::BerserkPotion => "Berserk Potion",
            Self::GiantPotion => "Giant's Strength",
            Self::LevitationPotion => "Levitation Potion",
            Self::XPPotion => "Potion of Experience",
            Self::FullRestorePotion => "Full Restore Elixir",
            Self::LuckPotion => "Luck Potion",
            Self::CriticalPotion => "Critical Strike Potion",
            Self::VisionPotion => "Potion of True Sight",
            Self::CureAllPotion => "Cure All Elixir",
            Self::UltimatePowerPotion => "Ultimate Power Elixir",

            Self::ScrollTeleport => "Scroll of Teleport",
            Self::ScrollFireball => "Scroll of Fireball",
            Self::ScrollIceStorm => "Scroll of Ice Storm",
            Self::ScrollLightning => "Scroll of Lightning",
            Self::ScrollMapping => "Scroll of Mapping",
            Self::ScrollIdentify => "Scroll of Identify",
            Self::ScrollEnchant => "Scroll of Enchant",
            Self::ScrollSummon => "Scroll of Summoning",
            Self::ScrollBanish => "Scroll of Banishment",
            Self::ScrollTimeStop => "Scroll of Time Stop",
            Self::ScrollMassHeal => "Scroll of Mass Heal",
            Self::ScrollDeath => "Scroll of Death",
            Self::ScrollEarthquake => "Scroll of Earthquake",
            Self::ScrollMeteor => "Scroll of Meteor",
            Self::ScrollBlizzard => "Scroll of Blizzard",
            Self::ScrollChainLightning => "Scroll of Chain Lightning",
            Self::ScrollDivineWrath => "Scroll of Divine Wrath",
            Self::ScrollDarkness => "Scroll of Darkness",

            Self::Dagger => "Dagger",
            Self::ShortSword => "Short Sword",
            Self::LongSword => "Long Sword",
            Self::Greatsword => "Greatsword",
            Self::Axe => "Battle Axe",
            Self::BattleAxe => "Great Axe",
            Self::Mace => "Mace",
            Self::WarHammer => "War Hammer",
            Self::Spear => "Spear",
            Self::Halberd => "Halberd",
            Self::Staff => "Staff",
            Self::Bow => "Bow",
            Self::Crossbow => "Crossbow",
            Self::Wand => "Wand",
            Self::Scythe => "Scythe",
            Self::Katana => "Katana",
            Self::Rapier => "Rapier",
            Self::Flail => "Flail",
            Self::Morningstar => "Morningstar",
            Self::Trident => "Trident",
            Self::FlameSword => "Flame Sword",
            Self::FrostBlade => "Frost Blade",
            Self::ThunderAxe => "Thunder Axe",
            Self::VoidStaff => "Void Staff",
            Self::DemonSlayer => "Demon Slayer",

            Self::Buckler => "Buckler",
            Self::WoodenShield => "Wooden Shield",
            Self::IronShield => "Iron Shield",
            Self::TowerShield => "Tower Shield",
            Self::MagicShield => "Magic Shield",
            Self::DragonShield => "Dragon Shield",
            Self::SpikedShield => "Spiked Shield",
            Self::MirrorShield => "Mirror Shield",
            Self::PhoenixShield => "Phoenix Shield",
            Self::AbyssalShield => "Abyssal Shield",

            Self::LeatherArmor => "Leather Armor",
            Self::ChainMail => "Chain Mail",
            Self::ScaleMail => "Scale Mail",
            Self::PlateMail => "Plate Mail",
            Self::DragonArmor => "Dragon Armor",
            Self::MageRobes => "Mage Robes",
            Self::AssassinGarb => "Assassin's Garb",
            Self::HolyArmor => "Holy Armor",
            Self::DemonArmor => "Demon Armor",
            Self::CrystalArmor => "Crystal Armor",
            Self::ShadowCloak => "Shadow Cloak",
            Self::TitanPlate => "Titan Plate",

            Self::LeatherCap => "Leather Cap",
            Self::IronHelm => "Iron Helm",
            Self::SteelHelm => "Steel Helm",
            Self::CrownOfKings => "Crown of Kings",
            Self::WizardHat => "Wizard's Hat",
            Self::DemonSkull => "Demon Skull",
            Self::DragonHelm => "Dragon Helm",
            Self::CrystalCrown => "Crystal Crown",
            Self::HoodOfShadows => "Hood of Shadows",
            Self::HelmOfValor => "Helm of Valor",

            Self::LeatherGloves => "Leather Gloves",
            Self::IronGauntlets => "Iron Gauntlets",
            Self::GlovesOfPower => "Gloves of Power",
            Self::ThievesGloves => "Thief's Gloves",
            Self::DragonGauntlets => "Dragon Gauntlets",
            Self::FrostGauntlets => "Frost Gauntlets",
            Self::FlameGauntlets => "Flame Gauntlets",
            Self::GauntletsOfMight => "Gauntlets of Might",

            Self::LeatherBoots => "Leather Boots",
            Self::IronBoots => "Iron Boots",
            Self::BootsOfSpeed => "Boots of Speed",
            Self::BootsOfLeaping => "Boots of Leaping",
            Self::WingedBoots => "Winged Boots",
            Self::ShadowBoots => "Shadow Boots",
            Self::LavaWalkers => "Lava Walkers",
            Self::BootsOfTheWind => "Boots of the Wind",

            Self::RingOfStrength => "Ring of Strength",
            Self::RingOfProtection => "Ring of Protection",
            Self::RingOfSpeed => "Ring of Speed",
            Self::RingOfRegeneration => "Ring of Regeneration",
            Self::RingOfFireball => "Ring of Fireball",
            Self::RingOfInvisibility => "Ring of Invisibility",
            Self::RingOfTheVampire => "Vampire Ring",
            Self::RingOfMana => "Ring of Mana",
            Self::RingOfLuck => "Ring of Luck",
            Self::RingOfDeath => "Ring of Death",
            Self::RingOfFrost => "Ring of Frost",
            Self::RingOfFlame => "Ring of Flame",
            Self::RingOfThunder => "Ring of Thunder",
            Self::RingOfShadows => "Ring of Shadows",
            Self::RingOfTheAncients => "Ring of the Ancients",

            Self::AmuletOfHealth => "Amulet of Health",
            Self::AmuletOfMana => "Amulet of Mana",
            Self::AmuletOfProtection => "Amulet of Protection",
            Self::AmuletOfPower => "Amulet of Power",
            Self::AmuletOfWisdom => "Amulet of Wisdom",
            Self::AmuletOfLife => "Amulet of Life",
            Self::AmuletOfDeath => "Amulet of Death",
            Self::AmuletOfTheGods => "Amulet of the Gods",
            Self::AmuletOfDragons => "Amulet of Dragons",
            Self::AmuletOfChaos => "Amulet of Chaos",
            Self::AmuletOfOrder => "Amulet of Order",
            Self::AmuletOfBalance => "Amulet of Balance",

            Self::Bread => "Bread",
            Self::Meat => "Meat",
            Self::Apple => "Apple",
            Self::Cheese => "Cheese",
            Self::Feast => "Royal Feast",
            Self::DragonFruit => "Dragon Fruit",
            Self::AncientWine => "Ancient Wine",
            Self::GoldenApple => "Golden Apple",

            Self::Gold => "Gold",
            Self::Key => "Key",
            Self::Bomb => "Bomb",
            Self::Torch => "Torch",
            Self::Compass => "Compass",
            Self::TeleportCrystal => "Teleport Crystal",
            Self::SoulGem => "Soul Gem",
            Self::AncientRelic => "Ancient Relic",
            Self::DragonScale => "Dragon Scale",
            Self::DemonHeart => "Demon Heart",

            // Crafting materials
            Self::IronOre => "Iron Ore",
            Self::SteelIngot => "Steel Ingot",
            Self::LeatherStrip => "Leather Strip",
            Self::DragonBlood => "Dragon Blood",
            Self::RedHerb => "Red Herb",
            Self::EmptyVial => "Empty Vial",
            Self::MoonFlower => "Moon Flower",
            Self::PhoenixFeather => "Phoenix Feather",
            Self::UnicornHorn => "Unicorn Horn",
            Self::ElixirOfLife => "Elixir of Life",
            Self::ManacrystalI => "Mana Crystal I",
            Self::ManacrystalII => "Mana Crystal II",
            Self::ManacrystalIII => "Mana Crystal III",
            Self::BlankScroll => "Blank Scroll",
            Self::RawMeat => "Raw Meat",
            Self::GoldBar => "Gold Bar",
            Self::SilverBar => "Silver Bar",
            Self::MithrilOre => "Mithril Ore",
            Self::RuneStone => "Rune Stone",
            Self::EnchantedGem => "Enchanted Gem",
            Self::FrostEssence => "Frost Essence",
            Self::FireEssence => "Fire Essence",
            Self::VoidEssence => "Void Essence",
            Self::AncientBone => "Ancient Bone",
            Self::CursedFabric => "Cursed Fabric",
        }
    }

    fn equip_slot(&self) -> Option<EquipSlot> {
        match self {
            Self::Dagger | Self::ShortSword | Self::LongSword | Self::Greatsword
            | Self::Axe | Self::BattleAxe | Self::Mace | Self::WarHammer
            | Self::Spear | Self::Halberd | Self::Staff | Self::Bow
            | Self::Crossbow | Self::Wand | Self::Scythe
            | Self::Katana | Self::Rapier | Self::Flail | Self::Morningstar
            | Self::Trident | Self::FlameSword | Self::FrostBlade | Self::ThunderAxe
            | Self::VoidStaff | Self::DemonSlayer => Some(EquipSlot::Weapon),

            Self::Buckler | Self::WoodenShield | Self::IronShield
            | Self::TowerShield | Self::MagicShield | Self::DragonShield
            | Self::SpikedShield | Self::MirrorShield | Self::PhoenixShield
            | Self::AbyssalShield => Some(EquipSlot::Shield),

            Self::LeatherArmor | Self::ChainMail | Self::ScaleMail | Self::PlateMail
            | Self::DragonArmor | Self::MageRobes | Self::AssassinGarb
            | Self::HolyArmor | Self::DemonArmor | Self::CrystalArmor
            | Self::ShadowCloak | Self::TitanPlate => Some(EquipSlot::Armor),

            Self::LeatherCap | Self::IronHelm | Self::SteelHelm
            | Self::CrownOfKings | Self::WizardHat | Self::DemonSkull
            | Self::DragonHelm | Self::CrystalCrown | Self::HoodOfShadows
            | Self::HelmOfValor => Some(EquipSlot::Helmet),

            Self::LeatherGloves | Self::IronGauntlets | Self::GlovesOfPower
            | Self::ThievesGloves | Self::DragonGauntlets
            | Self::FrostGauntlets | Self::FlameGauntlets | Self::GauntletsOfMight => Some(EquipSlot::Gloves),

            Self::LeatherBoots | Self::IronBoots | Self::BootsOfSpeed
            | Self::BootsOfLeaping | Self::WingedBoots
            | Self::ShadowBoots | Self::LavaWalkers | Self::BootsOfTheWind => Some(EquipSlot::Boots),

            Self::RingOfStrength | Self::RingOfProtection | Self::RingOfSpeed
            | Self::RingOfRegeneration | Self::RingOfFireball | Self::RingOfInvisibility
            | Self::RingOfTheVampire | Self::RingOfMana | Self::RingOfLuck
            | Self::RingOfDeath | Self::RingOfFrost | Self::RingOfFlame
            | Self::RingOfThunder | Self::RingOfShadows | Self::RingOfTheAncients => Some(EquipSlot::Ring1),

            Self::AmuletOfHealth | Self::AmuletOfMana | Self::AmuletOfProtection
            | Self::AmuletOfPower | Self::AmuletOfWisdom | Self::AmuletOfLife
            | Self::AmuletOfDeath | Self::AmuletOfTheGods
            | Self::AmuletOfDragons | Self::AmuletOfChaos | Self::AmuletOfOrder
            | Self::AmuletOfBalance => Some(EquipSlot::Amulet),

            _ => None,
        }
    }

    fn base_stats(&self) -> (i32, i32, i32, i32) {
        // (attack, defense, hp_bonus, mana_bonus)
        match self {
            // Weapons
            Self::Dagger => (3, 0, 0, 0),
            Self::ShortSword => (5, 0, 0, 0),
            Self::LongSword => (8, 0, 0, 0),
            Self::Greatsword => (12, 0, 0, 0),
            Self::Axe => (7, 0, 0, 0),
            Self::BattleAxe => (14, 0, 0, 0),
            Self::Mace => (6, 1, 0, 0),
            Self::WarHammer => (10, 2, 0, 0),
            Self::Spear => (6, 0, 0, 0),
            Self::Halberd => (11, 1, 0, 0),
            Self::Staff => (4, 0, 0, 20),
            Self::Bow => (7, 0, 0, 0),
            Self::Crossbow => (10, 0, 0, 0),
            Self::Wand => (3, 0, 0, 30),
            Self::Scythe => (15, 0, 0, 0),
            Self::Katana => (11, 0, 0, 0),
            Self::Rapier => (9, 1, 0, 0),
            Self::Flail => (10, 0, 0, 0),
            Self::Morningstar => (12, 1, 0, 0),
            Self::Trident => (10, 2, 0, 0),
            Self::FlameSword => (14, 0, 0, 5),
            Self::FrostBlade => (13, 0, 0, 5),
            Self::ThunderAxe => (16, 0, 0, 10),
            Self::VoidStaff => (8, 0, 0, 50),
            Self::DemonSlayer => (20, 0, 0, 0),

            // Shields
            Self::Buckler => (0, 2, 0, 0),
            Self::WoodenShield => (0, 3, 0, 0),
            Self::IronShield => (0, 5, 0, 0),
            Self::TowerShield => (0, 8, 0, 0),
            Self::MagicShield => (0, 6, 0, 10),
            Self::DragonShield => (0, 10, 10, 0),
            Self::SpikedShield => (3, 6, 0, 0),
            Self::MirrorShield => (0, 7, 0, 15),
            Self::PhoenixShield => (0, 9, 15, 5),
            Self::AbyssalShield => (2, 12, 0, 10),

            // Armor
            Self::LeatherArmor => (0, 3, 0, 0),
            Self::ChainMail => (0, 5, 0, 0),
            Self::ScaleMail => (0, 7, 0, 0),
            Self::PlateMail => (0, 10, 0, 0),
            Self::DragonArmor => (0, 15, 20, 0),
            Self::MageRobes => (0, 2, 0, 30),
            Self::AssassinGarb => (3, 4, 0, 0),
            Self::HolyArmor => (0, 12, 10, 10),
            Self::DemonArmor => (5, 14, 0, 0),
            Self::CrystalArmor => (0, 11, 0, 25),
            Self::ShadowCloak => (3, 6, 0, 15),
            Self::TitanPlate => (0, 18, 30, 0),

            // Helmets
            Self::LeatherCap => (0, 1, 0, 0),
            Self::IronHelm => (0, 3, 0, 0),
            Self::SteelHelm => (0, 5, 0, 0),
            Self::CrownOfKings => (2, 3, 20, 20),
            Self::WizardHat => (0, 1, 0, 20),
            Self::DemonSkull => (5, 2, 0, 0),
            Self::DragonHelm => (2, 6, 10, 0),
            Self::CrystalCrown => (0, 4, 10, 25),
            Self::HoodOfShadows => (2, 2, 0, 15),
            Self::HelmOfValor => (3, 5, 15, 0),

            // Gloves
            Self::LeatherGloves => (1, 0, 0, 0),
            Self::IronGauntlets => (2, 1, 0, 0),
            Self::GlovesOfPower => (5, 0, 0, 0),
            Self::ThievesGloves => (3, 0, 0, 0),
            Self::DragonGauntlets => (4, 3, 0, 0),
            Self::FrostGauntlets => (3, 2, 0, 10),
            Self::FlameGauntlets => (5, 1, 0, 5),
            Self::GauntletsOfMight => (7, 2, 5, 0),

            // Boots
            Self::LeatherBoots => (0, 1, 0, 0),
            Self::IronBoots => (0, 2, 0, 0),
            Self::BootsOfSpeed => (0, 1, 0, 0),
            Self::BootsOfLeaping => (0, 1, 0, 0),
            Self::WingedBoots => (0, 2, 0, 10),
            Self::ShadowBoots => (2, 1, 0, 10),
            Self::LavaWalkers => (0, 3, 0, 0),
            Self::BootsOfTheWind => (0, 2, 0, 15),

            // Rings
            Self::RingOfStrength => (5, 0, 0, 0),
            Self::RingOfProtection => (0, 5, 0, 0),
            Self::RingOfSpeed => (0, 0, 0, 0),
            Self::RingOfRegeneration => (0, 0, 10, 0),
            Self::RingOfFireball => (3, 0, 0, 10),
            Self::RingOfInvisibility => (0, 0, 0, 0),
            Self::RingOfTheVampire => (3, 0, 0, 0),
            Self::RingOfMana => (0, 0, 0, 30),
            Self::RingOfLuck => (1, 1, 5, 5),
            Self::RingOfDeath => (10, 0, -20, 0),
            Self::RingOfFrost => (2, 0, 0, 15),
            Self::RingOfFlame => (4, 0, 0, 10),
            Self::RingOfThunder => (5, 0, 0, 20),
            Self::RingOfShadows => (0, 2, 0, 20),
            Self::RingOfTheAncients => (6, 3, 15, 15),

            // Amulets
            Self::AmuletOfHealth => (0, 0, 30, 0),
            Self::AmuletOfMana => (0, 0, 0, 40),
            Self::AmuletOfProtection => (0, 8, 0, 0),
            Self::AmuletOfPower => (8, 0, 0, 0),
            Self::AmuletOfWisdom => (0, 0, 0, 50),
            Self::AmuletOfLife => (0, 0, 50, 0),
            Self::AmuletOfDeath => (15, 0, -30, 0),
            Self::AmuletOfTheGods => (5, 5, 25, 25),
            Self::AmuletOfDragons => (8, 4, 20, 10),
            Self::AmuletOfChaos => (12, 0, -10, 30),
            Self::AmuletOfOrder => (0, 10, 20, 20),
            Self::AmuletOfBalance => (6, 6, 20, 20),

            _ => (0, 0, 0, 0),
        }
    }

    fn is_consumable(&self) -> bool {
        matches!(
            self,
            Self::HealthPotion | Self::ManaPotion | Self::StrengthPotion
                | Self::DefensePotion | Self::SpeedPotion | Self::InvisibilityPotion
                | Self::FireResistPotion | Self::IceResistPotion | Self::PoisonResistPotion
                | Self::RegenerationPotion | Self::BerserkPotion | Self::GiantPotion
                | Self::LevitationPotion | Self::XPPotion | Self::FullRestorePotion
                | Self::LuckPotion | Self::CriticalPotion | Self::VisionPotion
                | Self::CureAllPotion | Self::UltimatePowerPotion
                | Self::ScrollTeleport | Self::ScrollFireball | Self::ScrollIceStorm
                | Self::ScrollLightning | Self::ScrollMapping | Self::ScrollIdentify
                | Self::ScrollEnchant | Self::ScrollSummon | Self::ScrollBanish
                | Self::ScrollTimeStop | Self::ScrollMassHeal | Self::ScrollDeath
                | Self::ScrollEarthquake | Self::ScrollMeteor | Self::ScrollBlizzard
                | Self::ScrollChainLightning | Self::ScrollDivineWrath | Self::ScrollDarkness
                | Self::Bread | Self::Meat | Self::Apple | Self::Cheese | Self::Feast
                | Self::DragonFruit | Self::AncientWine | Self::GoldenApple
                | Self::Bomb | Self::Torch
                | Self::TeleportCrystal | Self::SoulGem | Self::AncientRelic
                | Self::DragonScale | Self::DemonHeart
        )
    }

    fn is_food(&self) -> bool {
        matches!(
            self,
            Self::Bread | Self::Meat | Self::Apple | Self::Cheese | Self::Feast
                | Self::DragonFruit | Self::AncientWine | Self::GoldenApple
        )
    }

    fn food_value(&self) -> i32 {
        match self {
            Self::Apple => 10,
            Self::Bread => 25,
            Self::Cheese => 20,
            Self::Meat => 40,
            Self::Feast => 100,
            Self::DragonFruit => 30,
            Self::AncientWine => 35,
            Self::GoldenApple => 50,
            _ => 0,
        }
    }
}

// ============================================================================
// ADVANCED WEAPON SYSTEM
// ============================================================================

/// Weapon Type Categories - 35+ weapon types organized by class
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
enum WeaponType {
    // Swords (8 types)
    ShortSword,
    LongSword,
    Greatsword,
    Katana,
    Rapier,
    Scimitar,
    Claymore,
    Falchion,

    // Axes (5 types)
    HandAxe,
    BattleAxe,
    GreatAxe,
    ThrowingAxe,
    DoubleAxe,

    // Maces & Hammers (5 types)
    Mace,
    WarHammer,
    Morningstar,
    Flail,
    Maul,

    // Polearms (5 types)
    Spear,
    Halberd,
    Pike,
    Glaive,
    Trident,

    // Daggers & Short Blades (4 types)
    Dagger,
    Stiletto,
    Kris,
    MainGauche,

    // Ranged (4 types)
    ShortBow,
    LongBow,
    Crossbow,
    CompositeBow,

    // Staves & Wands (4 types)
    Staff,
    Quarterstaff,
    Wand,
    Scepter,
}

impl WeaponType {
    fn name(&self) -> &'static str {
        match self {
            Self::ShortSword => "Short Sword",
            Self::LongSword => "Long Sword",
            Self::Greatsword => "Greatsword",
            Self::Katana => "Katana",
            Self::Rapier => "Rapier",
            Self::Scimitar => "Scimitar",
            Self::Claymore => "Claymore",
            Self::Falchion => "Falchion",
            Self::HandAxe => "Hand Axe",
            Self::BattleAxe => "Battle Axe",
            Self::GreatAxe => "Great Axe",
            Self::ThrowingAxe => "Throwing Axe",
            Self::DoubleAxe => "Double Axe",
            Self::Mace => "Mace",
            Self::WarHammer => "War Hammer",
            Self::Morningstar => "Morningstar",
            Self::Flail => "Flail",
            Self::Maul => "Maul",
            Self::Spear => "Spear",
            Self::Halberd => "Halberd",
            Self::Pike => "Pike",
            Self::Glaive => "Glaive",
            Self::Trident => "Trident",
            Self::Dagger => "Dagger",
            Self::Stiletto => "Stiletto",
            Self::Kris => "Kris",
            Self::MainGauche => "Main Gauche",
            Self::ShortBow => "Short Bow",
            Self::LongBow => "Long Bow",
            Self::Crossbow => "Crossbow",
            Self::CompositeBow => "Composite Bow",
            Self::Staff => "Staff",
            Self::Quarterstaff => "Quarterstaff",
            Self::Wand => "Wand",
            Self::Scepter => "Scepter",
        }
    }

    fn category(&self) -> WeaponCategory {
        match self {
            Self::ShortSword | Self::LongSword | Self::Greatsword | Self::Katana
            | Self::Rapier | Self::Scimitar | Self::Claymore | Self::Falchion => WeaponCategory::Sword,
            Self::HandAxe | Self::BattleAxe | Self::GreatAxe | Self::ThrowingAxe
            | Self::DoubleAxe => WeaponCategory::Axe,
            Self::Mace | Self::WarHammer | Self::Morningstar | Self::Flail
            | Self::Maul => WeaponCategory::Blunt,
            Self::Spear | Self::Halberd | Self::Pike | Self::Glaive
            | Self::Trident => WeaponCategory::Polearm,
            Self::Dagger | Self::Stiletto | Self::Kris | Self::MainGauche => WeaponCategory::Dagger,
            Self::ShortBow | Self::LongBow | Self::Crossbow | Self::CompositeBow => WeaponCategory::Ranged,
            Self::Staff | Self::Quarterstaff | Self::Wand | Self::Scepter => WeaponCategory::Magic,
        }
    }

    fn grip_type(&self) -> WeaponGrip {
        match self {
            // Two-handed weapons
            Self::Greatsword | Self::Claymore | Self::GreatAxe | Self::DoubleAxe
            | Self::Maul | Self::Pike | Self::Halberd | Self::Glaive
            | Self::LongBow | Self::CompositeBow | Self::Staff | Self::Quarterstaff => WeaponGrip::TwoHanded,
            // Versatile (can be used one or two handed)
            Self::LongSword | Self::Katana | Self::BattleAxe | Self::WarHammer
            | Self::Spear | Self::Trident => WeaponGrip::Versatile,
            // One-handed weapons
            _ => WeaponGrip::OneHanded,
        }
    }

    fn base_damage(&self) -> (i32, i32) {
        // (min_damage, max_damage)
        match self {
            // Swords
            Self::ShortSword => (3, 7),
            Self::LongSword => (5, 10),
            Self::Greatsword => (8, 16),
            Self::Katana => (6, 12),
            Self::Rapier => (4, 9),
            Self::Scimitar => (5, 9),
            Self::Claymore => (10, 18),
            Self::Falchion => (5, 11),
            // Axes
            Self::HandAxe => (4, 8),
            Self::BattleAxe => (6, 14),
            Self::GreatAxe => (10, 20),
            Self::ThrowingAxe => (3, 7),
            Self::DoubleAxe => (12, 22),
            // Blunt
            Self::Mace => (4, 9),
            Self::WarHammer => (7, 15),
            Self::Morningstar => (5, 12),
            Self::Flail => (4, 11),
            Self::Maul => (12, 24),
            // Polearms
            Self::Spear => (4, 10),
            Self::Halberd => (8, 16),
            Self::Pike => (6, 14),
            Self::Glaive => (7, 15),
            Self::Trident => (5, 13),
            // Daggers
            Self::Dagger => (2, 5),
            Self::Stiletto => (3, 6),
            Self::Kris => (3, 7),
            Self::MainGauche => (2, 5),
            // Ranged
            Self::ShortBow => (3, 8),
            Self::LongBow => (5, 12),
            Self::Crossbow => (6, 14),
            Self::CompositeBow => (7, 15),
            // Magic
            Self::Staff => (2, 6),
            Self::Quarterstaff => (3, 8),
            Self::Wand => (1, 4),
            Self::Scepter => (2, 5),
        }
    }

    fn base_speed(&self) -> i32 {
        // Higher = faster attacks, range -5 to +5
        match self {
            Self::Dagger | Self::Stiletto | Self::Kris | Self::MainGauche => 5,
            Self::Wand | Self::Rapier => 4,
            Self::ShortSword | Self::Scimitar | Self::ShortBow => 3,
            Self::Katana | Self::Falchion | Self::ThrowingAxe => 2,
            Self::LongSword | Self::Mace | Self::Spear | Self::Scepter => 1,
            Self::BattleAxe | Self::Morningstar | Self::Trident | Self::LongBow => 0,
            Self::WarHammer | Self::Flail | Self::Halberd | Self::Crossbow => -1,
            Self::Greatsword | Self::Pike | Self::Glaive | Self::Staff => -2,
            Self::Claymore | Self::GreatAxe | Self::CompositeBow => -3,
            Self::DoubleAxe | Self::Maul | Self::Quarterstaff => -4,
            _ => 0,
        }
    }

    fn crit_chance_bonus(&self) -> i32 {
        // Bonus crit chance percentage
        match self {
            Self::Dagger | Self::Stiletto | Self::Kris => 15,
            Self::Rapier | Self::Katana => 10,
            Self::Scimitar | Self::Falchion => 8,
            Self::ShortSword | Self::LongSword | Self::MainGauche => 5,
            Self::Greatsword | Self::Claymore | Self::GreatAxe | Self::Maul => 3,
            Self::Crossbow => 8,
            _ => 2,
        }
    }
}

/// Weapon Category for mastery grouping
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
enum WeaponCategory {
    Sword,
    Axe,
    Blunt,
    Polearm,
    Dagger,
    Ranged,
    Magic,
}

impl WeaponCategory {
    fn name(&self) -> &'static str {
        match self {
            Self::Sword => "Swords",
            Self::Axe => "Axes",
            Self::Blunt => "Blunt Weapons",
            Self::Polearm => "Polearms",
            Self::Dagger => "Daggers",
            Self::Ranged => "Ranged Weapons",
            Self::Magic => "Magic Implements",
        }
    }
}

/// Weapon Grip Type
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
enum WeaponGrip {
    OneHanded,
    TwoHanded,
    Versatile,  // Can be used one or two handed
}

impl WeaponGrip {
    fn name(&self) -> &'static str {
        match self {
            Self::OneHanded => "One-Handed",
            Self::TwoHanded => "Two-Handed",
            Self::Versatile => "Versatile",
        }
    }

    fn allows_shield(&self) -> bool {
        matches!(self, Self::OneHanded | Self::Versatile)
    }

    fn allows_dual_wield(&self) -> bool {
        matches!(self, Self::OneHanded)
    }
}

/// Weapon Material - affects base stats and appearance
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
enum WeaponMaterial {
    Wood,
    Bone,
    Stone,
    Copper,
    Bronze,
    Iron,
    Steel,
    DarkSteel,
    Mithril,
    Adamantite,
    Obsidian,
    Crystal,
    Dragonbone,
    Demonic,
    Divine,
    Void,
    Astral,
}

impl WeaponMaterial {
    fn name(&self) -> &'static str {
        match self {
            Self::Wood => "Wooden",
            Self::Bone => "Bone",
            Self::Stone => "Stone",
            Self::Copper => "Copper",
            Self::Bronze => "Bronze",
            Self::Iron => "Iron",
            Self::Steel => "Steel",
            Self::DarkSteel => "Dark Steel",
            Self::Mithril => "Mithril",
            Self::Adamantite => "Adamantite",
            Self::Obsidian => "Obsidian",
            Self::Crystal => "Crystal",
            Self::Dragonbone => "Dragonbone",
            Self::Demonic => "Demonic",
            Self::Divine => "Divine",
            Self::Void => "Void",
            Self::Astral => "Astral",
        }
    }

    fn damage_multiplier(&self) -> f32 {
        match self {
            Self::Wood => 0.6,
            Self::Bone => 0.7,
            Self::Stone => 0.8,
            Self::Copper => 0.85,
            Self::Bronze => 0.9,
            Self::Iron => 1.0,
            Self::Steel => 1.15,
            Self::DarkSteel => 1.3,
            Self::Mithril => 1.5,
            Self::Adamantite => 1.75,
            Self::Obsidian => 1.4,
            Self::Crystal => 1.6,
            Self::Dragonbone => 1.8,
            Self::Demonic => 2.0,
            Self::Divine => 2.2,
            Self::Void => 2.5,
            Self::Astral => 3.0,
        }
    }

    fn durability_multiplier(&self) -> f32 {
        match self {
            Self::Wood => 0.5,
            Self::Bone => 0.6,
            Self::Stone => 0.7,
            Self::Copper => 0.8,
            Self::Bronze => 0.9,
            Self::Iron => 1.0,
            Self::Steel => 1.2,
            Self::DarkSteel => 1.4,
            Self::Mithril => 1.8,
            Self::Adamantite => 2.5,
            Self::Obsidian => 0.8,  // Obsidian is sharp but brittle
            Self::Crystal => 0.7,   // Crystal is magical but fragile
            Self::Dragonbone => 2.0,
            Self::Demonic => 1.6,
            Self::Divine => 3.0,
            Self::Void => 2.0,
            Self::Astral => 10.0,   // Nearly indestructible
        }
    }

    fn weight_multiplier(&self) -> f32 {
        match self {
            Self::Wood => 0.6,
            Self::Bone => 0.7,
            Self::Stone => 1.5,
            Self::Copper => 1.0,
            Self::Bronze => 1.1,
            Self::Iron => 1.2,
            Self::Steel => 1.0,
            Self::DarkSteel => 1.1,
            Self::Mithril => 0.5,    // Very light
            Self::Adamantite => 1.3,
            Self::Obsidian => 0.9,
            Self::Crystal => 0.8,
            Self::Dragonbone => 0.7,
            Self::Demonic => 1.2,
            Self::Divine => 0.3,     // Ethereal lightness
            Self::Void => 0.1,       // Nearly weightless
            Self::Astral => 0.0,     // Weightless
        }
    }

    fn magic_affinity(&self) -> i32 {
        match self {
            Self::Wood => 5,
            Self::Bone => 8,
            Self::Stone => 2,
            Self::Copper => 3,
            Self::Bronze => 4,
            Self::Iron => 0,    // Iron resists magic
            Self::Steel => 1,
            Self::DarkSteel => 10,
            Self::Mithril => 25,
            Self::Adamantite => 15,
            Self::Obsidian => 20,
            Self::Crystal => 35,
            Self::Dragonbone => 30,
            Self::Demonic => 40,
            Self::Divine => 50,
            Self::Void => 60,
            Self::Astral => 100,
        }
    }

    fn tier(&self) -> u8 {
        match self {
            Self::Wood | Self::Bone | Self::Stone => 1,
            Self::Copper | Self::Bronze => 2,
            Self::Iron => 3,
            Self::Steel | Self::DarkSteel => 4,
            Self::Mithril | Self::Obsidian => 5,
            Self::Adamantite | Self::Crystal => 6,
            Self::Dragonbone | Self::Demonic => 7,
            Self::Divine | Self::Void => 8,
            Self::Astral => 9,
        }
    }
}

/// Weapon Quality - affects overall stats
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
enum WeaponQuality {
    Broken,
    Rusted,
    Poor,
    Normal,
    Fine,
    Superior,
    Exceptional,
    Masterwork,
    Legendary,
    Mythic,
    Divine,
}

impl WeaponQuality {
    fn name(&self) -> &'static str {
        match self {
            Self::Broken => "Broken",
            Self::Rusted => "Rusted",
            Self::Poor => "Poor",
            Self::Normal => "",
            Self::Fine => "Fine",
            Self::Superior => "Superior",
            Self::Exceptional => "Exceptional",
            Self::Masterwork => "Masterwork",
            Self::Legendary => "Legendary",
            Self::Mythic => "Mythic",
            Self::Divine => "Divine",
        }
    }

    fn stat_multiplier(&self) -> f32 {
        match self {
            Self::Broken => 0.3,
            Self::Rusted => 0.5,
            Self::Poor => 0.7,
            Self::Normal => 1.0,
            Self::Fine => 1.2,
            Self::Superior => 1.4,
            Self::Exceptional => 1.6,
            Self::Masterwork => 2.0,
            Self::Legendary => 2.5,
            Self::Mythic => 3.5,
            Self::Divine => 5.0,
        }
    }

    fn durability_modifier(&self) -> f32 {
        match self {
            Self::Broken => 0.1,
            Self::Rusted => 0.3,
            Self::Poor => 0.6,
            Self::Normal => 1.0,
            Self::Fine => 1.1,
            Self::Superior => 1.25,
            Self::Exceptional => 1.4,
            Self::Masterwork => 1.6,
            Self::Legendary => 2.0,
            Self::Mythic => 3.0,
            Self::Divine => 10.0,
        }
    }

    fn enchantment_slots(&self) -> u8 {
        match self {
            Self::Broken | Self::Rusted | Self::Poor => 0,
            Self::Normal => 1,
            Self::Fine => 1,
            Self::Superior => 2,
            Self::Exceptional => 2,
            Self::Masterwork => 3,
            Self::Legendary => 4,
            Self::Mythic => 5,
            Self::Divine => 6,
        }
    }
}

/// Weapon Enchantment types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
enum WeaponEnchantment {
    // Elemental
    Fire,
    Ice,
    Lightning,
    Poison,
    Acid,

    // Holy/Unholy
    Holy,
    Cursed,
    Blessed,
    Profane,

    // Vampiric effects
    Vampiric,
    LifeSteal,
    ManaSteal,
    SoulDrain,

    // Status effects
    Stun,
    Slow,
    Blind,
    Silence,
    Fear,
    Confusion,
    Sleep,
    Paralyze,

    // Damage modifiers
    Keen,           // Increased crit chance
    Vorpal,         // Instant kill chance
    Brutal,         // Extra damage
    Precise,        // Increased accuracy
    Swift,          // Faster attacks

    // Special effects
    Disruption,     // Extra vs undead
    DemonBane,      // Extra vs demons
    DragonSlayer,   // Extra vs dragons
    GiantKiller,    // Extra vs large enemies
    Ethereal,       // Can hit ghosts
    Banishing,      // Send enemies to other planes

    // Utility
    Returning,      // For thrown weapons
    Seeking,        // Better accuracy
    Flaming,        // Light radius
    Glowing,        // Dim light

    // Legendary effects
    Annihilation,   // Massive bonus damage
    TimeStop,       // Chance to freeze time
    DimensionRift,  // Teleport enemies
    SoulReaper,     // Collect souls
}

impl WeaponEnchantment {
    fn name(&self) -> &'static str {
        match self {
            Self::Fire => "of Fire",
            Self::Ice => "of Ice",
            Self::Lightning => "of Lightning",
            Self::Poison => "of Poison",
            Self::Acid => "of Acid",
            Self::Holy => "of Holiness",
            Self::Cursed => "of Cursing",
            Self::Blessed => "of Blessing",
            Self::Profane => "of Profanity",
            Self::Vampiric => "of Vampirism",
            Self::LifeSteal => "of Life Stealing",
            Self::ManaSteal => "of Mana Stealing",
            Self::SoulDrain => "of Soul Draining",
            Self::Stun => "of Stunning",
            Self::Slow => "of Slowing",
            Self::Blind => "of Blinding",
            Self::Silence => "of Silencing",
            Self::Fear => "of Terror",
            Self::Confusion => "of Confusion",
            Self::Sleep => "of Sleep",
            Self::Paralyze => "of Paralysis",
            Self::Keen => "of Keenness",
            Self::Vorpal => "Vorpal",
            Self::Brutal => "of Brutality",
            Self::Precise => "of Precision",
            Self::Swift => "of Swiftness",
            Self::Disruption => "of Disruption",
            Self::DemonBane => "Demonbane",
            Self::DragonSlayer => "Dragonslayer",
            Self::GiantKiller => "of Giant Slaying",
            Self::Ethereal => "Ethereal",
            Self::Banishing => "of Banishment",
            Self::Returning => "of Returning",
            Self::Seeking => "of Seeking",
            Self::Flaming => "Flaming",
            Self::Glowing => "Glowing",
            Self::Annihilation => "of Annihilation",
            Self::TimeStop => "of Time Stop",
            Self::DimensionRift => "of Dimension Rifts",
            Self::SoulReaper => "of Soul Reaping",
        }
    }

    fn bonus_damage(&self) -> i32 {
        match self {
            Self::Fire | Self::Ice | Self::Lightning | Self::Poison | Self::Acid => 5,
            Self::Holy | Self::Profane => 8,
            Self::Cursed | Self::Blessed => 4,
            Self::Brutal => 10,
            Self::Keen | Self::Vorpal => 3,
            Self::Disruption | Self::DemonBane | Self::DragonSlayer | Self::GiantKiller => 15,
            Self::Annihilation => 30,
            Self::SoulReaper => 12,
            _ => 0,
        }
    }

    fn proc_chance(&self) -> u8 {
        // Percentage chance for effect to trigger
        match self {
            Self::Fire | Self::Ice | Self::Lightning => 30,
            Self::Poison | Self::Acid => 25,
            Self::Vampiric | Self::LifeSteal => 20,
            Self::ManaSteal | Self::SoulDrain => 15,
            Self::Stun | Self::Slow | Self::Blind => 15,
            Self::Silence | Self::Fear | Self::Confusion => 12,
            Self::Sleep | Self::Paralyze => 8,
            Self::Vorpal => 3,
            Self::TimeStop => 2,
            Self::DimensionRift => 10,
            Self::SoulReaper => 25,
            _ => 0,
        }
    }

    fn tier(&self) -> u8 {
        match self {
            Self::Glowing | Self::Returning => 1,
            Self::Fire | Self::Ice | Self::Lightning | Self::Poison => 2,
            Self::Acid | Self::Keen | Self::Precise | Self::Swift => 3,
            Self::Holy | Self::Cursed | Self::Blessed => 4,
            Self::Vampiric | Self::LifeSteal | Self::Stun | Self::Slow | Self::Flaming => 5,
            Self::ManaSteal | Self::Blind | Self::Silence | Self::Fear | Self::Seeking => 6,
            Self::Profane | Self::Confusion | Self::Sleep | Self::Brutal | Self::Ethereal => 7,
            Self::Disruption | Self::DemonBane | Self::DragonSlayer | Self::GiantKiller
            | Self::Banishing | Self::SoulDrain | Self::Paralyze => 8,
            Self::Vorpal | Self::Annihilation | Self::TimeStop | Self::DimensionRift
            | Self::SoulReaper => 9,
        }
    }
}

/// Weapon Mastery Levels
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
enum WeaponMasteryLevel {
    Untrained,
    Novice,
    Apprentice,
    Journeyman,
    Expert,
    Master,
    Grandmaster,
    Legendary,
}

impl WeaponMasteryLevel {
    fn name(&self) -> &'static str {
        match self {
            Self::Untrained => "Untrained",
            Self::Novice => "Novice",
            Self::Apprentice => "Apprentice",
            Self::Journeyman => "Journeyman",
            Self::Expert => "Expert",
            Self::Master => "Master",
            Self::Grandmaster => "Grandmaster",
            Self::Legendary => "Legendary",
        }
    }

    fn damage_bonus(&self) -> f32 {
        match self {
            Self::Untrained => 0.0,
            Self::Novice => 0.05,
            Self::Apprentice => 0.10,
            Self::Journeyman => 0.20,
            Self::Expert => 0.35,
            Self::Master => 0.50,
            Self::Grandmaster => 0.75,
            Self::Legendary => 1.0,
        }
    }

    fn crit_bonus(&self) -> i32 {
        match self {
            Self::Untrained => 0,
            Self::Novice => 1,
            Self::Apprentice => 2,
            Self::Journeyman => 4,
            Self::Expert => 6,
            Self::Master => 10,
            Self::Grandmaster => 15,
            Self::Legendary => 25,
        }
    }

    fn speed_bonus(&self) -> i32 {
        match self {
            Self::Untrained => 0,
            Self::Novice => 0,
            Self::Apprentice => 1,
            Self::Journeyman => 1,
            Self::Expert => 2,
            Self::Master => 3,
            Self::Grandmaster => 4,
            Self::Legendary => 5,
        }
    }

    fn xp_required(&self) -> u32 {
        match self {
            Self::Untrained => 0,
            Self::Novice => 100,
            Self::Apprentice => 500,
            Self::Journeyman => 2000,
            Self::Expert => 8000,
            Self::Master => 25000,
            Self::Grandmaster => 75000,
            Self::Legendary => 200000,
        }
    }

    fn next_level(&self) -> Option<Self> {
        match self {
            Self::Untrained => Some(Self::Novice),
            Self::Novice => Some(Self::Apprentice),
            Self::Apprentice => Some(Self::Journeyman),
            Self::Journeyman => Some(Self::Expert),
            Self::Expert => Some(Self::Master),
            Self::Master => Some(Self::Grandmaster),
            Self::Grandmaster => Some(Self::Legendary),
            Self::Legendary => None,
        }
    }
}

/// Special Weapon Abilities
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
enum WeaponAbility {
    // Attack modifiers
    Cleave,           // Hit multiple enemies
    Pierce,           // Ignore some armor
    Sweep,            // 360 degree attack
    Thrust,           // Extended range
    Bash,             // Stun chance
    Rend,             // Bleed damage over time
    Crush,            // Extra vs armored

    // Critical effects
    CriticalStrike,   // Bonus crit damage
    Decapitate,       // Instant kill on crit
    Eviscerate,       // Massive bleed on crit
    Shatter,          // Destroy armor on crit

    // Defensive abilities
    Parry,            // Block attacks
    Riposte,          // Counter-attack
    Deflect,          // Deflect projectiles

    // Movement abilities
    Charge,           // Rush and attack
    Leap,             // Jump attack
    Whirlwind,        // Spinning attack

    // Special attacks
    PowerAttack,      // Charge up big hit
    RapidStrike,      // Multiple quick hits
    PrecisionStrike,  // Guaranteed hit
    ExecutionersStrike, // Extra vs low hp

    // Magical abilities
    ArcaneStrike,     // Magic damage
    ElementalBurst,   // AOE elemental
    LifeTap,          // Heal on hit
    ManaBurn,         // Destroy mana

    // Legendary abilities
    Devastation,      // Massive AOE
    TimeSlice,        // Attack multiple times in one turn
    DimensionalCut,   // Ignore all defenses
    SoulStrike,       // Damage soul directly
}

impl WeaponAbility {
    fn name(&self) -> &'static str {
        match self {
            Self::Cleave => "Cleave",
            Self::Pierce => "Pierce",
            Self::Sweep => "Sweep",
            Self::Thrust => "Thrust",
            Self::Bash => "Bash",
            Self::Rend => "Rend",
            Self::Crush => "Crush",
            Self::CriticalStrike => "Critical Strike",
            Self::Decapitate => "Decapitate",
            Self::Eviscerate => "Eviscerate",
            Self::Shatter => "Shatter",
            Self::Parry => "Parry",
            Self::Riposte => "Riposte",
            Self::Deflect => "Deflect",
            Self::Charge => "Charge",
            Self::Leap => "Leap Attack",
            Self::Whirlwind => "Whirlwind",
            Self::PowerAttack => "Power Attack",
            Self::RapidStrike => "Rapid Strike",
            Self::PrecisionStrike => "Precision Strike",
            Self::ExecutionersStrike => "Executioner's Strike",
            Self::ArcaneStrike => "Arcane Strike",
            Self::ElementalBurst => "Elemental Burst",
            Self::LifeTap => "Life Tap",
            Self::ManaBurn => "Mana Burn",
            Self::Devastation => "Devastation",
            Self::TimeSlice => "Time Slice",
            Self::DimensionalCut => "Dimensional Cut",
            Self::SoulStrike => "Soul Strike",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::Cleave => "Strike hits all adjacent enemies",
            Self::Pierce => "Ignores 50% of target's armor",
            Self::Sweep => "Attack all enemies around you",
            Self::Thrust => "Attack enemies 2 tiles away",
            Self::Bash => "25% chance to stun target",
            Self::Rend => "Causes bleeding for 5 turns",
            Self::Crush => "Double damage vs armored enemies",
            Self::CriticalStrike => "Critical hits deal 50% more damage",
            Self::Decapitate => "Critical hits have 10% instant kill chance",
            Self::Eviscerate => "Critical hits cause massive bleeding",
            Self::Shatter => "Critical hits destroy target's armor",
            Self::Parry => "30% chance to block melee attacks",
            Self::Riposte => "Counter-attack when successfully parrying",
            Self::Deflect => "50% chance to deflect projectiles",
            Self::Charge => "Rush toward enemy and attack with bonus damage",
            Self::Leap => "Jump to target and deal falling damage",
            Self::Whirlwind => "Spin attack hitting all adjacent tiles",
            Self::PowerAttack => "Spend 2 turns to deal triple damage",
            Self::RapidStrike => "Attack 3 times at reduced damage",
            Self::PrecisionStrike => "Never misses but deals less damage",
            Self::ExecutionersStrike => "Deal 2x damage to enemies below 25% HP",
            Self::ArcaneStrike => "Attacks deal bonus magic damage",
            Self::ElementalBurst => "On hit, chance for AOE elemental explosion",
            Self::LifeTap => "Heal for 20% of damage dealt",
            Self::ManaBurn => "Destroy enemy mana equal to damage",
            Self::Devastation => "Massive AOE attack that destroys terrain",
            Self::TimeSlice => "Attack 5 times in one turn",
            Self::DimensionalCut => "Attack ignores all defenses and resistances",
            Self::SoulStrike => "Permanently reduce enemy max HP",
        }
    }

    fn mana_cost(&self) -> i32 {
        match self {
            Self::Pierce | Self::Thrust | Self::Parry => 5,
            Self::Cleave | Self::Bash | Self::Rend | Self::Riposte | Self::Deflect => 10,
            Self::Sweep | Self::Crush | Self::CriticalStrike | Self::Charge | Self::Leap => 15,
            Self::Eviscerate | Self::Shatter | Self::RapidStrike | Self::PrecisionStrike => 20,
            Self::Whirlwind | Self::PowerAttack | Self::ExecutionersStrike => 25,
            Self::Decapitate | Self::ArcaneStrike | Self::LifeTap | Self::ManaBurn => 30,
            Self::ElementalBurst => 40,
            Self::Devastation => 75,
            Self::TimeSlice => 100,
            Self::DimensionalCut => 150,
            Self::SoulStrike => 200,
        }
    }

    fn cooldown(&self) -> u8 {
        // Turns before ability can be used again
        match self {
            Self::Pierce | Self::Thrust | Self::Parry | Self::Riposte => 0,
            Self::Cleave | Self::Bash | Self::Deflect => 1,
            Self::Sweep | Self::Rend | Self::Crush | Self::Charge => 2,
            Self::Leap | Self::CriticalStrike | Self::RapidStrike => 3,
            Self::Whirlwind | Self::PowerAttack | Self::PrecisionStrike => 4,
            Self::Eviscerate | Self::Shatter | Self::ExecutionersStrike => 5,
            Self::Decapitate | Self::ArcaneStrike | Self::LifeTap => 6,
            Self::ManaBurn | Self::ElementalBurst => 8,
            Self::Devastation | Self::TimeSlice => 10,
            Self::DimensionalCut => 15,
            Self::SoulStrike => 20,
        }
    }
}

/// Weapon mastery tracking for a player
#[derive(Clone, Debug, Serialize, Deserialize)]
struct WeaponMastery {
    category: WeaponCategory,
    level: WeaponMasteryLevel,
    xp: u32,
    kills: u32,
    damage_dealt: u64,
}

impl WeaponMastery {
    fn new(category: WeaponCategory) -> Self {
        Self {
            category,
            level: WeaponMasteryLevel::Untrained,
            xp: 0,
            kills: 0,
            damage_dealt: 0,
        }
    }

    fn add_xp(&mut self, amount: u32) -> bool {
        self.xp += amount;
        if let Some(next) = self.level.next_level() {
            if self.xp >= next.xp_required() {
                self.level = next;
                return true; // Leveled up
            }
        }
        false
    }

    fn record_kill(&mut self) {
        self.kills += 1;
        self.add_xp(50); // Bonus XP for kills
    }

    fn record_damage(&mut self, damage: i32) {
        self.damage_dealt += damage as u64;
        self.add_xp((damage / 10) as u32); // XP based on damage
    }

    fn xp_to_next_level(&self) -> Option<u32> {
        self.level.next_level().map(|next| {
            let required = next.xp_required();
            if self.xp >= required { 0 } else { required - self.xp }
        })
    }
}

/// The complete weapon instance
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Weapon {
    weapon_type: WeaponType,
    material: WeaponMaterial,
    quality: WeaponQuality,
    enchantments: Vec<WeaponEnchantment>,
    abilities: Vec<WeaponAbility>,

    // Durability system
    max_durability: i32,
    current_durability: i32,

    // Unique weapon info
    is_unique: bool,
    unique_name: Option<String>,
    unique_lore: Option<String>,

    // Combat stats (cached)
    cached_min_damage: i32,
    cached_max_damage: i32,
    cached_crit_chance: i32,
    cached_speed: i32,

    // Soulbound/cursed
    is_soulbound: bool,
    is_cursed: bool,
    curse_level: u8,

    // Kill counter
    kills: u32,
    souls_absorbed: u32,
}

impl Weapon {
    fn new(weapon_type: WeaponType, material: WeaponMaterial, quality: WeaponQuality) -> Self {
        let base_durability = 100;
        let max_durability = (base_durability as f32
            * material.durability_multiplier()
            * quality.durability_modifier()) as i32;

        let mut weapon = Self {
            weapon_type,
            material,
            quality,
            enchantments: Vec::new(),
            abilities: Vec::new(),
            max_durability,
            current_durability: max_durability,
            is_unique: false,
            unique_name: None,
            unique_lore: None,
            cached_min_damage: 0,
            cached_max_damage: 0,
            cached_crit_chance: 0,
            cached_speed: 0,
            is_soulbound: false,
            is_cursed: false,
            curse_level: 0,
            kills: 0,
            souls_absorbed: 0,
        };
        weapon.recalculate_stats();
        weapon
    }

    fn recalculate_stats(&mut self) {
        let (base_min, base_max) = self.weapon_type.base_damage();
        let material_mult = self.material.damage_multiplier();
        let quality_mult = self.quality.stat_multiplier();

        let enchant_bonus: i32 = self.enchantments.iter()
            .map(|e| e.bonus_damage())
            .sum();

        self.cached_min_damage = ((base_min as f32 * material_mult * quality_mult) as i32) + enchant_bonus;
        self.cached_max_damage = ((base_max as f32 * material_mult * quality_mult) as i32) + enchant_bonus;
        self.cached_crit_chance = self.weapon_type.crit_chance_bonus()
            + if self.enchantments.contains(&WeaponEnchantment::Keen) { 10 } else { 0 };
        self.cached_speed = self.weapon_type.base_speed()
            + if self.enchantments.contains(&WeaponEnchantment::Swift) { 2 } else { 0 };
    }

    fn add_enchantment(&mut self, enchantment: WeaponEnchantment) -> bool {
        if self.enchantments.len() < self.quality.enchantment_slots() as usize {
            if !self.enchantments.contains(&enchantment) {
                self.enchantments.push(enchantment);
                self.recalculate_stats();
                return true;
            }
        }
        false
    }

    fn add_ability(&mut self, ability: WeaponAbility) {
        if !self.abilities.contains(&ability) {
            self.abilities.push(ability);
        }
    }

    fn damage_range(&self) -> (i32, i32) {
        (self.cached_min_damage, self.cached_max_damage)
    }

    fn roll_damage(&self, rng: &mut impl rand::Rng) -> i32 {
        rng.gen_range(self.cached_min_damage..=self.cached_max_damage)
    }

    fn apply_mastery_bonus(&self, base_damage: i32, mastery: &WeaponMastery) -> i32 {
        let bonus = mastery.level.damage_bonus();
        (base_damage as f32 * (1.0 + bonus)) as i32
    }

    fn use_durability(&mut self, amount: i32) {
        self.current_durability = (self.current_durability - amount).max(0);
    }

    fn repair(&mut self, amount: i32) {
        self.current_durability = (self.current_durability + amount).min(self.max_durability);
    }

    fn full_repair(&mut self) {
        self.current_durability = self.max_durability;
    }

    fn durability_percent(&self) -> f32 {
        if self.max_durability == 0 { return 0.0; }
        (self.current_durability as f32 / self.max_durability as f32) * 100.0
    }

    fn is_broken(&self) -> bool {
        self.current_durability <= 0
    }

    fn durability_damage_modifier(&self) -> f32 {
        let percent = self.durability_percent();
        if percent >= 75.0 { 1.0 }
        else if percent >= 50.0 { 0.9 }
        else if percent >= 25.0 { 0.75 }
        else if percent > 0.0 { 0.5 }
        else { 0.1 } // Broken
    }

    fn record_kill(&mut self) {
        self.kills += 1;
        if self.enchantments.contains(&WeaponEnchantment::SoulReaper) {
            self.souls_absorbed += 1;
        }
    }

    fn display_name(&self) -> String {
        if let Some(ref name) = self.unique_name {
            return name.clone();
        }

        let quality_prefix = if self.quality == WeaponQuality::Normal {
            String::new()
        } else {
            format!("{} ", self.quality.name())
        };

        let material_prefix = self.material.name();
        let weapon_name = self.weapon_type.name();

        let enchant_suffix = if !self.enchantments.is_empty() {
            format!(" {}", self.enchantments[0].name())
        } else {
            String::new()
        };

        format!("{}{} {}{}", quality_prefix, material_prefix, weapon_name, enchant_suffix)
    }

    fn full_description(&self) -> Vec<String> {
        let mut lines = Vec::new();

        lines.push(self.display_name());

        if let Some(ref lore) = self.unique_lore {
            lines.push(format!("\"{}\"", lore));
        }

        lines.push(format!("Damage: {}-{}", self.cached_min_damage, self.cached_max_damage));
        lines.push(format!("Crit Chance: {}%", self.cached_crit_chance));
        lines.push(format!("Speed: {:+}", self.cached_speed));
        lines.push(format!("Grip: {}", self.weapon_type.grip_type().name()));
        lines.push(format!("Durability: {}/{} ({:.0}%)",
            self.current_durability, self.max_durability, self.durability_percent()));

        if !self.enchantments.is_empty() {
            lines.push(String::from("Enchantments:"));
            for ench in &self.enchantments {
                lines.push(format!("  - {}", ench.name()));
            }
        }

        if !self.abilities.is_empty() {
            lines.push(String::from("Abilities:"));
            for ability in &self.abilities {
                lines.push(format!("  - {}: {}", ability.name(), ability.description()));
            }
        }

        if self.kills > 0 {
            lines.push(format!("Kills: {}", self.kills));
        }

        if self.souls_absorbed > 0 {
            lines.push(format!("Souls Absorbed: {}", self.souls_absorbed));
        }

        if self.is_cursed {
            lines.push(format!("CURSED (Level {})", self.curse_level));
        }

        if self.is_soulbound {
            lines.push(String::from("Soulbound"));
        }

        lines
    }
}

/// Collection of 20+ legendary unique weapons
#[derive(Clone, Debug)]
struct LegendaryWeapons;

impl LegendaryWeapons {
    fn excalibur() -> Weapon {
        let mut weapon = Weapon::new(WeaponType::LongSword, WeaponMaterial::Divine, WeaponQuality::Divine);
        weapon.is_unique = true;
        weapon.unique_name = Some(String::from("Excalibur"));
        weapon.unique_lore = Some(String::from("The legendary sword of kings, blessed by the Lady of the Lake"));
        weapon.add_enchantment(WeaponEnchantment::Holy);
        weapon.add_enchantment(WeaponEnchantment::Blessed);
        weapon.add_enchantment(WeaponEnchantment::Keen);
        weapon.add_ability(WeaponAbility::Decapitate);
        weapon.add_ability(WeaponAbility::Parry);
        weapon.is_soulbound = true;
        weapon
    }

    fn stormbringer() -> Weapon {
        let mut weapon = Weapon::new(WeaponType::Greatsword, WeaponMaterial::Demonic, WeaponQuality::Mythic);
        weapon.is_unique = true;
        weapon.unique_name = Some(String::from("Stormbringer"));
        weapon.unique_lore = Some(String::from("The black blade that drinks souls and brings chaos"));
        weapon.add_enchantment(WeaponEnchantment::SoulDrain);
        weapon.add_enchantment(WeaponEnchantment::Cursed);
        weapon.add_enchantment(WeaponEnchantment::Vampiric);
        weapon.add_ability(WeaponAbility::SoulStrike);
        weapon.add_ability(WeaponAbility::LifeTap);
        weapon.is_cursed = true;
        weapon.curse_level = 5;
        weapon
    }

    fn mjolnir() -> Weapon {
        let mut weapon = Weapon::new(WeaponType::WarHammer, WeaponMaterial::Divine, WeaponQuality::Legendary);
        weapon.is_unique = true;
        weapon.unique_name = Some(String::from("Mjolnir"));
        weapon.unique_lore = Some(String::from("The hammer of the thunder god, forged in a dying star"));
        weapon.add_enchantment(WeaponEnchantment::Lightning);
        weapon.add_enchantment(WeaponEnchantment::Returning);
        weapon.add_enchantment(WeaponEnchantment::Stun);
        weapon.add_ability(WeaponAbility::Bash);
        weapon.add_ability(WeaponAbility::Devastation);
        weapon
    }

    fn gungnir() -> Weapon {
        let mut weapon = Weapon::new(WeaponType::Spear, WeaponMaterial::Divine, WeaponQuality::Legendary);
        weapon.is_unique = true;
        weapon.unique_name = Some(String::from("Gungnir"));
        weapon.unique_lore = Some(String::from("The spear of the All-Father, which never misses its mark"));
        weapon.add_enchantment(WeaponEnchantment::Seeking);
        weapon.add_enchantment(WeaponEnchantment::Returning);
        weapon.add_ability(WeaponAbility::PrecisionStrike);
        weapon.add_ability(WeaponAbility::Thrust);
        weapon
    }

    fn kusanagi() -> Weapon {
        let mut weapon = Weapon::new(WeaponType::Katana, WeaponMaterial::Astral, WeaponQuality::Divine);
        weapon.is_unique = true;
        weapon.unique_name = Some(String::from("Kusanagi-no-Tsurugi"));
        weapon.unique_lore = Some(String::from("The Grass-Cutting Sword, pulled from a dragon's tail"));
        weapon.add_enchantment(WeaponEnchantment::DragonSlayer);
        weapon.add_enchantment(WeaponEnchantment::Keen);
        weapon.add_enchantment(WeaponEnchantment::Swift);
        weapon.add_ability(WeaponAbility::TimeSlice);
        weapon.add_ability(WeaponAbility::RapidStrike);
        weapon
    }

    fn tyrfing() -> Weapon {
        let mut weapon = Weapon::new(WeaponType::LongSword, WeaponMaterial::DarkSteel, WeaponQuality::Legendary);
        weapon.is_unique = true;
        weapon.unique_name = Some(String::from("Tyrfing"));
        weapon.unique_lore = Some(String::from("A cursed blade that must draw blood when unsheathed"));
        weapon.add_enchantment(WeaponEnchantment::Cursed);
        weapon.add_enchantment(WeaponEnchantment::Vorpal);
        weapon.add_ability(WeaponAbility::Decapitate);
        weapon.is_cursed = true;
        weapon.curse_level = 3;
        weapon
    }

    fn caladbolg() -> Weapon {
        let mut weapon = Weapon::new(WeaponType::Greatsword, WeaponMaterial::Mithril, WeaponQuality::Legendary);
        weapon.is_unique = true;
        weapon.unique_name = Some(String::from("Caladbolg"));
        weapon.unique_lore = Some(String::from("The rainbow sword that can cut through hills"));
        weapon.add_enchantment(WeaponEnchantment::Brutal);
        weapon.add_enchantment(WeaponEnchantment::Glowing);
        weapon.add_ability(WeaponAbility::Cleave);
        weapon.add_ability(WeaponAbility::Devastation);
        weapon
    }

    fn demonedge() -> Weapon {
        let mut weapon = Weapon::new(WeaponType::Scimitar, WeaponMaterial::Demonic, WeaponQuality::Mythic);
        weapon.is_unique = true;
        weapon.unique_name = Some(String::from("Demonedge"));
        weapon.unique_lore = Some(String::from("Forged in the deepest hells, hungry for mortal souls"));
        weapon.add_enchantment(WeaponEnchantment::Profane);
        weapon.add_enchantment(WeaponEnchantment::LifeSteal);
        weapon.add_enchantment(WeaponEnchantment::Fear);
        weapon.add_ability(WeaponAbility::SoulStrike);
        weapon
    }

    fn frostmourne() -> Weapon {
        let mut weapon = Weapon::new(WeaponType::Claymore, WeaponMaterial::Void, WeaponQuality::Mythic);
        weapon.is_unique = true;
        weapon.unique_name = Some(String::from("Frostmourne"));
        weapon.unique_lore = Some(String::from("The runeblade that hungers, stealing the souls of its victims"));
        weapon.add_enchantment(WeaponEnchantment::Ice);
        weapon.add_enchantment(WeaponEnchantment::SoulDrain);
        weapon.add_enchantment(WeaponEnchantment::Cursed);
        weapon.add_ability(WeaponAbility::SoulStrike);
        weapon.add_ability(WeaponAbility::ExecutionersStrike);
        weapon.is_cursed = true;
        weapon.curse_level = 4;
        weapon
    }

    fn dragonbane() -> Weapon {
        let mut weapon = Weapon::new(WeaponType::BattleAxe, WeaponMaterial::Dragonbone, WeaponQuality::Legendary);
        weapon.is_unique = true;
        weapon.unique_name = Some(String::from("Dragonbane"));
        weapon.unique_lore = Some(String::from("Crafted from a dragon's spine, anathema to all dragonkind"));
        weapon.add_enchantment(WeaponEnchantment::DragonSlayer);
        weapon.add_enchantment(WeaponEnchantment::Fire);
        weapon.add_ability(WeaponAbility::Cleave);
        weapon.add_ability(WeaponAbility::Crush);
        weapon
    }

    fn sunblade() -> Weapon {
        let mut weapon = Weapon::new(WeaponType::LongSword, WeaponMaterial::Crystal, WeaponQuality::Legendary);
        weapon.is_unique = true;
        weapon.unique_name = Some(String::from("Sunblade"));
        weapon.unique_lore = Some(String::from("A blade of pure crystallized sunlight, bane of the undead"));
        weapon.add_enchantment(WeaponEnchantment::Holy);
        weapon.add_enchantment(WeaponEnchantment::Disruption);
        weapon.add_enchantment(WeaponEnchantment::Flaming);
        weapon.add_ability(WeaponAbility::ElementalBurst);
        weapon
    }

    fn nightfall() -> Weapon {
        let mut weapon = Weapon::new(WeaponType::Dagger, WeaponMaterial::Obsidian, WeaponQuality::Legendary);
        weapon.is_unique = true;
        weapon.unique_name = Some(String::from("Nightfall"));
        weapon.unique_lore = Some(String::from("A blade of pure shadow, invisible in darkness"));
        weapon.add_enchantment(WeaponEnchantment::Vorpal);
        weapon.add_enchantment(WeaponEnchantment::Ethereal);
        weapon.add_ability(WeaponAbility::CriticalStrike);
        weapon.add_ability(WeaponAbility::RapidStrike);
        weapon
    }

    fn worldbreaker() -> Weapon {
        let mut weapon = Weapon::new(WeaponType::Maul, WeaponMaterial::Adamantite, WeaponQuality::Mythic);
        weapon.is_unique = true;
        weapon.unique_name = Some(String::from("Worldbreaker"));
        weapon.unique_lore = Some(String::from("A hammer so heavy it cracks the earth with each swing"));
        weapon.add_enchantment(WeaponEnchantment::Brutal);
        weapon.add_enchantment(WeaponEnchantment::Stun);
        weapon.add_ability(WeaponAbility::Devastation);
        weapon.add_ability(WeaponAbility::Crush);
        weapon.add_ability(WeaponAbility::Shatter);
        weapon
    }

    fn soulreaver() -> Weapon {
        let mut weapon = Weapon::new(WeaponType::Glaive, WeaponMaterial::Void, WeaponQuality::Mythic);
        weapon.is_unique = true;
        weapon.unique_name = Some(String::from("Soul Reaver"));
        weapon.unique_lore = Some(String::from("The blade that drinks souls, growing stronger with each kill"));
        weapon.add_enchantment(WeaponEnchantment::SoulReaper);
        weapon.add_enchantment(WeaponEnchantment::Vampiric);
        weapon.add_ability(WeaponAbility::SoulStrike);
        weapon.add_ability(WeaponAbility::LifeTap);
        weapon
    }

    fn ashbringer() -> Weapon {
        let mut weapon = Weapon::new(WeaponType::Claymore, WeaponMaterial::Divine, WeaponQuality::Legendary);
        weapon.is_unique = true;
        weapon.unique_name = Some(String::from("Ashbringer"));
        weapon.unique_lore = Some(String::from("The blade of light that reduces the undead to ashes"));
        weapon.add_enchantment(WeaponEnchantment::Holy);
        weapon.add_enchantment(WeaponEnchantment::Disruption);
        weapon.add_enchantment(WeaponEnchantment::Blessed);
        weapon.add_ability(WeaponAbility::Cleave);
        weapon.add_ability(WeaponAbility::ElementalBurst);
        weapon
    }

    fn apollos_bow() -> Weapon {
        let mut weapon = Weapon::new(WeaponType::LongBow, WeaponMaterial::Divine, WeaponQuality::Legendary);
        weapon.is_unique = true;
        weapon.unique_name = Some(String::from("Apollo's Bow"));
        weapon.unique_lore = Some(String::from("The bow of the sun god, its arrows are rays of pure light"));
        weapon.add_enchantment(WeaponEnchantment::Fire);
        weapon.add_enchantment(WeaponEnchantment::Seeking);
        weapon.add_enchantment(WeaponEnchantment::Flaming);
        weapon.add_ability(WeaponAbility::PrecisionStrike);
        weapon
    }

    fn void_staff() -> Weapon {
        let mut weapon = Weapon::new(WeaponType::Staff, WeaponMaterial::Void, WeaponQuality::Mythic);
        weapon.is_unique = true;
        weapon.unique_name = Some(String::from("Staff of the Void"));
        weapon.unique_lore = Some(String::from("A conduit to the emptiness between worlds"));
        weapon.add_enchantment(WeaponEnchantment::DimensionRift);
        weapon.add_enchantment(WeaponEnchantment::Banishing);
        weapon.add_ability(WeaponAbility::DimensionalCut);
        weapon.add_ability(WeaponAbility::ArcaneStrike);
        weapon
    }

    fn godslayer() -> Weapon {
        let mut weapon = Weapon::new(WeaponType::Greatsword, WeaponMaterial::Astral, WeaponQuality::Divine);
        weapon.is_unique = true;
        weapon.unique_name = Some(String::from("Godslayer"));
        weapon.unique_lore = Some(String::from("The only blade capable of slaying immortals"));
        weapon.add_enchantment(WeaponEnchantment::Annihilation);
        weapon.add_enchantment(WeaponEnchantment::Vorpal);
        weapon.add_ability(WeaponAbility::DimensionalCut);
        weapon.add_ability(WeaponAbility::TimeSlice);
        weapon.is_soulbound = true;
        weapon
    }

    fn deaths_embrace() -> Weapon {
        let mut weapon = Weapon::new(WeaponType::Stiletto, WeaponMaterial::Demonic, WeaponQuality::Legendary);
        weapon.is_unique = true;
        weapon.unique_name = Some(String::from("Death's Embrace"));
        weapon.unique_lore = Some(String::from("A kiss from this blade sends souls directly to the underworld"));
        weapon.add_enchantment(WeaponEnchantment::Vorpal);
        weapon.add_enchantment(WeaponEnchantment::Poison);
        weapon.add_ability(WeaponAbility::ExecutionersStrike);
        weapon.add_ability(WeaponAbility::CriticalStrike);
        weapon
    }

    fn oathkeeper() -> Weapon {
        let mut weapon = Weapon::new(WeaponType::LongSword, WeaponMaterial::Mithril, WeaponQuality::Legendary);
        weapon.is_unique = true;
        weapon.unique_name = Some(String::from("Oathkeeper"));
        weapon.unique_lore = Some(String::from("A blade forged from a paladin's unbreakable vow"));
        weapon.add_enchantment(WeaponEnchantment::Holy);
        weapon.add_enchantment(WeaponEnchantment::Blessed);
        weapon.add_ability(WeaponAbility::Parry);
        weapon.add_ability(WeaponAbility::Riposte);
        weapon.is_soulbound = true;
        weapon
    }

    fn widowmaker() -> Weapon {
        let mut weapon = Weapon::new(WeaponType::Crossbow, WeaponMaterial::DarkSteel, WeaponQuality::Legendary);
        weapon.is_unique = true;
        weapon.unique_name = Some(String::from("Widowmaker"));
        weapon.unique_lore = Some(String::from("A crossbow that has ended a thousand bloodlines"));
        weapon.add_enchantment(WeaponEnchantment::Keen);
        weapon.add_enchantment(WeaponEnchantment::Poison);
        weapon.add_ability(WeaponAbility::CriticalStrike);
        weapon.add_ability(WeaponAbility::ExecutionersStrike);
        weapon
    }

    fn random_legendary(rng: &mut impl rand::Rng) -> Weapon {
        let index = rng.gen_range(0..21);
        match index {
            0 => Self::excalibur(),
            1 => Self::stormbringer(),
            2 => Self::mjolnir(),
            3 => Self::gungnir(),
            4 => Self::kusanagi(),
            5 => Self::tyrfing(),
            6 => Self::caladbolg(),
            7 => Self::demonedge(),
            8 => Self::frostmourne(),
            9 => Self::dragonbane(),
            10 => Self::sunblade(),
            11 => Self::nightfall(),
            12 => Self::worldbreaker(),
            13 => Self::soulreaver(),
            14 => Self::ashbringer(),
            15 => Self::apollos_bow(),
            16 => Self::void_staff(),
            17 => Self::godslayer(),
            18 => Self::deaths_embrace(),
            19 => Self::oathkeeper(),
            _ => Self::widowmaker(),
        }
    }
}

/// Dual Wield configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
struct DualWieldSetup {
    main_hand: Option<Weapon>,
    off_hand: Option<Weapon>,
}

impl DualWieldSetup {
    fn new() -> Self {
        Self {
            main_hand: None,
            off_hand: None,
        }
    }

    fn can_equip_off_hand(&self, weapon: &Weapon) -> bool {
        weapon.weapon_type.grip_type().allows_dual_wield()
    }

    fn equip_main_hand(&mut self, weapon: Weapon) -> Option<Weapon> {
        let old = self.main_hand.take();
        self.main_hand = Some(weapon);
        // If main hand is two-handed, clear off hand
        if let Some(ref w) = self.main_hand {
            if w.weapon_type.grip_type() == WeaponGrip::TwoHanded {
                self.off_hand = None;
            }
        }
        old
    }

    fn equip_off_hand(&mut self, weapon: Weapon) -> Option<Weapon> {
        // Can't equip off-hand if main hand is two-handed
        if let Some(ref main) = self.main_hand {
            if main.weapon_type.grip_type() == WeaponGrip::TwoHanded {
                return Some(weapon); // Return the weapon, can't equip
            }
        }
        if !self.can_equip_off_hand(&weapon) {
            return Some(weapon); // Return the weapon, can't equip
        }
        let old = self.off_hand.take();
        self.off_hand = Some(weapon);
        old
    }

    fn is_dual_wielding(&self) -> bool {
        self.main_hand.is_some() && self.off_hand.is_some()
    }

    fn total_damage_range(&self) -> (i32, i32) {
        let mut min_total = 0;
        let mut max_total = 0;

        if let Some(ref main) = self.main_hand {
            let (min, max) = main.damage_range();
            min_total += min;
            max_total += max;
        }

        if let Some(ref off) = self.off_hand {
            let (min, max) = off.damage_range();
            // Off-hand does 50% damage
            min_total += min / 2;
            max_total += max / 2;
        }

        (min_total, max_total)
    }

    fn combined_crit_chance(&self) -> i32 {
        let mut crit = 0;
        if let Some(ref main) = self.main_hand {
            crit += main.cached_crit_chance;
        }
        if let Some(ref off) = self.off_hand {
            crit += off.cached_crit_chance / 2; // Off-hand contributes less
        }
        crit
    }
}

/// Combat calculations integrating the weapon system
struct WeaponCombat;

impl WeaponCombat {
    fn calculate_attack_damage(
        weapon: &Weapon,
        mastery: &WeaponMastery,
        attacker_strength: i32,
        target_defense: i32,
        is_crit: bool,
        rng: &mut impl rand::Rng,
    ) -> (i32, Vec<WeaponEnchantment>) {
        // Base weapon damage
        let mut damage = weapon.roll_damage(rng);

        // Apply mastery bonus
        damage = weapon.apply_mastery_bonus(damage, mastery);

        // Apply durability modifier
        damage = (damage as f32 * weapon.durability_damage_modifier()) as i32;

        // Add strength bonus
        damage += attacker_strength / 4;

        // Apply defense reduction
        let defense_reduction = if weapon.abilities.contains(&WeaponAbility::Pierce) {
            target_defense / 2
        } else {
            target_defense
        };
        damage = (damage - defense_reduction / 2).max(1);

        // Critical hit
        if is_crit {
            damage *= 2;
            if weapon.abilities.contains(&WeaponAbility::CriticalStrike) {
                damage = (damage as f32 * 1.5) as i32;
            }
        }

        // Collect triggered enchantments
        let mut triggered = Vec::new();
        for enchant in &weapon.enchantments {
            if rng.gen_range(0..100) < enchant.proc_chance() as i32 {
                damage += enchant.bonus_damage();
                triggered.push(*enchant);
            }
        }

        (damage, triggered)
    }

    fn roll_crit(weapon: &Weapon, mastery: &WeaponMastery, luck: i32, rng: &mut impl rand::Rng) -> bool {
        let crit_chance = weapon.cached_crit_chance
            + mastery.level.crit_bonus()
            + luck / 5;
        rng.gen_range(0..100) < crit_chance
    }

    fn calculate_attack_speed(weapon: &Weapon, mastery: &WeaponMastery, agility: i32) -> i32 {
        let base_speed = weapon.cached_speed;
        let mastery_bonus = mastery.level.speed_bonus();
        let agility_bonus = agility / 10;
        base_speed + mastery_bonus + agility_bonus
    }
}

// ============================================================================
// ITEM STRUCT
// ============================================================================

#[derive(Clone, Serialize, Deserialize)]
struct Item {
    x: usize,
    y: usize,
    kind: ItemKind,
    rarity: Rarity,
    // Optional advanced weapon data
    #[serde(default)]
    weapon_data: Option<Weapon>,
}

impl Item {
    fn new(x: usize, y: usize, kind: ItemKind, rarity: Rarity) -> Self {
        Self { x, y, kind, rarity, weapon_data: None }
    }

    fn new_with_weapon(x: usize, y: usize, kind: ItemKind, rarity: Rarity, weapon: Weapon) -> Self {
        Self { x, y, kind, rarity, weapon_data: Some(weapon) }
    }

    fn stats(&self) -> (i32, i32, i32, i32) {
        // If this item has advanced weapon data, use that for attack stat
        if let Some(ref weapon) = self.weapon_data {
            let (min_dmg, max_dmg) = weapon.damage_range();
            let avg_dmg = (min_dmg + max_dmg) / 2;
            return (avg_dmg, 0, 0, 0);
        }

        let (atk, def, hp, mana) = self.kind.base_stats();
        let mult = self.rarity.stat_bonus();
        (
            (atk as f32 * mult) as i32,
            (def as f32 * mult) as i32,
            (hp as f32 * mult) as i32,
            (mana as f32 * mult) as i32,
        )
    }

    fn display_name(&self) -> String {
        // If this item has advanced weapon data, use that name
        if let Some(ref weapon) = self.weapon_data {
            return weapon.display_name();
        }
        format!("{}{}", self.rarity.prefix(), self.kind.name())
    }

    fn has_weapon_data(&self) -> bool {
        self.weapon_data.is_some()
    }

    fn get_weapon(&self) -> Option<&Weapon> {
        self.weapon_data.as_ref()
    }

    fn get_weapon_mut(&mut self) -> Option<&mut Weapon> {
        self.weapon_data.as_mut()
    }

    /// Create an item with a randomly generated advanced weapon
    fn new_advanced_weapon(x: usize, y: usize, kind: ItemKind, rarity: Rarity, rng: &mut impl rand::Rng) -> Self {
        // Map ItemKind to WeaponType
        let weapon_type = match kind {
            ItemKind::Dagger => WeaponType::Dagger,
            ItemKind::ShortSword => WeaponType::ShortSword,
            ItemKind::LongSword => WeaponType::LongSword,
            ItemKind::Greatsword => WeaponType::Greatsword,
            ItemKind::Axe => WeaponType::HandAxe,
            ItemKind::BattleAxe => WeaponType::BattleAxe,
            ItemKind::Mace => WeaponType::Mace,
            ItemKind::WarHammer => WeaponType::WarHammer,
            ItemKind::Spear => WeaponType::Spear,
            ItemKind::Halberd => WeaponType::Halberd,
            ItemKind::Staff => WeaponType::Staff,
            ItemKind::Bow => WeaponType::LongBow,
            ItemKind::Crossbow => WeaponType::Crossbow,
            ItemKind::Wand => WeaponType::Wand,
            ItemKind::Scythe => WeaponType::Glaive,
            ItemKind::Katana => WeaponType::Katana,
            ItemKind::Rapier => WeaponType::Rapier,
            ItemKind::Flail => WeaponType::Flail,
            ItemKind::Morningstar => WeaponType::Morningstar,
            ItemKind::Trident => WeaponType::Trident,
            _ => WeaponType::LongSword, // Default
        };

        // Map rarity to material and quality
        let (material, quality) = match rarity {
            Rarity::Common => (
                match rng.gen_range(0..5) {
                    0 => WeaponMaterial::Wood,
                    1 => WeaponMaterial::Bone,
                    2 => WeaponMaterial::Copper,
                    3 => WeaponMaterial::Bronze,
                    _ => WeaponMaterial::Iron,
                },
                match rng.gen_range(0..4) {
                    0 => WeaponQuality::Poor,
                    1 | 2 => WeaponQuality::Normal,
                    _ => WeaponQuality::Fine,
                }
            ),
            Rarity::Uncommon => (
                match rng.gen_range(0..4) {
                    0 => WeaponMaterial::Iron,
                    1 | 2 => WeaponMaterial::Steel,
                    _ => WeaponMaterial::DarkSteel,
                },
                match rng.gen_range(0..3) {
                    0 => WeaponQuality::Fine,
                    1 => WeaponQuality::Superior,
                    _ => WeaponQuality::Exceptional,
                }
            ),
            Rarity::Rare => (
                match rng.gen_range(0..4) {
                    0 => WeaponMaterial::DarkSteel,
                    1 => WeaponMaterial::Mithril,
                    2 => WeaponMaterial::Obsidian,
                    _ => WeaponMaterial::Crystal,
                },
                match rng.gen_range(0..3) {
                    0 => WeaponQuality::Superior,
                    1 => WeaponQuality::Exceptional,
                    _ => WeaponQuality::Masterwork,
                }
            ),
            Rarity::Epic => (
                match rng.gen_range(0..4) {
                    0 => WeaponMaterial::Mithril,
                    1 => WeaponMaterial::Adamantite,
                    2 => WeaponMaterial::Dragonbone,
                    _ => WeaponMaterial::Crystal,
                },
                match rng.gen_range(0..3) {
                    0 => WeaponQuality::Exceptional,
                    1 => WeaponQuality::Masterwork,
                    _ => WeaponQuality::Legendary,
                }
            ),
            Rarity::Legendary => (
                match rng.gen_range(0..4) {
                    0 => WeaponMaterial::Adamantite,
                    1 => WeaponMaterial::Dragonbone,
                    2 => WeaponMaterial::Demonic,
                    _ => WeaponMaterial::Divine,
                },
                match rng.gen_range(0..2) {
                    0 => WeaponQuality::Legendary,
                    _ => WeaponQuality::Mythic,
                }
            ),
            Rarity::Mythic => (
                match rng.gen_range(0..3) {
                    0 => WeaponMaterial::Divine,
                    1 => WeaponMaterial::Void,
                    _ => WeaponMaterial::Astral,
                },
                match rng.gen_range(0..2) {
                    0 => WeaponQuality::Mythic,
                    _ => WeaponQuality::Divine,
                }
            ),
        };

        let mut weapon = Weapon::new(weapon_type, material, quality);

        // Add enchantments based on rarity
        let num_enchants = match rarity {
            Rarity::Common => 0,
            Rarity::Uncommon => if rng.gen_bool(0.3) { 1 } else { 0 },
            Rarity::Rare => rng.gen_range(0..=1),
            Rarity::Epic => rng.gen_range(1..=2),
            Rarity::Legendary => rng.gen_range(2..=3),
            Rarity::Mythic => rng.gen_range(3..=5),
        };

        let possible_enchants = [
            WeaponEnchantment::Fire,
            WeaponEnchantment::Ice,
            WeaponEnchantment::Lightning,
            WeaponEnchantment::Poison,
            WeaponEnchantment::Holy,
            WeaponEnchantment::Vampiric,
            WeaponEnchantment::Keen,
            WeaponEnchantment::Brutal,
            WeaponEnchantment::Swift,
            WeaponEnchantment::LifeSteal,
        ];

        for _ in 0..num_enchants {
            let enchant = possible_enchants[rng.gen_range(0..possible_enchants.len())];
            weapon.add_enchantment(enchant);
        }

        // Add abilities based on rarity
        let num_abilities = match rarity {
            Rarity::Common | Rarity::Uncommon => 0,
            Rarity::Rare => if rng.gen_bool(0.4) { 1 } else { 0 },
            Rarity::Epic => rng.gen_range(0..=1),
            Rarity::Legendary => rng.gen_range(1..=2),
            Rarity::Mythic => rng.gen_range(2..=3),
        };

        let possible_abilities = [
            WeaponAbility::Cleave,
            WeaponAbility::Pierce,
            WeaponAbility::CriticalStrike,
            WeaponAbility::Parry,
            WeaponAbility::RapidStrike,
            WeaponAbility::LifeTap,
        ];

        for _ in 0..num_abilities {
            let ability = possible_abilities[rng.gen_range(0..possible_abilities.len())];
            weapon.add_ability(ability);
        }

        Self {
            x,
            y,
            kind,
            rarity,
            weapon_data: Some(weapon),
        }
    }
}

// ============================================================================
// FACTION SYSTEM
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
enum Faction {
    OrcHorde,
    UndeadLegion,
    DemonCult,
    DragonFlight,
    Druids,
    ElvenCourt,
    DwarvenKingdom,
    FeyWild,
    Celestials,
    Aberrations,
}

// ============================================================================
// ENEMY TYPES (60+ types including 6 bosses)
// ============================================================================

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
enum EnemyKind {
    // Tier 1: Dungeon (levels 1-4)
    Rat,
    Bat,
    Spider,
    Goblin,
    Skeleton,
    Kobold,
    GiantRat,
    CaveCrawler,

    // Tier 2: Cave (levels 5-8)
    GiantSpider,
    Orc,
    Troll,
    CaveOgre,
    Slime,
    Hobgoblin,
    CaveBear,
    Mushroom,
    RockElemental,

    // Tier 3: Crypt (levels 9-12)
    Zombie,
    Ghost,
    Wraith,
    Vampire,
    Mummy,
    Ghoul,
    Banshee,
    DeathKnight,
    BoneGolem,

    // Tier 4: Forest (levels 13-16)
    Wolf,
    DireWolf,
    TreeEnt,
    ForestTroll,
    Druid,
    WildBoar,
    GiantWasp,
    VenomousVine,
    ForestSpirit,

    // Tier 5: Ice Cavern (levels 17-20)
    IceElemental,
    FrostGiant,
    YetiWarrior,
    IceWraith,
    FrostWolf,
    IceSpider,
    FrozenKnight,
    Wendigo,

    // Tier 6: Volcanic (levels 21-24)
    FireElemental,
    LavaGolem,
    Hellhound,
    FireDrake,
    MagmaSlime,
    Salamander,
    CinderWraith,
    InfernalImp,

    // Tier 7: Ancient Ruins (levels 25-28)
    Golem,
    AncientGuardian,
    Sphinx,
    Lich,
    Gargoyle,
    MummyLord,
    CursedStatue,
    ShadowAssassin,

    // Tier 8: Demon Realm (levels 29-30)
    Demon,
    DemonLord,
    Succubus,
    Balrog,
    PitFiend,
    ShadowDemon,
    AbyssalHorror,
    DoomGuard,

    // Bosses (one per area)
    BossGoblinKing,      // Level 5
    BossOrcWarlord,      // Level 10
    BossVampireLord,     // Level 15
    BossForestGuardian,  // Level 20
    BossIceDragon,       // Level 25
    BossDemonKing,       // Level 30

    // Mini-Bosses
    GoblinChampion,
    OrcBerserker,
    VampireElite,
    AncientWyrm,
    FrostLord,
    InfernalLord,

    // ========================================================================
    // NEW ENEMY TYPES
    // ========================================================================

    // Elf enemies (levels 5-16)
    HighElfMage,
    WoodElfRanger,
    DarkElfAssassin,
    BloodElfWarlock,
    ElfBladeDancer,
    ElvenSentinel,

    // Dwarf enemies (levels 5-16)
    DwarfWarrior,
    DwarfRunesmith,
    DwarfBerserker,
    IronGolem,        // Dwarf construct
    DwarfKing,        // Boss

    // Dragon enemies (levels 13-30)
    Wyrmling,         // Baby dragon
    YoungDragon,
    AdultDragon,
    AncientDragon,
    DragonPriest,     // Dragon worshipper
    Dragonborn,       // Half-dragon humanoid

    // Demon enemies (expand existing) (levels 21-30)
    ImpSwarm,
    DemonHound,
    ChainDevil,
    BoneDemon,
    PlagueDemon,
    DreamDemon,
    ArchDemon,

    // Undead enemies (expand) (levels 9-20)
    SkeletonWarrior,
    SkeletonArcher,
    SkeletonMage,
    ZombieBrute,
    ZombieSpitter,
    VampireSpawn,
    VampireNoble,
    Wight,
    Specter,
    Poltergeist,
    RevenantKnight,

    // Beast enemies (levels 13-24)
    AlphaWolf,
    DireBoar,
    GiantEagle,
    Manticore,
    Chimera,
    Griffon,
    Hydra,
    Basilisk,
    Cockatrice,

    // Elemental enemies (levels 17-28)
    StormElemental,
    MagmaElemental,
    MudElemental,
    LightElemental,
    DarkElemental,
    VoidElemental,

    // Celestial enemies (levels 25-30)
    FallenSeraph,
    CorruptedCherub,
    AngelicGuard,

    // Fey enemies (levels 13-24)
    Pixie,
    Satyr,
    Dryad,
    Treant,
    Unicorn,         // Can be aggressive
    FeyDragon,
    Archfey,

    // Construct enemies (levels 21-28)
    ClockworkSoldier,
    AnimatedArmor,
    ShieldGuardian,
    Colossus,

    // Aberration enemies (levels 25-30)
    Beholder,
    MindFlayer,
    Aboleth,
    Gibbering,
    Otyugh,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum OrcSubspecies {
    GreenOrc,       // Standard, balanced warrior
    BlackOrc,       // Larger, stronger, slower
    GreyOrc,        // Smarter, can use magic
    RedOrc,         // Berserker, fire affinity
    PaleOrc,        // Cave dweller, stealth
    SeaOrc,         // Aquatic, pirate
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum GoblinSubspecies {
    CommonGoblin,   // Small, sneaky, traps
    Hobgoblin,      // Larger, militaristic
    Bugbear,        // Stealthy brute
    Nilbog,         // Chaos magic, unpredictable
    GoblinShaman,   // Spirit magic
    GoblinTinker,   // Machines, explosives
}

impl OrcSubspecies {
    fn stat_bonuses(&self) -> (i32, i32, i32, i32, i32) {
        match self {
            Self::GreenOrc => (30, 25, 15, 5, 5),
            Self::BlackOrc => (50, 35, 25, -10, 0),
            Self::GreyOrc => (20, 15, 10, 10, 25),
            Self::RedOrc => (25, 40, 10, 10, 10),
            Self::PaleOrc => (20, 20, 15, 15, 10),
            Self::SeaOrc => (30, 25, 20, 10, 5),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::GreenOrc => "Green Orc",
            Self::BlackOrc => "Black Orc",
            Self::GreyOrc => "Grey Orc",
            Self::RedOrc => "Red Orc",
            Self::PaleOrc => "Pale Orc",
            Self::SeaOrc => "Sea Orc",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::GreenOrc => "Common orcs. Strong berserkers",
            Self::BlackOrc => "Elite orcs. Devastating attackers, slow",
            Self::GreyOrc => "Tactical orcs. Balanced warriors with magic",
            Self::RedOrc => "Frenzied orcs. Fast, fire damage",
            Self::PaleOrc => "Cave orcs. Stealthy ambushers",
            Self::SeaOrc => "Aquatic orcs. Water combat masters",
        }
    }

    fn orc_rage(&self) -> &'static str {
        match self {
            Self::GreenOrc => "WAAAGH! - Battle cry buffs attack",
            Self::BlackOrc => "Unstoppable - Immune to CC when raging",
            Self::GreyOrc => "Cunning - Can cast while raging",
            Self::RedOrc => "Blood Rage - Fire damage while raging",
            Self::PaleOrc => "Silent Rage - Stealth while raging",
            Self::SeaOrc => "Sea Fury - Water combat mastery",
        }
    }
}

impl GoblinSubspecies {
    fn stat_bonuses(&self) -> (i32, i32, i32, i32, i32) {
        match self {
            Self::CommonGoblin => (-10, 10, -5, 25, 10),
            Self::Hobgoblin => (20, 20, 15, 10, 5),
            Self::Bugbear => (25, 25, 15, 15, 0),
            Self::Nilbog => (0, 15, 5, 20, 25),
            Self::GoblinShaman => (-5, 5, 0, 15, 35),
            Self::GoblinTinker => (10, 15, 10, 20, 20),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::CommonGoblin => "Common Goblin",
            Self::Hobgoblin => "Hobgoblin",
            Self::Bugbear => "Bugbear",
            Self::Nilbog => "Nilbog",
            Self::GoblinShaman => "Goblin Shaman",
            Self::GoblinTinker => "Goblin Tinker",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::CommonGoblin => "Stealthy scouts. Fast, good at traps",
            Self::Hobgoblin => "Warrior goblins. Disciplined fighters",
            Self::Bugbear => "Large goblins. Strong ambushers",
            Self::Nilbog => "Chaos goblins. Powerful wild magic",
            Self::GoblinShaman => "Spirit callers. Summon ancestral spirits",
            Self::GoblinTinker => "Inventors. Create explosives and machines",
        }
    }

    fn goblin_trick(&self) -> &'static str {
        match self {
            Self::CommonGoblin => "Trap Master - Set deadly traps",
            Self::Hobgoblin => "Formation - Tactical combat bonuses",
            Self::Bugbear => "Ambush Expert - Surprise attack damage",
            Self::Nilbog => "Chaos Touch - Random magical effects",
            Self::GoblinShaman => "Spirit Call - Summon ancestral spirits",
            Self::GoblinTinker => "Bomb Craft - Create explosive devices",
        }
    }
}

impl EnemyKind {
    fn glyph(&self) -> char {
        match self {
            Self::Rat | Self::GiantRat => 'r',
            Self::Bat => 'b',
            Self::Spider => 's',
            Self::Goblin => 'g',
            Self::Skeleton => 'k',
            Self::Kobold => 'k',
            Self::CaveCrawler => 'c',
            Self::GiantSpider => 'S',
            Self::Orc => 'o',
            Self::Troll => 't',
            Self::CaveOgre => 'O',
            Self::Slime | Self::MagmaSlime => 'j',
            Self::Hobgoblin => 'h',
            Self::CaveBear => 'B',
            Self::Mushroom => 'm',
            Self::RockElemental => 'R',
            Self::Zombie => 'z',
            Self::Ghost => 'G',
            Self::Wraith | Self::CinderWraith => 'W',
            Self::Vampire | Self::VampireElite => 'V',
            Self::Mummy => 'M',
            Self::Ghoul => 'g',
            Self::Banshee => 'B',
            Self::DeathKnight => 'K',
            Self::BoneGolem => 'G',
            Self::Wolf | Self::FrostWolf => 'w',
            Self::DireWolf => 'W',
            Self::TreeEnt => 'T',
            Self::ForestTroll => 't',
            Self::Druid => 'd',
            Self::WildBoar => 'b',
            Self::GiantWasp => 'w',
            Self::VenomousVine => 'v',
            Self::ForestSpirit => 'S',
            Self::IceElemental => 'E',
            Self::FrostGiant => 'F',
            Self::YetiWarrior => 'Y',
            Self::IceWraith => 'w',
            Self::IceSpider => 'S',
            Self::FrozenKnight => 'K',
            Self::Wendigo => 'W',
            Self::FireElemental => 'E',
            Self::LavaGolem => 'L',
            Self::Hellhound => 'H',
            Self::FireDrake => 'D',
            Self::Salamander => 's',
            Self::InfernalImp => 'i',
            Self::Golem => 'G',
            Self::AncientGuardian => 'A',
            Self::Sphinx => 'X',
            Self::Lich => 'L',
            Self::Gargoyle => 'G',
            Self::MummyLord => 'M',
            Self::CursedStatue => 'S',
            Self::ShadowAssassin => 'a',
            Self::Demon => 'D',
            Self::DemonLord => '&',
            Self::Succubus => 's',
            Self::Balrog => 'B',
            Self::PitFiend => 'P',
            Self::ShadowDemon => 'S',
            Self::AbyssalHorror => 'H',
            Self::DoomGuard => 'D',
            Self::BossGoblinKing | Self::GoblinChampion => 'K',
            Self::BossOrcWarlord | Self::OrcBerserker => 'W',
            Self::BossVampireLord => 'V',
            Self::BossForestGuardian | Self::AncientWyrm => 'G',
            Self::BossIceDragon | Self::FrostLord => 'D',
            Self::BossDemonKing | Self::InfernalLord => '&',

            // Elf enemies
            Self::HighElfMage => 'e',
            Self::WoodElfRanger => 'e',
            Self::DarkElfAssassin => 'e',
            Self::BloodElfWarlock => 'e',
            Self::ElfBladeDancer => 'e',
            Self::ElvenSentinel => 'E',

            // Dwarf enemies
            Self::DwarfWarrior => 'd',
            Self::DwarfRunesmith => 'd',
            Self::DwarfBerserker => 'd',
            Self::IronGolem => 'I',
            Self::DwarfKing => 'K',

            // Dragon enemies
            Self::Wyrmling => 'd',
            Self::YoungDragon => 'D',
            Self::AdultDragon => 'D',
            Self::AncientDragon => 'D',
            Self::DragonPriest => 'p',
            Self::Dragonborn => 'D',

            // Demon enemies (expanded)
            Self::ImpSwarm => 'i',
            Self::DemonHound => 'h',
            Self::ChainDevil => 'C',
            Self::BoneDemon => 'B',
            Self::PlagueDemon => 'P',
            Self::DreamDemon => 'D',
            Self::ArchDemon => '&',

            // Undead enemies (expanded)
            Self::SkeletonWarrior => 'k',
            Self::SkeletonArcher => 'k',
            Self::SkeletonMage => 'k',
            Self::ZombieBrute => 'Z',
            Self::ZombieSpitter => 'z',
            Self::VampireSpawn => 'v',
            Self::VampireNoble => 'V',
            Self::Wight => 'W',
            Self::Specter => 'S',
            Self::Poltergeist => 'p',
            Self::RevenantKnight => 'R',

            // Beast enemies
            Self::AlphaWolf => 'W',
            Self::DireBoar => 'B',
            Self::GiantEagle => 'E',
            Self::Manticore => 'M',
            Self::Chimera => 'C',
            Self::Griffon => 'G',
            Self::Hydra => 'H',
            Self::Basilisk => 'B',
            Self::Cockatrice => 'c',

            // Elemental enemies
            Self::StormElemental => 'E',
            Self::MagmaElemental => 'E',
            Self::MudElemental => 'E',
            Self::LightElemental => 'E',
            Self::DarkElemental => 'E',
            Self::VoidElemental => 'V',

            // Celestial enemies
            Self::FallenSeraph => 'A',
            Self::CorruptedCherub => 'a',
            Self::AngelicGuard => 'A',

            // Fey enemies
            Self::Pixie => 'p',
            Self::Satyr => 's',
            Self::Dryad => 'd',
            Self::Treant => 'T',
            Self::Unicorn => 'U',
            Self::FeyDragon => 'F',
            Self::Archfey => 'A',

            // Construct enemies
            Self::ClockworkSoldier => 'C',
            Self::AnimatedArmor => 'A',
            Self::ShieldGuardian => 'S',
            Self::Colossus => 'C',

            // Aberration enemies
            Self::Beholder => '@',
            Self::MindFlayer => 'M',
            Self::Aboleth => 'A',
            Self::Gibbering => 'g',
            Self::Otyugh => 'O',
        }
    }

    fn color(&self) -> Color {
        match self {
            Self::Rat | Self::Bat | Self::GiantRat | Self::CaveCrawler => Color::DarkGrey,
            Self::Spider | Self::GiantSpider | Self::IceSpider => Color::DarkYellow,
            Self::Goblin | Self::BossGoblinKing | Self::GoblinChampion | Self::Kobold => Color::Green,
            Self::Skeleton | Self::Mummy | Self::BoneGolem => Color::White,
            Self::Orc | Self::BossOrcWarlord | Self::OrcBerserker | Self::Hobgoblin => Color::DarkGreen,
            Self::Troll | Self::ForestTroll => Color::DarkCyan,
            Self::CaveOgre | Self::CaveBear => Color::DarkYellow,
            Self::Slime | Self::Mushroom => Color::Green,
            Self::RockElemental => Color::Grey,
            Self::Zombie | Self::Ghoul => Color::DarkGreen,
            Self::Ghost | Self::Wraith | Self::IceWraith | Self::Banshee => Color::Grey,
            Self::Vampire | Self::BossVampireLord | Self::VampireElite => Color::DarkRed,
            Self::DeathKnight => Color::DarkMagenta,
            Self::Wolf | Self::DireWolf | Self::FrostWolf => Color::Grey,
            Self::TreeEnt | Self::BossForestGuardian | Self::VenomousVine => Color::Green,
            Self::Druid | Self::ForestSpirit => Color::DarkGreen,
            Self::WildBoar => Color::DarkYellow,
            Self::GiantWasp => Color::Yellow,
            Self::IceElemental | Self::FrostGiant | Self::YetiWarrior | Self::BossIceDragon
            | Self::FrozenKnight | Self::Wendigo | Self::FrostLord => Color::Cyan,
            Self::FireElemental | Self::LavaGolem | Self::Hellhound | Self::FireDrake
            | Self::MagmaSlime | Self::Salamander | Self::CinderWraith | Self::InfernalImp
            | Self::InfernalLord => Color::Red,
            Self::Golem | Self::AncientGuardian | Self::CursedStatue => Color::Yellow,
            Self::Sphinx | Self::Gargoyle => Color::Yellow,
            Self::Lich | Self::MummyLord => Color::Magenta,
            Self::ShadowAssassin => Color::DarkGrey,
            Self::Demon | Self::DemonLord | Self::Balrog | Self::BossDemonKing
            | Self::PitFiend | Self::DoomGuard => Color::Red,
            Self::Succubus | Self::ShadowDemon => Color::Magenta,
            Self::AbyssalHorror => Color::DarkRed,
            Self::AncientWyrm => Color::Green,

            // Elf enemies
            Self::HighElfMage => Color::Cyan,
            Self::WoodElfRanger => Color::Green,
            Self::DarkElfAssassin => Color::DarkMagenta,
            Self::BloodElfWarlock => Color::DarkRed,
            Self::ElfBladeDancer => Color::White,
            Self::ElvenSentinel => Color::Yellow,

            // Dwarf enemies
            Self::DwarfWarrior => Color::DarkYellow,
            Self::DwarfRunesmith => Color::Cyan,
            Self::DwarfBerserker => Color::Red,
            Self::IronGolem => Color::Grey,
            Self::DwarfKing => Color::Yellow,

            // Dragon enemies
            Self::Wyrmling => Color::Green,
            Self::YoungDragon => Color::Yellow,
            Self::AdultDragon => Color::Red,
            Self::AncientDragon => Color::Magenta,
            Self::DragonPriest => Color::DarkRed,
            Self::Dragonborn => Color::DarkYellow,

            // Demon enemies (expanded)
            Self::ImpSwarm => Color::Red,
            Self::DemonHound => Color::DarkRed,
            Self::ChainDevil => Color::DarkGrey,
            Self::BoneDemon => Color::White,
            Self::PlagueDemon => Color::DarkGreen,
            Self::DreamDemon => Color::Magenta,
            Self::ArchDemon => Color::Red,

            // Undead enemies (expanded)
            Self::SkeletonWarrior | Self::SkeletonArcher | Self::SkeletonMage => Color::White,
            Self::ZombieBrute | Self::ZombieSpitter => Color::DarkGreen,
            Self::VampireSpawn | Self::VampireNoble => Color::DarkRed,
            Self::Wight => Color::DarkCyan,
            Self::Specter | Self::Poltergeist => Color::Grey,
            Self::RevenantKnight => Color::DarkMagenta,

            // Beast enemies
            Self::AlphaWolf => Color::Grey,
            Self::DireBoar => Color::DarkYellow,
            Self::GiantEagle => Color::Yellow,
            Self::Manticore => Color::DarkYellow,
            Self::Chimera => Color::DarkRed,
            Self::Griffon => Color::Yellow,
            Self::Hydra => Color::DarkGreen,
            Self::Basilisk => Color::DarkYellow,
            Self::Cockatrice => Color::Yellow,

            // Elemental enemies
            Self::StormElemental => Color::Cyan,
            Self::MagmaElemental => Color::Red,
            Self::MudElemental => Color::DarkYellow,
            Self::LightElemental => Color::White,
            Self::DarkElemental => Color::DarkGrey,
            Self::VoidElemental => Color::Magenta,

            // Celestial enemies
            Self::FallenSeraph => Color::DarkMagenta,
            Self::CorruptedCherub => Color::DarkRed,
            Self::AngelicGuard => Color::White,

            // Fey enemies
            Self::Pixie => Color::Magenta,
            Self::Satyr => Color::DarkYellow,
            Self::Dryad => Color::Green,
            Self::Treant => Color::DarkGreen,
            Self::Unicorn => Color::White,
            Self::FeyDragon => Color::Cyan,
            Self::Archfey => Color::Magenta,

            // Construct enemies
            Self::ClockworkSoldier => Color::DarkYellow,
            Self::AnimatedArmor => Color::Grey,
            Self::ShieldGuardian => Color::Yellow,
            Self::Colossus => Color::DarkGrey,

            // Aberration enemies
            Self::Beholder => Color::Magenta,
            Self::MindFlayer => Color::DarkMagenta,
            Self::Aboleth => Color::DarkCyan,
            Self::Gibbering => Color::DarkYellow,
            Self::Otyugh => Color::DarkGreen,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Rat => "Rat",
            Self::Bat => "Bat",
            Self::Spider => "Spider",
            Self::Goblin => "Goblin",
            Self::Skeleton => "Skeleton",
            Self::Kobold => "Kobold",
            Self::GiantRat => "Giant Rat",
            Self::CaveCrawler => "Cave Crawler",
            Self::GiantSpider => "Giant Spider",
            Self::Orc => "Orc",
            Self::Troll => "Troll",
            Self::CaveOgre => "Cave Ogre",
            Self::Slime => "Slime",
            Self::Hobgoblin => "Hobgoblin",
            Self::CaveBear => "Cave Bear",
            Self::Mushroom => "Toxic Mushroom",
            Self::RockElemental => "Rock Elemental",
            Self::Zombie => "Zombie",
            Self::Ghost => "Ghost",
            Self::Wraith => "Wraith",
            Self::Vampire => "Vampire",
            Self::Mummy => "Mummy",
            Self::Ghoul => "Ghoul",
            Self::Banshee => "Banshee",
            Self::DeathKnight => "Death Knight",
            Self::BoneGolem => "Bone Golem",
            Self::Wolf => "Wolf",
            Self::DireWolf => "Dire Wolf",
            Self::TreeEnt => "Tree Ent",
            Self::ForestTroll => "Forest Troll",
            Self::Druid => "Corrupted Druid",
            Self::WildBoar => "Wild Boar",
            Self::GiantWasp => "Giant Wasp",
            Self::VenomousVine => "Venomous Vine",
            Self::ForestSpirit => "Forest Spirit",
            Self::IceElemental => "Ice Elemental",
            Self::FrostGiant => "Frost Giant",
            Self::YetiWarrior => "Yeti Warrior",
            Self::IceWraith => "Ice Wraith",
            Self::FrostWolf => "Frost Wolf",
            Self::IceSpider => "Ice Spider",
            Self::FrozenKnight => "Frozen Knight",
            Self::Wendigo => "Wendigo",
            Self::FireElemental => "Fire Elemental",
            Self::LavaGolem => "Lava Golem",
            Self::Hellhound => "Hellhound",
            Self::FireDrake => "Fire Drake",
            Self::MagmaSlime => "Magma Slime",
            Self::Salamander => "Salamander",
            Self::CinderWraith => "Cinder Wraith",
            Self::InfernalImp => "Infernal Imp",
            Self::Golem => "Stone Golem",
            Self::AncientGuardian => "Ancient Guardian",
            Self::Sphinx => "Sphinx",
            Self::Lich => "Lich",
            Self::Gargoyle => "Gargoyle",
            Self::MummyLord => "Mummy Lord",
            Self::CursedStatue => "Cursed Statue",
            Self::ShadowAssassin => "Shadow Assassin",
            Self::Demon => "Demon",
            Self::DemonLord => "Demon Lord",
            Self::Succubus => "Succubus",
            Self::Balrog => "Balrog",
            Self::PitFiend => "Pit Fiend",
            Self::ShadowDemon => "Shadow Demon",
            Self::AbyssalHorror => "Abyssal Horror",
            Self::DoomGuard => "Doom Guard",
            Self::BossGoblinKing => "GOBLIN KING",
            Self::BossOrcWarlord => "ORC WARLORD",
            Self::BossVampireLord => "VAMPIRE LORD",
            Self::BossForestGuardian => "FOREST GUARDIAN",
            Self::BossIceDragon => "ICE DRAGON",
            Self::BossDemonKing => "DEMON KING",
            Self::GoblinChampion => "Goblin Champion",
            Self::OrcBerserker => "Orc Berserker",
            Self::VampireElite => "Vampire Elite",
            Self::AncientWyrm => "Ancient Wyrm",
            Self::FrostLord => "Frost Lord",
            Self::InfernalLord => "Infernal Lord",

            // Elf enemies
            Self::HighElfMage => "High Elf Mage",
            Self::WoodElfRanger => "Wood Elf Ranger",
            Self::DarkElfAssassin => "Dark Elf Assassin",
            Self::BloodElfWarlock => "Blood Elf Warlock",
            Self::ElfBladeDancer => "Elf Blade Dancer",
            Self::ElvenSentinel => "Elven Sentinel",

            // Dwarf enemies
            Self::DwarfWarrior => "Dwarf Warrior",
            Self::DwarfRunesmith => "Dwarf Runesmith",
            Self::DwarfBerserker => "Dwarf Berserker",
            Self::IronGolem => "Iron Golem",
            Self::DwarfKing => "DWARF KING",

            // Dragon enemies
            Self::Wyrmling => "Wyrmling",
            Self::YoungDragon => "Young Dragon",
            Self::AdultDragon => "Adult Dragon",
            Self::AncientDragon => "ANCIENT DRAGON",
            Self::DragonPriest => "Dragon Priest",
            Self::Dragonborn => "Dragonborn",

            // Demon enemies (expanded)
            Self::ImpSwarm => "Imp Swarm",
            Self::DemonHound => "Demon Hound",
            Self::ChainDevil => "Chain Devil",
            Self::BoneDemon => "Bone Demon",
            Self::PlagueDemon => "Plague Demon",
            Self::DreamDemon => "Dream Demon",
            Self::ArchDemon => "ARCHDEMON",

            // Undead enemies (expanded)
            Self::SkeletonWarrior => "Skeleton Warrior",
            Self::SkeletonArcher => "Skeleton Archer",
            Self::SkeletonMage => "Skeleton Mage",
            Self::ZombieBrute => "Zombie Brute",
            Self::ZombieSpitter => "Zombie Spitter",
            Self::VampireSpawn => "Vampire Spawn",
            Self::VampireNoble => "Vampire Noble",
            Self::Wight => "Wight",
            Self::Specter => "Specter",
            Self::Poltergeist => "Poltergeist",
            Self::RevenantKnight => "Revenant Knight",

            // Beast enemies
            Self::AlphaWolf => "Alpha Wolf",
            Self::DireBoar => "Dire Boar",
            Self::GiantEagle => "Giant Eagle",
            Self::Manticore => "Manticore",
            Self::Chimera => "Chimera",
            Self::Griffon => "Griffon",
            Self::Hydra => "Hydra",
            Self::Basilisk => "Basilisk",
            Self::Cockatrice => "Cockatrice",

            // Elemental enemies
            Self::StormElemental => "Storm Elemental",
            Self::MagmaElemental => "Magma Elemental",
            Self::MudElemental => "Mud Elemental",
            Self::LightElemental => "Light Elemental",
            Self::DarkElemental => "Dark Elemental",
            Self::VoidElemental => "Void Elemental",

            // Celestial enemies
            Self::FallenSeraph => "Fallen Seraph",
            Self::CorruptedCherub => "Corrupted Cherub",
            Self::AngelicGuard => "Angelic Guard",

            // Fey enemies
            Self::Pixie => "Pixie",
            Self::Satyr => "Satyr",
            Self::Dryad => "Dryad",
            Self::Treant => "Treant",
            Self::Unicorn => "Unicorn",
            Self::FeyDragon => "Fey Dragon",
            Self::Archfey => "ARCHFEY",

            // Construct enemies
            Self::ClockworkSoldier => "Clockwork Soldier",
            Self::AnimatedArmor => "Animated Armor",
            Self::ShieldGuardian => "Shield Guardian",
            Self::Colossus => "COLOSSUS",

            // Aberration enemies
            Self::Beholder => "Beholder",
            Self::MindFlayer => "Mind Flayer",
            Self::Aboleth => "Aboleth",
            Self::Gibbering => "Gibbering Mouther",
            Self::Otyugh => "Otyugh",

            // Default for new enemies
            _ => "Unknown Enemy",
        }
    }

    fn plural_name(&self) -> &'static str {
        match self {
            Self::Rat => "Rats",
            Self::Bat => "Bats",
            Self::Spider => "Spiders",
            Self::Goblin => "Goblins",
            Self::Skeleton => "Skeletons",
            Self::Kobold => "Kobolds",
            Self::GiantRat => "Giant Rats",
            Self::CaveCrawler => "Cave Crawlers",
            Self::GiantSpider => "Giant Spiders",
            Self::Orc => "Orcs",
            Self::Troll => "Trolls",
            Self::CaveOgre => "Cave Ogres",
            Self::Slime => "Slimes",
            Self::Hobgoblin => "Hobgoblins",
            Self::CaveBear => "Cave Bears",
            Self::Mushroom => "Toxic Mushrooms",
            Self::RockElemental => "Rock Elementals",
            Self::Zombie => "Zombies",
            Self::Ghost => "Ghosts",
            Self::Wraith => "Wraiths",
            Self::Vampire => "Vampires",
            Self::Mummy => "Mummies",
            Self::Ghoul => "Ghouls",
            Self::Banshee => "Banshees",
            Self::DeathKnight => "Death Knights",
            Self::BoneGolem => "Bone Golems",
            Self::Wolf => "Wolves",
            Self::DireWolf => "Dire Wolves",
            Self::TreeEnt => "Tree Ents",
            Self::ForestTroll => "Forest Trolls",
            Self::Druid => "Corrupted Druids",
            Self::WildBoar => "Wild Boars",
            Self::GiantWasp => "Giant Wasps",
            Self::VenomousVine => "Venomous Vines",
            Self::ForestSpirit => "Forest Spirits",
            Self::IceElemental => "Ice Elementals",
            Self::FrostGiant => "Frost Giants",
            Self::YetiWarrior => "Yeti Warriors",
            Self::IceWraith => "Ice Wraiths",
            Self::FrostWolf => "Frost Wolves",
            Self::IceSpider => "Ice Spiders",
            Self::FrozenKnight => "Frozen Knights",
            Self::Wendigo => "Wendigos",
            Self::FireElemental => "Fire Elementals",
            Self::LavaGolem => "Lava Golems",
            Self::Hellhound => "Hellhounds",
            Self::FireDrake => "Fire Drakes",
            Self::MagmaSlime => "Magma Slimes",
            Self::Salamander => "Salamanders",
            Self::CinderWraith => "Cinder Wraiths",
            Self::InfernalImp => "Infernal Imps",
            Self::Golem => "Stone Golems",
            Self::AncientGuardian => "Ancient Guardians",
            Self::Sphinx => "Sphinxes",
            Self::Lich => "Liches",
            Self::Gargoyle => "Gargoyles",
            Self::MummyLord => "Mummy Lords",
            Self::CursedStatue => "Cursed Statues",
            Self::ShadowAssassin => "Shadow Assassins",
            Self::Demon => "Demons",
            Self::DemonLord => "Demon Lords",
            Self::Succubus => "Succubi",
            Self::Balrog => "Balrogs",
            Self::PitFiend => "Pit Fiends",
            Self::ShadowDemon => "Shadow Demons",
            Self::AbyssalHorror => "Abyssal Horrors",
            Self::DoomGuard => "Doom Guards",
            _ => "Enemies",
        }
    }

    fn base_stats(&self) -> (i32, i32, i32, i32) {
        // (hp, attack, defense, xp_value)
        match self {
            // Tier 1: Dungeon
            Self::Rat => (8, 3, 0, 5),
            Self::Bat => (6, 2, 0, 4),
            Self::Spider => (10, 4, 1, 8),
            Self::Goblin => (15, 5, 2, 12),
            Self::Skeleton => (12, 6, 1, 10),
            Self::Kobold => (10, 4, 1, 7),
            Self::GiantRat => (14, 5, 1, 10),
            Self::CaveCrawler => (18, 6, 2, 15),

            // Tier 2: Cave
            Self::GiantSpider => (25, 8, 3, 25),
            Self::Orc => (35, 10, 4, 30),
            Self::Troll => (50, 8, 6, 40),
            Self::CaveOgre => (60, 12, 5, 50),
            Self::Slime => (40, 6, 8, 35),
            Self::Hobgoblin => (30, 9, 4, 28),
            Self::CaveBear => (55, 14, 6, 45),
            Self::Mushroom => (20, 5, 2, 20),
            Self::RockElemental => (70, 10, 12, 55),

            // Tier 3: Crypt
            Self::Zombie => (45, 10, 4, 40),
            Self::Ghost => (30, 12, 2, 45),
            Self::Wraith => (35, 15, 3, 55),
            Self::Vampire => (55, 14, 6, 70),
            Self::Mummy => (50, 11, 8, 60),
            Self::Ghoul => (40, 12, 3, 50),
            Self::Banshee => (35, 16, 2, 65),
            Self::DeathKnight => (75, 18, 10, 85),
            Self::BoneGolem => (90, 14, 12, 80),

            // Tier 4: Forest
            Self::Wolf => (40, 12, 3, 50),
            Self::DireWolf => (60, 16, 5, 75),
            Self::TreeEnt => (100, 14, 12, 100),
            Self::ForestTroll => (80, 15, 8, 90),
            Self::Druid => (50, 18, 4, 80),
            Self::WildBoar => (55, 14, 5, 60),
            Self::GiantWasp => (35, 16, 2, 55),
            Self::VenomousVine => (45, 12, 6, 65),
            Self::ForestSpirit => (40, 20, 3, 75),

            // Tier 5: Ice Cavern
            Self::IceElemental => (70, 18, 8, 110),
            Self::FrostGiant => (120, 22, 12, 150),
            Self::YetiWarrior => (90, 20, 10, 130),
            Self::IceWraith => (60, 24, 6, 120),
            Self::FrostWolf => (55, 18, 6, 100),
            Self::IceSpider => (50, 16, 5, 95),
            Self::FrozenKnight => (100, 22, 14, 140),
            Self::Wendigo => (85, 26, 8, 160),

            // Tier 6: Volcanic
            Self::FireElemental => (80, 22, 8, 140),
            Self::LavaGolem => (150, 20, 18, 180),
            Self::Hellhound => (70, 25, 8, 150),
            Self::FireDrake => (100, 28, 12, 200),
            Self::MagmaSlime => (65, 18, 10, 130),
            Self::Salamander => (75, 24, 7, 160),
            Self::CinderWraith => (55, 28, 4, 170),
            Self::InfernalImp => (45, 22, 5, 120),

            // Tier 7: Ancient Ruins
            Self::Golem => (180, 22, 20, 220),
            Self::AncientGuardian => (200, 25, 22, 250),
            Self::Sphinx => (150, 30, 15, 280),
            Self::Lich => (120, 35, 12, 300),
            Self::Gargoyle => (130, 24, 18, 200),
            Self::MummyLord => (160, 28, 16, 260),
            Self::CursedStatue => (220, 20, 25, 240),
            Self::ShadowAssassin => (80, 40, 8, 280),

            // Tier 8: Demon Realm
            Self::Demon => (140, 30, 15, 280),
            Self::DemonLord => (200, 35, 20, 350),
            Self::Succubus => (100, 28, 10, 250),
            Self::Balrog => (250, 40, 25, 400),
            Self::PitFiend => (180, 38, 18, 380),
            Self::ShadowDemon => (120, 35, 12, 320),
            Self::AbyssalHorror => (300, 45, 22, 450),
            Self::DoomGuard => (220, 42, 20, 420),

            // Bosses
            Self::BossGoblinKing => (200, 20, 10, 500),
            Self::BossOrcWarlord => (400, 30, 15, 1000),
            Self::BossVampireLord => (600, 40, 20, 2000),
            Self::BossForestGuardian => (800, 45, 25, 3000),
            Self::BossIceDragon => (1200, 55, 30, 5000),
            Self::BossDemonKing => (2000, 70, 40, 10000),

            // Mini-Bosses
            Self::GoblinChampion => (150, 18, 8, 300),
            Self::OrcBerserker => (250, 28, 12, 600),
            Self::VampireElite => (350, 35, 18, 1200),
            Self::AncientWyrm => (500, 40, 22, 1800),
            Self::FrostLord => (700, 48, 28, 3500),
            Self::InfernalLord => (900, 55, 32, 5500),

            // Default for new enemies
            _ => (100, 20, 10, 200),
        }
    }

    fn is_boss(&self) -> bool {
        matches!(
            self,
            Self::BossGoblinKing
                | Self::BossOrcWarlord
                | Self::BossVampireLord
                | Self::BossForestGuardian
                | Self::BossIceDragon
                | Self::BossDemonKing
                | Self::GoblinChampion
                | Self::OrcBerserker
                | Self::VampireElite
                | Self::AncientWyrm
                | Self::FrostLord
                | Self::InfernalLord
        )
    }

    fn is_undead(&self) -> bool {
        matches!(
            self,
            Self::Skeleton | Self::Zombie | Self::Ghost | Self::Wraith
            | Self::Vampire | Self::Mummy | Self::Lich | Self::BossVampireLord
            | Self::Ghoul | Self::Banshee | Self::DeathKnight | Self::BoneGolem
            | Self::VampireElite | Self::MummyLord | Self::CinderWraith
            // New undead
            | Self::SkeletonWarrior | Self::SkeletonArcher | Self::SkeletonMage
            | Self::ZombieBrute | Self::ZombieSpitter | Self::VampireSpawn
            | Self::VampireNoble | Self::Wight | Self::Specter | Self::Poltergeist
            | Self::RevenantKnight | Self::BoneDemon
        )
    }

    fn can_poison(&self) -> bool {
        matches!(self, Self::Spider | Self::GiantSpider | Self::Slime
            | Self::Mushroom | Self::VenomousVine | Self::GiantWasp | Self::IceSpider
            | Self::PlagueDemon | Self::Cockatrice | Self::Otyugh)
    }

    fn can_burn(&self) -> bool {
        matches!(
            self,
            Self::FireElemental | Self::LavaGolem | Self::Hellhound | Self::FireDrake
            | Self::Demon | Self::DemonLord | Self::Balrog | Self::BossDemonKing
            | Self::MagmaSlime | Self::Salamander | Self::CinderWraith | Self::InfernalImp
            | Self::PitFiend | Self::InfernalLord
            // New fire enemies
            | Self::MagmaElemental | Self::ArchDemon | Self::AdultDragon | Self::AncientDragon
            | Self::BloodElfWarlock
        )
    }

    fn can_freeze(&self) -> bool {
        matches!(
            self,
            Self::IceElemental | Self::FrostGiant | Self::YetiWarrior
            | Self::IceWraith | Self::BossIceDragon | Self::FrostWolf
            | Self::IceSpider | Self::FrozenKnight | Self::Wendigo | Self::FrostLord
            // New ice enemies
            | Self::StormElemental | Self::FeyDragon
        )
    }

    fn can_bleed(&self) -> bool {
        matches!(
            self,
            Self::Wolf | Self::DireWolf | Self::Skeleton | Self::DeathKnight
            | Self::ShadowAssassin | Self::Vampire | Self::Ghoul
            | Self::BossVampireLord | Self::BossOrcWarlord
            // New bleed enemies
            | Self::DarkElfAssassin | Self::ElfBladeDancer | Self::DwarfBerserker
            | Self::AlphaWolf | Self::Manticore | Self::Chimera | Self::VampireNoble
        )
    }

    fn for_level(level: u32, rng: &mut impl Rng) -> Self {
        let enemies: Vec<Self> = match level {
            // Tier 1: Dungeon (levels 1-4)
            1..=4 => vec![
                Self::Rat, Self::Bat, Self::Spider, Self::Goblin, Self::Skeleton,
                Self::Kobold, Self::GiantRat, Self::CaveCrawler,
            ],
            // Tier 2: Cave (levels 5-8) + Elves/Dwarves intro
            5..=8 => vec![
                Self::GiantSpider, Self::Orc, Self::Troll, Self::CaveOgre, Self::Slime,
                Self::Hobgoblin, Self::CaveBear, Self::Mushroom, Self::RockElemental,
                Self::GoblinChampion,
                // New: Elves and Dwarves
                Self::HighElfMage, Self::WoodElfRanger, Self::DwarfWarrior, Self::DwarfRunesmith,
            ],
            // Tier 3: Crypt (levels 9-12) + More undead
            9..=12 => vec![
                Self::Zombie, Self::Ghost, Self::Wraith, Self::Vampire, Self::Mummy,
                Self::Ghoul, Self::Banshee, Self::DeathKnight, Self::BoneGolem,
                Self::OrcBerserker,
                // New: Expanded undead + more elves/dwarves
                Self::SkeletonWarrior, Self::SkeletonArcher, Self::SkeletonMage,
                Self::ZombieBrute, Self::ZombieSpitter, Self::VampireSpawn, Self::Wight,
                Self::DarkElfAssassin, Self::ElfBladeDancer, Self::DwarfBerserker,
            ],
            // Tier 4: Forest (levels 13-16) + Fey + Dragons intro + Beasts
            13..=16 => vec![
                Self::Wolf, Self::DireWolf, Self::TreeEnt, Self::ForestTroll, Self::Druid,
                Self::WildBoar, Self::GiantWasp, Self::VenomousVine, Self::ForestSpirit,
                Self::VampireElite,
                // New: Fey, Dragons, Beasts
                Self::Pixie, Self::Satyr, Self::Dryad, Self::Treant, Self::Unicorn,
                Self::Wyrmling, Self::Dragonborn, Self::AlphaWolf, Self::DireBoar,
                Self::GiantEagle, Self::Cockatrice, Self::BloodElfWarlock, Self::ElvenSentinel,
                Self::Specter, Self::Poltergeist, Self::VampireNoble,
            ],
            // Tier 5: Ice Cavern (levels 17-20) + More elementals + Advanced undead
            17..=20 => vec![
                Self::IceElemental, Self::FrostGiant, Self::YetiWarrior, Self::IceWraith,
                Self::FrostWolf, Self::IceSpider, Self::FrozenKnight, Self::Wendigo,
                Self::AncientWyrm,
                // New: Elementals, Dragons, Beasts
                Self::StormElemental, Self::YoungDragon, Self::DragonPriest,
                Self::Manticore, Self::Griffon, Self::Basilisk, Self::FeyDragon,
                Self::RevenantKnight, Self::IronGolem,
            ],
            // Tier 6: Volcanic (levels 21-24) + Demons + Constructs
            21..=24 => vec![
                Self::FireElemental, Self::LavaGolem, Self::Hellhound, Self::FireDrake,
                Self::MagmaSlime, Self::Salamander, Self::CinderWraith, Self::InfernalImp,
                Self::FrostLord,
                // New: Demons, Constructs, Beasts, Elementals
                Self::ImpSwarm, Self::DemonHound, Self::ChainDevil, Self::MagmaElemental,
                Self::MudElemental, Self::ClockworkSoldier, Self::AnimatedArmor,
                Self::AdultDragon, Self::Chimera, Self::Hydra, Self::Archfey,
            ],
            // Tier 7: Ancient Ruins (levels 25-28) + Aberrations + Celestials
            25..=28 => vec![
                Self::Golem, Self::AncientGuardian, Self::Sphinx, Self::Lich,
                Self::Gargoyle, Self::MummyLord, Self::CursedStatue, Self::ShadowAssassin,
                Self::InfernalLord,
                // New: Aberrations, Celestials, Advanced demons, Constructs
                Self::Beholder, Self::MindFlayer, Self::Aboleth, Self::Gibbering, Self::Otyugh,
                Self::FallenSeraph, Self::CorruptedCherub, Self::AngelicGuard,
                Self::BoneDemon, Self::PlagueDemon, Self::DreamDemon,
                Self::ShieldGuardian, Self::LightElemental, Self::DarkElemental, Self::VoidElemental,
            ],
            // Tier 8: Demon Realm (levels 29-30) + Final bosses
            _ => vec![
                Self::Demon, Self::DemonLord, Self::Succubus, Self::Balrog,
                Self::PitFiend, Self::ShadowDemon, Self::AbyssalHorror, Self::DoomGuard,
                // New: Ultimate enemies
                Self::ArchDemon, Self::AncientDragon, Self::Colossus, Self::DwarfKing,
            ],
        };
        enemies[rng.gen_range(0..enemies.len())]
    }

    fn boss_for_level(level: u32) -> Option<Self> {
        match level {
            5 => Some(Self::BossGoblinKing),
            10 => Some(Self::BossOrcWarlord),
            12 => Some(Self::DwarfKing),      // New boss
            15 => Some(Self::BossVampireLord),
            18 => Some(Self::Archfey),        // New boss
            20 => Some(Self::BossForestGuardian),
            22 => Some(Self::AncientDragon),  // New boss
            25 => Some(Self::BossIceDragon),
            27 => Some(Self::Colossus),       // New boss
            28 => Some(Self::ArchDemon),      // New boss
            30 => Some(Self::BossDemonKing),
            _ => None,
        }
    }

    fn faction(&self) -> Option<Faction> {
        match self {
            // Orc Horde
            Self::Orc | Self::Goblin | Self::Hobgoblin | Self::BossOrcWarlord | Self::BossGoblinKing
            | Self::GoblinChampion | Self::OrcBerserker | Self::Kobold => Some(Faction::OrcHorde),
            // Undead Legion
            Self::Skeleton | Self::Zombie | Self::Ghost | Self::Wraith | Self::Vampire | Self::Mummy
            | Self::Ghoul | Self::Banshee | Self::DeathKnight | Self::BoneGolem | Self::Lich
            | Self::BossVampireLord | Self::VampireElite | Self::MummyLord | Self::IceWraith
            | Self::CinderWraith => Some(Faction::UndeadLegion),
            // Demon Cult
            Self::Demon | Self::DemonLord | Self::Succubus | Self::Balrog | Self::PitFiend
            | Self::ShadowDemon | Self::AbyssalHorror | Self::DoomGuard | Self::BossDemonKing
            | Self::InfernalImp | Self::InfernalLord | Self::ArchDemon => Some(Faction::DemonCult),
            // Dragon Flight
            Self::FireDrake | Self::BossIceDragon | Self::AncientWyrm | Self::AncientDragon => Some(Faction::DragonFlight),
            // Nature/Druids (corrupted)
            Self::TreeEnt | Self::ForestTroll | Self::Druid | Self::VenomousVine | Self::ForestSpirit
            | Self::BossForestGuardian => Some(Faction::Druids),
            _ => None,
        }
    }
}

// ============================================================================
// ENEMY STRUCT
// ============================================================================

#[derive(Clone, Serialize, Deserialize)]
struct Enemy {
    x: usize,
    y: usize,
    kind: EnemyKind,
    hp: i32,
    #[allow(dead_code)]
    max_hp: i32,
    attack: i32,
    defense: i32,
    xp_value: u32,
    status_effects: HashMap<StatusEffect, u32>,
    last_seen_player: Option<(usize, usize)>,
}

impl Enemy {
    fn new(x: usize, y: usize, kind: EnemyKind, level: u32) -> Self {
        let (base_hp, base_atk, base_def, base_xp) = kind.base_stats();
        let scale = 1.0 + (level as f32 * 0.1);
        let hp = (base_hp as f32 * scale) as i32;
        Self {
            x,
            y,
            kind,
            hp,
            max_hp: hp,
            attack: (base_atk as f32 * scale) as i32,
            defense: (base_def as f32 * scale) as i32,
            xp_value: (base_xp as f32 * scale) as u32,
            status_effects: HashMap::new(),
            last_seen_player: None,
        }
    }

    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    fn take_damage(&mut self, amount: i32) -> i32 {
        let actual = (amount - self.defense).max(1);
        self.hp -= actual;
        actual
    }

    fn add_status(&mut self, effect: StatusEffect, duration: u32) {
        self.status_effects.insert(effect, duration);
    }

    fn has_status(&self, effect: StatusEffect) -> bool {
        self.status_effects.contains_key(&effect)
    }

    fn tick_status_effects(&mut self) -> Vec<(StatusEffect, i32)> {
        let mut damage_events = Vec::new();
        let mut to_remove = Vec::new();

        for (effect, duration) in self.status_effects.iter_mut() {
            match effect {
                StatusEffect::Poison => damage_events.push((*effect, 3)),
                StatusEffect::Burn => damage_events.push((*effect, 5)),
                StatusEffect::Bleed => damage_events.push((*effect, 2)),
                _ => {}
            }
            *duration = duration.saturating_sub(1);
            if *duration == 0 {
                to_remove.push(*effect);
            }
        }

        for effect in to_remove {
            self.status_effects.remove(&effect);
        }

        damage_events
    }
}

// ============================================================================
// SKILL SYSTEM
// ============================================================================

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
enum Skill {
    // Base Class Skills - Warrior
    Berserk, Cleave, ShieldBash, Whirlwind,
    // Base Class Skills - Mage
    Fireball, IceSpear, Lightning, Teleport,
    // Base Class Skills - Rogue
    Backstab, ShadowStep, PoisonBlade, Vanish,
    // Base Class Skills - Paladin
    HolyLight, DivineShield, Smite, Consecrate,
    // Base Class Skills - Ranger
    MultiShot, PoisonArrow, TrapSet, EagleEye,
    // Base Class Skills - Necromancer
    RaiseDead, LifeDrain, Curse, DarkPact,

    // Subclass Skills - Warrior
    Rage, Reckless, BloodFrenzy,           // Berserker
    ShieldWall, Fortify, Rally,             // Knight
    DualStrike, Riposte, FlurryOfBlows,     // Gladiator
    BattleCry, Devastate, Conqueror,        // Warlord
    HolySmite, DivineAura, Redemption,      // Paladin subclass
    DarkSlash, SoulReap, Corruption,        // DarkKnight
    Execute, GloryStrike, Unstoppable,      // Champion

    // Subclass Skills - Mage
    FireBlast, FrostNova, ChainLightning,   // Elementalist
    RaiseSkeleton, DeathCoil, BoneArmor,    // Necromancer subclass
    Empower, MagicShield, Haste,            // Enchanter
    Meteor, ElementalMastery, ArcaneNova,   // Archmage
    ArmyOfDead, DeathGrip, Lichform,        // LichLord
    TimeWarp, Omniscience, Transcendence,   // Sage

    // Subclass Skills - Rogue
    DeadlyStrike, Ambush, PoisonDagger,     // Assassin
    ShadowMeld, ShadowStrike,               // Shadow (Vanish is base)
    ThrowKnife, SmokeScreen, TrapMaster,    // Trickster
    ShadowDance, DeathMark, Eviscerate,     // ShadowBlade
    PhantomStrike, Assassination, VoidStep, // Nightstalker
    Pickpocket, GrandHeist, LuckOfThief,    // MasterThief

    // Subclass Skills - Cleric/Paladin
    Heal, Blessing, Purify,                 // Priest
    HolyStrike, Judgment, Exorcism,         // Inquisitor
    PalmStrike, InnerPeace,                 // Monk (FlurryOfBlows shared)
    Miracle, DivineIntervention, Resurrection, // HighPriest
    CrusaderStrike, HolyWrath, Zealot,      // Templar
    QuiveringPalm, Enlightenment, PerfectForm, // Grandmaster

    // Subclass Skills - Ranger
    CallPet, BeastBond, PackTactics,        // Beastmaster
    AimedShot, PiercingArrow,               // Archer (MultiShot shared)
    Entangle, NatureFury, Rejuvenate,       // Druid
    AlphaRoar, BeastMaster, Stampede,       // Alpha
    Headshot, RapidFire, KillShot,          // Marksman
    NatureWrath, TreeForm, ForceOfNature,   // Archdruid

    // Subclass Skills - Warlock
    SummonImp, DemonBolt, Hellfire,         // Demonologist
    BloodBolt, LifeTap, BloodShield,        // BloodMage
    Hex, DoomCurse, Weakness,               // Hexer
    SummonDemon, InfernalPact, DemonicForm, // DemonLord
    Exsanguinate, CrimsonPact, BloodNova,   // BloodLord
    Doom, CurseOfAgony, Pandemonium,        // CurseWeaver
}

impl Skill {
    fn name(&self) -> &'static str {
        match self {
            Self::Berserk => "Berserk",
            Self::Cleave => "Cleave",
            Self::ShieldBash => "Shield Bash",
            Self::Whirlwind => "Whirlwind",
            Self::Fireball => "Fireball",
            Self::IceSpear => "Ice Spear",
            Self::Lightning => "Lightning",
            Self::Teleport => "Teleport",
            Self::Backstab => "Backstab",
            Self::ShadowStep => "Shadow Step",
            Self::PoisonBlade => "Poison Blade",
            Self::Vanish => "Vanish",
            Self::HolyLight => "Holy Light",
            Self::DivineShield => "Divine Shield",
            Self::Smite => "Smite",
            Self::Consecrate => "Consecrate",
            Self::MultiShot => "Multi-Shot",
            Self::PoisonArrow => "Poison Arrow",
            Self::TrapSet => "Set Trap",
            Self::EagleEye => "Eagle Eye",
            Self::RaiseDead => "Raise Dead",
            Self::LifeDrain => "Life Drain",
            Self::Curse => "Curse",
            Self::DarkPact => "Dark Pact",
            // Subclass and advanced skills
            _ => "Unknown Skill",
        }
    }

    fn mana_cost(&self) -> i32 {
        match self {
            Self::Berserk | Self::Cleave | Self::ShieldBash => 10,
            Self::Whirlwind => 25,
            Self::Fireball | Self::IceSpear | Self::Lightning => 20,
            Self::Teleport => 30,
            Self::Backstab | Self::ShadowStep | Self::PoisonBlade => 15,
            Self::Vanish => 25,
            Self::HolyLight | Self::DivineShield | Self::Smite => 20,
            Self::Consecrate => 35,
            Self::MultiShot | Self::PoisonArrow | Self::TrapSet => 15,
            Self::EagleEye => 10,
            Self::RaiseDead => 40,
            Self::LifeDrain | Self::Curse => 20,
            Self::DarkPact => 50,
            // Advanced skills default cost
            _ => 30,
        }
    }

    fn for_class(class: CharacterClass) -> Vec<Self> {
        match class {
            CharacterClass::Warrior => vec![Self::Berserk, Self::Cleave, Self::ShieldBash, Self::Whirlwind],
            CharacterClass::Mage => vec![Self::Fireball, Self::IceSpear, Self::Lightning, Self::Teleport],
            CharacterClass::Rogue => vec![Self::Backstab, Self::ShadowStep, Self::PoisonBlade, Self::Vanish],
            CharacterClass::Paladin => vec![Self::HolyLight, Self::DivineShield, Self::Smite, Self::Consecrate],
            CharacterClass::Ranger => vec![Self::MultiShot, Self::PoisonArrow, Self::TrapSet, Self::EagleEye],
            CharacterClass::Necromancer => vec![Self::RaiseDead, Self::LifeDrain, Self::Curse, Self::DarkPact],
        }
    }
}

// ============================================================================
// COMPREHENSIVE SPELL SYSTEM
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum MagicSchool {
    Fire, Ice, Lightning, Earth, Wind, Water, Light, Dark,
    Arcane, Nature, Blood, Necromancy, Time, Space, Enchantment,
}

impl MagicSchool {
    fn name(&self) -> &'static str {
        match self {
            Self::Fire => "Fire", Self::Ice => "Ice", Self::Lightning => "Lightning",
            Self::Earth => "Earth", Self::Wind => "Wind", Self::Water => "Water",
            Self::Light => "Light", Self::Dark => "Dark", Self::Arcane => "Arcane",
            Self::Nature => "Nature", Self::Blood => "Blood", Self::Necromancy => "Necromancy",
            Self::Time => "Time", Self::Space => "Space", Self::Enchantment => "Enchantment",
        }
    }
    fn opposing(&self) -> Self {
        match self {
            Self::Fire => Self::Ice, Self::Ice => Self::Fire,
            Self::Lightning => Self::Earth, Self::Earth => Self::Lightning,
            Self::Wind => Self::Earth, Self::Water => Self::Fire,
            Self::Light => Self::Dark, Self::Dark => Self::Light,
            Self::Arcane => Self::Nature, Self::Nature => Self::Arcane,
            Self::Blood => Self::Light, Self::Necromancy => Self::Light,
            Self::Time => Self::Space, Self::Space => Self::Time,
            Self::Enchantment => Self::Arcane,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum ManaType { Raw, Fire, Ice, Lightning, Earth, Wind, Water, Holy, Shadow, Arcane, Natural, Blood, Necrotic, Temporal, Spatial, Corrupted, Prismatic }

impl ManaType {
    fn from_school(school: MagicSchool) -> Self {
        match school {
            MagicSchool::Fire => Self::Fire, MagicSchool::Ice => Self::Ice,
            MagicSchool::Lightning => Self::Lightning, MagicSchool::Earth => Self::Earth,
            MagicSchool::Wind => Self::Wind, MagicSchool::Water => Self::Water,
            MagicSchool::Light => Self::Holy, MagicSchool::Dark => Self::Shadow,
            MagicSchool::Arcane => Self::Arcane, MagicSchool::Nature => Self::Natural,
            MagicSchool::Blood => Self::Blood, MagicSchool::Necromancy => Self::Necrotic,
            MagicSchool::Time => Self::Temporal, MagicSchool::Space => Self::Spatial,
            MagicSchool::Enchantment => Self::Arcane,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
enum SpellMastery { Novice, Apprentice, Journeyman, Expert, Master, Grandmaster }

impl SpellMastery {
    fn from_xp(xp: u32) -> Self {
        match xp { 0..=99 => Self::Novice, 100..=499 => Self::Apprentice, 500..=1499 => Self::Journeyman, 1500..=3999 => Self::Expert, 4000..=9999 => Self::Master, _ => Self::Grandmaster }
    }
    fn damage_mult(&self) -> f32 { match self { Self::Novice => 1.0, Self::Apprentice => 1.15, Self::Journeyman => 1.35, Self::Expert => 1.6, Self::Master => 2.0, Self::Grandmaster => 2.5 } }
    fn mana_reduction(&self) -> f32 { match self { Self::Novice => 1.0, Self::Apprentice => 0.95, Self::Journeyman => 0.85, Self::Expert => 0.75, Self::Master => 0.6, Self::Grandmaster => 0.5 } }
    fn max_tier(&self) -> u8 { match self { Self::Novice => 2, Self::Apprentice => 4, Self::Journeyman => 6, Self::Expert => 8, Self::Master => 9, Self::Grandmaster => 10 } }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum SpellEffect {
    Damage(i32), Heal(i32), DamageOverTime { dpt: i32, dur: u32 }, HealOverTime { hpt: i32, dur: u32 },
    AoeDamage { radius: u32, dmg: i32 }, AoeHeal { radius: u32, heal: i32 },
    Buff { stat: String, amount: i32, dur: u32 }, Debuff { stat: String, amount: i32, dur: u32 },
    Stun(u32), Root(u32), Silence(u32), Slow { pct: u32, dur: u32 }, Fear(u32), Charm(u32), Blind(u32), Confuse(u32),
    Shield { amount: i32, dur: u32 }, MagicShield { amount: i32, dur: u32 }, ReflectShield { pct: u32, dur: u32 },
    Summon { creature: String, count: u32, dur: u32 }, SummonElemental { school: MagicSchool, power: i32, dur: u32 },
    Teleport(u32), Blink(u32), Pull(u32), Push(u32), Swap(u32),
    LifeSteal(u32), ManaDrain(i32), ManaRestore(i32), Cleanse(u32), Dispel(u32),
    Execute { threshold: u32 }, ChainHit { targets: u32, falloff: u32 }, Resurrect { hp_pct: u32 },
    Transform { form: String, dur: u32 }, Clone { count: u32, dur: u32 }, TimeStop(u32), Rewind(u32),
    Invisibility(u32), Flying(u32), Haste { actions: u32, dur: u32 }, Invulnerable(u32),
    MarkTarget(u32), Curse { amp: u32, dur: u32 }, Weaken { pct: u32, dur: u32 },
    Absorb { school: MagicSchool, dur: u32 }, Convert { from: ManaType, to: ManaType, ratio: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Spell {
    id: u32, name: String, school: MagicSchool, tier: u8, mana_cost: i32, mana_type: ManaType,
    cooldown: u32, damage: i32, effects: Vec<SpellEffect>, range: u32, area: u32, cast_time: u32, description: String,
}

impl Spell {
    fn new(id: u32, name: &str, school: MagicSchool, tier: u8, mana: i32, cd: u32, dmg: i32, range: u32, area: u32, cast: u32, desc: &str) -> Self {
        Self { id, name: name.to_string(), school, tier, mana_cost: mana, mana_type: ManaType::from_school(school), cooldown: cd, damage: dmg, effects: Vec::new(), range, area, cast_time: cast, description: desc.to_string() }
    }
    fn with_effect(mut self, e: SpellEffect) -> Self { self.effects.push(e); self }
    fn with_effects(mut self, es: Vec<SpellEffect>) -> Self { self.effects.extend(es); self }
    fn effective_mana(&self, mastery: SpellMastery) -> i32 { (self.mana_cost as f32 * mastery.mana_reduction()) as i32 }
    fn effective_damage(&self, mastery: SpellMastery) -> i32 { (self.damage as f32 * mastery.damage_mult()) as i32 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct SpellCombinationKey(MagicSchool, MagicSchool);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SpellCombination { schools: (MagicSchool, MagicSchool), result_spell_id: u32, name: String, mana_mult: f32, damage_mult: f32 }

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LearnedSpell { spell_id: u32, xp: u32, times_cast: u32, cooldown_remaining: u32 }

impl LearnedSpell {
    fn mastery(&self) -> SpellMastery { SpellMastery::from_xp(self.xp) }
    fn gain_xp(&mut self, amount: u32) { self.xp = self.xp.saturating_add(amount); self.times_cast += 1; }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SchoolMastery { school: MagicSchool, total_xp: u32, spells_learned: u32 }

impl SchoolMastery {
    fn new(school: MagicSchool) -> Self { Self { school, total_xp: 0, spells_learned: 0 } }
    fn mastery(&self) -> SpellMastery { SpellMastery::from_xp(self.total_xp) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ManaPool { raw: i32, max_raw: i32, elemental: HashMap<ManaType, i32>, max_elemental: HashMap<ManaType, i32>, corrupted: i32, max_corrupted: i32, regen_rate: i32 }

impl ManaPool {
    fn new(max_raw: i32) -> Self { Self { raw: max_raw, max_raw, elemental: HashMap::new(), max_elemental: HashMap::new(), corrupted: 0, max_corrupted: 50, regen_rate: 1 } }
    fn add_elemental(&mut self, t: ManaType, max: i32) { self.max_elemental.insert(t, max); self.elemental.insert(t, max); }
    fn spend(&mut self, t: ManaType, amount: i32) -> bool {
        if t == ManaType::Raw { if self.raw >= amount { self.raw -= amount; true } else { false } }
        else if t == ManaType::Corrupted { if self.corrupted >= amount { self.corrupted -= amount; true } else { false } }
        else { if let Some(v) = self.elemental.get_mut(&t) { if *v >= amount { *v -= amount; true } else { false } } else { if self.raw >= amount { self.raw -= amount; true } else { false } } }
    }
    fn regen(&mut self) { self.raw = (self.raw + self.regen_rate).min(self.max_raw); for (t, v) in self.elemental.iter_mut() { if let Some(max) = self.max_elemental.get(t) { *v = (*v + 1).min(*max); } } }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SpellBook { learned_spells: HashMap<u32, LearnedSpell>, school_mastery: HashMap<MagicSchool, SchoolMastery>, favorite_spells: Vec<u32>, max_favorites: usize, discovered_combinations: Vec<SpellCombinationKey> }

impl SpellBook {
    fn new() -> Self {
        let mut sm = HashMap::new();
        for s in [MagicSchool::Fire, MagicSchool::Ice, MagicSchool::Lightning, MagicSchool::Earth, MagicSchool::Wind, MagicSchool::Water, MagicSchool::Light, MagicSchool::Dark, MagicSchool::Arcane, MagicSchool::Nature, MagicSchool::Blood, MagicSchool::Necromancy, MagicSchool::Time, MagicSchool::Space, MagicSchool::Enchantment] { sm.insert(s, SchoolMastery::new(s)); }
        Self { learned_spells: HashMap::new(), school_mastery: sm, favorite_spells: Vec::new(), max_favorites: 10, discovered_combinations: Vec::new() }
    }
    fn learn(&mut self, spell: &Spell) -> bool {
        if self.learned_spells.contains_key(&spell.id) { return false; }
        self.learned_spells.insert(spell.id, LearnedSpell { spell_id: spell.id, xp: 0, times_cast: 0, cooldown_remaining: 0 });
        if let Some(m) = self.school_mastery.get_mut(&spell.school) { m.spells_learned += 1; }
        true
    }
    fn can_cast(&self, spell: &Spell, mana: &ManaPool) -> bool {
        if let Some(ls) = self.learned_spells.get(&spell.id) {
            if ls.cooldown_remaining > 0 { return false; }
            let cost = spell.effective_mana(ls.mastery());
            if spell.mana_type == ManaType::Raw { mana.raw >= cost } else { mana.elemental.get(&spell.mana_type).map_or(mana.raw >= cost, |v| *v >= cost) }
        } else { false }
    }
    fn cast(&mut self, spell: &Spell, mana: &mut ManaPool) -> Option<(i32, Vec<SpellEffect>)> {
        if !self.can_cast(spell, mana) { return None; }
        let ls = self.learned_spells.get_mut(&spell.id)?;
        let mastery = ls.mastery();
        let cost = spell.effective_mana(mastery);
        mana.spend(spell.mana_type, cost);
        ls.cooldown_remaining = spell.cooldown;
        ls.gain_xp(spell.tier as u32 * 10);
        if let Some(m) = self.school_mastery.get_mut(&spell.school) { m.total_xp += spell.tier as u32 * 5; }
        Some((spell.effective_damage(mastery), spell.effects.clone()))
    }
    fn tick_cooldowns(&mut self) { for ls in self.learned_spells.values_mut() { if ls.cooldown_remaining > 0 { ls.cooldown_remaining -= 1; } } }
}

// SPELL COMPENDIUM - 150+ SPELLS

fn create_fire_spells() -> Vec<Spell> { vec![
    Spell::new(1, "Ember", MagicSchool::Fire, 1, 5, 0, 8, 5, 0, 0, "A small flame projectile").with_effect(SpellEffect::Damage(8)),
    Spell::new(2, "Flame Bolt", MagicSchool::Fire, 2, 12, 1, 18, 6, 0, 0, "Concentrated bolt of fire").with_effect(SpellEffect::Damage(18)),
    Spell::new(3, "Burning Hands", MagicSchool::Fire, 2, 15, 2, 15, 2, 2, 0, "Cone of flames").with_effect(SpellEffect::AoeDamage { radius: 2, dmg: 15 }),
    Spell::new(4, "Fireball", MagicSchool::Fire, 3, 25, 3, 35, 8, 3, 1, "Explosive fireball").with_effects(vec![SpellEffect::AoeDamage { radius: 3, dmg: 35 }, SpellEffect::DamageOverTime { dpt: 5, dur: 3 }]),
    Spell::new(5, "Scorch", MagicSchool::Fire, 3, 20, 2, 25, 5, 0, 0, "Intense heat ignites target").with_effects(vec![SpellEffect::Damage(25), SpellEffect::DamageOverTime { dpt: 8, dur: 4 }]),
    Spell::new(6, "Fire Wall", MagicSchool::Fire, 4, 35, 5, 20, 6, 4, 1, "Wall of flames").with_effects(vec![SpellEffect::AoeDamage { radius: 4, dmg: 20 }, SpellEffect::DamageOverTime { dpt: 10, dur: 5 }]),
    Spell::new(7, "Immolate", MagicSchool::Fire, 5, 45, 4, 50, 4, 0, 1, "Engulfs target in flames").with_effects(vec![SpellEffect::Damage(50), SpellEffect::DamageOverTime { dpt: 15, dur: 5 }]),
    Spell::new(8, "Meteor Strike", MagicSchool::Fire, 7, 80, 8, 120, 10, 5, 3, "Meteor from the sky").with_effects(vec![SpellEffect::AoeDamage { radius: 5, dmg: 120 }, SpellEffect::Stun(2)]),
    Spell::new(9, "Inferno", MagicSchool::Fire, 8, 100, 10, 150, 8, 6, 2, "Massive firestorm").with_effects(vec![SpellEffect::AoeDamage { radius: 6, dmg: 150 }, SpellEffect::DamageOverTime { dpt: 25, dur: 5 }]),
    Spell::new(10, "Phoenix Flame", MagicSchool::Fire, 10, 150, 15, 200, 12, 8, 3, "Legendary fire that resurrects").with_effects(vec![SpellEffect::AoeDamage { radius: 8, dmg: 200 }, SpellEffect::Resurrect { hp_pct: 50 }]),
]}

fn create_ice_spells() -> Vec<Spell> { vec![
    Spell::new(11, "Frost Touch", MagicSchool::Ice, 1, 5, 0, 6, 1, 0, 0, "Chilling touch").with_effects(vec![SpellEffect::Damage(6), SpellEffect::Slow { pct: 20, dur: 2 }]),
    Spell::new(12, "Ice Shard", MagicSchool::Ice, 2, 10, 1, 15, 6, 0, 0, "Sharp ice projectile").with_effect(SpellEffect::Damage(15)),
    Spell::new(13, "Frost Nova", MagicSchool::Ice, 3, 25, 3, 25, 0, 4, 1, "Frost explosion around caster").with_effects(vec![SpellEffect::AoeDamage { radius: 4, dmg: 25 }, SpellEffect::Slow { pct: 50, dur: 3 }]),
    Spell::new(14, "Ice Spear", MagicSchool::Ice, 3, 22, 2, 30, 8, 0, 0, "Piercing ice lance").with_effects(vec![SpellEffect::Damage(30), SpellEffect::ChainHit { targets: 2, falloff: 30 }]),
    Spell::new(15, "Frozen Armor", MagicSchool::Ice, 4, 30, 6, 0, 0, 0, 1, "Protective ice shell").with_effects(vec![SpellEffect::Shield { amount: 50, dur: 10 }, SpellEffect::Buff { stat: "defense".into(), amount: 15, dur: 10 }]),
    Spell::new(16, "Blizzard", MagicSchool::Ice, 5, 50, 6, 40, 10, 6, 2, "Raging snowstorm").with_effects(vec![SpellEffect::AoeDamage { radius: 6, dmg: 40 }, SpellEffect::Slow { pct: 60, dur: 5 }, SpellEffect::DamageOverTime { dpt: 10, dur: 5 }]),
    Spell::new(17, "Flash Freeze", MagicSchool::Ice, 6, 60, 5, 60, 6, 0, 0, "Instantly freeze target").with_effects(vec![SpellEffect::Damage(60), SpellEffect::Stun(3)]),
    Spell::new(18, "Glacial Spike", MagicSchool::Ice, 7, 75, 6, 100, 8, 2, 1, "Massive ice spike").with_effects(vec![SpellEffect::Damage(100), SpellEffect::AoeDamage { radius: 2, dmg: 50 }]),
    Spell::new(19, "Ice Age", MagicSchool::Ice, 9, 120, 12, 180, 15, 10, 4, "Freezes entire battlefield").with_effects(vec![SpellEffect::AoeDamage { radius: 10, dmg: 180 }, SpellEffect::Stun(4), SpellEffect::Slow { pct: 80, dur: 8 }]),
    Spell::new(20, "Absolute Zero", MagicSchool::Ice, 10, 160, 15, 250, 6, 4, 3, "Ultimate cold stops all").with_effects(vec![SpellEffect::Damage(250), SpellEffect::TimeStop(2), SpellEffect::Execute { threshold: 20 }]),
]}

fn create_lightning_spells() -> Vec<Spell> { vec![
    Spell::new(21, "Spark", MagicSchool::Lightning, 1, 4, 0, 7, 5, 0, 0, "Small discharge").with_effect(SpellEffect::Damage(7)),
    Spell::new(22, "Shock", MagicSchool::Lightning, 2, 10, 1, 16, 6, 0, 0, "Stunning shock").with_effects(vec![SpellEffect::Damage(16), SpellEffect::Stun(1)]),
    Spell::new(23, "Lightning Bolt", MagicSchool::Lightning, 3, 22, 2, 35, 10, 0, 0, "Classic lightning strike").with_effect(SpellEffect::Damage(35)),
    Spell::new(24, "Chain Lightning", MagicSchool::Lightning, 4, 35, 3, 40, 8, 0, 1, "Bounces between enemies").with_effects(vec![SpellEffect::Damage(40), SpellEffect::ChainHit { targets: 5, falloff: 15 }]),
    Spell::new(25, "Thunderclap", MagicSchool::Lightning, 4, 38, 4, 35, 0, 5, 1, "Explosive thunder").with_effects(vec![SpellEffect::AoeDamage { radius: 5, dmg: 35 }, SpellEffect::Stun(2)]),
    Spell::new(26, "Static Field", MagicSchool::Lightning, 5, 45, 5, 30, 8, 4, 1, "Electrified zone").with_effects(vec![SpellEffect::AoeDamage { radius: 4, dmg: 30 }, SpellEffect::DamageOverTime { dpt: 12, dur: 4 }]),
    Spell::new(27, "Ball Lightning", MagicSchool::Lightning, 6, 55, 5, 70, 6, 3, 1, "Floating electric orb").with_effects(vec![SpellEffect::Damage(70), SpellEffect::AoeDamage { radius: 3, dmg: 40 }]),
    Spell::new(28, "Thunderstorm", MagicSchool::Lightning, 7, 80, 8, 90, 12, 8, 3, "Multiple lightning strikes").with_effects(vec![SpellEffect::AoeDamage { radius: 8, dmg: 90 }, SpellEffect::ChainHit { targets: 8, falloff: 10 }]),
    Spell::new(29, "Ride the Lightning", MagicSchool::Lightning, 8, 90, 6, 110, 15, 2, 0, "Become lightning").with_effects(vec![SpellEffect::Teleport(15), SpellEffect::AoeDamage { radius: 2, dmg: 110 }]),
    Spell::new(30, "Mjolnir's Wrath", MagicSchool::Lightning, 10, 170, 15, 280, 20, 6, 3, "Divine lightning").with_effects(vec![SpellEffect::Damage(280), SpellEffect::AoeDamage { radius: 6, dmg: 150 }, SpellEffect::Stun(5)]),
]}

fn create_earth_spells() -> Vec<Spell> { vec![
    Spell::new(31, "Stone Throw", MagicSchool::Earth, 1, 5, 0, 9, 5, 0, 0, "Hurl a rock").with_effect(SpellEffect::Damage(9)),
    Spell::new(32, "Tremor", MagicSchool::Earth, 2, 12, 2, 14, 6, 3, 1, "Minor earthquake").with_effects(vec![SpellEffect::AoeDamage { radius: 3, dmg: 14 }, SpellEffect::Slow { pct: 30, dur: 2 }]),
    Spell::new(33, "Stone Skin", MagicSchool::Earth, 2, 15, 5, 0, 0, 0, 1, "Harden skin").with_effects(vec![SpellEffect::Buff { stat: "defense".into(), amount: 20, dur: 8 }, SpellEffect::Shield { amount: 30, dur: 8 }]),
    Spell::new(34, "Rock Spike", MagicSchool::Earth, 3, 22, 2, 32, 6, 0, 1, "Sharp stone eruption").with_effects(vec![SpellEffect::Damage(32), SpellEffect::Root(2)]),
    Spell::new(35, "Earth Wall", MagicSchool::Earth, 4, 30, 6, 0, 6, 3, 1, "Protective barrier").with_effect(SpellEffect::Shield { amount: 80, dur: 6 }),
    Spell::new(36, "Quicksand", MagicSchool::Earth, 4, 35, 4, 25, 5, 2, 1, "Trapping sands").with_effects(vec![SpellEffect::AoeDamage { radius: 2, dmg: 25 }, SpellEffect::Root(4), SpellEffect::Slow { pct: 70, dur: 4 }]),
    Spell::new(37, "Boulder Crush", MagicSchool::Earth, 5, 45, 5, 65, 7, 2, 2, "Massive boulder").with_effects(vec![SpellEffect::Damage(65), SpellEffect::AoeDamage { radius: 2, dmg: 40 }, SpellEffect::Stun(2)]),
    Spell::new(38, "Earthquake", MagicSchool::Earth, 7, 75, 8, 85, 0, 10, 3, "Massive tremor").with_effects(vec![SpellEffect::AoeDamage { radius: 10, dmg: 85 }, SpellEffect::Stun(3), SpellEffect::Root(3)]),
    Spell::new(39, "Meteor Swarm", MagicSchool::Earth, 9, 130, 12, 160, 15, 8, 4, "Rain of meteors").with_effects(vec![SpellEffect::AoeDamage { radius: 8, dmg: 160 }, SpellEffect::DamageOverTime { dpt: 20, dur: 4 }]),
    Spell::new(40, "World Breaker", MagicSchool::Earth, 10, 180, 18, 300, 20, 12, 5, "Cataclysmic event").with_effects(vec![SpellEffect::AoeDamage { radius: 12, dmg: 300 }, SpellEffect::Stun(5), SpellEffect::Execute { threshold: 25 }]),
]}

fn create_wind_spells() -> Vec<Spell> { vec![
    Spell::new(41, "Gust", MagicSchool::Wind, 1, 4, 0, 6, 5, 0, 0, "Blast of air").with_effects(vec![SpellEffect::Damage(6), SpellEffect::Push(2)]),
    Spell::new(42, "Wind Slash", MagicSchool::Wind, 2, 10, 1, 18, 6, 0, 0, "Cutting air blade").with_effect(SpellEffect::Damage(18)),
    Spell::new(43, "Tailwind", MagicSchool::Wind, 2, 12, 4, 0, 0, 3, 0, "Speed boost").with_effects(vec![SpellEffect::Buff { stat: "speed".into(), amount: 30, dur: 6 }, SpellEffect::Haste { actions: 1, dur: 3 }]),
    Spell::new(44, "Cyclone", MagicSchool::Wind, 3, 25, 3, 28, 7, 4, 1, "Swirling vortex").with_effects(vec![SpellEffect::AoeDamage { radius: 4, dmg: 28 }, SpellEffect::Pull(3)]),
    Spell::new(45, "Air Shield", MagicSchool::Wind, 3, 20, 5, 0, 0, 0, 0, "Deflecting barrier").with_effects(vec![SpellEffect::ReflectShield { pct: 30, dur: 6 }, SpellEffect::Buff { stat: "evasion".into(), amount: 25, dur: 6 }]),
    Spell::new(46, "Whirlwind", MagicSchool::Wind, 4, 35, 4, 40, 0, 5, 1, "Spinning wind").with_effects(vec![SpellEffect::AoeDamage { radius: 5, dmg: 40 }, SpellEffect::Push(4)]),
    Spell::new(47, "Vacuum", MagicSchool::Wind, 5, 45, 5, 50, 6, 3, 1, "Remove air").with_effects(vec![SpellEffect::Damage(50), SpellEffect::Silence(4), SpellEffect::DamageOverTime { dpt: 15, dur: 3 }]),
    Spell::new(48, "Tornado", MagicSchool::Wind, 7, 80, 8, 100, 10, 6, 2, "Devastating twister").with_effects(vec![SpellEffect::AoeDamage { radius: 6, dmg: 100 }, SpellEffect::Pull(6), SpellEffect::Stun(3)]),
    Spell::new(49, "Flight", MagicSchool::Wind, 6, 50, 10, 0, 0, 0, 1, "Grants flight").with_effect(SpellEffect::Flying(20)),
    Spell::new(50, "Hurricane", MagicSchool::Wind, 10, 160, 15, 220, 20, 15, 4, "Ultimate storm").with_effects(vec![SpellEffect::AoeDamage { radius: 15, dmg: 220 }, SpellEffect::Push(8), SpellEffect::Stun(4), SpellEffect::DamageOverTime { dpt: 30, dur: 5 }]),
]}

fn create_water_spells() -> Vec<Spell> { vec![
    Spell::new(51, "Water Jet", MagicSchool::Water, 1, 5, 0, 7, 5, 0, 0, "Pressurized water").with_effect(SpellEffect::Damage(7)),
    Spell::new(52, "Healing Spring", MagicSchool::Water, 2, 15, 3, 0, 0, 2, 1, "Restorative waters").with_effects(vec![SpellEffect::AoeHeal { radius: 2, heal: 25 }, SpellEffect::HealOverTime { hpt: 5, dur: 4 }]),
    Spell::new(53, "Tidal Wave", MagicSchool::Water, 3, 25, 3, 30, 8, 4, 1, "Crashing wave").with_effects(vec![SpellEffect::AoeDamage { radius: 4, dmg: 30 }, SpellEffect::Push(4)]),
    Spell::new(54, "Purify", MagicSchool::Water, 3, 20, 4, 0, 4, 0, 0, "Cleanse ailments").with_effect(SpellEffect::Cleanse(3)),
    Spell::new(55, "Water Breathing", MagicSchool::Water, 2, 10, 8, 0, 0, 0, 1, "Breathe underwater").with_effect(SpellEffect::Buff { stat: "water_breathing".into(), amount: 1, dur: 100 }),
    Spell::new(56, "Hydro Pump", MagicSchool::Water, 5, 45, 4, 65, 8, 0, 1, "High pressure cannon").with_effects(vec![SpellEffect::Damage(65), SpellEffect::Push(5)]),
    Spell::new(57, "Healing Rain", MagicSchool::Water, 5, 50, 6, 0, 0, 6, 2, "Widespread healing").with_effects(vec![SpellEffect::AoeHeal { radius: 6, heal: 40 }, SpellEffect::HealOverTime { hpt: 10, dur: 5 }]),
    Spell::new(58, "Maelstrom", MagicSchool::Water, 7, 80, 8, 95, 10, 7, 3, "Churning whirlpool").with_effects(vec![SpellEffect::AoeDamage { radius: 7, dmg: 95 }, SpellEffect::Pull(5), SpellEffect::Slow { pct: 60, dur: 4 }]),
    Spell::new(59, "Tsunami", MagicSchool::Water, 9, 130, 12, 170, 15, 10, 4, "Massive tidal wave").with_effects(vec![SpellEffect::AoeDamage { radius: 10, dmg: 170 }, SpellEffect::Push(8), SpellEffect::Stun(3)]),
    Spell::new(60, "Leviathan's Embrace", MagicSchool::Water, 10, 170, 15, 240, 20, 12, 4, "Ocean's power").with_effects(vec![SpellEffect::AoeDamage { radius: 12, dmg: 240 }, SpellEffect::Heal(100), SpellEffect::Summon { creature: "Water Elemental".into(), count: 2, dur: 10 }]),
]}

fn create_light_spells() -> Vec<Spell> { vec![
    Spell::new(61, "Holy Light", MagicSchool::Light, 1, 8, 0, 0, 5, 0, 0, "Basic healing").with_effect(SpellEffect::Heal(20)),
    Spell::new(62, "Smite", MagicSchool::Light, 2, 12, 1, 20, 6, 0, 0, "Divine damage").with_effect(SpellEffect::Damage(20)),
    Spell::new(63, "Blessing", MagicSchool::Light, 2, 15, 5, 0, 4, 0, 1, "Divine protection").with_effects(vec![SpellEffect::Buff { stat: "all".into(), amount: 10, dur: 10 }, SpellEffect::Shield { amount: 25, dur: 10 }]),
    Spell::new(64, "Purifying Light", MagicSchool::Light, 3, 25, 3, 35, 6, 0, 1, "Damages evil heals allies").with_effects(vec![SpellEffect::Damage(35), SpellEffect::Heal(20)]),
    Spell::new(65, "Divine Shield", MagicSchool::Light, 4, 40, 8, 0, 0, 0, 0, "Invulnerability").with_effect(SpellEffect::Invulnerable(3)),
    Spell::new(66, "Consecration", MagicSchool::Light, 4, 35, 5, 25, 0, 5, 1, "Holy ground").with_effects(vec![SpellEffect::AoeDamage { radius: 5, dmg: 25 }, SpellEffect::AoeHeal { radius: 5, heal: 15 }, SpellEffect::DamageOverTime { dpt: 10, dur: 5 }]),
    Spell::new(67, "Resurrection", MagicSchool::Light, 7, 100, 20, 0, 3, 0, 3, "Bring back the dead").with_effect(SpellEffect::Resurrect { hp_pct: 75 }),
    Spell::new(68, "Solar Flare", MagicSchool::Light, 6, 60, 5, 80, 8, 4, 1, "Blinding light").with_effects(vec![SpellEffect::AoeDamage { radius: 4, dmg: 80 }, SpellEffect::Blind(4)]),
    Spell::new(69, "Judgment", MagicSchool::Light, 8, 100, 10, 150, 10, 0, 2, "Divine judgment").with_effects(vec![SpellEffect::Damage(150), SpellEffect::Execute { threshold: 30 }]),
    Spell::new(70, "Avatar of Light", MagicSchool::Light, 10, 180, 20, 200, 15, 10, 3, "Divine avatar").with_effects(vec![SpellEffect::Transform { form: "Avatar".into(), dur: 10 }, SpellEffect::AoeDamage { radius: 10, dmg: 200 }, SpellEffect::AoeHeal { radius: 10, heal: 100 }]),
]}

fn create_dark_spells() -> Vec<Spell> { vec![
    Spell::new(71, "Shadow Bolt", MagicSchool::Dark, 1, 6, 0, 10, 6, 0, 0, "Bolt of darkness").with_effect(SpellEffect::Damage(10)),
    Spell::new(72, "Life Drain", MagicSchool::Dark, 2, 15, 2, 18, 5, 0, 0, "Steal life force").with_effects(vec![SpellEffect::Damage(18), SpellEffect::LifeSteal(50)]),
    Spell::new(73, "Curse", MagicSchool::Dark, 2, 12, 4, 0, 6, 0, 0, "Weaken target").with_effect(SpellEffect::Curse { amp: 25, dur: 8 }),
    Spell::new(74, "Fear", MagicSchool::Dark, 3, 25, 4, 0, 6, 0, 1, "Terrify enemies").with_effect(SpellEffect::Fear(5)),
    Spell::new(75, "Shadow Step", MagicSchool::Dark, 3, 20, 2, 0, 8, 0, 0, "Teleport through shadows").with_effects(vec![SpellEffect::Teleport(8), SpellEffect::Invisibility(2)]),
    Spell::new(76, "Corruption", MagicSchool::Dark, 4, 35, 5, 40, 6, 0, 1, "Corrupt target").with_effects(vec![SpellEffect::Damage(40), SpellEffect::DamageOverTime { dpt: 15, dur: 6 }, SpellEffect::Weaken { pct: 20, dur: 6 }]),
    Spell::new(77, "Dark Pact", MagicSchool::Dark, 5, 0, 8, 80, 0, 0, 0, "Sacrifice HP for damage").with_effects(vec![SpellEffect::Damage(80), SpellEffect::DamageOverTime { dpt: -20, dur: 3 }]),
    Spell::new(78, "Soul Rend", MagicSchool::Dark, 6, 60, 5, 90, 7, 0, 1, "Tear at the soul").with_effects(vec![SpellEffect::Damage(90), SpellEffect::ManaDrain(30)]),
    Spell::new(79, "Void Zone", MagicSchool::Dark, 8, 100, 10, 120, 10, 6, 2, "Area of darkness").with_effects(vec![SpellEffect::AoeDamage { radius: 6, dmg: 120 }, SpellEffect::Blind(5), SpellEffect::Silence(4)]),
    Spell::new(80, "Apocalypse", MagicSchool::Dark, 10, 200, 20, 350, 15, 10, 4, "Ultimate destruction").with_effects(vec![SpellEffect::AoeDamage { radius: 10, dmg: 350 }, SpellEffect::Fear(6), SpellEffect::Curse { amp: 50, dur: 10 }]),
]}

fn create_arcane_spells() -> Vec<Spell> { vec![
    Spell::new(81, "Arcane Bolt", MagicSchool::Arcane, 1, 5, 0, 9, 6, 0, 0, "Pure magical energy").with_effect(SpellEffect::Damage(9)),
    Spell::new(82, "Mana Shield", MagicSchool::Arcane, 2, 20, 5, 0, 0, 0, 0, "Shield from mana").with_effect(SpellEffect::MagicShield { amount: 50, dur: 8 }),
    Spell::new(83, "Dispel Magic", MagicSchool::Arcane, 3, 25, 3, 0, 6, 0, 0, "Remove magical effects").with_effect(SpellEffect::Dispel(5)),
    Spell::new(84, "Arcane Missiles", MagicSchool::Arcane, 3, 28, 2, 45, 8, 0, 1, "Multiple missiles").with_effects(vec![SpellEffect::Damage(45), SpellEffect::ChainHit { targets: 3, falloff: 0 }]),
    Spell::new(85, "Counterspell", MagicSchool::Arcane, 4, 30, 4, 0, 8, 0, 0, "Interrupt and silence").with_effect(SpellEffect::Silence(5)),
    Spell::new(86, "Arcane Explosion", MagicSchool::Arcane, 4, 40, 4, 55, 0, 5, 1, "Arcane burst").with_effect(SpellEffect::AoeDamage { radius: 5, dmg: 55 }),
    Spell::new(87, "Spell Steal", MagicSchool::Arcane, 5, 45, 5, 0, 6, 0, 1, "Steal enemy buffs").with_effects(vec![SpellEffect::Dispel(3), SpellEffect::Buff { stat: "stolen".into(), amount: 20, dur: 10 }]),
    Spell::new(88, "Arcane Torrent", MagicSchool::Arcane, 6, 60, 6, 85, 10, 3, 2, "Stream of power").with_effects(vec![SpellEffect::Damage(85), SpellEffect::ManaDrain(20)]),
    Spell::new(89, "Prismatic Spray", MagicSchool::Arcane, 8, 100, 10, 140, 10, 6, 2, "Multi-element burst").with_effects(vec![SpellEffect::AoeDamage { radius: 6, dmg: 140 }, SpellEffect::Confuse(4), SpellEffect::ChainHit { targets: 6, falloff: 10 }]),
    Spell::new(90, "Arcane Singularity", MagicSchool::Arcane, 10, 180, 18, 280, 12, 8, 4, "Magical collapse").with_effects(vec![SpellEffect::AoeDamage { radius: 8, dmg: 280 }, SpellEffect::Pull(8), SpellEffect::ManaDrain(50)]),
]}

fn create_nature_spells() -> Vec<Spell> { vec![
    Spell::new(91, "Thorn", MagicSchool::Nature, 1, 4, 0, 8, 5, 0, 0, "Sharp thorn").with_effect(SpellEffect::Damage(8)),
    Spell::new(92, "Rejuvenation", MagicSchool::Nature, 2, 15, 3, 0, 4, 0, 0, "Healing over time").with_effect(SpellEffect::HealOverTime { hpt: 8, dur: 8 }),
    Spell::new(93, "Entangle", MagicSchool::Nature, 2, 12, 3, 10, 6, 3, 1, "Vines hold enemies").with_effects(vec![SpellEffect::AoeDamage { radius: 3, dmg: 10 }, SpellEffect::Root(4)]),
    Spell::new(94, "Poison Spores", MagicSchool::Nature, 3, 22, 4, 20, 6, 3, 1, "Toxic cloud").with_effects(vec![SpellEffect::AoeDamage { radius: 3, dmg: 20 }, SpellEffect::DamageOverTime { dpt: 10, dur: 5 }]),
    Spell::new(95, "Summon Beast", MagicSchool::Nature, 4, 40, 8, 0, 5, 0, 2, "Call wild beast").with_effect(SpellEffect::Summon { creature: "Wolf".into(), count: 2, dur: 15 }),
    Spell::new(96, "Barkskin", MagicSchool::Nature, 3, 25, 6, 0, 0, 0, 1, "Tough bark armor").with_effects(vec![SpellEffect::Buff { stat: "defense".into(), amount: 25, dur: 10 }, SpellEffect::Shield { amount: 40, dur: 10 }]),
    Spell::new(97, "Wild Growth", MagicSchool::Nature, 5, 50, 6, 0, 0, 5, 2, "Area heal").with_effects(vec![SpellEffect::AoeHeal { radius: 5, heal: 30 }, SpellEffect::HealOverTime { hpt: 12, dur: 6 }]),
    Spell::new(98, "Swarm", MagicSchool::Nature, 6, 55, 5, 70, 8, 4, 1, "Insect swarm").with_effects(vec![SpellEffect::AoeDamage { radius: 4, dmg: 70 }, SpellEffect::DamageOverTime { dpt: 15, dur: 5 }, SpellEffect::Confuse(3)]),
    Spell::new(99, "Wrath of Nature", MagicSchool::Nature, 8, 100, 10, 150, 12, 8, 3, "Nature's fury").with_effects(vec![SpellEffect::AoeDamage { radius: 8, dmg: 150 }, SpellEffect::Root(4), SpellEffect::Summon { creature: "Treant".into(), count: 1, dur: 10 }]),
    Spell::new(100, "Avatar of Nature", MagicSchool::Nature, 10, 180, 18, 200, 15, 10, 4, "Become one with nature").with_effects(vec![SpellEffect::Transform { form: "Treant Lord".into(), dur: 12 }, SpellEffect::AoeHeal { radius: 10, heal: 150 }, SpellEffect::Summon { creature: "Forest Spirit".into(), count: 3, dur: 12 }]),
]}

fn create_blood_spells() -> Vec<Spell> { vec![
    Spell::new(101, "Blood Bolt", MagicSchool::Blood, 1, 0, 0, 12, 5, 0, 0, "Costs HP not mana").with_effects(vec![SpellEffect::Damage(12), SpellEffect::DamageOverTime { dpt: -5, dur: 1 }]),
    Spell::new(102, "Blood Shield", MagicSchool::Blood, 2, 0, 4, 0, 0, 0, 0, "Shield from blood").with_effects(vec![SpellEffect::Shield { amount: 40, dur: 6 }, SpellEffect::DamageOverTime { dpt: -8, dur: 1 }]),
    Spell::new(103, "Sanguine Strike", MagicSchool::Blood, 3, 15, 2, 35, 3, 0, 0, "Attack that heals").with_effects(vec![SpellEffect::Damage(35), SpellEffect::LifeSteal(75)]),
    Spell::new(104, "Blood Boil", MagicSchool::Blood, 3, 20, 3, 40, 6, 0, 1, "Boil enemy blood").with_effects(vec![SpellEffect::Damage(40), SpellEffect::DamageOverTime { dpt: 12, dur: 4 }]),
    Spell::new(105, "Transfusion", MagicSchool::Blood, 4, 0, 5, 0, 5, 0, 1, "Transfer HP to ally").with_effects(vec![SpellEffect::Heal(50), SpellEffect::DamageOverTime { dpt: -25, dur: 1 }]),
    Spell::new(106, "Hemorrhage", MagicSchool::Blood, 5, 35, 4, 55, 6, 0, 1, "Severe bleeding").with_effects(vec![SpellEffect::Damage(55), SpellEffect::DamageOverTime { dpt: 20, dur: 6 }]),
    Spell::new(107, "Blood Frenzy", MagicSchool::Blood, 5, 0, 8, 0, 0, 0, 1, "Sacrifice HP for power").with_effects(vec![SpellEffect::Buff { stat: "attack".into(), amount: 50, dur: 8 }, SpellEffect::Buff { stat: "speed".into(), amount: 20, dur: 8 }, SpellEffect::DamageOverTime { dpt: -15, dur: 8 }]),
    Spell::new(108, "Exsanguinate", MagicSchool::Blood, 7, 60, 8, 120, 6, 0, 2, "Drain all blood").with_effects(vec![SpellEffect::Damage(120), SpellEffect::LifeSteal(100), SpellEffect::Execute { threshold: 25 }]),
    Spell::new(109, "Blood Nova", MagicSchool::Blood, 8, 0, 10, 150, 0, 6, 2, "Explosive blood").with_effects(vec![SpellEffect::AoeDamage { radius: 6, dmg: 150 }, SpellEffect::LifeSteal(50), SpellEffect::DamageOverTime { dpt: -50, dur: 1 }]),
    Spell::new(110, "Crimson Apocalypse", MagicSchool::Blood, 10, 100, 20, 300, 10, 8, 4, "Ultimate blood magic").with_effects(vec![SpellEffect::AoeDamage { radius: 8, dmg: 300 }, SpellEffect::LifeSteal(100), SpellEffect::Execute { threshold: 35 }]),
]}

fn create_necromancy_spells() -> Vec<Spell> { vec![
    Spell::new(111, "Touch of Death", MagicSchool::Necromancy, 1, 8, 0, 12, 1, 0, 0, "Deathly touch").with_effect(SpellEffect::Damage(12)),
    Spell::new(112, "Raise Skeleton", MagicSchool::Necromancy, 2, 25, 5, 0, 5, 0, 2, "Raise skeleton minion").with_effect(SpellEffect::Summon { creature: "Skeleton".into(), count: 1, dur: 20 }),
    Spell::new(113, "Bone Armor", MagicSchool::Necromancy, 2, 20, 6, 0, 0, 0, 1, "Shield of bones").with_effects(vec![SpellEffect::Shield { amount: 45, dur: 10 }, SpellEffect::ReflectShield { pct: 20, dur: 10 }]),
    Spell::new(114, "Death Coil", MagicSchool::Necromancy, 3, 25, 2, 35, 6, 0, 0, "Coil of death").with_effects(vec![SpellEffect::Damage(35), SpellEffect::Heal(20)]),
    Spell::new(115, "Corpse Explosion", MagicSchool::Necromancy, 4, 30, 4, 60, 8, 4, 1, "Detonate corpses").with_effect(SpellEffect::AoeDamage { radius: 4, dmg: 60 }),
    Spell::new(116, "Army of Dead", MagicSchool::Necromancy, 6, 80, 15, 0, 6, 0, 3, "Skeleton army").with_effect(SpellEffect::Summon { creature: "Skeleton Warrior".into(), count: 5, dur: 15 }),
    Spell::new(117, "Death Grip", MagicSchool::Necromancy, 4, 35, 4, 40, 8, 0, 0, "Pull and damage").with_effects(vec![SpellEffect::Damage(40), SpellEffect::Pull(8), SpellEffect::Root(2)]),
    Spell::new(118, "Plague", MagicSchool::Necromancy, 5, 50, 6, 30, 10, 5, 2, "Spreading disease").with_effects(vec![SpellEffect::AoeDamage { radius: 5, dmg: 30 }, SpellEffect::DamageOverTime { dpt: 15, dur: 8 }, SpellEffect::ChainHit { targets: 5, falloff: 0 }]),
    Spell::new(119, "Lichform", MagicSchool::Necromancy, 9, 150, 30, 0, 0, 0, 3, "Become a lich").with_effects(vec![SpellEffect::Transform { form: "Lich".into(), dur: 20 }, SpellEffect::Buff { stat: "all_magic".into(), amount: 50, dur: 20 }, SpellEffect::Invulnerable(2)]),
    Spell::new(120, "Apocalypse of Undeath", MagicSchool::Necromancy, 10, 200, 25, 250, 15, 10, 5, "Ultimate necromancy").with_effects(vec![SpellEffect::AoeDamage { radius: 10, dmg: 250 }, SpellEffect::Summon { creature: "Death Knight".into(), count: 3, dur: 20 }, SpellEffect::Fear(5)]),
]}

fn create_time_spells() -> Vec<Spell> { vec![
    Spell::new(121, "Slow", MagicSchool::Time, 1, 10, 2, 0, 6, 0, 0, "Slow target").with_effect(SpellEffect::Slow { pct: 50, dur: 5 }),
    Spell::new(122, "Haste", MagicSchool::Time, 2, 20, 5, 0, 4, 0, 1, "Speed up ally").with_effects(vec![SpellEffect::Haste { actions: 1, dur: 5 }, SpellEffect::Buff { stat: "speed".into(), amount: 30, dur: 5 }]),
    Spell::new(123, "Time Warp", MagicSchool::Time, 3, 30, 6, 0, 0, 4, 1, "Area haste").with_effects(vec![SpellEffect::Haste { actions: 1, dur: 4 }, SpellEffect::Buff { stat: "speed".into(), amount: 25, dur: 4 }]),
    Spell::new(124, "Temporal Shield", MagicSchool::Time, 4, 40, 8, 0, 0, 0, 1, "Rewinds damage").with_effects(vec![SpellEffect::Shield { amount: 60, dur: 6 }, SpellEffect::Rewind(1)]),
    Spell::new(125, "Age", MagicSchool::Time, 4, 35, 4, 50, 6, 0, 1, "Rapidly age target").with_effects(vec![SpellEffect::Damage(50), SpellEffect::Weaken { pct: 30, dur: 6 }]),
    Spell::new(126, "Rewind", MagicSchool::Time, 5, 60, 10, 0, 0, 0, 2, "Rewind damage").with_effects(vec![SpellEffect::Rewind(3), SpellEffect::Heal(50)]),
    Spell::new(127, "Temporal Stasis", MagicSchool::Time, 6, 70, 8, 0, 6, 0, 1, "Freeze in time").with_effects(vec![SpellEffect::Stun(5), SpellEffect::Invulnerable(5)]),
    Spell::new(128, "Glimpse of Eternity", MagicSchool::Time, 7, 90, 10, 100, 8, 0, 2, "Show their death").with_effects(vec![SpellEffect::Damage(100), SpellEffect::Fear(6), SpellEffect::Weaken { pct: 40, dur: 6 }]),
    Spell::new(129, "Time Stop", MagicSchool::Time, 9, 150, 20, 0, 0, 0, 3, "Stop time").with_effects(vec![SpellEffect::TimeStop(3), SpellEffect::Haste { actions: 3, dur: 3 }]),
    Spell::new(130, "Temporal Paradox", MagicSchool::Time, 10, 200, 25, 300, 10, 8, 4, "Create time paradox").with_effects(vec![SpellEffect::AoeDamage { radius: 8, dmg: 300 }, SpellEffect::TimeStop(2), SpellEffect::Clone { count: 2, dur: 10 }]),
]}

fn create_space_spells() -> Vec<Spell> { vec![
    Spell::new(131, "Blink", MagicSchool::Space, 1, 10, 1, 0, 5, 0, 0, "Short teleport").with_effect(SpellEffect::Blink(5)),
    Spell::new(132, "Spatial Rift", MagicSchool::Space, 2, 15, 2, 20, 6, 2, 0, "Tear in space").with_effects(vec![SpellEffect::Damage(20), SpellEffect::AoeDamage { radius: 2, dmg: 15 }]),
    Spell::new(133, "Teleport", MagicSchool::Space, 3, 30, 4, 0, 15, 0, 1, "Long range teleport").with_effect(SpellEffect::Teleport(15)),
    Spell::new(134, "Dimensional Anchor", MagicSchool::Space, 3, 25, 5, 0, 8, 0, 1, "Prevent teleportation").with_effects(vec![SpellEffect::Root(6), SpellEffect::Debuff { stat: "teleport".into(), amount: -100, dur: 6 }]),
    Spell::new(135, "Gravity Well", MagicSchool::Space, 4, 40, 5, 45, 8, 4, 1, "Crushing gravity").with_effects(vec![SpellEffect::AoeDamage { radius: 4, dmg: 45 }, SpellEffect::Pull(4), SpellEffect::Slow { pct: 50, dur: 4 }]),
    Spell::new(136, "Phase Shift", MagicSchool::Space, 4, 35, 6, 0, 0, 0, 0, "Become intangible").with_effects(vec![SpellEffect::Invisibility(4), SpellEffect::Invulnerable(2)]),
    Spell::new(137, "Wormhole", MagicSchool::Space, 6, 60, 8, 0, 20, 0, 2, "Create portal").with_effects(vec![SpellEffect::Teleport(20), SpellEffect::Swap(20)]),
    Spell::new(138, "Banish", MagicSchool::Space, 7, 80, 10, 100, 8, 0, 2, "Send to another dimension").with_effects(vec![SpellEffect::Damage(100), SpellEffect::Stun(8)]),
    Spell::new(139, "Dimensional Collapse", MagicSchool::Space, 9, 140, 15, 200, 10, 6, 3, "Collapse space").with_effects(vec![SpellEffect::AoeDamage { radius: 6, dmg: 200 }, SpellEffect::Pull(8), SpellEffect::Stun(4)]),
    Spell::new(140, "Reality Tear", MagicSchool::Space, 10, 200, 20, 350, 12, 10, 4, "Tear reality").with_effects(vec![SpellEffect::AoeDamage { radius: 10, dmg: 350 }, SpellEffect::Teleport(30), SpellEffect::Summon { creature: "Void Entity".into(), count: 2, dur: 15 }]),
]}

fn create_enchantment_spells() -> Vec<Spell> { vec![
    Spell::new(141, "Minor Enchant", MagicSchool::Enchantment, 1, 10, 3, 0, 4, 0, 1, "Small buff").with_effect(SpellEffect::Buff { stat: "attack".into(), amount: 10, dur: 10 }),
    Spell::new(142, "Charm", MagicSchool::Enchantment, 2, 20, 5, 0, 6, 0, 1, "Charm enemy").with_effect(SpellEffect::Charm(5)),
    Spell::new(143, "Empower", MagicSchool::Enchantment, 3, 30, 5, 0, 4, 0, 1, "Boost stats").with_effects(vec![SpellEffect::Buff { stat: "attack".into(), amount: 25, dur: 8 }, SpellEffect::Buff { stat: "defense".into(), amount: 25, dur: 8 }]),
    Spell::new(144, "Enchant Weapon", MagicSchool::Enchantment, 3, 35, 8, 0, 1, 0, 2, "Magic weapon").with_effects(vec![SpellEffect::Buff { stat: "weapon_damage".into(), amount: 30, dur: 15 }, SpellEffect::DamageOverTime { dpt: 5, dur: 15 }]),
    Spell::new(145, "Mass Charm", MagicSchool::Enchantment, 5, 60, 10, 0, 8, 4, 2, "Charm multiple").with_effects(vec![SpellEffect::Charm(4), SpellEffect::Confuse(4)]),
    Spell::new(146, "Dominate", MagicSchool::Enchantment, 6, 70, 12, 0, 6, 0, 2, "Control enemy").with_effects(vec![SpellEffect::Charm(10), SpellEffect::Debuff { stat: "will".into(), amount: -50, dur: 10 }]),
    Spell::new(147, "Heroism", MagicSchool::Enchantment, 5, 50, 8, 0, 0, 5, 2, "Inspire allies").with_effects(vec![SpellEffect::Buff { stat: "all".into(), amount: 20, dur: 10 }, SpellEffect::Cleanse(2)]),
    Spell::new(148, "Polymorph", MagicSchool::Enchantment, 7, 80, 10, 0, 6, 0, 2, "Transform enemy").with_effects(vec![SpellEffect::Transform { form: "Sheep".into(), dur: 8 }, SpellEffect::Weaken { pct: 90, dur: 8 }]),
    Spell::new(149, "Power Word: Stun", MagicSchool::Enchantment, 8, 100, 12, 80, 8, 0, 0, "Instant stun").with_effects(vec![SpellEffect::Damage(80), SpellEffect::Stun(6)]),
    Spell::new(150, "Absolute Command", MagicSchool::Enchantment, 10, 180, 20, 0, 10, 8, 3, "Control all minds").with_effects(vec![SpellEffect::Charm(15), SpellEffect::Buff { stat: "all".into(), amount: 50, dur: 15 }, SpellEffect::Summon { creature: "Dominated".into(), count: 3, dur: 15 }]),
]}

fn create_all_spells() -> Vec<Spell> {
    let mut s = Vec::new();
    s.extend(create_fire_spells()); s.extend(create_ice_spells()); s.extend(create_lightning_spells());
    s.extend(create_earth_spells()); s.extend(create_wind_spells()); s.extend(create_water_spells());
    s.extend(create_light_spells()); s.extend(create_dark_spells()); s.extend(create_arcane_spells());
    s.extend(create_nature_spells()); s.extend(create_blood_spells()); s.extend(create_necromancy_spells());
    s.extend(create_time_spells()); s.extend(create_space_spells()); s.extend(create_enchantment_spells());
    s
}

fn create_spell_combinations() -> Vec<SpellCombination> { vec![
    SpellCombination { schools: (MagicSchool::Fire, MagicSchool::Wind), result_spell_id: 1001, name: "Firestorm".into(), mana_mult: 1.5, damage_mult: 2.0 },
    SpellCombination { schools: (MagicSchool::Ice, MagicSchool::Wind), result_spell_id: 1002, name: "Blizzard Storm".into(), mana_mult: 1.5, damage_mult: 1.8 },
    SpellCombination { schools: (MagicSchool::Fire, MagicSchool::Earth), result_spell_id: 1003, name: "Lava Flow".into(), mana_mult: 1.6, damage_mult: 2.2 },
    SpellCombination { schools: (MagicSchool::Ice, MagicSchool::Water), result_spell_id: 1004, name: "Frozen Tundra".into(), mana_mult: 1.4, damage_mult: 1.7 },
    SpellCombination { schools: (MagicSchool::Lightning, MagicSchool::Water), result_spell_id: 1005, name: "Electrocution".into(), mana_mult: 1.5, damage_mult: 2.5 },
    SpellCombination { schools: (MagicSchool::Fire, MagicSchool::Dark), result_spell_id: 1006, name: "Hellfire".into(), mana_mult: 1.8, damage_mult: 2.3 },
    SpellCombination { schools: (MagicSchool::Light, MagicSchool::Fire), result_spell_id: 1007, name: "Holy Fire".into(), mana_mult: 1.6, damage_mult: 2.0 },
    SpellCombination { schools: (MagicSchool::Dark, MagicSchool::Necromancy), result_spell_id: 1008, name: "Death's Embrace".into(), mana_mult: 1.7, damage_mult: 2.1 },
    SpellCombination { schools: (MagicSchool::Time, MagicSchool::Space), result_spell_id: 1009, name: "Spacetime Rift".into(), mana_mult: 2.0, damage_mult: 3.0 },
    SpellCombination { schools: (MagicSchool::Nature, MagicSchool::Earth), result_spell_id: 1010, name: "Earthquake Bloom".into(), mana_mult: 1.4, damage_mult: 1.9 },
    SpellCombination { schools: (MagicSchool::Blood, MagicSchool::Dark), result_spell_id: 1011, name: "Blood Ritual".into(), mana_mult: 1.6, damage_mult: 2.4 },
    SpellCombination { schools: (MagicSchool::Arcane, MagicSchool::Lightning), result_spell_id: 1012, name: "Arcane Storm".into(), mana_mult: 1.5, damage_mult: 2.2 },
    SpellCombination { schools: (MagicSchool::Light, MagicSchool::Water), result_spell_id: 1013, name: "Purifying Wave".into(), mana_mult: 1.3, damage_mult: 1.5 },
    SpellCombination { schools: (MagicSchool::Wind, MagicSchool::Lightning), result_spell_id: 1014, name: "Thunderstrike".into(), mana_mult: 1.5, damage_mult: 2.3 },
    SpellCombination { schools: (MagicSchool::Enchantment, MagicSchool::Arcane), result_spell_id: 1015, name: "Reality Warp".into(), mana_mult: 1.8, damage_mult: 2.0 },
]}

fn create_combination_spells() -> Vec<Spell> { vec![
    Spell::new(1001, "Firestorm", MagicSchool::Fire, 8, 120, 10, 200, 12, 8, 3, "Fire+Wind: Flame tornado").with_effects(vec![SpellEffect::AoeDamage { radius: 8, dmg: 200 }, SpellEffect::DamageOverTime { dpt: 30, dur: 5 }, SpellEffect::Push(5)]),
    Spell::new(1002, "Blizzard Storm", MagicSchool::Ice, 8, 115, 10, 180, 12, 8, 3, "Ice+Wind: Freezing storm").with_effects(vec![SpellEffect::AoeDamage { radius: 8, dmg: 180 }, SpellEffect::Slow { pct: 70, dur: 6 }, SpellEffect::Stun(3)]),
    Spell::new(1003, "Lava Flow", MagicSchool::Fire, 9, 140, 12, 220, 10, 6, 4, "Fire+Earth: Molten rock").with_effects(vec![SpellEffect::AoeDamage { radius: 6, dmg: 220 }, SpellEffect::DamageOverTime { dpt: 40, dur: 6 }, SpellEffect::Root(4)]),
    Spell::new(1004, "Frozen Tundra", MagicSchool::Ice, 7, 100, 8, 150, 10, 7, 3, "Ice+Water: Frozen zone").with_effects(vec![SpellEffect::AoeDamage { radius: 7, dmg: 150 }, SpellEffect::Slow { pct: 80, dur: 8 }, SpellEffect::DamageOverTime { dpt: 20, dur: 8 }]),
    Spell::new(1005, "Electrocution", MagicSchool::Lightning, 8, 130, 8, 250, 8, 5, 2, "Lightning+Water: Lethal").with_effects(vec![SpellEffect::AoeDamage { radius: 5, dmg: 250 }, SpellEffect::Stun(4), SpellEffect::ChainHit { targets: 8, falloff: 5 }]),
    Spell::new(1006, "Hellfire", MagicSchool::Dark, 9, 160, 12, 280, 10, 7, 4, "Fire+Dark: Abyssal flames").with_effects(vec![SpellEffect::AoeDamage { radius: 7, dmg: 280 }, SpellEffect::Fear(5), SpellEffect::DamageOverTime { dpt: 35, dur: 6 }]),
    Spell::new(1007, "Holy Fire", MagicSchool::Light, 7, 110, 8, 180, 10, 6, 2, "Light+Fire: Sacred flames").with_effects(vec![SpellEffect::AoeDamage { radius: 6, dmg: 180 }, SpellEffect::Heal(60), SpellEffect::Cleanse(5)]),
    Spell::new(1008, "Death's Embrace", MagicSchool::Necromancy, 9, 150, 12, 240, 8, 5, 3, "Dark+Necro: Ultimate death").with_effects(vec![SpellEffect::AoeDamage { radius: 5, dmg: 240 }, SpellEffect::LifeSteal(80), SpellEffect::Summon { creature: "Wraith".into(), count: 3, dur: 12 }]),
    Spell::new(1009, "Spacetime Rift", MagicSchool::Space, 10, 200, 15, 350, 15, 10, 5, "Time+Space: Reality tear").with_effects(vec![SpellEffect::AoeDamage { radius: 10, dmg: 350 }, SpellEffect::TimeStop(2), SpellEffect::Teleport(20), SpellEffect::Stun(5)]),
    Spell::new(1010, "Earthquake Bloom", MagicSchool::Nature, 7, 95, 8, 160, 12, 8, 3, "Nature+Earth: Life from ruin").with_effects(vec![SpellEffect::AoeDamage { radius: 8, dmg: 160 }, SpellEffect::AoeHeal { radius: 8, heal: 80 }, SpellEffect::Root(4)]),
    Spell::new(1011, "Blood Ritual", MagicSchool::Blood, 9, 80, 15, 300, 6, 4, 4, "Blood+Dark: Sacrifice").with_effects(vec![SpellEffect::AoeDamage { radius: 4, dmg: 300 }, SpellEffect::LifeSteal(100), SpellEffect::DamageOverTime { dpt: -30, dur: 3 }]),
    Spell::new(1012, "Arcane Storm", MagicSchool::Arcane, 8, 125, 10, 220, 12, 7, 3, "Arcane+Lightning: Energy storm").with_effects(vec![SpellEffect::AoeDamage { radius: 7, dmg: 220 }, SpellEffect::ManaDrain(40), SpellEffect::ChainHit { targets: 6, falloff: 10 }]),
    Spell::new(1013, "Purifying Wave", MagicSchool::Water, 6, 90, 8, 100, 10, 8, 2, "Light+Water: Cleansing flood").with_effects(vec![SpellEffect::AoeDamage { radius: 8, dmg: 100 }, SpellEffect::AoeHeal { radius: 8, heal: 100 }, SpellEffect::Cleanse(5), SpellEffect::Push(4)]),
    Spell::new(1014, "Thunderstrike", MagicSchool::Lightning, 8, 120, 8, 230, 15, 4, 2, "Wind+Lightning: Precision strike").with_effects(vec![SpellEffect::Damage(230), SpellEffect::AoeDamage { radius: 4, dmg: 120 }, SpellEffect::Stun(4)]),
    Spell::new(1015, "Reality Warp", MagicSchool::Enchantment, 9, 150, 12, 180, 10, 6, 3, "Enchant+Arcane: Reshape reality").with_effects(vec![SpellEffect::AoeDamage { radius: 6, dmg: 180 }, SpellEffect::Charm(8), SpellEffect::Transform { form: "Warped".into(), dur: 6 }, SpellEffect::Confuse(6)]),
]}

// ============================================================================
// PLAYER
// ============================================================================

#[derive(Serialize, Deserialize)]
struct Player {
    x: usize,
    y: usize,
    class: CharacterClass,
    dwarf_subspecies: Option<DwarfSubspecies>,
    dragonian_subspecies: Option<DragonianSubspecies>,
    dragon_form: Option<DragonForm>,
    undead: Option<(UndeadSubspecies, UndeadTier)>,
    hp: i32,
    max_hp: i32,
    mana: i32,
    max_mana: i32,
    base_attack: i32,
    base_defense: i32,
    #[allow(dead_code)]
    speed: i32,
    gold: u32,
    level: u32,
    xp: u32,
    xp_to_level: u32,
    hunger: i32,
    max_hunger: i32,
    keys: u32,
    kills: u32,
    status_effects: HashMap<StatusEffect, u32>,
    equipment: HashMap<EquipSlot, Item>,
    inventory: Vec<Item>,
    skills: Vec<Skill>,
    active_skill: usize,
    minions: Vec<Enemy>,
}

impl Player {
    fn new_dragonian(x: usize, y: usize, class: CharacterClass, subspecies: DragonianSubspecies) -> Self {
        let (base_hp, base_attack, base_defense, base_mana, base_speed) = class.base_stats();
        let (hp_bonus, atk_bonus, def_bonus, spd_bonus, mana_bonus) = subspecies.stat_bonuses();

        Self {
            x,
            y,
            class,
            dwarf_subspecies: None,
            dragonian_subspecies: Some(subspecies),
            dragon_form: Some(DragonForm::Dragonian),
            undead: None,
            hp: base_hp + hp_bonus,
            max_hp: base_hp + hp_bonus,
            mana: base_mana + mana_bonus,
            max_mana: base_mana + mana_bonus,
            base_attack: base_attack + atk_bonus,
            base_defense: base_defense + def_bonus,
            speed: base_speed + spd_bonus,
            gold: 0,
            level: 1,
            xp: 0,
            xp_to_level: 100,
            hunger: 100,
            max_hunger: 100,
            keys: 0,
            kills: 0,
            status_effects: HashMap::new(),
            equipment: HashMap::new(),
            inventory: Vec::new(),
            skills: Skill::for_class(class),
            active_skill: 0,
            minions: Vec::new(),
        }
    }

    fn new(x: usize, y: usize, class: CharacterClass, dwarf_subspecies: Option<DwarfSubspecies>) -> Self {
        let (base_hp, base_attack, base_defense, base_mana, base_speed) = class.base_stats();

        // Apply dwarf subspecies bonuses if present
        let (hp_bonus, atk_bonus, def_bonus, spd_bonus, mana_bonus) = dwarf_subspecies
            .map(|d| d.stat_bonuses())
            .unwrap_or((0, 0, 0, 0, 0));

        let hp = base_hp + hp_bonus;
        let attack = base_attack + atk_bonus;
        let defense = base_defense + def_bonus;
        let mana = base_mana + mana_bonus;
        let speed = base_speed + spd_bonus;

        Self {
            x,
            y,
            class,
            dwarf_subspecies,
            dragonian_subspecies: None,
            dragon_form: None,
            undead: None,
            hp,
            max_hp: hp,
            mana,
            max_mana: mana,
            base_attack: attack,
            base_defense: defense,
            speed,
            gold: 0,
            level: 1,
            xp: 0,
            xp_to_level: 100,
            hunger: 100,
            max_hunger: 100,
            keys: 0,
            kills: 0,
            status_effects: HashMap::new(),
            equipment: HashMap::new(),
            inventory: Vec::new(),
            skills: Skill::for_class(class),
            active_skill: 0,
            minions: Vec::new(),
        }
    }

    fn total_attack(&self) -> i32 {
        let mut total = self.base_attack;

        // Add dragonian evolution bonus
        if let Some(form) = self.dragon_form {
            let (_, atk_bonus, _, _, _) = form.evolution_bonus();
            total += atk_bonus;
        }

        for item in self.equipment.values() {
            let (atk, _, _, _) = item.stats();
            total += atk;
        }
        if self.has_status(StatusEffect::Strength) {
            total = (total as f32 * 1.5) as i32;
        }
        if self.has_status(StatusEffect::Weakness) {
            total = (total as f32 * 0.5) as i32;
        }
        total
    }

    fn total_defense(&self) -> i32 {
        let mut total = self.base_defense;

        // Add dragonian evolution bonus
        if let Some(form) = self.dragon_form {
            let (_, _, def_bonus, _, _) = form.evolution_bonus();
            total += def_bonus;
        }

        for item in self.equipment.values() {
            let (_, def, _, _) = item.stats();
            total += def;
        }
        if self.has_status(StatusEffect::Shield) {
            total += 10;
        }
        total
    }

    fn total_max_hp(&self) -> i32 {
        let mut total = self.max_hp;

        // Add dragonian evolution bonus
        if let Some(form) = self.dragon_form {
            let (hp_bonus, _, _, _, _) = form.evolution_bonus();
            total += hp_bonus;
        }

        for item in self.equipment.values() {
            let (_, _, hp, _) = item.stats();
            total += hp;
        }
        total
    }

    fn total_max_mana(&self) -> i32 {
        let mut total = self.max_mana;

        // Add dragonian evolution bonus
        if let Some(form) = self.dragon_form {
            let (_, _, _, _, mana_bonus) = form.evolution_bonus();
            total += mana_bonus;
        }

        for item in self.equipment.values() {
            let (_, _, _, mana) = item.stats();
            total += mana;
        }
        total
    }

    /// Returns (leveled_up, evolved_form_name)
    fn gain_xp(&mut self, amount: u32) -> (bool, Option<String>) {
        self.xp += amount;
        let mut evolved_form: Option<String> = None;

        if self.xp >= self.xp_to_level {
            self.xp -= self.xp_to_level;
            self.level += 1;
            self.xp_to_level = (self.xp_to_level as f32 * 1.4) as u32;
            self.max_hp += 8 + (self.level as i32 / 3);
            self.hp = self.total_max_hp();
            self.max_mana += 5;
            self.mana = self.total_max_mana();
            self.base_attack += 2;
            self.base_defense += 1;

            // Check for Dragonian evolution
            if let Some(current_form) = self.dragon_form {
                if let Some(new_form) = DragonForm::can_evolve(self.level, current_form) {
                    self.dragon_form = Some(new_form);
                    evolved_form = Some(new_form.name().to_string());
                }
            }

            return (true, evolved_form);
        }
        (false, None)
    }

    fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.total_max_hp());
    }

    fn restore_mana(&mut self, amount: i32) {
        self.mana = (self.mana + amount).min(self.total_max_mana());
    }

    fn eat(&mut self, food_value: i32) {
        self.hunger = (self.hunger + food_value).min(self.max_hunger);
    }

    fn add_status(&mut self, effect: StatusEffect, duration: u32) {
        self.status_effects.insert(effect, duration);
    }

    fn has_status(&self, effect: StatusEffect) -> bool {
        self.status_effects.contains_key(&effect)
    }

    fn remove_status(&mut self, effect: StatusEffect) {
        self.status_effects.remove(&effect);
    }

    fn tick_status_effects(&mut self) -> Vec<String> {
        let mut messages = Vec::new();
        let mut to_remove = Vec::new();
        let mut damage = 0;
        let mut heal = 0;

        for (effect, duration) in self.status_effects.iter_mut() {
            match effect {
                StatusEffect::Poison => {
                    damage += 2;
                    messages.push("You take poison damage!".to_string());
                }
                StatusEffect::Burn => {
                    damage += 3;
                    messages.push("You are burning!".to_string());
                }
                StatusEffect::Bleed => {
                    damage += 1;
                    messages.push("You are bleeding!".to_string());
                }
                StatusEffect::Regeneration => {
                    heal += 3;
                }
                _ => {}
            }
            *duration = duration.saturating_sub(1);
            if *duration == 0 {
                to_remove.push(*effect);
                messages.push(format!("{} wore off.", effect.name()));
            }
        }

        self.hp -= damage;
        self.heal(heal);

        for effect in to_remove {
            self.status_effects.remove(&effect);
        }

        messages
    }

    fn tick_hunger(&mut self) -> Option<String> {
        // Undead don't need to eat - they are sustained by dark energies
        if self.undead.is_some() {
            return None;
        }

        self.hunger -= 1;
        if self.hunger <= 0 {
            self.hp -= 1;
            Some("You are starving!".to_string())
        } else if self.hunger < 20 {
            Some("You are very hungry!".to_string())
        } else {
            None
        }
    }

    fn equip(&mut self, item: Item) -> Option<Item> {
        if let Some(slot) = item.kind.equip_slot() {
            // Handle rings specially - can wear two
            let actual_slot = if slot == EquipSlot::Ring1 {
                if self.equipment.contains_key(&EquipSlot::Ring1) && !self.equipment.contains_key(&EquipSlot::Ring2) {
                    EquipSlot::Ring2
                } else {
                    EquipSlot::Ring1
                }
            } else {
                slot
            };
            let old = self.equipment.remove(&actual_slot);
            self.equipment.insert(actual_slot, item);
            old
        } else {
            None
        }
    }

    fn can_use_skill(&self) -> bool {
        if self.skills.is_empty() {
            return false;
        }
        let skill = self.skills[self.active_skill];
        self.mana >= skill.mana_cost()
    }

    fn current_skill(&self) -> Option<Skill> {
        if self.skills.is_empty() {
            None
        } else {
            Some(self.skills[self.active_skill])
        }
    }
}

// ============================================================================
// ROOM
// ============================================================================

#[derive(Clone, Serialize, Deserialize)]
struct Room {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    #[allow(dead_code)]
    is_boss_room: bool,
}

impl Room {
    fn center(&self) -> (usize, usize) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }

    fn intersects(&self, other: &Room) -> bool {
        self.x <= other.x + other.width + 1
            && self.x + self.width + 1 >= other.x
            && self.y <= other.y + other.height + 1
            && self.y + self.height + 1 >= other.y
    }

    fn random_point(&self, rng: &mut impl Rng) -> (usize, usize) {
        (
            rng.gen_range(self.x + 1..self.x + self.width - 1),
            rng.gen_range(self.y + 1..self.y + self.height - 1),
        )
    }
}

// ============================================================================
// MAP
// ============================================================================

#[derive(Serialize, Deserialize)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    visible: Vec<Vec<bool>>,
    explored: Vec<Vec<bool>>,
    rooms: Vec<Room>,
    theme: DungeonTheme,
}

impl Map {
    fn new() -> Self {
        Self {
            tiles: vec![vec![Tile::Wall; MAP_WIDTH]; MAP_HEIGHT],
            visible: vec![vec![false; MAP_WIDTH]; MAP_HEIGHT],
            explored: vec![vec![false; MAP_WIDTH]; MAP_HEIGHT],
            rooms: Vec::new(),
            theme: DungeonTheme::Dungeon,
        }
    }

    fn generate(&mut self, rng: &mut impl Rng, level: u32) {
        self.theme = DungeonTheme::from_level(level);
        let floor_tile = self.theme.floor_tile();

        // Reset
        self.tiles = vec![vec![Tile::Wall; MAP_WIDTH]; MAP_HEIGHT];
        self.visible = vec![vec![false; MAP_WIDTH]; MAP_HEIGHT];
        self.rooms.clear();

        let is_boss_level = BOSS_LEVELS.contains(&level);

        // Generate rooms
        let target_rooms = if is_boss_level { MAX_ROOMS + 1 } else { MAX_ROOMS };

        for _ in 0..target_rooms * 4 {
            if self.rooms.len() >= target_rooms {
                break;
            }

            let width = rng.gen_range(MIN_ROOM_SIZE..=MAX_ROOM_SIZE);
            let height = rng.gen_range(MIN_ROOM_SIZE..=MAX_ROOM_SIZE);
            let x = rng.gen_range(1..MAP_WIDTH - width - 1);
            let y = rng.gen_range(1..MAP_HEIGHT - height - 1);

            let new_room = Room {
                x,
                y,
                width,
                height,
                is_boss_room: false,
            };

            let overlaps = self.rooms.iter().any(|r| new_room.intersects(r));
            if !overlaps {
                self.carve_room(&new_room, floor_tile);

                if !self.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = self.rooms.last().unwrap().center();

                    if rng.gen_bool(0.5) {
                        self.carve_h_tunnel(prev_x, new_x, prev_y, floor_tile);
                        self.carve_v_tunnel(prev_y, new_y, new_x, floor_tile);
                    } else {
                        self.carve_v_tunnel(prev_y, new_y, prev_x, floor_tile);
                        self.carve_h_tunnel(prev_x, new_x, new_y, floor_tile);
                    }

                    // Add doors at tunnel intersections
                    if rng.gen_bool(0.3) {
                        self.tiles[new_y][prev_x] = Tile::Door;
                    }
                }

                self.rooms.push(new_room);
            }
        }

        // Add special features
        self.add_features(rng, level);

        // Place stairs
        if self.rooms.len() >= 2 {
            let last_room = self.rooms.last().unwrap();
            let (sx, sy) = last_room.center();

            if is_boss_level {
                self.tiles[sy][sx] = Tile::BossGate;
            } else {
                self.tiles[sy][sx] = Tile::StairsDown;
            }

            if level > 1 {
                let first_room = &self.rooms[0];
                let (ux, uy) = first_room.center();
                self.tiles[uy][ux] = Tile::StairsUp;
            }
        }
    }

    fn carve_room(&mut self, room: &Room, floor_tile: Tile) {
        for y in room.y..room.y + room.height {
            for x in room.x..room.x + room.width {
                self.tiles[y][x] = floor_tile;
            }
        }
    }

    fn carve_h_tunnel(&mut self, x1: usize, x2: usize, y: usize, floor_tile: Tile) {
        let (start, end) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        for x in start..=end {
            if self.tiles[y][x] == Tile::Wall {
                self.tiles[y][x] = floor_tile;
            }
        }
    }

    fn carve_v_tunnel(&mut self, y1: usize, y2: usize, x: usize, floor_tile: Tile) {
        let (start, end) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        for y in start..=end {
            if self.tiles[y][x] == Tile::Wall {
                self.tiles[y][x] = floor_tile;
            }
        }
    }

    fn add_features(&mut self, rng: &mut impl Rng, level: u32) {
        // Add traps
        for room in &self.rooms[1..] {
            if rng.gen_bool(0.2) {
                let (x, y) = room.random_point(rng);
                if self.tiles[y][x].walkable() && self.tiles[y][x] != Tile::StairsDown {
                    self.tiles[y][x] = Tile::Trap;
                }
            }
        }

        // Add chests
        for room in &self.rooms[1..] {
            if rng.gen_bool(0.15) {
                let (x, y) = room.random_point(rng);
                if self.tiles[y][x].walkable() {
                    self.tiles[y][x] = Tile::Chest;
                }
            }
        }

        // Add shrines
        if rng.gen_bool(0.1 + level as f64 * 0.01) {
            if let Some(room) = self.rooms.get(rng.gen_range(1..self.rooms.len())) {
                let (x, y) = room.center();
                if self.tiles[y][x].walkable() {
                    self.tiles[y][x] = Tile::Shrine;
                }
            }
        }

        // Add water/lava pools based on theme
        match self.theme {
            DungeonTheme::Cave | DungeonTheme::Dungeon => {
                for room in &self.rooms {
                    if rng.gen_bool(0.1) {
                        let (x, y) = room.random_point(rng);
                        for dy in 0..3 {
                            for dx in 0..3 {
                                let nx = x + dx;
                                let ny = y + dy;
                                if nx < MAP_WIDTH && ny < MAP_HEIGHT && self.tiles[ny][nx].walkable() {
                                    self.tiles[ny][nx] = Tile::Water;
                                }
                            }
                        }
                    }
                }
            }
            DungeonTheme::VolcanicLair | DungeonTheme::DemonRealm => {
                for room in &self.rooms {
                    if rng.gen_bool(0.15) {
                        let (x, y) = room.random_point(rng);
                        for dy in 0..2 {
                            for dx in 0..2 {
                                let nx = x + dx;
                                let ny = y + dy;
                                if nx < MAP_WIDTH && ny < MAP_HEIGHT && self.tiles[ny][nx].walkable() {
                                    self.tiles[ny][nx] = Tile::Lava;
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        // Add pillars
        for room in &self.rooms {
            if room.width > 8 && room.height > 8 && rng.gen_bool(0.3) {
                let cx = room.x + room.width / 2;
                let cy = room.y + room.height / 2;
                for &(dx, dy) in &[(-2i32, -2i32), (2, -2), (-2, 2), (2, 2)] {
                    let px = (cx as i32 + dx) as usize;
                    let py = (cy as i32 + dy) as usize;
                    if self.tiles[py][px].walkable() {
                        self.tiles[py][px] = Tile::Pillar;
                    }
                }
            }
        }
    }

    fn compute_fov(&mut self, px: usize, py: usize) {
        for row in &mut self.visible {
            for cell in row {
                *cell = false;
            }
        }

        for angle in 0..360 {
            let rad = (angle as f32) * std::f32::consts::PI / 180.0;
            let dx = rad.cos();
            let dy = rad.sin();

            let mut x = px as f32 + 0.5;
            let mut y = py as f32 + 0.5;

            for _ in 0..VIEW_RADIUS {
                let ix = x as usize;
                let iy = y as usize;

                if ix >= MAP_WIDTH || iy >= MAP_HEIGHT {
                    break;
                }

                self.visible[iy][ix] = true;
                self.explored[iy][ix] = true;

                if self.tiles[iy][ix].blocks_sight() {
                    break;
                }

                x += dx;
                y += dy;
            }
        }
    }

    fn is_walkable(&self, x: usize, y: usize) -> bool {
        if x >= MAP_WIDTH || y >= MAP_HEIGHT {
            return false;
        }
        self.tiles[y][x].walkable()
    }

    fn reveal_all(&mut self) {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                self.explored[y][x] = true;
            }
        }
    }
}

// ============================================================================
// GAME STATE
// ============================================================================

struct GameState {
    map: Map,
    player: Player,
    enemies: Vec<Enemy>,
    items: Vec<Item>,
    npcs: Vec<NPC>,
    messages: Vec<(String, Color)>,
    dungeon_level: u32,
    turn_count: u32,
    game_over: bool,
    victory: bool,
    boss_defeated: bool,
    show_inventory: bool,
    show_help: bool,
    show_dialogue: bool,
    show_shop: bool,
    show_achievements: bool,
    show_quest_journal: bool,
    current_npc_idx: Option<usize>,
    shop_selection: usize,
    achievement_tracker: AchievementTracker,
    quest_journal: QuestJournal,
    rng: StdRng,
}

impl GameState {
    fn new(class: CharacterClass, dwarf_subspecies: Option<DwarfSubspecies>) -> Self {
        let mut rng = StdRng::from_entropy();
        let mut map = Map::new();
        map.generate(&mut rng, 1);

        let (px, py) = map.rooms[0].center();
        let player = Player::new(px, py, class, dwarf_subspecies);

        let mut achievement_tracker = AchievementTracker::load();
        achievement_tracker.reset_session();

        let mut state = Self {
            map,
            player,
            enemies: Vec::new(),
            items: Vec::new(),
            npcs: Vec::new(),
            messages: Vec::new(),
            dungeon_level: 1,
            turn_count: 0,
            game_over: false,
            victory: false,
            boss_defeated: false,
            show_inventory: false,
            show_help: false,
            show_dialogue: false,
            show_shop: false,
            show_achievements: false,
            show_quest_journal: false,
            current_npc_idx: None,
            shop_selection: 0,
            achievement_tracker,
            quest_journal: QuestJournal::new(),
            rng,
        };

        state.add_message(format!("Welcome, {}! Descend to level 30 to defeat the Demon King!", class.name()), Color::Cyan);
        state.add_message(format!("Press ? for help. Your skill: {}", class.special_ability()), Color::Yellow);
        state.spawn_enemies();
        state.spawn_items();
        state.spawn_npcs();
        state.map.compute_fov(state.player.x, state.player.y);

        state
    }

    fn add_message(&mut self, msg: String, color: Color) {
        self.messages.push((msg, color));
        if self.messages.len() > 6 {
            self.messages.remove(0);
        }
    }

    fn spawn_enemies(&mut self) {
        self.enemies.clear();

        let is_boss_level = BOSS_LEVELS.contains(&self.dungeon_level);

        for (i, room) in self.map.rooms.iter().enumerate() {
            if i == 0 {
                continue;
            }

            // Spawn boss in last room on boss levels
            if is_boss_level && i == self.map.rooms.len() - 1 && !self.boss_defeated {
                if let Some(boss_kind) = EnemyKind::boss_for_level(self.dungeon_level) {
                    let (bx, by) = room.center();
                    self.enemies.push(Enemy::new(bx, by, boss_kind, self.dungeon_level));
                    continue;
                }
            }

            let max_enemies = 2 + (self.dungeon_level as usize / 5);
            let num_enemies = self.rng.gen_range(1..=max_enemies.min(5));

            for _ in 0..num_enemies {
                let (x, y) = room.random_point(&mut self.rng);
                let kind = EnemyKind::for_level(self.dungeon_level, &mut self.rng);
                self.enemies.push(Enemy::new(x, y, kind, self.dungeon_level));
            }
        }
    }

    fn spawn_items(&mut self) {
        self.items.clear();

        // Copy room data to avoid borrow issues
        let rooms: Vec<Room> = self.map.rooms.clone();

        for (i, room) in rooms.iter().enumerate() {
            if i == 0 { continue; }

            let num_items = self.rng.gen_range(0..=3);
            for _ in 0..num_items {
                let x = self.rng.gen_range(room.x + 1..room.x + room.width - 1);
                let y = self.rng.gen_range(room.y + 1..room.y + room.height - 1);
                let (kind, rarity) = self.random_item();
                self.items.push(Item::new(x, y, kind, rarity));
            }

            // Gold
            if self.rng.gen_bool(0.4) {
                let x = self.rng.gen_range(room.x + 1..room.x + room.width - 1);
                let y = self.rng.gen_range(room.y + 1..room.y + room.height - 1);
                self.items.push(Item::new(x, y, ItemKind::Gold, Rarity::Common));
            }

            // Food
            if self.rng.gen_bool(0.15) {
                let x = self.rng.gen_range(room.x + 1..room.x + room.width - 1);
                let y = self.rng.gen_range(room.y + 1..room.y + room.height - 1);
                let food = match self.rng.gen_range(0..5) {
                    0 => ItemKind::Apple,
                    1 => ItemKind::Bread,
                    2 => ItemKind::Cheese,
                    3 => ItemKind::Meat,
                    _ => ItemKind::Bread,
                };
                self.items.push(Item::new(x, y, food, Rarity::Common));
            }
        }
    }

    fn spawn_npcs(&mut self) {
        self.npcs.clear();
        let rooms: Vec<Room> = self.map.rooms.clone();
        let level = self.dungeon_level;
        for (i, room) in rooms.iter().enumerate() {
            if i == 0 { continue; }
            if i == rooms.len() - 1 && BOSS_LEVELS.contains(&level) { continue; }
            if self.rng.gen_bool(0.3) {
                let (x, y) = room.center();
                let npc = match self.rng.gen_range(0..10) {
                    0..=2 => NPC::merchant(x, y),
                    3 => NPC::healer(x, y),
                    4 => NPC::blacksmith(x, y),
                    5 => NPC::alchemist(x, y),
                    6 => NPC::sage(x, y, level),
                    7 => NPC::guard(x, y),
                    _ => NPC::enchanter(x, y),
                };
                self.npcs.push(npc);
            }
        }
        if level <= 3 && !self.npcs.iter().any(|n| n.npc_type == NPCType::Merchant) {
            if rooms.len() > 2 {
                let (x, y) = rooms[1].center();
                self.npcs.push(NPC::merchant(x, y));
            }
        }
    }

    fn random_item(&mut self) -> (ItemKind, Rarity) {
        let floor_bonus = self.dungeon_level as i32;
        let rarity = match self.rng.gen_range(0..100) + floor_bonus {
            0..=45 => Rarity::Common,
            46..=70 => Rarity::Uncommon,
            71..=88 => Rarity::Rare,
            89..=96 => Rarity::Epic,
            97..=105 => Rarity::Legendary,
            _ => Rarity::Mythic,
        };

        let kind = match self.rng.gen_range(0..100) {
            // Potions 25%
            0..=24 => {
                let potion_tier = match rarity {
                    Rarity::Common | Rarity::Uncommon => self.rng.gen_range(0..8),
                    Rarity::Rare => self.rng.gen_range(0..14),
                    _ => self.rng.gen_range(0..20),
                };
                match potion_tier {
                    0 => ItemKind::HealthPotion,
                    1 => ItemKind::ManaPotion,
                    2 => ItemKind::StrengthPotion,
                    3 => ItemKind::DefensePotion,
                    4 => ItemKind::SpeedPotion,
                    5 => ItemKind::RegenerationPotion,
                    6 => ItemKind::InvisibilityPotion,
                    7 => ItemKind::FireResistPotion,
                    8 => ItemKind::IceResistPotion,
                    9 => ItemKind::PoisonResistPotion,
                    10 => ItemKind::BerserkPotion,
                    11 => ItemKind::GiantPotion,
                    12 => ItemKind::LevitationPotion,
                    13 => ItemKind::XPPotion,
                    14 => ItemKind::FullRestorePotion,
                    15 => ItemKind::LuckPotion,
                    16 => ItemKind::CriticalPotion,
                    17 => ItemKind::VisionPotion,
                    18 => ItemKind::CureAllPotion,
                    _ => ItemKind::UltimatePowerPotion,
                }
            },
            // Scrolls 15%
            25..=39 => {
                let scroll_tier = match rarity {
                    Rarity::Common | Rarity::Uncommon => self.rng.gen_range(0..8),
                    Rarity::Rare | Rarity::Epic => self.rng.gen_range(0..14),
                    _ => self.rng.gen_range(0..18),
                };
                match scroll_tier {
                    0 => ItemKind::ScrollTeleport,
                    1 => ItemKind::ScrollFireball,
                    2 => ItemKind::ScrollIceStorm,
                    3 => ItemKind::ScrollLightning,
                    4 => ItemKind::ScrollMapping,
                    5 => ItemKind::ScrollIdentify,
                    6 => ItemKind::ScrollEnchant,
                    7 => ItemKind::ScrollSummon,
                    8 => ItemKind::ScrollBanish,
                    9 => ItemKind::ScrollTimeStop,
                    10 => ItemKind::ScrollMassHeal,
                    11 => ItemKind::ScrollDeath,
                    12 => ItemKind::ScrollEarthquake,
                    13 => ItemKind::ScrollMeteor,
                    14 => ItemKind::ScrollBlizzard,
                    15 => ItemKind::ScrollChainLightning,
                    16 => ItemKind::ScrollDivineWrath,
                    _ => ItemKind::ScrollDarkness,
                }
            },
            // Weapons 18%
            40..=57 => {
                let weapon_tier = match rarity {
                    Rarity::Common => self.rng.gen_range(0..8),
                    Rarity::Uncommon => self.rng.gen_range(0..15),
                    Rarity::Rare => self.rng.gen_range(0..20),
                    _ => self.rng.gen_range(0..25),
                };
                match weapon_tier {
                    0 => ItemKind::Dagger,
                    1 => ItemKind::ShortSword,
                    2 => ItemKind::LongSword,
                    3 => ItemKind::Axe,
                    4 => ItemKind::Mace,
                    5 => ItemKind::Spear,
                    6 => ItemKind::Staff,
                    7 => ItemKind::Wand,
                    8 => ItemKind::Greatsword,
                    9 => ItemKind::BattleAxe,
                    10 => ItemKind::WarHammer,
                    11 => ItemKind::Halberd,
                    12 => ItemKind::Bow,
                    13 => ItemKind::Crossbow,
                    14 => ItemKind::Scythe,
                    15 => ItemKind::Katana,
                    16 => ItemKind::Rapier,
                    17 => ItemKind::Flail,
                    18 => ItemKind::Morningstar,
                    19 => ItemKind::Trident,
                    20 => ItemKind::FlameSword,
                    21 => ItemKind::FrostBlade,
                    22 => ItemKind::ThunderAxe,
                    23 => ItemKind::VoidStaff,
                    _ => ItemKind::DemonSlayer,
                }
            },
            // Armor & Shields 16%
            58..=73 => {
                let armor_tier = match rarity {
                    Rarity::Common => self.rng.gen_range(0..8),
                    Rarity::Uncommon => self.rng.gen_range(0..14),
                    Rarity::Rare => self.rng.gen_range(0..22),
                    _ => self.rng.gen_range(0..32),
                };
                match armor_tier {
                    0 => ItemKind::LeatherArmor,
                    1 => ItemKind::ChainMail,
                    2 => ItemKind::Buckler,
                    3 => ItemKind::WoodenShield,
                    4 => ItemKind::LeatherCap,
                    5 => ItemKind::LeatherGloves,
                    6 => ItemKind::LeatherBoots,
                    7 => ItemKind::MageRobes,
                    8 => ItemKind::ScaleMail,
                    9 => ItemKind::PlateMail,
                    10 => ItemKind::IronShield,
                    11 => ItemKind::IronHelm,
                    12 => ItemKind::IronGauntlets,
                    13 => ItemKind::IronBoots,
                    14 => ItemKind::TowerShield,
                    15 => ItemKind::MagicShield,
                    16 => ItemKind::SteelHelm,
                    17 => ItemKind::BootsOfSpeed,
                    18 => ItemKind::AssassinGarb,
                    19 => ItemKind::WizardHat,
                    20 => ItemKind::DragonArmor,
                    21 => ItemKind::DragonShield,
                    22 => ItemKind::SpikedShield,
                    23 => ItemKind::MirrorShield,
                    24 => ItemKind::PhoenixShield,
                    25 => ItemKind::AbyssalShield,
                    26 => ItemKind::HolyArmor,
                    27 => ItemKind::DemonArmor,
                    28 => ItemKind::CrystalArmor,
                    29 => ItemKind::ShadowCloak,
                    30 => ItemKind::TitanPlate,
                    _ => ItemKind::DragonArmor,
                }
            },
            // Helmets, Gloves, Boots 10%
            74..=83 => {
                let gear_tier = match rarity {
                    Rarity::Common | Rarity::Uncommon => self.rng.gen_range(0..8),
                    Rarity::Rare => self.rng.gen_range(0..16),
                    _ => self.rng.gen_range(0..26),
                };
                match gear_tier {
                    0 => ItemKind::LeatherCap,
                    1 => ItemKind::LeatherGloves,
                    2 => ItemKind::LeatherBoots,
                    3 => ItemKind::IronHelm,
                    4 => ItemKind::IronGauntlets,
                    5 => ItemKind::IronBoots,
                    6 => ItemKind::BootsOfSpeed,
                    7 => ItemKind::SteelHelm,
                    8 => ItemKind::CrownOfKings,
                    9 => ItemKind::WizardHat,
                    10 => ItemKind::DemonSkull,
                    11 => ItemKind::DragonHelm,
                    12 => ItemKind::CrystalCrown,
                    13 => ItemKind::HoodOfShadows,
                    14 => ItemKind::HelmOfValor,
                    15 => ItemKind::GlovesOfPower,
                    16 => ItemKind::ThievesGloves,
                    17 => ItemKind::DragonGauntlets,
                    18 => ItemKind::FrostGauntlets,
                    19 => ItemKind::FlameGauntlets,
                    20 => ItemKind::GauntletsOfMight,
                    21 => ItemKind::BootsOfLeaping,
                    22 => ItemKind::WingedBoots,
                    23 => ItemKind::ShadowBoots,
                    24 => ItemKind::LavaWalkers,
                    _ => ItemKind::BootsOfTheWind,
                }
            },
            // Rings and amulets 12%
            84..=95 => {
                let jewel_tier = match rarity {
                    Rarity::Common | Rarity::Uncommon => self.rng.gen_range(0..10),
                    Rarity::Rare | Rarity::Epic => self.rng.gen_range(0..20),
                    _ => self.rng.gen_range(0..28),
                };
                match jewel_tier {
                    0 => ItemKind::RingOfStrength,
                    1 => ItemKind::RingOfProtection,
                    2 => ItemKind::RingOfSpeed,
                    3 => ItemKind::RingOfRegeneration,
                    4 => ItemKind::RingOfMana,
                    5 => ItemKind::AmuletOfHealth,
                    6 => ItemKind::AmuletOfMana,
                    7 => ItemKind::AmuletOfProtection,
                    8 => ItemKind::AmuletOfPower,
                    9 => ItemKind::AmuletOfWisdom,
                    10 => ItemKind::RingOfFireball,
                    11 => ItemKind::RingOfInvisibility,
                    12 => ItemKind::RingOfTheVampire,
                    13 => ItemKind::RingOfLuck,
                    14 => ItemKind::AmuletOfLife,
                    15 => ItemKind::RingOfDeath,
                    16 => ItemKind::RingOfFrost,
                    17 => ItemKind::RingOfFlame,
                    18 => ItemKind::RingOfThunder,
                    19 => ItemKind::RingOfShadows,
                    20 => ItemKind::RingOfTheAncients,
                    21 => ItemKind::AmuletOfDeath,
                    22 => ItemKind::AmuletOfTheGods,
                    23 => ItemKind::AmuletOfDragons,
                    24 => ItemKind::AmuletOfChaos,
                    25 => ItemKind::AmuletOfOrder,
                    26 => ItemKind::AmuletOfBalance,
                    _ => ItemKind::AmuletOfLife,
                }
            },
            // Food and misc 4%
            _ => {
                let misc_tier = match rarity {
                    Rarity::Common | Rarity::Uncommon => self.rng.gen_range(0..6),
                    Rarity::Rare => self.rng.gen_range(0..12),
                    _ => self.rng.gen_range(0..18),
                };
                match misc_tier {
                    0 => ItemKind::Bread,
                    1 => ItemKind::Apple,
                    2 => ItemKind::Meat,
                    3 => ItemKind::Key,
                    4 => ItemKind::Bomb,
                    5 => ItemKind::Torch,
                    6 => ItemKind::Cheese,
                    7 => ItemKind::Meat,
                    8 => ItemKind::Feast,
                    9 => ItemKind::DragonFruit,
                    10 => ItemKind::AncientWine,
                    11 => ItemKind::GoldenApple,
                    12 => ItemKind::Compass,
                    13 => ItemKind::TeleportCrystal,
                    14 => ItemKind::SoulGem,
                    15 => ItemKind::AncientRelic,
                    16 => ItemKind::DragonScale,
                    _ => ItemKind::DemonHeart,
                }
            },
        };

        (kind, rarity)
    }

    fn move_player(&mut self, dx: i32, dy: i32) {
        if self.player.has_status(StatusEffect::Stun) {
            self.add_message("You are stunned!".to_string(), Color::Yellow);
            self.end_turn();
            return;
        }

        let (dx, dy) = if self.player.has_status(StatusEffect::Confusion) && self.rng.gen_bool(0.3) {
            let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)];
            dirs[self.rng.gen_range(0..8)]
        } else {
            (dx, dy)
        };

        let new_x = (self.player.x as i32 + dx).max(0) as usize;
        let new_y = (self.player.y as i32 + dy).max(0) as usize;

        // Check for NPC
        if let Some(idx) = self.npcs.iter().position(|n| n.x == new_x && n.y == new_y && n.friendly) {
            self.interact_npc(idx);
            return;
        }

        // Check for enemy
        if let Some(idx) = self.enemies.iter().position(|e| e.x == new_x && e.y == new_y && e.is_alive()) {
            self.attack_enemy(idx);
            return;
        }

        // Check for door
        if self.map.tiles[new_y][new_x] == Tile::Door {
            self.map.tiles[new_y][new_x] = Tile::OpenDoor;
            self.add_message("You open the door.".to_string(), Color::Grey);
            self.end_turn();
            return;
        }

        // Check for chest
        if self.map.tiles[new_y][new_x] == Tile::Chest {
            self.open_chest(new_x, new_y);
            return;
        }

        // Check for shrine
        if self.map.tiles[new_y][new_x] == Tile::Shrine {
            self.use_shrine(new_x, new_y);
            return;
        }

        if self.map.is_walkable(new_x, new_y) {
            self.player.x = new_x;
            self.player.y = new_y;
            self.map.compute_fov(self.player.x, self.player.y);

            // Check for trap
            if self.map.tiles[new_y][new_x] == Tile::Trap {
                self.trigger_trap();
            }

            // Check for lava
            if self.map.tiles[new_y][new_x] == Tile::Lava {
                let damage = 5 + self.dungeon_level as i32;
                self.player.hp -= damage;
                self.add_message(format!("The lava burns you for {} damage!", damage), Color::Red);
                self.player.add_status(StatusEffect::Burn, 3);
            }

            self.pickup_items();
            self.end_turn();
        }
    }

    fn interact_npc(&mut self, idx: usize) {
        let npc = &self.npcs[idx];
        let npc_type = npc.npc_type;
        let greeting = match npc_type {
            npc::NPCType::Merchant => "The merchant shows you their wares.",
            npc::NPCType::Healer => "The healer offers to restore your health.",
            npc::NPCType::Blacksmith => "The blacksmith can upgrade your equipment.",
            npc::NPCType::QuestGiver => "The mysterious figure has a task for you.",
            npc::NPCType::Trainer => "The trainer can teach you new skills.",
            npc::NPCType::Sage => "The sage shares ancient knowledge.",
            npc::NPCType::Alchemist => "The alchemist offers to craft potions.",
            npc::NPCType::Enchanter => "The enchanter can imbue your items with magic.",
            npc::NPCType::Guard => "The guard nods at you.",
            npc::NPCType::Prisoner => "The prisoner begs for help.",
            npc::NPCType::Companion => "Your companion awaits your command.",
        };
        self.add_message(greeting.to_string(), Color::Cyan);
        self.end_turn();
    }

    fn attack_enemy(&mut self, idx: usize) {
        let player_attack = self.player.total_attack();
        let damage = self.enemies[idx].take_damage(player_attack);

        let enemy_name = self.enemies[idx].kind.name();
        self.add_message(format!("You hit {} for {} damage!", enemy_name, damage), Color::White);

        // Check for vampire ring life steal
        if self.player.equipment.values().any(|i| i.kind == ItemKind::RingOfTheVampire) {
            let heal = damage / 4;
            if heal > 0 {
                self.player.heal(heal);
                self.add_message(format!("Life steal: +{} HP", heal), Color::Magenta);
            }
        }

        if !self.enemies[idx].is_alive() {
            let xp = self.enemies[idx].xp_value;
            let is_boss = self.enemies[idx].kind.is_boss();
            let enemy_kind = self.enemies[idx].kind;

            self.add_message(format!("{} is dead! +{} XP", enemy_name, xp), Color::Green);
            self.player.kills += 1;

            // Update quest progress for kills
            self.quest_journal.update_progress(&ObjectiveType::KillEnemyType(enemy_kind, 1), 1);
            self.quest_journal.update_progress(&ObjectiveType::KillAnyEnemy(1), 1);
            if is_boss {
                self.quest_journal.update_progress(&ObjectiveType::KillBoss(Some(enemy_kind)), 1);
                self.quest_journal.update_progress(&ObjectiveType::KillBoss(None), 1);
            }

            let (leveled_up, evolved) = self.player.gain_xp(xp);
            if leveled_up {
                self.add_message(format!("LEVEL UP! You are now level {}!", self.player.level), Color::Yellow);
                // Update quest progress for level up
                self.quest_journal.update_progress(&ObjectiveType::LevelUp(self.player.level), 1);
                if let Some(form_name) = evolved {
                    self.add_message(format!("EVOLUTION! You have evolved into a {}!", form_name), Color::Magenta);
                }
            }

            if is_boss {
                self.boss_defeated = true;
                self.add_message("BOSS DEFEATED! The stairs are now accessible!".to_string(), Color::Yellow);

                // Boss drops legendary loot
                let loot_kinds = [
                    ItemKind::DragonArmor, ItemKind::DragonShield, ItemKind::Scythe,
                    ItemKind::CrownOfKings, ItemKind::AmuletOfTheGods, ItemKind::DragonGauntlets,
                ];
                let loot_kind = loot_kinds[self.rng.gen_range(0..loot_kinds.len())];
                self.items.push(Item::new(
                    self.enemies[idx].x,
                    self.enemies[idx].y,
                    loot_kind,
                    Rarity::Legendary,
                ));

                if self.dungeon_level == 30 {
                    self.victory = true;
                    self.add_message("YOU HAVE DEFEATED THE DEMON KING! VICTORY!".to_string(), Color::Yellow);
                }
            }
        }

        self.end_turn();
    }

    fn trigger_trap(&mut self) {
        let trap_type = self.rng.gen_range(0..5);
        match trap_type {
            0 => {
                let damage = 5 + self.dungeon_level as i32 / 2;
                self.player.hp -= damage;
                self.add_message(format!("Spike trap! {} damage!", damage), Color::Red);
            }
            1 => {
                self.player.add_status(StatusEffect::Poison, 5);
                self.add_message("Poison dart trap! You are poisoned!".to_string(), Color::Green);
            }
            2 => {
                self.teleport_player_random();
                self.add_message("Teleport trap! You are transported!".to_string(), Color::Blue);
            }
            3 => {
                self.player.add_status(StatusEffect::Blind, 10);
                self.add_message("Flash trap! You are blinded!".to_string(), Color::Yellow);
            }
            _ => {
                // Spawn enemies
                let (px, py) = (self.player.x, self.player.y);
                for _ in 0..2 {
                    let dx = self.rng.gen_range(-2..=2);
                    let dy = self.rng.gen_range(-2..=2);
                    let nx = (px as i32 + dx).max(0) as usize;
                    let ny = (py as i32 + dy).max(0) as usize;
                    if self.map.is_walkable(nx, ny) {
                        let kind = EnemyKind::for_level(self.dungeon_level, &mut self.rng);
                        self.enemies.push(Enemy::new(nx, ny, kind, self.dungeon_level));
                    }
                }
                self.add_message("Alarm trap! Enemies appear!".to_string(), Color::Red);
            }
        }
        self.map.tiles[self.player.y][self.player.x] = Tile::DisarmedTrap;
    }

    fn open_chest(&mut self, x: usize, y: usize) {
        self.map.tiles[y][x] = Tile::OpenChest;

        let num_items = self.rng.gen_range(1..=3);
        for _ in 0..num_items {
            let (kind, mut rarity) = self.random_item();
            // Chest items tend to be better
            if self.rng.gen_bool(0.3) {
                rarity = match rarity {
                    Rarity::Common => Rarity::Uncommon,
                    Rarity::Uncommon => Rarity::Rare,
                    Rarity::Rare => Rarity::Epic,
                    r => r,
                };
            }
            self.items.push(Item::new(x, y, kind, rarity));
        }

        // Gold in chests
        let gold_amount = self.rng.gen_range(10..=50) * self.dungeon_level;
        self.player.gold += gold_amount;

        self.add_message(format!("You open the chest! Found {} gold and {} items!", gold_amount, num_items), Color::Yellow);
        self.end_turn();
    }

    fn use_shrine(&mut self, x: usize, y: usize) {
        self.map.tiles[y][x] = Tile::UsedShrine;

        let effect = self.rng.gen_range(0..6);
        match effect {
            0 => {
                self.player.hp = self.player.total_max_hp();
                self.player.mana = self.player.total_max_mana();
                self.add_message("Shrine of Restoration! Fully healed!".to_string(), Color::Magenta);
            }
            1 => {
                self.player.max_hp += 10;
                self.player.hp += 10;
                self.add_message("Shrine of Vitality! +10 Max HP!".to_string(), Color::Red);
            }
            2 => {
                self.player.base_attack += 3;
                self.add_message("Shrine of Power! +3 Attack!".to_string(), Color::Yellow);
            }
            3 => {
                self.player.base_defense += 2;
                self.add_message("Shrine of Protection! +2 Defense!".to_string(), Color::Cyan);
            }
            4 => {
                self.player.max_mana += 15;
                self.player.mana += 15;
                self.add_message("Shrine of Wisdom! +15 Max Mana!".to_string(), Color::Blue);
            }
            _ => {
                let xp = 50 * self.dungeon_level;
                let (leveled_up, evolved) = self.player.gain_xp(xp);
                if leveled_up {
                    self.add_message(format!("Shrine of Experience! +{} XP! LEVEL UP!", xp), Color::Yellow);
                    if let Some(form_name) = evolved {
                        self.add_message(format!("EVOLUTION! You have evolved into a {}!", form_name), Color::Magenta);
                    }
                } else {
                    self.add_message(format!("Shrine of Experience! +{} XP!", xp), Color::Green);
                }
            }
        }
        self.end_turn();
    }

    fn pickup_items(&mut self) {
        let px = self.player.x;
        let py = self.player.y;

        let mut picked_up: Vec<(usize, ItemKind, Rarity)> = Vec::new();

        for (idx, item) in self.items.iter().enumerate() {
            if item.x == px && item.y == py {
                picked_up.push((idx, item.kind, item.rarity));
            }
        }

        for (_, kind, rarity) in picked_up.iter().rev() {
            match kind {
                ItemKind::Gold => {
                    let amount = self.rng.gen_range(5..=25) * (1 + self.dungeon_level / 3);
                    self.player.gold += amount;
                    self.add_message(format!("Picked up {} gold!", amount), Color::Yellow);
                    // Update quest progress for gold collection
                    self.quest_journal.update_progress(&ObjectiveType::CollectGold(amount), amount);
                }
                ItemKind::Key => {
                    self.player.keys += 1;
                    self.add_message("Picked up a key!".to_string(), Color::Yellow);
                }
                _ => {
                    let display_name = format!("{}{}", rarity.prefix(), kind.name());
                    if kind.equip_slot().is_some() || kind.is_consumable() {
                        if self.player.inventory.len() < 20 {
                            self.player.inventory.push(Item::new(0, 0, *kind, *rarity));
                            self.add_message(format!("Picked up {}!", display_name), rarity.color());
                            // Update quest progress for item collection
                            self.quest_journal.update_progress(&ObjectiveType::CollectItem(*kind, 1), 1);
                            self.quest_journal.update_progress(&ObjectiveType::CollectAnyItems(1), 1);
                            self.quest_journal.update_progress(&ObjectiveType::CollectRarity(*rarity, 1), 1);
                        } else {
                            self.add_message("Inventory full!".to_string(), Color::Red);
                        }
                    }
                }
            }
        }

        // Remove picked up items (Gold and Key don't go to inventory)
        let to_remove: Vec<usize> = self.items.iter().enumerate()
            .filter(|(_, item)| item.x == px && item.y == py)
            .map(|(idx, _)| idx)
            .collect();
        for idx in to_remove.into_iter().rev() {
            self.items.remove(idx);
        }
    }

    fn end_turn(&mut self) {
        self.turn_count += 1;
        self.enemy_turn();

        // Update quest progress for survival
        self.quest_journal.update_progress(&ObjectiveType::SurviveTurns(1), 1);

        // Check quest deadlines
        self.quest_journal.check_deadlines(self.turn_count);

        // Tick player status effects
        let status_msgs = self.player.tick_status_effects();
        for msg in status_msgs {
            self.add_message(msg, Color::Grey);
        }

        // Tick hunger every 20 turns
        if self.turn_count % 20 == 0 {
            if let Some(msg) = self.player.tick_hunger() {
                self.add_message(msg, Color::Red);
            }
        }

        // Regeneration from ring
        if self.player.equipment.values().any(|i| i.kind == ItemKind::RingOfRegeneration) {
            if self.turn_count % 5 == 0 {
                self.player.heal(1);
            }
        }

        // Check death
        if self.player.hp <= 0 {
            self.game_over = true;
            self.add_message("You have died! Game Over.".to_string(), Color::Red);
        }
    }

    fn enemy_turn(&mut self) {
        let mut attacks: Vec<(usize, i32, Option<StatusEffect>)> = Vec::new();
        let mut moves: Vec<(usize, usize, usize)> = Vec::new();

        let player_invisible = self.player.has_status(StatusEffect::Invisibility);
        let enemy_positions: Vec<(usize, usize)> = self.enemies.iter().map(|e| (e.x, e.y)).collect();

        for (idx, enemy) in self.enemies.iter_mut().enumerate() {
            if !enemy.is_alive() {
                continue;
            }

            // Tick enemy status effects
            let damage_events = enemy.tick_status_effects();
            for (_effect, dmg) in damage_events {
                enemy.hp -= dmg;
                if !enemy.is_alive() {
                    continue;
                }
            }

            if enemy.has_status(StatusEffect::Stun) || enemy.has_status(StatusEffect::Freeze) {
                continue;
            }

            let can_see_player = self.map.visible[enemy.y][enemy.x] && !player_invisible;

            if can_see_player {
                enemy.last_seen_player = Some((self.player.x, self.player.y));
            }

            let target = if can_see_player {
                Some((self.player.x, self.player.y))
            } else {
                enemy.last_seen_player
            };

            if let Some((tx, ty)) = target {
                let dx = tx as i32 - enemy.x as i32;
                let dy = ty as i32 - enemy.y as i32;
                let dist = ((dx * dx + dy * dy) as f32).sqrt();

                if dist < 1.5 && can_see_player {
                    // Attack
                    let mut damage = (enemy.attack - self.player.total_defense()).max(1);

                    // Boss attacks hit harder
                    if enemy.kind.is_boss() {
                        damage = (damage as f32 * 1.5) as i32;
                    }

                    let status = if enemy.kind.can_poison() && self.rng.gen_bool(0.3) {
                        Some(StatusEffect::Poison)
                    } else if enemy.kind.can_burn() && self.rng.gen_bool(0.3) {
                        Some(StatusEffect::Burn)
                    } else if enemy.kind.can_freeze() && self.rng.gen_bool(0.2) {
                        Some(StatusEffect::Freeze)
                    } else if enemy.kind.can_bleed() && self.rng.gen_bool(0.25) {
                        Some(StatusEffect::Bleed)
                    } else {
                        None
                    };

                    attacks.push((idx, damage, status));
                } else if dist < 15.0 {
                    // Move towards target
                    let move_x = dx.signum();
                    let move_y = dy.signum();
                    let new_x = (enemy.x as i32 + move_x).max(0) as usize;
                    let new_y = (enemy.y as i32 + move_y).max(0) as usize;

                    let blocked = enemy_positions.iter().any(|&(ex, ey)| ex == new_x && ey == new_y)
                        || (new_x == self.player.x && new_y == self.player.y);

                    if self.map.is_walkable(new_x, new_y) && !blocked {
                        moves.push((idx, new_x, new_y));
                    }
                }
            }
        }

        // Apply moves
        for (idx, new_x, new_y) in moves {
            self.enemies[idx].x = new_x;
            self.enemies[idx].y = new_y;
        }

        // Apply attacks
        for (idx, damage, status) in attacks {
            if !self.enemies[idx].is_alive() {
                continue;
            }

            // Check for divine shield
            if self.player.has_status(StatusEffect::Shield) {
                self.add_message("Your shield absorbs the attack!".to_string(), Color::Cyan);
                self.player.remove_status(StatusEffect::Shield);
                continue;
            }

            self.player.hp -= damage;
            let enemy_name = self.enemies[idx].kind.name();
            self.add_message(format!("{} hits you for {} damage!", enemy_name, damage), Color::Red);

            if let Some(effect) = status {
                self.player.add_status(effect, 5);
                self.add_message(format!("You are {}!", effect.name().to_lowercase()), effect.color());
            }
        }

        // Remove dead enemies
        self.enemies.retain(|e| e.is_alive());
    }

    fn use_skill(&mut self) {
        if !self.player.can_use_skill() {
            self.add_message("Not enough mana!".to_string(), Color::Red);
            return;
        }

        let skill = self.player.skills[self.player.active_skill];
        self.player.mana -= skill.mana_cost();

        // Update quest progress for skill usage
        self.quest_journal.update_progress(&ObjectiveType::UseSkill(1), 1);

        match skill {
            Skill::Berserk => {
                self.player.add_status(StatusEffect::Strength, 10);
                self.add_message("BERSERK! Your attacks are empowered!".to_string(), Color::Red);
            }
            Skill::Cleave | Skill::Whirlwind => {
                let (px, py) = (self.player.x as i32, self.player.y as i32);
                let atk = self.player.total_attack();
                let mut hits: Vec<(String, i32)> = Vec::new();
                for enemy in &mut self.enemies {
                    let dx = (enemy.x as i32 - px).abs();
                    let dy = (enemy.y as i32 - py).abs();
                    if dx <= 1 && dy <= 1 && enemy.is_alive() {
                        let damage = enemy.take_damage(atk);
                        hits.push((enemy.kind.name().to_string(), damage));
                    }
                }
                for (name, damage) in &hits {
                    self.add_message(format!("Hit {} for {}!", name, damage), Color::White);
                }
                self.add_message(format!("Hit {} enemies!", hits.len()), Color::Yellow);
            }
            Skill::ShieldBash => {
                let (px, py) = (self.player.x as i32, self.player.y as i32);
                let def = self.player.total_defense();
                let mut msg: Option<(String, i32)> = None;
                for enemy in &mut self.enemies {
                    let dx = (enemy.x as i32 - px).abs();
                    let dy = (enemy.y as i32 - py).abs();
                    if dx <= 1 && dy <= 1 && enemy.is_alive() {
                        enemy.add_status(StatusEffect::Stun, 3);
                        let damage = enemy.take_damage(def);
                        msg = Some((enemy.kind.name().to_string(), damage));
                        break;
                    }
                }
                if let Some((name, damage)) = msg {
                    self.add_message(format!("Shield bash! {} stunned for {} damage!", name, damage), Color::Cyan);
                }
            }
            Skill::Fireball => {
                self.cast_aoe_spell(StatusEffect::Burn, 20, 3, "Fireball", Color::Red);
            }
            Skill::IceSpear => {
                self.cast_aoe_spell(StatusEffect::Freeze, 15, 2, "Ice Spear", Color::Cyan);
            }
            Skill::Lightning => {
                // Hit random enemies
                let mut targets: Vec<usize> = self.enemies.iter().enumerate()
                    .filter(|(_, e)| e.is_alive() && self.map.visible[e.y][e.x])
                    .map(|(i, _)| i)
                    .collect();
                targets.shuffle(&mut self.rng);
                for idx in targets.into_iter().take(3) {
                    let damage = self.enemies[idx].take_damage(25);
                    self.add_message(format!("Lightning strikes {} for {}!", self.enemies[idx].kind.name(), damage), Color::Yellow);
                }
            }
            Skill::Teleport => {
                self.teleport_player_random();
                self.add_message("You teleport!".to_string(), Color::Blue);
            }
            Skill::Backstab => {
                let px = self.player.x as i32;
                let py = self.player.y as i32;
                let atk = self.player.total_attack() * 3;
                let target_idx = self.enemies.iter().enumerate()
                    .filter(|(_, e)| e.is_alive())
                    .min_by_key(|(_, e)| {
                        let dx = e.x as i32 - px;
                        let dy = e.y as i32 - py;
                        dx * dx + dy * dy
                    })
                    .map(|(i, _)| i);
                if let Some(idx) = target_idx {
                    let damage = self.enemies[idx].take_damage(atk);
                    let name = self.enemies[idx].kind.name().to_string();
                    self.add_message(format!("Backstab! {} takes {} damage!", name, damage), Color::White);
                }
            }
            Skill::Vanish => {
                self.player.add_status(StatusEffect::Invisibility, 10);
                self.add_message("You vanish into the shadows!".to_string(), Color::Grey);
            }
            Skill::PoisonBlade | Skill::PoisonArrow => {
                let px = self.player.x as i32;
                let py = self.player.y as i32;
                let atk = self.player.total_attack();
                let target_idx = self.enemies.iter().enumerate()
                    .filter(|(_, e)| e.is_alive())
                    .min_by_key(|(_, e)| {
                        let dx = e.x as i32 - px;
                        let dy = e.y as i32 - py;
                        dx * dx + dy * dy
                    })
                    .map(|(i, _)| i);
                if let Some(idx) = target_idx {
                    self.enemies[idx].add_status(StatusEffect::Poison, 10);
                    let damage = self.enemies[idx].take_damage(atk);
                    let name = self.enemies[idx].kind.name().to_string();
                    self.add_message(format!("Poison attack! {} takes {} damage and is poisoned!", name, damage), Color::Green);
                }
            }
            Skill::ShadowStep => {
                // Teleport behind an enemy
                if let Some(enemy) = self.enemies.iter().filter(|e| e.is_alive() && self.map.visible[e.y][e.x]).next() {
                    let dx = enemy.x as i32 - self.player.x as i32;
                    let dy = enemy.y as i32 - self.player.y as i32;
                    let new_x = (enemy.x as i32 - dx.signum()).max(0) as usize;
                    let new_y = (enemy.y as i32 - dy.signum()).max(0) as usize;
                    if self.map.is_walkable(new_x, new_y) {
                        self.player.x = new_x;
                        self.player.y = new_y;
                        self.map.compute_fov(self.player.x, self.player.y);
                        self.add_message("You shadow step!".to_string(), Color::Grey);
                    }
                }
            }
            Skill::HolyLight => {
                self.player.heal(20 + self.player.level as i32 * 2);
                let mut hits: Vec<(String, i32)> = Vec::new();
                for enemy in &mut self.enemies {
                    if enemy.kind.is_undead() && self.map.visible[enemy.y][enemy.x] {
                        let damage = enemy.take_damage(30);
                        hits.push((enemy.kind.name().to_string(), damage));
                    }
                }
                for (name, damage) in hits {
                    self.add_message(format!("Holy light burns {} for {}!", name, damage), Color::Yellow);
                }
                self.add_message("Holy light heals you!".to_string(), Color::Yellow);
            }
            Skill::DivineShield => {
                self.player.add_status(StatusEffect::Shield, 5);
                self.add_message("Divine shield protects you!".to_string(), Color::White);
            }
            Skill::Smite => {
                let atk = self.player.total_attack() * 2;
                let target_idx = self.enemies.iter().enumerate()
                    .filter(|(_, e)| e.is_alive() && self.map.visible[e.y][e.x])
                    .next()
                    .map(|(i, _)| i);
                if let Some(idx) = target_idx {
                    let is_undead = self.enemies[idx].kind.is_undead();
                    let damage = if is_undead { atk * 2 } else { atk };
                    let actual = self.enemies[idx].take_damage(damage);
                    let name = self.enemies[idx].kind.name().to_string();
                    self.add_message(format!("Smite! {} takes {} holy damage!", name, actual), Color::Yellow);
                }
            }
            Skill::Consecrate => {
                self.map.tiles[self.player.y][self.player.x] = Tile::Shrine;
                self.add_message("You consecrate the ground!".to_string(), Color::Magenta);
            }
            Skill::MultiShot => {
                let atk = self.player.total_attack();
                let targets: Vec<usize> = self.enemies.iter().enumerate()
                    .filter(|(_, e)| e.is_alive() && self.map.visible[e.y][e.x])
                    .map(|(i, _)| i)
                    .take(3)
                    .collect();
                let mut hits: Vec<(String, i32)> = Vec::new();
                for idx in targets {
                    let damage = self.enemies[idx].take_damage(atk);
                    hits.push((self.enemies[idx].kind.name().to_string(), damage));
                }
                for (name, damage) in hits {
                    self.add_message(format!("Arrow hits {} for {}!", name, damage), Color::White);
                }
            }
            Skill::TrapSet => {
                self.map.tiles[self.player.y][self.player.x] = Tile::Trap;
                self.add_message("You set a trap!".to_string(), Color::Red);
            }
            Skill::EagleEye => {
                self.map.reveal_all();
                self.add_message("You can see the entire floor!".to_string(), Color::White);
            }
            Skill::RaiseDead => {
                // Summon a skeleton minion
                let (px, py) = (self.player.x, self.player.y);
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let nx = (px as i32 + dx).max(0) as usize;
                        let ny = (py as i32 + dy).max(0) as usize;
                        if self.map.is_walkable(nx, ny) && !(nx == px && ny == py) {
                            let mut minion = Enemy::new(nx, ny, EnemyKind::Skeleton, self.player.level);
                            minion.attack = self.player.total_attack() / 2;
                            self.player.minions.push(minion);
                            self.add_message("You raise a skeleton!".to_string(), Color::Grey);
                            break;
                        }
                    }
                }
            }
            Skill::LifeDrain => {
                let target_idx = self.enemies.iter().enumerate()
                    .filter(|(_, e)| e.is_alive() && self.map.visible[e.y][e.x])
                    .next()
                    .map(|(i, _)| i);
                if let Some(idx) = target_idx {
                    let damage = self.enemies[idx].take_damage(15);
                    let name = self.enemies[idx].kind.name().to_string();
                    self.player.heal(damage);
                    self.add_message(format!("Life drain! {} loses {}, you gain {} HP!", name, damage, damage), Color::Magenta);
                }
            }
            Skill::Curse => {
                for enemy in &mut self.enemies {
                    if self.map.visible[enemy.y][enemy.x] {
                        enemy.add_status(StatusEffect::Weakness, 10);
                    }
                }
                self.add_message("You curse all visible enemies!".to_string(), Color::DarkMagenta);
            }
            Skill::DarkPact => {
                let sacrifice = self.player.hp / 4;
                self.player.hp -= sacrifice;
                self.player.mana = self.player.total_max_mana();
                self.player.add_status(StatusEffect::Strength, 15);
                self.add_message(format!("Dark pact! Sacrificed {} HP for full mana and power!", sacrifice), Color::DarkRed);
            }
            // Default for advanced skills - generic damage effect
            _ => {
                let atk = self.player.total_attack() + 10;
                if let Some(idx) = self.enemies.iter().enumerate()
                    .filter(|(_, e)| e.is_alive() && self.map.visible[e.y][e.x])
                    .next()
                    .map(|(i, _)| i) {
                    let damage = self.enemies[idx].take_damage(atk);
                    let name = self.enemies[idx].kind.name().to_string();
                    self.add_message(format!("{} hits {} for {}!", skill.name(), name, damage), Color::Magenta);
                }
            }
        }

        self.end_turn();
    }

    fn cast_aoe_spell(&mut self, effect: StatusEffect, base_damage: i32, radius: i32, name: &str, color: Color) {
        let (px, py) = (self.player.x as i32, self.player.y as i32);
        let mut hit_count = 0;

        for enemy in &mut self.enemies {
            let dx = enemy.x as i32 - px;
            let dy = enemy.y as i32 - py;
            let dist = ((dx * dx + dy * dy) as f32).sqrt() as i32;

            if dist <= radius && enemy.is_alive() {
                let _damage = enemy.take_damage(base_damage);
                enemy.add_status(effect, 5);
                hit_count += 1;
            }
        }

        self.add_message(format!("{}! Hit {} enemies!", name, hit_count), color);
    }

    fn teleport_player_random(&mut self) {
        if let Some(room) = self.map.rooms.get(self.rng.gen_range(0..self.map.rooms.len())) {
            let (x, y) = room.random_point(&mut self.rng);
            if self.map.is_walkable(x, y) {
                self.player.x = x;
                self.player.y = y;
                self.map.compute_fov(self.player.x, self.player.y);
            }
        }
    }

    fn use_item(&mut self, idx: usize) {
        if idx >= self.player.inventory.len() {
            return;
        }

        let item = self.player.inventory[idx].clone();

        if item.kind.is_food() {
            self.player.eat(item.kind.food_value());
            self.add_message(format!("You eat the {}. Hunger restored!", item.kind.name()), Color::Green);
            self.player.inventory.remove(idx);
            self.end_turn();
            return;
        }

        if item.kind.equip_slot().is_some() {
            if let Some(old) = self.player.equip(item.clone()) {
                self.player.inventory[idx] = old;
                self.add_message(format!("Equipped {}!", item.display_name()), item.rarity.color());
            } else {
                self.player.inventory.remove(idx);
                self.add_message(format!("Equipped {}!", item.display_name()), item.rarity.color());
            }
            return;
        }

        match item.kind {
            ItemKind::HealthPotion => {
                let heal = 30 + self.player.level as i32 * 5;
                self.player.heal(heal);
                self.add_message(format!("Healed {} HP!", heal), Color::Red);
            }
            ItemKind::ManaPotion => {
                let restore = 25 + self.player.level as i32 * 3;
                self.player.restore_mana(restore);
                self.add_message(format!("Restored {} mana!", restore), Color::Blue);
            }
            ItemKind::FullRestorePotion => {
                self.player.hp = self.player.total_max_hp();
                self.player.mana = self.player.total_max_mana();
                self.add_message("Fully restored!".to_string(), Color::Magenta);
            }
            ItemKind::StrengthPotion => {
                self.player.add_status(StatusEffect::Strength, 20);
                self.add_message("You feel stronger!".to_string(), Color::Yellow);
            }
            ItemKind::DefensePotion => {
                self.player.add_status(StatusEffect::Shield, 20);
                self.add_message("You feel protected!".to_string(), Color::Cyan);
            }
            ItemKind::SpeedPotion => {
                self.player.add_status(StatusEffect::Haste, 20);
                self.add_message("You feel faster!".to_string(), Color::Magenta);
            }
            ItemKind::InvisibilityPotion => {
                self.player.add_status(StatusEffect::Invisibility, 15);
                self.add_message("You turn invisible!".to_string(), Color::Grey);
            }
            ItemKind::RegenerationPotion => {
                self.player.add_status(StatusEffect::Regeneration, 30);
                self.add_message("You begin regenerating!".to_string(), Color::Magenta);
            }
            ItemKind::PoisonResistPotion => {
                self.player.remove_status(StatusEffect::Poison);
                self.add_message("Poison cured!".to_string(), Color::Green);
            }
            ItemKind::ScrollTeleport => {
                self.teleport_player_random();
                self.add_message("You teleport!".to_string(), Color::Blue);
            }
            ItemKind::ScrollFireball => {
                self.cast_aoe_spell(StatusEffect::Burn, 30, 4, "Fireball scroll", Color::Red);
            }
            ItemKind::ScrollIceStorm => {
                self.cast_aoe_spell(StatusEffect::Freeze, 25, 5, "Ice storm scroll", Color::Cyan);
            }
            ItemKind::ScrollLightning => {
                for enemy in &mut self.enemies {
                    if self.map.visible[enemy.y][enemy.x] {
                        enemy.take_damage(40);
                    }
                }
                self.add_message("Lightning strikes all visible enemies!".to_string(), Color::Yellow);
            }
            ItemKind::ScrollMapping => {
                self.map.reveal_all();
                self.add_message("The map is revealed!".to_string(), Color::White);
            }
            ItemKind::ScrollMassHeal => {
                self.player.hp = self.player.total_max_hp();
                self.add_message("Mass heal! Fully restored!".to_string(), Color::Red);
            }
            ItemKind::Bomb => {
                self.cast_aoe_spell(StatusEffect::Burn, 50, 3, "Bomb explodes", Color::Red);
            }
            ItemKind::XPPotion => {
                let xp = 100 * self.dungeon_level;
                let (leveled_up, evolved) = self.player.gain_xp(xp);
                if leveled_up {
                    self.add_message(format!("+{} XP! Level up!", xp), Color::Yellow);
                    if let Some(form_name) = evolved {
                        self.add_message(format!("EVOLUTION! You have evolved into a {}!", form_name), Color::Magenta);
                    }
                } else {
                    self.add_message(format!("+{} XP!", xp), Color::Cyan);
                }
            }
            _ => {
                self.add_message("Can't use that item.".to_string(), Color::Grey);
                return;
            }
        }

        self.player.inventory.remove(idx);
        self.end_turn();
    }

    fn descend(&mut self) {
        let tile = self.map.tiles[self.player.y][self.player.x];

        if tile == Tile::BossGate {
            if !self.boss_defeated {
                self.add_message("Defeat the boss to proceed!".to_string(), Color::Red);
                return;
            }
        } else if tile != Tile::StairsDown {
            self.add_message("No stairs here.".to_string(), Color::Grey);
            return;
        }

        self.dungeon_level += 1;
        self.boss_defeated = false;

        if self.dungeon_level > MAX_DUNGEON_LEVEL {
            self.victory = true;
            self.add_message("You have conquered the dungeon! VICTORY!".to_string(), Color::Yellow);
            return;
        }

        self.map.generate(&mut self.rng, self.dungeon_level);
        let (px, py) = self.map.rooms[0].center();
        self.player.x = px;
        self.player.y = py;
        self.spawn_enemies();
        self.spawn_items();
        self.spawn_npcs();
        self.map.compute_fov(self.player.x, self.player.y);

        let theme = DungeonTheme::from_level(self.dungeon_level);
        self.add_message(format!("Descended to {} - Level {}!", theme.name(), self.dungeon_level), Color::Cyan);

        // Update quest progress for floor exploration
        self.quest_journal.update_progress(&ObjectiveType::ReachFloor(self.dungeon_level), 1);
        self.quest_journal.update_progress(&ObjectiveType::ExploreRooms(self.map.rooms.len() as u32), self.map.rooms.len() as u32);

        // Refresh daily/weekly quests
        self.quest_journal.refresh_daily_quests(self.turn_count, self.player.level, self.dungeon_level, &mut self.rng);
        self.quest_journal.refresh_weekly_quests(self.turn_count, self.player.level, self.dungeon_level, &mut self.rng);

        if BOSS_LEVELS.contains(&self.dungeon_level) {
            self.add_message("A powerful boss awaits on this floor!".to_string(), Color::Red);
        }
    }

    fn ascend(&mut self) {
        if self.map.tiles[self.player.y][self.player.x] != Tile::StairsUp {
            self.add_message("No stairs here.".to_string(), Color::Grey);
            return;
        }

        if self.dungeon_level == 1 {
            self.add_message("You can't leave! Defeat the Demon King on level 30!".to_string(), Color::Red);
            return;
        }

        self.dungeon_level -= 1;
        self.map.generate(&mut self.rng, self.dungeon_level);

        if let Some(last_room) = self.map.rooms.last() {
            let (px, py) = last_room.center();
            self.player.x = px;
            self.player.y = py;
        }

        self.spawn_enemies();
        self.spawn_items();
        self.map.compute_fov(self.player.x, self.player.y);
        self.add_message(format!("Returned to level {}.", self.dungeon_level), Color::Cyan);
    }

    fn cycle_skill(&mut self) {
        if !self.player.skills.is_empty() {
            self.player.active_skill = (self.player.active_skill + 1) % self.player.skills.len();
            let skill = self.player.skills[self.player.active_skill];
            self.add_message(format!("Selected skill: {} ({} mana)", skill.name(), skill.mana_cost()), Color::Blue);
        }
    }
}

// ============================================================================
// RENDERING
// ============================================================================

fn render(state: &GameState) -> std::io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, MoveTo(0, 0))?;

    // Render map
    for y in 0..MAP_HEIGHT.min(43) {
        execute!(stdout, MoveTo(0, y as u16))?;

        for x in 0..MAP_WIDTH.min(100) {
            // Player
            if state.player.x == x && state.player.y == y {
                execute!(
                    stdout,
                    SetForegroundColor(Color::Yellow),
                    Print('@'),
                    ResetColor
                )?;
            }
            // Enemies
            else if let Some(enemy) = state.enemies.iter()
                .find(|e| e.x == x && e.y == y && e.is_alive() && state.map.visible[y][x])
            {
                let color = if enemy.kind.is_boss() {
                    Color::Red
                } else {
                    enemy.kind.color()
                };
                execute!(
                    stdout,
                    SetForegroundColor(color),
                    Print(enemy.kind.glyph()),
                    ResetColor
                )?;
            }
            // NPCs (with quest indicator)
            else if let Some(npc) = state.npcs.iter().find(|n| n.x == x && n.y == y && state.map.visible[y][x]) {
                // Check if NPC has quests
                let npc_quests = state.quest_journal.get_quests_for_npc(&npc.name);
                let has_available = npc_quests.iter().any(|q| q.status == QuestStatus::Available);
                let has_ready = npc_quests.iter().any(|q| q.status == QuestStatus::ReadyToComplete);
                let color = if has_ready {
                    Color::Yellow  // Quest ready to turn in
                } else if has_available {
                    Color::Green   // Quest available
                } else {
                    npc.color()
                };
                execute!(stdout, SetForegroundColor(color), Print(npc.glyph()), ResetColor)?;
            }
            // Quest markers
            else if let Some((_, _, icon, color)) = state.quest_journal.get_quest_markers()
                .iter()
                .find(|(mx, my, _, _)| *mx == x && *my == y && state.map.visible[y][x])
            {
                execute!(stdout, SetForegroundColor(*color), Print(*icon), ResetColor)?;
            }
            // Items
            else if let Some(item) = state.items.iter()
                .find(|i| i.x == x && i.y == y && state.map.visible[y][x])
            {
                execute!(
                    stdout,
                    SetForegroundColor(item.rarity.color()),
                    Print(item.kind.glyph()),
                    ResetColor
                )?;
            }
            // Visible tiles
            else if state.map.visible[y][x] {
                let tile = state.map.tiles[y][x];
                let bg = match tile {
                    Tile::Lava => Some(Color::DarkRed),
                    Tile::Water => Some(Color::DarkBlue),
                    Tile::Sand => Some(Color::DarkYellow),
                    _ => None,
                };
                if let Some(bg_color) = bg {
                    execute!(stdout, SetBackgroundColor(bg_color))?;
                }
                execute!(
                    stdout,
                    SetForegroundColor(tile.color()),
                    Print(tile.glyph()),
                    ResetColor
                )?;
            }
            // Explored tiles
            else if state.map.explored[y][x] {
                let tile = state.map.tiles[y][x];
                execute!(
                    stdout,
                    SetForegroundColor(Color::DarkGrey),
                    Print(tile.glyph()),
                    ResetColor
                )?;
            }
            // Unexplored
            else {
                execute!(stdout, Print(' '))?;
            }
        }
    }

    // Stats bar (line 43)
    let stats_y = 43u16;
    execute!(
        stdout,
        MoveTo(0, stats_y),
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::White)
    )?;

    // HP bar
    let hp_pct = (state.player.hp as f32 / state.player.total_max_hp() as f32 * 10.0) as usize;
    let hp_color = if hp_pct <= 2 { Color::Red } else if hp_pct <= 5 { Color::Yellow } else { Color::Green };
    execute!(stdout, SetForegroundColor(hp_color))?;
    write!(stdout, "HP:{}/{}", state.player.hp, state.player.total_max_hp())?;

    // Mana bar
    execute!(stdout, SetForegroundColor(Color::Blue))?;
    write!(stdout, " MP:{}/{}", state.player.mana, state.player.total_max_mana())?;

    // Stats
    execute!(stdout, SetForegroundColor(Color::White))?;
    write!(
        stdout,
        " ATK:{} DEF:{} LV:{} XP:{}/{} Gold:{} Keys:{} ",
        state.player.total_attack(),
        state.player.total_defense(),
        state.player.level,
        state.player.xp,
        state.player.xp_to_level,
        state.player.gold,
        state.player.keys
    )?;

    // Hunger
    let hunger_color = if state.player.hunger < 20 { Color::Red } else if state.player.hunger < 50 { Color::Yellow } else { Color::Green };
    execute!(stdout, SetForegroundColor(hunger_color))?;
    write!(stdout, "Food:{}", state.player.hunger)?;

    // Dungeon level
    execute!(stdout, SetForegroundColor(Color::Cyan))?;
    write!(stdout, " Floor:{}/30", state.dungeon_level)?;

    // Current skill
    if let Some(skill) = state.player.current_skill() {
        execute!(stdout, SetForegroundColor(Color::Magenta))?;
        write!(stdout, " [{}]", skill.name())?;
    }

    execute!(stdout, ResetColor)?;

    // Status effects (line 44)
    execute!(
        stdout,
        MoveTo(0, stats_y + 1),
        Clear(ClearType::CurrentLine)
    )?;

    if !state.player.status_effects.is_empty() {
        write!(stdout, "Status: ")?;
        for (effect, duration) in &state.player.status_effects {
            execute!(stdout, SetForegroundColor(effect.color()))?;
            write!(stdout, "{}({}) ", effect.name(), duration)?;
        }
        execute!(stdout, ResetColor)?;
    }

    // Messages (lines 45-50)
    for (i, (msg, color)) in state.messages.iter().enumerate() {
        execute!(
            stdout,
            MoveTo(0, stats_y + 2 + i as u16),
            Clear(ClearType::CurrentLine),
            SetForegroundColor(*color),
            Print(msg),
            ResetColor
        )?;
    }

    // Clear remaining message lines
    for i in state.messages.len()..6 {
        execute!(
            stdout,
            MoveTo(0, stats_y + 2 + i as u16),
            Clear(ClearType::CurrentLine)
        )?;
    }

    // Controls hint
    execute!(
        stdout,
        MoveTo(0, stats_y + 8),
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::DarkGrey),
        Print("[WASD:Move] [Space:Skill] [Tab:CycleSkill] [I:Inventory] [J:Quests] [>:Descend] [<:Ascend] [?:Help] [Q:Quit]"),
        ResetColor
    )?;

    // Inventory screen
    if state.show_inventory {
        render_inventory(state)?;
    }

    // Help screen
    if state.show_help {
        render_help(state)?;
    }

    // Quest journal screen
    if state.show_quest_journal {
        render_quest_journal(state)?;
    }

    stdout.flush()?;
    Ok(())
}

fn render_inventory(state: &GameState) -> std::io::Result<()> {
    let mut stdout = stdout();

    let start_x = 10;
    let start_y = 5;
    let width = 60;
    let height = 30;

    // Draw box
    for y in start_y..start_y + height {
        execute!(stdout, MoveTo(start_x, y))?;
        for x in 0..width {
            if y == start_y || y == start_y + height - 1 {
                execute!(stdout, SetForegroundColor(Color::White), Print('='))?;
            } else if x == 0 || x == width - 1 {
                execute!(stdout, SetForegroundColor(Color::White), Print('|'))?;
            } else {
                execute!(stdout, Print(' '))?;
            }
        }
    }

    // Title
    execute!(
        stdout,
        MoveTo(start_x + 20, start_y),
        SetForegroundColor(Color::Yellow),
        Print(" INVENTORY "),
        ResetColor
    )?;

    // Equipment
    execute!(
        stdout,
        MoveTo(start_x + 2, start_y + 2),
        SetForegroundColor(Color::Cyan),
        Print("=== EQUIPPED ==="),
        ResetColor
    )?;

    let slots = [
        (EquipSlot::Weapon, "Weapon"),
        (EquipSlot::Shield, "Shield"),
        (EquipSlot::Helmet, "Helmet"),
        (EquipSlot::Armor, "Armor"),
        (EquipSlot::Gloves, "Gloves"),
        (EquipSlot::Boots, "Boots"),
        (EquipSlot::Ring1, "Ring 1"),
        (EquipSlot::Ring2, "Ring 2"),
        (EquipSlot::Amulet, "Amulet"),
    ];

    for (i, (slot, name)) in slots.iter().enumerate() {
        execute!(stdout, MoveTo(start_x + 2, start_y + 3 + i as u16))?;
        if let Some(item) = state.player.equipment.get(slot) {
            let (atk, def, hp, mp) = item.stats();
            execute!(
                stdout,
                SetForegroundColor(item.rarity.color()),
                Print(format!("{}: {} (+{}atk +{}def +{}hp +{}mp)", name, item.display_name(), atk, def, hp, mp)),
                ResetColor
            )?;
        } else {
            execute!(
                stdout,
                SetForegroundColor(Color::DarkGrey),
                Print(format!("{}: (empty)", name)),
                ResetColor
            )?;
        }
    }

    // Inventory items
    execute!(
        stdout,
        MoveTo(start_x + 2, start_y + 13),
        SetForegroundColor(Color::Cyan),
        Print("=== INVENTORY (press 1-9,0 to use) ==="),
        ResetColor
    )?;

    for (i, item) in state.player.inventory.iter().enumerate().take(10) {
        execute!(stdout, MoveTo(start_x + 2, start_y + 14 + i as u16))?;
        let key = if i == 9 { '0' } else { (b'1' + i as u8) as char };
        execute!(
            stdout,
            SetForegroundColor(item.rarity.color()),
            Print(format!("[{}] {}", key, item.display_name())),
            ResetColor
        )?;
    }

    if state.player.inventory.len() > 10 {
        execute!(
            stdout,
            MoveTo(start_x + 2, start_y + 25),
            SetForegroundColor(Color::Grey),
            Print(format!("... and {} more items", state.player.inventory.len() - 10)),
            ResetColor
        )?;
    }

    execute!(
        stdout,
        MoveTo(start_x + 2, start_y + height - 2),
        SetForegroundColor(Color::Yellow),
        Print("Press I or ESC to close"),
        ResetColor
    )?;

    Ok(())
}

fn render_help(_state: &GameState) -> std::io::Result<()> {
    let mut stdout = stdout();

    let start_x = 15;
    let start_y = 3;
    let width = 50;
    let height = 35;

    // Draw box
    for y in start_y..start_y + height {
        execute!(stdout, MoveTo(start_x, y))?;
        for x in 0..width {
            if y == start_y || y == start_y + height - 1 {
                execute!(stdout, SetForegroundColor(Color::White), Print('='))?;
            } else if x == 0 || x == width - 1 {
                execute!(stdout, SetForegroundColor(Color::White), Print('|'))?;
            } else {
                execute!(stdout, Print(' '))?;
            }
        }
    }

    execute!(
        stdout,
        MoveTo(start_x + 18, start_y),
        SetForegroundColor(Color::Yellow),
        Print(" HELP "),
        ResetColor
    )?;

    let help_lines = [
        ("MOVEMENT:", Color::Cyan),
        ("  WASD / Arrow Keys - Move", Color::White),
        ("  HJKL / YUBN - Vim movement + diagonals", Color::White),
        ("", Color::White),
        ("ACTIONS:", Color::Cyan),
        ("  Space - Use active skill", Color::White),
        ("  Tab - Cycle skills", Color::White),
        ("  > or . - Descend stairs", Color::White),
        ("  < or , - Ascend stairs", Color::White),
        ("  I - Open inventory", Color::White),
        ("  1-9,0 - Use inventory item", Color::White),
        ("", Color::White),
        ("TILES:", Color::Cyan),
        ("  @ - You    # - Wall    . - Floor", Color::White),
        ("  > - Stairs down    < - Stairs up", Color::White),
        ("  + - Door    = - Chest    & - Shrine", Color::White),
        ("  ^ - Trap    ~ - Water/Lava    8 - Boss gate", Color::White),
        ("", Color::White),
        ("ENEMIES:", Color::Cyan),
        ("  lowercase = normal enemies", Color::White),
        ("  UPPERCASE = stronger enemies", Color::White),
        ("  Colored & = BOSSES (every 5 floors)", Color::Red),
        ("", Color::White),
        ("ITEMS:", Color::Cyan),
        ("  ! - Potions    ? - Scrolls    $ - Gold", Color::White),
        ("  / | - Weapons    ) - Shields    [ - Armor", Color::White),
        ("  ^ - Helmets    { - Gloves    % - Boots/Food", Color::White),
        ("  o - Rings    \" - Amulets", Color::White),
        ("", Color::White),
        ("GOAL: Descend to floor 30 and defeat the Demon King!", Color::Yellow),
    ];

    for (i, (line, color)) in help_lines.iter().enumerate() {
        execute!(
            stdout,
            MoveTo(start_x + 2, start_y + 2 + i as u16),
            SetForegroundColor(*color),
            Print(*line),
            ResetColor
        )?;
    }

    execute!(
        stdout,
        MoveTo(start_x + 2, start_y + height - 2),
        SetForegroundColor(Color::Yellow),
        Print("Press ? or ESC to close"),
        ResetColor
    )?;

    Ok(())
}

fn render_quest_journal(state: &GameState) -> std::io::Result<()> {
    let mut stdout = stdout();

    let start_x = 5;
    let start_y = 2;
    let width = 90;
    let height = 40;

    // Draw box
    for y in start_y..start_y + height {
        execute!(stdout, MoveTo(start_x, y))?;
        for x in 0..width {
            if y == start_y || y == start_y + height - 1 {
                execute!(stdout, SetForegroundColor(Color::White), Print('='))?;
            } else if x == 0 || x == width - 1 {
                execute!(stdout, SetForegroundColor(Color::White), Print('|'))?;
            } else {
                execute!(stdout, Print(' '))?;
            }
        }
    }

    // Title
    execute!(
        stdout,
        MoveTo(start_x + 35, start_y),
        SetForegroundColor(Color::Yellow),
        Print(" QUEST JOURNAL "),
        ResetColor
    )?;

    // Quest stats
    let (available, active, completed, failed) = state.quest_journal.quest_count_by_status();
    execute!(
        stdout,
        MoveTo(start_x + 2, start_y + 2),
        SetForegroundColor(Color::White),
        Print(format!("Available: {} | Active: {} | Completed: {} | Failed: {}", available, active, completed, failed)),
        ResetColor
    )?;

    // Active Quests Section
    execute!(
        stdout,
        MoveTo(start_x + 2, start_y + 4),
        SetForegroundColor(Color::Cyan),
        Print("=== ACTIVE QUESTS ==="),
        ResetColor
    )?;

    let active_quests = state.quest_journal.get_active_quests();
    let mut line_offset = 5;

    if active_quests.is_empty() {
        execute!(
            stdout,
            MoveTo(start_x + 4, start_y + line_offset as u16),
            SetForegroundColor(Color::DarkGrey),
            Print("No active quests. Talk to NPCs to find quests!"),
            ResetColor
        )?;
        line_offset += 1;
    } else {
        for (i, quest) in active_quests.iter().enumerate().take(8) {
            execute!(
                stdout,
                MoveTo(start_x + 4, start_y + line_offset as u16),
                SetForegroundColor(quest.quest_type.color()),
                Print(format!("[{}] ", quest.quest_type.icon())),
                SetForegroundColor(quest.difficulty.color()),
                Print(format!("{} ", quest.name)),
                SetForegroundColor(Color::Grey),
                Print(format!("({})", quest.difficulty.name())),
                ResetColor
            )?;
            line_offset += 1;

            // Show objectives
            for obj in &quest.objectives {
                let status_color = if obj.completed { Color::Green } else { Color::Yellow };
                let status_char = if obj.completed { '[X]' } else { '[ ]' };
                execute!(
                    stdout,
                    MoveTo(start_x + 6, start_y + line_offset as u16),
                    SetForegroundColor(status_color),
                    Print(format!("{} {} - {}", status_char, obj.description(), obj.progress_string())),
                    ResetColor
                )?;
                line_offset += 1;
            }

            // Show deadline if exists
            if let Some(remaining) = quest.remaining_turns(state.turn_count) {
                let time_color = if remaining < 50 { Color::Red } else if remaining < 150 { Color::Yellow } else { Color::Grey };
                execute!(
                    stdout,
                    MoveTo(start_x + 6, start_y + line_offset as u16),
                    SetForegroundColor(time_color),
                    Print(format!("Time remaining: {} turns", remaining)),
                    ResetColor
                )?;
                line_offset += 1;
            }

            // Show rewards
            execute!(
                stdout,
                MoveTo(start_x + 6, start_y + line_offset as u16),
                SetForegroundColor(Color::Magenta),
                Print(format!("Rewards: {}", quest.rewards.description())),
                ResetColor
            )?;
            line_offset += 2;

            if i >= 3 && active_quests.len() > 4 {
                execute!(
                    stdout,
                    MoveTo(start_x + 4, start_y + line_offset as u16),
                    SetForegroundColor(Color::Grey),
                    Print(format!("... and {} more active quests", active_quests.len() - 4)),
                    ResetColor
                )?;
                break;
            }
        }
    }

    // Available Quests Section
    line_offset = 22;
    execute!(
        stdout,
        MoveTo(start_x + 2, start_y + line_offset as u16),
        SetForegroundColor(Color::Green),
        Print("=== AVAILABLE QUESTS ==="),
        ResetColor
    )?;
    line_offset += 1;

    let available_quests = state.quest_journal.get_available_quests();
    if available_quests.is_empty() {
        execute!(
            stdout,
            MoveTo(start_x + 4, start_y + line_offset as u16),
            SetForegroundColor(Color::DarkGrey),
            Print("No available quests at the moment."),
            ResetColor
        )?;
    } else {
        for (i, quest) in available_quests.iter().enumerate().take(6) {
            let key = (b'1' + i as u8) as char;
            execute!(
                stdout,
                MoveTo(start_x + 4, start_y + line_offset as u16),
                SetForegroundColor(Color::White),
                Print(format!("[{}] ", key)),
                SetForegroundColor(quest.quest_type.color()),
                Print(format!("[{}] ", quest.quest_type.icon())),
                SetForegroundColor(quest.difficulty.color()),
                Print(format!("{} ", quest.name)),
                SetForegroundColor(Color::Grey),
                Print(format!("({}) - {}", quest.difficulty.name(), quest.description.chars().take(35).collect::<String>())),
                ResetColor
            )?;
            line_offset += 1;
        }
        if available_quests.len() > 6 {
            execute!(
                stdout,
                MoveTo(start_x + 4, start_y + line_offset as u16),
                SetForegroundColor(Color::Grey),
                Print(format!("... and {} more available quests", available_quests.len() - 6)),
                ResetColor
            )?;
        }
    }

    // Controls
    execute!(
        stdout,
        MoveTo(start_x + 2, start_y + height - 4),
        SetForegroundColor(Color::Yellow),
        Print("[1-6] Accept quest | [C] Complete ready quests | [T] Toggle completed"),
        ResetColor
    )?;

    execute!(
        stdout,
        MoveTo(start_x + 2, start_y + height - 2),
        SetForegroundColor(Color::Yellow),
        Print("Press J or ESC to close"),
        ResetColor
    )?;

    Ok(())
}

fn render_class_select() -> std::io::Result<()> {
    let mut stdout = stdout();

    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

    let title = r#"
   ███████╗██╗  ██╗ █████╗ ██████╗  ██████╗ ██╗    ██╗ ██████╗██████╗ ██╗   ██╗██████╗ ████████╗
   ██╔════╝██║  ██║██╔══██╗██╔══██╗██╔═══██╗██║    ██║██╔════╝██╔══██╗╚██╗ ██╔╝██╔══██╗╚══██╔══╝
   ███████╗███████║███████║██║  ██║██║   ██║██║ █╗ ██║██║     ██████╔╝ ╚████╔╝ ██████╔╝   ██║
   ╚════██║██╔══██║██╔══██║██║  ██║██║   ██║██║███╗██║██║     ██╔══██╗  ╚██╔╝  ██╔═══╝    ██║
   ███████║██║  ██║██║  ██║██████╔╝╚██████╔╝╚███╔███╔╝╚██████╗██║  ██║   ██║   ██║        ██║
   ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═════╝  ╚═════╝  ╚══╝╚══╝  ╚═════╝╚═╝  ╚═╝   ╚═╝   ╚═╝        ╚═╝
    "#;

    execute!(stdout, SetForegroundColor(Color::Red))?;
    for (i, line) in title.lines().enumerate() {
        execute!(stdout, MoveTo(5, i as u16), Print(line))?;
    }

    execute!(
        stdout,
        MoveTo(30, 10),
        SetForegroundColor(Color::Yellow),
        Print("=== SELECT YOUR CLASS ==="),
        ResetColor
    )?;

    let classes = [
        (CharacterClass::Warrior, Color::Red),
        (CharacterClass::Mage, Color::Blue),
        (CharacterClass::Rogue, Color::Grey),
        (CharacterClass::Paladin, Color::Yellow),
        (CharacterClass::Ranger, Color::Green),
        (CharacterClass::Necromancer, Color::Magenta),
    ];

    for (i, (class, color)) in classes.iter().enumerate() {
        let (hp, atk, def, mana, spd) = class.base_stats();
        execute!(
            stdout,
            MoveTo(15, 13 + i as u16 * 3),
            SetForegroundColor(*color),
            Print(format!("[{}] {} - HP:{} ATK:{} DEF:{} MANA:{} SPD:{}",
                i + 1, class.name(), hp, atk, def, mana, spd)),
            ResetColor
        )?;
        execute!(
            stdout,
            MoveTo(20, 14 + i as u16 * 3),
            SetForegroundColor(Color::Grey),
            Print(format!("Special: {}", class.special_ability())),
            ResetColor
        )?;
    }

    execute!(
        stdout,
        MoveTo(25, 35),
        SetForegroundColor(Color::Cyan),
        Print("Press 1-6 to select your class, or Q to quit"),
        ResetColor
    )?;

    stdout.flush()?;
    Ok(())
}

fn render_dwarf_select() -> std::io::Result<()> {
    let mut stdout = stdout();

    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

    execute!(
        stdout,
        MoveTo(25, 2),
        SetForegroundColor(Color::Yellow),
        Print("=== SELECT YOUR DWARF SUBSPECIES ==="),
        ResetColor
    )?;

    execute!(
        stdout,
        MoveTo(10, 4),
        SetForegroundColor(Color::Grey),
        Print("(Press 0 to skip dwarf subspecies selection)"),
        ResetColor
    )?;

    let subspecies = [
        (DwarfSubspecies::MountainDwarf, "+25 HP, +20 ATK, +20 DEF, -5 SPD"),
        (DwarfSubspecies::DeepDwarf, "+20 HP, +10 ATK, +15 DEF, +5 MANA"),
        (DwarfSubspecies::GoldDwarf, "+15 HP, +10 ATK, +10 DEF, +5 SPD, +10 MANA"),
        (DwarfSubspecies::IronDwarf, "+30 HP, +15 ATK, +30 DEF, -10 SPD"),
        (DwarfSubspecies::RuneDwarf, "+10 HP, +5 ATK, +10 DEF, +25 MANA"),
        (DwarfSubspecies::FrostDwarf, "+20 HP, +15 ATK, +20 DEF, +5 MANA"),
        (DwarfSubspecies::FireDwarf, "+15 HP, +25 ATK, +15 DEF, +5 MANA"),
        (DwarfSubspecies::HillDwarf, "+40 HP, +10 ATK, +15 DEF, +5 SPD"),
    ];

    for (i, (sub, stats)) in subspecies.iter().enumerate() {
        execute!(
            stdout,
            MoveTo(15, 6 + i as u16 * 3),
            SetForegroundColor(Color::DarkYellow),
            Print(format!("[{}] {} - {}", i + 1, sub.name(), stats)),
            ResetColor
        )?;
        execute!(
            stdout,
            MoveTo(20, 7 + i as u16 * 3),
            SetForegroundColor(Color::Grey),
            Print(format!("Special: {}", sub.special_ability())),
            ResetColor
        )?;
    }

    execute!(
        stdout,
        MoveTo(20, 32),
        SetForegroundColor(Color::Cyan),
        Print("Press 1-8 to select, 0 to skip"),
        ResetColor
    )?;

    stdout.flush()?;
    Ok(())
}

// ============================================================================
// AI AUTO-PLAY
// ============================================================================

#[derive(Clone, Copy, Debug)]
enum AIAction {
    Move(i32, i32),
    UseSkill,
    UseItem(usize),
    Descend,
    Ascend,
    Wait,
}

impl GameState {
    fn ai_decide(&self) -> AIAction {
        let px = self.player.x as i32;
        let py = self.player.y as i32;

        // Priority 1: Use health potion if HP is critical (below 30%)
        if self.player.hp < self.player.total_max_hp() * 30 / 100 {
            for (i, item) in self.player.inventory.iter().enumerate() {
                if matches!(item.kind, ItemKind::HealthPotion | ItemKind::FullRestorePotion) {
                    return AIAction::UseItem(i);
                }
            }
        }

        // Priority 2: Eat food if starving
        if self.player.hunger < 15 {
            for (i, item) in self.player.inventory.iter().enumerate() {
                if item.kind.is_food() {
                    return AIAction::UseItem(i);
                }
            }
        }

        // Priority 3: Attack adjacent enemy
        let directions = [
            (0, -1), (0, 1), (-1, 0), (1, 0),
            (-1, -1), (1, -1), (-1, 1), (1, 1),
        ];

        for (dx, dy) in directions.iter() {
            let nx = (px + dx) as usize;
            let ny = (py + dy) as usize;
            if self.enemies.iter().any(|e| e.x == nx && e.y == ny && e.is_alive()) {
                return AIAction::Move(*dx, *dy);
            }
        }

        // Priority 4: Use skill on nearby enemies if we have mana
        if self.player.can_use_skill() {
            let skill_range = 5;
            let has_nearby_enemy = self.enemies.iter().any(|e| {
                let dx = (e.x as i32 - px).abs();
                let dy = (e.y as i32 - py).abs();
                e.is_alive() && dx <= skill_range && dy <= skill_range && self.map.visible[e.y][e.x]
            });
            if has_nearby_enemy {
                return AIAction::UseSkill;
            }
        }

        // Priority 5: Move towards visible enemy
        if let Some(target) = self.enemies.iter()
            .filter(|e| e.is_alive() && self.map.visible[e.y][e.x])
            .min_by_key(|e| {
                let dx = e.x as i32 - px;
                let dy = e.y as i32 - py;
                dx * dx + dy * dy
            })
        {
            let dx = (target.x as i32 - px).signum();
            let dy = (target.y as i32 - py).signum();
            let nx = (px + dx) as usize;
            let ny = (py + dy) as usize;
            if self.map.is_walkable(nx, ny) {
                return AIAction::Move(dx, dy);
            }
            // Try horizontal or vertical if diagonal blocked
            if dx != 0 && self.map.is_walkable((px + dx) as usize, py as usize) {
                return AIAction::Move(dx, 0);
            }
            if dy != 0 && self.map.is_walkable(px as usize, (py + dy) as usize) {
                return AIAction::Move(0, dy);
            }
        }

        // Priority 6: Descend stairs if on them and boss defeated (or no boss)
        let current_tile = self.map.tiles[self.player.y][self.player.x];
        if current_tile == Tile::StairsDown && (self.boss_defeated || !BOSS_LEVELS.contains(&self.dungeon_level)) {
            return AIAction::Descend;
        }

        // Priority 7: Move towards stairs if visible and no enemies around
        let no_visible_enemies = !self.enemies.iter().any(|e| e.is_alive() && self.map.visible[e.y][e.x]);
        if no_visible_enemies {
            // Find stairs
            for y in 0..MAP_HEIGHT {
                for x in 0..MAP_WIDTH {
                    if self.map.tiles[y][x] == Tile::StairsDown && self.map.explored[y][x] {
                        let dx = (x as i32 - px).signum();
                        let dy = (y as i32 - py).signum();
                        if dx != 0 || dy != 0 {
                            let nx = (px + dx) as usize;
                            let ny = (py + dy) as usize;
                            if self.map.is_walkable(nx, ny) {
                                return AIAction::Move(dx, dy);
                            }
                            if dx != 0 && self.map.is_walkable((px + dx) as usize, py as usize) {
                                return AIAction::Move(dx, 0);
                            }
                            if dy != 0 && self.map.is_walkable(px as usize, (py + dy) as usize) {
                                return AIAction::Move(0, dy);
                            }
                        }
                    }
                }
            }
        }

        // Priority 8: Explore unexplored areas - move towards nearest unexplored visible tile
        let mut best_unexplored: Option<(usize, usize, i32)> = None;
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if self.map.visible[y][x] && !self.map.explored[y][x] && self.map.is_walkable(x, y) {
                    let dist = (x as i32 - px).abs() + (y as i32 - py).abs();
                    if best_unexplored.is_none() || dist < best_unexplored.unwrap().2 {
                        best_unexplored = Some((x, y, dist));
                    }
                }
            }
        }

        if let Some((tx, ty, _)) = best_unexplored {
            let dx = (tx as i32 - px).signum();
            let dy = (ty as i32 - py).signum();
            let nx = (px + dx) as usize;
            let ny = (py + dy) as usize;
            if self.map.is_walkable(nx, ny) {
                return AIAction::Move(dx, dy);
            }
        }

        // Priority 9: Random walk to explore
        let mut rng = thread_rng();
        let shuffled: Vec<(i32, i32)> = {
            let mut dirs = directions.to_vec();
            dirs.shuffle(&mut rng);
            dirs
        };

        for (dx, dy) in shuffled {
            let nx = (px + dx) as usize;
            let ny = (py + dy) as usize;
            if self.map.is_walkable(nx, ny) {
                return AIAction::Move(dx, dy);
            }
        }

        AIAction::Wait
    }

    fn ai_execute(&mut self, action: AIAction) {
        match action {
            AIAction::Move(dx, dy) => self.move_player(dx, dy),
            AIAction::UseSkill => self.use_skill(),
            AIAction::UseItem(idx) => self.use_item(idx),
            AIAction::Descend => self.descend(),
            AIAction::Ascend => self.ascend(),
            AIAction::Wait => self.end_turn(),
        }
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let auto_play = args.iter().any(|a| a == "--auto" || a == "-a");
    let auto_speed: u64 = args.iter()
        .position(|a| a == "--speed" || a == "-s")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(100); // Default 100ms per turn

    let mut stdout = stdout();

    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, Hide)?;

    // Class selection (auto-pick random in auto mode)
    let selected_class = if auto_play {
        let classes = [
            CharacterClass::Warrior,
            CharacterClass::Mage,
            CharacterClass::Rogue,
            CharacterClass::Paladin,
            CharacterClass::Ranger,
            CharacterClass::Necromancer,
        ];
        classes[thread_rng().gen_range(0..classes.len())]
    } else {
        loop {
            render_class_select()?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                    match code {
                        KeyCode::Char('1') => break CharacterClass::Warrior,
                        KeyCode::Char('2') => break CharacterClass::Mage,
                        KeyCode::Char('3') => break CharacterClass::Rogue,
                        KeyCode::Char('4') => break CharacterClass::Paladin,
                        KeyCode::Char('5') => break CharacterClass::Ranger,
                        KeyCode::Char('6') => break CharacterClass::Necromancer,
                        KeyCode::Char('q') | KeyCode::Esc => {
                            execute!(stdout, Show, LeaveAlternateScreen)?;
                            terminal::disable_raw_mode()?;
                            return Ok(());
                        }
                        _ => {}
                    }
                }
            }
        }
    };

    // Dwarf subspecies selection (auto-pick random in auto mode, or None)
    let selected_dwarf = if auto_play {
        // Random chance to be a dwarf in auto mode
        if thread_rng().gen_bool(0.25) {
            let subspecies = [
                DwarfSubspecies::MountainDwarf,
                DwarfSubspecies::DeepDwarf,
                DwarfSubspecies::GoldDwarf,
                DwarfSubspecies::IronDwarf,
                DwarfSubspecies::RuneDwarf,
                DwarfSubspecies::FrostDwarf,
                DwarfSubspecies::FireDwarf,
                DwarfSubspecies::HillDwarf,
            ];
            Some(subspecies[thread_rng().gen_range(0..subspecies.len())])
        } else {
            None
        }
    } else {
        loop {
            render_dwarf_select()?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                    match code {
                        KeyCode::Char('0') => break None,
                        KeyCode::Char('1') => break Some(DwarfSubspecies::MountainDwarf),
                        KeyCode::Char('2') => break Some(DwarfSubspecies::DeepDwarf),
                        KeyCode::Char('3') => break Some(DwarfSubspecies::GoldDwarf),
                        KeyCode::Char('4') => break Some(DwarfSubspecies::IronDwarf),
                        KeyCode::Char('5') => break Some(DwarfSubspecies::RuneDwarf),
                        KeyCode::Char('6') => break Some(DwarfSubspecies::FrostDwarf),
                        KeyCode::Char('7') => break Some(DwarfSubspecies::FireDwarf),
                        KeyCode::Char('8') => break Some(DwarfSubspecies::HillDwarf),
                        KeyCode::Char('q') | KeyCode::Esc => {
                            execute!(stdout, Show, LeaveAlternateScreen)?;
                            terminal::disable_raw_mode()?;
                            return Ok(());
                        }
                        _ => {}
                    }
                }
            }
        }
    };

    let mut state = GameState::new(selected_class, selected_dwarf);

    // Game loop
    loop {
        render(&state)?;

        if state.game_over || state.victory {
            if auto_play {
                // In auto mode, wait a bit then exit
                std::thread::sleep(Duration::from_millis(2000));
                break;
            }
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(_) = event::read()? {
                    break;
                }
            }
            continue;
        }

        // Auto-play mode
        if auto_play {
            // Check for 'q' to quit
            if event::poll(Duration::from_millis(10))? {
                if let Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) = event::read()? {
                    break;
                }
            }

            // AI makes decision
            let action = state.ai_decide();
            state.ai_execute(action);
            std::thread::sleep(Duration::from_millis(auto_speed));
            continue;
        }

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                // Inventory mode
                if state.show_inventory {
                    match code {
                        KeyCode::Char('i') | KeyCode::Esc => state.show_inventory = false,
                        KeyCode::Char(c) if c.is_ascii_digit() => {
                            let idx = if c == '0' { 9 } else { (c as u8 - b'1') as usize };
                            state.use_item(idx);
                        }
                        _ => {}
                    }
                    continue;
                }

                // Help mode
                if state.show_help {
                    match code {
                        KeyCode::Char('?') | KeyCode::Esc => state.show_help = false,
                        _ => {}
                    }
                    continue;
                }

                // Quest journal mode
                if state.show_quest_journal {
                    match code {
                        KeyCode::Char('j') | KeyCode::Esc => state.show_quest_journal = false,
                        KeyCode::Char(c) if c.is_ascii_digit() && c != '0' => {
                            let idx = (c as u8 - b'1') as usize;
                            let available_ids: Vec<u32> = state.quest_journal.available_quests.clone();
                            if idx < available_ids.len() {
                                let quest_id = available_ids[idx];
                                if state.quest_journal.accept_quest(quest_id, state.turn_count) {
                                    if let Some(quest) = state.quest_journal.quests.get(&quest_id) {
                                        state.add_message(format!("Quest accepted: {}", quest.name), Color::Green);
                                    }
                                }
                            }
                        }
                        KeyCode::Char('c') => {
                            // Complete ready quests
                            let ready_quests: Vec<u32> = state.quest_journal.active_quests.iter()
                                .filter(|&id| {
                                    state.quest_journal.quests.get(id)
                                        .map(|q| q.status == QuestStatus::ReadyToComplete)
                                        .unwrap_or(false)
                                })
                                .copied()
                                .collect();
                            for quest_id in ready_quests {
                                if let Some(reward) = state.quest_journal.complete_quest(quest_id) {
                                    state.player.xp += reward.xp;
                                    state.player.gold += reward.gold;
                                    if let Some((hp, atk, def, mana, _spd)) = reward.stat_bonuses {
                                        state.player.max_hp += hp;
                                        state.player.hp = (state.player.hp + hp).min(state.player.max_hp);
                                        state.player.base_attack += atk;
                                        state.player.base_defense += def;
                                        state.player.max_mana += mana;
                                    }
                                    state.add_message(format!("Quest completed! +{} XP, +{} Gold", reward.xp, reward.gold), Color::Yellow);
                                }
                            }
                        }
                        KeyCode::Char('t') => {
                            state.quest_journal.show_completed = !state.quest_journal.show_completed;
                        }
                        _ => {}
                    }
                    continue;
                }

                // Normal mode
                match code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => break,

                    // Movement
                    KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k') => state.move_player(0, -1),
                    KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j') => state.move_player(0, 1),
                    KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('h') => state.move_player(-1, 0),
                    KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('l') => state.move_player(1, 0),

                    // Diagonal
                    KeyCode::Char('y') => state.move_player(-1, -1),
                    KeyCode::Char('u') => state.move_player(1, -1),
                    KeyCode::Char('b') => state.move_player(-1, 1),
                    KeyCode::Char('n') => state.move_player(1, 1),

                    // Skills
                    KeyCode::Char(' ') => state.use_skill(),
                    KeyCode::Tab => state.cycle_skill(),

                    // Stairs
                    KeyCode::Char('>') | KeyCode::Char('.') => state.descend(),
                    KeyCode::Char('<') | KeyCode::Char(',') => state.ascend(),

                    // Inventory
                    KeyCode::Char('i') => state.show_inventory = true,
                    KeyCode::Char(c) if c.is_ascii_digit() => {
                        let idx = if c == '0' { 9 } else { (c as u8 - b'1') as usize };
                        state.use_item(idx);
                    }

                    // Help
                    KeyCode::Char('?') => state.show_help = true,

                    // Quest journal
                    KeyCode::Char('j') => state.show_quest_journal = true,

                    _ => {}
                }
            }
        }
    }

    // Cleanup
    execute!(stdout, Show, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    // Final stats
    println!();
    if state.victory {
        println!("*** CONGRATULATIONS! YOU HAVE CONQUERED SHADOWCRYPT! ***");
        println!();
        println!("Class: {}", state.player.class.name());
        println!("Final Level: {}", state.player.level);
        println!("Gold Collected: {}", state.player.gold);
        println!("Enemies Slain: {}", state.player.kills);
        println!("Turns Taken: {}", state.turn_count);
        println!("Floors Explored: {}", state.dungeon_level);
    } else if state.game_over {
        println!("*** GAME OVER ***");
        println!();
        println!("Class: {}", state.player.class.name());
        println!("Died on floor {} after {} turns.", state.dungeon_level, state.turn_count);
        println!("Level: {} | Gold: {} | Kills: {}", state.player.level, state.player.gold, state.player.kills);
    } else {
        println!("Thanks for playing ShadowCrypt!");
    }

    Ok(())
}
