use crate::{ItemKind, Rarity};
use crate::crafting::{Recipe, CraftingStation, CraftingSkill, Enchantment, FoodBuff};

impl Recipe {
    fn new(
        name: &str,
        desc: &str,
        result: ItemKind,
        count: u32,
        rarity: Rarity,
        ingredients: Vec<(ItemKind, u32)>,
        station: CraftingStation,
        skill: Option<(CraftingSkill, u32)>,
        xp: u32,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: desc.to_string(),
            result,
            result_count: count,
            result_rarity: rarity,
            ingredients,
            station_required: station,
            skill_required: skill,
            xp_reward: xp,
            is_rare_recipe: false,
            enchantment: None,
            food_buff: None,
            crafting_time: 1,
        }
    }

    fn with_enchant(mut self, ench: Enchantment) -> Self {
        self.enchantment = Some(ench);
        self
    }

    fn with_buff(mut self, buff: FoodBuff) -> Self {
        self.food_buff = Some(buff);
        self
    }

    fn rare(mut self) -> Self {
        self.is_rare_recipe = true;
        self
    }

    fn time(mut self, turns: u32) -> Self {
        self.crafting_time = turns;
        self
    }

    // ========================================================================
    // ALL RECIPES
    // ========================================================================
    pub fn all_recipes() -> Vec<Recipe> {
        let mut recipes = Vec::new();

        // Add all recipe categories
        recipes.extend(Self::smithing_recipes());
        recipes.extend(Self::alchemy_recipes());
        recipes.extend(Self::cooking_recipes());
        recipes.extend(Self::enchanting_recipes());
        recipes.extend(Self::tailoring_recipes());
        recipes.extend(Self::jewelcrafting_recipes());
        recipes.extend(Self::runecraft_recipes());
        recipes.extend(Self::rare_recipes());

        recipes
    }

    // ========================================================================
    // SMITHING RECIPES (100+)
    // ========================================================================
    fn smithing_recipes() -> Vec<Recipe> {
        vec![
            // === BASIC WEAPONS (Level 1-10) ===
            Recipe::new("Iron Dagger", "A simple iron dagger", ItemKind::Dagger, 1, Rarity::Common,
                vec![(ItemKind::IronOre, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 1)), 10),
            Recipe::new("Iron Short Sword", "A basic iron blade", ItemKind::ShortSword, 1, Rarity::Common,
                vec![(ItemKind::IronOre, 3)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 2)), 15),
            Recipe::new("Iron Axe", "A sturdy chopping axe", ItemKind::Axe, 1, Rarity::Common,
                vec![(ItemKind::IronOre, 3), (ItemKind::LeatherStrip, 1)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 3)), 18),
            Recipe::new("Iron Mace", "A heavy iron mace", ItemKind::Mace, 1, Rarity::Common,
                vec![(ItemKind::IronOre, 4)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 4)), 20),
            Recipe::new("Iron Spear", "A long iron-tipped spear", ItemKind::Spear, 1, Rarity::Common,
                vec![(ItemKind::IronOre, 2), (ItemKind::LeatherStrip, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 5)), 22),
            Recipe::new("Hunting Bow", "A simple wooden bow", ItemKind::Bow, 1, Rarity::Common,
                vec![(ItemKind::LeatherStrip, 3)], CraftingStation::Workbench,
                Some((CraftingSkill::Blacksmithing, 3)), 15),
            Recipe::new("Light Crossbow", "A compact crossbow", ItemKind::Crossbow, 1, Rarity::Uncommon,
                vec![(ItemKind::IronOre, 2), (ItemKind::LeatherStrip, 2)], CraftingStation::Workbench,
                Some((CraftingSkill::Blacksmithing, 6)), 25),
            Recipe::new("Wooden Staff", "A basic magical focus", ItemKind::Staff, 1, Rarity::Common,
                vec![(ItemKind::LeatherStrip, 2)], CraftingStation::Workbench,
                Some((CraftingSkill::Blacksmithing, 2)), 12),
            Recipe::new("Iron Warhammer", "A crushing iron hammer", ItemKind::WarHammer, 1, Rarity::Common,
                vec![(ItemKind::IronOre, 5), (ItemKind::LeatherStrip, 1)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 7)), 28),
            Recipe::new("Iron Flail", "A chain weapon with iron ball", ItemKind::Flail, 1, Rarity::Uncommon,
                vec![(ItemKind::IronOre, 4), (ItemKind::LeatherStrip, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 8)), 30),

            // === STEEL WEAPONS (Level 10-20) ===
            Recipe::new("Steel Long Sword", "A refined steel blade", ItemKind::LongSword, 1, Rarity::Uncommon,
                vec![(ItemKind::SteelIngot, 3), (ItemKind::LeatherStrip, 1)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 10)), 35),
            Recipe::new("Steel Greatsword", "A massive two-handed blade", ItemKind::Greatsword, 1, Rarity::Uncommon,
                vec![(ItemKind::SteelIngot, 5), (ItemKind::LeatherStrip, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 12)), 45),
            Recipe::new("Steel Battle Axe", "A fearsome war axe", ItemKind::BattleAxe, 1, Rarity::Uncommon,
                vec![(ItemKind::SteelIngot, 4), (ItemKind::LeatherStrip, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 11)), 40),
            Recipe::new("Steel Halberd", "A polearm with axe blade", ItemKind::Halberd, 1, Rarity::Uncommon,
                vec![(ItemKind::SteelIngot, 4), (ItemKind::LeatherStrip, 3)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 13)), 48),
            Recipe::new("Steel Rapier", "An elegant thrusting sword", ItemKind::Rapier, 1, Rarity::Uncommon,
                vec![(ItemKind::SteelIngot, 2), (ItemKind::LeatherStrip, 1)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 14)), 42),
            Recipe::new("Steel Morningstar", "A spiked mace", ItemKind::Morningstar, 1, Rarity::Uncommon,
                vec![(ItemKind::SteelIngot, 4), (ItemKind::LeatherStrip, 1)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 15)), 50),
            Recipe::new("War Trident", "A three-pronged spear", ItemKind::Trident, 1, Rarity::Rare,
                vec![(ItemKind::SteelIngot, 4), (ItemKind::LeatherStrip, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 16)), 55),
            Recipe::new("Reaper's Scythe", "A curved blade of death", ItemKind::Scythe, 1, Rarity::Rare,
                vec![(ItemKind::SteelIngot, 5), (ItemKind::LeatherStrip, 2), (ItemKind::AncientBone, 1)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 18)), 65),
            Recipe::new("Katana", "A curved eastern blade", ItemKind::Katana, 1, Rarity::Rare,
                vec![(ItemKind::SteelIngot, 4), (ItemKind::LeatherStrip, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 20)), 70).time(3),

            // === MITHRIL WEAPONS (Level 20-30) ===
            Recipe::new("Mithril Blade", "A blade of elven silver", ItemKind::LongSword, 1, Rarity::Rare,
                vec![(ItemKind::MithrilOre, 3), (ItemKind::SteelIngot, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 22)), 80),
            Recipe::new("Mithril Greatsword", "A massive mithril blade", ItemKind::Greatsword, 1, Rarity::Rare,
                vec![(ItemKind::MithrilOre, 5), (ItemKind::SteelIngot, 3)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 25)), 100),
            Recipe::new("Mithril Axe", "An axe of shimmering metal", ItemKind::BattleAxe, 1, Rarity::Rare,
                vec![(ItemKind::MithrilOre, 4), (ItemKind::SteelIngot, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 23)), 85),
            Recipe::new("Mithril Spear", "A lightweight spear", ItemKind::Halberd, 1, Rarity::Rare,
                vec![(ItemKind::MithrilOre, 3), (ItemKind::SteelIngot, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 24)), 90),

            // === ELEMENTAL WEAPONS (Level 25-40) ===
            Recipe::new("Flame Sword", "A blade wreathed in fire", ItemKind::FlameSword, 1, Rarity::Epic,
                vec![(ItemKind::SteelIngot, 4), (ItemKind::FireEssence, 3), (ItemKind::ManacrystalII, 1)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 28)), 120).with_enchant(Enchantment::Fire),
            Recipe::new("Frost Blade", "A sword of eternal ice", ItemKind::FrostBlade, 1, Rarity::Epic,
                vec![(ItemKind::SteelIngot, 4), (ItemKind::FrostEssence, 3), (ItemKind::ManacrystalII, 1)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 28)), 120).with_enchant(Enchantment::Frost),
            Recipe::new("Thunder Axe", "An axe crackling with lightning", ItemKind::ThunderAxe, 1, Rarity::Epic,
                vec![(ItemKind::SteelIngot, 5), (ItemKind::ManacrystalII, 2), (ItemKind::GoldBar, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 30)), 130).with_enchant(Enchantment::Lightning),

            // === LEGENDARY WEAPONS (Level 35-50) ===
            Recipe::new("Void Staff", "A staff of dark power", ItemKind::VoidStaff, 1, Rarity::Epic,
                vec![(ItemKind::Staff, 1), (ItemKind::VoidEssence, 3), (ItemKind::ManacrystalIII, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 35)), 150).time(5),
            Recipe::new("Demon Slayer", "Bane of all demons", ItemKind::DemonSlayer, 1, Rarity::Legendary,
                vec![(ItemKind::MithrilOre, 5), (ItemKind::DemonHeart, 1), (ItemKind::ManacrystalIII, 2), (ItemKind::PhoenixFeather, 1)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 40)), 200).with_enchant(Enchantment::Holy).time(8),

            // === BASIC ARMOR (Level 1-10) ===
            Recipe::new("Leather Cap", "A simple leather helmet", ItemKind::LeatherCap, 1, Rarity::Common,
                vec![(ItemKind::LeatherStrip, 2)], CraftingStation::Loom,
                Some((CraftingSkill::Blacksmithing, 1)), 8),
            Recipe::new("Leather Armor", "Basic leather protection", ItemKind::LeatherArmor, 1, Rarity::Common,
                vec![(ItemKind::LeatherStrip, 4)], CraftingStation::Loom,
                Some((CraftingSkill::Blacksmithing, 2)), 12),
            Recipe::new("Leather Gloves", "Simple hand protection", ItemKind::LeatherGloves, 1, Rarity::Common,
                vec![(ItemKind::LeatherStrip, 2)], CraftingStation::Loom,
                Some((CraftingSkill::Blacksmithing, 1)), 8),
            Recipe::new("Leather Boots", "Basic footwear", ItemKind::LeatherBoots, 1, Rarity::Common,
                vec![(ItemKind::LeatherStrip, 2)], CraftingStation::Loom,
                Some((CraftingSkill::Blacksmithing, 1)), 8),
            Recipe::new("Iron Helm", "A protective iron helmet", ItemKind::IronHelm, 1, Rarity::Common,
                vec![(ItemKind::IronOre, 3)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 5)), 18),
            Recipe::new("Chain Mail", "Interlocking iron rings", ItemKind::ChainMail, 1, Rarity::Common,
                vec![(ItemKind::IronOre, 6), (ItemKind::LeatherStrip, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 8)), 28),
            Recipe::new("Iron Gauntlets", "Armored hand protection", ItemKind::IronGauntlets, 1, Rarity::Common,
                vec![(ItemKind::IronOre, 3), (ItemKind::LeatherStrip, 1)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 6)), 20),
            Recipe::new("Iron Boots", "Heavy iron boots", ItemKind::IronBoots, 1, Rarity::Common,
                vec![(ItemKind::IronOre, 3), (ItemKind::LeatherStrip, 1)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 6)), 20),

            // === STEEL ARMOR (Level 10-20) ===
            Recipe::new("Steel Helm", "A sturdy steel helmet", ItemKind::SteelHelm, 1, Rarity::Uncommon,
                vec![(ItemKind::SteelIngot, 3)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 12)), 40),
            Recipe::new("Scale Mail", "Overlapping metal scales", ItemKind::ScaleMail, 1, Rarity::Uncommon,
                vec![(ItemKind::SteelIngot, 5), (ItemKind::LeatherStrip, 3)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 14)), 50),
            Recipe::new("Plate Mail", "Full plate armor", ItemKind::PlateMail, 1, Rarity::Uncommon,
                vec![(ItemKind::SteelIngot, 8), (ItemKind::LeatherStrip, 4)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 18)), 70).time(4),

            // === SHIELDS (Level 1-30) ===
            Recipe::new("Buckler", "A small round shield", ItemKind::Buckler, 1, Rarity::Common,
                vec![(ItemKind::IronOre, 2), (ItemKind::LeatherStrip, 1)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 2)), 10),
            Recipe::new("Wooden Shield", "A basic wooden shield", ItemKind::WoodenShield, 1, Rarity::Common,
                vec![(ItemKind::LeatherStrip, 3)], CraftingStation::Workbench,
                Some((CraftingSkill::Blacksmithing, 1)), 8),
            Recipe::new("Iron Shield", "A sturdy iron shield", ItemKind::IronShield, 1, Rarity::Common,
                vec![(ItemKind::IronOre, 4)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 6)), 22),
            Recipe::new("Tower Shield", "A massive defensive shield", ItemKind::TowerShield, 1, Rarity::Uncommon,
                vec![(ItemKind::SteelIngot, 6)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 15)), 55),
            Recipe::new("Spiked Shield", "An offensive shield", ItemKind::SpikedShield, 1, Rarity::Uncommon,
                vec![(ItemKind::SteelIngot, 5), (ItemKind::IronOre, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 16)), 58),

            // === DRAGON EQUIPMENT (Level 35-50) ===
            Recipe::new("Dragon Helm", "Helmet of dragon scales", ItemKind::DragonHelm, 1, Rarity::Epic,
                vec![(ItemKind::DragonScale, 2), (ItemKind::SteelIngot, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 35)), 140),
            Recipe::new("Dragon Armor", "Armor of dragon scales", ItemKind::DragonArmor, 1, Rarity::Epic,
                vec![(ItemKind::DragonScale, 5), (ItemKind::SteelIngot, 4), (ItemKind::LeatherStrip, 3)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 38)), 180).time(6),
            Recipe::new("Dragon Gauntlets", "Gauntlets of dragon scales", ItemKind::DragonGauntlets, 1, Rarity::Epic,
                vec![(ItemKind::DragonScale, 2), (ItemKind::SteelIngot, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 36)), 150),
            Recipe::new("Dragon Shield", "Shield of dragon scales", ItemKind::DragonShield, 1, Rarity::Epic,
                vec![(ItemKind::DragonScale, 3), (ItemKind::SteelIngot, 3)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 37)), 160),

            // === SPECIAL ARMOR (Level 40-60) ===
            Recipe::new("Crystal Armor", "Armor of magical crystal", ItemKind::CrystalArmor, 1, Rarity::Legendary,
                vec![(ItemKind::ManacrystalIII, 5), (ItemKind::MithrilOre, 3), (ItemKind::EnchantedGem, 3)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 45)), 220).time(8),
            Recipe::new("Titan Plate", "Armor of the titans", ItemKind::TitanPlate, 1, Rarity::Legendary,
                vec![(ItemKind::MithrilOre, 6), (ItemKind::SteelIngot, 6), (ItemKind::DragonScale, 2), (ItemKind::AncientRelic, 1)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 50)), 280).time(10),

            // === SMELTING RECIPES ===
            Recipe::new("Steel Ingot", "Refined steel", ItemKind::SteelIngot, 2, Rarity::Common,
                vec![(ItemKind::IronOre, 3)], CraftingStation::Forge,
                Some((CraftingSkill::Blacksmithing, 5)), 15),
            Recipe::new("Gold Bar", "Pure gold", ItemKind::GoldBar, 1, Rarity::Uncommon,
                vec![(ItemKind::IronOre, 5)], CraftingStation::Forge,
                Some((CraftingSkill::Blacksmithing, 10)), 25),
            Recipe::new("Silver Bar", "Pure silver", ItemKind::SilverBar, 1, Rarity::Uncommon,
                vec![(ItemKind::IronOre, 4)], CraftingStation::Forge,
                Some((CraftingSkill::Blacksmithing, 8)), 20),
            Recipe::new("Mithril Ingot", "Refined mithril", ItemKind::MithrilOre, 1, Rarity::Rare,
                vec![(ItemKind::MithrilOre, 2), (ItemKind::SteelIngot, 1)], CraftingStation::Forge,
                Some((CraftingSkill::Blacksmithing, 20)), 50),

            // === SPECIAL SHIELDS (Level 45-60) ===
            Recipe::new("Magic Shield", "An enchanted shield", ItemKind::MagicShield, 1, Rarity::Rare,
                vec![(ItemKind::IronShield, 1), (ItemKind::ManacrystalII, 2), (ItemKind::EnchantedGem, 1)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Blacksmithing, 25)), 100),
            Recipe::new("Mirror Shield", "Reflects magic", ItemKind::MirrorShield, 1, Rarity::Epic,
                vec![(ItemKind::SteelIngot, 4), (ItemKind::ManacrystalIII, 2), (ItemKind::SilverBar, 3)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 42)), 180),
            Recipe::new("Phoenix Shield", "Burns attackers", ItemKind::PhoenixShield, 1, Rarity::Legendary,
                vec![(ItemKind::DragonShield, 1), (ItemKind::PhoenixFeather, 2), (ItemKind::FireEssence, 5)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 48)), 250).time(6),
            Recipe::new("Abyssal Shield", "Shield from the void", ItemKind::AbyssalShield, 1, Rarity::Mythic,
                vec![(ItemKind::TowerShield, 1), (ItemKind::VoidEssence, 5), (ItemKind::DemonHeart, 1), (ItemKind::CursedFabric, 3)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 55)), 320).time(8),

            // === SPECIAL BOOTS (Level 30-50) ===
            Recipe::new("Boots of Speed", "Swift as the wind", ItemKind::BootsOfSpeed, 1, Rarity::Rare,
                vec![(ItemKind::LeatherBoots, 1), (ItemKind::ManacrystalII, 2), (ItemKind::SilverBar, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 30)), 110).with_enchant(Enchantment::Swiftness),
            Recipe::new("Boots of Leaping", "Jump incredible heights", ItemKind::BootsOfLeaping, 1, Rarity::Rare,
                vec![(ItemKind::LeatherBoots, 1), (ItemKind::ManacrystalII, 1), (ItemKind::LeatherStrip, 5)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 32)), 115),
            Recipe::new("Winged Boots", "Grants flight", ItemKind::WingedBoots, 1, Rarity::Epic,
                vec![(ItemKind::LeatherBoots, 1), (ItemKind::PhoenixFeather, 2), (ItemKind::ManacrystalIII, 1)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 40)), 160),
            Recipe::new("Shadow Boots", "Walk in shadows", ItemKind::ShadowBoots, 1, Rarity::Epic,
                vec![(ItemKind::LeatherBoots, 1), (ItemKind::VoidEssence, 2), (ItemKind::CursedFabric, 2)], CraftingStation::Loom,
                Some((CraftingSkill::Blacksmithing, 38)), 145).with_enchant(Enchantment::Stealth),
            Recipe::new("Lava Walkers", "Walk on fire", ItemKind::LavaWalkers, 1, Rarity::Epic,
                vec![(ItemKind::IronBoots, 1), (ItemKind::FireEssence, 4), (ItemKind::DragonScale, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 42)), 170).with_enchant(Enchantment::Resistance),
            Recipe::new("Boots of the Wind", "Control the wind", ItemKind::BootsOfTheWind, 1, Rarity::Legendary,
                vec![(ItemKind::WingedBoots, 1), (ItemKind::ManacrystalIII, 3), (ItemKind::PhoenixFeather, 1)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 50)), 240).time(5),

            // === SPECIAL GLOVES (Level 30-50) ===
            Recipe::new("Gloves of Power", "Grants incredible strength", ItemKind::GlovesOfPower, 1, Rarity::Rare,
                vec![(ItemKind::IronGauntlets, 1), (ItemKind::ManacrystalII, 2), (ItemKind::SteelIngot, 3)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 30)), 110),
            Recipe::new("Thieves Gloves", "For nimble fingers", ItemKind::ThievesGloves, 1, Rarity::Rare,
                vec![(ItemKind::LeatherGloves, 1), (ItemKind::SilverBar, 2), (ItemKind::VoidEssence, 1)], CraftingStation::Loom,
                Some((CraftingSkill::Blacksmithing, 28)), 100),
            Recipe::new("Frost Gauntlets", "Freeze on touch", ItemKind::FrostGauntlets, 1, Rarity::Epic,
                vec![(ItemKind::IronGauntlets, 1), (ItemKind::FrostEssence, 4), (ItemKind::ManacrystalII, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 35)), 140).with_enchant(Enchantment::Frost),
            Recipe::new("Flame Gauntlets", "Burn on touch", ItemKind::FlameGauntlets, 1, Rarity::Epic,
                vec![(ItemKind::IronGauntlets, 1), (ItemKind::FireEssence, 4), (ItemKind::ManacrystalII, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 35)), 140).with_enchant(Enchantment::Fire),
            Recipe::new("Gauntlets of Might", "Ultimate power", ItemKind::GauntletsOfMight, 1, Rarity::Legendary,
                vec![(ItemKind::DragonGauntlets, 1), (ItemKind::ManacrystalIII, 3), (ItemKind::DragonBlood, 1)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 48)), 230).time(5),

            // === SPECIAL HELMETS (Level 35-60) ===
            Recipe::new("Wizard Hat", "Amplifies magic", ItemKind::WizardHat, 1, Rarity::Rare,
                vec![(ItemKind::CursedFabric, 3), (ItemKind::ManacrystalII, 2)], CraftingStation::Loom,
                Some((CraftingSkill::Blacksmithing, 25)), 90).with_enchant(Enchantment::Wisdom),
            Recipe::new("Crown of Kings", "Symbol of royalty", ItemKind::CrownOfKings, 1, Rarity::Epic,
                vec![(ItemKind::GoldBar, 5), (ItemKind::EnchantedGem, 3), (ItemKind::ManacrystalIII, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 40)), 180),
            Recipe::new("Demon Skull", "Helm of a demon", ItemKind::DemonSkull, 1, Rarity::Epic,
                vec![(ItemKind::AncientBone, 3), (ItemKind::DemonHeart, 1), (ItemKind::CursedFabric, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 42)), 190),
            Recipe::new("Crystal Crown", "Crown of pure crystal", ItemKind::CrystalCrown, 1, Rarity::Legendary,
                vec![(ItemKind::ManacrystalIII, 4), (ItemKind::EnchantedGem, 4), (ItemKind::SilverBar, 3)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 50)), 260).time(6),
            Recipe::new("Hood of Shadows", "Become one with darkness", ItemKind::HoodOfShadows, 1, Rarity::Epic,
                vec![(ItemKind::CursedFabric, 4), (ItemKind::VoidEssence, 3)], CraftingStation::Loom,
                Some((CraftingSkill::Blacksmithing, 38)), 155).with_enchant(Enchantment::Stealth),
            Recipe::new("Helm of Valor", "Inspires courage", ItemKind::HelmOfValor, 1, Rarity::Legendary,
                vec![(ItemKind::MithrilOre, 4), (ItemKind::PhoenixFeather, 1), (ItemKind::GoldBar, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 52)), 270).with_enchant(Enchantment::Bravery).time(5),

            // === SPECIAL ARMOR SETS (Level 40-60) ===
            Recipe::new("Mage Robes", "Robes of power", ItemKind::MageRobes, 1, Rarity::Rare,
                vec![(ItemKind::CursedFabric, 5), (ItemKind::ManacrystalII, 3)], CraftingStation::Loom,
                Some((CraftingSkill::Blacksmithing, 28)), 95).with_enchant(Enchantment::Wisdom),
            Recipe::new("Assassin Garb", "Silent killer attire", ItemKind::AssassinGarb, 1, Rarity::Rare,
                vec![(ItemKind::LeatherStrip, 6), (ItemKind::CursedFabric, 2), (ItemKind::VoidEssence, 1)], CraftingStation::Loom,
                Some((CraftingSkill::Blacksmithing, 30)), 105).with_enchant(Enchantment::Stealth),
            Recipe::new("Holy Armor", "Blessed by the gods", ItemKind::HolyArmor, 1, Rarity::Epic,
                vec![(ItemKind::MithrilOre, 4), (ItemKind::PhoenixFeather, 1), (ItemKind::ManacrystalIII, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 45)), 200).with_enchant(Enchantment::Blessed).time(6),
            Recipe::new("Demon Armor", "Forged in hellfire", ItemKind::DemonArmor, 1, Rarity::Epic,
                vec![(ItemKind::SteelIngot, 6), (ItemKind::DemonHeart, 2), (ItemKind::FireEssence, 4)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 46)), 210).time(6),
            Recipe::new("Shadow Cloak", "Wrap yourself in darkness", ItemKind::ShadowCloak, 1, Rarity::Epic,
                vec![(ItemKind::CursedFabric, 6), (ItemKind::VoidEssence, 4)], CraftingStation::Loom,
                Some((CraftingSkill::Blacksmithing, 44)), 195).with_enchant(Enchantment::Evasion),
        ]
    }

    // ========================================================================
    // ALCHEMY RECIPES (100+)
    // ========================================================================
    fn alchemy_recipes() -> Vec<Recipe> {
        vec![
            // === BASIC POTIONS (Level 1-10) ===
            Recipe::new("Minor Health Potion", "Restores 25 HP", ItemKind::HealthPotion, 1, Rarity::Common,
                vec![(ItemKind::RedHerb, 2), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 1)), 10),
            Recipe::new("Minor Mana Potion", "Restores 20 mana", ItemKind::ManaPotion, 1, Rarity::Common,
                vec![(ItemKind::MoonFlower, 2), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 2)), 12),
            Recipe::new("Antidote", "Cures poison", ItemKind::CureAllPotion, 1, Rarity::Common,
                vec![(ItemKind::RedHerb, 1), (ItemKind::MoonFlower, 1), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 3)), 15),
            Recipe::new("Empty Vial", "A glass container", ItemKind::EmptyVial, 3, Rarity::Common,
                vec![(ItemKind::IronOre, 1)], CraftingStation::Forge,
                Some((CraftingSkill::Alchemy, 1)), 5),

            // === STANDARD POTIONS (Level 5-15) ===
            Recipe::new("Health Potion", "Restores 50 HP", ItemKind::HealthPotion, 1, Rarity::Common,
                vec![(ItemKind::RedHerb, 3), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 5)), 18),
            Recipe::new("Mana Potion", "Restores 40 mana", ItemKind::ManaPotion, 1, Rarity::Common,
                vec![(ItemKind::MoonFlower, 3), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 6)), 20),
            Recipe::new("Strength Potion", "Increases attack temporarily", ItemKind::StrengthPotion, 1, Rarity::Uncommon,
                vec![(ItemKind::RedHerb, 2), (ItemKind::MoonFlower, 1), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 8)), 28),
            Recipe::new("Defense Potion", "Increases defense temporarily", ItemKind::DefensePotion, 1, Rarity::Uncommon,
                vec![(ItemKind::RedHerb, 1), (ItemKind::MoonFlower, 2), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 8)), 28),
            Recipe::new("Speed Potion", "Increases speed temporarily", ItemKind::SpeedPotion, 1, Rarity::Uncommon,
                vec![(ItemKind::MoonFlower, 3), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 10)), 32),
            Recipe::new("Regeneration Potion", "Slowly restores HP", ItemKind::RegenerationPotion, 1, Rarity::Uncommon,
                vec![(ItemKind::RedHerb, 4), (ItemKind::MoonFlower, 1), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 12)), 38),

            // === RESISTANCE POTIONS (Level 10-20) ===
            Recipe::new("Fire Resistance Potion", "Resist fire damage", ItemKind::FireResistPotion, 1, Rarity::Uncommon,
                vec![(ItemKind::FireEssence, 2), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 10)), 30),
            Recipe::new("Ice Resistance Potion", "Resist ice damage", ItemKind::IceResistPotion, 1, Rarity::Uncommon,
                vec![(ItemKind::FrostEssence, 2), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 10)), 30),
            Recipe::new("Poison Resistance Potion", "Resist poison", ItemKind::PoisonResistPotion, 1, Rarity::Uncommon,
                vec![(ItemKind::RedHerb, 3), (ItemKind::MoonFlower, 2), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 12)), 35),

            // === ADVANCED POTIONS (Level 15-30) ===
            Recipe::new("Greater Health Potion", "Restores 100 HP", ItemKind::HealthPotion, 1, Rarity::Rare,
                vec![(ItemKind::RedHerb, 5), (ItemKind::MoonFlower, 2), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 15)), 45),
            Recipe::new("Greater Mana Potion", "Restores 80 mana", ItemKind::ManaPotion, 1, Rarity::Rare,
                vec![(ItemKind::MoonFlower, 5), (ItemKind::RedHerb, 2), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 16)), 48),
            Recipe::new("Invisibility Potion", "Become invisible", ItemKind::InvisibilityPotion, 1, Rarity::Rare,
                vec![(ItemKind::MoonFlower, 3), (ItemKind::VoidEssence, 1), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 18)), 55),
            Recipe::new("Berserk Potion", "Enter a rage state", ItemKind::BerserkPotion, 1, Rarity::Rare,
                vec![(ItemKind::RedHerb, 4), (ItemKind::FireEssence, 2), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 20)), 60),
            Recipe::new("Giant Potion", "Grow in size and power", ItemKind::GiantPotion, 1, Rarity::Rare,
                vec![(ItemKind::RedHerb, 5), (ItemKind::MoonFlower, 3), (ItemKind::DragonBlood, 1), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 22)), 70),
            Recipe::new("Levitation Potion", "Float in the air", ItemKind::LevitationPotion, 1, Rarity::Rare,
                vec![(ItemKind::MoonFlower, 4), (ItemKind::PhoenixFeather, 1), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 24)), 75),
            Recipe::new("Experience Potion", "Double XP gain", ItemKind::XPPotion, 1, Rarity::Rare,
                vec![(ItemKind::MoonFlower, 5), (ItemKind::EnchantedGem, 1), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 25)), 80),
            Recipe::new("Vision Potion", "See in darkness", ItemKind::VisionPotion, 1, Rarity::Uncommon,
                vec![(ItemKind::MoonFlower, 3), (ItemKind::FireEssence, 1), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 14)), 40),

            // === SUPERIOR POTIONS (Level 25-40) ===
            Recipe::new("Full Restore Potion", "Fully restores HP and Mana", ItemKind::FullRestorePotion, 1, Rarity::Epic,
                vec![(ItemKind::RedHerb, 5), (ItemKind::MoonFlower, 5), (ItemKind::PhoenixFeather, 1), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 28)), 100),
            Recipe::new("Luck Potion", "Increases luck", ItemKind::LuckPotion, 1, Rarity::Rare,
                vec![(ItemKind::MoonFlower, 4), (ItemKind::GoldBar, 1), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 26)), 85),
            Recipe::new("Critical Potion", "Increases crit chance", ItemKind::CriticalPotion, 1, Rarity::Rare,
                vec![(ItemKind::RedHerb, 4), (ItemKind::DragonBlood, 1), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 27)), 88),
            Recipe::new("Cure All Potion", "Removes all debuffs", ItemKind::CureAllPotion, 1, Rarity::Epic,
                vec![(ItemKind::RedHerb, 3), (ItemKind::MoonFlower, 3), (ItemKind::UnicornHorn, 1), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 30)), 110),

            // === LEGENDARY POTIONS (Level 35-60) ===
            Recipe::new("Ultimate Power Potion", "Massively boosts all stats", ItemKind::UltimatePowerPotion, 1, Rarity::Legendary,
                vec![(ItemKind::RedHerb, 5), (ItemKind::MoonFlower, 5), (ItemKind::DragonBlood, 2), (ItemKind::PhoenixFeather, 1), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 40)), 180).time(5),
            Recipe::new("Elixir of Life", "Revive from death once", ItemKind::ElixirOfLife, 1, Rarity::Mythic,
                vec![(ItemKind::PhoenixFeather, 2), (ItemKind::UnicornHorn, 1), (ItemKind::DragonBlood, 1), (ItemKind::FullRestorePotion, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 50)), 300).time(10),

            // === ELEMENTAL ESSENCES (Level 20-35) ===
            Recipe::new("Fire Essence", "Concentrated flame", ItemKind::FireEssence, 1, Rarity::Uncommon,
                vec![(ItemKind::RedHerb, 5), (ItemKind::ManacrystalI, 2)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 20)), 55),
            Recipe::new("Frost Essence", "Concentrated cold", ItemKind::FrostEssence, 1, Rarity::Uncommon,
                vec![(ItemKind::MoonFlower, 5), (ItemKind::ManacrystalI, 2)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 20)), 55),
            Recipe::new("Void Essence", "Concentrated void", ItemKind::VoidEssence, 1, Rarity::Rare,
                vec![(ItemKind::MoonFlower, 5), (ItemKind::ManacrystalII, 2), (ItemKind::AncientBone, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 28)), 85),

            // === MANA CRYSTALS (Level 15-35) ===
            Recipe::new("Mana Crystal I", "Basic mana storage", ItemKind::ManacrystalI, 1, Rarity::Common,
                vec![(ItemKind::MoonFlower, 5), (ItemKind::EnchantedGem, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 15)), 40),
            Recipe::new("Mana Crystal II", "Improved mana storage", ItemKind::ManacrystalII, 1, Rarity::Uncommon,
                vec![(ItemKind::ManacrystalI, 3), (ItemKind::EnchantedGem, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 25)), 65),
            Recipe::new("Mana Crystal III", "Superior mana storage", ItemKind::ManacrystalIII, 1, Rarity::Rare,
                vec![(ItemKind::ManacrystalII, 3), (ItemKind::EnchantedGem, 2)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 35)), 100),

            // === POISONS AND BOMBS (Level 10-40) ===
            Recipe::new("Poison Vial", "Coat weapons with poison", ItemKind::HealthPotion, 1, Rarity::Uncommon,
                vec![(ItemKind::RedHerb, 2), (ItemKind::AncientBone, 1), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 10)), 30),
            Recipe::new("Fire Bomb", "Explodes in flames", ItemKind::Bomb, 3, Rarity::Uncommon,
                vec![(ItemKind::FireEssence, 1), (ItemKind::IronOre, 2)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 12)), 35),
            Recipe::new("Frost Bomb", "Freezing explosion", ItemKind::Bomb, 3, Rarity::Uncommon,
                vec![(ItemKind::FrostEssence, 1), (ItemKind::IronOre, 2)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 12)), 35),
            Recipe::new("Void Bomb", "Tears reality", ItemKind::Bomb, 2, Rarity::Rare,
                vec![(ItemKind::VoidEssence, 1), (ItemKind::IronOre, 3)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 25)), 70),

            // === SPECIAL ALCHEMICAL ITEMS (Level 30-50) ===
            Recipe::new("Dragon Blood Extract", "Purified dragon blood", ItemKind::DragonBlood, 1, Rarity::Rare,
                vec![(ItemKind::DragonScale, 2), (ItemKind::RedHerb, 5), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 30)), 95),
            Recipe::new("Soul Gem", "Contains captured souls", ItemKind::SoulGem, 1, Rarity::Epic,
                vec![(ItemKind::EnchantedGem, 2), (ItemKind::VoidEssence, 2), (ItemKind::AncientBone, 3)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 35)), 130),
            Recipe::new("Teleport Crystal", "One-time teleport", ItemKind::TeleportCrystal, 1, Rarity::Rare,
                vec![(ItemKind::ManacrystalII, 2), (ItemKind::VoidEssence, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 28)), 85),

            // === ADDITIONAL POTIONS (Various levels) ===
            Recipe::new("Potion of Stone Skin", "Hardens your skin", ItemKind::DefensePotion, 1, Rarity::Rare,
                vec![(ItemKind::IronOre, 3), (ItemKind::RedHerb, 3), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 22)), 65),
            Recipe::new("Potion of Eagle Eye", "Enhanced vision", ItemKind::VisionPotion, 1, Rarity::Uncommon,
                vec![(ItemKind::MoonFlower, 4), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 16)), 48),
            Recipe::new("Potion of the Bear", "Massive strength boost", ItemKind::StrengthPotion, 1, Rarity::Epic,
                vec![(ItemKind::RedHerb, 6), (ItemKind::DragonBlood, 1), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 32)), 110),
            Recipe::new("Potion of the Hawk", "Extreme speed", ItemKind::SpeedPotion, 1, Rarity::Epic,
                vec![(ItemKind::MoonFlower, 6), (ItemKind::PhoenixFeather, 1), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 34)), 120),
            Recipe::new("Titan's Brew", "Temporary invincibility", ItemKind::DefensePotion, 1, Rarity::Legendary,
                vec![(ItemKind::DragonScale, 2), (ItemKind::DragonBlood, 1), (ItemKind::RedHerb, 5), (ItemKind::EmptyVial, 1)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 45)), 200).time(5),
        ]
    }

    // ========================================================================
    // COOKING RECIPES (50+)
    // ========================================================================
    fn cooking_recipes() -> Vec<Recipe> {
        vec![
            // === BASIC FOOD (Level 1-10) ===
            Recipe::new("Cooked Meat", "Simple roasted meat", ItemKind::Meat, 1, Rarity::Common,
                vec![(ItemKind::RawMeat, 1)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 1)), 5).with_buff(FoodBuff::WellFed),
            Recipe::new("Hearty Bread", "Fresh baked bread", ItemKind::Bread, 1, Rarity::Common,
                vec![(ItemKind::RedHerb, 2)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 2)), 8).with_buff(FoodBuff::WellFed),
            Recipe::new("Fresh Cheese", "Aged to perfection", ItemKind::Cheese, 1, Rarity::Common,
                vec![(ItemKind::RawMeat, 2)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 3)), 10).with_buff(FoodBuff::Nourished),
            Recipe::new("Baked Apple", "A warm treat", ItemKind::Apple, 1, Rarity::Common,
                vec![(ItemKind::Apple, 1), (ItemKind::RedHerb, 1)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 4)), 12).with_buff(FoodBuff::Energized),
            Recipe::new("Meat Stew", "Hearty and filling", ItemKind::Meat, 2, Rarity::Common,
                vec![(ItemKind::RawMeat, 2), (ItemKind::RedHerb, 2)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 5)), 18).with_buff(FoodBuff::WellFed),

            // === INTERMEDIATE FOOD (Level 10-25) ===
            Recipe::new("Warrior's Ration", "Boosts attack", ItemKind::Meat, 1, Rarity::Uncommon,
                vec![(ItemKind::RawMeat, 2), (ItemKind::RedHerb, 3)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 10)), 30).with_buff(FoodBuff::Strengthened),
            Recipe::new("Defender's Meal", "Boosts defense", ItemKind::Meat, 1, Rarity::Uncommon,
                vec![(ItemKind::RawMeat, 2), (ItemKind::MoonFlower, 2)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 10)), 30).with_buff(FoodBuff::Fortified),
            Recipe::new("Swift Bread", "Speed enhancing", ItemKind::Bread, 1, Rarity::Uncommon,
                vec![(ItemKind::RedHerb, 4), (ItemKind::MoonFlower, 2)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 12)), 35).with_buff(FoodBuff::Hastened),
            Recipe::new("Lucky Dumpling", "Increases luck", ItemKind::Bread, 1, Rarity::Uncommon,
                vec![(ItemKind::RedHerb, 3), (ItemKind::GoldBar, 1)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 14)), 40).with_buff(FoodBuff::Lucky),
            Recipe::new("Scholar's Porridge", "Increases XP gain", ItemKind::Bread, 1, Rarity::Uncommon,
                vec![(ItemKind::MoonFlower, 4), (ItemKind::RedHerb, 2)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 15)), 45).with_buff(FoodBuff::Focused),
            Recipe::new("Healing Soup", "Regenerates HP", ItemKind::Meat, 1, Rarity::Uncommon,
                vec![(ItemKind::RawMeat, 2), (ItemKind::RedHerb, 4)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 16)), 48).with_buff(FoodBuff::Nourished),
            Recipe::new("Mana Biscuits", "Regenerates mana", ItemKind::Bread, 1, Rarity::Uncommon,
                vec![(ItemKind::MoonFlower, 5), (ItemKind::RedHerb, 2)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 18)), 52).with_buff(FoodBuff::Enlightened),
            Recipe::new("Dragon Fruit Salad", "Exotic and powerful", ItemKind::DragonFruit, 1, Rarity::Rare,
                vec![(ItemKind::Apple, 3), (ItemKind::DragonScale, 1), (ItemKind::MoonFlower, 2)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 20)), 60).with_buff(FoodBuff::Heroic),

            // === ADVANCED FOOD (Level 25-40) ===
            Recipe::new("Hero's Feast", "Complete meal for warriors", ItemKind::Feast, 1, Rarity::Rare,
                vec![(ItemKind::Meat, 2), (ItemKind::Bread, 2), (ItemKind::Cheese, 1), (ItemKind::Apple, 2)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 25)), 80).with_buff(FoodBuff::Heroic),
            Recipe::new("King's Banquet", "Fit for royalty", ItemKind::Feast, 1, Rarity::Epic,
                vec![(ItemKind::Meat, 3), (ItemKind::Bread, 3), (ItemKind::Cheese, 2), (ItemKind::Apple, 3), (ItemKind::GoldBar, 1)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 30)), 110).with_buff(FoodBuff::Heroic).time(3),
            Recipe::new("Dragon Steak", "Meat from a dragon", ItemKind::Meat, 1, Rarity::Epic,
                vec![(ItemKind::RawMeat, 3), (ItemKind::DragonScale, 2), (ItemKind::FireEssence, 1)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 32)), 120).with_buff(FoodBuff::Strengthened),
            Recipe::new("Phoenix Pie", "Legendary pastry", ItemKind::Feast, 1, Rarity::Epic,
                vec![(ItemKind::PhoenixFeather, 1), (ItemKind::Apple, 5), (ItemKind::RedHerb, 3)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 35)), 140).with_buff(FoodBuff::Survivors_Will),
            Recipe::new("Ancient Wine", "Aged for centuries", ItemKind::AncientWine, 1, Rarity::Rare,
                vec![(ItemKind::Apple, 5), (ItemKind::MoonFlower, 3), (ItemKind::RedHerb, 3)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 28)), 95).with_buff(FoodBuff::Heroic).time(5),

            // === LEGENDARY FOOD (Level 40-60) ===
            Recipe::new("Golden Apple", "A divine fruit", ItemKind::GoldenApple, 1, Rarity::Legendary,
                vec![(ItemKind::Apple, 3), (ItemKind::GoldBar, 5), (ItemKind::MoonFlower, 3), (ItemKind::PhoenixFeather, 1)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 40)), 180).with_buff(FoodBuff::Legendary_Feast).time(5),
            Recipe::new("Ambrosia", "Food of the gods", ItemKind::Feast, 1, Rarity::Mythic,
                vec![(ItemKind::GoldenApple, 1), (ItemKind::PhoenixFeather, 2), (ItemKind::DragonBlood, 1), (ItemKind::UnicornHorn, 1)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 50)), 300).with_buff(FoodBuff::Legendary_Feast).time(10),
            Recipe::new("Survivor's Hardtack", "Never give up", ItemKind::Bread, 2, Rarity::Epic,
                vec![(ItemKind::RedHerb, 5), (ItemKind::MoonFlower, 5), (ItemKind::SteelIngot, 1)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 38)), 150).with_buff(FoodBuff::Survivors_Will),

            // === SPECIALTY DISHES (Various levels) ===
            Recipe::new("Spicy Stew", "Fire resistance food", ItemKind::Meat, 1, Rarity::Uncommon,
                vec![(ItemKind::RawMeat, 2), (ItemKind::FireEssence, 1), (ItemKind::RedHerb, 2)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 15)), 45),
            Recipe::new("Frozen Treat", "Ice resistance food", ItemKind::Apple, 1, Rarity::Uncommon,
                vec![(ItemKind::Apple, 2), (ItemKind::FrostEssence, 1), (ItemKind::MoonFlower, 2)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 15)), 45),
            Recipe::new("Dungeon Rations", "Long lasting food", ItemKind::Bread, 3, Rarity::Common,
                vec![(ItemKind::RawMeat, 2), (ItemKind::RedHerb, 2)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 8)), 25).with_buff(FoodBuff::WellFed),
            Recipe::new("Mage's Delight", "Mana focused meal", ItemKind::Bread, 1, Rarity::Rare,
                vec![(ItemKind::MoonFlower, 6), (ItemKind::ManacrystalI, 1)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 22)), 70).with_buff(FoodBuff::Energized),
            Recipe::new("Berserker's Blood Pudding", "Rage inducing", ItemKind::Meat, 1, Rarity::Rare,
                vec![(ItemKind::RawMeat, 3), (ItemKind::DragonBlood, 1)], CraftingStation::CookingFire,
                Some((CraftingSkill::Cooking, 24)), 75).with_buff(FoodBuff::Strengthened),
        ]
    }

    // ========================================================================
    // ENCHANTING RECIPES (50+)
    // ========================================================================
    fn enchanting_recipes() -> Vec<Recipe> {
        vec![
            // === BASIC SCROLLS (Level 1-15) ===
            Recipe::new("Scroll of Mapping", "Reveals the map", ItemKind::ScrollMapping, 1, Rarity::Common,
                vec![(ItemKind::BlankScroll, 1), (ItemKind::ManacrystalI, 1)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 2)), 15),
            Recipe::new("Scroll of Identify", "Identifies items", ItemKind::ScrollIdentify, 1, Rarity::Common,
                vec![(ItemKind::BlankScroll, 1), (ItemKind::ManacrystalI, 1)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 3)), 18),
            Recipe::new("Scroll of Fireball", "Casts fireball", ItemKind::ScrollFireball, 1, Rarity::Uncommon,
                vec![(ItemKind::BlankScroll, 1), (ItemKind::FireEssence, 2)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 5)), 25),
            Recipe::new("Scroll of Ice Storm", "Freezing attack", ItemKind::ScrollIceStorm, 1, Rarity::Uncommon,
                vec![(ItemKind::BlankScroll, 1), (ItemKind::FrostEssence, 2)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 5)), 25),
            Recipe::new("Scroll of Lightning", "Electric attack", ItemKind::ScrollLightning, 1, Rarity::Uncommon,
                vec![(ItemKind::BlankScroll, 1), (ItemKind::ManacrystalI, 2), (ItemKind::GoldBar, 1)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 6)), 28),
            Recipe::new("Scroll of Teleport", "Short range teleport", ItemKind::ScrollTeleport, 1, Rarity::Rare,
                vec![(ItemKind::BlankScroll, 1), (ItemKind::VoidEssence, 2)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 10)), 40),

            // === INTERMEDIATE SCROLLS (Level 15-30) ===
            Recipe::new("Scroll of Enchant", "Enchant equipment", ItemKind::ScrollEnchant, 1, Rarity::Rare,
                vec![(ItemKind::BlankScroll, 1), (ItemKind::ManacrystalII, 2), (ItemKind::EnchantedGem, 1)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 15)), 55),
            Recipe::new("Scroll of Summon", "Summon ally", ItemKind::ScrollSummon, 1, Rarity::Rare,
                vec![(ItemKind::BlankScroll, 1), (ItemKind::SoulGem, 1), (ItemKind::ManacrystalII, 1)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 18)), 65),
            Recipe::new("Scroll of Banish", "Banish enemies", ItemKind::ScrollBanish, 1, Rarity::Rare,
                vec![(ItemKind::BlankScroll, 1), (ItemKind::VoidEssence, 2), (ItemKind::ManacrystalII, 1)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 20)), 70),
            Recipe::new("Scroll of Mass Heal", "Heal all allies", ItemKind::ScrollMassHeal, 1, Rarity::Rare,
                vec![(ItemKind::BlankScroll, 1), (ItemKind::PhoenixFeather, 1), (ItemKind::ManacrystalII, 2)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 22)), 80),
            Recipe::new("Scroll of Chain Lightning", "Bouncing lightning", ItemKind::ScrollChainLightning, 1, Rarity::Rare,
                vec![(ItemKind::BlankScroll, 1), (ItemKind::ManacrystalII, 3), (ItemKind::GoldBar, 2)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 24)), 85),
            Recipe::new("Scroll of Blizzard", "Massive ice storm", ItemKind::ScrollBlizzard, 1, Rarity::Epic,
                vec![(ItemKind::BlankScroll, 1), (ItemKind::FrostEssence, 4), (ItemKind::ManacrystalII, 2)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 26)), 95),
            Recipe::new("Scroll of Meteor", "Summon meteors", ItemKind::ScrollMeteor, 1, Rarity::Epic,
                vec![(ItemKind::BlankScroll, 1), (ItemKind::FireEssence, 4), (ItemKind::ManacrystalII, 2)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 28)), 100),
            Recipe::new("Scroll of Earthquake", "Shake the ground", ItemKind::ScrollEarthquake, 1, Rarity::Epic,
                vec![(ItemKind::BlankScroll, 1), (ItemKind::IronOre, 5), (ItemKind::ManacrystalII, 2)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 25)), 90),

            // === ADVANCED SCROLLS (Level 30-50) ===
            Recipe::new("Scroll of Time Stop", "Freeze time", ItemKind::ScrollTimeStop, 1, Rarity::Legendary,
                vec![(ItemKind::BlankScroll, 1), (ItemKind::VoidEssence, 3), (ItemKind::ManacrystalIII, 2), (ItemKind::AncientRelic, 1)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 40)), 180).time(5),
            Recipe::new("Scroll of Death", "Instant death chance", ItemKind::ScrollDeath, 1, Rarity::Legendary,
                vec![(ItemKind::BlankScroll, 1), (ItemKind::DemonHeart, 1), (ItemKind::SoulGem, 2), (ItemKind::AncientBone, 3)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 45)), 200).time(5),
            Recipe::new("Scroll of Divine Wrath", "Holy devastation", ItemKind::ScrollDivineWrath, 1, Rarity::Legendary,
                vec![(ItemKind::BlankScroll, 1), (ItemKind::PhoenixFeather, 2), (ItemKind::ManacrystalIII, 3)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 48)), 220).time(5),
            Recipe::new("Scroll of Darkness", "Consume in shadow", ItemKind::ScrollDarkness, 1, Rarity::Legendary,
                vec![(ItemKind::BlankScroll, 1), (ItemKind::VoidEssence, 4), (ItemKind::CursedFabric, 3), (ItemKind::DemonHeart, 1)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 50)), 250).time(6),

            // === WAND CREATION (Level 15-40) ===
            Recipe::new("Wand of Fire", "Shoots fireballs", ItemKind::Wand, 1, Rarity::Uncommon,
                vec![(ItemKind::Staff, 1), (ItemKind::FireEssence, 3), (ItemKind::ManacrystalI, 2)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 15)), 55).with_enchant(Enchantment::Fire),
            Recipe::new("Wand of Frost", "Shoots ice bolts", ItemKind::Wand, 1, Rarity::Uncommon,
                vec![(ItemKind::Staff, 1), (ItemKind::FrostEssence, 3), (ItemKind::ManacrystalI, 2)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 15)), 55).with_enchant(Enchantment::Frost),
            Recipe::new("Wand of Lightning", "Shoots lightning", ItemKind::Wand, 1, Rarity::Rare,
                vec![(ItemKind::Staff, 1), (ItemKind::ManacrystalII, 3), (ItemKind::GoldBar, 2)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 20)), 75).with_enchant(Enchantment::Lightning),
            Recipe::new("Wand of the Void", "Shoots void bolts", ItemKind::Wand, 1, Rarity::Epic,
                vec![(ItemKind::Staff, 1), (ItemKind::VoidEssence, 4), (ItemKind::ManacrystalIII, 2)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 35)), 140).with_enchant(Enchantment::Chaos),

            // === ENCHANTED GEMS (Level 10-35) ===
            Recipe::new("Enchanted Gem", "A magical gemstone", ItemKind::EnchantedGem, 1, Rarity::Uncommon,
                vec![(ItemKind::ManacrystalI, 2), (ItemKind::GoldBar, 1)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 10)), 35),
            Recipe::new("Greater Enchanted Gem", "Powerful gem", ItemKind::EnchantedGem, 2, Rarity::Rare,
                vec![(ItemKind::ManacrystalII, 2), (ItemKind::EnchantedGem, 1)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 25)), 80),
            Recipe::new("Supreme Enchanted Gem", "Ultimate gem", ItemKind::EnchantedGem, 3, Rarity::Epic,
                vec![(ItemKind::ManacrystalIII, 2), (ItemKind::EnchantedGem, 2)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Enchanting, 35)), 130),

            // === SPECIAL ITEMS (Level 25-50) ===
            Recipe::new("Torch", "Illuminates darkness", ItemKind::Torch, 5, Rarity::Common,
                vec![(ItemKind::LeatherStrip, 1), (ItemKind::FireEssence, 1)], CraftingStation::Workbench,
                Some((CraftingSkill::Enchanting, 5)), 15),
            Recipe::new("Compass", "Points to exit", ItemKind::Compass, 1, Rarity::Uncommon,
                vec![(ItemKind::IronOre, 2), (ItemKind::ManacrystalI, 1)], CraftingStation::Workbench,
                Some((CraftingSkill::Enchanting, 8)), 28),
            Recipe::new("Blank Scroll", "For writing spells", ItemKind::BlankScroll, 3, Rarity::Common,
                vec![(ItemKind::LeatherStrip, 2)], CraftingStation::Workbench,
                Some((CraftingSkill::Enchanting, 1)), 8),
        ]
    }

    // ========================================================================
    // TAILORING RECIPES
    // ========================================================================
    fn tailoring_recipes() -> Vec<Recipe> {
        vec![
            // === BASIC LEATHER (Level 1-15) ===
            Recipe::new("Leather Strip", "Basic leather", ItemKind::LeatherStrip, 3, Rarity::Common,
                vec![(ItemKind::RawMeat, 2)], CraftingStation::Loom,
                Some((CraftingSkill::Tailoring, 1)), 5),
            Recipe::new("Leather Armor", "Basic protection", ItemKind::LeatherArmor, 1, Rarity::Common,
                vec![(ItemKind::LeatherStrip, 5)], CraftingStation::Loom,
                Some((CraftingSkill::Tailoring, 5)), 20),
            Recipe::new("Leather Cap", "Head protection", ItemKind::LeatherCap, 1, Rarity::Common,
                vec![(ItemKind::LeatherStrip, 3)], CraftingStation::Loom,
                Some((CraftingSkill::Tailoring, 3)), 12),
            Recipe::new("Leather Gloves", "Hand protection", ItemKind::LeatherGloves, 1, Rarity::Common,
                vec![(ItemKind::LeatherStrip, 2)], CraftingStation::Loom,
                Some((CraftingSkill::Tailoring, 2)), 8),
            Recipe::new("Leather Boots", "Foot protection", ItemKind::LeatherBoots, 1, Rarity::Common,
                vec![(ItemKind::LeatherStrip, 3)], CraftingStation::Loom,
                Some((CraftingSkill::Tailoring, 4)), 15),

            // === ADVANCED CLOTH (Level 15-35) ===
            Recipe::new("Cursed Fabric", "Dark cloth", ItemKind::CursedFabric, 1, Rarity::Uncommon,
                vec![(ItemKind::LeatherStrip, 3), (ItemKind::VoidEssence, 1)], CraftingStation::Loom,
                Some((CraftingSkill::Tailoring, 15)), 50),
            Recipe::new("Mage Robes", "Magical clothing", ItemKind::MageRobes, 1, Rarity::Rare,
                vec![(ItemKind::CursedFabric, 4), (ItemKind::ManacrystalII, 2)], CraftingStation::Loom,
                Some((CraftingSkill::Tailoring, 25)), 90).with_enchant(Enchantment::Wisdom),
            Recipe::new("Assassin Garb", "Stealthy attire", ItemKind::AssassinGarb, 1, Rarity::Rare,
                vec![(ItemKind::LeatherStrip, 5), (ItemKind::CursedFabric, 3), (ItemKind::VoidEssence, 2)], CraftingStation::Loom,
                Some((CraftingSkill::Tailoring, 28)), 100).with_enchant(Enchantment::Stealth),
            Recipe::new("Shadow Cloak", "Embrace darkness", ItemKind::ShadowCloak, 1, Rarity::Epic,
                vec![(ItemKind::CursedFabric, 6), (ItemKind::VoidEssence, 4)], CraftingStation::Loom,
                Some((CraftingSkill::Tailoring, 35)), 150).with_enchant(Enchantment::Evasion),
            Recipe::new("Hood of Shadows", "Hide in darkness", ItemKind::HoodOfShadows, 1, Rarity::Epic,
                vec![(ItemKind::CursedFabric, 4), (ItemKind::VoidEssence, 3)], CraftingStation::Loom,
                Some((CraftingSkill::Tailoring, 32)), 130).with_enchant(Enchantment::Stealth),
        ]
    }

    // ========================================================================
    // JEWELCRAFTING RECIPES
    // ========================================================================
    fn jewelcrafting_recipes() -> Vec<Recipe> {
        vec![
            // === BASIC RINGS (Level 1-20) ===
            Recipe::new("Ring of Strength", "+Attack ring", ItemKind::RingOfStrength, 1, Rarity::Uncommon,
                vec![(ItemKind::SilverBar, 2), (ItemKind::EnchantedGem, 1)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 5)), 30),
            Recipe::new("Ring of Protection", "+Defense ring", ItemKind::RingOfProtection, 1, Rarity::Uncommon,
                vec![(ItemKind::SilverBar, 2), (ItemKind::EnchantedGem, 1)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 5)), 30),
            Recipe::new("Ring of Speed", "+Speed ring", ItemKind::RingOfSpeed, 1, Rarity::Uncommon,
                vec![(ItemKind::SilverBar, 2), (ItemKind::ManacrystalI, 2)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 8)), 35),
            Recipe::new("Ring of Regeneration", "HP regen ring", ItemKind::RingOfRegeneration, 1, Rarity::Rare,
                vec![(ItemKind::GoldBar, 2), (ItemKind::EnchantedGem, 1), (ItemKind::RedHerb, 5)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 12)), 50),
            Recipe::new("Ring of Mana", "+Mana ring", ItemKind::RingOfMana, 1, Rarity::Rare,
                vec![(ItemKind::GoldBar, 2), (ItemKind::ManacrystalII, 2)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 15)), 60),
            Recipe::new("Ring of Luck", "Better drops", ItemKind::RingOfLuck, 1, Rarity::Rare,
                vec![(ItemKind::GoldBar, 3), (ItemKind::EnchantedGem, 2)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 18)), 70),

            // === ADVANCED RINGS (Level 20-40) ===
            Recipe::new("Ring of Fireball", "Cast fireballs", ItemKind::RingOfFireball, 1, Rarity::Rare,
                vec![(ItemKind::GoldBar, 2), (ItemKind::FireEssence, 3), (ItemKind::EnchantedGem, 1)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 20)), 80).with_enchant(Enchantment::Fire),
            Recipe::new("Ring of Invisibility", "Turn invisible", ItemKind::RingOfInvisibility, 1, Rarity::Epic,
                vec![(ItemKind::GoldBar, 3), (ItemKind::VoidEssence, 2), (ItemKind::ManacrystalII, 2)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 25)), 100).with_enchant(Enchantment::Stealth),
            Recipe::new("Ring of the Vampire", "Lifesteal", ItemKind::RingOfTheVampire, 1, Rarity::Epic,
                vec![(ItemKind::GoldBar, 3), (ItemKind::DragonBlood, 1), (ItemKind::EnchantedGem, 2)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 28)), 110).with_enchant(Enchantment::Vampiric),
            Recipe::new("Ring of Frost", "Freeze enemies", ItemKind::RingOfFrost, 1, Rarity::Rare,
                vec![(ItemKind::SilverBar, 3), (ItemKind::FrostEssence, 3), (ItemKind::EnchantedGem, 1)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 22)), 85).with_enchant(Enchantment::Frost),
            Recipe::new("Ring of Flame", "Burn enemies", ItemKind::RingOfFlame, 1, Rarity::Rare,
                vec![(ItemKind::GoldBar, 2), (ItemKind::FireEssence, 3), (ItemKind::EnchantedGem, 1)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 22)), 85).with_enchant(Enchantment::Fire),
            Recipe::new("Ring of Thunder", "Shock enemies", ItemKind::RingOfThunder, 1, Rarity::Rare,
                vec![(ItemKind::GoldBar, 3), (ItemKind::ManacrystalII, 2), (ItemKind::GoldBar, 1)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 24)), 90).with_enchant(Enchantment::Lightning),
            Recipe::new("Ring of Shadows", "Embrace shadows", ItemKind::RingOfShadows, 1, Rarity::Epic,
                vec![(ItemKind::SilverBar, 3), (ItemKind::VoidEssence, 3), (ItemKind::CursedFabric, 2)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 30)), 120).with_enchant(Enchantment::Stealth),
            Recipe::new("Ring of Death", "Instant kill chance", ItemKind::RingOfDeath, 1, Rarity::Legendary,
                vec![(ItemKind::GoldBar, 5), (ItemKind::DemonHeart, 1), (ItemKind::SoulGem, 1), (ItemKind::VoidEssence, 2)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 40)), 180).with_enchant(Enchantment::Oblivion).time(5),
            Recipe::new("Ring of the Ancients", "Ancient power", ItemKind::RingOfTheAncients, 1, Rarity::Legendary,
                vec![(ItemKind::GoldBar, 4), (ItemKind::AncientBone, 3), (ItemKind::ManacrystalIII, 2), (ItemKind::AncientRelic, 1)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 45)), 200).time(5),

            // === AMULETS (Level 10-50) ===
            Recipe::new("Amulet of Health", "+HP amulet", ItemKind::AmuletOfHealth, 1, Rarity::Rare,
                vec![(ItemKind::GoldBar, 3), (ItemKind::EnchantedGem, 2), (ItemKind::RedHerb, 5)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 15)), 60).with_enchant(Enchantment::Vitality),
            Recipe::new("Amulet of Mana", "+Mana amulet", ItemKind::AmuletOfMana, 1, Rarity::Rare,
                vec![(ItemKind::GoldBar, 3), (ItemKind::ManacrystalII, 2), (ItemKind::MoonFlower, 5)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 18)), 70).with_enchant(Enchantment::Wisdom),
            Recipe::new("Amulet of Protection", "+Defense amulet", ItemKind::AmuletOfProtection, 1, Rarity::Rare,
                vec![(ItemKind::GoldBar, 3), (ItemKind::SteelIngot, 2), (ItemKind::EnchantedGem, 2)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 20)), 75).with_enchant(Enchantment::Protection),
            Recipe::new("Amulet of Power", "+Attack amulet", ItemKind::AmuletOfPower, 1, Rarity::Epic,
                vec![(ItemKind::GoldBar, 4), (ItemKind::DragonBlood, 1), (ItemKind::EnchantedGem, 2)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 25)), 100).with_enchant(Enchantment::Sharpness),
            Recipe::new("Amulet of Wisdom", "+XP amulet", ItemKind::AmuletOfWisdom, 1, Rarity::Epic,
                vec![(ItemKind::GoldBar, 4), (ItemKind::ManacrystalIII, 2), (ItemKind::EnchantedGem, 2)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 28)), 110).with_enchant(Enchantment::Experience),
            Recipe::new("Amulet of Life", "Revive once", ItemKind::AmuletOfLife, 1, Rarity::Legendary,
                vec![(ItemKind::GoldBar, 5), (ItemKind::PhoenixFeather, 2), (ItemKind::ManacrystalIII, 2)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 35)), 160).with_enchant(Enchantment::Resurrection).time(5),
            Recipe::new("Amulet of Death", "Death aura", ItemKind::AmuletOfDeath, 1, Rarity::Legendary,
                vec![(ItemKind::GoldBar, 5), (ItemKind::DemonHeart, 2), (ItemKind::SoulGem, 2), (ItemKind::AncientBone, 3)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 40)), 180).time(5),
            Recipe::new("Amulet of Dragons", "Dragon power", ItemKind::AmuletOfDragons, 1, Rarity::Legendary,
                vec![(ItemKind::GoldBar, 5), (ItemKind::DragonScale, 3), (ItemKind::DragonBlood, 2), (ItemKind::ManacrystalIII, 2)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 42)), 190).time(5),
            Recipe::new("Amulet of Chaos", "Random effects", ItemKind::AmuletOfChaos, 1, Rarity::Legendary,
                vec![(ItemKind::GoldBar, 5), (ItemKind::VoidEssence, 4), (ItemKind::DemonHeart, 1), (ItemKind::PhoenixFeather, 1)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 45)), 200).with_enchant(Enchantment::Chaos).time(5),
            Recipe::new("Amulet of Order", "Stability", ItemKind::AmuletOfOrder, 1, Rarity::Legendary,
                vec![(ItemKind::GoldBar, 5), (ItemKind::ManacrystalIII, 4), (ItemKind::EnchantedGem, 4)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 45)), 200).with_enchant(Enchantment::Fortification).time(5),
            Recipe::new("Amulet of Balance", "Perfect harmony", ItemKind::AmuletOfBalance, 1, Rarity::Legendary,
                vec![(ItemKind::AmuletOfChaos, 1), (ItemKind::AmuletOfOrder, 1)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 50)), 300).time(8),
            Recipe::new("Amulet of the Gods", "Divine power", ItemKind::AmuletOfTheGods, 1, Rarity::Mythic,
                vec![(ItemKind::GoldBar, 8), (ItemKind::ManacrystalIII, 5), (ItemKind::PhoenixFeather, 2), (ItemKind::DragonScale, 3), (ItemKind::UnicornHorn, 1)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 60)), 400).with_enchant(Enchantment::Blessed).time(10),
        ]
    }

    // ========================================================================
    // RUNECRAFT RECIPES
    // ========================================================================
    fn runecraft_recipes() -> Vec<Recipe> {
        vec![
            Recipe::new("Rune Stone", "Basic rune material", ItemKind::RuneStone, 1, Rarity::Common,
                vec![(ItemKind::IronOre, 2), (ItemKind::ManacrystalI, 1)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Runecraft, 1)), 10),
            Recipe::new("Fire Rune", "Imbue with fire", ItemKind::FireEssence, 1, Rarity::Uncommon,
                vec![(ItemKind::RuneStone, 1), (ItemKind::FireEssence, 2)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Runecraft, 10)), 35),
            Recipe::new("Frost Rune", "Imbue with frost", ItemKind::FrostEssence, 1, Rarity::Uncommon,
                vec![(ItemKind::RuneStone, 1), (ItemKind::FrostEssence, 2)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Runecraft, 10)), 35),
            Recipe::new("Void Rune", "Imbue with void", ItemKind::VoidEssence, 1, Rarity::Rare,
                vec![(ItemKind::RuneStone, 1), (ItemKind::VoidEssence, 2)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Runecraft, 20)), 65),
            Recipe::new("Ancient Relic", "Mysterious power", ItemKind::AncientRelic, 1, Rarity::Epic,
                vec![(ItemKind::RuneStone, 3), (ItemKind::AncientBone, 2), (ItemKind::ManacrystalII, 2)], CraftingStation::EnchantingAltar,
                Some((CraftingSkill::Runecraft, 30)), 120),
        ]
    }

    // ========================================================================
    // RARE RECIPES (Found as loot)
    // ========================================================================
    fn rare_recipes() -> Vec<Recipe> {
        vec![
            // === LEGENDARY WEAPONS ===
            Recipe::new("Godslayer Blade", "Bane of the divine", ItemKind::DemonSlayer, 1, Rarity::Mythic,
                vec![(ItemKind::MithrilOre, 8), (ItemKind::DragonBlood, 3), (ItemKind::PhoenixFeather, 2), (ItemKind::DemonHeart, 2), (ItemKind::AncientRelic, 1)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 60)), 500).with_enchant(Enchantment::Godslayer).rare().time(15),

            // === LEGENDARY ARMOR ===
            Recipe::new("Armor of Eternity", "Timeless protection", ItemKind::TitanPlate, 1, Rarity::Mythic,
                vec![(ItemKind::MithrilOre, 10), (ItemKind::DragonScale, 5), (ItemKind::PhoenixFeather, 2), (ItemKind::ManacrystalIII, 5)], CraftingStation::Anvil,
                Some((CraftingSkill::Blacksmithing, 65)), 600).with_enchant(Enchantment::Indestructible).rare().time(20),

            // === LEGENDARY ACCESSORIES ===
            Recipe::new("Crown of the Universe", "Cosmic power", ItemKind::CrystalCrown, 1, Rarity::Mythic,
                vec![(ItemKind::GoldBar, 10), (ItemKind::ManacrystalIII, 8), (ItemKind::PhoenixFeather, 3), (ItemKind::UnicornHorn, 2)], CraftingStation::Anvil,
                Some((CraftingSkill::Jewelcrafting, 70)), 700).with_enchant(Enchantment::Omniscience).rare().time(25),

            // === LEGENDARY POTIONS ===
            Recipe::new("Potion of Immortality", "Cheat death forever", ItemKind::ElixirOfLife, 1, Rarity::Mythic,
                vec![(ItemKind::ElixirOfLife, 3), (ItemKind::PhoenixFeather, 5), (ItemKind::UnicornHorn, 3), (ItemKind::DragonBlood, 3)], CraftingStation::AlchemyTable,
                Some((CraftingSkill::Alchemy, 70)), 800).rare().time(30),

            // === SECRET RECIPES ===
            Recipe::new("Timewarp Ring", "Bend time itself", ItemKind::RingOfTheAncients, 1, Rarity::Mythic,
                vec![(ItemKind::GoldBar, 8), (ItemKind::VoidEssence, 6), (ItemKind::ManacrystalIII, 5), (ItemKind::AncientRelic, 2)], CraftingStation::Workbench,
                Some((CraftingSkill::Jewelcrafting, 75)), 900).with_enchant(Enchantment::Timewarp).rare().time(20),
        ]
    }
}
