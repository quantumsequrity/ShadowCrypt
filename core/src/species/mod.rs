//! Species and Subspecies System
//!
//! Comprehensive race system with 12 major species and 73+ subspecies,
//! each with unique traits, abilities, and stat modifiers.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Major species categories
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Species {
    Human,
    Elf,
    Dwarf,
    Orc,
    Undead,
    Demon,
    Beastkin,
    Dragonborn,
    Fae,
    Construct,
    Elemental,
    Celestial,
}

impl Species {
    pub fn all() -> &'static [Species] {
        &[
            Self::Human, Self::Elf, Self::Dwarf, Self::Orc,
            Self::Undead, Self::Demon, Self::Beastkin, Self::Dragonborn,
            Self::Fae, Self::Construct, Self::Elemental, Self::Celestial,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Human => "Human",
            Self::Elf => "Elf",
            Self::Dwarf => "Dwarf",
            Self::Orc => "Orc",
            Self::Undead => "Undead",
            Self::Demon => "Demon",
            Self::Beastkin => "Beastkin",
            Self::Dragonborn => "Dragonborn",
            Self::Fae => "Fae",
            Self::Construct => "Construct",
            Self::Elemental => "Elemental",
            Self::Celestial => "Celestial",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Human => "Versatile and adaptable, humans excel at any role.",
            Self::Elf => "Ancient and magical, elves are masters of arcane arts.",
            Self::Dwarf => "Sturdy and resilient, dwarves are natural defenders.",
            Self::Orc => "Fierce and powerful, orcs dominate in combat.",
            Self::Undead => "Risen from death, undead are immune to many afflictions.",
            Self::Demon => "Born of darkness, demons wield forbidden power.",
            Self::Beastkin => "Half-beast hybrids with animal instincts and abilities.",
            Self::Dragonborn => "Dragon-blooded warriors with elemental breath.",
            Self::Fae => "Mystical beings from the spirit realm.",
            Self::Construct => "Artificial beings powered by magic or machinery.",
            Self::Elemental => "Beings of pure elemental energy.",
            Self::Celestial => "Divine beings touched by the heavens.",
        }
    }

    pub fn subspecies(&self) -> &'static [Subspecies] {
        match self {
            Self::Human => &[
                Subspecies::Noble, Subspecies::Commoner, Subspecies::Barbarian,
                Subspecies::Islander, Subspecies::Northerner, Subspecies::Southerner,
                Subspecies::Imperial, Subspecies::Nomad,
            ],
            Self::Elf => &[
                Subspecies::HighElf, Subspecies::WoodElf, Subspecies::DarkElf,
                Subspecies::SeaElf, Subspecies::BloodElf, Subspecies::MoonElf,
                Subspecies::SunElf, Subspecies::WildElf,
            ],
            Self::Dwarf => &[
                Subspecies::Mountain, Subspecies::Hill, Subspecies::Deep,
                Subspecies::Frost, Subspecies::GoldDwarf, Subspecies::IronDwarf,
                Subspecies::RuneDwarf, Subspecies::FireDwarf,
            ],
            Self::Orc => &[
                Subspecies::Green, Subspecies::Grey, Subspecies::Black,
                Subspecies::HalfOrc,
            ],
            Self::Undead => &[
                Subspecies::Skeleton, Subspecies::Zombie, Subspecies::Ghost,
                Subspecies::Vampire, Subspecies::Lich,
            ],
            Self::Demon => &[
                Subspecies::Imp, Subspecies::Succubus, Subspecies::PitFiend,
                Subspecies::Balor,
            ],
            Self::Beastkin => &[
                Subspecies::Wolf, Subspecies::Cat, Subspecies::Bear,
                Subspecies::Fox, Subspecies::Rabbit,
            ],
            Self::Dragonborn => &[
                Subspecies::FireDrake, Subspecies::IceDrake, Subspecies::StormDrake,
                Subspecies::ShadowDrake, Subspecies::Drake, Subspecies::Wyrm,
                Subspecies::TrueDragon, Subspecies::ElderDragon, Subspecies::DragonGod,
            ],
            Self::Fae => &[
                Subspecies::Pixie, Subspecies::Sprite, Subspecies::Dryad,
                Subspecies::Nymph, Subspecies::Sylph, Subspecies::WillOWisp,
                Subspecies::DarkFairy, Subspecies::Leprechaun,
            ],
            Self::Construct => &[
                Subspecies::Golem, Subspecies::Automaton, Subspecies::Warforged,
                Subspecies::Clockwork,
            ],
            Self::Elemental => &[
                Subspecies::Fire, Subspecies::Water, Subspecies::Earth,
                Subspecies::Air,
            ],
            Self::Celestial => &[
                Subspecies::Angel, Subspecies::Seraph, Subspecies::Archon,
                Subspecies::Nephilim, Subspecies::Cherub, Subspecies::Throne,
                Subspecies::Dominion, Subspecies::Virtue,
            ],
        }
    }

    pub fn base_modifiers(&self) -> StatModifiers {
        match self {
            Self::Human => StatModifiers {
                hp: 1.0, attack: 1.0, defense: 1.0, mana: 1.0, speed: 1.0,
                xp_bonus: 1.1, // Humans learn faster
            },
            Self::Elf => StatModifiers {
                hp: 0.85, attack: 0.95, defense: 0.90, mana: 1.30, speed: 1.15,
                xp_bonus: 1.0,
            },
            Self::Dwarf => StatModifiers {
                hp: 1.20, attack: 1.05, defense: 1.15, mana: 0.80, speed: 0.90,
                xp_bonus: 1.0,
            },
            Self::Orc => StatModifiers {
                hp: 1.15, attack: 1.20, defense: 0.95, mana: 0.70, speed: 1.0,
                xp_bonus: 0.95,
            },
            Self::Undead => StatModifiers {
                hp: 1.0, attack: 1.0, defense: 1.10, mana: 0.90, speed: 0.85,
                xp_bonus: 1.0,
            },
            Self::Demon => StatModifiers {
                hp: 0.95, attack: 1.15, defense: 0.90, mana: 1.20, speed: 1.05,
                xp_bonus: 1.0,
            },
            Self::Beastkin => StatModifiers {
                hp: 1.05, attack: 1.10, defense: 0.95, mana: 0.85, speed: 1.20,
                xp_bonus: 1.0,
            },
            Self::Dragonborn => StatModifiers {
                hp: 1.10, attack: 1.10, defense: 1.10, mana: 1.0, speed: 0.95,
                xp_bonus: 0.95,
            },
            Self::Fae => StatModifiers {
                hp: 0.70, attack: 0.80, defense: 0.70, mana: 1.50, speed: 1.30,
                xp_bonus: 1.0,
            },
            Self::Construct => StatModifiers {
                hp: 1.30, attack: 1.0, defense: 1.25, mana: 0.50, speed: 0.80,
                xp_bonus: 0.90,
            },
            Self::Elemental => StatModifiers {
                hp: 0.90, attack: 1.15, defense: 0.85, mana: 1.25, speed: 1.10,
                xp_bonus: 1.0,
            },
            Self::Celestial => StatModifiers {
                hp: 1.0, attack: 1.0, defense: 1.0, mana: 1.20, speed: 1.10,
                xp_bonus: 1.05,
            },
        }
    }

    pub fn innate_resistances(&self) -> Vec<DamageType> {
        match self {
            Self::Undead => vec![DamageType::Poison, DamageType::Necrotic],
            Self::Demon => vec![DamageType::Fire, DamageType::Necrotic],
            Self::Construct => vec![DamageType::Poison, DamageType::Psychic],
            Self::Elemental => vec![], // Depends on subspecies
            Self::Celestial => vec![DamageType::Radiant, DamageType::Necrotic],
            _ => vec![],
        }
    }

    pub fn innate_weaknesses(&self) -> Vec<DamageType> {
        match self {
            Self::Undead => vec![DamageType::Radiant, DamageType::Fire],
            Self::Demon => vec![DamageType::Radiant],
            Self::Fae => vec![DamageType::Cold],
            Self::Construct => vec![DamageType::Lightning],
            _ => vec![],
        }
    }
}

/// Subspecies with unique traits
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Subspecies {
    // Human subspecies
    Noble,        // High status, +leadership
    Commoner,     // Balanced, +xp gain
    Barbarian,    // Wild, +combat stats
    Islander,     // Sea-faring, +speed

    // Elf subspecies
    HighElf,      // Arcane masters, +mana
    WoodElf,      // Forest dwellers, +speed
    DarkElf,      // Underground, +stealth
    SeaElf,       // Aquatic, water breathing

    // Dwarf subspecies
    Mountain,     // Traditional, +defense
    Hill,         // Surface dwellers, +hp
    Deep,         // Underdark, darkvision
    Frost,        // Ice mountains, cold resist

    // Orc subspecies
    Green,        // Common, balanced
    Grey,         // Cunning, +defense
    Black,        // Elite warriors, +attack
    HalfOrc,      // Human heritage, +versatility

    // Undead subspecies
    Skeleton,     // Basic, immune to bleed
    Zombie,       // Tough, +hp, slow
    Ghost,        // Ethereal, dodge chance
    Vampire,      // Life drain, sun weakness
    Lich,         // Powerful mage, phylactery

    // Demon subspecies
    Imp,          // Small, fast, trickster
    Succubus,     // Charmer, life drain
    PitFiend,     // Commander, +leadership
    Balor,        // Fire demon, burn attacks

    // Beastkin subspecies
    Wolf,         // Pack tactics, tracking
    Cat,          // Agile, night vision
    Bear,         // Strong, rage ability
    Fox,          // Cunning, +intelligence
    Rabbit,       // Fast, +evasion

    // Dragonborn subspecies
    FireDrake,    // Fire breath, fire resist
    IceDrake,     // Ice breath, cold resist
    StormDrake,   // Lightning breath, storm resist
    ShadowDrake,  // Shadow breath, stealth

    // Fae subspecies
    Pixie,        // Tiny, flying, illusions
    Sprite,       // Nature magic, healing
    Dryad,        // Tree-bound, plant control
    Nymph,        // Water/nature, charm

    // Construct subspecies
    Golem,        // Stone/clay, high defense
    Automaton,    // Metal, balanced
    Warforged,    // Battle construct, weapons
    Clockwork,    // Precision, +accuracy

    // Elemental subspecies
    Fire,         // Fire elemental
    Water,        // Water elemental
    Earth,        // Earth elemental
    Air,          // Air elemental

    // Celestial subspecies
    Angel,        // Divine messenger
    Seraph,       // Burning one, fire/holy
    Archon,       // Law incarnate
    Nephilim,     // Half-mortal, +stats
    Cherub,       // Guardian spirit, +defense
    Throne,       // Seat of power, +leadership
    Dominion,     // Authority incarnate, +attack
    Virtue,       // Embodiment of good, +healing

    // Additional Elf subspecies
    BloodElf,     // Mana-addicted, +mana drain
    MoonElf,      // Lunar magic, +night power
    SunElf,       // Solar magic, +day power
    WildElf,      // Feral, +survival

    // Additional Dwarf subspecies
    GoldDwarf,    // Wealthy, +trade
    IronDwarf,    // Metalworkers, +crafting
    RuneDwarf,    // Runic magic, +enchanting
    FireDwarf,    // Volcanic, +fire resist

    // Additional Fae subspecies
    Sylph,        // Wind spirit, +speed
    WillOWisp,    // Light spirit, +evasion
    DarkFairy,    // Corrupted, +shadow magic
    Leprechaun,   // Lucky, +gold find

    // Additional Human subspecies
    Northerner,   // Hardy, +cold resist
    Southerner,   // Desert, +heat resist
    Imperial,     // Military, +discipline
    Nomad,        // Wanderer, +survival

    // Dragonborn evolution subspecies
    Drake,        // Young dragon, balanced
    Wyrm,         // Ancient, +mana
    TrueDragon,   // Full dragon form, +all stats
    ElderDragon,  // Ancient wisdom, +mana/defense
    DragonGod,    // Transcendent, massive +all
}

impl Subspecies {
    pub fn name(&self) -> &'static str {
        match self {
            // Humans
            Self::Noble => "Noble",
            Self::Commoner => "Commoner",
            Self::Barbarian => "Barbarian",
            Self::Islander => "Islander",
            // Elves
            Self::HighElf => "High Elf",
            Self::WoodElf => "Wood Elf",
            Self::DarkElf => "Dark Elf",
            Self::SeaElf => "Sea Elf",
            // Dwarves
            Self::Mountain => "Mountain Dwarf",
            Self::Hill => "Hill Dwarf",
            Self::Deep => "Deep Dwarf",
            Self::Frost => "Frost Dwarf",
            // Orcs
            Self::Green => "Green Orc",
            Self::Grey => "Grey Orc",
            Self::Black => "Black Orc",
            Self::HalfOrc => "Half-Orc",
            // Undead
            Self::Skeleton => "Skeleton",
            Self::Zombie => "Zombie",
            Self::Ghost => "Ghost",
            Self::Vampire => "Vampire",
            Self::Lich => "Lich",
            // Demons
            Self::Imp => "Imp",
            Self::Succubus => "Succubus",
            Self::PitFiend => "Pit Fiend",
            Self::Balor => "Balor",
            // Beastkin
            Self::Wolf => "Wolfkin",
            Self::Cat => "Catfolk",
            Self::Bear => "Bearkin",
            Self::Fox => "Foxkin",
            Self::Rabbit => "Rabbitfolk",
            // Dragonborn
            Self::FireDrake => "Fire Drake",
            Self::IceDrake => "Ice Drake",
            Self::StormDrake => "Storm Drake",
            Self::ShadowDrake => "Shadow Drake",
            // Fae
            Self::Pixie => "Pixie",
            Self::Sprite => "Sprite",
            Self::Dryad => "Dryad",
            Self::Nymph => "Nymph",
            // Constructs
            Self::Golem => "Golem",
            Self::Automaton => "Automaton",
            Self::Warforged => "Warforged",
            Self::Clockwork => "Clockwork",
            // Elementals
            Self::Fire => "Fire Elemental",
            Self::Water => "Water Elemental",
            Self::Earth => "Earth Elemental",
            Self::Air => "Air Elemental",
            // Celestials
            Self::Angel => "Angel",
            Self::Seraph => "Seraph",
            Self::Archon => "Archon",
            Self::Nephilim => "Nephilim",
            Self::Cherub => "Cherub",
            Self::Throne => "Throne",
            Self::Dominion => "Dominion",
            Self::Virtue => "Virtue",
            // Additional Elves
            Self::BloodElf => "Blood Elf",
            Self::MoonElf => "Moon Elf",
            Self::SunElf => "Sun Elf",
            Self::WildElf => "Wild Elf",
            // Additional Dwarves
            Self::GoldDwarf => "Gold Dwarf",
            Self::IronDwarf => "Iron Dwarf",
            Self::RuneDwarf => "Rune Dwarf",
            Self::FireDwarf => "Fire Dwarf",
            // Additional Fae
            Self::Sylph => "Sylph",
            Self::WillOWisp => "Will-o'-Wisp",
            Self::DarkFairy => "Dark Fairy",
            Self::Leprechaun => "Leprechaun",
            // Additional Humans
            Self::Northerner => "Northerner",
            Self::Southerner => "Southerner",
            Self::Imperial => "Imperial",
            Self::Nomad => "Nomad",
            // Dragonborn evolution
            Self::Drake => "Drake",
            Self::Wyrm => "Wyrm",
            Self::TrueDragon => "True Dragon",
            Self::ElderDragon => "Elder Dragon",
            Self::DragonGod => "Dragon God",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Noble => "Educated and wealthy, born to lead.",
            Self::Commoner => "Hard-working and adaptable, learns quickly.",
            Self::Barbarian => "Fierce warrior from the wilds.",
            Self::Islander => "Born of the sea, swift and agile.",
            Self::HighElf => "Master of arcane arts and ancient lore.",
            Self::WoodElf => "Guardian of the forest, swift and silent.",
            Self::DarkElf => "Dweller of the deep, master of shadow.",
            Self::SeaElf => "At home in the water as much as land.",
            Self::Mountain => "Stout defender of the mountain holds.",
            Self::Hill => "Trader and craftsman of the surface hills.",
            Self::Deep => "Survivor of the dangerous underdark.",
            Self::Frost => "Hardened by the eternal winter peaks.",
            Self::Green => "The most common orc, balanced and strong.",
            Self::Grey => "Cunning strategist of the rocky wastes.",
            Self::Black => "Elite warrior bred for war.",
            Self::HalfOrc => "Walking two worlds, versatile and tough.",
            Self::Skeleton => "Bones animated by magic, tireless.",
            Self::Zombie => "Flesh animated by dark rites, resilient.",
            Self::Ghost => "Spirit bound to the mortal plane.",
            Self::Vampire => "Aristocrat of the night, drains life.",
            Self::Lich => "Master of death magic, immortal.",
            Self::Imp => "Tiny mischief maker, hard to hit.",
            Self::Succubus => "Manipulator of hearts and minds.",
            Self::PitFiend => "General of the infernal armies.",
            Self::Balor => "Living engine of fire and destruction.",
            Self::Wolf => "Hunter with keen senses and pack instincts.",
            Self::Cat => "Graceful predator, always lands on feet.",
            Self::Bear => "Powerhouse of raw strength.",
            Self::Fox => "Trickster spirit, clever and quick.",
            Self::Rabbit => "Impossible to catch, senses danger.",
            Self::FireDrake => "Blood of the red dragons, breathes fire.",
            Self::IceDrake => "Blood of the white dragons, breathes ice.",
            Self::StormDrake => "Blood of the blue dragons, breathes lightning.",
            Self::ShadowDrake => "Blood of the shadow dragons, breathes darkness.",
            Self::Pixie => "Tiny magical prankster.",
            Self::Sprite => "Protector of nature's beauty.",
            Self::Dryad => "One with the trees and plants.",
            Self::Nymph => "Spirit of the waters and wild places.",
            Self::Golem => "Animated matter, slow but unbreakable.",
            Self::Automaton => "Clockwork precision and metal skin.",
            Self::Warforged => "Built for a war that ended ages ago.",
            Self::Clockwork => "Intricate machinery given life.",
            Self::Fire => "Living flame, burns all it touches.",
            Self::Water => "Fluid and changing, crashes like waves.",
            Self::Earth => "Solid and unmoving as the mountain.",
            Self::Air => "Free and unseen as the wind.",
            Self::Angel => "Messenger of the divine.",
            Self::Seraph => "Burning purity of the highest order.",
            Self::Archon => "Enforcer of cosmic law.",
            Self::Nephilim => "Child of heaven and earth.",
            Self::Cherub => "Guardian spirit that shields the worthy.",
            Self::Throne => "Living seat of divine power and authority.",
            Self::Dominion => "Authority incarnate, commands the celestial host.",
            Self::Virtue => "Embodiment of goodness and divine healing.",
            // Additional Elves
            Self::BloodElf => "Addicted to mana, drains magic from others.",
            Self::MoonElf => "Blessed by the moon, powerful at night.",
            Self::SunElf => "Empowered by sunlight, radiant in day.",
            Self::WildElf => "Feral and untamed, at one with nature.",
            // Additional Dwarves
            Self::GoldDwarf => "Wealthy traders with a nose for treasure.",
            Self::IronDwarf => "Master metalworkers who forge legendary arms.",
            Self::RuneDwarf => "Practitioners of ancient runic magic.",
            Self::FireDwarf => "Volcanic dwellers, resistant to flame.",
            // Additional Fae
            Self::Sylph => "Wind spirit, swift and elusive.",
            Self::WillOWisp => "Ethereal light spirit, nearly impossible to hit.",
            Self::DarkFairy => "Corrupted fae wielding shadow magic.",
            Self::Leprechaun => "Lucky trickster who always finds gold.",
            // Additional Humans
            Self::Northerner => "Hardy folk from frozen lands, tough and resilient.",
            Self::Southerner => "Desert-born, accustomed to scorching heat.",
            Self::Imperial => "Disciplined soldier of the great empire.",
            Self::Nomad => "Wanderer of the wilds, survivalist and explorer.",
            // Dragonborn evolution
            Self::Drake => "Young dragon-blooded warrior, balanced in all ways.",
            Self::Wyrm => "Ancient serpentine dragon, deep in magic.",
            Self::TrueDragon => "Fully awakened dragon form, power incarnate.",
            Self::ElderDragon => "Ancient dragon of immense wisdom and power.",
            Self::DragonGod => "Transcendent draconic deity, beyond mortal limits.",
        }
    }

    pub fn parent_species(&self) -> Species {
        match self {
            Self::Noble | Self::Commoner | Self::Barbarian | Self::Islander
                | Self::Northerner | Self::Southerner | Self::Imperial | Self::Nomad => Species::Human,
            Self::HighElf | Self::WoodElf | Self::DarkElf | Self::SeaElf
                | Self::BloodElf | Self::MoonElf | Self::SunElf | Self::WildElf => Species::Elf,
            Self::Mountain | Self::Hill | Self::Deep | Self::Frost
                | Self::GoldDwarf | Self::IronDwarf | Self::RuneDwarf | Self::FireDwarf => Species::Dwarf,
            Self::Green | Self::Grey | Self::Black | Self::HalfOrc => Species::Orc,
            Self::Skeleton | Self::Zombie | Self::Ghost | Self::Vampire | Self::Lich => Species::Undead,
            Self::Imp | Self::Succubus | Self::PitFiend | Self::Balor => Species::Demon,
            Self::Wolf | Self::Cat | Self::Bear | Self::Fox | Self::Rabbit => Species::Beastkin,
            Self::FireDrake | Self::IceDrake | Self::StormDrake | Self::ShadowDrake
                | Self::Drake | Self::Wyrm | Self::TrueDragon | Self::ElderDragon | Self::DragonGod => Species::Dragonborn,
            Self::Pixie | Self::Sprite | Self::Dryad | Self::Nymph
                | Self::Sylph | Self::WillOWisp | Self::DarkFairy | Self::Leprechaun => Species::Fae,
            Self::Golem | Self::Automaton | Self::Warforged | Self::Clockwork => Species::Construct,
            Self::Fire | Self::Water | Self::Earth | Self::Air => Species::Elemental,
            Self::Angel | Self::Seraph | Self::Archon | Self::Nephilim
                | Self::Cherub | Self::Throne | Self::Dominion | Self::Virtue => Species::Celestial,
        }
    }

    pub fn modifiers(&self) -> StatModifiers {
        match self {
            // Human: +10% XP, Versatile
            Self::Noble => StatModifiers { hp: 1.0, attack: 1.0, defense: 1.0, mana: 1.1, speed: 1.0, xp_bonus: 1.15 },
            Self::Commoner => StatModifiers { hp: 1.05, attack: 1.0, defense: 1.0, mana: 1.0, speed: 1.0, xp_bonus: 1.20 },
            Self::Barbarian => StatModifiers { hp: 1.15, attack: 1.1, defense: 0.9, mana: 0.8, speed: 1.05, xp_bonus: 1.10 },
            Self::Islander => StatModifiers { hp: 1.0, attack: 1.0, defense: 0.95, mana: 1.0, speed: 1.15, xp_bonus: 1.10 },

            // Elf: +20% Mana, +10% Speed
            Self::HighElf => StatModifiers { hp: 0.9, attack: 0.9, defense: 0.9, mana: 1.3, speed: 1.1, xp_bonus: 1.0 },
            Self::WoodElf => StatModifiers { hp: 1.0, attack: 1.0, defense: 0.9, mana: 1.2, speed: 1.2, xp_bonus: 1.0 },
            Self::DarkElf => StatModifiers { hp: 0.95, attack: 1.1, defense: 0.9, mana: 1.25, speed: 1.15, xp_bonus: 1.0 },
            Self::SeaElf => StatModifiers { hp: 1.0, attack: 1.0, defense: 1.0, mana: 1.2, speed: 1.15, xp_bonus: 1.0 },

            // Dwarf: +20% Defense
            Self::Mountain => StatModifiers { hp: 1.1, attack: 1.0, defense: 1.3, mana: 0.8, speed: 0.9, xp_bonus: 1.0 },
            Self::Hill => StatModifiers { hp: 1.2, attack: 1.0, defense: 1.2, mana: 0.9, speed: 0.9, xp_bonus: 1.0 },
            Self::Deep => StatModifiers { hp: 1.0, attack: 1.1, defense: 1.2, mana: 1.0, speed: 0.9, xp_bonus: 1.0 },
            Self::Frost => StatModifiers { hp: 1.15, attack: 1.05, defense: 1.2, mana: 0.8, speed: 0.9, xp_bonus: 1.0 },

            // Orc: +25% Attack, -10% Int
            Self::Green => StatModifiers { hp: 1.1, attack: 1.25, defense: 1.0, mana: 0.7, speed: 1.0, xp_bonus: 1.0 },
            Self::Grey => StatModifiers { hp: 1.0, attack: 1.2, defense: 1.1, mana: 0.8, speed: 1.0, xp_bonus: 1.0 },
            Self::Black => StatModifiers { hp: 1.15, attack: 1.35, defense: 1.0, mana: 0.6, speed: 1.0, xp_bonus: 0.95 },
            Self::HalfOrc => StatModifiers { hp: 1.05, attack: 1.15, defense: 1.0, mana: 0.9, speed: 1.0, xp_bonus: 1.05 },

            // Undead: Immunity/Resistances focus
            Self::Skeleton => StatModifiers { hp: 0.8, attack: 1.0, defense: 1.1, mana: 0.9, speed: 1.1, xp_bonus: 1.0 },
            Self::Zombie => StatModifiers { hp: 1.4, attack: 1.1, defense: 1.1, mana: 0.5, speed: 0.7, xp_bonus: 1.0 },
            Self::Ghost => StatModifiers { hp: 0.6, attack: 0.8, defense: 1.5, mana: 1.2, speed: 1.2, xp_bonus: 1.0 },
            Self::Vampire => StatModifiers { hp: 1.1, attack: 1.2, defense: 1.0, mana: 1.1, speed: 1.1, xp_bonus: 1.0 },
            Self::Lich => StatModifiers { hp: 0.7, attack: 0.8, defense: 0.8, mana: 1.5, speed: 0.9, xp_bonus: 1.0 },

            // Demon: +15% Attack
            Self::Imp => StatModifiers { hp: 0.7, attack: 1.1, defense: 0.8, mana: 1.1, speed: 1.4, xp_bonus: 1.0 },
            Self::Succubus => StatModifiers { hp: 0.9, attack: 1.0, defense: 0.9, mana: 1.3, speed: 1.1, xp_bonus: 1.0 },
            Self::PitFiend => StatModifiers { hp: 1.2, attack: 1.3, defense: 1.1, mana: 1.0, speed: 1.0, xp_bonus: 1.0 },
            Self::Balor => StatModifiers { hp: 1.3, attack: 1.4, defense: 1.0, mana: 1.1, speed: 0.9, xp_bonus: 0.9 },

            // Beastkin: +15% Speed
            Self::Wolf => StatModifiers { hp: 1.0, attack: 1.1, defense: 0.95, mana: 0.9, speed: 1.2, xp_bonus: 1.0 },
            Self::Cat => StatModifiers { hp: 0.9, attack: 1.1, defense: 0.85, mana: 1.0, speed: 1.3, xp_bonus: 1.0 },
            Self::Bear => StatModifiers { hp: 1.3, attack: 1.2, defense: 1.1, mana: 0.8, speed: 0.9, xp_bonus: 1.0 },
            Self::Fox => StatModifiers { hp: 0.85, attack: 0.95, defense: 0.9, mana: 1.2, speed: 1.2, xp_bonus: 1.0 },
            Self::Rabbit => StatModifiers { hp: 0.7, attack: 0.8, defense: 0.8, mana: 1.0, speed: 1.5, xp_bonus: 1.1 },

            // Dragonborn: +20% HP
            Self::FireDrake => StatModifiers { hp: 1.2, attack: 1.1, defense: 1.1, mana: 1.0, speed: 0.9, xp_bonus: 1.0 },
            Self::IceDrake => StatModifiers { hp: 1.25, attack: 1.0, defense: 1.15, mana: 1.0, speed: 0.9, xp_bonus: 1.0 },
            Self::StormDrake => StatModifiers { hp: 1.2, attack: 1.1, defense: 1.0, mana: 1.1, speed: 1.0, xp_bonus: 1.0 },
            Self::ShadowDrake => StatModifiers { hp: 1.15, attack: 1.1, defense: 1.0, mana: 1.1, speed: 1.1, xp_bonus: 1.0 },

            // Fae: +30% Mana
            Self::Pixie => StatModifiers { hp: 0.5, attack: 0.6, defense: 0.6, mana: 1.4, speed: 1.4, xp_bonus: 1.0 },
            Self::Sprite => StatModifiers { hp: 0.7, attack: 0.8, defense: 0.7, mana: 1.3, speed: 1.3, xp_bonus: 1.0 },
            Self::Dryad => StatModifiers { hp: 1.1, attack: 0.9, defense: 1.1, mana: 1.3, speed: 0.9, xp_bonus: 1.0 },
            Self::Nymph => StatModifiers { hp: 0.9, attack: 0.8, defense: 0.8, mana: 1.4, speed: 1.1, xp_bonus: 1.0 },

            // Construct: +25% Defense
            Self::Golem => StatModifiers { hp: 1.4, attack: 1.1, defense: 1.4, mana: 0.4, speed: 0.6, xp_bonus: 1.0 },
            Self::Automaton => StatModifiers { hp: 1.1, attack: 1.0, defense: 1.25, mana: 0.7, speed: 1.0, xp_bonus: 1.0 },
            Self::Warforged => StatModifiers { hp: 1.2, attack: 1.2, defense: 1.3, mana: 0.6, speed: 0.9, xp_bonus: 1.0 },
            Self::Clockwork => StatModifiers { hp: 1.0, attack: 1.1, defense: 1.2, mana: 0.8, speed: 1.1, xp_bonus: 1.0 },

            // Elemental: +50% Elemental Dmg (simulated via higher mana/attack)
            Self::Fire => StatModifiers { hp: 0.9, attack: 1.3, defense: 0.8, mana: 1.3, speed: 1.1, xp_bonus: 1.0 },
            Self::Water => StatModifiers { hp: 1.1, attack: 0.9, defense: 1.1, mana: 1.3, speed: 1.0, xp_bonus: 1.0 },
            Self::Earth => StatModifiers { hp: 1.3, attack: 1.1, defense: 1.3, mana: 0.9, speed: 0.7, xp_bonus: 1.0 },
            Self::Air => StatModifiers { hp: 0.8, attack: 1.0, defense: 0.7, mana: 1.2, speed: 1.5, xp_bonus: 1.0 },

            // Celestial: +20% Holy Dmg
            Self::Angel => StatModifiers { hp: 1.1, attack: 1.1, defense: 1.1, mana: 1.2, speed: 1.1, xp_bonus: 1.0 },
            Self::Seraph => StatModifiers { hp: 1.0, attack: 1.2, defense: 1.0, mana: 1.3, speed: 1.2, xp_bonus: 1.0 },
            Self::Archon => StatModifiers { hp: 1.2, attack: 1.2, defense: 1.2, mana: 1.1, speed: 1.0, xp_bonus: 1.0 },
            Self::Nephilim => StatModifiers { hp: 1.1, attack: 1.1, defense: 1.1, mana: 1.1, speed: 1.1, xp_bonus: 1.1 },
            Self::Cherub => StatModifiers { hp: 1.1, attack: 0.9, defense: 1.3, mana: 1.1, speed: 1.0, xp_bonus: 1.0 },
            Self::Throne => StatModifiers { hp: 1.2, attack: 1.0, defense: 1.1, mana: 1.2, speed: 0.9, xp_bonus: 1.05 },
            Self::Dominion => StatModifiers { hp: 1.0, attack: 1.3, defense: 1.0, mana: 1.1, speed: 1.1, xp_bonus: 1.0 },
            Self::Virtue => StatModifiers { hp: 1.0, attack: 0.9, defense: 1.0, mana: 1.4, speed: 1.0, xp_bonus: 1.05 },

            // Additional Elf subspecies
            Self::BloodElf => StatModifiers { hp: 0.9, attack: 1.0, defense: 0.85, mana: 1.35, speed: 1.1, xp_bonus: 1.0 },
            Self::MoonElf => StatModifiers { hp: 0.95, attack: 0.95, defense: 0.9, mana: 1.3, speed: 1.15, xp_bonus: 1.0 },
            Self::SunElf => StatModifiers { hp: 1.0, attack: 1.05, defense: 0.95, mana: 1.25, speed: 1.1, xp_bonus: 1.0 },
            Self::WildElf => StatModifiers { hp: 1.05, attack: 1.1, defense: 0.95, mana: 1.0, speed: 1.2, xp_bonus: 1.0 },

            // Additional Dwarf subspecies
            Self::GoldDwarf => StatModifiers { hp: 1.1, attack: 0.95, defense: 1.15, mana: 0.9, speed: 0.9, xp_bonus: 1.15 },
            Self::IronDwarf => StatModifiers { hp: 1.15, attack: 1.1, defense: 1.25, mana: 0.8, speed: 0.85, xp_bonus: 1.0 },
            Self::RuneDwarf => StatModifiers { hp: 1.0, attack: 0.95, defense: 1.1, mana: 1.2, speed: 0.9, xp_bonus: 1.0 },
            Self::FireDwarf => StatModifiers { hp: 1.15, attack: 1.1, defense: 1.2, mana: 0.85, speed: 0.9, xp_bonus: 1.0 },

            // Additional Fae subspecies
            Self::Sylph => StatModifiers { hp: 0.65, attack: 0.75, defense: 0.65, mana: 1.3, speed: 1.5, xp_bonus: 1.0 },
            Self::WillOWisp => StatModifiers { hp: 0.5, attack: 0.7, defense: 1.6, mana: 1.2, speed: 1.4, xp_bonus: 1.0 },
            Self::DarkFairy => StatModifiers { hp: 0.75, attack: 1.0, defense: 0.7, mana: 1.4, speed: 1.2, xp_bonus: 1.0 },
            Self::Leprechaun => StatModifiers { hp: 0.7, attack: 0.8, defense: 0.75, mana: 1.2, speed: 1.3, xp_bonus: 1.2 },

            // Additional Human subspecies
            Self::Northerner => StatModifiers { hp: 1.15, attack: 1.05, defense: 1.1, mana: 0.9, speed: 0.95, xp_bonus: 1.1 },
            Self::Southerner => StatModifiers { hp: 1.0, attack: 1.0, defense: 1.0, mana: 1.05, speed: 1.1, xp_bonus: 1.1 },
            Self::Imperial => StatModifiers { hp: 1.05, attack: 1.1, defense: 1.1, mana: 0.95, speed: 1.0, xp_bonus: 1.1 },
            Self::Nomad => StatModifiers { hp: 1.05, attack: 1.0, defense: 0.95, mana: 1.0, speed: 1.15, xp_bonus: 1.15 },

            // Dragonborn evolution subspecies
            Self::Drake => StatModifiers { hp: 1.15, attack: 1.1, defense: 1.1, mana: 1.0, speed: 1.0, xp_bonus: 1.0 },
            Self::Wyrm => StatModifiers { hp: 1.2, attack: 1.05, defense: 1.1, mana: 1.3, speed: 0.85, xp_bonus: 1.0 },
            Self::TrueDragon => StatModifiers { hp: 1.3, attack: 1.2, defense: 1.2, mana: 1.2, speed: 1.0, xp_bonus: 0.9 },
            Self::ElderDragon => StatModifiers { hp: 1.25, attack: 1.1, defense: 1.3, mana: 1.35, speed: 0.85, xp_bonus: 0.9 },
            Self::DragonGod => StatModifiers { hp: 1.5, attack: 1.4, defense: 1.4, mana: 1.4, speed: 1.1, xp_bonus: 0.8 },
        }
    }

    pub fn racial_abilities(&self) -> Vec<RacialAbility> {
        match self {
            Self::Vampire => vec![RacialAbility::LifeDrain, RacialAbility::BatForm, RacialAbility::MistForm],
            Self::Lich => vec![RacialAbility::Phylactery, RacialAbility::SoulDrain, RacialAbility::UndeadCommand],
            Self::FireDrake => vec![RacialAbility::FireBreath, RacialAbility::HeatAura],
            Self::IceDrake => vec![RacialAbility::IceBreath, RacialAbility::FrostAura],
            Self::StormDrake => vec![RacialAbility::LightningBreath, RacialAbility::StormCall],
            Self::ShadowDrake => vec![RacialAbility::ShadowMeld, RacialAbility::DarkVision],
            Self::Pixie => vec![RacialAbility::Flight, RacialAbility::Invisibility, RacialAbility::GlamourMagic],
            Self::Noble => vec![RacialAbility::Leadership],
            Self::Barbarian => vec![RacialAbility::BloodRage],
            Self::Ghost => vec![RacialAbility::Incorporeal, RacialAbility::PhaseShift],
            Self::Wolf => vec![RacialAbility::PackTactics, RacialAbility::Howl, RacialAbility::Tracking],
            Self::Cat => vec![RacialAbility::NightVision, RacialAbility::NineLives, RacialAbility::AlwaysLandOnFeet],
            Self::SeaElf => vec![RacialAbility::WaterBreathing, RacialAbility::AquaticSpeed],
            Self::Deep => vec![RacialAbility::DarkVision, RacialAbility::StoneCunning],
            Self::Frost => vec![RacialAbility::ColdImmunity, RacialAbility::IceWalk],
            // Additional Celestial subspecies
            Self::Cherub => vec![RacialAbility::DivineShield, RacialAbility::Flight],
            Self::Throne => vec![RacialAbility::DivineAuthority, RacialAbility::Leadership],
            Self::Dominion => vec![RacialAbility::HolySmite, RacialAbility::DivineAuthority],
            Self::Virtue => vec![RacialAbility::HealingLight, RacialAbility::Flight],
            // Additional Elf subspecies
            Self::BloodElf => vec![RacialAbility::ManaDrain, RacialAbility::ArcaneHunger],
            Self::MoonElf => vec![RacialAbility::LunarMagic, RacialAbility::NightVision],
            Self::SunElf => vec![RacialAbility::SolarMagic, RacialAbility::HeatAura],
            Self::WildElf => vec![RacialAbility::NatureSurvival, RacialAbility::Tracking],
            // Additional Dwarf subspecies
            Self::GoldDwarf => vec![RacialAbility::GoldSense, RacialAbility::StoneCunning],
            Self::IronDwarf => vec![RacialAbility::StoneCunning],
            Self::RuneDwarf => vec![RacialAbility::RuneForge, RacialAbility::EnchantWeapon],
            Self::FireDwarf => vec![RacialAbility::FireImmunity, RacialAbility::HeatAura],
            // Additional Fae subspecies
            Self::Sylph => vec![RacialAbility::Flight, RacialAbility::WindDash],
            Self::WillOWisp => vec![RacialAbility::WillOWispLight, RacialAbility::Incorporeal],
            Self::DarkFairy => vec![RacialAbility::ShadowCurse, RacialAbility::ShadowMeld],
            Self::Leprechaun => vec![RacialAbility::LuckAura, RacialAbility::Invisibility],
            // Additional Human subspecies
            Self::Northerner => vec![RacialAbility::ColdResistance],
            Self::Southerner => vec![RacialAbility::HeatResistance],
            Self::Imperial => vec![RacialAbility::MilitaryDiscipline, RacialAbility::Leadership],
            Self::Nomad => vec![RacialAbility::WandererInstinct, RacialAbility::Tracking],
            // Dragonborn evolution subspecies
            Self::Drake => vec![RacialAbility::FireBreath],
            Self::Wyrm => vec![RacialAbility::FireBreath, RacialAbility::AncientWisdom],
            Self::TrueDragon => vec![RacialAbility::FireBreath, RacialAbility::DragonForm, RacialAbility::Flight],
            Self::ElderDragon => vec![RacialAbility::FireBreath, RacialAbility::AncientWisdom, RacialAbility::DragonForm, RacialAbility::Flight],
            Self::DragonGod => vec![RacialAbility::FireBreath, RacialAbility::DragonForm, RacialAbility::Transcendence, RacialAbility::Flight, RacialAbility::AncientWisdom],
            _ => vec![],
        }
    }
}

/// Stat modifiers from species/subspecies
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct StatModifiers {
    pub hp: f32,
    pub attack: f32,
    pub defense: f32,
    pub mana: f32,
    pub speed: f32,
    pub xp_bonus: f32,
}

impl StatModifiers {
    pub fn combine(&self, other: &StatModifiers) -> StatModifiers {
        StatModifiers {
            hp: self.hp * other.hp,
            attack: self.attack * other.attack,
            defense: self.defense * other.defense,
            mana: self.mana * other.mana,
            speed: self.speed * other.speed,
            xp_bonus: self.xp_bonus * other.xp_bonus,
        }
    }

    pub fn apply_to_stats(&self, base_hp: i32, base_atk: i32, base_def: i32, base_mana: i32, base_spd: i32) -> (i32, i32, i32, i32, i32) {
        (
            (base_hp as f32 * self.hp) as i32,
            (base_atk as f32 * self.attack) as i32,
            (base_def as f32 * self.defense) as i32,
            (base_mana as f32 * self.mana) as i32,
            (base_spd as f32 * self.speed) as i32,
        )
    }
}

/// Damage types for resistances/weaknesses
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageType {
    Physical,
    Fire,
    Cold,
    Lightning,
    Poison,
    Necrotic,
    Radiant,
    Psychic,
    Arcane,
    Shadow,
    Nature,
    Force,
}

/// Racial abilities unique to certain subspecies
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RacialAbility {
    // Movement
    Flight,
    Burrow,
    Swim,
    PhaseShift,
    Incorporeal,
    AquaticSpeed,
    IceWalk,

    // Transformation
    Shapeshift,
    Mimic,
    BatForm,
    MistForm,

    // Vision
    DarkVision,
    NightVision,
    TrueSight,

    // Combat
    FireBreath,
    IceBreath,
    LightningBreath,
    PoisonBite,
    Constrict,
    ParalyzingTouch,
    BloodRage,
    LastStand,
    PackTactics,
    Howl,

    // Magic
    LifeDrain,
    ManaDrain,
    SoulDrain,
    Charm,
    Seduction,
    GlamourMagic,
    UndeadCommand,
    ArcaneHunger,
    Phylactery,

    // Survival
    Regeneration,
    NineLives,
    AlwaysLandOnFeet,
    CorpseEater,
    WaterBreathing,
    ColdImmunity,
    FireImmunity,

    // Utility
    Tracking,
    StoneCunning,
    ShadowMeld,
    Invisibility,
    HeatAura,
    FrostAura,
    StormCall,

    // Social
    Leadership,

    // Additional abilities for new subspecies
    DivineShield,
    DivineAuthority,
    HolySmite,
    HealingLight,
    LunarMagic,
    SolarMagic,
    NatureSurvival,
    RuneForge,
    EnchantWeapon,
    GoldSense,
    WindDash,
    WillOWispLight,
    ShadowCurse,
    LuckAura,
    ColdResistance,
    HeatResistance,
    MilitaryDiscipline,
    WandererInstinct,
    DragonForm,
    AncientWisdom,
    Transcendence,
}

impl RacialAbility {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Flight => "Flight",
            Self::Burrow => "Burrow",
            Self::Swim => "Swim",
            Self::PhaseShift => "Phase Shift",
            Self::Incorporeal => "Incorporeal",
            Self::AquaticSpeed => "Aquatic Speed",
            Self::IceWalk => "Ice Walk",
            Self::Shapeshift => "Shapeshift",
            Self::Mimic => "Mimic",
            Self::BatForm => "Bat Form",
            Self::MistForm => "Mist Form",
            Self::DarkVision => "Darkvision",
            Self::NightVision => "Night Vision",
            Self::TrueSight => "True Sight",
            Self::FireBreath => "Fire Breath",
            Self::IceBreath => "Ice Breath",
            Self::LightningBreath => "Lightning Breath",
            Self::PoisonBite => "Poison Bite",
            Self::Constrict => "Constrict",
            Self::ParalyzingTouch => "Paralyzing Touch",
            Self::BloodRage => "Blood Rage",
            Self::LastStand => "Last Stand",
            Self::PackTactics => "Pack Tactics",
            Self::Howl => "Howl",
            Self::LifeDrain => "Life Drain",
            Self::ManaDrain => "Mana Drain",
            Self::SoulDrain => "Soul Drain",
            Self::Charm => "Charm",
            Self::Seduction => "Seduction",
            Self::GlamourMagic => "Glamour Magic",
            Self::UndeadCommand => "Command Undead",
            Self::ArcaneHunger => "Arcane Hunger",
            Self::Phylactery => "Phylactery",
            Self::Regeneration => "Regeneration",
            Self::NineLives => "Nine Lives",
            Self::AlwaysLandOnFeet => "Always Land on Feet",
            Self::CorpseEater => "Corpse Eater",
            Self::WaterBreathing => "Water Breathing",
            Self::ColdImmunity => "Cold Immunity",
            Self::FireImmunity => "Fire Immunity",
            Self::Tracking => "Tracking",
            Self::StoneCunning => "Stone Cunning",
            Self::ShadowMeld => "Shadow Meld",
            Self::Invisibility => "Invisibility",
            Self::HeatAura => "Heat Aura",
            Self::FrostAura => "Frost Aura",
            Self::StormCall => "Storm Call",
            Self::Leadership => "Leadership",
            // Additional abilities for new subspecies
            Self::DivineShield => "Divine Shield",
            Self::DivineAuthority => "Divine Authority",
            Self::HolySmite => "Holy Smite",
            Self::HealingLight => "Healing Light",
            Self::LunarMagic => "Lunar Magic",
            Self::SolarMagic => "Solar Magic",
            Self::NatureSurvival => "Nature Survival",
            Self::RuneForge => "Rune Forge",
            Self::EnchantWeapon => "Enchant Weapon",
            Self::GoldSense => "Gold Sense",
            Self::WindDash => "Wind Dash",
            Self::WillOWispLight => "Will-o'-Wisp Light",
            Self::ShadowCurse => "Shadow Curse",
            Self::LuckAura => "Luck Aura",
            Self::ColdResistance => "Cold Resistance",
            Self::HeatResistance => "Heat Resistance",
            Self::MilitaryDiscipline => "Military Discipline",
            Self::WandererInstinct => "Wanderer Instinct",
            Self::DragonForm => "Dragon Form",
            Self::AncientWisdom => "Ancient Wisdom",
            Self::Transcendence => "Transcendence",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Flight => "Can fly over obstacles and water.",
            Self::Burrow => "Can tunnel through soft ground.",
            Self::Swim => "Can move freely through water.",
            Self::PhaseShift => "Can phase through walls temporarily.",
            Self::Incorporeal => "Physical attacks have 50% miss chance.",
            Self::AquaticSpeed => "Move twice as fast in water.",
            Self::IceWalk => "Move normally on ice, immune to slipping.",
            Self::Shapeshift => "Transform into other creatures.",
            Self::Mimic => "Copy abilities of nearby creatures.",
            Self::BatForm => "Transform into a bat for flight.",
            Self::MistForm => "Become mist to avoid attacks.",
            Self::DarkVision => "See in complete darkness.",
            Self::NightVision => "See well in low light.",
            Self::TrueSight => "See through illusions and invisibility.",
            Self::FireBreath => "Breathe fire in a cone.",
            Self::IceBreath => "Breathe ice in a cone.",
            Self::LightningBreath => "Breathe lightning in a line.",
            Self::PoisonBite => "Attacks have a chance to poison.",
            Self::Constrict => "Grapple and crush enemies.",
            Self::ParalyzingTouch => "Touch attacks can paralyze.",
            Self::BloodRage => "Deal more damage when wounded.",
            Self::LastStand => "Survive one killing blow per day.",
            Self::PackTactics => "Bonus damage when allies are nearby.",
            Self::Howl => "Buff allies and frighten enemies.",
            Self::LifeDrain => "Heal by dealing damage.",
            Self::ManaDrain => "Steal mana from enemies.",
            Self::SoulDrain => "Absorb souls of the fallen.",
            Self::Charm => "Temporarily control enemy minds.",
            Self::Seduction => "Reduce enemy will to fight.",
            Self::GlamourMagic => "Create illusions and glamours.",
            Self::UndeadCommand => "Control undead creatures.",
            Self::ArcaneHunger => "Gain power from absorbing magic.",
            Self::Phylactery => "Cannot truly die while it exists.",
            Self::Regeneration => "Slowly heal over time.",
            Self::NineLives => "Automatically resurrect 9 times.",
            Self::AlwaysLandOnFeet => "Never take fall damage.",
            Self::CorpseEater => "Heal by eating corpses.",
            Self::WaterBreathing => "Breathe underwater.",
            Self::ColdImmunity => "Immune to cold damage.",
            Self::FireImmunity => "Immune to fire damage.",
            Self::Tracking => "Track creatures by scent.",
            Self::StoneCunning => "Detect traps and secret doors in stone.",
            Self::ShadowMeld => "Become invisible in shadows.",
            Self::Invisibility => "Turn invisible at will.",
            Self::HeatAura => "Nearby enemies take fire damage.",
            Self::FrostAura => "Nearby enemies are slowed.",
            Self::StormCall => "Summon lightning strikes.",
            Self::Leadership => "Inspire allies with commanding presence.",
            // Additional abilities for new subspecies
            Self::DivineShield => "Project a divine barrier that absorbs damage.",
            Self::DivineAuthority => "Command celestial forces with divine authority.",
            Self::HolySmite => "Strike enemies with concentrated divine wrath.",
            Self::HealingLight => "Channel divine light to heal wounds.",
            Self::LunarMagic => "Draw power from the moon, stronger at night.",
            Self::SolarMagic => "Channel the sun's energy, stronger during the day.",
            Self::NatureSurvival => "Expert at surviving in the wilderness.",
            Self::RuneForge => "Inscribe magical runes onto equipment.",
            Self::EnchantWeapon => "Imbue weapons with magical properties.",
            Self::GoldSense => "Detect gold and treasure nearby.",
            Self::WindDash => "Dash through the air at incredible speed.",
            Self::WillOWispLight => "Emit a mesmerizing light that confuses enemies.",
            Self::ShadowCurse => "Curse enemies with debilitating shadow magic.",
            Self::LuckAura => "Aura of luck increases gold and item find rate.",
            Self::ColdResistance => "Natural resistance to cold damage.",
            Self::HeatResistance => "Natural resistance to heat and fire damage.",
            Self::MilitaryDiscipline => "Trained combat discipline reduces damage taken.",
            Self::WandererInstinct => "Instinctive awareness of surroundings and dangers.",
            Self::DragonForm => "Transform into a full dragon temporarily.",
            Self::AncientWisdom => "Centuries of knowledge grant bonus mana regeneration.",
            Self::Transcendence => "Transcend mortal limits, boosting all abilities.",
        }
    }
}

/// Character's species configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterSpecies {
    pub species: Species,
    pub subspecies: Subspecies,
    pub abilities: Vec<RacialAbility>,
    pub resistances: HashMap<DamageType, f32>,  // 0.0 = immune, 0.5 = 50% resist, 1.0 = normal, 2.0 = vulnerable
    pub combined_modifiers: StatModifiers,
}

impl CharacterSpecies {
    pub fn new(subspecies: Subspecies) -> Self {
        let species = subspecies.parent_species();
        let species_mods = species.base_modifiers();
        let subspecies_mods = subspecies.modifiers();
        let combined = species_mods.combine(&subspecies_mods);

        let mut resistances = HashMap::new();
        for resist in species.innate_resistances() {
            resistances.insert(resist, 0.5);
        }
        for weakness in species.innate_weaknesses() {
            resistances.insert(weakness, 1.5);
        }

        Self {
            species,
            subspecies,
            abilities: subspecies.racial_abilities(),
            resistances,
            combined_modifiers: combined,
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} ({})", self.subspecies.name(), self.species.name())
    }

    pub fn has_ability(&self, ability: RacialAbility) -> bool {
        self.abilities.contains(&ability)
    }

    pub fn damage_multiplier(&self, damage_type: DamageType) -> f32 {
        *self.resistances.get(&damage_type).unwrap_or(&1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_species_subspecies() {
        for species in Species::all() {
            assert!(!species.subspecies().is_empty());
            for sub in species.subspecies() {
                assert_eq!(sub.parent_species(), *species);
            }
        }
    }

    #[test]
    fn test_character_species() {
        let char_species = CharacterSpecies::new(Subspecies::Vampire);
        assert_eq!(char_species.species, Species::Undead);
        assert!(char_species.has_ability(RacialAbility::LifeDrain));
    }

    #[test]
    fn test_stat_modifiers() {
        let mods = StatModifiers { hp: 1.2, attack: 1.1, defense: 1.0, mana: 0.9, speed: 1.0, xp_bonus: 1.0 };
        let (hp, atk, def, mana, spd) = mods.apply_to_stats(100, 10, 10, 50, 10);
        assert_eq!(hp, 120);
        assert_eq!(atk, 11);
        assert_eq!(mana, 45);
    }
}