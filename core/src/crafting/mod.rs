//! Crafting system: recipes, workbenches, material gathering, and item creation
//!
//! This module provides a comprehensive crafting system for ShadowCrypt including:
//! - Crafting materials that can be gathered from enemies, mining, or harvesting
//! - Workbenches for different types of crafting (forge, alchemy, enchanting)
//! - Recipes that combine materials into equipment, potions, and other items
//! - Quality modifiers based on player skill and material rarity

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::items::{Item, ItemKind, Rarity};

/// Crafting materials that can be gathered and used in recipes
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum CraftingMaterial {
    // Metals (gathered from mining or enemies)
    IronOre,
    SteelIngot,
    MithrilOre,
    AdamantiteOre,
    DarksteelIngot,
    DragonsteelIngot,

    // Wood (gathered from environment)
    CommonWood,
    HardWood,
    ElvenWood,
    AncientWood,
    PetrifiedWood,

    // Leather and Cloth (from enemies)
    RawLeather,
    TannedLeather,
    DragonHide,
    DemonSkin,
    SilkCloth,
    EnchantedFabric,

    // Herbs and Plants (gathered from environment)
    HealingHerb,
    ManaBloom,
    Nightshade,
    FirePetal,
    IceMoss,
    VoidRoot,
    LifeFlower,
    DeathCap,

    // Monster Parts (dropped by enemies)
    BoneFragment,
    FangOrClaw,
    MonsterEye,
    VenomSac,
    ElementalEssence,
    GhostEctoplasm,
    DemonBlood,
    DragonHeart,
    LichDust,

    // Gems and Crystals (from mining or enemies)
    RubyFragment,
    SapphireFragment,
    EmeraldFragment,
    DiamondShard,
    SoulCrystal,
    VoidCrystal,

    // Magical Components
    ArcaneEssence,
    HolyWater,
    DarkEnergy,
    ElementalCore,
    RuneStone,
    EnchantedInk,

    // Rare Components (boss drops and special locations)
    PhoenixFeather,
    DragonScale,
    DemonHeart,
    AncientRelic,
    TitanBone,
    GodsbloodVial,
}

impl CraftingMaterial {
    /// Returns the display name of the material
    pub fn name(&self) -> &'static str {
        match self {
            Self::IronOre => "Iron Ore",
            Self::SteelIngot => "Steel Ingot",
            Self::MithrilOre => "Mithril Ore",
            Self::AdamantiteOre => "Adamantite Ore",
            Self::DarksteelIngot => "Darksteel Ingot",
            Self::DragonsteelIngot => "Dragonsteel Ingot",
            Self::CommonWood => "Common Wood",
            Self::HardWood => "Hard Wood",
            Self::ElvenWood => "Elven Wood",
            Self::AncientWood => "Ancient Wood",
            Self::PetrifiedWood => "Petrified Wood",
            Self::RawLeather => "Raw Leather",
            Self::TannedLeather => "Tanned Leather",
            Self::DragonHide => "Dragon Hide",
            Self::DemonSkin => "Demon Skin",
            Self::SilkCloth => "Silk Cloth",
            Self::EnchantedFabric => "Enchanted Fabric",
            Self::HealingHerb => "Healing Herb",
            Self::ManaBloom => "Mana Bloom",
            Self::Nightshade => "Nightshade",
            Self::FirePetal => "Fire Petal",
            Self::IceMoss => "Ice Moss",
            Self::VoidRoot => "Void Root",
            Self::LifeFlower => "Life Flower",
            Self::DeathCap => "Death Cap",
            Self::BoneFragment => "Bone Fragment",
            Self::FangOrClaw => "Fang or Claw",
            Self::MonsterEye => "Monster Eye",
            Self::VenomSac => "Venom Sac",
            Self::ElementalEssence => "Elemental Essence",
            Self::GhostEctoplasm => "Ghost Ectoplasm",
            Self::DemonBlood => "Demon Blood",
            Self::DragonHeart => "Dragon Heart",
            Self::LichDust => "Lich Dust",
            Self::RubyFragment => "Ruby Fragment",
            Self::SapphireFragment => "Sapphire Fragment",
            Self::EmeraldFragment => "Emerald Fragment",
            Self::DiamondShard => "Diamond Shard",
            Self::SoulCrystal => "Soul Crystal",
            Self::VoidCrystal => "Void Crystal",
            Self::ArcaneEssence => "Arcane Essence",
            Self::HolyWater => "Holy Water",
            Self::DarkEnergy => "Dark Energy",
            Self::ElementalCore => "Elemental Core",
            Self::RuneStone => "Rune Stone",
            Self::EnchantedInk => "Enchanted Ink",
            Self::PhoenixFeather => "Phoenix Feather",
            Self::DragonScale => "Dragon Scale",
            Self::DemonHeart => "Demon Heart",
            Self::AncientRelic => "Ancient Relic",
            Self::TitanBone => "Titan Bone",
            Self::GodsbloodVial => "Godsblood Vial",
        }
    }

    /// Returns the rarity tier of this material
    pub fn rarity(&self) -> MaterialRarity {
        match self {
            Self::IronOre | Self::CommonWood | Self::RawLeather
            | Self::HealingHerb | Self::BoneFragment => MaterialRarity::Common,

            Self::SteelIngot | Self::HardWood | Self::TannedLeather
            | Self::ManaBloom | Self::FangOrClaw | Self::SilkCloth => MaterialRarity::Uncommon,

            Self::MithrilOre | Self::ElvenWood | Self::Nightshade
            | Self::FirePetal | Self::IceMoss | Self::MonsterEye
            | Self::VenomSac | Self::RubyFragment | Self::SapphireFragment
            | Self::EmeraldFragment | Self::ArcaneEssence | Self::HolyWater
            | Self::RuneStone => MaterialRarity::Rare,

            Self::AdamantiteOre | Self::AncientWood | Self::DragonHide
            | Self::VoidRoot | Self::LifeFlower | Self::ElementalEssence
            | Self::GhostEctoplasm | Self::DiamondShard | Self::SoulCrystal
            | Self::DarkEnergy | Self::ElementalCore | Self::EnchantedInk
            | Self::EnchantedFabric => MaterialRarity::Epic,

            Self::DarksteelIngot | Self::PetrifiedWood | Self::DemonSkin
            | Self::DeathCap | Self::DemonBlood | Self::LichDust
            | Self::VoidCrystal | Self::PhoenixFeather | Self::DragonScale
            | Self::AncientRelic | Self::TitanBone => MaterialRarity::Legendary,

            Self::DragonsteelIngot | Self::DragonHeart | Self::DemonHeart
            | Self::GodsbloodVial => MaterialRarity::Mythic,
        }
    }

    /// Returns the glyph for displaying this material
    pub fn glyph(&self) -> char {
        match self {
            Self::IronOre | Self::SteelIngot | Self::MithrilOre
            | Self::AdamantiteOre | Self::DarksteelIngot
            | Self::DragonsteelIngot => '*',

            Self::CommonWood | Self::HardWood | Self::ElvenWood
            | Self::AncientWood | Self::PetrifiedWood => '=',

            Self::RawLeather | Self::TannedLeather | Self::DragonHide
            | Self::DemonSkin | Self::SilkCloth | Self::EnchantedFabric => '~',

            Self::HealingHerb | Self::ManaBloom | Self::Nightshade
            | Self::FirePetal | Self::IceMoss | Self::VoidRoot
            | Self::LifeFlower | Self::DeathCap => ',',

            Self::BoneFragment | Self::FangOrClaw | Self::MonsterEye
            | Self::VenomSac | Self::ElementalEssence | Self::GhostEctoplasm
            | Self::DemonBlood | Self::DragonHeart | Self::LichDust => '%',

            Self::RubyFragment | Self::SapphireFragment | Self::EmeraldFragment
            | Self::DiamondShard | Self::SoulCrystal | Self::VoidCrystal => 'o',

            Self::ArcaneEssence | Self::HolyWater | Self::DarkEnergy
            | Self::ElementalCore | Self::RuneStone | Self::EnchantedInk => '+',

            Self::PhoenixFeather | Self::DragonScale | Self::DemonHeart
            | Self::AncientRelic | Self::TitanBone | Self::GodsbloodVial => '&',
        }
    }

    /// Returns the base value in gold
    pub fn base_value(&self) -> u32 {
        match self.rarity() {
            MaterialRarity::Common => 5,
            MaterialRarity::Uncommon => 15,
            MaterialRarity::Rare => 50,
            MaterialRarity::Epic => 150,
            MaterialRarity::Legendary => 500,
            MaterialRarity::Mythic => 2000,
        }
    }
}

/// Rarity tier for crafting materials
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, Debug)]
pub enum MaterialRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

impl MaterialRarity {
    /// Convert to item rarity for crafted items
    pub fn to_item_rarity(&self) -> Rarity {
        match self {
            Self::Common => Rarity::Common,
            Self::Uncommon => Rarity::Uncommon,
            Self::Rare => Rarity::Rare,
            Self::Epic => Rarity::Epic,
            Self::Legendary => Rarity::Legendary,
            Self::Mythic => Rarity::Mythic,
        }
    }
}

/// A stack of crafting materials with quantity
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MaterialStack {
    pub material: CraftingMaterial,
    pub quantity: u32,
}

impl MaterialStack {
    /// Create a new material stack
    pub fn new(material: CraftingMaterial, quantity: u32) -> Self {
        Self { material, quantity }
    }
}

/// Types of workbenches for crafting
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum WorkbenchType {
    /// Basic workbench for simple items and wood crafting
    BasicWorkbench,
    /// Forge for metal weapons and armor
    Forge,
    /// Anvil for advanced metalworking
    Anvil,
    /// Alchemy table for potions and elixirs
    AlchemyTable,
    /// Enchanting altar for magical enhancements
    EnchantingAltar,
    /// Tanning rack for leather working
    TanningRack,
    /// Jeweler's bench for rings and amulets
    JewelerBench,
    /// Ancient forge for legendary items
    AncientForge,
    /// Demonic altar for dark crafting
    DemonicAltar,
    /// Divine shrine for holy items
    DivineShrine,
}

impl WorkbenchType {
    /// Returns the display name of this workbench
    pub fn name(&self) -> &'static str {
        match self {
            Self::BasicWorkbench => "Basic Workbench",
            Self::Forge => "Forge",
            Self::Anvil => "Anvil",
            Self::AlchemyTable => "Alchemy Table",
            Self::EnchantingAltar => "Enchanting Altar",
            Self::TanningRack => "Tanning Rack",
            Self::JewelerBench => "Jeweler's Bench",
            Self::AncientForge => "Ancient Forge",
            Self::DemonicAltar => "Demonic Altar",
            Self::DivineShrine => "Divine Shrine",
        }
    }

    /// Returns the glyph for this workbench
    pub fn glyph(&self) -> char {
        match self {
            Self::BasicWorkbench => '#',
            Self::Forge | Self::Anvil | Self::AncientForge => '&',
            Self::AlchemyTable => 'A',
            Self::EnchantingAltar => 'E',
            Self::TanningRack => 'T',
            Self::JewelerBench => 'J',
            Self::DemonicAltar => 'D',
            Self::DivineShrine => 'S',
        }
    }

    /// Returns the minimum dungeon level where this workbench can be found
    pub fn min_level(&self) -> u32 {
        match self {
            Self::BasicWorkbench => 1,
            Self::Forge | Self::TanningRack => 3,
            Self::Anvil | Self::AlchemyTable => 6,
            Self::EnchantingAltar | Self::JewelerBench => 10,
            Self::AncientForge => 20,
            Self::DemonicAltar => 25,
            Self::DivineShrine => 25,
        }
    }
}

/// A workbench placed in the world
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Workbench {
    pub x: usize,
    pub y: usize,
    pub workbench_type: WorkbenchType,
}

impl Workbench {
    /// Create a new workbench at the given position
    pub fn new(x: usize, y: usize, workbench_type: WorkbenchType) -> Self {
        Self { x, y, workbench_type }
    }
}

/// Recipe category for organization
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum RecipeCategory {
    Weapons,
    Armor,
    Accessories,
    Potions,
    Scrolls,
    Materials,
    Special,
}

/// A crafting recipe
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Recipe {
    /// Unique identifier for this recipe
    pub id: &'static str,
    /// Display name
    pub name: &'static str,
    /// Category for UI organization
    pub category: RecipeCategory,
    /// Required workbench type
    pub workbench: WorkbenchType,
    /// Required materials and quantities
    pub ingredients: Vec<(CraftingMaterial, u32)>,
    /// Resulting item
    pub result: ItemKind,
    /// Number of items produced
    pub result_quantity: u32,
    /// Minimum crafting skill level required
    pub skill_required: u32,
    /// Base rarity of crafted item (can be improved with better materials)
    pub base_rarity: Rarity,
    /// Experience granted when crafting
    pub xp_granted: u32,
}

impl Recipe {
    /// Create a new recipe
    pub const fn new(
        id: &'static str,
        name: &'static str,
        category: RecipeCategory,
        workbench: WorkbenchType,
        ingredients: &'static [(CraftingMaterial, u32)],
        result: ItemKind,
        result_quantity: u32,
        skill_required: u32,
        base_rarity: Rarity,
        xp_granted: u32,
    ) -> Self {
        Self {
            id,
            name,
            category,
            workbench,
            ingredients: Vec::new(), // Will be populated at runtime
            result,
            result_quantity,
            skill_required,
            base_rarity,
            xp_granted,
        }
    }
}

/// Result of attempting to craft an item
#[derive(Debug)]
pub enum CraftResult {
    /// Successfully crafted the item
    Success {
        item: Item,
        quantity: u32,
        quality_bonus: bool,
        xp_gained: u32,
    },
    /// Missing required materials
    MissingMaterials(Vec<(CraftingMaterial, u32)>),
    /// Wrong workbench type
    WrongWorkbench(WorkbenchType),
    /// Insufficient crafting skill
    InsufficientSkill { required: u32, current: u32 },
    /// Recipe not found
    RecipeNotFound,
}

/// Source of gathered materials
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum GatherSource {
    /// Mining ore veins
    Mining,
    /// Harvesting plants
    Herbalism,
    /// Skinning defeated enemies
    Skinning,
    /// Salvaging items
    Salvaging,
    /// Looting enemies
    EnemyDrop,
    /// Found in chests or special locations
    Treasure,
}

/// A gatherable resource node in the world
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ResourceNode {
    pub x: usize,
    pub y: usize,
    pub source: GatherSource,
    pub material: CraftingMaterial,
    pub quantity_remaining: u32,
    pub respawn_turns: u32,
    pub depleted: bool,
}

impl ResourceNode {
    /// Create a new resource node
    pub fn new(x: usize, y: usize, source: GatherSource, material: CraftingMaterial, quantity: u32) -> Self {
        Self {
            x,
            y,
            source,
            material,
            quantity_remaining: quantity,
            respawn_turns: 0,
            depleted: false,
        }
    }

    /// Gather from this node and return materials obtained
    pub fn gather(&mut self, skill_bonus: u32) -> Option<MaterialStack> {
        if self.depleted {
            return None;
        }

        // Base amount gathered, with skill bonus
        let amount = 1 + (skill_bonus / 10);
        let gathered = amount.min(self.quantity_remaining);
        self.quantity_remaining -= gathered;

        if self.quantity_remaining == 0 {
            self.depleted = true;
            self.respawn_turns = 50 + (self.material.rarity() as u32 * 20);
        }

        Some(MaterialStack::new(self.material, gathered))
    }

    /// Tick the node for respawning
    pub fn tick(&mut self) {
        if self.depleted && self.respawn_turns > 0 {
            self.respawn_turns -= 1;
            if self.respawn_turns == 0 {
                self.depleted = false;
                self.quantity_remaining = 2 + (rand::random::<u32>() % 4);
            }
        }
    }

    /// Returns the glyph for this resource node
    pub fn glyph(&self) -> char {
        if self.depleted {
            '.'
        } else {
            match self.source {
                GatherSource::Mining => '^',
                GatherSource::Herbalism => '"',
                GatherSource::Skinning => '%',
                GatherSource::Salvaging => '*',
                GatherSource::EnemyDrop => '%',
                GatherSource::Treasure => '$',
            }
        }
    }
}

/// The player's crafting inventory and skills
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct CraftingInventory {
    /// Materials owned by the player
    pub materials: HashMap<CraftingMaterial, u32>,
    /// Crafting skill levels by category
    pub skill_levels: HashMap<RecipeCategory, u32>,
    /// Crafting experience by category
    pub skill_xp: HashMap<RecipeCategory, u32>,
    /// Recipes the player has learned
    pub known_recipes: Vec<String>,
}

impl CraftingInventory {
    /// Create a new empty crafting inventory
    pub fn new() -> Self {
        let mut inventory = Self {
            materials: HashMap::new(),
            skill_levels: HashMap::new(),
            skill_xp: HashMap::new(),
            known_recipes: Vec::new(),
        };
        // Initialize skill levels
        for category in [
            RecipeCategory::Weapons,
            RecipeCategory::Armor,
            RecipeCategory::Accessories,
            RecipeCategory::Potions,
            RecipeCategory::Scrolls,
            RecipeCategory::Materials,
            RecipeCategory::Special,
        ] {
            inventory.skill_levels.insert(category, 1);
            inventory.skill_xp.insert(category, 0);
        }
        inventory
    }

    /// Add materials to the inventory
    pub fn add_material(&mut self, material: CraftingMaterial, quantity: u32) {
        *self.materials.entry(material).or_insert(0) += quantity;
    }

    /// Remove materials from the inventory, returns true if successful
    pub fn remove_material(&mut self, material: CraftingMaterial, quantity: u32) -> bool {
        if let Some(count) = self.materials.get_mut(&material) {
            if *count >= quantity {
                *count -= quantity;
                if *count == 0 {
                    self.materials.remove(&material);
                }
                return true;
            }
        }
        false
    }

    /// Check if the inventory has enough of a material
    pub fn has_material(&self, material: CraftingMaterial, quantity: u32) -> bool {
        self.materials.get(&material).copied().unwrap_or(0) >= quantity
    }

    /// Get the count of a specific material
    pub fn material_count(&self, material: CraftingMaterial) -> u32 {
        self.materials.get(&material).copied().unwrap_or(0)
    }

    /// Get skill level for a category
    pub fn skill_level(&self, category: RecipeCategory) -> u32 {
        self.skill_levels.get(&category).copied().unwrap_or(1)
    }

    /// Add crafting experience and potentially level up
    pub fn add_xp(&mut self, category: RecipeCategory, xp: u32) -> bool {
        let current_xp = self.skill_xp.entry(category).or_insert(0);
        *current_xp += xp;

        let level = self.skill_levels.entry(category).or_insert(1);
        let xp_needed = *level * 100;

        if *current_xp >= xp_needed {
            *current_xp -= xp_needed;
            *level += 1;
            return true; // Leveled up
        }
        false
    }

    /// Learn a new recipe
    pub fn learn_recipe(&mut self, recipe_id: &str) -> bool {
        if !self.known_recipes.contains(&recipe_id.to_string()) {
            self.known_recipes.push(recipe_id.to_string());
            true
        } else {
            false
        }
    }

    /// Check if a recipe is known
    pub fn knows_recipe(&self, recipe_id: &str) -> bool {
        self.known_recipes.contains(&recipe_id.to_string())
    }
}

/// The main crafting system that manages recipes and crafting operations
#[derive(Clone, Debug)]
pub struct CraftingSystem {
    /// All available recipes
    recipes: Vec<Recipe>,
}

impl Default for CraftingSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl CraftingSystem {
    /// Create a new crafting system with all default recipes
    pub fn new() -> Self {
        let mut system = Self {
            recipes: Vec::new(),
        };
        system.register_default_recipes();
        system
    }

    /// Register all default recipes
    fn register_default_recipes(&mut self) {
        // === WEAPONS ===

        // Basic Weapons (BasicWorkbench)
        self.add_recipe(Recipe {
            id: "wooden_staff",
            name: "Wooden Staff",
            category: RecipeCategory::Weapons,
            workbench: WorkbenchType::BasicWorkbench,
            ingredients: vec![(CraftingMaterial::CommonWood, 3)],
            result: ItemKind::Staff,
            result_quantity: 1,
            skill_required: 1,
            base_rarity: Rarity::Common,
            xp_granted: 10,
        });

        self.add_recipe(Recipe {
            id: "wooden_bow",
            name: "Wooden Bow",
            category: RecipeCategory::Weapons,
            workbench: WorkbenchType::BasicWorkbench,
            ingredients: vec![
                (CraftingMaterial::HardWood, 2),
                (CraftingMaterial::TannedLeather, 1),
            ],
            result: ItemKind::Bow,
            result_quantity: 1,
            skill_required: 2,
            base_rarity: Rarity::Common,
            xp_granted: 15,
        });

        // Forge Weapons
        self.add_recipe(Recipe {
            id: "iron_dagger",
            name: "Iron Dagger",
            category: RecipeCategory::Weapons,
            workbench: WorkbenchType::Forge,
            ingredients: vec![
                (CraftingMaterial::IronOre, 2),
                (CraftingMaterial::CommonWood, 1),
            ],
            result: ItemKind::Dagger,
            result_quantity: 1,
            skill_required: 1,
            base_rarity: Rarity::Common,
            xp_granted: 10,
        });

        self.add_recipe(Recipe {
            id: "iron_sword",
            name: "Iron Short Sword",
            category: RecipeCategory::Weapons,
            workbench: WorkbenchType::Forge,
            ingredients: vec![
                (CraftingMaterial::IronOre, 4),
                (CraftingMaterial::CommonWood, 1),
                (CraftingMaterial::TannedLeather, 1),
            ],
            result: ItemKind::ShortSword,
            result_quantity: 1,
            skill_required: 2,
            base_rarity: Rarity::Common,
            xp_granted: 20,
        });

        self.add_recipe(Recipe {
            id: "steel_longsword",
            name: "Steel Long Sword",
            category: RecipeCategory::Weapons,
            workbench: WorkbenchType::Forge,
            ingredients: vec![
                (CraftingMaterial::SteelIngot, 4),
                (CraftingMaterial::HardWood, 1),
                (CraftingMaterial::TannedLeather, 2),
            ],
            result: ItemKind::LongSword,
            result_quantity: 1,
            skill_required: 4,
            base_rarity: Rarity::Uncommon,
            xp_granted: 35,
        });

        self.add_recipe(Recipe {
            id: "steel_greatsword",
            name: "Steel Greatsword",
            category: RecipeCategory::Weapons,
            workbench: WorkbenchType::Anvil,
            ingredients: vec![
                (CraftingMaterial::SteelIngot, 8),
                (CraftingMaterial::HardWood, 2),
                (CraftingMaterial::TannedLeather, 2),
            ],
            result: ItemKind::Greatsword,
            result_quantity: 1,
            skill_required: 6,
            base_rarity: Rarity::Uncommon,
            xp_granted: 50,
        });

        self.add_recipe(Recipe {
            id: "battle_axe",
            name: "Battle Axe",
            category: RecipeCategory::Weapons,
            workbench: WorkbenchType::Forge,
            ingredients: vec![
                (CraftingMaterial::SteelIngot, 5),
                (CraftingMaterial::HardWood, 2),
            ],
            result: ItemKind::Axe,
            result_quantity: 1,
            skill_required: 4,
            base_rarity: Rarity::Uncommon,
            xp_granted: 40,
        });

        self.add_recipe(Recipe {
            id: "war_hammer",
            name: "War Hammer",
            category: RecipeCategory::Weapons,
            workbench: WorkbenchType::Anvil,
            ingredients: vec![
                (CraftingMaterial::SteelIngot, 6),
                (CraftingMaterial::HardWood, 2),
                (CraftingMaterial::TannedLeather, 1),
            ],
            result: ItemKind::WarHammer,
            result_quantity: 1,
            skill_required: 5,
            base_rarity: Rarity::Uncommon,
            xp_granted: 45,
        });

        self.add_recipe(Recipe {
            id: "mithril_katana",
            name: "Mithril Katana",
            category: RecipeCategory::Weapons,
            workbench: WorkbenchType::Anvil,
            ingredients: vec![
                (CraftingMaterial::MithrilOre, 6),
                (CraftingMaterial::ElvenWood, 1),
                (CraftingMaterial::SilkCloth, 1),
            ],
            result: ItemKind::Katana,
            result_quantity: 1,
            skill_required: 8,
            base_rarity: Rarity::Rare,
            xp_granted: 75,
        });

        self.add_recipe(Recipe {
            id: "flame_sword",
            name: "Flame Sword",
            category: RecipeCategory::Weapons,
            workbench: WorkbenchType::AncientForge,
            ingredients: vec![
                (CraftingMaterial::AdamantiteOre, 5),
                (CraftingMaterial::FirePetal, 3),
                (CraftingMaterial::ElementalEssence, 2),
                (CraftingMaterial::RubyFragment, 1),
            ],
            result: ItemKind::FlameSword,
            result_quantity: 1,
            skill_required: 12,
            base_rarity: Rarity::Epic,
            xp_granted: 150,
        });

        self.add_recipe(Recipe {
            id: "frost_blade",
            name: "Frost Blade",
            category: RecipeCategory::Weapons,
            workbench: WorkbenchType::AncientForge,
            ingredients: vec![
                (CraftingMaterial::AdamantiteOre, 5),
                (CraftingMaterial::IceMoss, 3),
                (CraftingMaterial::ElementalEssence, 2),
                (CraftingMaterial::SapphireFragment, 1),
            ],
            result: ItemKind::FrostBlade,
            result_quantity: 1,
            skill_required: 12,
            base_rarity: Rarity::Epic,
            xp_granted: 150,
        });

        self.add_recipe(Recipe {
            id: "void_staff",
            name: "Void Staff",
            category: RecipeCategory::Weapons,
            workbench: WorkbenchType::EnchantingAltar,
            ingredients: vec![
                (CraftingMaterial::AncientWood, 3),
                (CraftingMaterial::VoidRoot, 2),
                (CraftingMaterial::VoidCrystal, 1),
                (CraftingMaterial::ArcaneEssence, 3),
            ],
            result: ItemKind::VoidStaff,
            result_quantity: 1,
            skill_required: 15,
            base_rarity: Rarity::Legendary,
            xp_granted: 200,
        });

        self.add_recipe(Recipe {
            id: "demon_slayer",
            name: "Demon Slayer",
            category: RecipeCategory::Weapons,
            workbench: WorkbenchType::DivineShrine,
            ingredients: vec![
                (CraftingMaterial::DragonsteelIngot, 6),
                (CraftingMaterial::HolyWater, 3),
                (CraftingMaterial::PhoenixFeather, 1),
                (CraftingMaterial::DiamondShard, 2),
            ],
            result: ItemKind::DemonSlayer,
            result_quantity: 1,
            skill_required: 20,
            base_rarity: Rarity::Legendary,
            xp_granted: 300,
        });

        // === ARMOR ===

        self.add_recipe(Recipe {
            id: "leather_armor",
            name: "Leather Armor",
            category: RecipeCategory::Armor,
            workbench: WorkbenchType::TanningRack,
            ingredients: vec![
                (CraftingMaterial::TannedLeather, 5),
                (CraftingMaterial::SilkCloth, 1),
            ],
            result: ItemKind::LeatherArmor,
            result_quantity: 1,
            skill_required: 1,
            base_rarity: Rarity::Common,
            xp_granted: 15,
        });

        self.add_recipe(Recipe {
            id: "chain_mail",
            name: "Chain Mail",
            category: RecipeCategory::Armor,
            workbench: WorkbenchType::Forge,
            ingredients: vec![
                (CraftingMaterial::IronOre, 8),
                (CraftingMaterial::TannedLeather, 2),
            ],
            result: ItemKind::ChainMail,
            result_quantity: 1,
            skill_required: 3,
            base_rarity: Rarity::Common,
            xp_granted: 25,
        });

        self.add_recipe(Recipe {
            id: "scale_mail",
            name: "Scale Mail",
            category: RecipeCategory::Armor,
            workbench: WorkbenchType::Forge,
            ingredients: vec![
                (CraftingMaterial::SteelIngot, 6),
                (CraftingMaterial::TannedLeather, 3),
            ],
            result: ItemKind::ScaleMail,
            result_quantity: 1,
            skill_required: 5,
            base_rarity: Rarity::Uncommon,
            xp_granted: 40,
        });

        self.add_recipe(Recipe {
            id: "plate_mail",
            name: "Plate Mail",
            category: RecipeCategory::Armor,
            workbench: WorkbenchType::Anvil,
            ingredients: vec![
                (CraftingMaterial::SteelIngot, 10),
                (CraftingMaterial::TannedLeather, 4),
                (CraftingMaterial::SilkCloth, 2),
            ],
            result: ItemKind::PlateMail,
            result_quantity: 1,
            skill_required: 7,
            base_rarity: Rarity::Rare,
            xp_granted: 60,
        });

        self.add_recipe(Recipe {
            id: "mage_robes",
            name: "Mage Robes",
            category: RecipeCategory::Armor,
            workbench: WorkbenchType::EnchantingAltar,
            ingredients: vec![
                (CraftingMaterial::SilkCloth, 5),
                (CraftingMaterial::ArcaneEssence, 2),
                (CraftingMaterial::ManaBloom, 2),
            ],
            result: ItemKind::MageRobes,
            result_quantity: 1,
            skill_required: 5,
            base_rarity: Rarity::Uncommon,
            xp_granted: 45,
        });

        self.add_recipe(Recipe {
            id: "dragon_armor",
            name: "Dragon Armor",
            category: RecipeCategory::Armor,
            workbench: WorkbenchType::AncientForge,
            ingredients: vec![
                (CraftingMaterial::DragonHide, 6),
                (CraftingMaterial::DragonScale, 4),
                (CraftingMaterial::AdamantiteOre, 4),
                (CraftingMaterial::EnchantedFabric, 2),
            ],
            result: ItemKind::DragonArmor,
            result_quantity: 1,
            skill_required: 15,
            base_rarity: Rarity::Legendary,
            xp_granted: 250,
        });

        self.add_recipe(Recipe {
            id: "demon_armor",
            name: "Demon Armor",
            category: RecipeCategory::Armor,
            workbench: WorkbenchType::DemonicAltar,
            ingredients: vec![
                (CraftingMaterial::DemonSkin, 6),
                (CraftingMaterial::DemonBlood, 3),
                (CraftingMaterial::DarksteelIngot, 4),
                (CraftingMaterial::DarkEnergy, 2),
            ],
            result: ItemKind::DemonArmor,
            result_quantity: 1,
            skill_required: 18,
            base_rarity: Rarity::Legendary,
            xp_granted: 280,
        });

        self.add_recipe(Recipe {
            id: "holy_armor",
            name: "Holy Armor",
            category: RecipeCategory::Armor,
            workbench: WorkbenchType::DivineShrine,
            ingredients: vec![
                (CraftingMaterial::MithrilOre, 8),
                (CraftingMaterial::HolyWater, 4),
                (CraftingMaterial::EnchantedFabric, 3),
                (CraftingMaterial::DiamondShard, 2),
            ],
            result: ItemKind::HolyArmor,
            result_quantity: 1,
            skill_required: 18,
            base_rarity: Rarity::Legendary,
            xp_granted: 280,
        });

        // === SHIELDS ===

        self.add_recipe(Recipe {
            id: "wooden_shield",
            name: "Wooden Shield",
            category: RecipeCategory::Armor,
            workbench: WorkbenchType::BasicWorkbench,
            ingredients: vec![
                (CraftingMaterial::HardWood, 4),
                (CraftingMaterial::IronOre, 2),
            ],
            result: ItemKind::WoodenShield,
            result_quantity: 1,
            skill_required: 1,
            base_rarity: Rarity::Common,
            xp_granted: 12,
        });

        self.add_recipe(Recipe {
            id: "iron_shield",
            name: "Iron Shield",
            category: RecipeCategory::Armor,
            workbench: WorkbenchType::Forge,
            ingredients: vec![
                (CraftingMaterial::IronOre, 6),
                (CraftingMaterial::TannedLeather, 2),
            ],
            result: ItemKind::IronShield,
            result_quantity: 1,
            skill_required: 3,
            base_rarity: Rarity::Common,
            xp_granted: 20,
        });

        self.add_recipe(Recipe {
            id: "tower_shield",
            name: "Tower Shield",
            category: RecipeCategory::Armor,
            workbench: WorkbenchType::Anvil,
            ingredients: vec![
                (CraftingMaterial::SteelIngot, 8),
                (CraftingMaterial::TannedLeather, 3),
                (CraftingMaterial::HardWood, 2),
            ],
            result: ItemKind::TowerShield,
            result_quantity: 1,
            skill_required: 6,
            base_rarity: Rarity::Uncommon,
            xp_granted: 50,
        });

        self.add_recipe(Recipe {
            id: "dragon_shield",
            name: "Dragon Shield",
            category: RecipeCategory::Armor,
            workbench: WorkbenchType::AncientForge,
            ingredients: vec![
                (CraftingMaterial::DragonScale, 5),
                (CraftingMaterial::DragonHide, 2),
                (CraftingMaterial::AdamantiteOre, 3),
            ],
            result: ItemKind::DragonShield,
            result_quantity: 1,
            skill_required: 14,
            base_rarity: Rarity::Epic,
            xp_granted: 180,
        });

        // === HELMETS ===

        self.add_recipe(Recipe {
            id: "leather_cap",
            name: "Leather Cap",
            category: RecipeCategory::Armor,
            workbench: WorkbenchType::TanningRack,
            ingredients: vec![(CraftingMaterial::TannedLeather, 2)],
            result: ItemKind::LeatherCap,
            result_quantity: 1,
            skill_required: 1,
            base_rarity: Rarity::Common,
            xp_granted: 8,
        });

        self.add_recipe(Recipe {
            id: "iron_helm",
            name: "Iron Helm",
            category: RecipeCategory::Armor,
            workbench: WorkbenchType::Forge,
            ingredients: vec![
                (CraftingMaterial::IronOre, 4),
                (CraftingMaterial::TannedLeather, 1),
            ],
            result: ItemKind::IronHelm,
            result_quantity: 1,
            skill_required: 2,
            base_rarity: Rarity::Common,
            xp_granted: 15,
        });

        self.add_recipe(Recipe {
            id: "steel_helm",
            name: "Steel Helm",
            category: RecipeCategory::Armor,
            workbench: WorkbenchType::Anvil,
            ingredients: vec![
                (CraftingMaterial::SteelIngot, 4),
                (CraftingMaterial::TannedLeather, 1),
            ],
            result: ItemKind::SteelHelm,
            result_quantity: 1,
            skill_required: 5,
            base_rarity: Rarity::Uncommon,
            xp_granted: 30,
        });

        self.add_recipe(Recipe {
            id: "wizard_hat",
            name: "Wizard's Hat",
            category: RecipeCategory::Armor,
            workbench: WorkbenchType::EnchantingAltar,
            ingredients: vec![
                (CraftingMaterial::SilkCloth, 3),
                (CraftingMaterial::ArcaneEssence, 1),
                (CraftingMaterial::ManaBloom, 1),
            ],
            result: ItemKind::WizardHat,
            result_quantity: 1,
            skill_required: 4,
            base_rarity: Rarity::Uncommon,
            xp_granted: 25,
        });

        // === GLOVES & BOOTS ===

        self.add_recipe(Recipe {
            id: "leather_gloves",
            name: "Leather Gloves",
            category: RecipeCategory::Armor,
            workbench: WorkbenchType::TanningRack,
            ingredients: vec![(CraftingMaterial::TannedLeather, 2)],
            result: ItemKind::LeatherGloves,
            result_quantity: 1,
            skill_required: 1,
            base_rarity: Rarity::Common,
            xp_granted: 8,
        });

        self.add_recipe(Recipe {
            id: "leather_boots",
            name: "Leather Boots",
            category: RecipeCategory::Armor,
            workbench: WorkbenchType::TanningRack,
            ingredients: vec![(CraftingMaterial::TannedLeather, 3)],
            result: ItemKind::LeatherBoots,
            result_quantity: 1,
            skill_required: 1,
            base_rarity: Rarity::Common,
            xp_granted: 10,
        });

        self.add_recipe(Recipe {
            id: "iron_gauntlets",
            name: "Iron Gauntlets",
            category: RecipeCategory::Armor,
            workbench: WorkbenchType::Forge,
            ingredients: vec![
                (CraftingMaterial::IronOre, 3),
                (CraftingMaterial::TannedLeather, 1),
            ],
            result: ItemKind::IronGauntlets,
            result_quantity: 1,
            skill_required: 2,
            base_rarity: Rarity::Common,
            xp_granted: 12,
        });

        self.add_recipe(Recipe {
            id: "boots_of_speed",
            name: "Boots of Speed",
            category: RecipeCategory::Armor,
            workbench: WorkbenchType::EnchantingAltar,
            ingredients: vec![
                (CraftingMaterial::TannedLeather, 3),
                (CraftingMaterial::ArcaneEssence, 2),
                (CraftingMaterial::ElementalEssence, 1),
            ],
            result: ItemKind::BootsOfSpeed,
            result_quantity: 1,
            skill_required: 8,
            base_rarity: Rarity::Rare,
            xp_granted: 70,
        });

        // === ACCESSORIES ===

        self.add_recipe(Recipe {
            id: "ring_of_strength",
            name: "Ring of Strength",
            category: RecipeCategory::Accessories,
            workbench: WorkbenchType::JewelerBench,
            ingredients: vec![
                (CraftingMaterial::IronOre, 2),
                (CraftingMaterial::RubyFragment, 1),
            ],
            result: ItemKind::RingOfStrength,
            result_quantity: 1,
            skill_required: 3,
            base_rarity: Rarity::Uncommon,
            xp_granted: 30,
        });

        self.add_recipe(Recipe {
            id: "ring_of_protection",
            name: "Ring of Protection",
            category: RecipeCategory::Accessories,
            workbench: WorkbenchType::JewelerBench,
            ingredients: vec![
                (CraftingMaterial::SteelIngot, 2),
                (CraftingMaterial::SapphireFragment, 1),
            ],
            result: ItemKind::RingOfProtection,
            result_quantity: 1,
            skill_required: 3,
            base_rarity: Rarity::Uncommon,
            xp_granted: 30,
        });

        self.add_recipe(Recipe {
            id: "ring_of_mana",
            name: "Ring of Mana",
            category: RecipeCategory::Accessories,
            workbench: WorkbenchType::JewelerBench,
            ingredients: vec![
                (CraftingMaterial::MithrilOre, 2),
                (CraftingMaterial::SapphireFragment, 1),
                (CraftingMaterial::ArcaneEssence, 1),
            ],
            result: ItemKind::RingOfMana,
            result_quantity: 1,
            skill_required: 6,
            base_rarity: Rarity::Rare,
            xp_granted: 50,
        });

        self.add_recipe(Recipe {
            id: "ring_of_the_ancients",
            name: "Ring of the Ancients",
            category: RecipeCategory::Accessories,
            workbench: WorkbenchType::JewelerBench,
            ingredients: vec![
                (CraftingMaterial::DragonsteelIngot, 2),
                (CraftingMaterial::DiamondShard, 2),
                (CraftingMaterial::SoulCrystal, 1),
                (CraftingMaterial::AncientRelic, 1),
            ],
            result: ItemKind::RingOfTheAncients,
            result_quantity: 1,
            skill_required: 18,
            base_rarity: Rarity::Legendary,
            xp_granted: 250,
        });

        self.add_recipe(Recipe {
            id: "amulet_of_health",
            name: "Amulet of Health",
            category: RecipeCategory::Accessories,
            workbench: WorkbenchType::JewelerBench,
            ingredients: vec![
                (CraftingMaterial::SteelIngot, 2),
                (CraftingMaterial::EmeraldFragment, 1),
                (CraftingMaterial::LifeFlower, 1),
            ],
            result: ItemKind::AmuletOfHealth,
            result_quantity: 1,
            skill_required: 5,
            base_rarity: Rarity::Uncommon,
            xp_granted: 40,
        });

        self.add_recipe(Recipe {
            id: "amulet_of_the_gods",
            name: "Amulet of the Gods",
            category: RecipeCategory::Accessories,
            workbench: WorkbenchType::DivineShrine,
            ingredients: vec![
                (CraftingMaterial::DragonsteelIngot, 3),
                (CraftingMaterial::DiamondShard, 3),
                (CraftingMaterial::GodsbloodVial, 1),
                (CraftingMaterial::HolyWater, 2),
            ],
            result: ItemKind::AmuletOfTheGods,
            result_quantity: 1,
            skill_required: 25,
            base_rarity: Rarity::Mythic,
            xp_granted: 500,
        });

        // === POTIONS ===

        self.add_recipe(Recipe {
            id: "health_potion",
            name: "Health Potion",
            category: RecipeCategory::Potions,
            workbench: WorkbenchType::AlchemyTable,
            ingredients: vec![
                (CraftingMaterial::HealingHerb, 2),
                (CraftingMaterial::LifeFlower, 1),
            ],
            result: ItemKind::HealthPotion,
            result_quantity: 2,
            skill_required: 1,
            base_rarity: Rarity::Common,
            xp_granted: 10,
        });

        self.add_recipe(Recipe {
            id: "mana_potion",
            name: "Mana Potion",
            category: RecipeCategory::Potions,
            workbench: WorkbenchType::AlchemyTable,
            ingredients: vec![
                (CraftingMaterial::ManaBloom, 2),
                (CraftingMaterial::ArcaneEssence, 1),
            ],
            result: ItemKind::ManaPotion,
            result_quantity: 2,
            skill_required: 1,
            base_rarity: Rarity::Common,
            xp_granted: 10,
        });

        self.add_recipe(Recipe {
            id: "strength_potion",
            name: "Strength Potion",
            category: RecipeCategory::Potions,
            workbench: WorkbenchType::AlchemyTable,
            ingredients: vec![
                (CraftingMaterial::FangOrClaw, 2),
                (CraftingMaterial::HealingHerb, 1),
                (CraftingMaterial::BoneFragment, 1),
            ],
            result: ItemKind::StrengthPotion,
            result_quantity: 1,
            skill_required: 3,
            base_rarity: Rarity::Uncommon,
            xp_granted: 20,
        });

        self.add_recipe(Recipe {
            id: "defense_potion",
            name: "Defense Potion",
            category: RecipeCategory::Potions,
            workbench: WorkbenchType::AlchemyTable,
            ingredients: vec![
                (CraftingMaterial::BoneFragment, 3),
                (CraftingMaterial::HealingHerb, 1),
            ],
            result: ItemKind::DefensePotion,
            result_quantity: 1,
            skill_required: 3,
            base_rarity: Rarity::Uncommon,
            xp_granted: 20,
        });

        self.add_recipe(Recipe {
            id: "fire_resist_potion",
            name: "Fire Resistance Potion",
            category: RecipeCategory::Potions,
            workbench: WorkbenchType::AlchemyTable,
            ingredients: vec![
                (CraftingMaterial::FirePetal, 2),
                (CraftingMaterial::ElementalEssence, 1),
            ],
            result: ItemKind::FireResistPotion,
            result_quantity: 1,
            skill_required: 5,
            base_rarity: Rarity::Rare,
            xp_granted: 35,
        });

        self.add_recipe(Recipe {
            id: "ice_resist_potion",
            name: "Ice Resistance Potion",
            category: RecipeCategory::Potions,
            workbench: WorkbenchType::AlchemyTable,
            ingredients: vec![
                (CraftingMaterial::IceMoss, 2),
                (CraftingMaterial::ElementalEssence, 1),
            ],
            result: ItemKind::IceResistPotion,
            result_quantity: 1,
            skill_required: 5,
            base_rarity: Rarity::Rare,
            xp_granted: 35,
        });

        self.add_recipe(Recipe {
            id: "antidote",
            name: "Antidote",
            category: RecipeCategory::Potions,
            workbench: WorkbenchType::AlchemyTable,
            ingredients: vec![
                (CraftingMaterial::VenomSac, 1),
                (CraftingMaterial::HealingHerb, 2),
            ],
            result: ItemKind::PoisonResistPotion,
            result_quantity: 2,
            skill_required: 2,
            base_rarity: Rarity::Common,
            xp_granted: 15,
        });

        self.add_recipe(Recipe {
            id: "regeneration_potion",
            name: "Regeneration Potion",
            category: RecipeCategory::Potions,
            workbench: WorkbenchType::AlchemyTable,
            ingredients: vec![
                (CraftingMaterial::LifeFlower, 2),
                (CraftingMaterial::HealingHerb, 2),
                (CraftingMaterial::GhostEctoplasm, 1),
            ],
            result: ItemKind::RegenerationPotion,
            result_quantity: 1,
            skill_required: 7,
            base_rarity: Rarity::Rare,
            xp_granted: 50,
        });

        self.add_recipe(Recipe {
            id: "invisibility_potion",
            name: "Invisibility Potion",
            category: RecipeCategory::Potions,
            workbench: WorkbenchType::AlchemyTable,
            ingredients: vec![
                (CraftingMaterial::GhostEctoplasm, 2),
                (CraftingMaterial::Nightshade, 1),
                (CraftingMaterial::ArcaneEssence, 1),
            ],
            result: ItemKind::InvisibilityPotion,
            result_quantity: 1,
            skill_required: 8,
            base_rarity: Rarity::Rare,
            xp_granted: 60,
        });

        self.add_recipe(Recipe {
            id: "full_restore",
            name: "Full Restore Elixir",
            category: RecipeCategory::Potions,
            workbench: WorkbenchType::AlchemyTable,
            ingredients: vec![
                (CraftingMaterial::LifeFlower, 3),
                (CraftingMaterial::ManaBloom, 3),
                (CraftingMaterial::PhoenixFeather, 1),
            ],
            result: ItemKind::FullRestorePotion,
            result_quantity: 1,
            skill_required: 15,
            base_rarity: Rarity::Legendary,
            xp_granted: 150,
        });

        self.add_recipe(Recipe {
            id: "ultimate_power",
            name: "Ultimate Power Elixir",
            category: RecipeCategory::Potions,
            workbench: WorkbenchType::AlchemyTable,
            ingredients: vec![
                (CraftingMaterial::DragonHeart, 1),
                (CraftingMaterial::DemonBlood, 2),
                (CraftingMaterial::GodsbloodVial, 1),
                (CraftingMaterial::VoidRoot, 2),
            ],
            result: ItemKind::UltimatePowerPotion,
            result_quantity: 1,
            skill_required: 25,
            base_rarity: Rarity::Mythic,
            xp_granted: 400,
        });

        // === SCROLLS ===

        self.add_recipe(Recipe {
            id: "scroll_teleport",
            name: "Scroll of Teleport",
            category: RecipeCategory::Scrolls,
            workbench: WorkbenchType::EnchantingAltar,
            ingredients: vec![
                (CraftingMaterial::EnchantedInk, 1),
                (CraftingMaterial::ArcaneEssence, 2),
            ],
            result: ItemKind::ScrollTeleport,
            result_quantity: 1,
            skill_required: 3,
            base_rarity: Rarity::Uncommon,
            xp_granted: 25,
        });

        self.add_recipe(Recipe {
            id: "scroll_fireball",
            name: "Scroll of Fireball",
            category: RecipeCategory::Scrolls,
            workbench: WorkbenchType::EnchantingAltar,
            ingredients: vec![
                (CraftingMaterial::EnchantedInk, 1),
                (CraftingMaterial::FirePetal, 2),
                (CraftingMaterial::ElementalEssence, 1),
            ],
            result: ItemKind::ScrollFireball,
            result_quantity: 1,
            skill_required: 5,
            base_rarity: Rarity::Rare,
            xp_granted: 40,
        });

        self.add_recipe(Recipe {
            id: "scroll_ice_storm",
            name: "Scroll of Ice Storm",
            category: RecipeCategory::Scrolls,
            workbench: WorkbenchType::EnchantingAltar,
            ingredients: vec![
                (CraftingMaterial::EnchantedInk, 1),
                (CraftingMaterial::IceMoss, 2),
                (CraftingMaterial::ElementalEssence, 1),
            ],
            result: ItemKind::ScrollIceStorm,
            result_quantity: 1,
            skill_required: 5,
            base_rarity: Rarity::Rare,
            xp_granted: 40,
        });

        self.add_recipe(Recipe {
            id: "scroll_mapping",
            name: "Scroll of Mapping",
            category: RecipeCategory::Scrolls,
            workbench: WorkbenchType::EnchantingAltar,
            ingredients: vec![
                (CraftingMaterial::EnchantedInk, 1),
                (CraftingMaterial::ArcaneEssence, 1),
                (CraftingMaterial::MonsterEye, 1),
            ],
            result: ItemKind::ScrollMapping,
            result_quantity: 1,
            skill_required: 4,
            base_rarity: Rarity::Uncommon,
            xp_granted: 30,
        });

        self.add_recipe(Recipe {
            id: "scroll_enchant",
            name: "Scroll of Enchant",
            category: RecipeCategory::Scrolls,
            workbench: WorkbenchType::EnchantingAltar,
            ingredients: vec![
                (CraftingMaterial::EnchantedInk, 2),
                (CraftingMaterial::ArcaneEssence, 3),
                (CraftingMaterial::RuneStone, 1),
            ],
            result: ItemKind::ScrollEnchant,
            result_quantity: 1,
            skill_required: 10,
            base_rarity: Rarity::Epic,
            xp_granted: 100,
        });

        self.add_recipe(Recipe {
            id: "scroll_divine_wrath",
            name: "Scroll of Divine Wrath",
            category: RecipeCategory::Scrolls,
            workbench: WorkbenchType::DivineShrine,
            ingredients: vec![
                (CraftingMaterial::EnchantedInk, 3),
                (CraftingMaterial::HolyWater, 3),
                (CraftingMaterial::SoulCrystal, 1),
                (CraftingMaterial::DiamondShard, 1),
            ],
            result: ItemKind::ScrollDivineWrath,
            result_quantity: 1,
            skill_required: 20,
            base_rarity: Rarity::Legendary,
            xp_granted: 250,
        });

        // === MATERIAL PROCESSING ===

        self.add_recipe(Recipe {
            id: "smelt_steel",
            name: "Smelt Steel Ingot",
            category: RecipeCategory::Materials,
            workbench: WorkbenchType::Forge,
            ingredients: vec![
                (CraftingMaterial::IronOre, 3),
                (CraftingMaterial::CommonWood, 1), // For fuel
            ],
            result: ItemKind::Gold, // Placeholder - represents crafting material output
            result_quantity: 2,
            skill_required: 2,
            base_rarity: Rarity::Common,
            xp_granted: 15,
        });

        self.add_recipe(Recipe {
            id: "tan_leather",
            name: "Tan Leather",
            category: RecipeCategory::Materials,
            workbench: WorkbenchType::TanningRack,
            ingredients: vec![(CraftingMaterial::RawLeather, 2)],
            result: ItemKind::Gold, // Placeholder
            result_quantity: 1,
            skill_required: 1,
            base_rarity: Rarity::Common,
            xp_granted: 8,
        });

        self.add_recipe(Recipe {
            id: "enchanted_ink",
            name: "Enchanted Ink",
            category: RecipeCategory::Materials,
            workbench: WorkbenchType::AlchemyTable,
            ingredients: vec![
                (CraftingMaterial::Nightshade, 1),
                (CraftingMaterial::ArcaneEssence, 1),
                (CraftingMaterial::MonsterEye, 1),
            ],
            result: ItemKind::Gold, // Placeholder
            result_quantity: 2,
            skill_required: 4,
            base_rarity: Rarity::Rare,
            xp_granted: 30,
        });

        self.add_recipe(Recipe {
            id: "forge_darksteel",
            name: "Forge Darksteel Ingot",
            category: RecipeCategory::Materials,
            workbench: WorkbenchType::AncientForge,
            ingredients: vec![
                (CraftingMaterial::AdamantiteOre, 2),
                (CraftingMaterial::DarkEnergy, 1),
                (CraftingMaterial::DemonBlood, 1),
            ],
            result: ItemKind::Gold, // Placeholder
            result_quantity: 1,
            skill_required: 12,
            base_rarity: Rarity::Legendary,
            xp_granted: 100,
        });

        self.add_recipe(Recipe {
            id: "forge_dragonsteel",
            name: "Forge Dragonsteel Ingot",
            category: RecipeCategory::Materials,
            workbench: WorkbenchType::AncientForge,
            ingredients: vec![
                (CraftingMaterial::DarksteelIngot, 2),
                (CraftingMaterial::DragonScale, 1),
                (CraftingMaterial::ElementalCore, 1),
            ],
            result: ItemKind::Gold, // Placeholder
            result_quantity: 1,
            skill_required: 18,
            base_rarity: Rarity::Mythic,
            xp_granted: 200,
        });
    }

    /// Add a recipe to the system
    pub fn add_recipe(&mut self, recipe: Recipe) {
        self.recipes.push(recipe);
    }

    /// Get all recipes
    pub fn recipes(&self) -> &[Recipe] {
        &self.recipes
    }

    /// Get recipes for a specific workbench
    pub fn recipes_for_workbench(&self, workbench: WorkbenchType) -> Vec<&Recipe> {
        self.recipes.iter()
            .filter(|r| r.workbench == workbench)
            .collect()
    }

    /// Get recipes in a category
    pub fn recipes_in_category(&self, category: RecipeCategory) -> Vec<&Recipe> {
        self.recipes.iter()
            .filter(|r| r.category == category)
            .collect()
    }

    /// Get a recipe by ID
    pub fn get_recipe(&self, id: &str) -> Option<&Recipe> {
        self.recipes.iter().find(|r| r.id == id)
    }

    /// Check if a recipe can be crafted with the given inventory
    pub fn can_craft(&self, recipe_id: &str, inventory: &CraftingInventory) -> Result<(), CraftResult> {
        let recipe = match self.get_recipe(recipe_id) {
            Some(r) => r,
            None => return Err(CraftResult::RecipeNotFound),
        };

        // Check skill level
        let current_skill = inventory.skill_level(recipe.category);
        if current_skill < recipe.skill_required {
            return Err(CraftResult::InsufficientSkill {
                required: recipe.skill_required,
                current: current_skill,
            });
        }

        // Check materials
        let mut missing = Vec::new();
        for (material, quantity) in &recipe.ingredients {
            let have = inventory.material_count(*material);
            if have < *quantity {
                missing.push((*material, *quantity - have));
            }
        }

        if !missing.is_empty() {
            return Err(CraftResult::MissingMaterials(missing));
        }

        Ok(())
    }

    /// Attempt to craft an item
    pub fn craft(
        &self,
        recipe_id: &str,
        inventory: &mut CraftingInventory,
        workbench: WorkbenchType,
    ) -> CraftResult {
        let recipe = match self.get_recipe(recipe_id) {
            Some(r) => r.clone(),
            None => return CraftResult::RecipeNotFound,
        };

        // Check workbench
        if recipe.workbench != workbench {
            return CraftResult::WrongWorkbench(recipe.workbench);
        }

        // Check if can craft
        if let Err(result) = self.can_craft(recipe_id, inventory) {
            return result;
        }

        // Consume materials
        for (material, quantity) in &recipe.ingredients {
            inventory.remove_material(*material, *quantity);
        }

        // Calculate quality bonus based on skill
        let current_skill = inventory.skill_level(recipe.category);
        let skill_bonus = current_skill.saturating_sub(recipe.skill_required);
        let quality_bonus = skill_bonus >= 3;

        // Determine final rarity
        let final_rarity = if quality_bonus {
            match recipe.base_rarity {
                Rarity::Common => Rarity::Uncommon,
                Rarity::Uncommon => Rarity::Rare,
                Rarity::Rare => Rarity::Epic,
                Rarity::Epic => Rarity::Legendary,
                Rarity::Legendary | Rarity::Mythic => Rarity::Mythic,
            }
        } else {
            recipe.base_rarity
        };

        // Create the item
        let item = Item::new(0, 0, recipe.result, final_rarity);

        // Grant XP
        let xp_gained = recipe.xp_granted + (skill_bonus * 5);
        inventory.add_xp(recipe.category, xp_gained);

        CraftResult::Success {
            item,
            quantity: recipe.result_quantity,
            quality_bonus,
            xp_gained,
        }
    }

    /// Get available recipes that can be crafted with current materials
    pub fn available_recipes(&self, inventory: &CraftingInventory, workbench: Option<WorkbenchType>) -> Vec<&Recipe> {
        self.recipes.iter()
            .filter(|r| {
                // Filter by workbench if specified
                if let Some(wb) = workbench {
                    if r.workbench != wb {
                        return false;
                    }
                }

                // Check skill level
                if inventory.skill_level(r.category) < r.skill_required {
                    return false;
                }

                // Check materials
                for (material, quantity) in &r.ingredients {
                    if inventory.material_count(*material) < *quantity {
                        return false;
                    }
                }

                true
            })
            .collect()
    }
}

/// Get materials that can drop from an enemy based on dungeon level
pub fn get_enemy_drops(dungeon_level: u32, is_boss: bool) -> Vec<(CraftingMaterial, u32, f32)> {
    let mut drops = Vec::new();

    // Common drops for all levels
    drops.push((CraftingMaterial::BoneFragment, 1, 0.3));
    drops.push((CraftingMaterial::RawLeather, 1, 0.2));

    // Tier-based drops
    match dungeon_level {
        1..=4 => {
            drops.push((CraftingMaterial::FangOrClaw, 1, 0.15));
        }
        5..=8 => {
            drops.push((CraftingMaterial::FangOrClaw, 1, 0.2));
            drops.push((CraftingMaterial::VenomSac, 1, 0.1));
            drops.push((CraftingMaterial::MonsterEye, 1, 0.08));
        }
        9..=12 => {
            drops.push((CraftingMaterial::GhostEctoplasm, 1, 0.15));
            drops.push((CraftingMaterial::BoneFragment, 2, 0.2));
            drops.push((CraftingMaterial::MonsterEye, 1, 0.12));
        }
        13..=16 => {
            drops.push((CraftingMaterial::VenomSac, 1, 0.15));
            drops.push((CraftingMaterial::FangOrClaw, 2, 0.2));
        }
        17..=20 => {
            drops.push((CraftingMaterial::ElementalEssence, 1, 0.15));
            drops.push((CraftingMaterial::IceMoss, 1, 0.1));
        }
        21..=24 => {
            drops.push((CraftingMaterial::ElementalEssence, 1, 0.2));
            drops.push((CraftingMaterial::FirePetal, 1, 0.1));
            drops.push((CraftingMaterial::ElementalCore, 1, 0.05));
        }
        25..=28 => {
            drops.push((CraftingMaterial::ArcaneEssence, 1, 0.15));
            drops.push((CraftingMaterial::LichDust, 1, 0.08));
            drops.push((CraftingMaterial::SoulCrystal, 1, 0.05));
        }
        _ => {
            drops.push((CraftingMaterial::DemonBlood, 1, 0.2));
            drops.push((CraftingMaterial::DarkEnergy, 1, 0.1));
            drops.push((CraftingMaterial::DemonSkin, 1, 0.15));
        }
    }

    // Boss-only drops
    if is_boss {
        match dungeon_level {
            5 => drops.push((CraftingMaterial::RubyFragment, 1, 0.5)),
            10 => drops.push((CraftingMaterial::SapphireFragment, 1, 0.5)),
            15 => drops.push((CraftingMaterial::GhostEctoplasm, 3, 0.8)),
            20 => drops.push((CraftingMaterial::AncientWood, 2, 0.6)),
            25 => {
                drops.push((CraftingMaterial::DragonScale, 2, 0.7));
                drops.push((CraftingMaterial::DragonHide, 2, 0.6));
            }
            30 => {
                drops.push((CraftingMaterial::DemonHeart, 1, 0.8));
                drops.push((CraftingMaterial::DragonHeart, 1, 0.5));
                drops.push((CraftingMaterial::GodsbloodVial, 1, 0.2));
            }
            _ => {}
        }
    }

    drops
}

/// Get resource nodes that can spawn on a dungeon level
pub fn get_level_resources(dungeon_level: u32) -> Vec<(GatherSource, CraftingMaterial, u32)> {
    let mut resources = Vec::new();

    // Mining nodes
    resources.push((GatherSource::Mining, CraftingMaterial::IronOre, 3));

    if dungeon_level >= 5 {
        resources.push((GatherSource::Mining, CraftingMaterial::SteelIngot, 2));
    }
    if dungeon_level >= 10 {
        resources.push((GatherSource::Mining, CraftingMaterial::MithrilOre, 2));
    }
    if dungeon_level >= 18 {
        resources.push((GatherSource::Mining, CraftingMaterial::AdamantiteOre, 2));
    }

    // Herbs
    resources.push((GatherSource::Herbalism, CraftingMaterial::HealingHerb, 3));
    resources.push((GatherSource::Herbalism, CraftingMaterial::ManaBloom, 2));

    if dungeon_level >= 8 {
        resources.push((GatherSource::Herbalism, CraftingMaterial::Nightshade, 2));
    }
    if dungeon_level >= 15 {
        resources.push((GatherSource::Herbalism, CraftingMaterial::LifeFlower, 1));
    }
    if dungeon_level >= 20 {
        resources.push((GatherSource::Herbalism, CraftingMaterial::FirePetal, 2));
        resources.push((GatherSource::Herbalism, CraftingMaterial::IceMoss, 2));
    }
    if dungeon_level >= 25 {
        resources.push((GatherSource::Herbalism, CraftingMaterial::VoidRoot, 1));
        resources.push((GatherSource::Herbalism, CraftingMaterial::DeathCap, 1));
    }

    // Gems (rare in all levels, better at higher levels)
    if dungeon_level >= 5 {
        resources.push((GatherSource::Mining, CraftingMaterial::RubyFragment, 1));
        resources.push((GatherSource::Mining, CraftingMaterial::SapphireFragment, 1));
        resources.push((GatherSource::Mining, CraftingMaterial::EmeraldFragment, 1));
    }
    if dungeon_level >= 15 {
        resources.push((GatherSource::Mining, CraftingMaterial::DiamondShard, 1));
    }
    if dungeon_level >= 25 {
        resources.push((GatherSource::Mining, CraftingMaterial::SoulCrystal, 1));
        resources.push((GatherSource::Mining, CraftingMaterial::VoidCrystal, 1));
    }

    resources
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_rarity() {
        assert!(CraftingMaterial::IronOre.rarity() == MaterialRarity::Common);
        assert!(CraftingMaterial::DragonHeart.rarity() == MaterialRarity::Mythic);
    }

    #[test]
    fn test_crafting_inventory() {
        let mut inv = CraftingInventory::new();
        inv.add_material(CraftingMaterial::IronOre, 5);
        assert_eq!(inv.material_count(CraftingMaterial::IronOre), 5);

        assert!(inv.remove_material(CraftingMaterial::IronOre, 3));
        assert_eq!(inv.material_count(CraftingMaterial::IronOre), 2);

        assert!(!inv.remove_material(CraftingMaterial::IronOre, 5));
        assert_eq!(inv.material_count(CraftingMaterial::IronOre), 2);
    }

    #[test]
    fn test_crafting_system() {
        let system = CraftingSystem::new();
        let mut inv = CraftingInventory::new();

        // Add materials for iron dagger
        inv.add_material(CraftingMaterial::IronOre, 2);
        inv.add_material(CraftingMaterial::CommonWood, 1);

        // Should be able to craft
        let available = system.available_recipes(&inv, Some(WorkbenchType::Forge));
        assert!(!available.is_empty());

        // Craft the dagger
        let result = system.craft("iron_dagger", &mut inv, WorkbenchType::Forge);
        match result {
            CraftResult::Success { item, .. } => {
                assert_eq!(item.kind, ItemKind::Dagger);
            }
            _ => panic!("Crafting should have succeeded"),
        }

        // Materials should be consumed
        assert_eq!(inv.material_count(CraftingMaterial::IronOre), 0);
    }

    #[test]
    fn test_resource_node() {
        let mut node = ResourceNode::new(
            5, 5,
            GatherSource::Mining,
            CraftingMaterial::IronOre,
            3
        );

        assert!(!node.depleted);

        // Gather until depleted
        while !node.depleted {
            let _ = node.gather(0);
        }

        assert!(node.depleted);
        assert!(node.gather(0).is_none());

        // Tick until respawn
        while node.depleted {
            node.tick();
        }

        assert!(!node.depleted);
        assert!(node.quantity_remaining > 0);
    }

    #[test]
    fn test_skill_leveling() {
        let mut inv = CraftingInventory::new();

        let initial_level = inv.skill_level(RecipeCategory::Weapons);
        assert_eq!(initial_level, 1);

        // Add enough XP to level up
        let leveled = inv.add_xp(RecipeCategory::Weapons, 100);
        assert!(leveled);
        assert_eq!(inv.skill_level(RecipeCategory::Weapons), 2);
    }

    #[test]
    fn test_workbench_min_level() {
        assert_eq!(WorkbenchType::BasicWorkbench.min_level(), 1);
        assert!(WorkbenchType::AncientForge.min_level() > WorkbenchType::Forge.min_level());
    }
}
