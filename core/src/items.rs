//! Item system: items, equipment, and inventory

use serde::{Serialize, Deserialize};

/// Equipment slots
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum EquipSlot {
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

/// Item rarity tiers
#[derive(Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

/// Food quality levels - affects hunger restoration and bonuses
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub enum FoodQuality {
    Rotten,      // Spoiled food - might cause sickness
    Raw,         // Uncooked - less effective
    Stale,       // Old food - reduced effectiveness
    Fresh,       // Normal quality
    Cooked,      // Cooked food - bonus effectiveness
    WellCooked,  // Skillfully prepared
    Gourmet,     // Master chef quality
    Legendary,   // Magical/divine food
}

impl FoodQuality {
    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            FoodQuality::Rotten => "Rotten",
            FoodQuality::Raw => "Raw",
            FoodQuality::Stale => "Stale",
            FoodQuality::Fresh => "Fresh",
            FoodQuality::Cooked => "Cooked",
            FoodQuality::WellCooked => "Well-Cooked",
            FoodQuality::Gourmet => "Gourmet",
            FoodQuality::Legendary => "Legendary",
        }
    }

    /// Get color index for display
    pub fn color_index(&self) -> u8 {
        match self {
            FoodQuality::Rotten => 4,       // Dark/brown
            FoodQuality::Raw => 3,          // Red
            FoodQuality::Stale => 6,        // Orange
            FoodQuality::Fresh => 1,        // White
            FoodQuality::Cooked => 5,       // Green
            FoodQuality::WellCooked => 13,  // Bright green
            FoodQuality::Gourmet => 11,     // Yellow/gold
            FoodQuality::Legendary => 7,    // Cyan/magic
        }
    }

    /// Get the hunger value multiplier for this quality
    pub fn hunger_multiplier(&self) -> f32 {
        match self {
            FoodQuality::Rotten => 0.3,
            FoodQuality::Raw => 0.6,
            FoodQuality::Stale => 0.8,
            FoodQuality::Fresh => 1.0,
            FoodQuality::Cooked => 1.3,
            FoodQuality::WellCooked => 1.5,
            FoodQuality::Gourmet => 2.0,
            FoodQuality::Legendary => 3.0,
        }
    }

    /// Check if food can be cooked
    pub fn can_cook(&self) -> bool {
        matches!(self, FoodQuality::Raw | FoodQuality::Fresh)
    }

    /// Check if food can spoil/decay
    pub fn can_spoil(&self) -> bool {
        matches!(self, FoodQuality::Fresh | FoodQuality::Cooked | FoodQuality::WellCooked)
    }

    /// Get next quality level when food spoils
    pub fn spoiled(&self) -> FoodQuality {
        match self {
            FoodQuality::Fresh => FoodQuality::Stale,
            FoodQuality::Cooked => FoodQuality::Stale,
            FoodQuality::WellCooked => FoodQuality::Cooked,
            FoodQuality::Gourmet => FoodQuality::WellCooked,
            FoodQuality::Stale => FoodQuality::Rotten,
            _ => *self,
        }
    }
}

impl Rarity {
    /// Returns a color index for the rarity (for UI rendering)
    pub fn color_index(&self) -> u8 {
        match self {
            Self::Common => 1,      // Grey
            Self::Uncommon => 5,    // Green
            Self::Rare => 7,        // Blue
            Self::Epic => 13,       // Magenta
            Self::Legendary => 11,  // Yellow
            Self::Mythic => 3,      // Red
        }
    }

    /// Returns the name prefix for this rarity
    pub fn prefix(&self) -> &'static str {
        match self {
            Self::Common => "",
            Self::Uncommon => "Fine ",
            Self::Rare => "Superior ",
            Self::Epic => "Epic ",
            Self::Legendary => "Legendary ",
            Self::Mythic => "Mythic ",
        }
    }

    /// Returns the stat multiplier for this rarity
    pub fn stat_bonus(&self) -> f32 {
        match self {
            Self::Common => 1.0,
            Self::Uncommon => 1.25,
            Self::Rare => 1.5,
            Self::Epic => 2.0,
            Self::Legendary => 3.0,
            Self::Mythic => 5.0,
        }
    }
}

/// All item types in the game
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum ItemKind {
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

    // Food - Basic (8)
    Bread,
    Meat,
    Apple,
    Cheese,
    Feast,
    DragonFruit,
    AncientWine,
    GoldenApple,

    // Food - Raw ingredients (can be cooked)
    RawMeat,
    RawFish,
    RawVegetables,
    RawEgg,
    Mushrooms,
    RawPoultry,

    // Food - Cooked dishes
    CookedMeat,
    GrilledFish,
    Stew,
    Omelette,
    RoastChicken,
    MeatPie,
    FruitSalad,
    HeartyStew,
    DragonSteak,
    FeastOfKings,

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
}

impl ItemKind {
    /// Returns the glyph character for this item
    pub fn glyph(&self) -> char {
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
            | Self::DragonFruit | Self::AncientWine | Self::GoldenApple
            | Self::RawMeat | Self::RawFish | Self::RawVegetables | Self::RawEgg
            | Self::Mushrooms | Self::RawPoultry | Self::CookedMeat | Self::GrilledFish
            | Self::Stew | Self::Omelette | Self::RoastChicken | Self::MeatPie
            | Self::FruitSalad | Self::HeartyStew | Self::DragonSteak | Self::FeastOfKings => '%',

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
        }
    }

    /// Returns the display name of the item
    pub fn name(&self) -> &'static str {
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

            // Raw ingredients
            Self::RawMeat => "Raw Meat",
            Self::RawFish => "Raw Fish",
            Self::RawVegetables => "Raw Vegetables",
            Self::RawEgg => "Raw Egg",
            Self::Mushrooms => "Wild Mushrooms",
            Self::RawPoultry => "Raw Poultry",

            // Cooked dishes
            Self::CookedMeat => "Cooked Meat",
            Self::GrilledFish => "Grilled Fish",
            Self::Stew => "Hearty Stew",
            Self::Omelette => "Omelette",
            Self::RoastChicken => "Roast Chicken",
            Self::MeatPie => "Meat Pie",
            Self::FruitSalad => "Fruit Salad",
            Self::HeartyStew => "Hunter's Stew",
            Self::DragonSteak => "Dragon Steak",
            Self::FeastOfKings => "Feast of Kings",

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
        }
    }

    /// Returns the equipment slot for this item, if any
    pub fn equip_slot(&self) -> Option<EquipSlot> {
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

    /// Returns base stats: (attack, defense, hp_bonus, mana_bonus)
    pub fn base_stats(&self) -> (i32, i32, i32, i32) {
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

    /// Returns whether this item is consumable
    pub fn is_consumable(&self) -> bool {
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

    /// Returns whether this item is food
    pub fn is_food(&self) -> bool {
        matches!(
            self,
            Self::Bread | Self::Meat | Self::Apple | Self::Cheese | Self::Feast
                | Self::DragonFruit | Self::AncientWine | Self::GoldenApple
                | Self::RawMeat | Self::RawFish | Self::RawVegetables | Self::RawEgg
                | Self::Mushrooms | Self::RawPoultry | Self::CookedMeat | Self::GrilledFish
                | Self::Stew | Self::Omelette | Self::RoastChicken | Self::MeatPie
                | Self::FruitSalad | Self::HeartyStew | Self::DragonSteak | Self::FeastOfKings
        )
    }

    /// Returns whether this item can be cooked
    pub fn is_cookable(&self) -> bool {
        matches!(
            self,
            Self::RawMeat | Self::RawFish | Self::RawVegetables | Self::RawEgg
                | Self::Mushrooms | Self::RawPoultry | Self::Meat
        )
    }

    /// Returns the base food/hunger value of this item (before quality adjustment)
    pub fn food_value(&self) -> i32 {
        match self {
            // Basic foods
            Self::Apple => 10,
            Self::Bread => 25,
            Self::Cheese => 20,
            Self::Meat => 40,
            Self::Feast => 100,
            Self::DragonFruit => 30,
            Self::AncientWine => 35,
            Self::GoldenApple => 50,

            // Raw ingredients (lower base value)
            Self::RawMeat => 25,
            Self::RawFish => 20,
            Self::RawVegetables => 15,
            Self::RawEgg => 10,
            Self::Mushrooms => 12,
            Self::RawPoultry => 22,

            // Cooked dishes (higher base value)
            Self::CookedMeat => 45,
            Self::GrilledFish => 40,
            Self::Stew => 55,
            Self::Omelette => 35,
            Self::RoastChicken => 50,
            Self::MeatPie => 60,
            Self::FruitSalad => 30,
            Self::HeartyStew => 70,
            Self::DragonSteak => 90,
            Self::FeastOfKings => 150,

            _ => 0,
        }
    }

    /// Returns the default food quality for this item type
    pub fn default_food_quality(&self) -> FoodQuality {
        match self {
            // Raw ingredients default to Raw quality
            Self::RawMeat | Self::RawFish | Self::RawVegetables
            | Self::RawEgg | Self::RawPoultry => FoodQuality::Raw,

            // Wild mushrooms are risky
            Self::Mushrooms => FoodQuality::Raw,

            // Basic preserved foods are Fresh
            Self::Bread | Self::Cheese | Self::Apple | Self::Meat => FoodQuality::Fresh,

            // Cooked dishes
            Self::CookedMeat | Self::GrilledFish | Self::Omelette => FoodQuality::Cooked,
            Self::Stew | Self::RoastChicken | Self::MeatPie | Self::FruitSalad => FoodQuality::WellCooked,
            Self::HeartyStew => FoodQuality::WellCooked,

            // Special foods
            Self::DragonFruit | Self::AncientWine | Self::GoldenApple => FoodQuality::Gourmet,
            Self::DragonSteak => FoodQuality::Gourmet,
            Self::Feast | Self::FeastOfKings => FoodQuality::Legendary,

            _ => FoodQuality::Fresh,
        }
    }

    /// Returns what this food item turns into when cooked (if cookable)
    pub fn cooked_result(&self) -> Option<ItemKind> {
        match self {
            Self::RawMeat | Self::Meat => Some(Self::CookedMeat),
            Self::RawFish => Some(Self::GrilledFish),
            Self::RawEgg => Some(Self::Omelette),
            Self::RawPoultry => Some(Self::RoastChicken),
            Self::RawVegetables => Some(Self::Stew),
            Self::Mushrooms => Some(Self::Stew),
            _ => None,
        }
    }

    /// Check if combining two ingredients creates a special dish
    pub fn combine_ingredients(a: ItemKind, b: ItemKind) -> Option<ItemKind> {
        let mut ingredients = [a, b];
        ingredients.sort_by_key(|i| *i as i32);

        match ingredients {
            [ItemKind::RawMeat, ItemKind::RawVegetables] => Some(ItemKind::HeartyStew),
            [ItemKind::CookedMeat, ItemKind::Bread] => Some(ItemKind::MeatPie),
            [ItemKind::Apple, ItemKind::DragonFruit] => Some(ItemKind::FruitSalad),
            [ItemKind::RawMeat, ItemKind::DragonScale] => Some(ItemKind::DragonSteak),
            [ItemKind::Feast, ItemKind::GoldenApple] => Some(ItemKind::FeastOfKings),
            _ => None,
        }
    }
}

/// An item instance with position and rarity
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Item {
    pub x: usize,
    pub y: usize,
    pub kind: ItemKind,
    pub rarity: Rarity,
    /// Food quality (only relevant for food items)
    pub food_quality: Option<FoodQuality>,
    /// Turns until food spoils (0 = doesn't spoil)
    pub spoil_timer: u32,
}

impl Item {
    /// Create a new item at the given position
    pub fn new(x: usize, y: usize, kind: ItemKind, rarity: Rarity) -> Self {
        let (food_quality, spoil_timer) = if kind.is_food() {
            let quality = kind.default_food_quality();
            // Foods can spoil over time (except legendary and rotten)
            let timer = if quality.can_spoil() { 500 } else { 0 };
            (Some(quality), timer)
        } else {
            (None, 0)
        };

        Self { x, y, kind, rarity, food_quality, spoil_timer }
    }

    /// Create a food item with specific quality
    pub fn new_food(x: usize, y: usize, kind: ItemKind, quality: FoodQuality) -> Self {
        let spoil_timer = if quality.can_spoil() { 500 } else { 0 };
        Self {
            x,
            y,
            kind,
            rarity: Rarity::Common,
            food_quality: Some(quality),
            spoil_timer,
        }
    }

    /// Get the food quality, defaulting to Fresh if not set
    pub fn get_food_quality(&self) -> FoodQuality {
        self.food_quality.unwrap_or(self.kind.default_food_quality())
    }

    /// Tick the spoil timer and potentially degrade quality
    pub fn tick_spoil(&mut self) -> bool {
        if self.spoil_timer > 0 {
            self.spoil_timer -= 1;
            if self.spoil_timer == 0 {
                if let Some(ref mut quality) = self.food_quality {
                    *quality = quality.spoiled();
                    // Reset timer if can still spoil further
                    if quality.can_spoil() {
                        self.spoil_timer = 300;
                    }
                    return true; // Quality changed
                }
            }
        }
        false
    }

    /// Returns the stats of this item scaled by rarity
    pub fn stats(&self) -> (i32, i32, i32, i32) {
        let (atk, def, hp, mana) = self.kind.base_stats();
        let mult = self.rarity.stat_bonus();
        (
            (atk as f32 * mult) as i32,
            (def as f32 * mult) as i32,
            (hp as f32 * mult) as i32,
            (mana as f32 * mult) as i32,
        )
    }

    /// Returns the display name including rarity prefix
    pub fn display_name(&self) -> String {
        format!("{}{}", self.rarity.prefix(), self.kind.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rarity_ordering() {
        assert!(Rarity::Common < Rarity::Uncommon);
        assert!(Rarity::Uncommon < Rarity::Rare);
        assert!(Rarity::Rare < Rarity::Epic);
        assert!(Rarity::Epic < Rarity::Legendary);
        assert!(Rarity::Legendary < Rarity::Mythic);
    }

    #[test]
    fn test_item_stats_scale_with_rarity() {
        let common = Item::new(0, 0, ItemKind::LongSword, Rarity::Common);
        let legendary = Item::new(0, 0, ItemKind::LongSword, Rarity::Legendary);

        assert!(legendary.stats().0 > common.stats().0);
    }
}
