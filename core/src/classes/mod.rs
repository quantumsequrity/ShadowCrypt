//! Character classes, specializations, and their properties
//!
//! Classes can specialize at level 20 (Tier 1) and further at level 50 (Tier 2).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Constants
// ============================================================================

pub const SPECIALIZATION_TIER1_LEVEL: u32 = 20;
pub const SPECIALIZATION_TIER2_LEVEL: u32 = 50;

// ============================================================================
// Base Character Classes
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum CharacterClass {
    Warrior,
    Mage,
    Rogue,
    Cleric,
    Ranger,
    Monk,
}

impl CharacterClass {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Warrior => "Warrior",
            Self::Mage => "Mage",
            Self::Rogue => "Rogue",
            Self::Cleric => "Cleric",
            Self::Ranger => "Ranger",
            Self::Monk => "Monk",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Warrior => "Masters of martial combat, warriors excel in physical strength and endurance.",
            Self::Mage => "Wielders of arcane power, mages command devastating magical forces.",
            Self::Rogue => "Swift and cunning, rogues strike from shadows with deadly precision.",
            Self::Cleric => "Divine servants who channel holy power to heal allies and smite foes.",
            Self::Ranger => "Skilled trackers and marksmen who thrive in the wilderness.",
            Self::Monk => "Disciplined martial artists who harness inner energy for combat.",
        }
    }

    /// Returns base stats: (hp, attack, defense, mana, speed, crit_chance)
    pub fn base_stats(&self) -> ClassStats {
        match self {
            Self::Warrior => ClassStats::new(120, 15, 12, 20, 8, 5.0),
            Self::Mage => ClassStats::new(70, 8, 5, 150, 7, 10.0),
            Self::Rogue => ClassStats::new(85, 12, 7, 50, 15, 15.0),
            Self::Cleric => ClassStats::new(95, 10, 10, 120, 6, 5.0),
            Self::Ranger => ClassStats::new(90, 14, 8, 60, 12, 12.0),
            Self::Monk => ClassStats::new(100, 13, 9, 80, 14, 8.0),
        }
    }

    pub fn primary_attribute(&self) -> Attribute {
        match self {
            Self::Warrior => Attribute::Strength,
            Self::Mage => Attribute::Intelligence,
            Self::Rogue => Attribute::Agility,
            Self::Cleric => Attribute::Wisdom,
            Self::Ranger => Attribute::Agility,
            Self::Monk => Attribute::Dexterity,
        }
    }

    pub fn available_specializations(&self) -> Vec<Specialization> {
        match self {
            Self::Warrior => vec![
                Specialization::Berserker,
                Specialization::Guardian,
                Specialization::Weaponmaster,
                Specialization::Warlord,
            ],
            Self::Mage => vec![
                Specialization::Elementalist,
                Specialization::Archmage,
                Specialization::Battlemage,
                Specialization::Necromancer,
                Specialization::Chronomancer,
            ],
            Self::Rogue => vec![
                Specialization::Assassin,
                Specialization::Shadowdancer,
                Specialization::Trickster,
                Specialization::Duelist,
            ],
            Self::Cleric => vec![
                Specialization::Healer,
                Specialization::Crusader,
                Specialization::Inquisitor,
                Specialization::Oracle,
            ],
            Self::Ranger => vec![
                Specialization::Hunter,
                Specialization::Beastmaster,
                Specialization::Sniper,
                Specialization::Warden,
            ],
            Self::Monk => vec![
                Specialization::MartialArtist,
                Specialization::SpiritWalker,
                Specialization::IronBody,
                Specialization::WayOfShadows,
            ],
        }
    }

    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::Warrior,
            Self::Mage,
            Self::Rogue,
            Self::Cleric,
            Self::Ranger,
            Self::Monk,
        ]
        .into_iter()
    }
}

// ============================================================================
// Class Stats
// ============================================================================

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ClassStats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub mana: i32,
    pub speed: i32,
    pub crit_chance: f32,
}

impl ClassStats {
    pub fn new(hp: i32, attack: i32, defense: i32, mana: i32, speed: i32, crit_chance: f32) -> Self {
        Self { hp, attack, defense, mana, speed, crit_chance }
    }

    pub fn apply_modifiers(&self, modifiers: &StatModifiers) -> Self {
        Self {
            hp: ((self.hp as f32) * modifiers.hp_mult) as i32 + modifiers.hp_flat,
            attack: ((self.attack as f32) * modifiers.attack_mult) as i32 + modifiers.attack_flat,
            defense: ((self.defense as f32) * modifiers.defense_mult) as i32 + modifiers.defense_flat,
            mana: ((self.mana as f32) * modifiers.mana_mult) as i32 + modifiers.mana_flat,
            speed: ((self.speed as f32) * modifiers.speed_mult) as i32 + modifiers.speed_flat,
            crit_chance: self.crit_chance * modifiers.crit_mult + modifiers.crit_flat,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct StatModifiers {
    pub hp_mult: f32,
    pub hp_flat: i32,
    pub attack_mult: f32,
    pub attack_flat: i32,
    pub defense_mult: f32,
    pub defense_flat: i32,
    pub mana_mult: f32,
    pub mana_flat: i32,
    pub speed_mult: f32,
    pub speed_flat: i32,
    pub crit_mult: f32,
    pub crit_flat: f32,
}

impl Default for StatModifiers {
    fn default() -> Self {
        Self {
            hp_mult: 1.0, hp_flat: 0,
            attack_mult: 1.0, attack_flat: 0,
            defense_mult: 1.0, defense_flat: 0,
            mana_mult: 1.0, mana_flat: 0,
            speed_mult: 1.0, speed_flat: 0,
            crit_mult: 1.0, crit_flat: 0.0,
        }
    }
}

// ============================================================================
// Attributes
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum Attribute {
    Strength,
    Agility,
    Intelligence,
    Wisdom,
    Dexterity,
    Constitution,
    Charisma,
}

impl Attribute {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Strength => "Strength",
            Self::Agility => "Agility",
            Self::Intelligence => "Intelligence",
            Self::Wisdom => "Wisdom",
            Self::Dexterity => "Dexterity",
            Self::Constitution => "Constitution",
            Self::Charisma => "Charisma",
        }
    }
}

// ============================================================================
// Specializations
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum Specialization {
    // Warrior Specializations
    Berserker,
    Guardian,
    Weaponmaster,
    Warlord,
    // Mage Specializations
    Elementalist,
    Archmage,
    Battlemage,
    Necromancer,
    Chronomancer,
    // Rogue Specializations
    Assassin,
    Shadowdancer,
    Trickster,
    Duelist,
    // Cleric Specializations
    Healer,
    Crusader,
    Inquisitor,
    Oracle,
    // Ranger Specializations
    Hunter,
    Beastmaster,
    Sniper,
    Warden,
    // Monk Specializations
    MartialArtist,
    SpiritWalker,
    IronBody,
    WayOfShadows,
}

impl Specialization {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Berserker => "Berserker",
            Self::Guardian => "Guardian",
            Self::Weaponmaster => "Weaponmaster",
            Self::Warlord => "Warlord",
            Self::Elementalist => "Elementalist",
            Self::Archmage => "Archmage",
            Self::Battlemage => "Battlemage",
            Self::Necromancer => "Necromancer",
            Self::Chronomancer => "Chronomancer",
            Self::Assassin => "Assassin",
            Self::Shadowdancer => "Shadowdancer",
            Self::Trickster => "Trickster",
            Self::Duelist => "Duelist",
            Self::Healer => "Healer",
            Self::Crusader => "Crusader",
            Self::Inquisitor => "Inquisitor",
            Self::Oracle => "Oracle",
            Self::Hunter => "Hunter",
            Self::Beastmaster => "Beastmaster",
            Self::Sniper => "Sniper",
            Self::Warden => "Warden",
            Self::MartialArtist => "Martial Artist",
            Self::SpiritWalker => "Spirit Walker",
            Self::IronBody => "Iron Body",
            Self::WayOfShadows => "Way of Shadows",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Berserker => "Unleash primal rage for devastating damage at the cost of defense.",
            Self::Guardian => "An unbreakable wall protecting allies with shields and defensive arts.",
            Self::Weaponmaster => "Master of all weapons, adapting fighting style to any situation.",
            Self::Warlord => "Inspiring leader who strengthens allies through tactical commands.",
            Self::Elementalist => "Channel the raw power of fire, ice, and lightning.",
            Self::Archmage => "Pure arcane mastery delivering overwhelming magical destruction.",
            Self::Battlemage => "Blend steel and sorcery in deadly melee combat.",
            Self::Necromancer => "Command the forces of death and raise undead servants.",
            Self::Chronomancer => "Manipulate the flow of time itself.",
            Self::Assassin => "Silent killer specializing in poisons and instant death strikes.",
            Self::Shadowdancer => "Move through shadows with supernatural agility and grace.",
            Self::Trickster => "Master of traps, illusions, and misdirection.",
            Self::Duelist => "Precise fencer excelling in one-on-one combat with deadly counters.",
            Self::Healer => "Supreme restorative power keeping allies alive against all odds.",
            Self::Crusader => "Holy warrior smiting evil with divine wrath and blessed steel.",
            Self::Inquisitor => "Hunter of heretics with anti-magic abilities and demon-slaying power.",
            Self::Oracle => "Prophet who sees the future and bestows powerful blessings.",
            Self::Hunter => "Expert tracker using traps and terrain to defeat prey.",
            Self::Beastmaster => "Form deep bonds with animal companions who fight alongside you.",
            Self::Sniper => "Patient marksman delivering devastating critical hits from afar.",
            Self::Warden => "Guardian of nature wielding primal magic and protective auras.",
            Self::MartialArtist => "Perfect unarmed combat technique delivering rapid, precise strikes.",
            Self::SpiritWalker => "Channel chi energy for mystical abilities and spirit communion.",
            Self::IronBody => "Transform the body into an unbreakable weapon through discipline.",
            Self::WayOfShadows => "Blend stealth and martial arts for silent, deadly efficiency.",
        }
    }

    pub fn base_class(&self) -> CharacterClass {
        match self {
            Self::Berserker | Self::Guardian | Self::Weaponmaster | Self::Warlord => CharacterClass::Warrior,
            Self::Elementalist | Self::Archmage | Self::Battlemage | Self::Necromancer | Self::Chronomancer => CharacterClass::Mage,
            Self::Assassin | Self::Shadowdancer | Self::Trickster | Self::Duelist => CharacterClass::Rogue,
            Self::Healer | Self::Crusader | Self::Inquisitor | Self::Oracle => CharacterClass::Cleric,
            Self::Hunter | Self::Beastmaster | Self::Sniper | Self::Warden => CharacterClass::Ranger,
            Self::MartialArtist | Self::SpiritWalker | Self::IronBody | Self::WayOfShadows => CharacterClass::Monk,
        }
    }

    pub fn stat_modifiers(&self) -> StatModifiers {
        match self {
            Self::Berserker => StatModifiers {
                attack_mult: 1.5, defense_mult: 0.8, crit_flat: 10.0, ..Default::default()
            },
            Self::Guardian => StatModifiers {
                hp_mult: 1.3, defense_mult: 1.5, attack_mult: 0.9, ..Default::default()
            },
            Self::Weaponmaster => StatModifiers {
                attack_mult: 1.2, crit_flat: 5.0, speed_flat: 2, ..Default::default()
            },
            Self::Warlord => StatModifiers {
                hp_mult: 1.1, attack_mult: 1.1, defense_mult: 1.1, ..Default::default()
            },
            Self::Elementalist => StatModifiers {
                mana_mult: 1.3, attack_mult: 1.3, ..Default::default()
            },
            Self::Archmage => StatModifiers {
                mana_mult: 1.5, attack_mult: 1.4, defense_mult: 0.9, ..Default::default()
            },
            Self::Battlemage => StatModifiers {
                hp_mult: 1.2, attack_mult: 1.2, defense_mult: 1.2, mana_mult: 0.8, ..Default::default()
            },
            Self::Necromancer => StatModifiers {
                mana_mult: 1.2, hp_flat: -10, attack_mult: 1.1, ..Default::default()
            },
            Self::Chronomancer => StatModifiers {
                mana_mult: 1.4, speed_mult: 1.3, ..Default::default()
            },
            Self::Assassin => StatModifiers {
                attack_mult: 1.4, crit_flat: 20.0, hp_mult: 0.9, ..Default::default()
            },
            Self::Shadowdancer => StatModifiers {
                speed_mult: 1.5, crit_flat: 10.0, ..Default::default()
            },
            Self::Trickster => StatModifiers {
                speed_mult: 1.2, mana_mult: 1.3, crit_flat: 5.0, ..Default::default()
            },
            Self::Duelist => StatModifiers {
                attack_mult: 1.2, defense_mult: 1.2, crit_flat: 15.0, ..Default::default()
            },
            Self::Healer => StatModifiers {
                mana_mult: 1.5, hp_mult: 1.1, attack_mult: 0.8, ..Default::default()
            },
            Self::Crusader => StatModifiers {
                hp_mult: 1.2, attack_mult: 1.2, defense_mult: 1.2, ..Default::default()
            },
            Self::Inquisitor => StatModifiers {
                attack_mult: 1.3, defense_mult: 1.1, mana_mult: 1.1, ..Default::default()
            },
            Self::Oracle => StatModifiers {
                mana_mult: 1.4, speed_mult: 1.2, crit_flat: 5.0, ..Default::default()
            },
            Self::Hunter => StatModifiers {
                attack_mult: 1.2, speed_mult: 1.2, crit_flat: 8.0, ..Default::default()
            },
            Self::Beastmaster => StatModifiers {
                hp_mult: 1.1, attack_mult: 1.1, mana_mult: 1.2, ..Default::default()
            },
            Self::Sniper => StatModifiers {
                attack_mult: 1.4, crit_flat: 25.0, speed_mult: 0.9, ..Default::default()
            },
            Self::Warden => StatModifiers {
                hp_mult: 1.2, defense_mult: 1.3, mana_mult: 1.2, ..Default::default()
            },
            Self::MartialArtist => StatModifiers {
                attack_mult: 1.3, speed_mult: 1.3, crit_flat: 10.0, ..Default::default()
            },
            Self::SpiritWalker => StatModifiers {
                mana_mult: 1.4, attack_mult: 1.1, speed_mult: 1.1, ..Default::default()
            },
            Self::IronBody => StatModifiers {
                hp_mult: 1.4, defense_mult: 1.4, speed_mult: 0.9, ..Default::default()
            },
            Self::WayOfShadows => StatModifiers {
                speed_mult: 1.4, crit_flat: 15.0, attack_mult: 1.2, ..Default::default()
            },
        }
    }

    pub fn abilities(&self) -> Vec<SpecializationAbility> {
        match self {
            Self::Berserker => vec![
                SpecializationAbility::new("Bloodrage", "Enter a fury increasing damage by 50% but taking 25% more damage", 20, 30, AbilityType::Active),
                SpecializationAbility::new("Dual Wield Mastery", "Wield two weapons with full effectiveness", 20, 0, AbilityType::Passive),
                SpecializationAbility::new("Reckless Swing", "Massive attack hitting all nearby enemies", 25, 40, AbilityType::Active),
                SpecializationAbility::new("Blood Frenzy", "Gain attack speed with each kill", 30, 0, AbilityType::Passive),
                SpecializationAbility::new("Deathwish", "Below 30% HP, deal double damage", 40, 0, AbilityType::Passive),
                SpecializationAbility::new("Unstoppable Rage", "Become immune to CC while in Bloodrage", 50, 60, AbilityType::Active),
            ],
            Self::Guardian => vec![
                SpecializationAbility::new("Shield Wall", "Block all frontal attacks for 5 seconds", 20, 25, AbilityType::Active),
                SpecializationAbility::new("Fortress", "Increased block chance and armor", 20, 0, AbilityType::Passive),
                SpecializationAbility::new("Taunt", "Force enemies to attack you", 25, 15, AbilityType::Active),
                SpecializationAbility::new("Iron Skin", "Reduce all damage taken by 15%", 30, 0, AbilityType::Passive),
                SpecializationAbility::new("Last Stand", "Become invulnerable at 1 HP for 3 seconds", 40, 120, AbilityType::Active),
                SpecializationAbility::new("Phalanx", "Extend shield protection to nearby allies", 50, 45, AbilityType::Active),
            ],
            Self::Weaponmaster => vec![
                SpecializationAbility::new("Weapon Swap", "Instantly switch weapons with no penalty", 20, 0, AbilityType::Passive),
                SpecializationAbility::new("Exploit Weakness", "Attacks ignore 20% armor", 20, 0, AbilityType::Passive),
                SpecializationAbility::new("Blade Dance", "Series of rapid strikes hitting multiple times", 25, 20, AbilityType::Active),
                SpecializationAbility::new("Adaptive Style", "Gain bonuses based on equipped weapon type", 30, 0, AbilityType::Passive),
                SpecializationAbility::new("Disarm", "Remove enemy weapon temporarily", 35, 30, AbilityType::Active),
                SpecializationAbility::new("Perfect Form", "All weapon skills deal 30% more damage", 50, 0, AbilityType::Passive),
            ],
            Self::Warlord => vec![
                SpecializationAbility::new("Rally", "Boost ally attack and defense by 20%", 20, 30, AbilityType::Active),
                SpecializationAbility::new("Commanding Presence", "Allies near you gain 10% all stats", 20, 0, AbilityType::Passive),
                SpecializationAbility::new("Battle Cry", "Fear nearby enemies and buff allies", 25, 35, AbilityType::Active),
                SpecializationAbility::new("Tactical Insight", "See enemy weaknesses and resistances", 30, 0, AbilityType::Passive),
                SpecializationAbility::new("Coordinated Strike", "Command allies to attack in unison", 40, 25, AbilityType::Active),
                SpecializationAbility::new("Legendary Commander", "All ally buffs doubled in effectiveness", 50, 0, AbilityType::Passive),
            ],
            Self::Elementalist => vec![
                SpecializationAbility::new("Fireball", "Launch an explosive ball of fire", 20, 15, AbilityType::Active),
                SpecializationAbility::new("Frost Nova", "Freeze all nearby enemies", 20, 20, AbilityType::Active),
                SpecializationAbility::new("Lightning Bolt", "Strike with chain lightning", 25, 18, AbilityType::Active),
                SpecializationAbility::new("Elemental Affinity", "Reduced mana cost for elemental spells", 30, 0, AbilityType::Passive),
                SpecializationAbility::new("Meteor Shower", "Rain destruction from above", 40, 45, AbilityType::Active),
                SpecializationAbility::new("Elemental Mastery", "Combine elements for devastating combos", 50, 0, AbilityType::Passive),
            ],
            Self::Archmage => vec![
                SpecializationAbility::new("Arcane Missiles", "Rapid fire magical projectiles", 20, 12, AbilityType::Active),
                SpecializationAbility::new("Mana Shield", "Convert mana to absorb damage", 20, 25, AbilityType::Active),
                SpecializationAbility::new("Spellpower", "All spells deal 25% more damage", 25, 0, AbilityType::Passive),
                SpecializationAbility::new("Arcane Explosion", "Massive AoE magic damage", 30, 35, AbilityType::Active),
                SpecializationAbility::new("Infinite Wisdom", "Mana regeneration doubled", 40, 0, AbilityType::Passive),
                SpecializationAbility::new("Armageddon", "Channel ultimate destruction spell", 50, 90, AbilityType::Active),
            ],
            Self::Battlemage => vec![
                SpecializationAbility::new("Spellblade", "Enchant weapon with magical damage", 20, 20, AbilityType::Active),
                SpecializationAbility::new("War Magic", "Cast spells in melee without penalty", 20, 0, AbilityType::Passive),
                SpecializationAbility::new("Arcane Strike", "Melee attack with bonus magic damage", 25, 10, AbilityType::Active),
                SpecializationAbility::new("Battlemage Armor", "Wear heavy armor without spell penalty", 30, 0, AbilityType::Passive),
                SpecializationAbility::new("Spell Parry", "Counter spells with melee attacks", 40, 15, AbilityType::Active),
                SpecializationAbility::new("Eldritch Knight", "Permanent weapon enchantment", 50, 0, AbilityType::Passive),
            ],
            Self::Necromancer => vec![
                SpecializationAbility::new("Raise Skeleton", "Summon a skeletal warrior", 20, 25, AbilityType::Active),
                SpecializationAbility::new("Life Drain", "Steal health from enemies", 20, 15, AbilityType::Active),
                SpecializationAbility::new("Death Coil", "Damage enemies or heal undead", 25, 12, AbilityType::Active),
                SpecializationAbility::new("Undead Mastery", "Summons are 50% stronger", 30, 0, AbilityType::Passive),
                SpecializationAbility::new("Army of the Dead", "Raise multiple undead at once", 40, 60, AbilityType::Active),
                SpecializationAbility::new("Lich Form", "Transform into a powerful lich", 50, 120, AbilityType::Active),
            ],
            Self::Chronomancer => vec![
                SpecializationAbility::new("Time Slow", "Slow time around you", 20, 30, AbilityType::Active),
                SpecializationAbility::new("Temporal Echo", "Chance to repeat actions", 20, 0, AbilityType::Passive),
                SpecializationAbility::new("Rewind", "Undo last few seconds of damage", 25, 45, AbilityType::Active),
                SpecializationAbility::new("Haste", "Dramatically increase action speed", 30, 25, AbilityType::Active),
                SpecializationAbility::new("Paradox", "Exist in two places at once briefly", 40, 60, AbilityType::Active),
                SpecializationAbility::new("Time Stop", "Freeze time completely for 3 seconds", 50, 180, AbilityType::Active),
            ],
            Self::Assassin => vec![
                SpecializationAbility::new("Assassination", "Instant kill on low HP targets", 20, 45, AbilityType::Active),
                SpecializationAbility::new("Poison Mastery", "All attacks apply deadly poison", 20, 0, AbilityType::Passive),
                SpecializationAbility::new("Vanish", "Become invisible instantly", 25, 30, AbilityType::Active),
                SpecializationAbility::new("Lethal Toxins", "Poisons deal 50% more damage", 30, 0, AbilityType::Passive),
                SpecializationAbility::new("Death Mark", "Mark target for guaranteed crit", 40, 35, AbilityType::Active),
                SpecializationAbility::new("Deathblow", "Execute targets below 35% HP", 50, 20, AbilityType::Active),
            ],
            Self::Shadowdancer => vec![
                SpecializationAbility::new("Shadow Step", "Teleport through shadows", 20, 10, AbilityType::Active),
                SpecializationAbility::new("One with Shadows", "Invisible in dark areas", 20, 0, AbilityType::Passive),
                SpecializationAbility::new("Shadow Clone", "Create a decoy of yourself", 25, 25, AbilityType::Active),
                SpecializationAbility::new("Evasion", "50% chance to dodge attacks", 30, 0, AbilityType::Passive),
                SpecializationAbility::new("Shadow Assault", "Rapid strikes from multiple angles", 40, 30, AbilityType::Active),
                SpecializationAbility::new("Umbral Form", "Become pure shadow, immune to physical", 50, 60, AbilityType::Active),
            ],
            Self::Trickster => vec![
                SpecializationAbility::new("Deploy Trap", "Place a damaging trap", 20, 15, AbilityType::Active),
                SpecializationAbility::new("Misdirection", "Enemies have reduced accuracy", 20, 0, AbilityType::Passive),
                SpecializationAbility::new("Smoke Bomb", "Blind and confuse enemies", 25, 20, AbilityType::Active),
                SpecializationAbility::new("Trap Mastery", "Traps deal double damage", 30, 0, AbilityType::Passive),
                SpecializationAbility::new("Grand Illusion", "Create multiple illusory copies", 40, 40, AbilityType::Active),
                SpecializationAbility::new("Master of Deception", "Traps are invisible, illusions persistent", 50, 0, AbilityType::Passive),
            ],
            Self::Duelist => vec![
                SpecializationAbility::new("Riposte", "Counter after successful parry", 20, 0, AbilityType::Passive),
                SpecializationAbility::new("En Garde", "Increased parry chance", 20, 0, AbilityType::Passive),
                SpecializationAbility::new("Lunge", "Quick strike with extended range", 25, 8, AbilityType::Active),
                SpecializationAbility::new("Precision Strike", "Target vital points for bonus damage", 30, 12, AbilityType::Active),
                SpecializationAbility::new("Blade Flurry", "Series of precise cuts", 40, 25, AbilityType::Active),
                SpecializationAbility::new("Perfect Riposte", "Counterattacks deal triple damage", 50, 0, AbilityType::Passive),
            ],
            Self::Healer => vec![
                SpecializationAbility::new("Greater Heal", "Powerful single-target heal", 20, 20, AbilityType::Active),
                SpecializationAbility::new("Healing Aura", "Passive healing to nearby allies", 20, 0, AbilityType::Passive),
                SpecializationAbility::new("Purify", "Remove all negative effects", 25, 18, AbilityType::Active),
                SpecializationAbility::new("Blessed Recovery", "Heals are 30% more effective", 30, 0, AbilityType::Passive),
                SpecializationAbility::new("Mass Heal", "Heal all allies significantly", 40, 45, AbilityType::Active),
                SpecializationAbility::new("Divine Intervention", "Prevent ally death once per battle", 50, 300, AbilityType::Active),
            ],
            Self::Crusader => vec![
                SpecializationAbility::new("Holy Strike", "Smite enemies with divine power", 20, 15, AbilityType::Active),
                SpecializationAbility::new("Righteous Fury", "Bonus damage against evil", 20, 0, AbilityType::Passive),
                SpecializationAbility::new("Divine Shield", "Become invulnerable briefly", 25, 60, AbilityType::Active),
                SpecializationAbility::new("Consecrate", "Create holy ground damaging enemies", 30, 25, AbilityType::Active),
                SpecializationAbility::new("Zealot's Fervor", "Attack speed increases with faith", 40, 0, AbilityType::Passive),
                SpecializationAbility::new("Avatar of Light", "Transform into holy champion", 50, 120, AbilityType::Active),
            ],
            Self::Inquisitor => vec![
                SpecializationAbility::new("Dispel Magic", "Remove magical effects from target", 20, 20, AbilityType::Active),
                SpecializationAbility::new("Magic Resistance", "Take reduced magic damage", 20, 0, AbilityType::Passive),
                SpecializationAbility::new("Exorcism", "Massive damage to demons/undead", 25, 25, AbilityType::Active),
                SpecializationAbility::new("Silence", "Prevent enemy spellcasting", 30, 22, AbilityType::Active),
                SpecializationAbility::new("Demon Hunter", "Track and detect demonic presence", 35, 0, AbilityType::Passive),
                SpecializationAbility::new("Judgment", "Execute heretics and demons", 50, 45, AbilityType::Active),
            ],
            Self::Oracle => vec![
                SpecializationAbility::new("Foresight", "See enemy attacks before they happen", 20, 30, AbilityType::Active),
                SpecializationAbility::new("Blessed Vision", "Cannot be surprised or flanked", 20, 0, AbilityType::Passive),
                SpecializationAbility::new("Prophecy", "Predict and modify battle outcome", 25, 40, AbilityType::Active),
                SpecializationAbility::new("Fortune's Favor", "Increase ally critical chance", 30, 25, AbilityType::Active),
                SpecializationAbility::new("Divine Guidance", "Allies gain accuracy bonus", 40, 0, AbilityType::Passive),
                SpecializationAbility::new("Fate Weaver", "Reroll any dice roll in battle", 50, 60, AbilityType::Active),
            ],
            Self::Hunter => vec![
                SpecializationAbility::new("Track Prey", "Reveal hidden enemies", 20, 15, AbilityType::Active),
                SpecializationAbility::new("Trap Expert", "Traps are more effective", 20, 0, AbilityType::Passive),
                SpecializationAbility::new("Snare", "Immobilize target", 25, 18, AbilityType::Active),
                SpecializationAbility::new("Hunter's Mark", "Marked targets take bonus damage", 30, 20, AbilityType::Active),
                SpecializationAbility::new("Survival Instinct", "Sense danger before it strikes", 35, 0, AbilityType::Passive),
                SpecializationAbility::new("Alpha Predator", "Become the ultimate hunter", 50, 0, AbilityType::Passive),
            ],
            Self::Beastmaster => vec![
                SpecializationAbility::new("Call Companion", "Summon animal companion", 20, 30, AbilityType::Active),
                SpecializationAbility::new("Beast Bond", "Share senses with companion", 20, 0, AbilityType::Passive),
                SpecializationAbility::new("Pack Tactics", "Bonus when attacking with companion", 25, 0, AbilityType::Passive),
                SpecializationAbility::new("Wild Command", "Direct companion to use special attack", 30, 15, AbilityType::Active),
                SpecializationAbility::new("Spirit Animal", "Companion gains magical abilities", 40, 0, AbilityType::Passive),
                SpecializationAbility::new("Summon Pack", "Call multiple animal allies", 50, 60, AbilityType::Active),
            ],
            Self::Sniper => vec![
                SpecializationAbility::new("Steady Aim", "Greatly increased accuracy and crit", 20, 0, AbilityType::Passive),
                SpecializationAbility::new("Headshot", "Attempt instant kill from range", 20, 35, AbilityType::Active),
                SpecializationAbility::new("Camouflage", "Become hidden while stationary", 25, 20, AbilityType::Active),
                SpecializationAbility::new("Killshot", "Guaranteed critical hit", 30, 40, AbilityType::Active),
                SpecializationAbility::new("Eagle Eye", "Extended range and accuracy", 35, 0, AbilityType::Passive),
                SpecializationAbility::new("One Shot One Kill", "First hit in combat deals triple damage", 50, 0, AbilityType::Passive),
            ],
            Self::Warden => vec![
                SpecializationAbility::new("Nature's Grasp", "Entangle enemies with vines", 20, 20, AbilityType::Active),
                SpecializationAbility::new("Forest Guardian", "Bonus stats in natural terrain", 20, 0, AbilityType::Passive),
                SpecializationAbility::new("Barkskin", "Gain natural armor", 25, 25, AbilityType::Active),
                SpecializationAbility::new("Regrowth", "Heal over time effect", 30, 22, AbilityType::Active),
                SpecializationAbility::new("Nature's Wrath", "Call down lightning and thorns", 40, 35, AbilityType::Active),
                SpecializationAbility::new("Avatar of Nature", "Transform into nature's champion", 50, 90, AbilityType::Active),
            ],
            Self::MartialArtist => vec![
                SpecializationAbility::new("Flurry of Blows", "Rapid unarmed strikes", 20, 12, AbilityType::Active),
                SpecializationAbility::new("Unarmed Mastery", "Fists count as magical weapons", 20, 0, AbilityType::Passive),
                SpecializationAbility::new("Stunning Fist", "Chance to stun on hit", 25, 15, AbilityType::Active),
                SpecializationAbility::new("Perfect Balance", "Cannot be knocked down", 30, 0, AbilityType::Passive),
                SpecializationAbility::new("Pressure Points", "Target weak points for bonus damage", 40, 18, AbilityType::Active),
                SpecializationAbility::new("Thousand Fists", "Ultimate flurry of devastating strikes", 50, 45, AbilityType::Active),
            ],
            Self::SpiritWalker => vec![
                SpecializationAbility::new("Chi Bolt", "Project chi energy as attack", 20, 15, AbilityType::Active),
                SpecializationAbility::new("Inner Peace", "Increased mana regeneration", 20, 0, AbilityType::Passive),
                SpecializationAbility::new("Spirit Sight", "See invisible and spiritual beings", 25, 20, AbilityType::Active),
                SpecializationAbility::new("Chi Healing", "Heal self and allies with chi", 30, 25, AbilityType::Active),
                SpecializationAbility::new("Astral Form", "Project spirit to scout ahead", 40, 35, AbilityType::Active),
                SpecializationAbility::new("Transcendence", "Achieve perfect unity of body and spirit", 50, 0, AbilityType::Passive),
            ],
            Self::IronBody => vec![
                SpecializationAbility::new("Stone Stance", "Greatly increased defense, reduced speed", 20, 20, AbilityType::Active),
                SpecializationAbility::new("Toughened Hide", "Reduced physical damage taken", 20, 0, AbilityType::Passive),
                SpecializationAbility::new("Iron Palm", "Devastating strike that ignores armor", 25, 18, AbilityType::Active),
                SpecializationAbility::new("Diamond Body", "Immune to critical hits", 30, 0, AbilityType::Passive),
                SpecializationAbility::new("Unbreakable", "Cannot be reduced below 1 HP for 5 seconds", 40, 90, AbilityType::Active),
                SpecializationAbility::new("Adamantine Form", "Transform body into living metal", 50, 60, AbilityType::Active),
            ],
            Self::WayOfShadows => vec![
                SpecializationAbility::new("Shadow Strike", "Attack from shadows for bonus damage", 20, 12, AbilityType::Active),
                SpecializationAbility::new("Darkness Within", "Easier to hide and move silently", 20, 0, AbilityType::Passive),
                SpecializationAbility::new("Cloak of Shadows", "Become invisible briefly", 25, 25, AbilityType::Active),
                SpecializationAbility::new("Silent Kill", "Eliminate unaware targets instantly", 30, 30, AbilityType::Active),
                SpecializationAbility::new("Shadow Meld", "Merge with shadows for perfect stealth", 40, 0, AbilityType::Passive),
                SpecializationAbility::new("Death from Shadows", "Ultimate assassination technique", 50, 45, AbilityType::Active),
            ],
        }
    }

    pub fn passives(&self) -> Vec<SpecializationPassive> {
        match self {
            Self::Berserker => vec![
                SpecializationPassive::new("Bloodlust", "Gain rage on taking damage", 20),
                SpecializationPassive::new("Thick Skinned", "Reduced damage while in rage", 35),
                SpecializationPassive::new("Death's Door", "Cannot die while in rage", 50),
            ],
            Self::Guardian => vec![
                SpecializationPassive::new("Shield Mastery", "+25% block effectiveness", 20),
                SpecializationPassive::new("Stalwart", "Reduced knockback and stun duration", 35),
                SpecializationPassive::new("Indomitable", "Auto-block lethal attacks once per battle", 50),
            ],
            Self::Weaponmaster => vec![
                SpecializationPassive::new("Quick Draw", "Faster weapon swap speed", 20),
                SpecializationPassive::new("Weapon Specialist", "+15% damage with all weapons", 35),
                SpecializationPassive::new("Master of Arms", "Critical hits have bonus effects per weapon", 50),
            ],
            Self::Warlord => vec![
                SpecializationPassive::new("Inspiring Leader", "Allies resist fear effects", 20),
                SpecializationPassive::new("Tactical Mind", "Reduced cooldown on command abilities", 35),
                SpecializationPassive::new("Supreme Commander", "All allies gain your leadership bonus", 50),
            ],
            Self::Elementalist => vec![
                SpecializationPassive::new("Elemental Attunement", "Resistance to fire/ice/lightning", 20),
                SpecializationPassive::new("Reactive Elements", "Chance to trigger elemental burst on hit", 35),
                SpecializationPassive::new("Primordial Power", "Elemental spells ignore resistance", 50),
            ],
            Self::Archmage => vec![
                SpecializationPassive::new("Arcane Knowledge", "Learn spells faster", 20),
                SpecializationPassive::new("Spell Efficiency", "Reduced mana costs", 35),
                SpecializationPassive::new("Arcane Supremacy", "Spells cannot be interrupted", 50),
            ],
            Self::Battlemage => vec![
                SpecializationPassive::new("Combat Casting", "No penalty for casting in melee", 20),
                SpecializationPassive::new("Arcane Warrior", "Melee attacks restore mana", 35),
                SpecializationPassive::new("Spell Sword", "All melee attacks carry spell effects", 50),
            ],
            Self::Necromancer => vec![
                SpecializationPassive::new("Dark Pact", "Heal when minions deal damage", 20),
                SpecializationPassive::new("Soul Harvest", "Gain power from killed enemies", 35),
                SpecializationPassive::new("Lord of the Dead", "Unlimited undead minions", 50),
            ],
            Self::Chronomancer => vec![
                SpecializationPassive::new("Time Sense", "See enemy actions before they happen", 20),
                SpecializationPassive::new("Temporal Shield", "Chance to undo damage taken", 35),
                SpecializationPassive::new("Master of Time", "Cooldowns reduced by 50%", 50),
            ],
            Self::Assassin => vec![
                SpecializationPassive::new("Deadly Precision", "Increased critical damage", 20),
                SpecializationPassive::new("Venomous", "Poisons stack and last longer", 35),
                SpecializationPassive::new("Death Incarnate", "Guaranteed crit from stealth", 50),
            ],
            Self::Shadowdancer => vec![
                SpecializationPassive::new("Shadow Affinity", "Move faster in darkness", 20),
                SpecializationPassive::new("Ethereal", "Chance to phase through attacks", 35),
                SpecializationPassive::new("Living Shadow", "Permanent partial invisibility", 50),
            ],
            Self::Trickster => vec![
                SpecializationPassive::new("Cunning", "Enemies have reduced accuracy vs you", 20),
                SpecializationPassive::new("Elaborate Schemes", "Traps trigger chain reactions", 35),
                SpecializationPassive::new("Master Illusionist", "Illusions can deal real damage", 50),
            ],
            Self::Duelist => vec![
                SpecializationPassive::new("Fencer's Grace", "Bonus dodge in light armor", 20),
                SpecializationPassive::new("Blade Dancer", "Chain attacks without penalty", 35),
                SpecializationPassive::new("Untouchable", "Evade most attacks in 1v1 combat", 50),
            ],
            Self::Healer => vec![
                SpecializationPassive::new("Healing Touch", "Basic attacks heal allies", 20),
                SpecializationPassive::new("Preservation", "Overhealing creates shields", 35),
                SpecializationPassive::new("Font of Life", "Massive passive healing aura", 50),
            ],
            Self::Crusader => vec![
                SpecializationPassive::new("Holy Fervor", "Damage scales with missing HP", 20),
                SpecializationPassive::new("Divine Protection", "Allies near you take less damage", 35),
                SpecializationPassive::new("Champion of Light", "Immune to darkness and death effects", 50),
            ],
            Self::Inquisitor => vec![
                SpecializationPassive::new("Spell Breaker", "Chance to reflect spells", 20),
                SpecializationPassive::new("Purifying Flame", "Attacks cleanse magical effects", 35),
                SpecializationPassive::new("Demon's Bane", "Massive damage bonus vs demons", 50),
            ],
            Self::Oracle => vec![
                SpecializationPassive::new("Prescience", "Cannot be surprised", 20),
                SpecializationPassive::new("Destiny's Child", "Chance to avoid fatal damage", 35),
                SpecializationPassive::new("Omniscience", "See all hidden information", 50),
            ],
            Self::Hunter => vec![
                SpecializationPassive::new("Keen Senses", "Detect hidden enemies", 20),
                SpecializationPassive::new("Patient Stalker", "Bonus damage to marked targets", 35),
                SpecializationPassive::new("Apex Hunter", "Track any creature perfectly", 50),
            ],
            Self::Beastmaster => vec![
                SpecializationPassive::new("Animal Empathy", "Beasts won't attack first", 20),
                SpecializationPassive::new("Pack Leader", "Companion gains your buffs", 35),
                SpecializationPassive::new("One with Nature", "Transform into companion temporarily", 50),
            ],
            Self::Sniper => vec![
                SpecializationPassive::new("Hawk Eye", "Extended critical range", 20),
                SpecializationPassive::new("Patience", "Bonus damage for aiming longer", 35),
                SpecializationPassive::new("Perfect Shot", "Ignore all range penalties", 50),
            ],
            Self::Warden => vec![
                SpecializationPassive::new("Nature's Ally", "Animals assist in combat", 20),
                SpecializationPassive::new("Thorns", "Damage attackers when hit", 35),
                SpecializationPassive::new("Primal Guardian", "Massive bonuses in natural terrain", 50),
            ],
            Self::MartialArtist => vec![
                SpecializationPassive::new("Lightning Reflexes", "Increased dodge chance", 20),
                SpecializationPassive::new("Chi Flow", "Attacks restore energy", 35),
                SpecializationPassive::new("Fist of Legend", "Unarmed attacks rival magical weapons", 50),
            ],
            Self::SpiritWalker => vec![
                SpecializationPassive::new("Spiritual Awareness", "Sense supernatural beings", 20),
                SpecializationPassive::new("Chi Armor", "Invisible spiritual protection", 35),
                SpecializationPassive::new("Enlightenment", "Immune to mind effects", 50),
            ],
            Self::IronBody => vec![
                SpecializationPassive::new("Hardened", "Reduced damage from all sources", 20),
                SpecializationPassive::new("Endurance", "Stamina regenerates faster", 35),
                SpecializationPassive::new("Impervious", "Massive damage reduction", 50),
            ],
            Self::WayOfShadows => vec![
                SpecializationPassive::new("Shadow Step", "Silent movement always", 20),
                SpecializationPassive::new("Assassin's Eye", "See vital points", 35),
                SpecializationPassive::new("One with Darkness", "Invisible in any shadow", 50),
            ],
        }
    }
}

// ============================================================================
// Tier 2 (Advanced) Specializations
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum AdvancedSpecialization {
    // Berserker Tier 2
    BloodReaver,
    RageLord,
    // Guardian Tier 2
    Juggernaut,
    Paladin,
    // Weaponmaster Tier 2
    BladesSaint,
    ArmsChampion,
    // Warlord Tier 2
    HighCommander,
    BattleLord,
    // Elementalist Tier 2
    Pyromancer,
    Cryomancer,
    Stormcaller,
    // Archmage Tier 2
    Magister,
    VoidMage,
    // Battlemage Tier 2
    SpellKnight,
    RuneWarrior,
    // Necromancer Tier 2
    LichKing,
    DeathKnight,
    // Chronomancer Tier 2
    TimeLord,
    FateWeaver,
    // Assassin Tier 2
    DeathShadow,
    PoisonMaster,
    // Shadowdancer Tier 2
    NightBlade,
    PhantomStriker,
    // Trickster Tier 2
    GrandIllusionist,
    TrapMaster,
    // Duelist Tier 2
    SwordSaint,
    BladeMaster,
    // Healer Tier 2
    HighPriest,
    LifeWarden,
    // Crusader Tier 2
    Templar,
    HolyChampion,
    // Inquisitor Tier 2
    WitchHunter,
    DemonSlayer,
    // Oracle Tier 2
    Prophet,
    Seer,
    // Hunter Tier 2
    MasterTracker,
    PrimeHunter,
    // Beastmaster Tier 2
    AlphaMaster,
    SpiritBonder,
    // Sniper Tier 2
    Deadeye,
    GhostShot,
    // Warden Tier 2
    ArchDruid,
    ForestSentinel,
    // Martial Artist Tier 2
    GrandMaster,
    FistSage,
    // Spirit Walker Tier 2
    ChiMaster,
    SpiritSage,
    // Iron Body Tier 2
    LivingFortress,
    SteelDragon,
    // Way of Shadows Tier 2
    ShadowMaster,
    NightLord,
}

impl AdvancedSpecialization {
    pub fn name(&self) -> &'static str {
        match self {
            Self::BloodReaver => "Blood Reaver",
            Self::RageLord => "Rage Lord",
            Self::Juggernaut => "Juggernaut",
            Self::Paladin => "Paladin",
            Self::BladesSaint => "Blade Saint",
            Self::ArmsChampion => "Arms Champion",
            Self::HighCommander => "High Commander",
            Self::BattleLord => "Battle Lord",
            Self::Pyromancer => "Pyromancer",
            Self::Cryomancer => "Cryomancer",
            Self::Stormcaller => "Stormcaller",
            Self::Magister => "Magister",
            Self::VoidMage => "Void Mage",
            Self::SpellKnight => "Spell Knight",
            Self::RuneWarrior => "Rune Warrior",
            Self::LichKing => "Lich King",
            Self::DeathKnight => "Death Knight",
            Self::TimeLord => "Time Lord",
            Self::FateWeaver => "Fate Weaver",
            Self::DeathShadow => "Death Shadow",
            Self::PoisonMaster => "Poison Master",
            Self::NightBlade => "Night Blade",
            Self::PhantomStriker => "Phantom Striker",
            Self::GrandIllusionist => "Grand Illusionist",
            Self::TrapMaster => "Trap Master",
            Self::SwordSaint => "Sword Saint",
            Self::BladeMaster => "Blade Master",
            Self::HighPriest => "High Priest",
            Self::LifeWarden => "Life Warden",
            Self::Templar => "Templar",
            Self::HolyChampion => "Holy Champion",
            Self::WitchHunter => "Witch Hunter",
            Self::DemonSlayer => "Demon Slayer",
            Self::Prophet => "Prophet",
            Self::Seer => "Seer",
            Self::MasterTracker => "Master Tracker",
            Self::PrimeHunter => "Prime Hunter",
            Self::AlphaMaster => "Alpha Master",
            Self::SpiritBonder => "Spirit Bonder",
            Self::Deadeye => "Deadeye",
            Self::GhostShot => "Ghost Shot",
            Self::ArchDruid => "Arch Druid",
            Self::ForestSentinel => "Forest Sentinel",
            Self::GrandMaster => "Grand Master",
            Self::FistSage => "Fist Sage",
            Self::ChiMaster => "Chi Master",
            Self::SpiritSage => "Spirit Sage",
            Self::LivingFortress => "Living Fortress",
            Self::SteelDragon => "Steel Dragon",
            Self::ShadowMaster => "Shadow Master",
            Self::NightLord => "Night Lord",
        }
    }

    pub fn parent_specialization(&self) -> Specialization {
        match self {
            Self::BloodReaver | Self::RageLord => Specialization::Berserker,
            Self::Juggernaut | Self::Paladin => Specialization::Guardian,
            Self::BladesSaint | Self::ArmsChampion => Specialization::Weaponmaster,
            Self::HighCommander | Self::BattleLord => Specialization::Warlord,
            Self::Pyromancer | Self::Cryomancer | Self::Stormcaller => Specialization::Elementalist,
            Self::Magister | Self::VoidMage => Specialization::Archmage,
            Self::SpellKnight | Self::RuneWarrior => Specialization::Battlemage,
            Self::LichKing | Self::DeathKnight => Specialization::Necromancer,
            Self::TimeLord | Self::FateWeaver => Specialization::Chronomancer,
            Self::DeathShadow | Self::PoisonMaster => Specialization::Assassin,
            Self::NightBlade | Self::PhantomStriker => Specialization::Shadowdancer,
            Self::GrandIllusionist | Self::TrapMaster => Specialization::Trickster,
            Self::SwordSaint | Self::BladeMaster => Specialization::Duelist,
            Self::HighPriest | Self::LifeWarden => Specialization::Healer,
            Self::Templar | Self::HolyChampion => Specialization::Crusader,
            Self::WitchHunter | Self::DemonSlayer => Specialization::Inquisitor,
            Self::Prophet | Self::Seer => Specialization::Oracle,
            Self::MasterTracker | Self::PrimeHunter => Specialization::Hunter,
            Self::AlphaMaster | Self::SpiritBonder => Specialization::Beastmaster,
            Self::Deadeye | Self::GhostShot => Specialization::Sniper,
            Self::ArchDruid | Self::ForestSentinel => Specialization::Warden,
            Self::GrandMaster | Self::FistSage => Specialization::MartialArtist,
            Self::ChiMaster | Self::SpiritSage => Specialization::SpiritWalker,
            Self::LivingFortress | Self::SteelDragon => Specialization::IronBody,
            Self::ShadowMaster | Self::NightLord => Specialization::WayOfShadows,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::BloodReaver => "Embrace the bloodlust, healing through carnage.",
            Self::RageLord => "Master of fury with unparalleled rage duration.",
            Self::Juggernaut => "Unstoppable force that cannot be moved or stopped.",
            Self::Paladin => "Holy defender blessed with divine protection.",
            Self::BladesSaint => "Transcendent weapon skill approaching perfection.",
            Self::ArmsChampion => "Master of every weapon style and technique.",
            Self::HighCommander => "Strategic genius maximizing ally potential.",
            Self::BattleLord => "Lead from the front with devastating presence.",
            Self::Pyromancer => "Master of fire and explosive destruction.",
            Self::Cryomancer => "Control ice to freeze and shatter enemies.",
            Self::Stormcaller => "Command lightning and weather itself.",
            Self::Magister => "Supreme arcane authority with ultimate power.",
            Self::VoidMage => "Wield the power of nothingness and entropy.",
            Self::SpellKnight => "Perfect fusion of martial and magical arts.",
            Self::RuneWarrior => "Ancient rune magic enhances every strike.",
            Self::LichKing => "Ultimate necromantic power over death itself.",
            Self::DeathKnight => "Undead warrior with necrotic might.",
            Self::TimeLord => "Absolute mastery over the flow of time.",
            Self::FateWeaver => "Manipulate destiny and probability itself.",
            Self::DeathShadow => "Perfect killer leaving no trace.",
            Self::PoisonMaster => "Create toxins that slay gods.",
            Self::NightBlade => "One with the darkness, untouchable.",
            Self::PhantomStriker => "Strike from impossible angles.",
            Self::GrandIllusionist => "Create illusions indistinguishable from reality.",
            Self::TrapMaster => "Legendary trap-making skills.",
            Self::SwordSaint => "Blade skill approaching divine perfection.",
            Self::BladeMaster => "Ultimate mastery of sword techniques.",
            Self::HighPriest => "Channel divine healing power directly.",
            Self::LifeWarden => "Prevent death itself through pure will.",
            Self::Templar => "Holy warrior with impenetrable faith.",
            Self::HolyChampion => "Avatar of divine justice and might.",
            Self::WitchHunter => "Specialized in eliminating magic users.",
            Self::DemonSlayer => "Ultimate demon and undead destroyer.",
            Self::Prophet => "See and shape the future itself.",
            Self::Seer => "Omniscient awareness of all things.",
            Self::MasterTracker => "Track anything across any terrain.",
            Self::PrimeHunter => "Ultimate predator of any prey.",
            Self::AlphaMaster => "Command entire packs of beasts.",
            Self::SpiritBonder => "Deep spiritual connection with companions.",
            Self::Deadeye => "Never miss a shot, ever.",
            Self::GhostShot => "Invisible shots from impossible positions.",
            Self::ArchDruid => "Supreme nature magic authority.",
            Self::ForestSentinel => "Eternal guardian of the wilds.",
            Self::GrandMaster => "Legendary martial arts mastery.",
            Self::FistSage => "Transcendent unarmed combat wisdom.",
            Self::ChiMaster => "Perfect control of life energy.",
            Self::SpiritSage => "Commune with spirits as equals.",
            Self::LivingFortress => "Body harder than any fortress wall.",
            Self::SteelDragon => "Legendary defensive martial art.",
            Self::ShadowMaster => "Command shadows as extensions of self.",
            Self::NightLord => "Ruler of darkness and stealth.",
        }
    }

    pub fn stat_modifiers(&self) -> StatModifiers {
        // Advanced specs give additional bonuses on top of base spec
        match self {
            Self::BloodReaver => StatModifiers {
                attack_mult: 1.2, hp_mult: 1.1, ..Default::default()
            },
            Self::RageLord => StatModifiers {
                attack_mult: 1.3, crit_flat: 5.0, ..Default::default()
            },
            Self::Juggernaut => StatModifiers {
                hp_mult: 1.3, defense_mult: 1.2, speed_mult: 0.9, ..Default::default()
            },
            Self::Paladin => StatModifiers {
                hp_mult: 1.1, defense_mult: 1.1, mana_mult: 1.2, ..Default::default()
            },
            _ => StatModifiers::default(), // Other specs use similar patterns
        }
    }
}

// ============================================================================
// Abilities
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum AbilityType {
    Active,
    Passive,
    Toggle,
    Ultimate,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpecializationAbility {
    pub name: String,
    pub description: String,
    pub level_required: u32,
    pub cooldown: u32,
    pub ability_type: AbilityType,
}

impl SpecializationAbility {
    pub fn new(name: &str, description: &str, level_required: u32, cooldown: u32, ability_type: AbilityType) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            level_required,
            cooldown,
            ability_type,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpecializationPassive {
    pub name: String,
    pub description: String,
    pub level_required: u32,
}

impl SpecializationPassive {
    pub fn new(name: &str, description: &str, level_required: u32) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            level_required,
        }
    }
}

// ============================================================================
// Character Class State
// ============================================================================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterClassState {
    pub base_class: CharacterClass,
    pub specialization: Option<Specialization>,
    pub advanced_specialization: Option<AdvancedSpecialization>,
    pub level: u32,
    pub experience: u64,
    pub unlocked_abilities: Vec<String>,
    pub unlocked_passives: Vec<String>,
    pub ability_cooldowns: HashMap<String, u32>,
}

impl CharacterClassState {
    pub fn new(base_class: CharacterClass) -> Self {
        Self {
            base_class,
            specialization: None,
            advanced_specialization: None,
            level: 1,
            experience: 0,
            unlocked_abilities: Vec::new(),
            unlocked_passives: Vec::new(),
            ability_cooldowns: HashMap::new(),
        }
    }

    pub fn can_specialize(&self) -> bool {
        self.level >= SPECIALIZATION_TIER1_LEVEL && self.specialization.is_none()
    }

    pub fn can_advance_specialize(&self) -> bool {
        self.level >= SPECIALIZATION_TIER2_LEVEL
            && self.specialization.is_some()
            && self.advanced_specialization.is_none()
    }

    pub fn specialize(&mut self, spec: Specialization) -> Result<(), ClassError> {
        if self.level < SPECIALIZATION_TIER1_LEVEL {
            return Err(ClassError::LevelTooLow(SPECIALIZATION_TIER1_LEVEL));
        }
        if self.specialization.is_some() {
            return Err(ClassError::AlreadySpecialized);
        }
        if !self.base_class.available_specializations().contains(&spec) {
            return Err(ClassError::InvalidSpecialization);
        }
        self.specialization = Some(spec);
        self.unlock_abilities_for_level();
        Ok(())
    }

    pub fn advance_specialize(&mut self, spec: AdvancedSpecialization) -> Result<(), ClassError> {
        if self.level < SPECIALIZATION_TIER2_LEVEL {
            return Err(ClassError::LevelTooLow(SPECIALIZATION_TIER2_LEVEL));
        }
        let current_spec = self.specialization.ok_or(ClassError::NoSpecialization)?;
        if spec.parent_specialization() != current_spec {
            return Err(ClassError::InvalidAdvancedSpecialization);
        }
        if self.advanced_specialization.is_some() {
            return Err(ClassError::AlreadyAdvancedSpecialized);
        }
        self.advanced_specialization = Some(spec);
        Ok(())
    }

    pub fn get_stats(&self) -> ClassStats {
        let mut stats = self.base_class.base_stats();

        if let Some(spec) = &self.specialization {
            stats = stats.apply_modifiers(&spec.stat_modifiers());
        }

        if let Some(adv_spec) = &self.advanced_specialization {
            stats = stats.apply_modifiers(&adv_spec.stat_modifiers());
        }

        // Apply level scaling
        let level_mult = 1.0 + (self.level as f32 - 1.0) * 0.05;
        stats.hp = (stats.hp as f32 * level_mult) as i32;
        stats.attack = (stats.attack as f32 * level_mult) as i32;
        stats.defense = (stats.defense as f32 * level_mult) as i32;
        stats.mana = (stats.mana as f32 * level_mult) as i32;

        stats
    }

    pub fn get_available_abilities(&self) -> Vec<SpecializationAbility> {
        if let Some(spec) = &self.specialization {
            spec.abilities()
                .into_iter()
                .filter(|a| a.level_required <= self.level)
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_available_passives(&self) -> Vec<SpecializationPassive> {
        if let Some(spec) = &self.specialization {
            spec.passives()
                .into_iter()
                .filter(|p| p.level_required <= self.level)
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn add_experience(&mut self, exp: u64) {
        self.experience += exp;
        while self.experience >= self.exp_for_next_level() {
            self.experience -= self.exp_for_next_level();
            self.level += 1;
            self.unlock_abilities_for_level();
        }
    }

    pub fn exp_for_next_level(&self) -> u64 {
        100 * (self.level as u64).pow(2)
    }

    fn unlock_abilities_for_level(&mut self) {
        if let Some(spec) = &self.specialization {
            for ability in spec.abilities() {
                if ability.level_required <= self.level
                    && !self.unlocked_abilities.contains(&ability.name) {
                    self.unlocked_abilities.push(ability.name.clone());
                }
            }
            for passive in spec.passives() {
                if passive.level_required <= self.level
                    && !self.unlocked_passives.contains(&passive.name) {
                    self.unlocked_passives.push(passive.name.clone());
                }
            }
        }
    }

    pub fn display_name(&self) -> String {
        if let Some(adv) = &self.advanced_specialization {
            format!("{} ({})", adv.name(), self.base_class.name())
        } else if let Some(spec) = &self.specialization {
            format!("{} ({})", spec.name(), self.base_class.name())
        } else {
            self.base_class.name().to_string()
        }
    }
}

// ============================================================================
// Errors
// ============================================================================

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClassError {
    LevelTooLow(u32),
    AlreadySpecialized,
    AlreadyAdvancedSpecialized,
    InvalidSpecialization,
    InvalidAdvancedSpecialization,
    NoSpecialization,
}

impl std::fmt::Display for ClassError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LevelTooLow(req) => write!(f, "Level {} required for this action", req),
            Self::AlreadySpecialized => write!(f, "Already specialized"),
            Self::AlreadyAdvancedSpecialized => write!(f, "Already has advanced specialization"),
            Self::InvalidSpecialization => write!(f, "Invalid specialization for this class"),
            Self::InvalidAdvancedSpecialization => write!(f, "Invalid advanced specialization"),
            Self::NoSpecialization => write!(f, "Must specialize first"),
        }
    }
}

impl std::error::Error for ClassError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_classes_have_names() {
        for class in CharacterClass::all() {
            assert!(!class.name().is_empty());
        }
    }

    #[test]
    fn test_base_stats_positive() {
        for class in CharacterClass::all() {
            let stats = class.base_stats();
            assert!(stats.hp > 0);
            assert!(stats.attack > 0);
            assert!(stats.defense >= 0);
            assert!(stats.mana >= 0);
            assert!(stats.speed > 0);
        }
    }

    #[test]
    fn test_specialization_requires_level() {
        let mut state = CharacterClassState::new(CharacterClass::Warrior);
        assert!(state.specialize(Specialization::Berserker).is_err());

        state.level = SPECIALIZATION_TIER1_LEVEL;
        assert!(state.specialize(Specialization::Berserker).is_ok());
    }

    #[test]
    fn test_advanced_spec_requires_base_spec() {
        let mut state = CharacterClassState::new(CharacterClass::Warrior);
        state.level = SPECIALIZATION_TIER2_LEVEL;

        assert!(state.advance_specialize(AdvancedSpecialization::BloodReaver).is_err());

        state.specialize(Specialization::Berserker).unwrap();
        assert!(state.advance_specialize(AdvancedSpecialization::BloodReaver).is_ok());
    }

    #[test]
    fn test_spec_belongs_to_class() {
        let mut state = CharacterClassState::new(CharacterClass::Warrior);
        state.level = SPECIALIZATION_TIER1_LEVEL;

        assert!(state.specialize(Specialization::Assassin).is_err());
        assert!(state.specialize(Specialization::Berserker).is_ok());
    }

    #[test]
    fn test_stats_scale_with_level() {
        let mut state = CharacterClassState::new(CharacterClass::Warrior);
        let base_stats = state.get_stats();

        state.level = 10;
        let scaled_stats = state.get_stats();

        assert!(scaled_stats.hp > base_stats.hp);
        assert!(scaled_stats.attack > base_stats.attack);
    }

    #[test]
    fn test_all_specs_have_abilities() {
        for class in CharacterClass::all() {
            for spec in class.available_specializations() {
                assert!(!spec.abilities().is_empty(), "{:?} has no abilities", spec);
                assert!(!spec.passives().is_empty(), "{:?} has no passives", spec);
            }
        }
    }
}
