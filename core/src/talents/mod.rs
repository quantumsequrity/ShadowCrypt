//! Talent/Skill Tree System
//!
//! This module implements a comprehensive talent tree system for each character class.
//! Players earn talent points on level up and can spend them to unlock passive bonuses
//! and active abilities.

use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use crate::classes::CharacterClass;
use crate::combat::StatusEffect;

/// Talent point cost constants
pub const TALENT_POINTS_PER_LEVEL: u32 = 1;
pub const BONUS_POINTS_AT_LEVELS: &[u32] = &[5, 10, 15, 20, 25, 30];

/// Unique identifier for each talent in the game
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum TalentId {
    // ========================================
    // WARRIOR TALENTS - Arms Tree
    // ========================================
    /// Tier 1
    WarriorWeaponMastery,      // +10% weapon damage
    WarriorIronSkin,           // +5 base defense
    WarriorBattleCry,          // Active: intimidate enemies, reducing their attack

    /// Tier 2 (requires 3 points in tree)
    WarriorCleaveEnhanced,     // Cleave hits in wider arc
    WarriorArmorPenetration,   // Attacks ignore 20% of enemy defense
    WarriorSecondWind,         // Heal 15% HP when dropping below 30%

    /// Tier 3 (requires 6 points in tree)
    WarriorBerserkerRage,      // Berserk lasts longer and gives more damage
    WarriorUnstoppable,        // Immune to stun and freeze
    WarriorVeteranReflexes,    // 15% chance to dodge attacks

    /// Tier 4 (requires 9 points in tree)
    WarriorDeathBlow,          // Execute enemies below 20% HP
    WarriorTitanStrength,      // +25% max HP, +15% attack
    WarriorWarlord,            // All combat stats +10%

    // ========================================
    // MAGE TALENTS - Arcane Tree
    // ========================================
    /// Tier 1
    MageArcaneAffinity,        // +15% spell damage
    MageManaWell,              // +20 max mana
    MageQuickCast,             // -10% mana cost on all spells

    /// Tier 2 (requires 3 points in tree)
    MageFireMastery,           // Fireball burns longer and deals more damage
    MageFrostMastery,          // Ice spells have higher freeze chance
    MageLightningMastery,      // Lightning chains to additional targets

    /// Tier 3 (requires 6 points in tree)
    MageManaShield,            // Take damage from mana before HP
    MageSpellEcho,             // 20% chance to cast spell twice
    MageArcaneResonance,       // Killing enemies restores mana

    /// Tier 4 (requires 9 points in tree)
    MageMeteorStrike,          // Active: devastating AoE attack
    MageTimeWarp,              // Active: slow all enemies for 5 turns
    MageArchmage,              // +30% spell damage, +50 max mana

    // ========================================
    // ROGUE TALENTS - Shadow Tree
    // ========================================
    /// Tier 1
    RogueDaggerMastery,        // +20% dagger damage
    RogueShadowWalk,           // Move silently, enemies less likely to detect
    RogueQuickFingers,         // Find more gold and items

    /// Tier 2 (requires 3 points in tree)
    RogueCriticalStrike,       // +15% critical hit chance
    RoguePoisonExpert,         // Poisons deal double damage
    RogueEvasion,              // +20% dodge chance

    /// Tier 3 (requires 6 points in tree)
    RogueAssassinate,          // Massive damage from stealth
    RogueShadowClone,          // Create decoy when attacked
    RogueBladeFlurry,          // Attack all adjacent enemies

    /// Tier 4 (requires 9 points in tree)
    RogueDeathMark,            // Mark enemy to take +50% damage
    RogueShadowMaster,         // Invisibility lasts twice as long
    RogueKingslayer,           // +100% damage vs bosses

    // ========================================
    // PALADIN TALENTS - Holy Tree
    // ========================================
    /// Tier 1
    PaladinDivineStrength,     // +10% attack and defense
    PaladinHolyResilience,     // +15 max HP
    PaladinBlessedAura,        // Slow HP regeneration

    /// Tier 2 (requires 3 points in tree)
    PaladinHolySmiteEnhanced,  // Smite deals more damage to undead
    PaladinShieldOfFaith,      // Divine Shield absorbs more damage
    PaladinHealingHands,       // Holy Light heals more

    /// Tier 3 (requires 6 points in tree)
    PaladinRetribution,        // Reflect 25% of melee damage
    PaladinPurify,             // Remove all negative status effects
    PaladinConsecrationEnhanced, // Consecrated ground damages enemies

    /// Tier 4 (requires 9 points in tree)
    PaladinDivineIntervention, // Survive lethal damage once per level
    PaladinHolyAvenger,        // +30% damage vs undead and demons
    PaladinChampion,           // All healing +50%, all holy damage +25%

    // ========================================
    // RANGER TALENTS - Nature Tree
    // ========================================
    /// Tier 1
    RangerBowMastery,          // +15% ranged damage
    RangerEagleEyeEnhanced,    // Permanent increased vision range
    RangerNaturesBounty,       // Food heals more

    /// Tier 2 (requires 3 points in tree)
    RangerMultiShotEnhanced,   // Multi-shot hits 5 targets
    RangerTrapMastery,         // Traps deal double damage
    RangerSwiftFeet,           // +2 speed, move faster

    /// Tier 3 (requires 6 points in tree)
    RangerDeadlyAim,           // +25% critical damage
    RangerAnimalCompanion,     // Summon wolf companion
    RangerNaturalRemedy,       // Immune to poison

    /// Tier 4 (requires 9 points in tree)
    RangerPiercingShot,        // Arrows pierce through enemies
    RangerHuntersMark,         // Marked enemies take +30% damage
    RangerMasterHunter,        // +40% damage to beasts and bosses

    // ========================================
    // NECROMANCER TALENTS - Death Tree
    // ========================================
    /// Tier 1
    NecromancerDarkAffinity,   // +15% dark spell damage
    NecromancerSoulHarvest,    // Gain mana from kills
    NecromancerBoneArmor,      // Skeletons provide defense bonus

    /// Tier 2 (requires 3 points in tree)
    NecromancerLifeDrainEnhanced, // Life Drain heals more
    NecromancerCurseOfWeakness,   // Curse also reduces enemy defense
    NecromancerUndeadMastery,     // Raised skeletons are stronger

    /// Tier 3 (requires 6 points in tree)
    NecromancerDeathCoil,      // Damage enemies and heal self
    NecromancerArmyOfDead,     // Raise multiple skeletons
    NecromancerPlagueSpreader, // Curses spread to nearby enemies

    /// Tier 4 (requires 9 points in tree)
    NecromancerSoulReaper,     // Instant kill enemies below 15% HP
    NecromancerLichForm,       // Transform into lich, massive bonuses
    NecromancerDeathLord,      // +50% undead damage, unlimited minions
}

impl TalentId {
    /// Returns all talent IDs for a given class
    pub fn for_class(class: CharacterClass) -> Vec<Self> {
        match class {
            CharacterClass::Warrior => vec![
                Self::WarriorWeaponMastery, Self::WarriorIronSkin, Self::WarriorBattleCry,
                Self::WarriorCleaveEnhanced, Self::WarriorArmorPenetration, Self::WarriorSecondWind,
                Self::WarriorBerserkerRage, Self::WarriorUnstoppable, Self::WarriorVeteranReflexes,
                Self::WarriorDeathBlow, Self::WarriorTitanStrength, Self::WarriorWarlord,
            ],
            CharacterClass::Mage => vec![
                Self::MageArcaneAffinity, Self::MageManaWell, Self::MageQuickCast,
                Self::MageFireMastery, Self::MageFrostMastery, Self::MageLightningMastery,
                Self::MageManaShield, Self::MageSpellEcho, Self::MageArcaneResonance,
                Self::MageMeteorStrike, Self::MageTimeWarp, Self::MageArchmage,
            ],
            CharacterClass::Rogue => vec![
                Self::RogueDaggerMastery, Self::RogueShadowWalk, Self::RogueQuickFingers,
                Self::RogueCriticalStrike, Self::RoguePoisonExpert, Self::RogueEvasion,
                Self::RogueAssassinate, Self::RogueShadowClone, Self::RogueBladeFlurry,
                Self::RogueDeathMark, Self::RogueShadowMaster, Self::RogueKingslayer,
            ],
            CharacterClass::Paladin => vec![
                Self::PaladinDivineStrength, Self::PaladinHolyResilience, Self::PaladinBlessedAura,
                Self::PaladinHolySmiteEnhanced, Self::PaladinShieldOfFaith, Self::PaladinHealingHands,
                Self::PaladinRetribution, Self::PaladinPurify, Self::PaladinConsecrationEnhanced,
                Self::PaladinDivineIntervention, Self::PaladinHolyAvenger, Self::PaladinChampion,
            ],
            CharacterClass::Ranger => vec![
                Self::RangerBowMastery, Self::RangerEagleEyeEnhanced, Self::RangerNaturesBounty,
                Self::RangerMultiShotEnhanced, Self::RangerTrapMastery, Self::RangerSwiftFeet,
                Self::RangerDeadlyAim, Self::RangerAnimalCompanion, Self::RangerNaturalRemedy,
                Self::RangerPiercingShot, Self::RangerHuntersMark, Self::RangerMasterHunter,
            ],
            CharacterClass::Necromancer => vec![
                Self::NecromancerDarkAffinity, Self::NecromancerSoulHarvest, Self::NecromancerBoneArmor,
                Self::NecromancerLifeDrainEnhanced, Self::NecromancerCurseOfWeakness, Self::NecromancerUndeadMastery,
                Self::NecromancerDeathCoil, Self::NecromancerArmyOfDead, Self::NecromancerPlagueSpreader,
                Self::NecromancerSoulReaper, Self::NecromancerLichForm, Self::NecromancerDeathLord,
            ],
            CharacterClass::Cleric => vec![
                Self::PaladinDivineStrength, Self::PaladinHolyResilience, Self::PaladinBlessedAura,
                Self::PaladinHolySmiteEnhanced, Self::PaladinShieldOfFaith, Self::PaladinHealingHands,
                Self::PaladinRetribution, Self::PaladinPurify, Self::PaladinConsecrationEnhanced,
                Self::PaladinDivineIntervention, Self::PaladinHolyAvenger, Self::PaladinChampion,
            ],
            CharacterClass::Monk => vec![
                Self::WarriorWeaponMastery, Self::WarriorIronSkin, Self::WarriorBattleCry,
                Self::WarriorCleaveEnhanced, Self::WarriorArmorPenetration, Self::WarriorSecondWind,
                Self::WarriorBerserkerRage, Self::WarriorUnstoppable, Self::WarriorVeteranReflexes,
                Self::WarriorDeathBlow, Self::WarriorTitanStrength, Self::WarriorWarlord,
            ],
        }
    }

    /// Returns the tier (1-4) of this talent
    pub fn tier(&self) -> u32 {
        match self {
            // Tier 1 talents
            Self::WarriorWeaponMastery | Self::WarriorIronSkin | Self::WarriorBattleCry |
            Self::MageArcaneAffinity | Self::MageManaWell | Self::MageQuickCast |
            Self::RogueDaggerMastery | Self::RogueShadowWalk | Self::RogueQuickFingers |
            Self::PaladinDivineStrength | Self::PaladinHolyResilience | Self::PaladinBlessedAura |
            Self::RangerBowMastery | Self::RangerEagleEyeEnhanced | Self::RangerNaturesBounty |
            Self::NecromancerDarkAffinity | Self::NecromancerSoulHarvest | Self::NecromancerBoneArmor => 1,

            // Tier 2 talents
            Self::WarriorCleaveEnhanced | Self::WarriorArmorPenetration | Self::WarriorSecondWind |
            Self::MageFireMastery | Self::MageFrostMastery | Self::MageLightningMastery |
            Self::RogueCriticalStrike | Self::RoguePoisonExpert | Self::RogueEvasion |
            Self::PaladinHolySmiteEnhanced | Self::PaladinShieldOfFaith | Self::PaladinHealingHands |
            Self::RangerMultiShotEnhanced | Self::RangerTrapMastery | Self::RangerSwiftFeet |
            Self::NecromancerLifeDrainEnhanced | Self::NecromancerCurseOfWeakness | Self::NecromancerUndeadMastery => 2,

            // Tier 3 talents
            Self::WarriorBerserkerRage | Self::WarriorUnstoppable | Self::WarriorVeteranReflexes |
            Self::MageManaShield | Self::MageSpellEcho | Self::MageArcaneResonance |
            Self::RogueAssassinate | Self::RogueShadowClone | Self::RogueBladeFlurry |
            Self::PaladinRetribution | Self::PaladinPurify | Self::PaladinConsecrationEnhanced |
            Self::RangerDeadlyAim | Self::RangerAnimalCompanion | Self::RangerNaturalRemedy |
            Self::NecromancerDeathCoil | Self::NecromancerArmyOfDead | Self::NecromancerPlagueSpreader => 3,

            // Tier 4 talents
            Self::WarriorDeathBlow | Self::WarriorTitanStrength | Self::WarriorWarlord |
            Self::MageMeteorStrike | Self::MageTimeWarp | Self::MageArchmage |
            Self::RogueDeathMark | Self::RogueShadowMaster | Self::RogueKingslayer |
            Self::PaladinDivineIntervention | Self::PaladinHolyAvenger | Self::PaladinChampion |
            Self::RangerPiercingShot | Self::RangerHuntersMark | Self::RangerMasterHunter |
            Self::NecromancerSoulReaper | Self::NecromancerLichForm | Self::NecromancerDeathLord => 4,
        }
    }

    /// Returns the number of points required in the tree to unlock this tier
    pub fn points_required(&self) -> u32 {
        match self.tier() {
            1 => 0,
            2 => 3,
            3 => 6,
            4 => 9,
            _ => 0,
        }
    }

    /// Returns the point cost to learn this talent
    pub fn cost(&self) -> u32 {
        match self.tier() {
            1 => 1,
            2 => 2,
            3 => 2,
            4 => 3,
            _ => 1,
        }
    }

    /// Returns whether this is an active ability (vs passive)
    pub fn is_active(&self) -> bool {
        matches!(self,
            Self::WarriorBattleCry | Self::WarriorDeathBlow |
            Self::MageMeteorStrike | Self::MageTimeWarp |
            Self::RogueAssassinate | Self::RogueBladeFlurry | Self::RogueDeathMark |
            Self::PaladinPurify | Self::PaladinDivineIntervention |
            Self::RangerAnimalCompanion | Self::RangerPiercingShot |
            Self::NecromancerDeathCoil | Self::NecromancerArmyOfDead | Self::NecromancerSoulReaper
        )
    }
}

/// A talent definition with all its properties
#[derive(Clone, Debug)]
pub struct Talent {
    pub id: TalentId,
    pub name: &'static str,
    pub description: &'static str,
    pub effects: Vec<TalentEffect>,
}

impl Talent {
    /// Get talent information by ID
    pub fn get(id: TalentId) -> Self {
        match id {
            // ========== WARRIOR TALENTS ==========
            TalentId::WarriorWeaponMastery => Self {
                id,
                name: "Weapon Mastery",
                description: "Increases weapon damage by 10%",
                effects: vec![TalentEffect::DamageMultiplier(1.10)],
            },
            TalentId::WarriorIronSkin => Self {
                id,
                name: "Iron Skin",
                description: "Increases base defense by 5",
                effects: vec![TalentEffect::BonusDefense(5)],
            },
            TalentId::WarriorBattleCry => Self {
                id,
                name: "Battle Cry",
                description: "Active: Intimidate nearby enemies, reducing their attack by 25% for 5 turns",
                effects: vec![TalentEffect::ActiveAbility],
            },
            TalentId::WarriorCleaveEnhanced => Self {
                id,
                name: "Improved Cleave",
                description: "Cleave attacks hit in a wider arc and deal 20% more damage",
                effects: vec![TalentEffect::SkillEnhancement],
            },
            TalentId::WarriorArmorPenetration => Self {
                id,
                name: "Armor Penetration",
                description: "Your attacks ignore 20% of enemy defense",
                effects: vec![TalentEffect::ArmorPenetration(0.20)],
            },
            TalentId::WarriorSecondWind => Self {
                id,
                name: "Second Wind",
                description: "Heal 15% of max HP when dropping below 30% health (once per combat)",
                effects: vec![TalentEffect::TriggerHeal { threshold: 30, heal_percent: 15 }],
            },
            TalentId::WarriorBerserkerRage => Self {
                id,
                name: "Berserker Rage",
                description: "Berserk lasts 5 additional turns and grants 25% more damage",
                effects: vec![TalentEffect::SkillEnhancement],
            },
            TalentId::WarriorUnstoppable => Self {
                id,
                name: "Unstoppable",
                description: "Immune to stun and freeze effects",
                effects: vec![
                    TalentEffect::StatusImmunity(StatusEffect::Stun),
                    TalentEffect::StatusImmunity(StatusEffect::Freeze),
                ],
            },
            TalentId::WarriorVeteranReflexes => Self {
                id,
                name: "Veteran's Reflexes",
                description: "15% chance to dodge incoming attacks",
                effects: vec![TalentEffect::DodgeChance(15)],
            },
            TalentId::WarriorDeathBlow => Self {
                id,
                name: "Death Blow",
                description: "Active: Instantly execute enemies below 20% health",
                effects: vec![TalentEffect::ExecuteThreshold(20)],
            },
            TalentId::WarriorTitanStrength => Self {
                id,
                name: "Titan's Strength",
                description: "Increases max HP by 25% and attack by 15%",
                effects: vec![
                    TalentEffect::MaxHpMultiplier(1.25),
                    TalentEffect::DamageMultiplier(1.15),
                ],
            },
            TalentId::WarriorWarlord => Self {
                id,
                name: "Warlord",
                description: "All combat stats increased by 10%",
                effects: vec![
                    TalentEffect::DamageMultiplier(1.10),
                    TalentEffect::DefenseMultiplier(1.10),
                    TalentEffect::MaxHpMultiplier(1.10),
                ],
            },

            // ========== MAGE TALENTS ==========
            TalentId::MageArcaneAffinity => Self {
                id,
                name: "Arcane Affinity",
                description: "Increases spell damage by 15%",
                effects: vec![TalentEffect::SpellDamageMultiplier(1.15)],
            },
            TalentId::MageManaWell => Self {
                id,
                name: "Mana Well",
                description: "Increases maximum mana by 20",
                effects: vec![TalentEffect::BonusMana(20)],
            },
            TalentId::MageQuickCast => Self {
                id,
                name: "Quick Cast",
                description: "Reduces mana cost of all spells by 10%",
                effects: vec![TalentEffect::ManaCostReduction(0.10)],
            },
            TalentId::MageFireMastery => Self {
                id,
                name: "Fire Mastery",
                description: "Fireball burns longer and deals 25% more damage",
                effects: vec![TalentEffect::ElementalMastery { element: Element::Fire, bonus: 0.25 }],
            },
            TalentId::MageFrostMastery => Self {
                id,
                name: "Frost Mastery",
                description: "Ice spells have 30% higher freeze chance and deal 20% more damage",
                effects: vec![TalentEffect::ElementalMastery { element: Element::Ice, bonus: 0.20 }],
            },
            TalentId::MageLightningMastery => Self {
                id,
                name: "Lightning Mastery",
                description: "Lightning chains to 2 additional targets",
                effects: vec![TalentEffect::ChainTargets(2)],
            },
            TalentId::MageManaShield => Self {
                id,
                name: "Mana Shield",
                description: "Take 50% of damage from mana before HP when mana > 20",
                effects: vec![TalentEffect::ManaShield { absorption: 0.50 }],
            },
            TalentId::MageSpellEcho => Self {
                id,
                name: "Spell Echo",
                description: "20% chance to cast any spell twice",
                effects: vec![TalentEffect::SpellEchoChance(20)],
            },
            TalentId::MageArcaneResonance => Self {
                id,
                name: "Arcane Resonance",
                description: "Restore 5 mana for each enemy killed",
                effects: vec![TalentEffect::ManaOnKill(5)],
            },
            TalentId::MageMeteorStrike => Self {
                id,
                name: "Meteor Strike",
                description: "Active: Call down a devastating meteor dealing massive AoE damage",
                effects: vec![TalentEffect::ActiveAbility],
            },
            TalentId::MageTimeWarp => Self {
                id,
                name: "Time Warp",
                description: "Active: Slow all enemies for 5 turns, reducing their speed by 50%",
                effects: vec![TalentEffect::ActiveAbility],
            },
            TalentId::MageArchmage => Self {
                id,
                name: "Archmage",
                description: "Spell damage +30%, max mana +50",
                effects: vec![
                    TalentEffect::SpellDamageMultiplier(1.30),
                    TalentEffect::BonusMana(50),
                ],
            },

            // ========== ROGUE TALENTS ==========
            TalentId::RogueDaggerMastery => Self {
                id,
                name: "Dagger Mastery",
                description: "Increases dagger damage by 20%",
                effects: vec![TalentEffect::DamageMultiplier(1.20)],
            },
            TalentId::RogueShadowWalk => Self {
                id,
                name: "Shadow Walk",
                description: "Enemies are 30% less likely to detect you",
                effects: vec![TalentEffect::StealthBonus(30)],
            },
            TalentId::RogueQuickFingers => Self {
                id,
                name: "Quick Fingers",
                description: "Find 25% more gold and have better item drop rates",
                effects: vec![TalentEffect::GoldBonus(0.25), TalentEffect::ItemFindBonus(0.15)],
            },
            TalentId::RogueCriticalStrike => Self {
                id,
                name: "Critical Strike",
                description: "Increases critical hit chance by 15%",
                effects: vec![TalentEffect::CritChance(15)],
            },
            TalentId::RoguePoisonExpert => Self {
                id,
                name: "Poison Expert",
                description: "Poison effects deal double damage",
                effects: vec![TalentEffect::PoisonMultiplier(2.0)],
            },
            TalentId::RogueEvasion => Self {
                id,
                name: "Evasion",
                description: "20% chance to dodge incoming attacks",
                effects: vec![TalentEffect::DodgeChance(20)],
            },
            TalentId::RogueAssassinate => Self {
                id,
                name: "Assassinate",
                description: "Active: Deal 300% damage when attacking from stealth",
                effects: vec![TalentEffect::StealthDamageMultiplier(3.0)],
            },
            TalentId::RogueShadowClone => Self {
                id,
                name: "Shadow Clone",
                description: "When hit, 25% chance to create a decoy and turn invisible",
                effects: vec![TalentEffect::ShadowCloneChance(25)],
            },
            TalentId::RogueBladeFlurry => Self {
                id,
                name: "Blade Flurry",
                description: "Active: Attack all adjacent enemies at once",
                effects: vec![TalentEffect::ActiveAbility],
            },
            TalentId::RogueDeathMark => Self {
                id,
                name: "Death Mark",
                description: "Active: Mark an enemy to take 50% increased damage for 10 turns",
                effects: vec![TalentEffect::ActiveAbility],
            },
            TalentId::RogueShadowMaster => Self {
                id,
                name: "Shadow Master",
                description: "Invisibility duration is doubled",
                effects: vec![TalentEffect::InvisibilityDurationMultiplier(2.0)],
            },
            TalentId::RogueKingslayer => Self {
                id,
                name: "Kingslayer",
                description: "Deal 100% increased damage to bosses",
                effects: vec![TalentEffect::BossDamageMultiplier(2.0)],
            },

            // ========== PALADIN TALENTS ==========
            TalentId::PaladinDivineStrength => Self {
                id,
                name: "Divine Strength",
                description: "Increases attack and defense by 10%",
                effects: vec![
                    TalentEffect::DamageMultiplier(1.10),
                    TalentEffect::DefenseMultiplier(1.10),
                ],
            },
            TalentId::PaladinHolyResilience => Self {
                id,
                name: "Holy Resilience",
                description: "Increases max HP by 15",
                effects: vec![TalentEffect::BonusHp(15)],
            },
            TalentId::PaladinBlessedAura => Self {
                id,
                name: "Blessed Aura",
                description: "Regenerate 1 HP every 3 turns",
                effects: vec![TalentEffect::HpRegen { amount: 1, interval: 3 }],
            },
            TalentId::PaladinHolySmiteEnhanced => Self {
                id,
                name: "Empowered Smite",
                description: "Smite deals 50% more damage to undead and demons",
                effects: vec![TalentEffect::HolyDamageBonus(0.50)],
            },
            TalentId::PaladinShieldOfFaith => Self {
                id,
                name: "Shield of Faith",
                description: "Divine Shield absorbs 2 additional hits",
                effects: vec![TalentEffect::ShieldCharges(2)],
            },
            TalentId::PaladinHealingHands => Self {
                id,
                name: "Healing Hands",
                description: "Holy Light heals 50% more",
                effects: vec![TalentEffect::HealingMultiplier(1.50)],
            },
            TalentId::PaladinRetribution => Self {
                id,
                name: "Retribution",
                description: "Reflect 25% of melee damage back to attackers",
                effects: vec![TalentEffect::DamageReflect(0.25)],
            },
            TalentId::PaladinPurify => Self {
                id,
                name: "Purify",
                description: "Active: Remove all negative status effects",
                effects: vec![TalentEffect::ActiveAbility],
            },
            TalentId::PaladinConsecrationEnhanced => Self {
                id,
                name: "Sacred Ground",
                description: "Consecrated ground damages enemies standing on it",
                effects: vec![TalentEffect::ConsecratedDamage(5)],
            },
            TalentId::PaladinDivineIntervention => Self {
                id,
                name: "Divine Intervention",
                description: "Once per level, survive lethal damage with 1 HP",
                effects: vec![TalentEffect::DeathSave],
            },
            TalentId::PaladinHolyAvenger => Self {
                id,
                name: "Holy Avenger",
                description: "Deal 30% more damage to undead and demons",
                effects: vec![TalentEffect::HolyDamageBonus(0.30)],
            },
            TalentId::PaladinChampion => Self {
                id,
                name: "Champion of Light",
                description: "All healing +50%, holy damage +25%",
                effects: vec![
                    TalentEffect::HealingMultiplier(1.50),
                    TalentEffect::HolyDamageBonus(0.25),
                ],
            },

            // ========== RANGER TALENTS ==========
            TalentId::RangerBowMastery => Self {
                id,
                name: "Bow Mastery",
                description: "Increases ranged damage by 15%",
                effects: vec![TalentEffect::RangedDamageMultiplier(1.15)],
            },
            TalentId::RangerEagleEyeEnhanced => Self {
                id,
                name: "Enhanced Eagle Eye",
                description: "Permanently increases vision range by 2",
                effects: vec![TalentEffect::VisionRange(2)],
            },
            TalentId::RangerNaturesBounty => Self {
                id,
                name: "Nature's Bounty",
                description: "Food heals 50% more hunger",
                effects: vec![TalentEffect::FoodBonus(0.50)],
            },
            TalentId::RangerMultiShotEnhanced => Self {
                id,
                name: "Improved Multi-Shot",
                description: "Multi-shot hits 5 targets instead of 3",
                effects: vec![TalentEffect::SkillEnhancement],
            },
            TalentId::RangerTrapMastery => Self {
                id,
                name: "Trap Mastery",
                description: "Your traps deal double damage and have a chance to stun",
                effects: vec![TalentEffect::TrapDamageMultiplier(2.0)],
            },
            TalentId::RangerSwiftFeet => Self {
                id,
                name: "Swift Feet",
                description: "Increases movement speed by 2",
                effects: vec![TalentEffect::BonusSpeed(2)],
            },
            TalentId::RangerDeadlyAim => Self {
                id,
                name: "Deadly Aim",
                description: "Critical hits deal 25% more damage",
                effects: vec![TalentEffect::CritDamageBonus(0.25)],
            },
            TalentId::RangerAnimalCompanion => Self {
                id,
                name: "Animal Companion",
                description: "Active: Summon a wolf companion to fight alongside you",
                effects: vec![TalentEffect::SummonCompanion],
            },
            TalentId::RangerNaturalRemedy => Self {
                id,
                name: "Natural Remedy",
                description: "Immune to poison effects",
                effects: vec![TalentEffect::StatusImmunity(StatusEffect::Poison)],
            },
            TalentId::RangerPiercingShot => Self {
                id,
                name: "Piercing Shot",
                description: "Active: Fire an arrow that pierces through all enemies in a line",
                effects: vec![TalentEffect::ActiveAbility],
            },
            TalentId::RangerHuntersMark => Self {
                id,
                name: "Hunter's Mark",
                description: "Marked enemies take 30% increased damage from all sources",
                effects: vec![TalentEffect::MarkDamageBonus(0.30)],
            },
            TalentId::RangerMasterHunter => Self {
                id,
                name: "Master Hunter",
                description: "Deal 40% more damage to beasts and bosses",
                effects: vec![
                    TalentEffect::BeastDamageBonus(0.40),
                    TalentEffect::BossDamageMultiplier(1.40),
                ],
            },

            // ========== NECROMANCER TALENTS ==========
            TalentId::NecromancerDarkAffinity => Self {
                id,
                name: "Dark Affinity",
                description: "Increases dark spell damage by 15%",
                effects: vec![TalentEffect::SpellDamageMultiplier(1.15)],
            },
            TalentId::NecromancerSoulHarvest => Self {
                id,
                name: "Soul Harvest",
                description: "Restore 8 mana when killing an enemy",
                effects: vec![TalentEffect::ManaOnKill(8)],
            },
            TalentId::NecromancerBoneArmor => Self {
                id,
                name: "Bone Armor",
                description: "Each active skeleton increases your defense by 2",
                effects: vec![TalentEffect::MinionDefenseBonus(2)],
            },
            TalentId::NecromancerLifeDrainEnhanced => Self {
                id,
                name: "Improved Life Drain",
                description: "Life Drain heals 50% more",
                effects: vec![TalentEffect::LifeStealBonus(0.50)],
            },
            TalentId::NecromancerCurseOfWeakness => Self {
                id,
                name: "Curse of Weakness",
                description: "Curse also reduces enemy defense by 25%",
                effects: vec![TalentEffect::CurseDefenseReduction(0.25)],
            },
            TalentId::NecromancerUndeadMastery => Self {
                id,
                name: "Undead Mastery",
                description: "Raised skeletons have 50% more HP and damage",
                effects: vec![TalentEffect::MinionPowerBonus(0.50)],
            },
            TalentId::NecromancerDeathCoil => Self {
                id,
                name: "Death Coil",
                description: "Active: Deal damage to an enemy and heal yourself",
                effects: vec![TalentEffect::ActiveAbility],
            },
            TalentId::NecromancerArmyOfDead => Self {
                id,
                name: "Army of the Dead",
                description: "Active: Raise up to 3 skeletons at once",
                effects: vec![TalentEffect::ActiveAbility],
            },
            TalentId::NecromancerPlagueSpreader => Self {
                id,
                name: "Plague Spreader",
                description: "Curses spread to nearby enemies when the target dies",
                effects: vec![TalentEffect::CurseSpread],
            },
            TalentId::NecromancerSoulReaper => Self {
                id,
                name: "Soul Reaper",
                description: "Active: Instantly kill enemies below 15% health",
                effects: vec![TalentEffect::ExecuteThreshold(15)],
            },
            TalentId::NecromancerLichForm => Self {
                id,
                name: "Lich Form",
                description: "Transform into a lich: +50% spell damage, +100 max mana, immune to death effects",
                effects: vec![
                    TalentEffect::SpellDamageMultiplier(1.50),
                    TalentEffect::BonusMana(100),
                    TalentEffect::StatusImmunity(StatusEffect::Weakness),
                ],
            },
            TalentId::NecromancerDeathLord => Self {
                id,
                name: "Death Lord",
                description: "Undead minions deal 50% more damage, no limit on active minions",
                effects: vec![
                    TalentEffect::MinionPowerBonus(0.50),
                    TalentEffect::UnlimitedMinions,
                ],
            },
        }
    }
}

/// Effects that talents can provide
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TalentEffect {
    // Stat bonuses
    BonusHp(i32),
    BonusMana(i32),
    BonusAttack(i32),
    BonusDefense(i32),
    BonusSpeed(i32),

    // Multipliers
    DamageMultiplier(f32),
    SpellDamageMultiplier(f32),
    RangedDamageMultiplier(f32),
    DefenseMultiplier(f32),
    MaxHpMultiplier(f32),
    HealingMultiplier(f32),

    // Combat mechanics
    CritChance(i32),           // Percent
    CritDamageBonus(f32),      // Additional multiplier
    DodgeChance(i32),          // Percent
    ArmorPenetration(f32),     // Percent of defense ignored
    LifeStealBonus(f32),       // Additional life steal percent
    DamageReflect(f32),        // Percent of damage reflected

    // Status effects
    StatusImmunity(StatusEffect),
    PoisonMultiplier(f32),

    // Mana
    ManaCostReduction(f32),
    ManaOnKill(i32),
    ManaShield { absorption: f32 },

    // Elemental
    ElementalMastery { element: Element, bonus: f32 },
    ChainTargets(i32),

    // Triggers
    TriggerHeal { threshold: i32, heal_percent: i32 },
    HpRegen { amount: i32, interval: i32 },
    ExecuteThreshold(i32),
    SpellEchoChance(i32),
    ShadowCloneChance(i32),

    // Special
    ActiveAbility,
    SkillEnhancement,
    DeathSave,
    SummonCompanion,
    UnlimitedMinions,
    CurseSpread,

    // Stealth
    StealthBonus(i32),
    StealthDamageMultiplier(f32),
    InvisibilityDurationMultiplier(f32),

    // Enemy-specific
    BossDamageMultiplier(f32),
    HolyDamageBonus(f32),
    BeastDamageBonus(f32),
    MarkDamageBonus(f32),

    // Minions
    MinionDefenseBonus(i32),
    MinionPowerBonus(f32),

    // Miscellaneous
    GoldBonus(f32),
    ItemFindBonus(f32),
    VisionRange(i32),
    FoodBonus(f32),
    ShieldCharges(i32),
    TrapDamageMultiplier(f32),
    ConsecratedDamage(i32),
    CurseDefenseReduction(f32),
}

/// Elemental types for mastery bonuses
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Element {
    Fire,
    Ice,
    Lightning,
    Holy,
    Dark,
}

/// A talent tree containing all talents for a class organized by tier
#[derive(Clone, Debug)]
pub struct TalentTree {
    pub class: CharacterClass,
    pub talents: Vec<Talent>,
}

impl TalentTree {
    /// Create a new talent tree for the given class
    pub fn new(class: CharacterClass) -> Self {
        let talent_ids = TalentId::for_class(class);
        let talents = talent_ids.into_iter().map(Talent::get).collect();
        Self { class, talents }
    }

    /// Get talents by tier
    pub fn tier(&self, tier: u32) -> Vec<&Talent> {
        self.talents.iter().filter(|t| t.id.tier() == tier).collect()
    }

    /// Get all talent IDs in this tree
    pub fn talent_ids(&self) -> Vec<TalentId> {
        self.talents.iter().map(|t| t.id).collect()
    }
}

/// Player's talent progression state
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PlayerTalents {
    /// Total talent points available to spend
    pub available_points: u32,
    /// Total talent points ever earned
    pub total_points_earned: u32,
    /// Set of unlocked talent IDs
    pub unlocked: HashSet<TalentId>,
    /// Cooldowns for active abilities (turns remaining)
    pub cooldowns: HashMap<TalentId, u32>,
    /// Whether death save has been used this level
    pub death_save_used: bool,
    /// Whether second wind has been used this combat
    pub second_wind_used: bool,
    /// Currently marked enemy (for Death Mark)
    pub marked_enemy: Option<usize>,
    /// Wolf companion HP (if summoned)
    pub companion_hp: Option<i32>,
}

impl PlayerTalents {
    /// Create a new empty talent state
    pub fn new() -> Self {
        Self::default()
    }

    /// Award talent points for leveling up
    pub fn on_level_up(&mut self, new_level: u32) {
        let points = TALENT_POINTS_PER_LEVEL;
        let bonus = if BONUS_POINTS_AT_LEVELS.contains(&new_level) { 1 } else { 0 };
        self.available_points += points + bonus;
        self.total_points_earned += points + bonus;

        // Reset death save on level up
        self.death_save_used = false;
    }

    /// Check if a talent can be learned
    pub fn can_learn(&self, talent_id: TalentId, class: CharacterClass) -> bool {
        // Already learned
        if self.unlocked.contains(&talent_id) {
            return false;
        }

        // Must be correct class
        if !TalentId::for_class(class).contains(&talent_id) {
            return false;
        }

        // Must have enough points
        let talent = Talent::get(talent_id);
        if self.available_points < talent.id.cost() {
            return false;
        }

        // Must meet tier requirements
        let points_in_tree = self.points_in_tree(class);
        if points_in_tree < talent.id.points_required() {
            return false;
        }

        true
    }

    /// Learn a talent
    pub fn learn(&mut self, talent_id: TalentId, class: CharacterClass) -> Result<(), &'static str> {
        if !self.can_learn(talent_id, class) {
            return Err("Cannot learn this talent");
        }

        let cost = talent_id.cost();
        self.available_points -= cost;
        self.unlocked.insert(talent_id);
        Ok(())
    }

    /// Calculate total points spent in a class's talent tree
    pub fn points_in_tree(&self, class: CharacterClass) -> u32 {
        TalentId::for_class(class)
            .iter()
            .filter(|id| self.unlocked.contains(id))
            .map(|id| id.cost())
            .sum()
    }

    /// Check if a talent is unlocked
    pub fn has_talent(&self, talent_id: TalentId) -> bool {
        self.unlocked.contains(&talent_id)
    }

    /// Get all unlocked talents
    pub fn get_unlocked(&self) -> Vec<TalentId> {
        self.unlocked.iter().copied().collect()
    }

    /// Get all active abilities that are unlocked and off cooldown
    pub fn available_actives(&self) -> Vec<TalentId> {
        self.unlocked
            .iter()
            .filter(|id| id.is_active() && !self.cooldowns.contains_key(id))
            .copied()
            .collect()
    }

    /// Use an active ability (sets cooldown)
    pub fn use_active(&mut self, talent_id: TalentId) {
        if talent_id.is_active() {
            let cooldown = match talent_id.tier() {
                4 => 15,
                3 => 10,
                _ => 5,
            };
            self.cooldowns.insert(talent_id, cooldown);
        }
    }

    /// Tick cooldowns (call each turn)
    pub fn tick_cooldowns(&mut self) {
        let to_remove: Vec<TalentId> = self.cooldowns
            .iter_mut()
            .filter_map(|(id, cd)| {
                *cd = cd.saturating_sub(1);
                if *cd == 0 { Some(*id) } else { None }
            })
            .collect();

        for id in to_remove {
            self.cooldowns.remove(&id);
        }
    }

    /// Reset combat-specific states
    pub fn on_combat_end(&mut self) {
        self.second_wind_used = false;
        self.marked_enemy = None;
    }

    /// Get accumulated stat bonuses from all unlocked talents
    pub fn get_stat_bonuses(&self) -> TalentStatBonuses {
        let mut bonuses = TalentStatBonuses::default();

        for talent_id in &self.unlocked {
            let talent = Talent::get(*talent_id);
            for effect in &talent.effects {
                match effect {
                    TalentEffect::BonusHp(v) => bonuses.bonus_hp += v,
                    TalentEffect::BonusMana(v) => bonuses.bonus_mana += v,
                    TalentEffect::BonusAttack(v) => bonuses.bonus_attack += v,
                    TalentEffect::BonusDefense(v) => bonuses.bonus_defense += v,
                    TalentEffect::BonusSpeed(v) => bonuses.bonus_speed += v,
                    TalentEffect::DamageMultiplier(v) => bonuses.damage_multiplier *= v,
                    TalentEffect::SpellDamageMultiplier(v) => bonuses.spell_damage_multiplier *= v,
                    TalentEffect::RangedDamageMultiplier(v) => bonuses.ranged_damage_multiplier *= v,
                    TalentEffect::DefenseMultiplier(v) => bonuses.defense_multiplier *= v,
                    TalentEffect::MaxHpMultiplier(v) => bonuses.max_hp_multiplier *= v,
                    TalentEffect::HealingMultiplier(v) => bonuses.healing_multiplier *= v,
                    TalentEffect::CritChance(v) => bonuses.crit_chance += v,
                    TalentEffect::CritDamageBonus(v) => bonuses.crit_damage_bonus += v,
                    TalentEffect::DodgeChance(v) => bonuses.dodge_chance += v,
                    TalentEffect::ArmorPenetration(v) => bonuses.armor_penetration += v,
                    TalentEffect::ManaCostReduction(v) => bonuses.mana_cost_reduction += v,
                    TalentEffect::ManaOnKill(v) => bonuses.mana_on_kill += v,
                    TalentEffect::LifeStealBonus(v) => bonuses.life_steal_bonus += v,
                    TalentEffect::DamageReflect(v) => bonuses.damage_reflect += v,
                    TalentEffect::VisionRange(v) => bonuses.vision_range += v,
                    TalentEffect::GoldBonus(v) => bonuses.gold_bonus += v,
                    TalentEffect::ItemFindBonus(v) => bonuses.item_find_bonus += v,
                    TalentEffect::FoodBonus(v) => bonuses.food_bonus += v,
                    TalentEffect::BossDamageMultiplier(v) => bonuses.boss_damage_multiplier *= v,
                    TalentEffect::HolyDamageBonus(v) => bonuses.holy_damage_bonus += v,
                    TalentEffect::MinionDefenseBonus(v) => bonuses.minion_defense_bonus += v,
                    TalentEffect::MinionPowerBonus(v) => bonuses.minion_power_bonus += v,
                    TalentEffect::StatusImmunity(status) => {
                        bonuses.status_immunities.push(*status);
                    }
                    _ => {}
                }
            }
        }

        bonuses
    }
}

/// Accumulated stat bonuses from talents
#[derive(Clone, Debug, Default)]
pub struct TalentStatBonuses {
    // Flat bonuses
    pub bonus_hp: i32,
    pub bonus_mana: i32,
    pub bonus_attack: i32,
    pub bonus_defense: i32,
    pub bonus_speed: i32,
    pub vision_range: i32,
    pub mana_on_kill: i32,
    pub minion_defense_bonus: i32,

    // Multipliers (start at 1.0)
    pub damage_multiplier: f32,
    pub spell_damage_multiplier: f32,
    pub ranged_damage_multiplier: f32,
    pub defense_multiplier: f32,
    pub max_hp_multiplier: f32,
    pub healing_multiplier: f32,
    pub boss_damage_multiplier: f32,
    pub minion_power_bonus: f32,

    // Percentages
    pub crit_chance: i32,
    pub crit_damage_bonus: f32,
    pub dodge_chance: i32,
    pub armor_penetration: f32,
    pub mana_cost_reduction: f32,
    pub life_steal_bonus: f32,
    pub damage_reflect: f32,
    pub gold_bonus: f32,
    pub item_find_bonus: f32,
    pub food_bonus: f32,
    pub holy_damage_bonus: f32,

    // Status immunities
    pub status_immunities: Vec<StatusEffect>,
}

impl TalentStatBonuses {
    pub fn new() -> Self {
        Self {
            damage_multiplier: 1.0,
            spell_damage_multiplier: 1.0,
            ranged_damage_multiplier: 1.0,
            defense_multiplier: 1.0,
            max_hp_multiplier: 1.0,
            healing_multiplier: 1.0,
            boss_damage_multiplier: 1.0,
            minion_power_bonus: 1.0,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_talent_tree_creation() {
        for class in CharacterClass::all() {
            let tree = TalentTree::new(class);
            assert_eq!(tree.talents.len(), 12, "Class {:?} should have 12 talents", class);

            // Check tier distribution: 3 per tier
            for tier in 1..=4 {
                assert_eq!(
                    tree.tier(tier).len(), 3,
                    "Class {:?} tier {} should have 3 talents", class, tier
                );
            }
        }
    }

    #[test]
    fn test_talent_learning() {
        let mut talents = PlayerTalents::new();
        talents.available_points = 5;

        // Should be able to learn tier 1 talent
        assert!(talents.can_learn(TalentId::WarriorWeaponMastery, CharacterClass::Warrior));

        // Should NOT be able to learn wrong class talent
        assert!(!talents.can_learn(TalentId::MageArcaneAffinity, CharacterClass::Warrior));

        // Learn a talent
        talents.learn(TalentId::WarriorWeaponMastery, CharacterClass::Warrior).unwrap();
        assert!(talents.has_talent(TalentId::WarriorWeaponMastery));
        assert_eq!(talents.available_points, 4); // Cost 1 for tier 1

        // Can't learn same talent twice
        assert!(!talents.can_learn(TalentId::WarriorWeaponMastery, CharacterClass::Warrior));
    }

    #[test]
    fn test_tier_requirements() {
        let mut talents = PlayerTalents::new();
        talents.available_points = 10;

        // Cannot learn tier 2 without 3 points in tree
        assert!(!talents.can_learn(TalentId::WarriorCleaveEnhanced, CharacterClass::Warrior));

        // Learn 3 tier 1 talents
        talents.learn(TalentId::WarriorWeaponMastery, CharacterClass::Warrior).unwrap();
        talents.learn(TalentId::WarriorIronSkin, CharacterClass::Warrior).unwrap();
        talents.learn(TalentId::WarriorBattleCry, CharacterClass::Warrior).unwrap();

        // Now can learn tier 2
        assert!(talents.can_learn(TalentId::WarriorCleaveEnhanced, CharacterClass::Warrior));
    }

    #[test]
    fn test_level_up_points() {
        let mut talents = PlayerTalents::new();

        talents.on_level_up(2);
        assert_eq!(talents.available_points, 1);

        // Level 5 gives bonus point
        talents.on_level_up(5);
        assert_eq!(talents.available_points, 3); // 1 + 1 (bonus)

        talents.on_level_up(6);
        assert_eq!(talents.available_points, 4);
    }

    #[test]
    fn test_stat_bonuses() {
        let mut talents = PlayerTalents::new();
        talents.available_points = 10;

        talents.learn(TalentId::WarriorWeaponMastery, CharacterClass::Warrior).unwrap();
        talents.learn(TalentId::WarriorIronSkin, CharacterClass::Warrior).unwrap();

        let bonuses = talents.get_stat_bonuses();
        assert!((bonuses.damage_multiplier - 1.10).abs() < 0.001);
        assert_eq!(bonuses.bonus_defense, 5);
    }

    #[test]
    fn test_cooldowns() {
        let mut talents = PlayerTalents::new();
        talents.unlocked.insert(TalentId::WarriorBattleCry);

        talents.use_active(TalentId::WarriorBattleCry);
        assert!(talents.cooldowns.contains_key(&TalentId::WarriorBattleCry));

        // Tick down cooldown
        for _ in 0..5 {
            talents.tick_cooldowns();
        }

        assert!(!talents.cooldowns.contains_key(&TalentId::WarriorBattleCry));
    }
}
