//! Armor System for ShadowCrypt
//!
//! A comprehensive armor system for the dark fantasy roguelike, featuring
//! multiple armor slots, types, quality tiers, enchantments, set bonuses,
//! and legendary abilities.
//!
//! # Overview
//!
//! - **8 equipment slots** from head to cape
//! - **9 armor types** ranging from cloth to void-forged materials
//! - **9 quality tiers** with stat multipliers from 0.5x (Broken) to 3.0x (Divine)
//! - **26 enchantments** including 4 legendary-only enchantments
//! - **25 armor sets** across 4 rarity tiers with 2/4/6 piece bonuses
//! - **16 legendary abilities** unlocked through set completion

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// ArmorSlot
// ---------------------------------------------------------------------------

/// Equipment slot where an armor piece can be worn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArmorSlot {
    Head,
    Chest,
    Legs,
    Boots,
    Gloves,
    Shoulders,
    Belt,
    Cape,
}

impl ArmorSlot {
    /// Human-readable name of the slot.
    pub fn name(&self) -> &'static str {
        match self {
            ArmorSlot::Head => "Head",
            ArmorSlot::Chest => "Chest",
            ArmorSlot::Legs => "Legs",
            ArmorSlot::Boots => "Boots",
            ArmorSlot::Gloves => "Gloves",
            ArmorSlot::Shoulders => "Shoulders",
            ArmorSlot::Belt => "Belt",
            ArmorSlot::Cape => "Cape",
        }
    }

    /// Flavor description for the slot.
    pub fn description(&self) -> &'static str {
        match self {
            ArmorSlot::Head => "Protects the skull and mind from physical and psychic harm.",
            ArmorSlot::Chest => "The core defense layer shielding vital organs.",
            ArmorSlot::Legs => "Greaves and leggings that guard the lower body.",
            ArmorSlot::Boots => "Footwear forged to tread through blood-soaked battlefields.",
            ArmorSlot::Gloves => "Gauntlets that shield the hands without hindering grip.",
            ArmorSlot::Shoulders => "Pauldrons that deflect overhead blows and falling debris.",
            ArmorSlot::Belt => "A girdle that binds the armor together and holds enchanted pouches.",
            ArmorSlot::Cape => "A mantle woven with protective wards and dark enchantments.",
        }
    }

    /// The defense multiplier each slot contributes relative to a base armor type value.
    /// Chest provides the highest share; belt and cape the lowest.
    pub fn defense_multiplier(&self) -> f32 {
        match self {
            ArmorSlot::Head => 0.80,
            ArmorSlot::Chest => 1.00,
            ArmorSlot::Legs => 0.85,
            ArmorSlot::Boots => 0.60,
            ArmorSlot::Gloves => 0.55,
            ArmorSlot::Shoulders => 0.75,
            ArmorSlot::Belt => 0.40,
            ArmorSlot::Cape => 0.45,
        }
    }

    /// Returns all armor slots in equip order.
    pub fn all() -> &'static [ArmorSlot] {
        &[
            ArmorSlot::Head,
            ArmorSlot::Chest,
            ArmorSlot::Legs,
            ArmorSlot::Boots,
            ArmorSlot::Gloves,
            ArmorSlot::Shoulders,
            ArmorSlot::Belt,
            ArmorSlot::Cape,
        ]
    }
}

// ---------------------------------------------------------------------------
// ArmorType
// ---------------------------------------------------------------------------

/// The material an armor piece is forged from.  Higher-tier materials provide
/// more defense but weigh more and may require greater levels to equip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArmorType {
    Cloth,
    Leather,
    ChainMail,
    ScaleMail,
    PlateMail,
    Dragonscale,
    Crystal,
    Shadow,
    Void,
}

impl ArmorType {
    pub fn name(&self) -> &'static str {
        match self {
            ArmorType::Cloth => "Cloth",
            ArmorType::Leather => "Leather",
            ArmorType::ChainMail => "Chain Mail",
            ArmorType::ScaleMail => "Scale Mail",
            ArmorType::PlateMail => "Plate Mail",
            ArmorType::Dragonscale => "Dragonscale",
            ArmorType::Crystal => "Crystal",
            ArmorType::Shadow => "Shadow",
            ArmorType::Void => "Void",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ArmorType::Cloth => "Simple woven fabric offering minimal protection but unhindered spellcasting.",
            ArmorType::Leather => "Cured hides providing a balance of mobility and modest defense.",
            ArmorType::ChainMail => "Interlocking metal rings that deflect slashing attacks.",
            ArmorType::ScaleMail => "Overlapping metal scales riveted to a leather backing.",
            ArmorType::PlateMail => "Full forged steel plates offering heavy protection at the cost of agility.",
            ArmorType::Dragonscale => "Scales shed by elder wyrms, nearly impervious to elemental damage.",
            ArmorType::Crystal => "Arcane-infused crystalline plating that resonates with magical energy.",
            ArmorType::Shadow => "Armor woven from solidified darkness, nearly weightless yet resilient.",
            ArmorType::Void => "Forged in the space between worlds, it devours incoming force.",
        }
    }

    /// Base physical defense value for the armor type (before slot/quality modifiers).
    pub fn base_defense(&self) -> i32 {
        match self {
            ArmorType::Cloth => 5,
            ArmorType::Leather => 12,
            ArmorType::ChainMail => 20,
            ArmorType::ScaleMail => 28,
            ArmorType::PlateMail => 38,
            ArmorType::Dragonscale => 50,
            ArmorType::Crystal => 45,
            ArmorType::Shadow => 42,
            ArmorType::Void => 60,
        }
    }

    /// Base magic defense value.
    pub fn base_magic_defense(&self) -> i32 {
        match self {
            ArmorType::Cloth => 20,
            ArmorType::Leather => 15,
            ArmorType::ChainMail => 10,
            ArmorType::ScaleMail => 12,
            ArmorType::PlateMail => 8,
            ArmorType::Dragonscale => 35,
            ArmorType::Crystal => 55,
            ArmorType::Shadow => 40,
            ArmorType::Void => 50,
        }
    }

    /// Base weight in abstract units.
    pub fn base_weight(&self) -> f32 {
        match self {
            ArmorType::Cloth => 1.0,
            ArmorType::Leather => 3.0,
            ArmorType::ChainMail => 6.0,
            ArmorType::ScaleMail => 8.0,
            ArmorType::PlateMail => 12.0,
            ArmorType::Dragonscale => 10.0,
            ArmorType::Crystal => 7.0,
            ArmorType::Shadow => 2.0,
            ArmorType::Void => 0.5,
        }
    }

    /// Minimum character level required to equip this type.
    pub fn level_requirement(&self) -> u32 {
        match self {
            ArmorType::Cloth => 1,
            ArmorType::Leather => 3,
            ArmorType::ChainMail => 8,
            ArmorType::ScaleMail => 14,
            ArmorType::PlateMail => 20,
            ArmorType::Dragonscale => 35,
            ArmorType::Crystal => 40,
            ArmorType::Shadow => 45,
            ArmorType::Void => 55,
        }
    }

    /// Returns all armor types in ascending power order.
    pub fn all() -> &'static [ArmorType] {
        &[
            ArmorType::Cloth,
            ArmorType::Leather,
            ArmorType::ChainMail,
            ArmorType::ScaleMail,
            ArmorType::PlateMail,
            ArmorType::Dragonscale,
            ArmorType::Crystal,
            ArmorType::Shadow,
            ArmorType::Void,
        ]
    }
}

// ---------------------------------------------------------------------------
// ArmorQuality
// ---------------------------------------------------------------------------

/// Quality tier of an armor piece.  Each tier applies a stat multiplier to
/// the base defense and magic defense values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArmorQuality {
    Broken,
    Crude,
    Common,
    Fine,
    Superior,
    Masterwork,
    Legendary,
    Mythic,
    Divine,
}

impl ArmorQuality {
    pub fn name(&self) -> &'static str {
        match self {
            ArmorQuality::Broken => "Broken",
            ArmorQuality::Crude => "Crude",
            ArmorQuality::Common => "Common",
            ArmorQuality::Fine => "Fine",
            ArmorQuality::Superior => "Superior",
            ArmorQuality::Masterwork => "Masterwork",
            ArmorQuality::Legendary => "Legendary",
            ArmorQuality::Mythic => "Mythic",
            ArmorQuality::Divine => "Divine",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ArmorQuality::Broken => "Cracked and barely holding together. Better than nothing.",
            ArmorQuality::Crude => "Roughly fashioned with obvious flaws in its construction.",
            ArmorQuality::Common => "Standard craftsmanship meeting baseline expectations.",
            ArmorQuality::Fine => "Carefully worked with attention to structural integrity.",
            ArmorQuality::Superior => "Expert craftsmanship that exceeds ordinary standards.",
            ArmorQuality::Masterwork => "The pinnacle of mortal smithing, forged by a true master.",
            ArmorQuality::Legendary => "A once-in-an-age creation whispered of in tavern tales.",
            ArmorQuality::Mythic => "Touched by forces beyond mortal comprehension.",
            ArmorQuality::Divine => "Wrought by the gods themselves, perfection made manifest.",
        }
    }

    /// The multiplier applied to base defense and magic defense values.
    pub fn stat_multiplier(&self) -> f32 {
        match self {
            ArmorQuality::Broken => 0.5,
            ArmorQuality::Crude => 0.7,
            ArmorQuality::Common => 1.0,
            ArmorQuality::Fine => 1.2,
            ArmorQuality::Superior => 1.4,
            ArmorQuality::Masterwork => 1.6,
            ArmorQuality::Legendary => 2.0,
            ArmorQuality::Mythic => 2.5,
            ArmorQuality::Divine => 3.0,
        }
    }

    /// Bonus enchantment slots granted by quality.
    pub fn bonus_enchantment_slots(&self) -> usize {
        match self {
            ArmorQuality::Broken => 0,
            ArmorQuality::Crude => 0,
            ArmorQuality::Common => 1,
            ArmorQuality::Fine => 1,
            ArmorQuality::Superior => 2,
            ArmorQuality::Masterwork => 2,
            ArmorQuality::Legendary => 3,
            ArmorQuality::Mythic => 4,
            ArmorQuality::Divine => 5,
        }
    }

    /// Maximum durability multiplier relative to a base of 100.
    pub fn durability_multiplier(&self) -> f32 {
        match self {
            ArmorQuality::Broken => 0.3,
            ArmorQuality::Crude => 0.6,
            ArmorQuality::Common => 1.0,
            ArmorQuality::Fine => 1.3,
            ArmorQuality::Superior => 1.6,
            ArmorQuality::Masterwork => 2.0,
            ArmorQuality::Legendary => 3.0,
            ArmorQuality::Mythic => 5.0,
            ArmorQuality::Divine => 10.0,
        }
    }

    /// Returns all quality tiers in ascending order.
    pub fn all() -> &'static [ArmorQuality] {
        &[
            ArmorQuality::Broken,
            ArmorQuality::Crude,
            ArmorQuality::Common,
            ArmorQuality::Fine,
            ArmorQuality::Superior,
            ArmorQuality::Masterwork,
            ArmorQuality::Legendary,
            ArmorQuality::Mythic,
            ArmorQuality::Divine,
        ]
    }
}

// ---------------------------------------------------------------------------
// ArmorEnchantment
// ---------------------------------------------------------------------------

/// Enchantments that can be applied to armor pieces.  The last four are
/// legendary-grade and can only appear on Legendary/Mythic/Divine quality items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArmorEnchantment {
    // --- Standard enchantments (22) ---
    FireResist,
    IceResist,
    LightningResist,
    PoisonResist,
    Thorns,
    Fortification,
    Vitality,
    Agility,
    Stealth,
    ManaShield,
    Reflection,
    Absorption,
    Regeneration,
    SpeedBoost,
    LifeSteal,
    ManaSteal,
    ElementalWard,
    HolyProtection,
    ShadowCloak,
    NecroticShield,
    ArcaneBarrier,
    DamageReduction,

    // --- Legendary enchantments (4) ---
    PhysicalImmunity,
    ElementalImmunity,
    DivineAegis,
    VoidShield,
}

impl ArmorEnchantment {
    pub fn name(&self) -> &'static str {
        match self {
            ArmorEnchantment::FireResist => "Fire Resistance",
            ArmorEnchantment::IceResist => "Ice Resistance",
            ArmorEnchantment::LightningResist => "Lightning Resistance",
            ArmorEnchantment::PoisonResist => "Poison Resistance",
            ArmorEnchantment::Thorns => "Thorns",
            ArmorEnchantment::Fortification => "Fortification",
            ArmorEnchantment::Vitality => "Vitality",
            ArmorEnchantment::Agility => "Agility",
            ArmorEnchantment::Stealth => "Stealth",
            ArmorEnchantment::ManaShield => "Mana Shield",
            ArmorEnchantment::Reflection => "Reflection",
            ArmorEnchantment::Absorption => "Absorption",
            ArmorEnchantment::Regeneration => "Regeneration",
            ArmorEnchantment::SpeedBoost => "Speed Boost",
            ArmorEnchantment::LifeSteal => "Life Steal",
            ArmorEnchantment::ManaSteal => "Mana Steal",
            ArmorEnchantment::ElementalWard => "Elemental Ward",
            ArmorEnchantment::HolyProtection => "Holy Protection",
            ArmorEnchantment::ShadowCloak => "Shadow Cloak",
            ArmorEnchantment::NecroticShield => "Necrotic Shield",
            ArmorEnchantment::ArcaneBarrier => "Arcane Barrier",
            ArmorEnchantment::DamageReduction => "Damage Reduction",
            ArmorEnchantment::PhysicalImmunity => "Physical Immunity",
            ArmorEnchantment::ElementalImmunity => "Elemental Immunity",
            ArmorEnchantment::DivineAegis => "Divine Aegis",
            ArmorEnchantment::VoidShield => "Void Shield",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ArmorEnchantment::FireResist => "Reduces fire damage taken by 25%.",
            ArmorEnchantment::IceResist => "Reduces ice damage taken by 25%.",
            ArmorEnchantment::LightningResist => "Reduces lightning damage taken by 25%.",
            ArmorEnchantment::PoisonResist => "Reduces poison damage taken by 25% and shortens poison duration.",
            ArmorEnchantment::Thorns => "Reflects 15% of melee damage back to the attacker.",
            ArmorEnchantment::Fortification => "Increases maximum HP by 10%.",
            ArmorEnchantment::Vitality => "Grants +20 flat HP bonus.",
            ArmorEnchantment::Agility => "Increases dodge chance by 5%.",
            ArmorEnchantment::Stealth => "Reduces enemy detection range by 15%.",
            ArmorEnchantment::ManaShield => "Absorbs 20% of incoming damage using mana instead of HP.",
            ArmorEnchantment::Reflection => "10% chance to fully reflect a spell back at the caster.",
            ArmorEnchantment::Absorption => "Converts 8% of damage taken into mana.",
            ArmorEnchantment::Regeneration => "Restores 2 HP per turn.",
            ArmorEnchantment::SpeedBoost => "Increases movement speed by 10%.",
            ArmorEnchantment::LifeSteal => "Heals for 5% of physical damage dealt.",
            ArmorEnchantment::ManaSteal => "Restores mana equal to 5% of magic damage dealt.",
            ArmorEnchantment::ElementalWard => "Reduces all elemental damage by 10%.",
            ArmorEnchantment::HolyProtection => "Grants 20% resistance to undead and demonic attacks.",
            ArmorEnchantment::ShadowCloak => "Grants partial invisibility in dark areas.",
            ArmorEnchantment::NecroticShield => "Grants 20% resistance to necrotic and death magic.",
            ArmorEnchantment::ArcaneBarrier => "Creates a barrier absorbing the first 30 magic damage per encounter.",
            ArmorEnchantment::DamageReduction => "Flat reduction of 5 from all incoming damage.",
            ArmorEnchantment::PhysicalImmunity => "[LEGENDARY] Grants full immunity to physical damage for 3 seconds upon taking a lethal blow (once per encounter).",
            ArmorEnchantment::ElementalImmunity => "[LEGENDARY] Grants full immunity to all elemental damage for 3 seconds upon taking a lethal blow (once per encounter).",
            ArmorEnchantment::DivineAegis => "[LEGENDARY] An angelic shield absorbs the next fatal blow and restores 50% HP (once per floor).",
            ArmorEnchantment::VoidShield => "[LEGENDARY] Banishes all incoming damage to the void for 5 seconds (once per floor).",
        }
    }

    /// Whether this enchantment is legendary-grade (only for Legendary+ quality).
    pub fn is_legendary(&self) -> bool {
        matches!(
            self,
            ArmorEnchantment::PhysicalImmunity
                | ArmorEnchantment::ElementalImmunity
                | ArmorEnchantment::DivineAegis
                | ArmorEnchantment::VoidShield
        )
    }

    /// Flat bonus to physical defense granted by the enchantment.
    pub fn defense_bonus(&self) -> i32 {
        match self {
            ArmorEnchantment::Fortification => 10,
            ArmorEnchantment::DamageReduction => 5,
            ArmorEnchantment::PhysicalImmunity => 25,
            ArmorEnchantment::DivineAegis => 20,
            ArmorEnchantment::VoidShield => 30,
            _ => 0,
        }
    }

    /// Flat bonus to magic defense granted by the enchantment.
    pub fn magic_defense_bonus(&self) -> i32 {
        match self {
            ArmorEnchantment::ManaShield => 10,
            ArmorEnchantment::ArcaneBarrier => 15,
            ArmorEnchantment::ElementalWard => 8,
            ArmorEnchantment::HolyProtection => 12,
            ArmorEnchantment::NecroticShield => 12,
            ArmorEnchantment::Reflection => 10,
            ArmorEnchantment::ElementalImmunity => 30,
            ArmorEnchantment::DivineAegis => 20,
            ArmorEnchantment::VoidShield => 30,
            _ => 0,
        }
    }

    /// Flat HP bonus from the enchantment.
    pub fn hp_bonus(&self) -> i32 {
        match self {
            ArmorEnchantment::Vitality => 20,
            ArmorEnchantment::Fortification => 15,
            ArmorEnchantment::Regeneration => 5,
            ArmorEnchantment::DivineAegis => 50,
            _ => 0,
        }
    }

    /// Flat mana bonus from the enchantment.
    pub fn mana_bonus(&self) -> i32 {
        match self {
            ArmorEnchantment::ManaShield => 15,
            ArmorEnchantment::ManaSteal => 10,
            ArmorEnchantment::ArcaneBarrier => 20,
            _ => 0,
        }
    }

    /// Speed modifier as a percentage (e.g. 10 means +10%).
    pub fn speed_bonus(&self) -> i32 {
        match self {
            ArmorEnchantment::SpeedBoost => 10,
            ArmorEnchantment::Agility => 5,
            ArmorEnchantment::ShadowCloak => 5,
            _ => 0,
        }
    }
}

// ---------------------------------------------------------------------------
// LegendaryAbility
// ---------------------------------------------------------------------------

/// Powerful abilities that can be granted by completing legendary armor sets
/// or through exceptionally rare enchantment combinations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegendaryAbility {
    Invulnerability,
    TimeStop,
    Resurrection,
    Omniscience,
    ElementalMastery,
    ShadowRealm,
    DivineJudgment,
    VoidRift,
    DragonForm,
    PhoenixRebirth,
    UnholyAura,
    NatureWrath,
    ArcaneSupremacy,
    CelestialAscension,
    DemonicTransformation,
    PrimordialPower,
}

impl LegendaryAbility {
    pub fn name(&self) -> &'static str {
        match self {
            LegendaryAbility::Invulnerability => "Invulnerability",
            LegendaryAbility::TimeStop => "Time Stop",
            LegendaryAbility::Resurrection => "Resurrection",
            LegendaryAbility::Omniscience => "Omniscience",
            LegendaryAbility::ElementalMastery => "Elemental Mastery",
            LegendaryAbility::ShadowRealm => "Shadow Realm",
            LegendaryAbility::DivineJudgment => "Divine Judgment",
            LegendaryAbility::VoidRift => "Void Rift",
            LegendaryAbility::DragonForm => "Dragon Form",
            LegendaryAbility::PhoenixRebirth => "Phoenix Rebirth",
            LegendaryAbility::UnholyAura => "Unholy Aura",
            LegendaryAbility::NatureWrath => "Nature's Wrath",
            LegendaryAbility::ArcaneSupremacy => "Arcane Supremacy",
            LegendaryAbility::CelestialAscension => "Celestial Ascension",
            LegendaryAbility::DemonicTransformation => "Demonic Transformation",
            LegendaryAbility::PrimordialPower => "Primordial Power",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            LegendaryAbility::Invulnerability => "Become completely immune to all damage for 10 seconds. Cooldown: once per floor.",
            LegendaryAbility::TimeStop => "Freeze time for all enemies for 8 seconds, allowing free actions. Cooldown: once per floor.",
            LegendaryAbility::Resurrection => "Automatically revive with full HP upon death. Cooldown: once per dungeon run.",
            LegendaryAbility::Omniscience => "Reveal the entire floor map, all enemies, traps, and hidden rooms for 60 seconds.",
            LegendaryAbility::ElementalMastery => "All elemental damage dealt is doubled and all elemental damage taken is halved for 15 seconds.",
            LegendaryAbility::ShadowRealm => "Phase into the shadow realm for 10 seconds, becoming untargetable while still able to attack.",
            LegendaryAbility::DivineJudgment => "Call down a pillar of holy light dealing massive damage to all enemies in sight.",
            LegendaryAbility::VoidRift => "Tear open a rift that pulls all nearby enemies in and deals continuous void damage.",
            LegendaryAbility::DragonForm => "Transform into an ancient dragon for 20 seconds, gaining flight, breath attacks, and immense stats.",
            LegendaryAbility::PhoenixRebirth => "Upon death, explode in flame dealing area damage and revive with 75% HP.",
            LegendaryAbility::UnholyAura => "Emit a necrotic aura that drains HP from all nearby enemies and heals the wearer.",
            LegendaryAbility::NatureWrath => "Summon a storm of thorns and lightning that devastates the entire floor.",
            LegendaryAbility::ArcaneSupremacy => "All spells cost no mana and deal 50% more damage for 20 seconds.",
            LegendaryAbility::CelestialAscension => "Ascend to a higher plane, gaining +100% to all stats for 15 seconds.",
            LegendaryAbility::DemonicTransformation => "Transform into a demon lord, gaining immense attack power but taking 10% HP drain per second.",
            LegendaryAbility::PrimordialPower => "Channel the raw energy of creation, dealing true damage that ignores all resistances.",
        }
    }

    /// Cooldown in turns (0 = once per encounter, negative = once per floor/run).
    pub fn cooldown_turns(&self) -> i32 {
        match self {
            LegendaryAbility::Invulnerability => -1,    // once per floor
            LegendaryAbility::TimeStop => -1,            // once per floor
            LegendaryAbility::Resurrection => -100,      // once per dungeon run
            LegendaryAbility::Omniscience => 50,
            LegendaryAbility::ElementalMastery => 30,
            LegendaryAbility::ShadowRealm => -1,
            LegendaryAbility::DivineJudgment => 40,
            LegendaryAbility::VoidRift => 45,
            LegendaryAbility::DragonForm => -1,
            LegendaryAbility::PhoenixRebirth => -100,
            LegendaryAbility::UnholyAura => 25,
            LegendaryAbility::NatureWrath => -1,
            LegendaryAbility::ArcaneSupremacy => 35,
            LegendaryAbility::CelestialAscension => -1,
            LegendaryAbility::DemonicTransformation => 30,
            LegendaryAbility::PrimordialPower => 50,
        }
    }
}

// ---------------------------------------------------------------------------
// ArmorSetBonus
// ---------------------------------------------------------------------------

/// Stat bonuses granted when a certain number of pieces from a set are equipped.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArmorSetBonus {
    /// Pieces required to activate this bonus tier.
    pub pieces_required: usize,
    /// Flat bonus to physical defense.
    pub defense: i32,
    /// Flat bonus to magic defense.
    pub magic_defense: i32,
    /// Flat bonus to maximum HP.
    pub hp: i32,
    /// Flat bonus to maximum mana.
    pub mana: i32,
    /// Speed modifier as a percentage.
    pub speed: i32,
    /// Optional legendary ability unlocked at this tier.
    pub special_ability: Option<LegendaryAbility>,
}

impl ArmorSetBonus {
    /// Create a new set bonus.
    pub fn new(
        pieces_required: usize,
        defense: i32,
        magic_defense: i32,
        hp: i32,
        mana: i32,
        speed: i32,
        special_ability: Option<LegendaryAbility>,
    ) -> Self {
        Self {
            pieces_required,
            defense,
            magic_defense,
            hp,
            mana,
            speed,
            special_ability,
        }
    }
}

// ---------------------------------------------------------------------------
// ArmorSet
// ---------------------------------------------------------------------------

/// Named armor sets.  Equipping multiple pieces from the same set activates
/// increasingly powerful bonuses.  Sets are organized into four rarity tiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArmorSet {
    // --- Common (5) ---
    LeatherTraveler,
    IronGuard,
    ApprenticeRobes,
    ScoutsGarb,
    MilitiaPlate,

    // --- Rare (7) ---
    Dragonscale,
    ShadowWeave,
    HolyCrusader,
    FrostGiant,
    PhoenixFeather,
    Stormcaller,
    BloodKnight,

    // --- Epic (7) ---
    VoidWalker,
    CelestialRadiance,
    DemonLord,
    AncientGuardian,
    ArchmagesRegalia,
    DeathKnight,
    NaturesEmbrace,

    // --- Legendary (6) ---
    ArmorOfTheGods,
    ShadowSovereign,
    DragonEmperor,
    VoidTyrant,
    CelestialArbiter,
    PrimordialTitan,
}

impl ArmorSet {
    pub fn name(&self) -> &'static str {
        match self {
            // Common
            ArmorSet::LeatherTraveler => "Leather Traveler",
            ArmorSet::IronGuard => "Iron Guard",
            ArmorSet::ApprenticeRobes => "Apprentice Robes",
            ArmorSet::ScoutsGarb => "Scout's Garb",
            ArmorSet::MilitiaPlate => "Militia Plate",
            // Rare
            ArmorSet::Dragonscale => "Dragonscale",
            ArmorSet::ShadowWeave => "Shadow Weave",
            ArmorSet::HolyCrusader => "Holy Crusader",
            ArmorSet::FrostGiant => "Frost Giant",
            ArmorSet::PhoenixFeather => "Phoenix Feather",
            ArmorSet::Stormcaller => "Stormcaller",
            ArmorSet::BloodKnight => "Blood Knight",
            // Epic
            ArmorSet::VoidWalker => "Void Walker",
            ArmorSet::CelestialRadiance => "Celestial Radiance",
            ArmorSet::DemonLord => "Demon Lord",
            ArmorSet::AncientGuardian => "Ancient Guardian",
            ArmorSet::ArchmagesRegalia => "Archmage's Regalia",
            ArmorSet::DeathKnight => "Death Knight",
            ArmorSet::NaturesEmbrace => "Nature's Embrace",
            // Legendary
            ArmorSet::ArmorOfTheGods => "Armor of the Gods",
            ArmorSet::ShadowSovereign => "Shadow Sovereign",
            ArmorSet::DragonEmperor => "Dragon Emperor",
            ArmorSet::VoidTyrant => "Void Tyrant",
            ArmorSet::CelestialArbiter => "Celestial Arbiter",
            ArmorSet::PrimordialTitan => "Primordial Titan",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ArmorSet::LeatherTraveler => "Well-worn leather armor favored by wandering adventurers.",
            ArmorSet::IronGuard => "Standard-issue iron plate worn by city guardsmen.",
            ArmorSet::ApprenticeRobes => "Enchanted robes given to novice mages upon initiation.",
            ArmorSet::ScoutsGarb => "Lightweight gear designed for silent reconnaissance.",
            ArmorSet::MilitiaPlate => "Mass-produced plate armor issued to conscripted soldiers.",
            ArmorSet::Dragonscale => "Armor forged from scales of slain dragons, shimmering with residual fire.",
            ArmorSet::ShadowWeave => "Woven from threads of living shadow, found only in the Abyssal Markets.",
            ArmorSet::HolyCrusader => "Blessed armor of the Radiant Order, searing undead on contact.",
            ArmorSet::FrostGiant => "Armor carved from the bones of a frost giant jarl.",
            ArmorSet::PhoenixFeather => "Feathers of a dying phoenix, fused into armor that burns with renewal.",
            ArmorSet::Stormcaller => "Crackling with captured lightning, forged during a magical tempest.",
            ArmorSet::BloodKnight => "Crimson armor that drinks the blood of fallen foes.",
            ArmorSet::VoidWalker => "Armor that phases between dimensions, existing in multiple planes at once.",
            ArmorSet::CelestialRadiance => "Forged in the heart of a dying star, blinding in its brilliance.",
            ArmorSet::DemonLord => "Infernal armor torn from a vanquished demon prince.",
            ArmorSet::AncientGuardian => "Relic armor from the First Age, predating recorded history.",
            ArmorSet::ArchmagesRegalia => "The ceremonial garb of the Grand Archmage, thrumming with raw mana.",
            ArmorSet::DeathKnight => "Cursed plate that binds the wearer between life and death.",
            ArmorSet::NaturesEmbrace => "Living bark and vine armor grown by the World Tree itself.",
            ArmorSet::ArmorOfTheGods => "Divine plate forged by the gods during the Celestial War.",
            ArmorSet::ShadowSovereign => "The royal regalia of the Shadow King who rules the Dark Plane.",
            ArmorSet::DragonEmperor => "Armor fused from the scales of all five elder dragons.",
            ArmorSet::VoidTyrant => "Born from the nothingness between universes, it hungers for reality.",
            ArmorSet::CelestialArbiter => "Worn by the judge of souls at the gates of the afterlife.",
            ArmorSet::PrimordialTitan => "Hewn from the bones of the first being to exist before the gods.",
        }
    }

    /// The rarity tier of this set.
    pub fn tier(&self) -> &'static str {
        match self {
            ArmorSet::LeatherTraveler
            | ArmorSet::IronGuard
            | ArmorSet::ApprenticeRobes
            | ArmorSet::ScoutsGarb
            | ArmorSet::MilitiaPlate => "Common",

            ArmorSet::Dragonscale
            | ArmorSet::ShadowWeave
            | ArmorSet::HolyCrusader
            | ArmorSet::FrostGiant
            | ArmorSet::PhoenixFeather
            | ArmorSet::Stormcaller
            | ArmorSet::BloodKnight => "Rare",

            ArmorSet::VoidWalker
            | ArmorSet::CelestialRadiance
            | ArmorSet::DemonLord
            | ArmorSet::AncientGuardian
            | ArmorSet::ArchmagesRegalia
            | ArmorSet::DeathKnight
            | ArmorSet::NaturesEmbrace => "Epic",

            ArmorSet::ArmorOfTheGods
            | ArmorSet::ShadowSovereign
            | ArmorSet::DragonEmperor
            | ArmorSet::VoidTyrant
            | ArmorSet::CelestialArbiter
            | ArmorSet::PrimordialTitan => "Legendary",
        }
    }

    /// Return the full list of set bonuses (2-piece, 4-piece, 6-piece) for
    /// this armor set.  Each entry is an `ArmorSetBonus`.
    pub fn set_bonuses(&self) -> Vec<ArmorSetBonus> {
        match self {
            // ---------------------------------------------------------------
            // Common sets
            // ---------------------------------------------------------------
            ArmorSet::LeatherTraveler => vec![
                ArmorSetBonus::new(2, 5, 3, 10, 0, 5, None),
                ArmorSetBonus::new(4, 12, 8, 25, 5, 10, None),
                ArmorSetBonus::new(6, 20, 15, 50, 10, 15, None),
            ],
            ArmorSet::IronGuard => vec![
                ArmorSetBonus::new(2, 10, 2, 15, 0, 0, None),
                ArmorSetBonus::new(4, 25, 5, 40, 0, -5, None),
                ArmorSetBonus::new(6, 45, 10, 80, 0, -10, None),
            ],
            ArmorSet::ApprenticeRobes => vec![
                ArmorSetBonus::new(2, 2, 10, 5, 20, 0, None),
                ArmorSetBonus::new(4, 5, 22, 10, 50, 0, None),
                ArmorSetBonus::new(6, 8, 40, 20, 100, 5, None),
            ],
            ArmorSet::ScoutsGarb => vec![
                ArmorSetBonus::new(2, 4, 4, 8, 5, 10, None),
                ArmorSetBonus::new(4, 10, 10, 18, 12, 20, None),
                ArmorSetBonus::new(6, 18, 18, 35, 25, 30, None),
            ],
            ArmorSet::MilitiaPlate => vec![
                ArmorSetBonus::new(2, 8, 2, 20, 0, -3, None),
                ArmorSetBonus::new(4, 20, 5, 50, 0, -5, None),
                ArmorSetBonus::new(6, 35, 8, 100, 0, -8, None),
            ],

            // ---------------------------------------------------------------
            // Rare sets
            // ---------------------------------------------------------------
            ArmorSet::Dragonscale => vec![
                ArmorSetBonus::new(2, 15, 15, 25, 10, 0, None),
                ArmorSetBonus::new(4, 35, 35, 60, 25, 5, None),
                ArmorSetBonus::new(6, 60, 60, 120, 50, 10, None),
            ],
            ArmorSet::ShadowWeave => vec![
                ArmorSetBonus::new(2, 8, 12, 10, 15, 15, None),
                ArmorSetBonus::new(4, 18, 28, 25, 35, 25, None),
                ArmorSetBonus::new(6, 30, 50, 50, 70, 35, None),
            ],
            ArmorSet::HolyCrusader => vec![
                ArmorSetBonus::new(2, 12, 12, 30, 15, 0, None),
                ArmorSetBonus::new(4, 28, 28, 70, 35, 0, None),
                ArmorSetBonus::new(6, 50, 50, 140, 70, 5, None),
            ],
            ArmorSet::FrostGiant => vec![
                ArmorSetBonus::new(2, 18, 8, 35, 5, -5, None),
                ArmorSetBonus::new(4, 40, 18, 80, 10, -10, None),
                ArmorSetBonus::new(6, 70, 30, 160, 20, -15, None),
            ],
            ArmorSet::PhoenixFeather => vec![
                ArmorSetBonus::new(2, 6, 14, 15, 20, 10, None),
                ArmorSetBonus::new(4, 14, 32, 35, 50, 15, None),
                ArmorSetBonus::new(6, 25, 55, 70, 100, 20, None),
            ],
            ArmorSet::Stormcaller => vec![
                ArmorSetBonus::new(2, 8, 14, 15, 20, 8, None),
                ArmorSetBonus::new(4, 18, 32, 35, 45, 15, None),
                ArmorSetBonus::new(6, 30, 55, 70, 90, 22, None),
            ],
            ArmorSet::BloodKnight => vec![
                ArmorSetBonus::new(2, 14, 6, 40, 0, 5, None),
                ArmorSetBonus::new(4, 32, 14, 90, 0, 10, None),
                ArmorSetBonus::new(6, 55, 25, 180, 0, 15, None),
            ],

            // ---------------------------------------------------------------
            // Epic sets
            // ---------------------------------------------------------------
            ArmorSet::VoidWalker => vec![
                ArmorSetBonus::new(2, 20, 20, 30, 30, 10, None),
                ArmorSetBonus::new(4, 45, 45, 70, 70, 20, None),
                ArmorSetBonus::new(6, 80, 80, 150, 150, 30, Some(LegendaryAbility::VoidRift)),
            ],
            ArmorSet::CelestialRadiance => vec![
                ArmorSetBonus::new(2, 18, 25, 35, 40, 5, None),
                ArmorSetBonus::new(4, 40, 55, 80, 90, 10, None),
                ArmorSetBonus::new(6, 70, 100, 160, 180, 15, Some(LegendaryAbility::CelestialAscension)),
            ],
            ArmorSet::DemonLord => vec![
                ArmorSetBonus::new(2, 25, 15, 50, 20, 8, None),
                ArmorSetBonus::new(4, 55, 35, 110, 45, 15, None),
                ArmorSetBonus::new(6, 100, 60, 220, 90, 20, Some(LegendaryAbility::DemonicTransformation)),
            ],
            ArmorSet::AncientGuardian => vec![
                ArmorSetBonus::new(2, 28, 18, 45, 15, 0, None),
                ArmorSetBonus::new(4, 60, 40, 100, 35, 0, None),
                ArmorSetBonus::new(6, 110, 70, 200, 70, 5, Some(LegendaryAbility::Invulnerability)),
            ],
            ArmorSet::ArchmagesRegalia => vec![
                ArmorSetBonus::new(2, 10, 30, 20, 50, 5, None),
                ArmorSetBonus::new(4, 22, 65, 45, 110, 10, None),
                ArmorSetBonus::new(6, 40, 120, 90, 220, 15, Some(LegendaryAbility::ArcaneSupremacy)),
            ],
            ArmorSet::DeathKnight => vec![
                ArmorSetBonus::new(2, 22, 18, 40, 20, 5, None),
                ArmorSetBonus::new(4, 50, 40, 90, 45, 10, None),
                ArmorSetBonus::new(6, 90, 70, 180, 90, 15, Some(LegendaryAbility::UnholyAura)),
            ],
            ArmorSet::NaturesEmbrace => vec![
                ArmorSetBonus::new(2, 15, 22, 35, 35, 8, None),
                ArmorSetBonus::new(4, 35, 50, 80, 80, 15, None),
                ArmorSetBonus::new(6, 60, 90, 160, 160, 25, Some(LegendaryAbility::NatureWrath)),
            ],

            // ---------------------------------------------------------------
            // Legendary sets
            // ---------------------------------------------------------------
            ArmorSet::ArmorOfTheGods => vec![
                ArmorSetBonus::new(2, 35, 35, 60, 60, 10, None),
                ArmorSetBonus::new(4, 80, 80, 150, 150, 20, Some(LegendaryAbility::DivineJudgment)),
                ArmorSetBonus::new(6, 150, 150, 300, 300, 30, Some(LegendaryAbility::Invulnerability)),
            ],
            ArmorSet::ShadowSovereign => vec![
                ArmorSetBonus::new(2, 25, 35, 40, 50, 20, None),
                ArmorSetBonus::new(4, 55, 80, 90, 120, 35, Some(LegendaryAbility::ShadowRealm)),
                ArmorSetBonus::new(6, 100, 150, 180, 240, 50, Some(LegendaryAbility::TimeStop)),
            ],
            ArmorSet::DragonEmperor => vec![
                ArmorSetBonus::new(2, 40, 30, 70, 40, 10, None),
                ArmorSetBonus::new(4, 90, 70, 160, 90, 15, Some(LegendaryAbility::DragonForm)),
                ArmorSetBonus::new(6, 170, 130, 320, 180, 25, Some(LegendaryAbility::PhoenixRebirth)),
            ],
            ArmorSet::VoidTyrant => vec![
                ArmorSetBonus::new(2, 30, 40, 50, 60, 15, None),
                ArmorSetBonus::new(4, 70, 90, 120, 140, 25, Some(LegendaryAbility::VoidRift)),
                ArmorSetBonus::new(6, 130, 170, 240, 280, 40, Some(LegendaryAbility::Omniscience)),
            ],
            ArmorSet::CelestialArbiter => vec![
                ArmorSetBonus::new(2, 32, 38, 55, 65, 12, None),
                ArmorSetBonus::new(4, 75, 85, 130, 150, 22, Some(LegendaryAbility::CelestialAscension)),
                ArmorSetBonus::new(6, 140, 160, 260, 300, 35, Some(LegendaryAbility::Resurrection)),
            ],
            ArmorSet::PrimordialTitan => vec![
                ArmorSetBonus::new(2, 45, 30, 80, 40, 5, None),
                ArmorSetBonus::new(4, 100, 70, 180, 90, 10, Some(LegendaryAbility::ElementalMastery)),
                ArmorSetBonus::new(6, 190, 130, 360, 180, 15, Some(LegendaryAbility::PrimordialPower)),
            ],
        }
    }

    /// Return only the bonuses that are active given the number of equipped pieces.
    pub fn get_active_bonuses(&self, equipped_count: usize) -> Vec<ArmorSetBonus> {
        self.set_bonuses()
            .into_iter()
            .filter(|b| equipped_count >= b.pieces_required)
            .collect()
    }

    /// Compute combined totals for all active set bonuses at `equipped_count` pieces.
    pub fn total_active_bonus(&self, equipped_count: usize) -> ArmorSetBonus {
        let active = self.get_active_bonuses(equipped_count);
        let mut total = ArmorSetBonus::new(0, 0, 0, 0, 0, 0, None);
        for bonus in &active {
            total.defense += bonus.defense;
            total.magic_defense += bonus.magic_defense;
            total.hp += bonus.hp;
            total.mana += bonus.mana;
            total.speed += bonus.speed;
            // Keep the highest-tier special ability
            if bonus.special_ability.is_some() {
                total.special_ability = bonus.special_ability;
            }
        }
        total
    }

    /// Returns all armor sets.
    pub fn all() -> &'static [ArmorSet] {
        &[
            // Common
            ArmorSet::LeatherTraveler,
            ArmorSet::IronGuard,
            ArmorSet::ApprenticeRobes,
            ArmorSet::ScoutsGarb,
            ArmorSet::MilitiaPlate,
            // Rare
            ArmorSet::Dragonscale,
            ArmorSet::ShadowWeave,
            ArmorSet::HolyCrusader,
            ArmorSet::FrostGiant,
            ArmorSet::PhoenixFeather,
            ArmorSet::Stormcaller,
            ArmorSet::BloodKnight,
            // Epic
            ArmorSet::VoidWalker,
            ArmorSet::CelestialRadiance,
            ArmorSet::DemonLord,
            ArmorSet::AncientGuardian,
            ArmorSet::ArchmagesRegalia,
            ArmorSet::DeathKnight,
            ArmorSet::NaturesEmbrace,
            // Legendary
            ArmorSet::ArmorOfTheGods,
            ArmorSet::ShadowSovereign,
            ArmorSet::DragonEmperor,
            ArmorSet::VoidTyrant,
            ArmorSet::CelestialArbiter,
            ArmorSet::PrimordialTitan,
        ]
    }
}

// ---------------------------------------------------------------------------
// ArmorPiece
// ---------------------------------------------------------------------------

/// A single armor piece that can be equipped by the player.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArmorPiece {
    /// Display name of this specific piece (e.g. "Masterwork Dragonscale Helm").
    pub name: String,
    /// Which equipment slot this piece occupies.
    pub slot: ArmorSlot,
    /// The base material type.
    pub armor_type: ArmorType,
    /// Quality tier affecting stat multipliers.
    pub quality: ArmorQuality,
    /// List of enchantments applied to this piece.
    pub enchantments: Vec<ArmorEnchantment>,
    /// Optional set membership.
    pub set: Option<ArmorSet>,
    /// Physical defense value (after quality and slot modifiers).
    pub defense: i32,
    /// Magic defense value (after quality and slot modifiers).
    pub magic_defense: i32,
    /// Weight of the piece affecting movement speed.
    pub weight: f32,
    /// Current durability.
    pub durability: i32,
    /// Maximum durability.
    pub max_durability: i32,
    /// Minimum character level to equip.
    pub level_requirement: u32,
}

impl ArmorPiece {
    /// Create a new armor piece.  Defense, magic defense, weight, and durability
    /// are computed from the armor type, slot, and quality.
    pub fn new(
        name: String,
        slot: ArmorSlot,
        armor_type: ArmorType,
        quality: ArmorQuality,
        enchantments: Vec<ArmorEnchantment>,
        set: Option<ArmorSet>,
    ) -> Self {
        let slot_mult = slot.defense_multiplier();
        let quality_mult = quality.stat_multiplier();

        let defense = (armor_type.base_defense() as f32 * slot_mult * quality_mult).round() as i32;
        let magic_defense =
            (armor_type.base_magic_defense() as f32 * slot_mult * quality_mult).round() as i32;
        let weight = armor_type.base_weight() * slot_mult;
        let max_durability = (100.0 * quality.durability_multiplier()).round() as i32;
        let level_requirement = armor_type.level_requirement();

        Self {
            name,
            slot,
            armor_type,
            quality,
            enchantments,
            set,
            defense,
            magic_defense,
            weight,
            durability: max_durability,
            max_durability,
            level_requirement,
        }
    }

    /// Total physical defense including enchantment bonuses.
    pub fn total_defense(&self) -> i32 {
        let enchant_bonus: i32 = self.enchantments.iter().map(|e| e.defense_bonus()).sum();
        self.defense + enchant_bonus
    }

    /// Total magic defense including enchantment bonuses.
    pub fn total_magic_defense(&self) -> i32 {
        let enchant_bonus: i32 = self.enchantments.iter().map(|e| e.magic_defense_bonus()).sum();
        self.magic_defense + enchant_bonus
    }

    /// Total HP bonus from enchantments on this piece.
    pub fn total_hp_bonus(&self) -> i32 {
        self.enchantments.iter().map(|e| e.hp_bonus()).sum()
    }

    /// Total mana bonus from enchantments on this piece.
    pub fn total_mana_bonus(&self) -> i32 {
        self.enchantments.iter().map(|e| e.mana_bonus()).sum()
    }

    /// Total speed bonus from enchantments on this piece.
    pub fn total_speed_bonus(&self) -> i32 {
        self.enchantments.iter().map(|e| e.speed_bonus()).sum()
    }

    /// Recalculate defense and magic defense based on current quality.
    /// Useful after upgrading or degrading quality.
    pub fn apply_quality(&mut self) {
        let slot_mult = self.slot.defense_multiplier();
        let quality_mult = self.quality.stat_multiplier();

        self.defense =
            (self.armor_type.base_defense() as f32 * slot_mult * quality_mult).round() as i32;
        self.magic_defense =
            (self.armor_type.base_magic_defense() as f32 * slot_mult * quality_mult).round()
                as i32;
        self.max_durability = (100.0 * self.quality.durability_multiplier()).round() as i32;
        if self.durability > self.max_durability {
            self.durability = self.max_durability;
        }
    }

    /// Take durability damage.  Returns `true` if the piece broke (durability hit 0).
    pub fn take_durability_damage(&mut self, amount: i32) -> bool {
        self.durability = (self.durability - amount).max(0);
        self.durability == 0
    }

    /// Repair the piece by a given amount, capped at max durability.
    pub fn repair(&mut self, amount: i32) {
        self.durability = (self.durability + amount).min(self.max_durability);
    }

    /// Whether this piece is broken (zero durability).
    pub fn is_broken(&self) -> bool {
        self.durability <= 0
    }

    /// Durability as a percentage (0.0 to 1.0).
    pub fn durability_percent(&self) -> f32 {
        if self.max_durability == 0 {
            return 0.0;
        }
        self.durability as f32 / self.max_durability as f32
    }

    /// Check if the piece can accept an additional enchantment based on quality.
    pub fn can_enchant(&self) -> bool {
        self.enchantments.len() < self.quality.bonus_enchantment_slots()
    }

    /// Attempt to add an enchantment.  Fails if the piece is at capacity or if
    /// the enchantment is legendary but the quality is below Legendary.
    pub fn add_enchantment(&mut self, enchantment: ArmorEnchantment) -> Result<(), &'static str> {
        if enchantment.is_legendary() {
            match self.quality {
                ArmorQuality::Legendary | ArmorQuality::Mythic | ArmorQuality::Divine => {}
                _ => return Err("Legendary enchantments require Legendary or higher quality."),
            }
        }
        if !self.can_enchant() {
            return Err("No enchantment slots available for this quality tier.");
        }
        if self.enchantments.contains(&enchantment) {
            return Err("This enchantment is already applied to this piece.");
        }
        self.enchantments.push(enchantment);
        Ok(())
    }

    /// Generate a display string summarizing the piece.
    pub fn display_summary(&self) -> String {
        let set_str = match &self.set {
            Some(s) => format!(" [{}]", s.name()),
            None => String::new(),
        };
        let enchant_str = if self.enchantments.is_empty() {
            String::new()
        } else {
            let names: Vec<&str> = self.enchantments.iter().map(|e| e.name()).collect();
            format!(" ({})", names.join(", "))
        };
        format!(
            "{} - {} {} {}{}{} | DEF:{} MDEF:{} DUR:{}/{} WT:{:.1} LVL:{}",
            self.name,
            self.quality.name(),
            self.armor_type.name(),
            self.slot.name(),
            set_str,
            enchant_str,
            self.total_defense(),
            self.total_magic_defense(),
            self.durability,
            self.max_durability,
            self.weight,
            self.level_requirement,
        )
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_armor_quality_multipliers() {
        assert!((ArmorQuality::Broken.stat_multiplier() - 0.5).abs() < f32::EPSILON);
        assert!((ArmorQuality::Common.stat_multiplier() - 1.0).abs() < f32::EPSILON);
        assert!((ArmorQuality::Divine.stat_multiplier() - 3.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_armor_piece_creation() {
        let piece = ArmorPiece::new(
            "Iron Helm".to_string(),
            ArmorSlot::Head,
            ArmorType::ChainMail,
            ArmorQuality::Common,
            vec![],
            None,
        );
        // ChainMail base defense = 20, head slot mult = 0.80, common quality = 1.0
        // 20 * 0.80 * 1.0 = 16
        assert_eq!(piece.defense, 16);
        assert_eq!(piece.total_defense(), 16);
        assert_eq!(piece.durability, 100);
    }

    #[test]
    fn test_quality_application() {
        let mut piece = ArmorPiece::new(
            "Crystal Cuirass".to_string(),
            ArmorSlot::Chest,
            ArmorType::Crystal,
            ArmorQuality::Common,
            vec![],
            None,
        );
        let common_def = piece.defense;
        piece.quality = ArmorQuality::Masterwork;
        piece.apply_quality();
        assert!(piece.defense > common_def);
        // Crystal base 45, chest slot 1.0, masterwork 1.6 => 72
        assert_eq!(piece.defense, 72);
    }

    #[test]
    fn test_enchantment_defense_bonus() {
        let piece = ArmorPiece::new(
            "Shadow Boots".to_string(),
            ArmorSlot::Boots,
            ArmorType::Shadow,
            ArmorQuality::Superior,
            vec![ArmorEnchantment::Fortification, ArmorEnchantment::DamageReduction],
            None,
        );
        // Fortification defense_bonus = 10, DamageReduction = 5 => +15
        assert_eq!(piece.total_defense(), piece.defense + 15);
    }

    #[test]
    fn test_legendary_enchantment_restriction() {
        let mut piece = ArmorPiece::new(
            "Common Helm".to_string(),
            ArmorSlot::Head,
            ArmorType::Cloth,
            ArmorQuality::Common,
            vec![],
            None,
        );
        let result = piece.add_enchantment(ArmorEnchantment::VoidShield);
        assert!(result.is_err());

        let mut legendary_piece = ArmorPiece::new(
            "Legendary Helm".to_string(),
            ArmorSlot::Head,
            ArmorType::Void,
            ArmorQuality::Legendary,
            vec![],
            None,
        );
        let result = legendary_piece.add_enchantment(ArmorEnchantment::VoidShield);
        assert!(result.is_ok());
    }

    #[test]
    fn test_durability_system() {
        let mut piece = ArmorPiece::new(
            "Test Armor".to_string(),
            ArmorSlot::Chest,
            ArmorType::PlateMail,
            ArmorQuality::Common,
            vec![],
            None,
        );
        assert_eq!(piece.durability, 100);
        assert!(!piece.is_broken());

        let broke = piece.take_durability_damage(80);
        assert!(!broke);
        assert_eq!(piece.durability, 20);

        piece.repair(50);
        assert_eq!(piece.durability, 70);

        let broke = piece.take_durability_damage(100);
        assert!(broke);
        assert!(piece.is_broken());
        assert_eq!(piece.durability, 0);
    }

    #[test]
    fn test_set_bonuses() {
        let bonuses = ArmorSet::ArmorOfTheGods.get_active_bonuses(4);
        assert_eq!(bonuses.len(), 2); // 2-piece and 4-piece both active

        let bonuses = ArmorSet::ArmorOfTheGods.get_active_bonuses(1);
        assert!(bonuses.is_empty());

        let total = ArmorSet::ArmorOfTheGods.total_active_bonus(6);
        // Sum of all three tiers: 35+80+150 = 265 defense
        assert_eq!(total.defense, 265);
        assert!(total.special_ability.is_some());
    }

    #[test]
    fn test_armor_slot_all() {
        assert_eq!(ArmorSlot::all().len(), 8);
    }

    #[test]
    fn test_armor_type_all() {
        assert_eq!(ArmorType::all().len(), 9);
    }

    #[test]
    fn test_armor_quality_all() {
        assert_eq!(ArmorQuality::all().len(), 9);
    }

    #[test]
    fn test_armor_set_all() {
        assert_eq!(ArmorSet::all().len(), 25);
    }

    #[test]
    fn test_set_tier_counts() {
        let common = ArmorSet::all().iter().filter(|s| s.tier() == "Common").count();
        let rare = ArmorSet::all().iter().filter(|s| s.tier() == "Rare").count();
        let epic = ArmorSet::all().iter().filter(|s| s.tier() == "Epic").count();
        let legendary = ArmorSet::all().iter().filter(|s| s.tier() == "Legendary").count();
        assert_eq!(common, 5);
        assert_eq!(rare, 7);
        assert_eq!(epic, 7);
        assert_eq!(legendary, 6);
    }

    #[test]
    fn test_display_summary() {
        let piece = ArmorPiece::new(
            "Void Emperor Plate".to_string(),
            ArmorSlot::Chest,
            ArmorType::Void,
            ArmorQuality::Divine,
            vec![ArmorEnchantment::VoidShield, ArmorEnchantment::DivineAegis],
            Some(ArmorSet::VoidTyrant),
        );
        let summary = piece.display_summary();
        assert!(summary.contains("Void Emperor Plate"));
        assert!(summary.contains("Void Tyrant"));
        assert!(summary.contains("Void Shield"));
        assert!(summary.contains("Divine Aegis"));
    }
}
