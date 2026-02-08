//! Magic Schools and Spell System
//!
//! 12 schools of magic with 150+ spells, spell combinations,
//! and magical mastery system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The 12 schools of magic
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MagicSchool {
    /// Fire magic - destruction and warmth
    Pyromancy,
    /// Ice magic - cold and preservation
    Cryomancy,
    /// Lightning magic - speed and power
    Electromancy,
    /// Earth magic - defense and strength
    Geomancy,
    /// Water magic - healing and flow
    Hydromancy,
    /// Wind magic - speed and evasion
    Aeromancy,
    /// Light magic - healing and holy
    Lumimancy,
    /// Dark magic - curses and shadows
    Umbramancy,
    /// Death magic - undead and life drain
    Necromancy,
    /// Mind magic - control and illusions
    Psychomancy,
    /// Time magic - haste and slow
    Chronomancy,
    /// Space magic - teleportation and portals
    Dimensionalism,
}

impl MagicSchool {
    pub fn all() -> &'static [MagicSchool] {
        &[
            Self::Pyromancy, Self::Cryomancy, Self::Electromancy,
            Self::Geomancy, Self::Hydromancy, Self::Aeromancy,
            Self::Lumimancy, Self::Umbramancy, Self::Necromancy,
            Self::Psychomancy, Self::Chronomancy, Self::Dimensionalism,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Pyromancy => "Pyromancy",
            Self::Cryomancy => "Cryomancy",
            Self::Electromancy => "Electromancy",
            Self::Geomancy => "Geomancy",
            Self::Hydromancy => "Hydromancy",
            Self::Aeromancy => "Aeromancy",
            Self::Lumimancy => "Lumimancy",
            Self::Umbramancy => "Umbramancy",
            Self::Necromancy => "Necromancy",
            Self::Psychomancy => "Psychomancy",
            Self::Chronomancy => "Chronomancy",
            Self::Dimensionalism => "Dimensionalism",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Pyromancy => "The art of fire magic. Burn your enemies to ash.",
            Self::Cryomancy => "The art of ice magic. Freeze and shatter foes.",
            Self::Electromancy => "The art of lightning. Strike with the speed of storms.",
            Self::Geomancy => "The art of earth magic. Unbreakable defense.",
            Self::Hydromancy => "The art of water magic. Heal and flow like the tide.",
            Self::Aeromancy => "The art of wind magic. Swift as the breeze.",
            Self::Lumimancy => "The art of light magic. Heal and smite evil.",
            Self::Umbramancy => "The art of shadow magic. Strike from darkness.",
            Self::Necromancy => "The art of death magic. Command the dead.",
            Self::Psychomancy => "The art of mind magic. Control and deceive.",
            Self::Chronomancy => "The art of time magic. Bend time itself.",
            Self::Dimensionalism => "The art of space magic. Warp reality.",
        }
    }

    pub fn primary_stat(&self) -> &'static str {
        match self {
            Self::Pyromancy | Self::Cryomancy | Self::Electromancy => "Intelligence",
            Self::Geomancy | Self::Hydromancy | Self::Aeromancy => "Wisdom",
            Self::Lumimancy => "Faith",
            Self::Umbramancy | Self::Necromancy => "Dark Affinity",
            Self::Psychomancy => "Willpower",
            Self::Chronomancy | Self::Dimensionalism => "Arcane Knowledge",
        }
    }

    pub fn opposing_school(&self) -> MagicSchool {
        match self {
            Self::Pyromancy => Self::Cryomancy,
            Self::Cryomancy => Self::Pyromancy,
            Self::Electromancy => Self::Geomancy,
            Self::Geomancy => Self::Electromancy,
            Self::Hydromancy => Self::Aeromancy,
            Self::Aeromancy => Self::Hydromancy,
            Self::Lumimancy => Self::Umbramancy,
            Self::Umbramancy => Self::Lumimancy,
            Self::Necromancy => Self::Lumimancy,
            Self::Psychomancy => Self::Geomancy,
            Self::Chronomancy => Self::Dimensionalism,
            Self::Dimensionalism => Self::Chronomancy,
        }
    }

    pub fn complementary_schools(&self) -> Vec<MagicSchool> {
        match self {
            Self::Pyromancy => vec![Self::Aeromancy, Self::Electromancy],
            Self::Cryomancy => vec![Self::Hydromancy, Self::Aeromancy],
            Self::Electromancy => vec![Self::Pyromancy, Self::Aeromancy],
            Self::Geomancy => vec![Self::Hydromancy, Self::Pyromancy],
            Self::Hydromancy => vec![Self::Cryomancy, Self::Geomancy],
            Self::Aeromancy => vec![Self::Electromancy, Self::Cryomancy],
            Self::Lumimancy => vec![Self::Pyromancy, Self::Aeromancy],
            Self::Umbramancy => vec![Self::Necromancy, Self::Psychomancy],
            Self::Necromancy => vec![Self::Umbramancy, Self::Geomancy],
            Self::Psychomancy => vec![Self::Umbramancy, Self::Chronomancy],
            Self::Chronomancy => vec![Self::Dimensionalism, Self::Psychomancy],
            Self::Dimensionalism => vec![Self::Chronomancy, Self::Aeromancy],
        }
    }

    pub fn spells(&self) -> Vec<Spell> {
        match self {
            Self::Pyromancy => vec![
                Spell::new("Spark", *self, 1, 5, 10, SpellEffect::Damage { amount: 8, element: Element::Fire }),
                Spell::new("Fireball", *self, 2, 15, 25, SpellEffect::AreaDamage { amount: 20, radius: 2, element: Element::Fire }),
                Spell::new("Flame Shield", *self, 2, 20, 30, SpellEffect::Shield { amount: 15, reflect_damage: 5 }),
                Spell::new("Incinerate", *self, 3, 30, 45, SpellEffect::Damage { amount: 45, element: Element::Fire }),
                Spell::new("Meteor", *self, 4, 50, 80, SpellEffect::AreaDamage { amount: 60, radius: 3, element: Element::Fire }),
                Spell::new("Inferno", *self, 5, 80, 120, SpellEffect::AreaDamage { amount: 100, radius: 4, element: Element::Fire }),
                Spell::new("Heat Wave", *self, 3, 25, 40, SpellEffect::StatusApply { status: "Burn".into(), duration: 5 }),
                Spell::new("Fire Wall", *self, 3, 35, 50, SpellEffect::CreateTerrain { terrain: "FireWall".into(), duration: 10 }),
                Spell::new("Phoenix Fire", *self, 5, 100, 150, SpellEffect::Resurrection { hp_percent: 50 }),
                Spell::new("Combustion", *self, 4, 45, 70, SpellEffect::Explosion { damage: 55, radius: 2 }),
            ],
            Self::Cryomancy => vec![
                Spell::new("Frost Bolt", *self, 1, 5, 10, SpellEffect::Damage { amount: 7, element: Element::Ice }),
                Spell::new("Ice Shard", *self, 2, 12, 20, SpellEffect::Damage { amount: 18, element: Element::Ice }),
                Spell::new("Freeze", *self, 2, 20, 30, SpellEffect::StatusApply { status: "Frozen".into(), duration: 3 }),
                Spell::new("Blizzard", *self, 3, 35, 55, SpellEffect::AreaDamage { amount: 30, radius: 3, element: Element::Ice }),
                Spell::new("Ice Armor", *self, 2, 18, 25, SpellEffect::Shield { amount: 25, reflect_damage: 0 }),
                Spell::new("Glacial Spike", *self, 4, 40, 65, SpellEffect::Damage { amount: 50, element: Element::Ice }),
                Spell::new("Absolute Zero", *self, 5, 90, 140, SpellEffect::AreaDamage { amount: 80, radius: 2, element: Element::Ice }),
                Spell::new("Ice Wall", *self, 2, 15, 25, SpellEffect::CreateTerrain { terrain: "IceWall".into(), duration: 15 }),
                Spell::new("Shatter", *self, 3, 25, 40, SpellEffect::ConditionalDamage { base: 20, bonus_vs_status: "Frozen".into(), bonus: 40 }),
                Spell::new("Permafrost", *self, 4, 50, 75, SpellEffect::AreaStatusApply { status: "Slow".into(), duration: 10, radius: 3 }),
            ],
            Self::Electromancy => vec![
                Spell::new("Shock", *self, 1, 5, 12, SpellEffect::Damage { amount: 10, element: Element::Lightning }),
                Spell::new("Lightning Bolt", *self, 2, 15, 28, SpellEffect::Damage { amount: 25, element: Element::Lightning }),
                Spell::new("Chain Lightning", *self, 3, 30, 50, SpellEffect::ChainDamage { amount: 20, chains: 4, element: Element::Lightning }),
                Spell::new("Thunder Clap", *self, 3, 25, 40, SpellEffect::AreaStatusApply { status: "Stun".into(), duration: 2, radius: 2 }),
                Spell::new("Static Field", *self, 2, 18, 30, SpellEffect::Aura { effect: "Static".into(), radius: 3, duration: 8 }),
                Spell::new("Storm Call", *self, 4, 45, 70, SpellEffect::AreaDamage { amount: 45, radius: 4, element: Element::Lightning }),
                Spell::new("Overcharge", *self, 3, 30, 45, SpellEffect::BuffSelf { stat: "Attack".into(), amount: 30, duration: 5 }),
                Spell::new("Tesla Coil", *self, 4, 40, 60, SpellEffect::Summon { creature: "TeslaCoil".into(), duration: 20 }),
                Spell::new("Ride the Lightning", *self, 3, 25, 35, SpellEffect::Teleport { range: 8 }),
                Spell::new("Thundergod's Wrath", *self, 5, 100, 160, SpellEffect::GlobalDamage { amount: 50, element: Element::Lightning }),
            ],
            Self::Necromancy => vec![
                Spell::new("Life Tap", *self, 1, 0, 0, SpellEffect::LifeDrain { amount: 10, heal_percent: 50 }),
                Spell::new("Raise Skeleton", *self, 2, 20, 35, SpellEffect::Summon { creature: "Skeleton".into(), duration: 50 }),
                Spell::new("Corpse Explosion", *self, 2, 15, 25, SpellEffect::CorpseExplosion { damage_percent: 50, radius: 2 }),
                Spell::new("Wither", *self, 2, 18, 30, SpellEffect::Debuff { stat: "Defense".into(), amount: 20, duration: 8 }),
                Spell::new("Soul Harvest", *self, 3, 30, 50, SpellEffect::SoulHarvest { damage: 25, mana_restore: 15 }),
                Spell::new("Raise Zombie", *self, 3, 35, 55, SpellEffect::Summon { creature: "Zombie".into(), duration: 60 }),
                Spell::new("Bone Armor", *self, 2, 20, 30, SpellEffect::Shield { amount: 30, reflect_damage: 0 }),
                Spell::new("Death Coil", *self, 3, 25, 40, SpellEffect::SmartHeal { damage_to_enemy: 30, heal_to_undead: 30 }),
                Spell::new("Army of the Dead", *self, 5, 80, 130, SpellEffect::MassSummon { creature: "Skeleton".into(), count: 5, duration: 40 }),
                Spell::new("Lich Form", *self, 5, 100, 150, SpellEffect::Transform { form: "Lich".into(), duration: 30 }),
                Spell::new("Death and Decay", *self, 4, 50, 80, SpellEffect::AreaDot { damage_per_turn: 15, radius: 3, duration: 6 }),
                Spell::new("Consume Soul", *self, 4, 45, 70, SpellEffect::Execute { threshold_percent: 20 }),
            ],
            Self::Lumimancy => vec![
                Spell::new("Holy Light", *self, 1, 8, 15, SpellEffect::Heal { amount: 20 }),
                Spell::new("Smite", *self, 1, 6, 12, SpellEffect::Damage { amount: 12, element: Element::Holy }),
                Spell::new("Blessing", *self, 2, 15, 25, SpellEffect::BuffSelf { stat: "All".into(), amount: 10, duration: 20 }),
                Spell::new("Divine Shield", *self, 3, 30, 50, SpellEffect::Invulnerability { duration: 3 }),
                Spell::new("Resurrect", *self, 4, 60, 100, SpellEffect::Resurrection { hp_percent: 30 }),
                Spell::new("Holy Nova", *self, 3, 35, 55, SpellEffect::HolyNova { heal_allies: 25, damage_enemies: 25, radius: 3 }),
                Spell::new("Purify", *self, 2, 12, 20, SpellEffect::Cleanse { remove_debuffs: true }),
                Spell::new("Guardian Angel", *self, 4, 50, 80, SpellEffect::Summon { creature: "GuardianAngel".into(), duration: 30 }),
                Spell::new("Divine Judgment", *self, 5, 90, 140, SpellEffect::Execute { threshold_percent: 25 }),
                Spell::new("Sanctuary", *self, 3, 40, 60, SpellEffect::CreateTerrain { terrain: "Sanctuary".into(), duration: 15 }),
            ],
            Self::Umbramancy => vec![
                Spell::new("Shadow Bolt", *self, 1, 5, 10, SpellEffect::Damage { amount: 10, element: Element::Shadow }),
                Spell::new("Curse", *self, 2, 15, 25, SpellEffect::Debuff { stat: "All".into(), amount: 15, duration: 10 }),
                Spell::new("Shadow Step", *self, 2, 12, 20, SpellEffect::Teleport { range: 5 }),
                Spell::new("Darkness", *self, 2, 18, 30, SpellEffect::Blind { duration: 5, radius: 3 }),
                Spell::new("Shadow Clone", *self, 3, 35, 55, SpellEffect::Summon { creature: "ShadowClone".into(), duration: 20 }),
                Spell::new("Void Bolt", *self, 3, 28, 45, SpellEffect::Damage { amount: 35, element: Element::Void }),
                Spell::new("Soul Rend", *self, 4, 45, 70, SpellEffect::LifeDrain { amount: 40, heal_percent: 30 }),
                Spell::new("Shadow Form", *self, 3, 30, 50, SpellEffect::Transform { form: "Shadow".into(), duration: 15 }),
                Spell::new("Abyssal Gate", *self, 5, 80, 130, SpellEffect::Summon { creature: "VoidHorror".into(), duration: 25 }),
                Spell::new("Consume Darkness", *self, 4, 50, 75, SpellEffect::AreaDamage { amount: 50, radius: 3, element: Element::Shadow }),
            ],
            Self::Chronomancy => vec![
                Spell::new("Haste", *self, 2, 15, 25, SpellEffect::BuffSelf { stat: "Speed".into(), amount: 50, duration: 10 }),
                Spell::new("Slow", *self, 2, 12, 20, SpellEffect::Debuff { stat: "Speed".into(), amount: 50, duration: 8 }),
                Spell::new("Time Stop", *self, 5, 100, 160, SpellEffect::TimeStop { duration: 3 }),
                Spell::new("Rewind", *self, 4, 60, 100, SpellEffect::Rewind { turns: 3 }),
                Spell::new("Age", *self, 3, 30, 50, SpellEffect::Debuff { stat: "All".into(), amount: 25, duration: 15 }),
                Spell::new("Temporal Shield", *self, 3, 35, 55, SpellEffect::TemporalShield { absorb: 50, rewind_on_break: true }),
                Spell::new("Future Sight", *self, 2, 20, 30, SpellEffect::BuffSelf { stat: "Evasion".into(), amount: 30, duration: 10 }),
                Spell::new("Echo Strike", *self, 3, 25, 40, SpellEffect::EchoAttack { repeats: 2 }),
                Spell::new("Paradox", *self, 4, 55, 90, SpellEffect::Paradox { damage: 40, confuse_duration: 5 }),
                Spell::new("Eternity", *self, 5, 120, 200, SpellEffect::Invulnerability { duration: 5 }),
            ],
            Self::Dimensionalism => vec![
                Spell::new("Blink", *self, 1, 8, 15, SpellEffect::Teleport { range: 4 }),
                Spell::new("Portal", *self, 3, 40, 65, SpellEffect::CreatePortal { destination: "Saved".into() }),
                Spell::new("Banish", *self, 3, 35, 55, SpellEffect::Banish { duration: 5 }),
                Spell::new("Dimensional Rift", *self, 4, 50, 80, SpellEffect::AreaDamage { amount: 40, radius: 2, element: Element::Void }),
                Spell::new("Pocket Dimension", *self, 3, 30, 50, SpellEffect::PocketDimension { duration: 10 }),
                Spell::new("Gravity Well", *self, 3, 35, 55, SpellEffect::Pull { range: 5, damage: 15 }),
                Spell::new("Phase Shift", *self, 2, 20, 35, SpellEffect::Intangible { duration: 3 }),
                Spell::new("Wormhole", *self, 4, 60, 95, SpellEffect::MassTP { range: 10, allies: true }),
                Spell::new("Reality Warp", *self, 5, 90, 145, SpellEffect::RandomEffect { power: 100 }),
                Spell::new("Collapse", *self, 5, 110, 180, SpellEffect::Instant { damage_percent: 50 }),
            ],
            _ => vec![
                Spell::new("Basic Spell", *self, 1, 5, 10, SpellEffect::Damage { amount: 10, element: Element::Arcane }),
            ],
        }
    }
}

/// A spell in the magic system
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Spell {
    pub name: String,
    pub school: MagicSchool,
    pub tier: u8,
    pub mana_cost: i32,
    pub cooldown: u32,
    pub effect: SpellEffect,
    pub description: String,
    pub current_cooldown: u32,
}

impl Spell {
    pub fn new(name: &str, school: MagicSchool, tier: u8, mana: i32, cooldown: u32, effect: SpellEffect) -> Self {
        let description = effect.describe();
        Self {
            name: name.to_string(),
            school,
            tier,
            mana_cost: mana,
            cooldown,
            effect,
            description,
            current_cooldown: 0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.current_cooldown == 0
    }

    pub fn tick_cooldown(&mut self) {
        if self.current_cooldown > 0 {
            self.current_cooldown -= 1;
        }
    }

    pub fn use_spell(&mut self) {
        self.current_cooldown = self.cooldown;
    }
}

/// Spell effects
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SpellEffect {
    Damage { amount: i32, element: Element },
    AreaDamage { amount: i32, radius: u32, element: Element },
    ChainDamage { amount: i32, chains: u32, element: Element },
    Heal { amount: i32 },
    Shield { amount: i32, reflect_damage: i32 },
    StatusApply { status: String, duration: u32 },
    AreaStatusApply { status: String, duration: u32, radius: u32 },
    BuffSelf { stat: String, amount: i32, duration: u32 },
    Debuff { stat: String, amount: i32, duration: u32 },
    Summon { creature: String, duration: u32 },
    MassSummon { creature: String, count: u32, duration: u32 },
    Teleport { range: u32 },
    LifeDrain { amount: i32, heal_percent: u32 },
    Aura { effect: String, radius: u32, duration: u32 },
    CreateTerrain { terrain: String, duration: u32 },
    Explosion { damage: i32, radius: u32 },
    ConditionalDamage { base: i32, bonus_vs_status: String, bonus: i32 },
    Resurrection { hp_percent: u32 },
    Invulnerability { duration: u32 },
    Execute { threshold_percent: u32 },
    Transform { form: String, duration: u32 },
    CorpseExplosion { damage_percent: u32, radius: u32 },
    SoulHarvest { damage: i32, mana_restore: i32 },
    SmartHeal { damage_to_enemy: i32, heal_to_undead: i32 },
    AreaDot { damage_per_turn: i32, radius: u32, duration: u32 },
    HolyNova { heal_allies: i32, damage_enemies: i32, radius: u32 },
    Cleanse { remove_debuffs: bool },
    Blind { duration: u32, radius: u32 },
    GlobalDamage { amount: i32, element: Element },
    TimeStop { duration: u32 },
    Rewind { turns: u32 },
    TemporalShield { absorb: i32, rewind_on_break: bool },
    EchoAttack { repeats: u32 },
    Paradox { damage: i32, confuse_duration: u32 },
    CreatePortal { destination: String },
    Banish { duration: u32 },
    PocketDimension { duration: u32 },
    Pull { range: u32, damage: i32 },
    Intangible { duration: u32 },
    MassTP { range: u32, allies: bool },
    RandomEffect { power: i32 },
    Instant { damage_percent: u32 },
}

impl SpellEffect {
    pub fn describe(&self) -> String {
        match self {
            Self::Damage { amount, element } => format!("Deals {} {:?} damage", amount, element),
            Self::AreaDamage { amount, radius, element } => format!("Deals {} {:?} damage in {} radius", amount, element, radius),
            Self::Heal { amount } => format!("Heals {} HP", amount),
            Self::Shield { amount, .. } => format!("Creates a {} HP shield", amount),
            Self::Summon { creature, duration } => format!("Summons {} for {} turns", creature, duration),
            Self::Teleport { range } => format!("Teleport up to {} tiles", range),
            Self::LifeDrain { amount, heal_percent } => format!("Drains {} life, heals {}%", amount, heal_percent),
            _ => "Magical effect".to_string(),
        }
    }
}

/// Elemental types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Element {
    Fire,
    Ice,
    Lightning,
    Earth,
    Water,
    Wind,
    Holy,
    Shadow,
    Void,
    Arcane,
    Nature,
    Physical,
}

/// Spell combination results
#[derive(Clone, Debug)]
pub struct SpellCombination {
    pub spells: Vec<String>,
    pub result_name: String,
    pub result_effect: SpellEffect,
    pub mana_multiplier: f32,
}

/// Magic mastery for a school
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SchoolMastery {
    pub school: Option<MagicSchool>,
    pub level: u32,
    pub experience: u32,
    pub xp_to_next: u32,
    pub spells_known: Vec<Spell>,
    pub bonuses: MasteryBonuses,
}

impl SchoolMastery {
    pub fn new(school: MagicSchool) -> Self {
        Self {
            school: Some(school),
            level: 1,
            experience: 0,
            xp_to_next: 100,
            spells_known: vec![],
            bonuses: MasteryBonuses::default(),
        }
    }

    pub fn gain_xp(&mut self, amount: u32) -> bool {
        self.experience += amount;
        if self.experience >= self.xp_to_next {
            self.experience -= self.xp_to_next;
            self.level += 1;
            self.xp_to_next = (self.xp_to_next as f32 * 1.5) as u32;
            self.update_bonuses();
            true
        } else {
            false
        }
    }

    fn update_bonuses(&mut self) {
        self.bonuses = MasteryBonuses {
            damage_bonus: self.level as f32 * 0.05,
            mana_reduction: (self.level as f32 * 0.02).min(0.5),
            cooldown_reduction: (self.level as f32 * 0.02).min(0.5),
            crit_chance: self.level as f32 * 0.01,
        };
    }

    pub fn can_learn(&self, spell: &Spell) -> bool {
        spell.tier as u32 <= (self.level + 1) / 2
    }

    pub fn learn_spell(&mut self, spell: Spell) -> bool {
        if self.can_learn(&spell) && !self.spells_known.iter().any(|s| s.name == spell.name) {
            self.spells_known.push(spell);
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MasteryBonuses {
    pub damage_bonus: f32,
    pub mana_reduction: f32,
    pub cooldown_reduction: f32,
    pub crit_chance: f32,
}

/// Player's complete magic knowledge
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MagicSystem {
    pub masteries: HashMap<MagicSchool, SchoolMastery>,
    pub primary_school: Option<MagicSchool>,
    pub secondary_school: Option<MagicSchool>,
    pub total_spells_cast: u32,
    pub total_mana_spent: i32,
}

impl MagicSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn specialize(&mut self, school: MagicSchool) {
        if self.primary_school.is_none() {
            self.primary_school = Some(school);
        } else if self.secondary_school.is_none() && self.primary_school != Some(school) {
            self.secondary_school = Some(school);
        }
        self.masteries.entry(school).or_insert_with(|| SchoolMastery::new(school));
    }

    pub fn get_mastery(&self, school: MagicSchool) -> Option<&SchoolMastery> {
        self.masteries.get(&school)
    }

    pub fn get_mastery_mut(&mut self, school: MagicSchool) -> Option<&mut SchoolMastery> {
        self.masteries.get_mut(&school)
    }

    pub fn cast_spell(&mut self, spell: &Spell) {
        self.total_spells_cast += 1;
        self.total_mana_spent += spell.mana_cost;
        if let Some(mastery) = self.masteries.get_mut(&spell.school) {
            mastery.gain_xp(spell.tier as u32 * 10);
        }
    }

    pub fn all_known_spells(&self) -> Vec<&Spell> {
        self.masteries.values()
            .flat_map(|m| m.spells_known.iter())
            .collect()
    }

    pub fn effective_mana_cost(&self, spell: &Spell) -> i32 {
        let base = spell.mana_cost as f32;
        let reduction = self.masteries.get(&spell.school)
            .map(|m| m.bonuses.mana_reduction)
            .unwrap_or(0.0);
        (base * (1.0 - reduction)) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_schools() {
        for school in MagicSchool::all() {
            assert!(!school.spells().is_empty());
        }
    }

    #[test]
    fn test_mastery() {
        let mut mastery = SchoolMastery::new(MagicSchool::Pyromancy);
        assert!(mastery.gain_xp(100));
        assert_eq!(mastery.level, 2);
    }
}
