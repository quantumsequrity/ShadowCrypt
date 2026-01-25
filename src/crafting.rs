use crossterm::style::Color;
use serde::{Serialize, Deserialize};
use rand::Rng;

use crate::{ItemKind, Rarity};

// ============================================================================
// CRAFTING STATIONS
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum CraftingStation {
    Forge,           // For smelting ores into ingots
    Anvil,           // For smithing weapons and armor
    Workbench,       // For general crafting and repairs
    AlchemyTable,    // For brewing potions
    EnchantingAltar, // For magical enchantments
    CookingFire,     // For cooking food
    Loom,            // For tailoring cloth and leather
}

impl CraftingStation {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Forge => "Forge",
            Self::Anvil => "Anvil",
            Self::Workbench => "Workbench",
            Self::AlchemyTable => "Alchemy Table",
            Self::EnchantingAltar => "Enchanting Altar",
            Self::CookingFire => "Cooking Fire",
            Self::Loom => "Loom",
        }
    }

    pub fn glyph(&self) -> char {
        match self {
            Self::Forge => '\u{2302}',      // House symbol for forge
            Self::Anvil => '\u{2666}',       // Diamond for anvil
            Self::Workbench => '\u{25A0}',   // Square for workbench
            Self::AlchemyTable => '\u{2697}', // Alembic
            Self::EnchantingAltar => '\u{2605}', // Star
            Self::CookingFire => '\u{2600}', // Sun for fire
            Self::Loom => '\u{2261}',        // Triple bar for loom
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::Forge => Color::Red,
            Self::Anvil => Color::Grey,
            Self::Workbench => Color::Yellow,
            Self::AlchemyTable => Color::Green,
            Self::EnchantingAltar => Color::Magenta,
            Self::CookingFire => Color::DarkRed,
            Self::Loom => Color::Cyan,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Forge => "Smelt ores into ingots and heat metal for shaping",
            Self::Anvil => "Hammer metal into weapons and armor",
            Self::Workbench => "Craft tools, repairs, and general items",
            Self::AlchemyTable => "Brew potions from herbs and essences",
            Self::EnchantingAltar => "Imbue items with magical properties",
            Self::CookingFire => "Cook food for sustenance and buffs",
            Self::Loom => "Weave cloth and work leather into armor",
        }
    }

    pub fn all() -> Vec<CraftingStation> {
        vec![
            CraftingStation::Forge,
            CraftingStation::Anvil,
            CraftingStation::Workbench,
            CraftingStation::AlchemyTable,
            CraftingStation::EnchantingAltar,
            CraftingStation::CookingFire,
            CraftingStation::Loom,
        ]
    }
}

// ============================================================================
// MATERIAL QUALITY
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum MaterialQuality {
    Poor,       // -25% stats
    Normal,     // Base stats
    Fine,       // +15% stats
    Superior,   // +30% stats
    Exceptional,// +50% stats
    Masterwork, // +75% stats
    Legendary,  // +100% stats (double)
}

impl MaterialQuality {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Poor => "Poor",
            Self::Normal => "Normal",
            Self::Fine => "Fine",
            Self::Superior => "Superior",
            Self::Exceptional => "Exceptional",
            Self::Masterwork => "Masterwork",
            Self::Legendary => "Legendary",
        }
    }

    pub fn stat_multiplier(&self) -> f32 {
        match self {
            Self::Poor => 0.75,
            Self::Normal => 1.0,
            Self::Fine => 1.15,
            Self::Superior => 1.30,
            Self::Exceptional => 1.50,
            Self::Masterwork => 1.75,
            Self::Legendary => 2.0,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::Poor => Color::DarkGrey,
            Self::Normal => Color::Grey,
            Self::Fine => Color::White,
            Self::Superior => Color::Green,
            Self::Exceptional => Color::Blue,
            Self::Masterwork => Color::Magenta,
            Self::Legendary => Color::Yellow,
        }
    }

    pub fn drop_chance(&self) -> f32 {
        match self {
            Self::Poor => 0.15,
            Self::Normal => 0.40,
            Self::Fine => 0.25,
            Self::Superior => 0.12,
            Self::Exceptional => 0.05,
            Self::Masterwork => 0.025,
            Self::Legendary => 0.005,
        }
    }

    pub fn random() -> Self {
        let roll: f32 = rand::thread_rng().random();
        let mut cumulative = 0.0;
        for quality in [Self::Poor, Self::Normal, Self::Fine, Self::Superior,
                        Self::Exceptional, Self::Masterwork, Self::Legendary] {
            cumulative += quality.drop_chance();
            if roll < cumulative {
                return quality;
            }
        }
        Self::Normal
    }

    pub fn salvage_bonus(&self) -> f32 {
        match self {
            Self::Poor => 0.5,
            Self::Normal => 1.0,
            Self::Fine => 1.2,
            Self::Superior => 1.5,
            Self::Exceptional => 2.0,
            Self::Masterwork => 2.5,
            Self::Legendary => 3.0,
        }
    }
}

// ============================================================================
// CRAFTING SKILLS
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum CraftingSkill {
    Blacksmithing,  // Weapons and metal armor at Forge/Anvil
    Alchemy,        // Potions at Alchemy Table
    Enchanting,     // Magic items at Enchanting Altar
    Cooking,        // Food at Cooking Fire
    Tailoring,      // Cloth/Leather at Loom
    Jewelcrafting,  // Rings/Amulets at Workbench
    Runecraft,      // Runes and scrolls
}

impl CraftingSkill {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Blacksmithing => "Blacksmithing",
            Self::Alchemy => "Alchemy",
            Self::Enchanting => "Enchanting",
            Self::Cooking => "Cooking",
            Self::Tailoring => "Tailoring",
            Self::Jewelcrafting => "Jewelcrafting",
            Self::Runecraft => "Runecraft",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::Blacksmithing => Color::Grey,
            Self::Alchemy => Color::Green,
            Self::Enchanting => Color::Magenta,
            Self::Cooking => Color::Yellow,
            Self::Tailoring => Color::Cyan,
            Self::Jewelcrafting => Color::Blue,
            Self::Runecraft => Color::DarkMagenta,
        }
    }

    pub fn primary_station(&self) -> CraftingStation {
        match self {
            Self::Blacksmithing => CraftingStation::Anvil,
            Self::Alchemy => CraftingStation::AlchemyTable,
            Self::Enchanting => CraftingStation::EnchantingAltar,
            Self::Cooking => CraftingStation::CookingFire,
            Self::Tailoring => CraftingStation::Loom,
            Self::Jewelcrafting => CraftingStation::Workbench,
            Self::Runecraft => CraftingStation::EnchantingAltar,
        }
    }

    pub fn all() -> Vec<CraftingSkill> {
        vec![
            CraftingSkill::Blacksmithing,
            CraftingSkill::Alchemy,
            CraftingSkill::Enchanting,
            CraftingSkill::Cooking,
            CraftingSkill::Tailoring,
            CraftingSkill::Jewelcrafting,
            CraftingSkill::Runecraft,
        ]
    }

    pub fn max_level() -> u32 { 100 }
}

// ============================================================================
// CRAFTING PROGRESS
// ============================================================================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CraftingProgress {
    pub skill: CraftingSkill,
    pub level: u32,
    pub experience: u32,
    pub exp_to_next: u32,
    pub recipes_discovered: Vec<String>,
    pub total_crafted: u32,
    pub critical_successes: u32,
    pub failures: u32,
}

impl CraftingProgress {
    pub fn new(skill: CraftingSkill) -> Self {
        Self {
            skill,
            level: 1,
            experience: 0,
            exp_to_next: 100,
            recipes_discovered: Vec::new(),
            total_crafted: 0,
            critical_successes: 0,
            failures: 0,
        }
    }

    pub fn gain_xp(&mut self, amount: u32) -> bool {
        self.experience += amount;
        let mut leveled = false;
        while self.experience >= self.exp_to_next && self.level < CraftingSkill::max_level() {
            self.experience -= self.exp_to_next;
            self.level += 1;
            self.exp_to_next = self.calculate_exp_for_level(self.level + 1);
            leveled = true;
        }
        leveled
    }

    fn calculate_exp_for_level(&self, level: u32) -> u32 {
        (100.0 * (1.15_f32).powi(level as i32 - 1)) as u32
    }

    pub fn discover_recipe(&mut self, recipe_name: &str) -> bool {
        if !self.recipes_discovered.contains(&recipe_name.to_string()) {
            self.recipes_discovered.push(recipe_name.to_string());
            true
        } else {
            false
        }
    }

    pub fn critical_chance(&self) -> f32 {
        // Base 5% + 0.5% per level, max 55%
        (0.05 + 0.005 * self.level as f32).min(0.55)
    }

    pub fn failure_chance(&self, recipe_level: u32) -> f32 {
        if self.level >= recipe_level + 10 {
            0.0 // Trivial recipe, no failure
        } else if self.level >= recipe_level {
            0.05 // 5% base failure
        } else {
            // Higher failure for harder recipes
            let diff = recipe_level - self.level;
            (0.05 + 0.1 * diff as f32).min(0.75)
        }
    }
}

// ============================================================================
// CRAFTING RESULT
// ============================================================================

#[derive(Clone, Debug)]
pub enum CraftingResult {
    Success {
        item: ItemKind,
        quality: MaterialQuality,
        bonus_stats: i32,
    },
    CriticalSuccess {
        item: ItemKind,
        quality: MaterialQuality,
        bonus_stats: i32,
        extra_item: Option<ItemKind>,
    },
    Failure {
        materials_lost: bool,
        partial_refund: Vec<(ItemKind, u32)>,
    },
    CriticalFailure {
        materials_lost: bool,
        injury: bool,
        damage: i32,
    },
}

// ============================================================================
// ENCHANTMENT TYPES
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Enchantment {
    // Weapon Enchantments
    Sharpness,      // +damage
    Fire,           // Fire damage
    Frost,          // Frost damage + slow
    Lightning,      // Chain lightning
    Vampiric,       // Lifesteal
    Vorpal,         // Crit damage
    Poison,         // Poison DOT
    Holy,           // Bonus vs undead/demons
    Chaos,          // Random element
    Executioner,    // Bonus vs low HP

    // Armor Enchantments
    Protection,     // +defense
    Thorns,         // Reflect damage
    Regeneration,   // HP regen
    ManaShield,     // Absorb with mana
    Fortification,  // Reduce crit damage taken
    Resistance,     // Elemental resist
    Evasion,        // Dodge chance
    Vitality,       // +max HP
    Wisdom,         // +max mana
    Swiftness,      // +speed

    // Ring/Amulet Enchantments
    Luck,           // Better drops
    Experience,     // +XP gain
    Greed,          // +gold find
    Stealth,        // Detection reduction
    Insight,        // Identify items
    Bravery,        // Fear immunity

    // Special Enchantments
    Soulbound,      // Cannot be dropped
    Indestructible, // Never breaks
    Cursed,         // Cannot be removed
    Blessed,        // Holy aura

    // Legendary Enchantments (rare recipes only)
    Godslayer,      // Massive boss damage
    Timewarp,       // Chance for extra turn
    Resurrection,   // One-time revival
    Omniscience,    // See all enemies
    Oblivion,       // Chance to instant kill
}

impl Enchantment {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Sharpness => "Sharpness",
            Self::Fire => "Fire",
            Self::Frost => "Frost",
            Self::Lightning => "Lightning",
            Self::Vampiric => "Vampiric",
            Self::Vorpal => "Vorpal",
            Self::Poison => "Poison",
            Self::Holy => "Holy",
            Self::Chaos => "Chaos",
            Self::Executioner => "Executioner",
            Self::Protection => "Protection",
            Self::Thorns => "Thorns",
            Self::Regeneration => "Regeneration",
            Self::ManaShield => "Mana Shield",
            Self::Fortification => "Fortification",
            Self::Resistance => "Resistance",
            Self::Evasion => "Evasion",
            Self::Vitality => "Vitality",
            Self::Wisdom => "Wisdom",
            Self::Swiftness => "Swiftness",
            Self::Luck => "Luck",
            Self::Experience => "Experience",
            Self::Greed => "Greed",
            Self::Stealth => "Stealth",
            Self::Insight => "Insight",
            Self::Bravery => "Bravery",
            Self::Soulbound => "Soulbound",
            Self::Indestructible => "Indestructible",
            Self::Cursed => "Cursed",
            Self::Blessed => "Blessed",
            Self::Godslayer => "Godslayer",
            Self::Timewarp => "Timewarp",
            Self::Resurrection => "Resurrection",
            Self::Omniscience => "Omniscience",
            Self::Oblivion => "Oblivion",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Sharpness => "Increases weapon damage",
            Self::Fire => "Deals fire damage and can ignite",
            Self::Frost => "Deals frost damage and slows",
            Self::Lightning => "Chains lightning to nearby enemies",
            Self::Vampiric => "Steals life on hit",
            Self::Vorpal => "Increases critical hit damage",
            Self::Poison => "Applies poison damage over time",
            Self::Holy => "Extra damage to undead and demons",
            Self::Chaos => "Random elemental damage each hit",
            Self::Executioner => "Bonus damage to wounded enemies",
            Self::Protection => "Reduces incoming damage",
            Self::Thorns => "Reflects damage to attackers",
            Self::Regeneration => "Slowly regenerates health",
            Self::ManaShield => "Absorbs damage using mana",
            Self::Fortification => "Reduces critical damage taken",
            Self::Resistance => "Resists elemental damage",
            Self::Evasion => "Chance to dodge attacks",
            Self::Vitality => "Increases maximum health",
            Self::Wisdom => "Increases maximum mana",
            Self::Swiftness => "Increases movement speed",
            Self::Luck => "Improves item drop quality",
            Self::Experience => "Increases experience gained",
            Self::Greed => "Increases gold found",
            Self::Stealth => "Reduces enemy detection range",
            Self::Insight => "Automatically identifies items",
            Self::Bravery => "Immunity to fear effects",
            Self::Soulbound => "Cannot be dropped or traded",
            Self::Indestructible => "Never loses durability",
            Self::Cursed => "Cannot be unequipped normally",
            Self::Blessed => "Grants holy aura",
            Self::Godslayer => "Massive damage bonus vs bosses",
            Self::Timewarp => "Small chance for extra action",
            Self::Resurrection => "Revive once when killed",
            Self::Omniscience => "See all enemies on the map",
            Self::Oblivion => "Chance to instantly kill",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::Fire => Color::Red,
            Self::Frost => Color::Cyan,
            Self::Lightning => Color::Yellow,
            Self::Poison => Color::Green,
            Self::Holy | Self::Blessed => Color::White,
            Self::Vampiric | Self::Cursed => Color::DarkRed,
            Self::Chaos | Self::Oblivion => Color::DarkMagenta,
            Self::Godslayer | Self::Timewarp | Self::Resurrection | Self::Omniscience => Color::Magenta,
            _ => Color::Blue,
        }
    }

    pub fn tier(&self) -> u32 {
        match self {
            Self::Sharpness | Self::Protection | Self::Vitality => 1,
            Self::Fire | Self::Frost | Self::Thorns | Self::Regeneration => 2,
            Self::Lightning | Self::Poison | Self::Evasion | Self::Wisdom => 3,
            Self::Vampiric | Self::Vorpal | Self::ManaShield | Self::Swiftness => 4,
            Self::Holy | Self::Chaos | Self::Fortification | Self::Resistance => 5,
            Self::Executioner | Self::Luck | Self::Experience | Self::Greed => 6,
            Self::Stealth | Self::Insight | Self::Bravery => 7,
            Self::Soulbound | Self::Indestructible | Self::Blessed => 8,
            Self::Cursed => 9,
            Self::Godslayer | Self::Timewarp | Self::Resurrection | Self::Omniscience | Self::Oblivion => 10,
        }
    }
}

// ============================================================================
// HERB TYPES FOR ALCHEMY
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum HerbType {
    // Common Herbs
    Bloodroot,      // Health restoration
    Moonpetal,      // Mana restoration
    Sunleaf,        // Energy/stamina
    Nightshade,     // Poison base
    Feverfew,       // Cure disease

    // Uncommon Herbs
    Dragontongue,   // Fire properties
    Frostbloom,     // Ice properties
    Stormweed,      // Lightning properties
    Shadowmoss,     // Stealth/invisibility
    Glowcap,        // Light/vision

    // Rare Herbs
    Voidroot,       // Void/teleport
    Phoenixwort,    // Resurrection
    Unicornhair,    // Purification
    Mandrake,       // Transformation
    Ghostorchid,    // Spirit properties

    // Legendary Herbs
    Worldtree_Leaf, // Ultimate healing
    Dragonheart,    // Ultimate power
    Starfall_Dust,  // Cosmic power
}

impl HerbType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Bloodroot => "Bloodroot",
            Self::Moonpetal => "Moonpetal",
            Self::Sunleaf => "Sunleaf",
            Self::Nightshade => "Nightshade",
            Self::Feverfew => "Feverfew",
            Self::Dragontongue => "Dragontongue",
            Self::Frostbloom => "Frostbloom",
            Self::Stormweed => "Stormweed",
            Self::Shadowmoss => "Shadowmoss",
            Self::Glowcap => "Glowcap",
            Self::Voidroot => "Voidroot",
            Self::Phoenixwort => "Phoenixwort",
            Self::Unicornhair => "Unicornhair",
            Self::Mandrake => "Mandrake",
            Self::Ghostorchid => "Ghost Orchid",
            Self::Worldtree_Leaf => "Worldtree Leaf",
            Self::Dragonheart => "Dragonheart Flower",
            Self::Starfall_Dust => "Starfall Dust",
        }
    }

    pub fn rarity(&self) -> Rarity {
        match self {
            Self::Bloodroot | Self::Moonpetal | Self::Sunleaf |
            Self::Nightshade | Self::Feverfew => Rarity::Common,
            Self::Dragontongue | Self::Frostbloom | Self::Stormweed |
            Self::Shadowmoss | Self::Glowcap => Rarity::Uncommon,
            Self::Voidroot | Self::Phoenixwort | Self::Unicornhair |
            Self::Mandrake | Self::Ghostorchid => Rarity::Rare,
            Self::Worldtree_Leaf | Self::Dragonheart | Self::Starfall_Dust => Rarity::Legendary,
        }
    }
}

// ============================================================================
// FOOD BUFF TYPES
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum FoodBuff {
    WellFed,           // +10% max HP
    Energized,         // +10% max mana
    Strengthened,      // +15% attack
    Fortified,         // +15% defense
    Hastened,          // +20% speed
    Lucky,             // +10% crit chance
    Focused,           // +20% XP gain
    Nourished,         // HP regen
    Enlightened,       // Mana regen
    Heroic,            // All stats +5%
    Legendary_Feast,   // All stats +15%
    Survivors_Will,    // Survive lethal blow once
}

impl FoodBuff {
    pub fn name(&self) -> &'static str {
        match self {
            Self::WellFed => "Well Fed",
            Self::Energized => "Energized",
            Self::Strengthened => "Strengthened",
            Self::Fortified => "Fortified",
            Self::Hastened => "Hastened",
            Self::Lucky => "Lucky",
            Self::Focused => "Focused",
            Self::Nourished => "Nourished",
            Self::Enlightened => "Enlightened",
            Self::Heroic => "Heroic",
            Self::Legendary_Feast => "Legendary Feast",
            Self::Survivors_Will => "Survivor's Will",
        }
    }

    pub fn duration_turns(&self) -> u32 {
        match self {
            Self::WellFed | Self::Energized => 100,
            Self::Strengthened | Self::Fortified | Self::Hastened => 75,
            Self::Lucky | Self::Focused => 50,
            Self::Nourished | Self::Enlightened => 80,
            Self::Heroic => 60,
            Self::Legendary_Feast => 120,
            Self::Survivors_Will => 200,
        }
    }
}

// ============================================================================
// RECIPE STRUCTURE
// ============================================================================

#[derive(Clone, Debug)]
pub struct Recipe {
    pub name: String,
    pub description: String,
    pub result: ItemKind,
    pub result_count: u32,
    pub result_rarity: Rarity,
    pub ingredients: Vec<(ItemKind, u32)>,
    pub station_required: CraftingStation,
    pub skill_required: Option<(CraftingSkill, u32)>,
    pub xp_reward: u32,
    pub is_rare_recipe: bool,       // Found as loot
    pub enchantment: Option<Enchantment>,
    pub food_buff: Option<FoodBuff>,
    pub crafting_time: u32,         // In game turns
}

impl Recipe {
    pub fn can_craft(&self, skill_level: u32, has_station: bool, inventory: &[(ItemKind, u32)]) -> bool {
        // Check station
        if !has_station {
            return false;
        }

        // Check skill level
        if let Some((_, required_level)) = self.skill_required {
            if skill_level < required_level {
                return false;
            }
        }

        // Check ingredients
        for (item, count) in &self.ingredients {
            let available = inventory.iter()
                .filter(|(i, _)| i == item)
                .map(|(_, c)| c)
                .sum::<u32>();
            if available < *count {
                return false;
            }
        }

        true
    }
}

// ============================================================================
// SALVAGE RESULT
// ============================================================================

#[derive(Clone, Debug)]
pub struct SalvageResult {
    pub materials: Vec<(ItemKind, u32)>,
    pub rare_component: Option<ItemKind>,
    pub xp_gained: u32,
}

// ============================================================================
// CRAFTING SYSTEM IMPLEMENTATION
// ============================================================================

pub struct CraftingSystem;

impl CraftingSystem {
    /// Attempt to craft a recipe
    pub fn craft(
        recipe: &Recipe,
        progress: &mut CraftingProgress,
        material_quality: MaterialQuality,
    ) -> CraftingResult {
        let mut rng = rand::thread_rng();

        let skill_level = progress.level;
        let recipe_level = recipe.skill_required.map(|(_, l)| l).unwrap_or(1);

        // Calculate failure chance
        let failure_chance = progress.failure_chance(recipe_level);
        let crit_chance = progress.critical_chance();

        // Quality bonus reduces failure, increases crit
        let quality_mod = material_quality.stat_multiplier();
        let adjusted_failure = (failure_chance / quality_mod).max(0.0);
        let adjusted_crit = (crit_chance * quality_mod).min(0.75);

        let roll: f32 = rng.random();

        // Critical failure (only on hard recipes)
        if roll < adjusted_failure * 0.2 && recipe_level > skill_level {
            progress.failures += 1;
            return CraftingResult::CriticalFailure {
                materials_lost: true,
                injury: rng.gen_bool(0.3),
                damage: rng.gen_range(5..15),
            };
        }

        // Normal failure
        if roll < adjusted_failure {
            progress.failures += 1;
            let refund_chance = 0.3 + (skill_level as f32 * 0.01);
            let partial_refund: Vec<(ItemKind, u32)> = recipe.ingredients.iter()
                .filter(|_| rng.random::<f32>() < refund_chance)
                .map(|(item, count)| (*item, (*count + 1) / 2))
                .collect();

            return CraftingResult::Failure {
                materials_lost: partial_refund.is_empty(),
                partial_refund,
            };
        }

        // Grant XP
        progress.gain_xp(recipe.xp_reward);
        progress.total_crafted += 1;

        // Calculate output quality
        let output_quality = Self::calculate_output_quality(material_quality, skill_level, recipe_level);
        let bonus_stats = ((output_quality.stat_multiplier() - 1.0) * 10.0) as i32;

        // Critical success
        if roll > 1.0 - adjusted_crit {
            progress.critical_successes += 1;
            let extra = if rng.gen_bool(0.25) {
                Some(recipe.result)
            } else {
                None
            };

            // Upgrade quality on crit
            let crit_quality = match output_quality {
                MaterialQuality::Poor => MaterialQuality::Normal,
                MaterialQuality::Normal => MaterialQuality::Fine,
                MaterialQuality::Fine => MaterialQuality::Superior,
                MaterialQuality::Superior => MaterialQuality::Exceptional,
                MaterialQuality::Exceptional => MaterialQuality::Masterwork,
                MaterialQuality::Masterwork | MaterialQuality::Legendary => MaterialQuality::Legendary,
            };

            return CraftingResult::CriticalSuccess {
                item: recipe.result,
                quality: crit_quality,
                bonus_stats: bonus_stats + 5,
                extra_item: extra,
            };
        }

        // Normal success
        CraftingResult::Success {
            item: recipe.result,
            quality: output_quality,
            bonus_stats,
        }
    }

    fn calculate_output_quality(
        material_quality: MaterialQuality,
        skill_level: u32,
        recipe_level: u32,
    ) -> MaterialQuality {
        let mut rng = rand::thread_rng();

        // Base quality from materials
        let base = match material_quality {
            MaterialQuality::Poor => 0,
            MaterialQuality::Normal => 1,
            MaterialQuality::Fine => 2,
            MaterialQuality::Superior => 3,
            MaterialQuality::Exceptional => 4,
            MaterialQuality::Masterwork => 5,
            MaterialQuality::Legendary => 6,
        };

        // Skill bonus (higher skill = chance to upgrade)
        let skill_bonus = if skill_level > recipe_level + 20 { 2 }
            else if skill_level > recipe_level + 10 { 1 }
            else { 0 };

        // Random variance
        let variance: i32 = rng.gen_range(-1..=1);

        let final_quality = (base as i32 + skill_bonus + variance).clamp(0, 6);

        match final_quality {
            0 => MaterialQuality::Poor,
            1 => MaterialQuality::Normal,
            2 => MaterialQuality::Fine,
            3 => MaterialQuality::Superior,
            4 => MaterialQuality::Exceptional,
            5 => MaterialQuality::Masterwork,
            _ => MaterialQuality::Legendary,
        }
    }

    /// Salvage an item for materials
    pub fn salvage(
        item: ItemKind,
        item_rarity: Rarity,
        item_quality: MaterialQuality,
        skill: &CraftingProgress,
    ) -> SalvageResult {
        let mut rng = rand::thread_rng();
        let mut materials = Vec::new();

        let quality_bonus = item_quality.salvage_bonus();
        let skill_bonus = 1.0 + (skill.level as f32 * 0.02);

        // Determine salvage based on item type
        let base_materials = Self::get_salvage_materials(item);

        for (mat, base_count) in base_materials {
            let count = ((base_count as f32 * quality_bonus * skill_bonus) as u32).max(1);
            // Random chance to get more or less
            let final_count = if rng.gen_bool(0.3) {
                count + rng.gen_range(0..=2)
            } else {
                count.saturating_sub(rng.gen_range(0..=1))
            }.max(1);
            materials.push((mat, final_count));
        }

        // Rare component chance based on rarity
        let rare_chance = match item_rarity {
            Rarity::Common => 0.01,
            Rarity::Uncommon => 0.05,
            Rarity::Rare => 0.15,
            Rarity::Epic => 0.30,
            Rarity::Legendary => 0.50,
            Rarity::Mythic => 0.75,
        };

        let rare_component = if rng.random::<f32>() < rare_chance * skill_bonus {
            Some(Self::get_rare_salvage_component(item))
        } else {
            None
        };

        let xp = match item_rarity {
            Rarity::Common => 5,
            Rarity::Uncommon => 10,
            Rarity::Rare => 20,
            Rarity::Epic => 40,
            Rarity::Legendary => 80,
            Rarity::Mythic => 150,
        };

        SalvageResult {
            materials,
            rare_component,
            xp_gained: xp,
        }
    }

    fn get_salvage_materials(item: ItemKind) -> Vec<(ItemKind, u32)> {
        match item {
            // Weapons
            ItemKind::Dagger | ItemKind::ShortSword => vec![(ItemKind::IronOre, 1)],
            ItemKind::LongSword | ItemKind::Axe => vec![(ItemKind::IronOre, 2)],
            ItemKind::Greatsword | ItemKind::BattleAxe => vec![(ItemKind::SteelIngot, 2)],
            ItemKind::Katana => vec![(ItemKind::SteelIngot, 2), (ItemKind::LeatherStrip, 1)],
            ItemKind::FlameSword => vec![(ItemKind::SteelIngot, 2), (ItemKind::FireEssence, 1)],
            ItemKind::FrostBlade => vec![(ItemKind::SteelIngot, 2), (ItemKind::FrostEssence, 1)],

            // Armor
            ItemKind::LeatherArmor => vec![(ItemKind::LeatherStrip, 3)],
            ItemKind::ChainMail => vec![(ItemKind::IronOre, 3)],
            ItemKind::PlateMail => vec![(ItemKind::SteelIngot, 4)],
            ItemKind::DragonArmor => vec![(ItemKind::DragonScale, 2), (ItemKind::SteelIngot, 2)],

            // Shields
            ItemKind::WoodenShield => vec![(ItemKind::LeatherStrip, 1)],
            ItemKind::IronShield => vec![(ItemKind::IronOre, 2)],
            ItemKind::TowerShield => vec![(ItemKind::SteelIngot, 3)],

            // Potions return vials
            ItemKind::HealthPotion | ItemKind::ManaPotion => vec![(ItemKind::EmptyVial, 1)],

            // Default
            _ => vec![(ItemKind::IronOre, 1)],
        }
    }

    fn get_rare_salvage_component(item: ItemKind) -> ItemKind {
        match item {
            ItemKind::FlameSword | ItemKind::FlameGauntlets => ItemKind::FireEssence,
            ItemKind::FrostBlade | ItemKind::FrostGauntlets => ItemKind::FrostEssence,
            ItemKind::DragonArmor | ItemKind::DragonHelm | ItemKind::DragonShield => ItemKind::DragonScale,
            ItemKind::VoidStaff => ItemKind::VoidEssence,
            _ => ItemKind::EnchantedGem,
        }
    }
}
