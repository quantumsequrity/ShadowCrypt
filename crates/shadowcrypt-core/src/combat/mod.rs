//! Combat system for the ShadowCrypt roguelike
//!
//! This module defines status effects, enemies, enemy types, combat mechanics,
//! critical hits, dodge chances, combo attacks, varied enemy abilities,
//! elite/champion monster variants with affixes, and champion pack mechanics.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use rand::prelude::*;

// ============================================================================
// ELITE/CHAMPION MONSTER SYSTEM
// ============================================================================

/// Monster rarity tier - determines stat bonuses and affix count
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum MonsterRarity {
    /// Normal monster - no bonuses, no affixes
    Normal,
    /// Elite monster - +50% stats, 1-2 affixes, glows slightly
    Elite,
    /// Champion monster - +100% stats, 2-3 affixes, leader of a pack
    Champion,
    /// Rare monster - +75% stats, 1-2 powerful affixes, unique appearance
    Rare,
    /// Legendary monster - +150% stats, 3-4 affixes, extremely dangerous
    Legendary,
}

impl MonsterRarity {
    /// Returns the stat multiplier for this rarity
    pub fn stat_multiplier(&self) -> f32 {
        match self {
            Self::Normal => 1.0,
            Self::Elite => 1.5,
            Self::Champion => 2.0,
            Self::Rare => 1.75,
            Self::Legendary => 2.5,
        }
    }

    /// Returns the XP multiplier for this rarity
    pub fn xp_multiplier(&self) -> f32 {
        match self {
            Self::Normal => 1.0,
            Self::Elite => 2.5,
            Self::Champion => 4.0,
            Self::Rare => 3.5,
            Self::Legendary => 6.0,
        }
    }

    /// Returns the number of affixes for this rarity
    pub fn affix_count(&self) -> (usize, usize) {
        match self {
            Self::Normal => (0, 0),
            Self::Elite => (1, 2),
            Self::Champion => (2, 3),
            Self::Rare => (1, 2),
            Self::Legendary => (3, 4),
        }
    }

    /// Returns the display name prefix for this rarity
    pub fn name_prefix(&self) -> &'static str {
        match self {
            Self::Normal => "",
            Self::Elite => "Elite ",
            Self::Champion => "Champion ",
            Self::Rare => "Rare ",
            Self::Legendary => "Legendary ",
        }
    }

    /// Returns a color index for UI display
    pub fn color_index(&self) -> u8 {
        match self {
            Self::Normal => 2,   // White
            Self::Elite => 11,  // Yellow
            Self::Champion => 7, // Blue
            Self::Rare => 13,   // Magenta
            Self::Legendary => 3, // Red/Orange
        }
    }

    /// Returns the drop rate bonus for this rarity (percentage)
    pub fn drop_rate_bonus(&self) -> i32 {
        match self {
            Self::Normal => 0,
            Self::Elite => 25,
            Self::Champion => 50,
            Self::Rare => 75,
            Self::Legendary => 150,
        }
    }

    /// Roll for monster rarity based on dungeon level and luck
    pub fn roll(dungeon_level: u32, luck_bonus: f32, rng: &mut impl Rng) -> Self {
        let base_roll = rng.gen::<f32>();
        let level_bonus = (dungeon_level as f32 * 0.01).min(0.15);
        let roll = base_roll + level_bonus + luck_bonus;

        if roll > 0.995 {
            Self::Legendary
        } else if roll > 0.97 {
            Self::Rare
        } else if roll > 0.92 {
            Self::Champion
        } else if roll > 0.80 {
            Self::Elite
        } else {
            Self::Normal
        }
    }
}

/// Monster affixes that modify behavior and grant special abilities
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum MonsterAffix {
    // Offensive Affixes
    /// Heals for a percentage of damage dealt
    Vampiric,
    /// Can teleport to the player or away when low HP
    Teleporting,
    /// Explodes on death, dealing AoE damage
    Explosive,
    /// Attacks apply extra fire damage and burning
    Molten,
    /// Attacks apply extra cold damage and slow
    Frozen,
    /// Attacks apply extra lightning damage and can chain
    Electrified,
    /// Attacks apply poison damage over time
    Venomous,
    /// Deals extra damage but moves slower
    Brutish,
    /// Attacks ignore a portion of armor
    ArmorPiercing,
    /// Deals massive damage on first hit from stealth
    Assassin,
    /// Attacks cause bleeding wounds
    Jagged,
    /// Attacks drain mana from the player
    ManaDrain,
    /// Attacks can curse the player
    Cursing,
    /// Has a chance to instantly kill low HP targets
    Executioner,

    // Defensive Affixes
    /// Takes reduced damage from all sources
    Shielded,
    /// Regenerates HP over time
    Regenerating,
    /// Reflects a portion of damage back to attacker
    Thorned,
    /// High evasion chance
    Elusive,
    /// Immune to crowd control effects
    Relentless,
    /// Creates a temporary shield when taking large hits
    AdaptiveArmor,
    /// Cannot be killed by a single hit (survives with 1 HP)
    Undying,
    /// Takes reduced damage from the front
    Fortified,
    /// Immune to fire damage
    FireImmune,
    /// Immune to cold damage
    ColdImmune,
    /// Immune to lightning damage
    LightningImmune,
    /// Immune to poison damage
    PoisonImmune,

    // Utility/Special Affixes
    /// Moves and attacks faster
    Haste,
    /// Can summon minions to fight alongside
    Summoner,
    /// Nearby allies deal more damage
    Warcry,
    /// Creates dangerous ground effects
    GroundEffect,
    /// Can go invisible temporarily
    Phasing,
    /// Creates copies of itself
    Illusionist,
    /// Steals buffs from the player
    Spellbreaker,
    /// Heals nearby allies
    Healer,
    /// Enrages when health is low, gaining massive damage
    Berserker,
    /// Teleports the player to itself
    Vortex,
    /// Periodically spawns minions from corpses
    Necromancer,
    /// Leaves a trail of damaging ground
    TrailBlazer,
    /// Attacks cause knockback
    Knockback,
    /// Can possess and empower nearby corpses
    Possessor,
}

impl MonsterAffix {
    /// Returns the display name of this affix
    pub fn name(&self) -> &'static str {
        match self {
            Self::Vampiric => "Vampiric",
            Self::Teleporting => "Teleporting",
            Self::Explosive => "Explosive",
            Self::Molten => "Molten",
            Self::Frozen => "Frozen",
            Self::Electrified => "Electrified",
            Self::Venomous => "Venomous",
            Self::Brutish => "Brutish",
            Self::ArmorPiercing => "Armor Piercing",
            Self::Assassin => "Assassin",
            Self::Jagged => "Jagged",
            Self::ManaDrain => "Mana Drain",
            Self::Cursing => "Cursing",
            Self::Executioner => "Executioner",
            Self::Shielded => "Shielded",
            Self::Regenerating => "Regenerating",
            Self::Thorned => "Thorned",
            Self::Elusive => "Elusive",
            Self::Relentless => "Relentless",
            Self::AdaptiveArmor => "Adaptive Armor",
            Self::Undying => "Undying",
            Self::Fortified => "Fortified",
            Self::FireImmune => "Fire Immune",
            Self::ColdImmune => "Cold Immune",
            Self::LightningImmune => "Lightning Immune",
            Self::PoisonImmune => "Poison Immune",
            Self::Haste => "Haste",
            Self::Summoner => "Summoner",
            Self::Warcry => "Warcry",
            Self::GroundEffect => "Ground Effect",
            Self::Phasing => "Phasing",
            Self::Illusionist => "Illusionist",
            Self::Spellbreaker => "Spellbreaker",
            Self::Healer => "Healer",
            Self::Berserker => "Berserker",
            Self::Vortex => "Vortex",
            Self::Necromancer => "Necromancer",
            Self::TrailBlazer => "Trail Blazer",
            Self::Knockback => "Knockback",
            Self::Possessor => "Possessor",
        }
    }

    /// Returns a description of what this affix does
    pub fn description(&self) -> &'static str {
        match self {
            Self::Vampiric => "Heals for 20% of damage dealt",
            Self::Teleporting => "Can teleport short distances",
            Self::Explosive => "Explodes on death, dealing heavy AoE damage",
            Self::Molten => "Attacks deal fire damage and apply burning",
            Self::Frozen => "Attacks deal cold damage and slow targets",
            Self::Electrified => "Attacks deal lightning damage that chains to nearby targets",
            Self::Venomous => "Attacks poison targets for damage over time",
            Self::Brutish => "Deals 40% more damage but moves 20% slower",
            Self::ArmorPiercing => "Ignores 50% of target's armor",
            Self::Assassin => "Deals triple damage on first attack from stealth",
            Self::Jagged => "Attacks cause bleeding for 4 turns",
            Self::ManaDrain => "Attacks drain 10% of target's mana",
            Self::Cursing => "Attacks have 30% chance to curse target",
            Self::Executioner => "Instantly kills targets below 10% HP",
            Self::Shielded => "Takes 25% reduced damage from all sources",
            Self::Regenerating => "Regenerates 2% max HP per turn",
            Self::Thorned => "Reflects 30% of melee damage back to attacker",
            Self::Elusive => "Has +30% dodge chance",
            Self::Relentless => "Immune to stun, freeze, and other crowd control",
            Self::AdaptiveArmor => "Gains temporary shield after taking large hits",
            Self::Undying => "Cannot be killed by a single hit; survives with 1 HP",
            Self::Fortified => "Takes 40% less damage from frontal attacks",
            Self::FireImmune => "Immune to fire damage",
            Self::ColdImmune => "Immune to cold damage",
            Self::LightningImmune => "Immune to lightning damage",
            Self::PoisonImmune => "Immune to poison damage",
            Self::Haste => "Moves and attacks 50% faster",
            Self::Summoner => "Periodically summons minions to fight",
            Self::Warcry => "Nearby allies deal 25% more damage",
            Self::GroundEffect => "Creates damaging ground effects",
            Self::Phasing => "Can become invisible for short periods",
            Self::Illusionist => "Creates mirror images of itself",
            Self::Spellbreaker => "Dispels buffs from targets on hit",
            Self::Healer => "Heals nearby allies for 5% max HP per turn",
            Self::Berserker => "Gains +75% damage when below 30% HP",
            Self::Vortex => "Can pull the player towards itself",
            Self::Necromancer => "Raises nearby corpses as minions",
            Self::TrailBlazer => "Leaves a damaging trail when moving",
            Self::Knockback => "Attacks push targets back",
            Self::Possessor => "Can empower corpses into stronger minions",
        }
    }

    /// Returns the color index for this affix (for UI)
    pub fn color_index(&self) -> u8 {
        match self {
            Self::Vampiric => 4,       // Dark Red
            Self::Teleporting => 13,   // Magenta
            Self::Explosive => 3,      // Red
            Self::Molten => 6,         // Orange
            Self::Frozen => 9,         // Cyan
            Self::Electrified => 11,   // Yellow
            Self::Venomous => 5,       // Green
            Self::Brutish => 4,        // Dark Red
            Self::ArmorPiercing => 1,  // Grey
            Self::Assassin => 0,       // Dark Grey
            Self::Jagged => 4,         // Dark Red
            Self::ManaDrain => 7,      // Blue
            Self::Cursing => 14,       // Dark Magenta
            Self::Executioner => 3,    // Red
            Self::Shielded => 2,       // White
            Self::Regenerating => 5,   // Green
            Self::Thorned => 6,        // Green
            Self::Elusive => 9,        // Cyan
            Self::Relentless => 11,    // Yellow
            Self::AdaptiveArmor => 7,  // Blue
            Self::Undying => 14,       // Dark Magenta
            Self::Fortified => 1,      // Grey
            Self::FireImmune => 3,     // Red
            Self::ColdImmune => 9,     // Cyan
            Self::LightningImmune => 11, // Yellow
            Self::PoisonImmune => 5,   // Green
            Self::Haste => 11,         // Yellow
            Self::Summoner => 14,      // Dark Magenta
            Self::Warcry => 3,         // Red
            Self::GroundEffect => 6,   // Orange
            Self::Phasing => 1,        // Grey
            Self::Illusionist => 13,   // Magenta
            Self::Spellbreaker => 7,   // Blue
            Self::Healer => 5,         // Green
            Self::Berserker => 3,      // Red
            Self::Vortex => 13,        // Magenta
            Self::Necromancer => 14,   // Dark Magenta
            Self::TrailBlazer => 6,    // Orange
            Self::Knockback => 11,     // Yellow
            Self::Possessor => 14,     // Dark Magenta
        }
    }

    /// Check if this affix is offensive
    pub fn is_offensive(&self) -> bool {
        matches!(
            self,
            Self::Vampiric | Self::Explosive | Self::Molten | Self::Frozen |
            Self::Electrified | Self::Venomous | Self::Brutish | Self::ArmorPiercing |
            Self::Assassin | Self::Jagged | Self::ManaDrain | Self::Cursing |
            Self::Executioner
        )
    }

    /// Check if this affix is defensive
    pub fn is_defensive(&self) -> bool {
        matches!(
            self,
            Self::Shielded | Self::Regenerating | Self::Thorned | Self::Elusive |
            Self::Relentless | Self::AdaptiveArmor | Self::Undying | Self::Fortified |
            Self::FireImmune | Self::ColdImmune | Self::LightningImmune | Self::PoisonImmune
        )
    }

    /// Get attack damage modifier from this affix
    pub fn attack_modifier(&self) -> f32 {
        match self {
            Self::Brutish => 1.4,
            Self::Assassin => 1.0, // First hit is 3x, handled specially
            Self::Berserker => 1.0, // Conditional, handled specially
            _ => 1.0,
        }
    }

    /// Get defense modifier from this affix
    pub fn defense_modifier(&self) -> f32 {
        match self {
            Self::Shielded => 1.25,
            Self::Fortified => 1.4, // Only from front
            _ => 1.0,
        }
    }

    /// Get dodge chance bonus from this affix
    pub fn dodge_bonus(&self) -> f32 {
        match self {
            Self::Elusive => 0.30,
            Self::Haste => 0.10,
            _ => 0.0,
        }
    }

    /// Get life steal percentage for vampiric affix
    pub fn life_steal_percent(&self) -> f32 {
        match self {
            Self::Vampiric => 0.20,
            _ => 0.0,
        }
    }

    /// Get regeneration percentage per turn
    pub fn regen_percent(&self) -> f32 {
        match self {
            Self::Regenerating => 0.02,
            _ => 0.0,
        }
    }

    /// Get thorns damage reflection percentage
    pub fn reflect_percent(&self) -> f32 {
        match self {
            Self::Thorned => 0.30,
            _ => 0.0,
        }
    }

    /// Check if this affix grants immunity to a status effect
    pub fn grants_immunity(&self, effect: StatusEffect) -> bool {
        match self {
            Self::FireImmune => matches!(effect, StatusEffect::Burn),
            Self::ColdImmune => matches!(effect, StatusEffect::Freeze),
            Self::PoisonImmune => matches!(effect, StatusEffect::Poison),
            Self::Relentless => matches!(effect, StatusEffect::Stun | StatusEffect::Freeze | StatusEffect::Confusion),
            _ => false,
        }
    }

    /// Roll random affixes for a monster
    pub fn roll_affixes(count_min: usize, count_max: usize, rng: &mut impl Rng) -> Vec<Self> {
        let count = rng.gen_range(count_min..=count_max);
        let all_affixes = vec![
            Self::Vampiric, Self::Teleporting, Self::Explosive, Self::Molten,
            Self::Frozen, Self::Electrified, Self::Venomous, Self::Brutish,
            Self::ArmorPiercing, Self::Assassin, Self::Jagged, Self::ManaDrain,
            Self::Cursing, Self::Executioner, Self::Shielded, Self::Regenerating,
            Self::Thorned, Self::Elusive, Self::Relentless, Self::AdaptiveArmor,
            Self::Undying, Self::Fortified, Self::Haste, Self::Summoner,
            Self::Warcry, Self::GroundEffect, Self::Phasing, Self::Illusionist,
            Self::Spellbreaker, Self::Healer, Self::Berserker, Self::Vortex,
            Self::Necromancer, Self::TrailBlazer, Self::Knockback, Self::Possessor,
        ];

        let mut selected = Vec::new();
        let mut available = all_affixes.clone();

        for _ in 0..count {
            if available.is_empty() {
                break;
            }
            let idx = rng.gen_range(0..available.len());
            let affix = available.remove(idx);

            // Avoid conflicting immunities (pick at most one immunity)
            if matches!(affix, Self::FireImmune | Self::ColdImmune | Self::LightningImmune | Self::PoisonImmune) {
                available.retain(|a| !matches!(a, Self::FireImmune | Self::ColdImmune | Self::LightningImmune | Self::PoisonImmune));
            }

            selected.push(affix);
        }

        selected
    }
}

/// Represents a champion pack - a champion with minions
#[derive(Clone, Debug)]
pub struct ChampionPack {
    /// The champion leader's enemy ID (index in enemy list)
    pub champion_id: usize,
    /// IDs of minions in this pack
    pub minion_ids: Vec<usize>,
    /// Whether the pack is currently active (champion alive)
    pub is_active: bool,
    /// Pack-wide buffs from champion's affixes
    pub pack_buffs: Vec<MonsterAffix>,
}

impl ChampionPack {
    /// Create a new champion pack
    pub fn new(champion_id: usize, minion_ids: Vec<usize>, champion_affixes: &[MonsterAffix]) -> Self {
        // Extract affixes that affect the whole pack
        let pack_buffs: Vec<MonsterAffix> = champion_affixes
            .iter()
            .filter(|a| matches!(a, MonsterAffix::Warcry | MonsterAffix::Healer))
            .copied()
            .collect();

        Self {
            champion_id,
            minion_ids,
            is_active: true,
            pack_buffs,
        }
    }

    /// Check if a given enemy ID is the champion
    pub fn is_champion(&self, enemy_id: usize) -> bool {
        self.champion_id == enemy_id
    }

    /// Check if a given enemy ID is a minion in this pack
    pub fn is_minion(&self, enemy_id: usize) -> bool {
        self.minion_ids.contains(&enemy_id)
    }

    /// Check if a given enemy ID belongs to this pack
    pub fn contains(&self, enemy_id: usize) -> bool {
        self.is_champion(enemy_id) || self.is_minion(enemy_id)
    }

    /// Called when the champion dies - pack disbands
    pub fn on_champion_death(&mut self) {
        self.is_active = false;
        self.pack_buffs.clear();
    }

    /// Get damage bonus for minions from pack buffs
    pub fn minion_damage_bonus(&self) -> f32 {
        if self.pack_buffs.contains(&MonsterAffix::Warcry) {
            1.25
        } else {
            1.0
        }
    }
}

/// Elite monster data attached to an Enemy
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EliteData {
    /// The rarity tier of this monster
    pub rarity: MonsterRarity,
    /// List of affixes this monster has
    pub affixes: Vec<MonsterAffix>,
    /// Whether first strike bonus is available (for Assassin affix)
    pub first_strike_available: bool,
    /// Turns since last teleport (for Teleporting affix)
    pub teleport_cooldown: u32,
    /// Turns since last summon (for Summoner affix)
    pub summon_cooldown: u32,
    /// Turns since last phase shift (for Phasing affix)
    pub phase_cooldown: u32,
    /// Currently phased/invisible
    pub is_phased: bool,
    /// Shield from Adaptive Armor affix
    pub adaptive_shield: i32,
    /// Number of illusions created (for Illusionist)
    pub illusion_count: u32,
    /// Trail positions for TrailBlazer
    pub trail_positions: Vec<(usize, usize)>,
    /// Berserker mode active
    pub is_berserk: bool,
}

impl EliteData {
    /// Create new elite data for a normal monster
    pub fn normal() -> Self {
        Self {
            rarity: MonsterRarity::Normal,
            affixes: Vec::new(),
            first_strike_available: false,
            teleport_cooldown: 0,
            summon_cooldown: 0,
            phase_cooldown: 0,
            is_phased: false,
            adaptive_shield: 0,
            illusion_count: 0,
            trail_positions: Vec::new(),
            is_berserk: false,
        }
    }

    /// Create new elite data with specified rarity
    pub fn new(rarity: MonsterRarity, rng: &mut impl Rng) -> Self {
        let (min_affixes, max_affixes) = rarity.affix_count();
        let affixes = MonsterAffix::roll_affixes(min_affixes, max_affixes, rng);

        let first_strike_available = affixes.contains(&MonsterAffix::Assassin);

        Self {
            rarity,
            affixes,
            first_strike_available,
            teleport_cooldown: 0,
            summon_cooldown: 0,
            phase_cooldown: 0,
            is_phased: false,
            adaptive_shield: 0,
            illusion_count: 0,
            trail_positions: Vec::new(),
            is_berserk: false,
        }
    }

    /// Check if this monster has a specific affix
    pub fn has_affix(&self, affix: MonsterAffix) -> bool {
        self.affixes.contains(&affix)
    }

    /// Get the display name with rarity and affixes
    pub fn format_name(&self, base_name: &str) -> String {
        if self.rarity == MonsterRarity::Normal {
            return base_name.to_string();
        }

        let affix_names: Vec<&str> = self.affixes.iter().map(|a| a.name()).collect();
        if affix_names.is_empty() {
            format!("{}{}", self.rarity.name_prefix(), base_name)
        } else {
            format!("{}{} [{}]", self.rarity.name_prefix(), base_name, affix_names.join(", "))
        }
    }

    /// Calculate total attack modifier from affixes
    pub fn total_attack_modifier(&self, hp_percent: f32) -> f32 {
        let mut modifier = 1.0;

        for affix in &self.affixes {
            modifier *= affix.attack_modifier();

            // Berserker bonus when low HP
            if *affix == MonsterAffix::Berserker && hp_percent < 0.30 {
                modifier *= 1.75;
            }
        }

        // First strike bonus for Assassin
        if self.first_strike_available && self.has_affix(MonsterAffix::Assassin) {
            modifier *= 3.0;
        }

        modifier
    }

    /// Calculate total defense modifier from affixes
    pub fn total_defense_modifier(&self) -> f32 {
        let mut modifier = 1.0;

        for affix in &self.affixes {
            modifier *= affix.defense_modifier();
        }

        modifier
    }

    /// Calculate total dodge bonus from affixes
    pub fn total_dodge_bonus(&self) -> f32 {
        let mut bonus = 0.0;

        for affix in &self.affixes {
            bonus += affix.dodge_bonus();
        }

        // Phased monsters are very hard to hit
        if self.is_phased {
            bonus += 0.50;
        }

        bonus
    }

    /// Check if immune to a status effect
    pub fn is_immune_to(&self, effect: StatusEffect) -> bool {
        self.affixes.iter().any(|a| a.grants_immunity(effect))
    }

    /// Tick cooldowns and effects each turn
    pub fn tick(&mut self) {
        self.teleport_cooldown = self.teleport_cooldown.saturating_sub(1);
        self.summon_cooldown = self.summon_cooldown.saturating_sub(1);
        self.phase_cooldown = self.phase_cooldown.saturating_sub(1);

        // Phase shift ends after a turn
        if self.is_phased && self.phase_cooldown == 0 {
            self.is_phased = false;
        }

        // Decay adaptive shield
        self.adaptive_shield = (self.adaptive_shield - 5).max(0);

        // Trim trail positions (keep last 5)
        while self.trail_positions.len() > 5 {
            self.trail_positions.remove(0);
        }
    }

    /// Called when this monster takes damage
    pub fn on_damage_taken(&mut self, damage: i32, max_hp: i32) {
        // Adaptive Armor triggers on large hits
        if self.has_affix(MonsterAffix::AdaptiveArmor) && damage > max_hp / 5 {
            self.adaptive_shield = (max_hp as f32 * 0.15) as i32;
        }
    }

    /// Calculate damage after adaptive armor
    pub fn apply_adaptive_armor(&mut self, damage: i32) -> i32 {
        if self.adaptive_shield > 0 {
            let absorbed = damage.min(self.adaptive_shield);
            self.adaptive_shield -= absorbed;
            damage - absorbed
        } else {
            damage
        }
    }

    /// Check if should trigger Undying affix
    pub fn check_undying(&self, current_hp: i32, damage: i32) -> bool {
        self.has_affix(MonsterAffix::Undying) && current_hp > 1 && damage >= current_hp
    }

    /// Check if can teleport this turn
    pub fn can_teleport(&self) -> bool {
        self.has_affix(MonsterAffix::Teleporting) && self.teleport_cooldown == 0
    }

    /// Use teleport ability
    pub fn use_teleport(&mut self) {
        self.teleport_cooldown = 5;
    }

    /// Check if can summon this turn
    pub fn can_summon(&self) -> bool {
        self.has_affix(MonsterAffix::Summoner) && self.summon_cooldown == 0
    }

    /// Use summon ability
    pub fn use_summon(&mut self) {
        self.summon_cooldown = 8;
    }

    /// Check if can phase shift this turn
    pub fn can_phase(&self) -> bool {
        self.has_affix(MonsterAffix::Phasing) && self.phase_cooldown == 0 && !self.is_phased
    }

    /// Use phase shift ability
    pub fn use_phase(&mut self) {
        self.is_phased = true;
        self.phase_cooldown = 6;
    }

    /// Add position to trail (for TrailBlazer)
    pub fn add_trail_position(&mut self, x: usize, y: usize) {
        if self.has_affix(MonsterAffix::TrailBlazer) {
            self.trail_positions.push((x, y));
        }
    }

    /// Consume first strike bonus
    pub fn consume_first_strike(&mut self) {
        self.first_strike_available = false;
    }

    /// Calculate life steal amount from damage dealt
    pub fn calculate_life_steal(&self, damage: i32) -> i32 {
        if self.has_affix(MonsterAffix::Vampiric) {
            (damage as f32 * 0.20) as i32
        } else {
            0
        }
    }

    /// Calculate mana drain amount from damage dealt
    pub fn calculate_mana_drain(&self, damage: i32) -> i32 {
        if self.has_affix(MonsterAffix::ManaDrain) {
            (damage as f32 * 0.10) as i32
        } else {
            0
        }
    }

    /// Calculate thorns damage
    pub fn calculate_thorns_damage(&self, damage_received: i32) -> i32 {
        if self.has_affix(MonsterAffix::Thorned) {
            (damage_received as f32 * 0.30) as i32
        } else {
            0
        }
    }

    /// Calculate regeneration amount per turn
    pub fn calculate_regen(&self, max_hp: i32) -> i32 {
        if self.has_affix(MonsterAffix::Regenerating) {
            (max_hp as f32 * 0.02) as i32
        } else {
            0
        }
    }

    /// Check if should explode on death
    pub fn should_explode(&self) -> bool {
        self.has_affix(MonsterAffix::Explosive)
    }

    /// Get explosion damage
    pub fn explosion_damage(&self, base_attack: i32) -> i32 {
        if self.has_affix(MonsterAffix::Explosive) {
            (base_attack as f32 * 2.0) as i32
        } else {
            0
        }
    }

    /// Get explosion radius
    pub fn explosion_radius(&self) -> i32 {
        if self.has_affix(MonsterAffix::Explosive) {
            3
        } else {
            0
        }
    }

    /// Get status effects to apply on hit
    pub fn on_hit_effects(&self) -> Vec<(StatusEffect, u32)> {
        let mut effects = Vec::new();

        for affix in &self.affixes {
            match affix {
                MonsterAffix::Molten => effects.push((StatusEffect::Burn, 3)),
                MonsterAffix::Frozen => effects.push((StatusEffect::Freeze, 2)),
                MonsterAffix::Venomous => effects.push((StatusEffect::Poison, 4)),
                MonsterAffix::Jagged => effects.push((StatusEffect::Bleed, 4)),
                _ => {}
            }
        }

        effects
    }

    /// Check if Cursing affix triggers
    pub fn roll_curse(&self, rng: &mut impl Rng) -> bool {
        self.has_affix(MonsterAffix::Cursing) && rng.gen::<f32>() < 0.30
    }

    /// Check if Executioner affix triggers
    pub fn check_execute(&self, target_hp_percent: f32) -> bool {
        self.has_affix(MonsterAffix::Executioner) && target_hp_percent < 0.10
    }

    /// Calculate armor piercing
    pub fn armor_pierce_percent(&self) -> f32 {
        if self.has_affix(MonsterAffix::ArmorPiercing) {
            0.50
        } else {
            0.0
        }
    }

    /// Check and set berserker mode
    pub fn update_berserker(&mut self, hp_percent: f32) {
        if self.has_affix(MonsterAffix::Berserker) {
            self.is_berserk = hp_percent < 0.30;
        }
    }
}

impl Default for EliteData {
    fn default() -> Self {
        Self::normal()
    }
}

/// Result of elite affix processing
#[derive(Clone, Debug, Default)]
pub struct EliteAffixResult {
    /// Bonus damage to deal
    pub bonus_damage: i32,
    /// Bonus elemental damage type
    pub elemental_type: Option<ElementalDamageType>,
    /// Status effects to apply to target
    pub apply_effects: Vec<(StatusEffect, u32)>,
    /// Life stolen from target
    pub life_steal: i32,
    /// Mana drained from target
    pub mana_drain: i32,
    /// Damage reflected back to attacker
    pub reflected_damage: i32,
    /// Should knockback target
    pub knockback: bool,
    /// Player should be pulled (Vortex)
    pub vortex_pull: bool,
    /// Should trigger explosion on death
    pub trigger_explosion: bool,
    /// Explosion damage if triggered
    pub explosion_damage: i32,
    /// Explosion radius if triggered
    pub explosion_radius: i32,
    /// Message to display
    pub message: Option<String>,
}

/// Types of elemental damage
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ElementalDamageType {
    Fire,
    Cold,
    Lightning,
    Poison,
    Shadow,
}

impl ElementalDamageType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Fire => "fire",
            Self::Cold => "cold",
            Self::Lightning => "lightning",
            Self::Poison => "poison",
            Self::Shadow => "shadow",
        }
    }

    pub fn associated_status(&self) -> StatusEffect {
        match self {
            Self::Fire => StatusEffect::Burn,
            Self::Cold => StatusEffect::Freeze,
            Self::Lightning => StatusEffect::Stun,
            Self::Poison => StatusEffect::Poison,
            Self::Shadow => StatusEffect::Blind,
        }
    }
}

// ============================================================================
// ORIGINAL COMBAT SYSTEM CODE BELOW
// ============================================================================

/// Status effects that can be applied to characters and enemies
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum StatusEffect {
    /// Takes damage over time from poison
    Poison,
    /// Takes fire damage over time
    Burn,
    /// Movement and attack speed reduced
    Freeze,
    /// Takes damage over time from bleeding
    Bleed,
    /// Cannot take actions
    Stun,
    /// Reduced vision radius
    Blind,
    /// Increased movement and attack speed
    Haste,
    /// Absorbs incoming damage
    Shield,
    /// Heals over time
    Regeneration,
    /// Increased attack power
    Strength,
    /// Decreased attack power
    Weakness,
    /// Cannot be seen by enemies
    Invisibility,
    /// Random movement direction
    Confusion,
    /// Increased critical hit chance
    Focus,
    /// Increased dodge chance
    Evasion,
    /// Reduced armor/defense
    ArmorBreak,
    /// Silenced - cannot use abilities
    Silence,
    /// Slowed movement
    Slow,
    /// Marked for extra damage
    Vulnerable,
    /// Enraged - more damage but less defense
    Enrage,
    /// Cursed - bad luck, reduced crit chance
    Curse,
    /// Blessed - good luck, increased crit chance
    Blessed,
    /// Life steal on attacks
    Vampiric,
    /// Thorns - reflect damage to attackers
    Thorns,
}

impl StatusEffect {
    /// Returns the display name of the status effect
    pub fn name(&self) -> &'static str {
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
            Self::Focus => "Focused",
            Self::Evasion => "Evasive",
            Self::ArmorBreak => "Armor Broken",
            Self::Silence => "Silenced",
            Self::Slow => "Slowed",
            Self::Vulnerable => "Vulnerable",
            Self::Enrage => "Enraged",
            Self::Curse => "Cursed",
            Self::Blessed => "Blessed",
            Self::Vampiric => "Vampiric",
            Self::Thorns => "Thorns",
        }
    }

    /// Returns whether this effect is harmful (debuff) or helpful (buff)
    pub fn is_harmful(&self) -> bool {
        matches!(
            self,
            Self::Poison | Self::Burn | Self::Freeze | Self::Bleed |
            Self::Stun | Self::Blind | Self::Weakness | Self::Confusion |
            Self::ArmorBreak | Self::Silence | Self::Slow | Self::Vulnerable | Self::Curse
        )
    }

    /// Returns the damage per turn this effect deals (0 if not a damage effect)
    pub fn damage_per_turn(&self) -> i32 {
        match self {
            Self::Poison => 3,
            Self::Burn => 5,
            Self::Bleed => 2,
            _ => 0,
        }
    }

    /// Returns the stat modifier for this effect (multiplier, 1.0 = no change)
    pub fn attack_modifier(&self) -> f32 {
        match self {
            Self::Strength => 1.5,
            Self::Weakness => 0.6,
            Self::Enrage => 1.4,
            Self::Focus => 1.1,
            _ => 1.0,
        }
    }

    /// Returns the defense modifier for this effect
    pub fn defense_modifier(&self) -> f32 {
        match self {
            Self::Shield => 1.5,
            Self::ArmorBreak => 0.5,
            Self::Enrage => 0.7,
            Self::Vulnerable => 0.6,
            _ => 1.0,
        }
    }

    /// Returns the dodge chance modifier for this effect
    pub fn dodge_modifier(&self) -> f32 {
        match self {
            Self::Evasion => 0.25,
            Self::Haste => 0.15,
            Self::Freeze => -0.20,
            Self::Slow => -0.15,
            Self::Stun => -1.0, // Cannot dodge while stunned
            _ => 0.0,
        }
    }

    /// Returns the crit chance modifier for this effect
    pub fn crit_modifier(&self) -> f32 {
        match self {
            Self::Focus => 0.20,
            Self::Blessed => 0.15,
            Self::Curse => -0.15,
            Self::Blind => -0.20,
            _ => 0.0,
        }
    }
}

/// Types of special abilities enemies can use
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum EnemyAbility {
    // Offensive abilities
    /// Basic melee attack
    MeleeAttack,
    /// Ranged attack (arrows, spells, etc)
    RangedAttack,
    /// Area of effect attack
    AreaAttack,
    /// Multi-hit attack
    MultiStrike,
    /// Powerful single attack with windup
    HeavyStrike,
    /// Attack that ignores armor
    PiercingStrike,
    /// Leaping attack that closes distance
    LeapAttack,
    /// Attack that applies poison
    PoisonBite,
    /// Fire breath attack
    FireBreath,
    /// Ice attack that freezes
    FrostBlast,
    /// Shadow attack that blinds
    ShadowStrike,
    /// Life-draining attack
    LifeDrain,
    /// Attack that stuns
    StunningBlow,
    /// Attack that causes bleeding
    RendingClaws,
    /// Explosive attack
    Explosion,
    /// Chain lightning that jumps between targets
    ChainLightning,
    /// Summons minions
    SummonMinions,
    /// Berserk frenzy - multiple rapid attacks
    Frenzy,

    // Defensive abilities
    /// Raises a shield
    RaiseShield,
    /// Heals self
    Heal,
    /// Teleports away
    Teleport,
    /// Burrows underground
    Burrow,
    /// Becomes ethereal (immune to physical)
    PhaseShift,
    /// Counter-attack stance
    Riposte,

    // Buff/Debuff abilities
    /// Enrages self for more damage
    EnrageSelf,
    /// Weakens the player
    Weaken,
    /// Curses the player
    CursePlayer,
    /// Silences the player
    SilencePlayer,
    /// Marks player for extra damage
    MarkTarget,
    /// Battle cry that buffs nearby allies
    BattleCry,
    /// Fear - causes confusion
    TerroringScream,
    /// Applies thorns to self
    ThornsAura,
}

impl EnemyAbility {
    /// Returns the cooldown in turns for this ability
    pub fn cooldown(&self) -> u32 {
        match self {
            Self::MeleeAttack => 0,
            Self::RangedAttack => 0,
            Self::MultiStrike => 3,
            Self::HeavyStrike => 4,
            Self::AreaAttack => 5,
            Self::PiercingStrike => 2,
            Self::LeapAttack => 3,
            Self::PoisonBite => 2,
            Self::FireBreath => 4,
            Self::FrostBlast => 4,
            Self::ShadowStrike => 3,
            Self::LifeDrain => 3,
            Self::StunningBlow => 5,
            Self::RendingClaws => 2,
            Self::Explosion => 6,
            Self::ChainLightning => 5,
            Self::SummonMinions => 8,
            Self::Frenzy => 6,
            Self::RaiseShield => 4,
            Self::Heal => 5,
            Self::Teleport => 4,
            Self::Burrow => 3,
            Self::PhaseShift => 5,
            Self::Riposte => 3,
            Self::EnrageSelf => 6,
            Self::Weaken => 4,
            Self::CursePlayer => 6,
            Self::SilencePlayer => 5,
            Self::MarkTarget => 3,
            Self::BattleCry => 7,
            Self::TerroringScream => 6,
            Self::ThornsAura => 5,
        }
    }

    /// Returns the damage multiplier for this ability
    pub fn damage_multiplier(&self) -> f32 {
        match self {
            Self::MeleeAttack => 1.0,
            Self::RangedAttack => 0.9,
            Self::MultiStrike => 0.6, // Hits 3 times
            Self::HeavyStrike => 2.0,
            Self::AreaAttack => 0.8,
            Self::PiercingStrike => 1.2,
            Self::LeapAttack => 1.3,
            Self::PoisonBite => 0.8,
            Self::FireBreath => 1.4,
            Self::FrostBlast => 1.2,
            Self::ShadowStrike => 1.5,
            Self::LifeDrain => 1.0,
            Self::StunningBlow => 0.7,
            Self::RendingClaws => 1.1,
            Self::Explosion => 1.8,
            Self::ChainLightning => 1.3,
            Self::Frenzy => 0.5, // Hits 4 times
            _ => 0.0,
        }
    }

    /// Returns the number of hits for multi-hit abilities
    pub fn hit_count(&self) -> u32 {
        match self {
            Self::MultiStrike => 3,
            Self::Frenzy => 4,
            Self::ChainLightning => 3,
            _ => 1,
        }
    }
}

/// Result of a combat attack
#[derive(Clone, Debug)]
pub struct AttackResult {
    /// Base damage before modifiers
    pub base_damage: i32,
    /// Final damage dealt
    pub final_damage: i32,
    /// Whether the attack was a critical hit
    pub is_critical: bool,
    /// Critical multiplier applied (if any)
    pub crit_multiplier: f32,
    /// Whether the attack was dodged
    pub is_dodged: bool,
    /// Whether the attack was blocked/parried
    pub is_blocked: bool,
    /// Status effects applied by the attack
    pub applied_effects: Vec<(StatusEffect, u32)>,
    /// Damage reflected back to attacker (thorns)
    pub reflected_damage: i32,
    /// Health stolen by attacker
    pub life_stolen: i32,
    /// Combat message to display
    pub message: String,
}

impl AttackResult {
    pub fn new() -> Self {
        Self {
            base_damage: 0,
            final_damage: 0,
            is_critical: false,
            crit_multiplier: 1.0,
            is_dodged: false,
            is_blocked: false,
            applied_effects: Vec::new(),
            reflected_damage: 0,
            life_stolen: 0,
            message: String::new(),
        }
    }

    pub fn missed() -> Self {
        let mut result = Self::new();
        result.is_dodged = true;
        result.message = "Attack missed!".to_string();
        result
    }

    pub fn blocked() -> Self {
        let mut result = Self::new();
        result.is_blocked = true;
        result.message = "Attack blocked!".to_string();
        result
    }
}

impl Default for AttackResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Combat statistics for tracking combos and combat state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CombatStats {
    /// Current combo counter
    pub combo_count: u32,
    /// Maximum combo achieved this combat
    pub max_combo: u32,
    /// Turns since last successful hit (combo breaks after 2)
    pub turns_since_hit: u32,
    /// Total damage dealt this combat
    pub total_damage_dealt: i64,
    /// Total damage received this combat
    pub total_damage_received: i64,
    /// Number of critical hits landed
    pub critical_hits: u32,
    /// Number of attacks dodged
    pub attacks_dodged: u32,
    /// Number of enemies killed
    pub enemies_killed: u32,
    /// Base critical hit chance (0.0 to 1.0)
    pub base_crit_chance: f32,
    /// Base dodge chance (0.0 to 1.0)
    pub base_dodge_chance: f32,
    /// Critical damage multiplier
    pub crit_damage_multiplier: f32,
}

impl CombatStats {
    pub fn new() -> Self {
        Self {
            combo_count: 0,
            max_combo: 0,
            turns_since_hit: 0,
            total_damage_dealt: 0,
            total_damage_received: 0,
            critical_hits: 0,
            attacks_dodged: 0,
            enemies_killed: 0,
            base_crit_chance: 0.05, // 5% base crit chance
            base_dodge_chance: 0.05, // 5% base dodge chance
            crit_damage_multiplier: 2.0, // 200% crit damage
        }
    }

    /// Increments combo on successful hit
    pub fn hit_landed(&mut self) {
        self.combo_count += 1;
        self.turns_since_hit = 0;
        if self.combo_count > self.max_combo {
            self.max_combo = self.combo_count;
        }
    }

    /// Called when a turn passes without hitting
    pub fn turn_passed(&mut self) {
        self.turns_since_hit += 1;
        if self.turns_since_hit >= 2 {
            self.combo_count = 0;
        }
    }

    /// Resets combo (on miss or getting hit)
    pub fn combo_broken(&mut self) {
        self.combo_count = 0;
        self.turns_since_hit = 0;
    }

    /// Returns the combo damage multiplier
    pub fn combo_multiplier(&self) -> f32 {
        match self.combo_count {
            0..=2 => 1.0,
            3..=5 => 1.1,
            6..=9 => 1.2,
            10..=14 => 1.35,
            15..=19 => 1.5,
            20..=29 => 1.75,
            _ => 2.0, // 30+ combo
        }
    }

    /// Returns a combo tier name for display
    pub fn combo_tier_name(&self) -> &'static str {
        match self.combo_count {
            0 => "",
            1..=2 => "",
            3..=5 => "Combo!",
            6..=9 => "Great Combo!",
            10..=14 => "Excellent!",
            15..=19 => "Incredible!",
            20..=29 => "Unstoppable!",
            _ => "LEGENDARY!",
        }
    }
}

impl Default for CombatStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents the different types of enemies in the game
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum EnemyKind {
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
    BossGoblinKing,
    BossOrcWarlord,
    BossVampireLord,
    BossForestGuardian,
    BossIceDragon,
    BossDemonKing,

    // Mini-Bosses
    GoblinChampion,
    OrcBerserker,
    VampireElite,
    AncientWyrm,
    FrostLord,
    InfernalLord,
}

impl EnemyKind {
    /// Returns the display glyph for the enemy
    pub fn glyph(&self) -> char {
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
        }
    }

    /// Returns the display name of the enemy
    pub fn name(&self) -> &'static str {
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
        }
    }

    /// Returns the base stats for this enemy type (hp, attack, defense, xp_value)
    pub fn base_stats(&self) -> (i32, i32, i32, i32) {
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
        }
    }

    /// Returns combat stats (crit_chance, dodge_chance, crit_multiplier)
    pub fn combat_stats(&self) -> (f32, f32, f32) {
        match self {
            // Fast enemies have high dodge
            Self::Rat | Self::Bat => (0.05, 0.20, 1.5),
            Self::Spider | Self::GiantSpider | Self::IceSpider => (0.10, 0.15, 1.8),
            Self::Wolf | Self::DireWolf | Self::FrostWolf => (0.15, 0.20, 1.8),
            Self::GiantWasp => (0.10, 0.30, 1.5),

            // Assassin types have high crit
            Self::ShadowAssassin => (0.35, 0.25, 3.0),
            Self::Ghoul => (0.20, 0.10, 2.0),
            Self::Wraith | Self::IceWraith | Self::CinderWraith => (0.15, 0.20, 2.0),
            Self::Ghost => (0.10, 0.40, 1.5), // Ghosts are hard to hit

            // Heavy hitters have high crit damage
            Self::Troll | Self::ForestTroll => (0.10, 0.05, 2.5),
            Self::CaveOgre | Self::CaveBear => (0.15, 0.05, 2.2),
            Self::FrostGiant => (0.10, 0.03, 2.5),
            Self::Balrog => (0.20, 0.10, 2.5),
            Self::DeathKnight | Self::FrozenKnight => (0.20, 0.10, 2.0),

            // Vampires have life steal mechanics (crit = life steal trigger)
            Self::Vampire | Self::VampireElite | Self::BossVampireLord => (0.25, 0.15, 2.0),
            Self::Succubus => (0.20, 0.25, 1.8),

            // Tanky enemies have low dodge/crit but are tough
            Self::Golem | Self::BoneGolem | Self::LavaGolem => (0.05, 0.00, 1.5),
            Self::RockElemental | Self::TreeEnt => (0.05, 0.00, 1.8),
            Self::CursedStatue | Self::AncientGuardian => (0.10, 0.00, 2.0),
            Self::Gargoyle => (0.10, 0.05, 2.0),

            // Elemental enemies
            Self::IceElemental | Self::FireElemental => (0.15, 0.15, 2.0),
            Self::Hellhound => (0.20, 0.15, 1.8),
            Self::FireDrake => (0.15, 0.10, 2.2),
            Self::MagmaSlime => (0.05, 0.10, 1.5),
            Self::Salamander => (0.15, 0.20, 1.8),

            // Magic users
            Self::Lich => (0.25, 0.15, 2.5),
            Self::Sphinx => (0.20, 0.20, 2.0),
            Self::Druid => (0.15, 0.15, 1.8),
            Self::Banshee => (0.20, 0.25, 2.0),
            Self::ForestSpirit => (0.15, 0.30, 1.5),

            // Bosses - generally balanced but powerful
            Self::BossGoblinKing => (0.15, 0.10, 2.0),
            Self::BossOrcWarlord => (0.20, 0.08, 2.5),
            Self::BossForestGuardian => (0.15, 0.12, 2.2),
            Self::BossIceDragon => (0.20, 0.10, 2.5),
            Self::BossDemonKing => (0.25, 0.15, 3.0),

            // Mini-bosses
            Self::GoblinChampion => (0.15, 0.12, 2.0),
            Self::OrcBerserker => (0.25, 0.05, 2.5),
            Self::AncientWyrm => (0.18, 0.12, 2.2),
            Self::FrostLord => (0.20, 0.10, 2.3),
            Self::InfernalLord => (0.22, 0.12, 2.5),

            // Demon realm
            Self::Demon | Self::DemonLord => (0.18, 0.12, 2.2),
            Self::PitFiend => (0.20, 0.08, 2.3),
            Self::ShadowDemon => (0.22, 0.20, 2.0),
            Self::AbyssalHorror => (0.15, 0.05, 2.5),
            Self::DoomGuard => (0.20, 0.10, 2.2),

            // Yeti and Wendigo
            Self::YetiWarrior => (0.15, 0.08, 2.2),
            Self::Wendigo => (0.25, 0.15, 2.5),

            // Others - default stats
            _ => (0.08, 0.08, 1.8),
        }
    }

    /// Returns the abilities this enemy type can use
    pub fn abilities(&self) -> Vec<EnemyAbility> {
        match self {
            // Tier 1 - Basic abilities
            Self::Rat | Self::GiantRat => vec![EnemyAbility::MeleeAttack, EnemyAbility::MultiStrike],
            Self::Bat => vec![EnemyAbility::MeleeAttack, EnemyAbility::LeapAttack],
            Self::Spider => vec![EnemyAbility::MeleeAttack, EnemyAbility::PoisonBite],
            Self::Goblin => vec![EnemyAbility::MeleeAttack, EnemyAbility::RangedAttack],
            Self::Skeleton => vec![EnemyAbility::MeleeAttack, EnemyAbility::RendingClaws],
            Self::Kobold => vec![EnemyAbility::MeleeAttack, EnemyAbility::Teleport],
            Self::CaveCrawler => vec![EnemyAbility::MeleeAttack, EnemyAbility::Burrow, EnemyAbility::StunningBlow],

            // Tier 2 - More varied
            Self::GiantSpider => vec![EnemyAbility::MeleeAttack, EnemyAbility::PoisonBite, EnemyAbility::LeapAttack],
            Self::Orc => vec![EnemyAbility::MeleeAttack, EnemyAbility::HeavyStrike, EnemyAbility::EnrageSelf],
            Self::Troll => vec![EnemyAbility::MeleeAttack, EnemyAbility::HeavyStrike, EnemyAbility::Heal],
            Self::CaveOgre => vec![EnemyAbility::MeleeAttack, EnemyAbility::AreaAttack, EnemyAbility::StunningBlow],
            Self::Slime => vec![EnemyAbility::MeleeAttack, EnemyAbility::PoisonBite, EnemyAbility::AreaAttack],
            Self::Hobgoblin => vec![EnemyAbility::MeleeAttack, EnemyAbility::RangedAttack, EnemyAbility::BattleCry],
            Self::CaveBear => vec![EnemyAbility::MeleeAttack, EnemyAbility::HeavyStrike, EnemyAbility::RendingClaws, EnemyAbility::EnrageSelf],
            Self::Mushroom => vec![EnemyAbility::PoisonBite, EnemyAbility::AreaAttack],
            Self::RockElemental => vec![EnemyAbility::MeleeAttack, EnemyAbility::HeavyStrike, EnemyAbility::RaiseShield, EnemyAbility::ThornsAura],

            // Tier 3 - Undead abilities
            Self::Zombie => vec![EnemyAbility::MeleeAttack, EnemyAbility::PoisonBite, EnemyAbility::Frenzy],
            Self::Ghost => vec![EnemyAbility::MeleeAttack, EnemyAbility::PhaseShift, EnemyAbility::TerroringScream],
            Self::Wraith => vec![EnemyAbility::MeleeAttack, EnemyAbility::LifeDrain, EnemyAbility::PhaseShift],
            Self::Vampire => vec![EnemyAbility::MeleeAttack, EnemyAbility::LifeDrain, EnemyAbility::ShadowStrike, EnemyAbility::Teleport],
            Self::Mummy => vec![EnemyAbility::MeleeAttack, EnemyAbility::CursePlayer, EnemyAbility::SummonMinions],
            Self::Ghoul => vec![EnemyAbility::MeleeAttack, EnemyAbility::RendingClaws, EnemyAbility::Frenzy],
            Self::Banshee => vec![EnemyAbility::RangedAttack, EnemyAbility::TerroringScream, EnemyAbility::SilencePlayer],
            Self::DeathKnight => vec![EnemyAbility::MeleeAttack, EnemyAbility::HeavyStrike, EnemyAbility::LifeDrain, EnemyAbility::RaiseShield],
            Self::BoneGolem => vec![EnemyAbility::MeleeAttack, EnemyAbility::AreaAttack, EnemyAbility::RaiseShield, EnemyAbility::ThornsAura],

            // Tier 4 - Forest creatures
            Self::Wolf | Self::DireWolf => vec![EnemyAbility::MeleeAttack, EnemyAbility::LeapAttack, EnemyAbility::RendingClaws, EnemyAbility::BattleCry],
            Self::TreeEnt => vec![EnemyAbility::MeleeAttack, EnemyAbility::AreaAttack, EnemyAbility::Heal, EnemyAbility::ThornsAura],
            Self::ForestTroll => vec![EnemyAbility::MeleeAttack, EnemyAbility::HeavyStrike, EnemyAbility::Heal, EnemyAbility::EnrageSelf],
            Self::Druid => vec![EnemyAbility::RangedAttack, EnemyAbility::Heal, EnemyAbility::SummonMinions, EnemyAbility::CursePlayer],
            Self::WildBoar => vec![EnemyAbility::MeleeAttack, EnemyAbility::LeapAttack, EnemyAbility::StunningBlow],
            Self::GiantWasp => vec![EnemyAbility::MeleeAttack, EnemyAbility::PoisonBite, EnemyAbility::MultiStrike],
            Self::VenomousVine => vec![EnemyAbility::PoisonBite, EnemyAbility::AreaAttack, EnemyAbility::Weaken],
            Self::ForestSpirit => vec![EnemyAbility::RangedAttack, EnemyAbility::Heal, EnemyAbility::PhaseShift, EnemyAbility::SilencePlayer],

            // Tier 5 - Ice creatures
            Self::IceElemental => vec![EnemyAbility::MeleeAttack, EnemyAbility::FrostBlast, EnemyAbility::AreaAttack],
            Self::FrostGiant => vec![EnemyAbility::MeleeAttack, EnemyAbility::HeavyStrike, EnemyAbility::FrostBlast, EnemyAbility::StunningBlow],
            Self::YetiWarrior => vec![EnemyAbility::MeleeAttack, EnemyAbility::FrostBlast, EnemyAbility::EnrageSelf, EnemyAbility::TerroringScream],
            Self::IceWraith => vec![EnemyAbility::MeleeAttack, EnemyAbility::FrostBlast, EnemyAbility::PhaseShift, EnemyAbility::LifeDrain],
            Self::FrostWolf => vec![EnemyAbility::MeleeAttack, EnemyAbility::LeapAttack, EnemyAbility::FrostBlast],
            Self::IceSpider => vec![EnemyAbility::MeleeAttack, EnemyAbility::PoisonBite, EnemyAbility::FrostBlast],
            Self::FrozenKnight => vec![EnemyAbility::MeleeAttack, EnemyAbility::HeavyStrike, EnemyAbility::FrostBlast, EnemyAbility::RaiseShield, EnemyAbility::Riposte],
            Self::Wendigo => vec![EnemyAbility::MeleeAttack, EnemyAbility::LifeDrain, EnemyAbility::FrostBlast, EnemyAbility::TerroringScream, EnemyAbility::Frenzy],

            // Tier 6 - Fire creatures
            Self::FireElemental => vec![EnemyAbility::MeleeAttack, EnemyAbility::FireBreath, EnemyAbility::AreaAttack, EnemyAbility::Explosion],
            Self::LavaGolem => vec![EnemyAbility::MeleeAttack, EnemyAbility::FireBreath, EnemyAbility::AreaAttack, EnemyAbility::ThornsAura],
            Self::Hellhound => vec![EnemyAbility::MeleeAttack, EnemyAbility::FireBreath, EnemyAbility::LeapAttack, EnemyAbility::RendingClaws],
            Self::FireDrake => vec![EnemyAbility::MeleeAttack, EnemyAbility::FireBreath, EnemyAbility::AreaAttack, EnemyAbility::LeapAttack],
            Self::MagmaSlime => vec![EnemyAbility::MeleeAttack, EnemyAbility::FireBreath, EnemyAbility::AreaAttack],
            Self::Salamander => vec![EnemyAbility::MeleeAttack, EnemyAbility::FireBreath, EnemyAbility::MultiStrike],
            Self::CinderWraith => vec![EnemyAbility::MeleeAttack, EnemyAbility::FireBreath, EnemyAbility::PhaseShift, EnemyAbility::Explosion],
            Self::InfernalImp => vec![EnemyAbility::MeleeAttack, EnemyAbility::FireBreath, EnemyAbility::Teleport, EnemyAbility::CursePlayer],

            // Tier 7 - Ancient enemies
            Self::Golem => vec![EnemyAbility::MeleeAttack, EnemyAbility::HeavyStrike, EnemyAbility::AreaAttack, EnemyAbility::RaiseShield],
            Self::AncientGuardian => vec![EnemyAbility::MeleeAttack, EnemyAbility::HeavyStrike, EnemyAbility::RaiseShield, EnemyAbility::Riposte, EnemyAbility::ThornsAura],
            Self::Sphinx => vec![EnemyAbility::RangedAttack, EnemyAbility::ChainLightning, EnemyAbility::CursePlayer, EnemyAbility::SilencePlayer],
            Self::Lich => vec![EnemyAbility::RangedAttack, EnemyAbility::ChainLightning, EnemyAbility::LifeDrain, EnemyAbility::SummonMinions, EnemyAbility::CursePlayer],
            Self::Gargoyle => vec![EnemyAbility::MeleeAttack, EnemyAbility::LeapAttack, EnemyAbility::RaiseShield, EnemyAbility::PhaseShift],
            Self::MummyLord => vec![EnemyAbility::MeleeAttack, EnemyAbility::LifeDrain, EnemyAbility::CursePlayer, EnemyAbility::SummonMinions, EnemyAbility::TerroringScream],
            Self::CursedStatue => vec![EnemyAbility::MeleeAttack, EnemyAbility::HeavyStrike, EnemyAbility::CursePlayer, EnemyAbility::ThornsAura],
            Self::ShadowAssassin => vec![EnemyAbility::MeleeAttack, EnemyAbility::ShadowStrike, EnemyAbility::MultiStrike, EnemyAbility::Teleport, EnemyAbility::MarkTarget],

            // Tier 8 - Demons
            Self::Demon => vec![EnemyAbility::MeleeAttack, EnemyAbility::FireBreath, EnemyAbility::HeavyStrike, EnemyAbility::EnrageSelf],
            Self::DemonLord => vec![EnemyAbility::MeleeAttack, EnemyAbility::FireBreath, EnemyAbility::AreaAttack, EnemyAbility::SummonMinions, EnemyAbility::EnrageSelf],
            Self::Succubus => vec![EnemyAbility::MeleeAttack, EnemyAbility::LifeDrain, EnemyAbility::CursePlayer, EnemyAbility::Weaken, EnemyAbility::SilencePlayer],
            Self::Balrog => vec![EnemyAbility::MeleeAttack, EnemyAbility::FireBreath, EnemyAbility::HeavyStrike, EnemyAbility::AreaAttack, EnemyAbility::TerroringScream],
            Self::PitFiend => vec![EnemyAbility::MeleeAttack, EnemyAbility::FireBreath, EnemyAbility::HeavyStrike, EnemyAbility::Frenzy, EnemyAbility::EnrageSelf],
            Self::ShadowDemon => vec![EnemyAbility::MeleeAttack, EnemyAbility::ShadowStrike, EnemyAbility::PhaseShift, EnemyAbility::Teleport, EnemyAbility::TerroringScream],
            Self::AbyssalHorror => vec![EnemyAbility::MeleeAttack, EnemyAbility::AreaAttack, EnemyAbility::TerroringScream, EnemyAbility::CursePlayer, EnemyAbility::Frenzy],
            Self::DoomGuard => vec![EnemyAbility::MeleeAttack, EnemyAbility::HeavyStrike, EnemyAbility::PiercingStrike, EnemyAbility::RaiseShield, EnemyAbility::Riposte],

            // Bosses - Full arsenal
            Self::BossGoblinKing => vec![
                EnemyAbility::MeleeAttack, EnemyAbility::HeavyStrike, EnemyAbility::BattleCry,
                EnemyAbility::SummonMinions, EnemyAbility::EnrageSelf, EnemyAbility::RaiseShield
            ],
            Self::BossOrcWarlord => vec![
                EnemyAbility::MeleeAttack, EnemyAbility::HeavyStrike, EnemyAbility::AreaAttack,
                EnemyAbility::BattleCry, EnemyAbility::EnrageSelf, EnemyAbility::Frenzy
            ],
            Self::BossVampireLord => vec![
                EnemyAbility::MeleeAttack, EnemyAbility::LifeDrain, EnemyAbility::ShadowStrike,
                EnemyAbility::Teleport, EnemyAbility::SummonMinions, EnemyAbility::TerroringScream
            ],
            Self::BossForestGuardian => vec![
                EnemyAbility::MeleeAttack, EnemyAbility::AreaAttack, EnemyAbility::Heal,
                EnemyAbility::SummonMinions, EnemyAbility::ThornsAura, EnemyAbility::EnrageSelf
            ],
            Self::BossIceDragon => vec![
                EnemyAbility::MeleeAttack, EnemyAbility::FrostBlast, EnemyAbility::AreaAttack,
                EnemyAbility::LeapAttack, EnemyAbility::TerroringScream, EnemyAbility::RaiseShield
            ],
            Self::BossDemonKing => vec![
                EnemyAbility::MeleeAttack, EnemyAbility::FireBreath, EnemyAbility::AreaAttack,
                EnemyAbility::SummonMinions, EnemyAbility::ChainLightning, EnemyAbility::TerroringScream,
                EnemyAbility::EnrageSelf, EnemyAbility::Teleport
            ],

            // Mini-bosses
            Self::GoblinChampion => vec![EnemyAbility::MeleeAttack, EnemyAbility::HeavyStrike, EnemyAbility::BattleCry, EnemyAbility::EnrageSelf],
            Self::OrcBerserker => vec![EnemyAbility::MeleeAttack, EnemyAbility::HeavyStrike, EnemyAbility::Frenzy, EnemyAbility::EnrageSelf],
            Self::VampireElite => vec![EnemyAbility::MeleeAttack, EnemyAbility::LifeDrain, EnemyAbility::ShadowStrike, EnemyAbility::Teleport],
            Self::AncientWyrm => vec![EnemyAbility::MeleeAttack, EnemyAbility::FireBreath, EnemyAbility::AreaAttack, EnemyAbility::LeapAttack],
            Self::FrostLord => vec![EnemyAbility::MeleeAttack, EnemyAbility::FrostBlast, EnemyAbility::AreaAttack, EnemyAbility::RaiseShield],
            Self::InfernalLord => vec![EnemyAbility::MeleeAttack, EnemyAbility::FireBreath, EnemyAbility::AreaAttack, EnemyAbility::SummonMinions, EnemyAbility::EnrageSelf],
        }
    }

    /// Returns whether this enemy type is a boss
    pub fn is_boss(&self) -> bool {
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

    /// Returns whether this enemy is undead
    pub fn is_undead(&self) -> bool {
        matches!(
            self,
            Self::Skeleton | Self::Zombie | Self::Ghost | Self::Wraith
            | Self::Vampire | Self::Mummy | Self::Lich | Self::BossVampireLord
            | Self::Ghoul | Self::Banshee | Self::DeathKnight | Self::BoneGolem
            | Self::VampireElite | Self::MummyLord | Self::CinderWraith
        )
    }

    /// Returns whether this enemy can inflict poison
    pub fn can_poison(&self) -> bool {
        matches!(
            self,
            Self::Spider | Self::GiantSpider | Self::Slime
            | Self::Mushroom | Self::VenomousVine | Self::GiantWasp | Self::IceSpider
        )
    }

    /// Returns whether this enemy can inflict burning
    pub fn can_burn(&self) -> bool {
        matches!(
            self,
            Self::FireElemental | Self::LavaGolem | Self::Hellhound | Self::FireDrake
            | Self::Demon | Self::DemonLord | Self::Balrog | Self::BossDemonKing
            | Self::MagmaSlime | Self::Salamander | Self::CinderWraith | Self::InfernalImp
            | Self::PitFiend | Self::InfernalLord
        )
    }

    /// Returns whether this enemy can inflict freezing
    pub fn can_freeze(&self) -> bool {
        matches!(
            self,
            Self::IceElemental | Self::FrostGiant | Self::YetiWarrior
            | Self::IceWraith | Self::BossIceDragon | Self::FrostWolf
            | Self::IceSpider | Self::FrozenKnight | Self::Wendigo | Self::FrostLord
        )
    }

    /// Returns whether this enemy can inflict bleeding
    pub fn can_bleed(&self) -> bool {
        matches!(
            self,
            Self::Wolf | Self::DireWolf | Self::Skeleton | Self::DeathKnight
            | Self::ShadowAssassin | Self::Vampire | Self::Ghoul
            | Self::BossVampireLord | Self::BossOrcWarlord
        )
    }

    /// Returns a random enemy appropriate for the given dungeon level
    pub fn for_level(level: u32, rng: &mut impl Rng) -> Self {
        let enemies: Vec<Self> = match level {
            1..=4 => vec![Self::Rat, Self::Bat, Self::Spider, Self::Goblin, Self::Skeleton,
                         Self::Kobold, Self::GiantRat, Self::CaveCrawler],
            5..=8 => vec![Self::GiantSpider, Self::Orc, Self::Troll, Self::CaveOgre, Self::Slime,
                         Self::Hobgoblin, Self::CaveBear, Self::Mushroom, Self::RockElemental,
                         Self::GoblinChampion],
            9..=12 => vec![Self::Zombie, Self::Ghost, Self::Wraith, Self::Vampire, Self::Mummy,
                          Self::Ghoul, Self::Banshee, Self::DeathKnight, Self::BoneGolem,
                          Self::OrcBerserker],
            13..=16 => vec![Self::Wolf, Self::DireWolf, Self::TreeEnt, Self::ForestTroll, Self::Druid,
                           Self::WildBoar, Self::GiantWasp, Self::VenomousVine, Self::ForestSpirit,
                           Self::VampireElite],
            17..=20 => vec![Self::IceElemental, Self::FrostGiant, Self::YetiWarrior, Self::IceWraith,
                           Self::FrostWolf, Self::IceSpider, Self::FrozenKnight, Self::Wendigo,
                           Self::AncientWyrm],
            21..=24 => vec![Self::FireElemental, Self::LavaGolem, Self::Hellhound, Self::FireDrake,
                           Self::MagmaSlime, Self::Salamander, Self::CinderWraith, Self::InfernalImp,
                           Self::FrostLord],
            25..=28 => vec![Self::Golem, Self::AncientGuardian, Self::Sphinx, Self::Lich,
                           Self::Gargoyle, Self::MummyLord, Self::CursedStatue, Self::ShadowAssassin,
                           Self::InfernalLord],
            _ => vec![Self::Demon, Self::DemonLord, Self::Succubus, Self::Balrog,
                     Self::PitFiend, Self::ShadowDemon, Self::AbyssalHorror, Self::DoomGuard],
        };
        enemies[rng.gen_range(0..enemies.len())]
    }

    /// Returns the boss for a given level, if it's a boss level
    pub fn boss_for_level(level: u32) -> Option<Self> {
        match level {
            5 => Some(Self::BossGoblinKing),
            10 => Some(Self::BossOrcWarlord),
            15 => Some(Self::BossVampireLord),
            20 => Some(Self::BossForestGuardian),
            25 => Some(Self::BossIceDragon),
            30 => Some(Self::BossDemonKing),
            _ => None,
        }
    }
}

/// Represents an enemy in the game
#[derive(Clone, Debug)]
pub struct Enemy {
    /// X coordinate on the map
    pub x: usize,
    /// Y coordinate on the map
    pub y: usize,
    /// Type of enemy
    pub kind: EnemyKind,
    /// Current hit points
    pub hp: i32,
    /// Maximum hit points
    pub max_hp: i32,
    /// Attack power
    pub attack: i32,
    /// Defense value
    pub defense: i32,
    /// Experience value when killed
    pub xp_value: u32,
    /// Active status effects and their remaining duration
    pub status_effects: HashMap<StatusEffect, u32>,
    /// Last known player position (for AI)
    pub last_seen_player: Option<(usize, usize)>,
    /// Critical hit chance (0.0 to 1.0)
    pub crit_chance: f32,
    /// Dodge chance (0.0 to 1.0)
    pub dodge_chance: f32,
    /// Critical damage multiplier
    pub crit_multiplier: f32,
    /// Ability cooldowns
    pub ability_cooldowns: HashMap<EnemyAbility, u32>,
    /// Combat stats for tracking combos
    pub combat_stats: CombatStats,
    /// Whether this enemy is currently in riposte stance
    pub riposte_active: bool,
    /// Turns until can act again (for stun-like effects)
    pub skip_turns: u32,
}

impl Enemy {
    /// Creates a new enemy of the given type at the specified position
    pub fn new(x: usize, y: usize, kind: EnemyKind, level: u32) -> Self {
        let (base_hp, base_atk, base_def, base_xp) = kind.base_stats();
        let (crit, dodge, crit_mult) = kind.combat_stats();
        let scale = 1.0 + (level as f32 * 0.1);
        let hp = (base_hp as f32 * scale) as i32;

        // Initialize ability cooldowns
        let mut ability_cooldowns = HashMap::new();
        for ability in kind.abilities() {
            ability_cooldowns.insert(ability, 0);
        }

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
            crit_chance: crit,
            dodge_chance: dodge,
            crit_multiplier: crit_mult,
            ability_cooldowns,
            combat_stats: CombatStats::new(),
            riposte_active: false,
            skip_turns: 0,
        }
    }

    /// Returns true if the enemy is still alive
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    /// Returns the effective defense after status effect modifiers
    pub fn effective_defense(&self) -> i32 {
        let mut modifier = 1.0;
        for effect in self.status_effects.keys() {
            modifier *= effect.defense_modifier();
        }
        ((self.defense as f32) * modifier).max(0.0) as i32
    }

    /// Returns the effective attack after status effect modifiers
    pub fn effective_attack(&self) -> i32 {
        let mut modifier = 1.0;
        for effect in self.status_effects.keys() {
            modifier *= effect.attack_modifier();
        }
        ((self.attack as f32) * modifier).max(1.0) as i32
    }

    /// Returns the effective dodge chance after status effect modifiers
    pub fn effective_dodge_chance(&self) -> f32 {
        let mut dodge = self.dodge_chance;
        for effect in self.status_effects.keys() {
            dodge += effect.dodge_modifier();
        }
        dodge.clamp(0.0, 0.75) // Cap at 75% dodge
    }

    /// Returns the effective crit chance after status effect modifiers
    pub fn effective_crit_chance(&self) -> f32 {
        let mut crit = self.crit_chance;
        for effect in self.status_effects.keys() {
            crit += effect.crit_modifier();
        }
        crit.clamp(0.0, 0.80) // Cap at 80% crit
    }

    /// Attempts to dodge an attack
    pub fn try_dodge(&self, rng: &mut impl Rng) -> bool {
        rng.gen::<f32>() < self.effective_dodge_chance()
    }

    /// Rolls for critical hit
    pub fn roll_crit(&self, rng: &mut impl Rng) -> bool {
        rng.gen::<f32>() < self.effective_crit_chance()
    }

    /// Applies damage to the enemy, returns attack result
    pub fn take_damage(&mut self, amount: i32, rng: &mut impl Rng) -> AttackResult {
        let mut result = AttackResult::new();
        result.base_damage = amount;

        // Check for dodge
        if self.try_dodge(rng) {
            result.is_dodged = true;
            result.final_damage = 0;
            result.message = format!("{} dodged the attack!", self.kind.name());
            self.combat_stats.attacks_dodged += 1;
            return result;
        }

        // Calculate damage with defense
        let effective_def = self.effective_defense();
        let mut damage = (amount - effective_def).max(1);

        // Check for vulnerability
        if self.has_status(StatusEffect::Vulnerable) {
            damage = (damage as f32 * 1.25) as i32;
        }

        result.final_damage = damage;
        self.hp -= damage;

        // Check for thorns damage reflection
        if self.has_status(StatusEffect::Thorns) {
            result.reflected_damage = (damage as f32 * 0.2) as i32;
        }

        result.message = format!("{} takes {} damage!", self.kind.name(), damage);
        result
    }

    /// Enemy performs an attack, returns damage and effects
    pub fn perform_attack(&mut self, target_defense: i32, rng: &mut impl Rng) -> AttackResult {
        let mut result = AttackResult::new();
        let base_attack = self.effective_attack();
        result.base_damage = base_attack;

        // Roll for critical hit
        let is_crit = self.roll_crit(rng);
        result.is_critical = is_crit;

        let mut damage = base_attack;
        if is_crit {
            result.crit_multiplier = self.crit_multiplier;
            damage = (damage as f32 * self.crit_multiplier) as i32;
            self.combat_stats.critical_hits += 1;
        }

        // Apply combo bonus
        let combo_mult = self.combat_stats.combo_multiplier();
        damage = (damage as f32 * combo_mult) as i32;

        // Calculate final damage
        result.final_damage = (damage - target_defense).max(1);

        // Apply status effects based on enemy type
        if self.kind.can_poison() && rng.gen::<f32>() < 0.3 {
            result.applied_effects.push((StatusEffect::Poison, 3));
        }
        if self.kind.can_burn() && rng.gen::<f32>() < 0.25 {
            result.applied_effects.push((StatusEffect::Burn, 2));
        }
        if self.kind.can_freeze() && rng.gen::<f32>() < 0.2 {
            result.applied_effects.push((StatusEffect::Freeze, 2));
        }
        if self.kind.can_bleed() && rng.gen::<f32>() < 0.25 {
            result.applied_effects.push((StatusEffect::Bleed, 3));
        }

        // Life steal for vampiric enemies
        if self.has_status(StatusEffect::Vampiric) || matches!(self.kind, EnemyKind::Vampire | EnemyKind::VampireElite | EnemyKind::BossVampireLord | EnemyKind::Succubus | EnemyKind::Wraith) {
            result.life_stolen = (result.final_damage as f32 * 0.2) as i32;
            self.hp = (self.hp + result.life_stolen).min(self.max_hp);
        }

        // Build combo message
        let combo_msg = self.combat_stats.combo_tier_name();
        if is_crit {
            result.message = format!("{} lands a CRITICAL HIT for {} damage! {}",
                self.kind.name(), result.final_damage, combo_msg);
        } else {
            result.message = format!("{} attacks for {} damage! {}",
                self.kind.name(), result.final_damage, combo_msg);
        }

        self.combat_stats.hit_landed();
        result
    }

    /// Selects and uses an ability
    pub fn use_ability(&mut self, ability: EnemyAbility, target_defense: i32, rng: &mut impl Rng) -> AttackResult {
        // Reset cooldown
        if let Some(cd) = self.ability_cooldowns.get_mut(&ability) {
            *cd = ability.cooldown();
        }

        let mut result = self.perform_attack(target_defense, rng);

        // Modify damage based on ability
        let multiplier = ability.damage_multiplier();
        let hit_count = ability.hit_count();

        if hit_count > 1 {
            // Multi-hit ability
            let per_hit_damage = (result.final_damage as f32 * multiplier) as i32;
            result.final_damage = per_hit_damage * hit_count as i32;
            result.message = format!("{} uses {} - {} hits for {} total damage!",
                self.kind.name(), format!("{:?}", ability), hit_count, result.final_damage);
        } else if multiplier > 0.0 {
            result.final_damage = (result.final_damage as f32 * multiplier) as i32;
        }

        // Special ability effects
        match ability {
            EnemyAbility::PoisonBite => {
                result.applied_effects.push((StatusEffect::Poison, 4));
            }
            EnemyAbility::FireBreath => {
                result.applied_effects.push((StatusEffect::Burn, 3));
            }
            EnemyAbility::FrostBlast => {
                result.applied_effects.push((StatusEffect::Freeze, 2));
            }
            EnemyAbility::StunningBlow => {
                result.applied_effects.push((StatusEffect::Stun, 1));
            }
            EnemyAbility::RendingClaws => {
                result.applied_effects.push((StatusEffect::Bleed, 4));
            }
            EnemyAbility::LifeDrain => {
                result.life_stolen = (result.final_damage as f32 * 0.5) as i32;
                self.hp = (self.hp + result.life_stolen).min(self.max_hp);
            }
            EnemyAbility::ShadowStrike => {
                result.applied_effects.push((StatusEffect::Blind, 2));
            }
            EnemyAbility::Weaken => {
                result.applied_effects.push((StatusEffect::Weakness, 3));
            }
            EnemyAbility::CursePlayer => {
                result.applied_effects.push((StatusEffect::Curse, 5));
            }
            EnemyAbility::SilencePlayer => {
                result.applied_effects.push((StatusEffect::Silence, 3));
            }
            EnemyAbility::MarkTarget => {
                result.applied_effects.push((StatusEffect::Vulnerable, 3));
            }
            EnemyAbility::TerroringScream => {
                result.applied_effects.push((StatusEffect::Confusion, 2));
            }
            EnemyAbility::EnrageSelf => {
                self.add_status(StatusEffect::Enrage, 4);
                result.final_damage = 0;
                result.message = format!("{} becomes ENRAGED!", self.kind.name());
            }
            EnemyAbility::RaiseShield => {
                self.add_status(StatusEffect::Shield, 3);
                result.final_damage = 0;
                result.message = format!("{} raises a shield!", self.kind.name());
            }
            EnemyAbility::Heal => {
                let heal_amount = (self.max_hp as f32 * 0.2) as i32;
                self.hp = (self.hp + heal_amount).min(self.max_hp);
                result.final_damage = 0;
                result.message = format!("{} heals for {} HP!", self.kind.name(), heal_amount);
            }
            EnemyAbility::Riposte => {
                self.riposte_active = true;
                result.final_damage = 0;
                result.message = format!("{} takes a riposte stance!", self.kind.name());
            }
            EnemyAbility::ThornsAura => {
                self.add_status(StatusEffect::Thorns, 4);
                result.final_damage = 0;
                result.message = format!("{} activates thorns aura!", self.kind.name());
            }
            EnemyAbility::BattleCry => {
                self.add_status(StatusEffect::Strength, 4);
                result.final_damage = 0;
                result.message = format!("{} lets out a battle cry!", self.kind.name());
            }
            EnemyAbility::PhaseShift => {
                self.add_status(StatusEffect::Evasion, 2);
                result.final_damage = 0;
                result.message = format!("{} phases out of reality!", self.kind.name());
            }
            _ => {}
        }

        result
    }

    /// Choose best ability to use based on situation
    pub fn choose_ability(&self, player_hp_percent: f32, distance: i32, rng: &mut impl Rng) -> EnemyAbility {
        let available: Vec<_> = self.kind.abilities()
            .into_iter()
            .filter(|a| self.ability_cooldowns.get(a).map_or(true, |&cd| cd == 0))
            .collect();

        if available.is_empty() {
            return EnemyAbility::MeleeAttack;
        }

        // AI decision making
        let hp_percent = self.hp as f32 / self.max_hp as f32;

        // Low HP - prioritize healing or defensive abilities
        if hp_percent < 0.3 {
            if available.contains(&EnemyAbility::Heal) {
                return EnemyAbility::Heal;
            }
            if available.contains(&EnemyAbility::Teleport) {
                return EnemyAbility::Teleport;
            }
            if available.contains(&EnemyAbility::RaiseShield) {
                return EnemyAbility::RaiseShield;
            }
        }

        // Player low HP - go for the kill with heavy attacks
        if player_hp_percent < 0.25 {
            if available.contains(&EnemyAbility::HeavyStrike) {
                return EnemyAbility::HeavyStrike;
            }
            if available.contains(&EnemyAbility::Frenzy) {
                return EnemyAbility::Frenzy;
            }
        }

        // At range - use ranged attacks
        if distance > 1 {
            if available.contains(&EnemyAbility::RangedAttack) {
                return EnemyAbility::RangedAttack;
            }
            if available.contains(&EnemyAbility::FireBreath) {
                return EnemyAbility::FireBreath;
            }
            if available.contains(&EnemyAbility::FrostBlast) {
                return EnemyAbility::FrostBlast;
            }
            if available.contains(&EnemyAbility::LeapAttack) {
                return EnemyAbility::LeapAttack;
            }
        }

        // Random weighted selection for variety
        let weights: Vec<f32> = available.iter().map(|a| {
            match a {
                EnemyAbility::MeleeAttack => 1.0,
                EnemyAbility::HeavyStrike => 0.5,
                EnemyAbility::MultiStrike => 0.6,
                EnemyAbility::PoisonBite | EnemyAbility::FireBreath | EnemyAbility::FrostBlast => 0.7,
                EnemyAbility::LifeDrain => 0.8,
                EnemyAbility::EnrageSelf if hp_percent > 0.5 => 0.6,
                EnemyAbility::Frenzy => 0.4,
                _ => 0.3,
            }
        }).collect();

        let total: f32 = weights.iter().sum();
        let mut roll = rng.gen::<f32>() * total;
        for (i, w) in weights.iter().enumerate() {
            roll -= w;
            if roll <= 0.0 {
                return available[i];
            }
        }

        available[0]
    }

    /// Adds a status effect to the enemy
    pub fn add_status(&mut self, effect: StatusEffect, duration: u32) {
        self.status_effects.insert(effect, duration);
    }

    /// Checks if the enemy has a specific status effect
    pub fn has_status(&self, effect: StatusEffect) -> bool {
        self.status_effects.contains_key(&effect)
    }

    /// Tick all cooldowns
    pub fn tick_cooldowns(&mut self) {
        for cd in self.ability_cooldowns.values_mut() {
            *cd = cd.saturating_sub(1);
        }
        if self.skip_turns > 0 {
            self.skip_turns -= 1;
        }
        self.riposte_active = false;
        self.combat_stats.turn_passed();
    }

    /// Processes status effects, returns damage events
    pub fn tick_status_effects(&mut self) -> Vec<(StatusEffect, i32)> {
        let mut damage_events = Vec::new();
        let mut to_remove = Vec::new();

        for (effect, duration) in self.status_effects.iter_mut() {
            match effect {
                StatusEffect::Poison => damage_events.push((*effect, 3)),
                StatusEffect::Burn => damage_events.push((*effect, 5)),
                StatusEffect::Bleed => damage_events.push((*effect, 2)),
                StatusEffect::Regeneration => {
                    let heal = (self.max_hp as f32 * 0.05) as i32;
                    self.hp = (self.hp + heal).min(self.max_hp);
                }
                StatusEffect::Stun => {
                    self.skip_turns = 1;
                }
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

    /// Returns whether the enemy can act this turn
    pub fn can_act(&self) -> bool {
        self.skip_turns == 0 && !self.has_status(StatusEffect::Stun) && !self.has_status(StatusEffect::Freeze)
    }
}

/// Calculate damage with full combat mechanics
pub fn calculate_combat_damage(
    attacker_attack: i32,
    defender_defense: i32,
    attacker_crit_chance: f32,
    attacker_crit_multiplier: f32,
    defender_dodge_chance: f32,
    combo_multiplier: f32,
    rng: &mut impl Rng,
) -> AttackResult {
    let mut result = AttackResult::new();
    result.base_damage = attacker_attack;

    // Check dodge
    if rng.gen::<f32>() < defender_dodge_chance {
        return AttackResult::missed();
    }

    // Check crit
    let is_crit = rng.gen::<f32>() < attacker_crit_chance;
    result.is_critical = is_crit;

    let mut damage = attacker_attack;
    if is_crit {
        result.crit_multiplier = attacker_crit_multiplier;
        damage = (damage as f32 * attacker_crit_multiplier) as i32;
    }

    // Apply combo
    damage = (damage as f32 * combo_multiplier) as i32;

    // Apply defense
    result.final_damage = (damage - defender_defense).max(1);

    if is_crit {
        result.message = format!("CRITICAL HIT! {} damage!", result.final_damage);
    } else {
        result.message = format!("{} damage!", result.final_damage);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_effect_names() {
        assert_eq!(StatusEffect::Poison.name(), "Poisoned");
        assert_eq!(StatusEffect::Burn.name(), "Burning");
        assert_eq!(StatusEffect::Focus.name(), "Focused");
        assert_eq!(StatusEffect::Evasion.name(), "Evasive");
    }

    #[test]
    fn test_enemy_creation() {
        let enemy = Enemy::new(10, 10, EnemyKind::Goblin, 1);
        assert!(enemy.hp > 0);
        assert_eq!(enemy.kind, EnemyKind::Goblin);
        assert!(enemy.crit_chance > 0.0);
        assert!(enemy.dodge_chance >= 0.0);
    }

    #[test]
    fn test_enemy_damage() {
        let mut rng = rand::thread_rng();
        let mut enemy = Enemy::new(10, 10, EnemyKind::Rat, 1);
        let initial_hp = enemy.hp;
        let result = enemy.take_damage(50, &mut rng); // High damage to ensure it goes through
        if !result.is_dodged {
            assert!(enemy.hp < initial_hp);
            assert!(result.final_damage >= 1);
        }
    }

    #[test]
    fn test_boss_identification() {
        assert!(EnemyKind::BossDemonKing.is_boss());
        assert!(!EnemyKind::Goblin.is_boss());
    }

    #[test]
    fn test_combat_stats_combo() {
        let mut stats = CombatStats::new();
        assert_eq!(stats.combo_count, 0);
        assert_eq!(stats.combo_multiplier(), 1.0);

        for _ in 0..5 {
            stats.hit_landed();
        }
        assert_eq!(stats.combo_count, 5);
        assert_eq!(stats.combo_multiplier(), 1.1);

        stats.combo_broken();
        assert_eq!(stats.combo_count, 0);
    }

    #[test]
    fn test_enemy_abilities() {
        let abilities = EnemyKind::BossDemonKing.abilities();
        assert!(abilities.len() > 3);
        assert!(abilities.contains(&EnemyAbility::FireBreath));
    }

    #[test]
    fn test_status_effect_modifiers() {
        assert!(StatusEffect::Strength.attack_modifier() > 1.0);
        assert!(StatusEffect::Weakness.attack_modifier() < 1.0);
        assert!(StatusEffect::Evasion.dodge_modifier() > 0.0);
        assert!(StatusEffect::Focus.crit_modifier() > 0.0);
    }

    #[test]
    fn test_effective_stats_with_status() {
        let mut enemy = Enemy::new(10, 10, EnemyKind::Orc, 5);
        let base_attack = enemy.effective_attack();

        enemy.add_status(StatusEffect::Strength, 3);
        assert!(enemy.effective_attack() > base_attack);

        enemy.add_status(StatusEffect::Weakness, 3);
        // Strength * Weakness should roughly cancel out
    }

    #[test]
    fn test_attack_result_creation() {
        let result = AttackResult::new();
        assert_eq!(result.final_damage, 0);
        assert!(!result.is_critical);
        assert!(!result.is_dodged);

        let missed = AttackResult::missed();
        assert!(missed.is_dodged);
        assert_eq!(missed.final_damage, 0);
    }
}
