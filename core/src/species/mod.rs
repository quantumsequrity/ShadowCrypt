//! Species and Subspecies System
//!
//! Comprehensive race system with 12 major species and 48+ subspecies,
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
                Subspecies::Highlander, Subspecies::Lowlander, Subspecies::Islander,
                Subspecies::Nomad, Subspecies::Imperial,
            ],
            Self::Elf => &[
                Subspecies::HighElf, Subspecies::WoodElf, Subspecies::DarkElf,
                Subspecies::SeaElf, Subspecies::BloodElf,
            ],
            Self::Dwarf => &[
                Subspecies::MountainDwarf, Subspecies::HillDwarf, Subspecies::DeepDwarf,
                Subspecies::ForgeDwarf, Subspecies::FrostDwarf,
            ],
            Self::Orc => &[
                Subspecies::GreyOrc, Subspecies::BlackOrc, Subspecies::BloodOrc,
                Subspecies::ShamanOrc, Subspecies::IronOrc,
            ],
            Self::Undead => &[
                Subspecies::Skeleton, Subspecies::Zombie, Subspecies::Ghoul,
                Subspecies::Vampire, Subspecies::Lich,
            ],
            Self::Demon => &[
                Subspecies::Imp, Subspecies::Succubus, Subspecies::Hellspawn,
                Subspecies::ShadowFiend, Subspecies::ArchDemon,
            ],
            Self::Beastkin => &[
                Subspecies::Wolfkin, Subspecies::Catfolk, Subspecies::Bearkin,
                Subspecies::Ratling, Subspecies::Serpentine,
            ],
            Self::Dragonborn => &[
                Subspecies::FireDrake, Subspecies::IceDrake, Subspecies::StormDrake,
                Subspecies::ShadowDrake, Subspecies::GoldenDrake,
            ],
            Self::Fae => &[
                Subspecies::Pixie, Subspecies::Sprite, Subspecies::Dryad,
                Subspecies::Wisp, Subspecies::Changeling,
            ],
            Self::Construct => &[
                Subspecies::Golem, Subspecies::Automaton, Subspecies::Warforged,
                Subspecies::Homunculus, Subspecies::ClockworkKnight,
            ],
            Self::Elemental => &[
                Subspecies::Flameling, Subspecies::Tideling, Subspecies::Stoneling,
                Subspecies::Windling, Subspecies::Voidling,
            ],
            Self::Celestial => &[
                Subspecies::Aasimar, Subspecies::Nephilim, Subspecies::Seraphim,
                Subspecies::Valkyrie, Subspecies::Archon,
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
    Highlander,   // Mountain warriors, +defense
    Lowlander,    // Plains dwellers, balanced
    Islander,     // Sea-faring, +speed
    Nomad,        // Desert wanderers, +stamina
    Imperial,     // Noble blood, +charisma

    // Elf subspecies
    HighElf,      // Arcane masters, +mana
    WoodElf,      // Forest dwellers, +speed, +stealth
    DarkElf,      // Underground, +dark magic
    SeaElf,       // Aquatic, water breathing
    BloodElf,     // Magic addicts, mana leech

    // Dwarf subspecies
    MountainDwarf, // Traditional, +defense
    HillDwarf,     // Surface dwellers, +hp
    DeepDwarf,     // Underdark, darkvision
    ForgeDwarf,    // Smiths, +crafting
    FrostDwarf,    // Ice mountains, cold resist

    // Orc subspecies
    GreyOrc,      // Common, balanced
    BlackOrc,     // Elite warriors, +attack
    BloodOrc,     // Berserkers, +damage when hurt
    ShamanOrc,    // Magic users, +mana
    IronOrc,      // Heavy armor, +defense

    // Undead subspecies
    Skeleton,     // Basic, immune to bleed
    Zombie,       // Tough, +hp, slow
    Ghoul,        // Fast, paralysis touch
    Vampire,      // Life drain, sun weakness
    Lich,         // Powerful mage, phylactery

    // Demon subspecies
    Imp,          // Small, fast, trickster
    Succubus,     // Charmer, life drain
    Hellspawn,    // Fire demon, burn attacks
    ShadowFiend,  // Shadow magic, stealth
    ArchDemon,    // Powerful, all demon abilities

    // Beastkin subspecies
    Wolfkin,      // Pack tactics, tracking
    Catfolk,      // Agile, night vision
    Bearkin,      // Strong, rage ability
    Ratling,      // Sneaky, disease resist
    Serpentine,   // Poison, flexible

    // Dragonborn subspecies
    FireDrake,    // Fire breath, fire resist
    IceDrake,     // Ice breath, cold resist
    StormDrake,   // Lightning breath, storm resist
    ShadowDrake,  // Shadow breath, stealth
    GoldenDrake,  // Radiant breath, charisma

    // Fae subspecies
    Pixie,        // Tiny, flying, illusions
    Sprite,       // Nature magic, healing
    Dryad,        // Tree-bound, plant control
    Wisp,         // Ethereal, phase through walls
    Changeling,   // Shapeshifter, mimic

    // Construct subspecies
    Golem,        // Stone/clay, high defense
    Automaton,    // Metal, balanced
    Warforged,    // Battle construct, weapons
    Homunculus,   // Small, familiar bond
    ClockworkKnight, // Mechanical, precision

    // Elemental subspecies
    Flameling,    // Fire elemental
    Tideling,     // Water elemental
    Stoneling,    // Earth elemental
    Windling,     // Air elemental
    Voidling,     // Void/dark elemental

    // Celestial subspecies
    Aasimar,      // Human-angel hybrid
    Nephilim,     // Fallen angel offspring
    Seraphim,     // Pure angel form
    Valkyrie,     // Warrior angel
    Archon,       // Justice incarnate
}

impl Subspecies {
    pub fn name(&self) -> &'static str {
        match self {
            // Humans
            Self::Highlander => "Highlander",
            Self::Lowlander => "Lowlander",
            Self::Islander => "Islander",
            Self::Nomad => "Nomad",
            Self::Imperial => "Imperial",
            // Elves
            Self::HighElf => "High Elf",
            Self::WoodElf => "Wood Elf",
            Self::DarkElf => "Dark Elf",
            Self::SeaElf => "Sea Elf",
            Self::BloodElf => "Blood Elf",
            // Dwarves
            Self::MountainDwarf => "Mountain Dwarf",
            Self::HillDwarf => "Hill Dwarf",
            Self::DeepDwarf => "Deep Dwarf",
            Self::ForgeDwarf => "Forge Dwarf",
            Self::FrostDwarf => "Frost Dwarf",
            // Orcs
            Self::GreyOrc => "Grey Orc",
            Self::BlackOrc => "Black Orc",
            Self::BloodOrc => "Blood Orc",
            Self::ShamanOrc => "Shaman Orc",
            Self::IronOrc => "Iron Orc",
            // Undead
            Self::Skeleton => "Skeleton",
            Self::Zombie => "Zombie",
            Self::Ghoul => "Ghoul",
            Self::Vampire => "Vampire",
            Self::Lich => "Lich",
            // Demons
            Self::Imp => "Imp",
            Self::Succubus => "Succubus",
            Self::Hellspawn => "Hellspawn",
            Self::ShadowFiend => "Shadow Fiend",
            Self::ArchDemon => "Arch Demon",
            // Beastkin
            Self::Wolfkin => "Wolfkin",
            Self::Catfolk => "Catfolk",
            Self::Bearkin => "Bearkin",
            Self::Ratling => "Ratling",
            Self::Serpentine => "Serpentine",
            // Dragonborn
            Self::FireDrake => "Fire Drake",
            Self::IceDrake => "Ice Drake",
            Self::StormDrake => "Storm Drake",
            Self::ShadowDrake => "Shadow Drake",
            Self::GoldenDrake => "Golden Drake",
            // Fae
            Self::Pixie => "Pixie",
            Self::Sprite => "Sprite",
            Self::Dryad => "Dryad",
            Self::Wisp => "Wisp",
            Self::Changeling => "Changeling",
            // Constructs
            Self::Golem => "Golem",
            Self::Automaton => "Automaton",
            Self::Warforged => "Warforged",
            Self::Homunculus => "Homunculus",
            Self::ClockworkKnight => "Clockwork Knight",
            // Elementals
            Self::Flameling => "Flameling",
            Self::Tideling => "Tideling",
            Self::Stoneling => "Stoneling",
            Self::Windling => "Windling",
            Self::Voidling => "Voidling",
            // Celestials
            Self::Aasimar => "Aasimar",
            Self::Nephilim => "Nephilim",
            Self::Seraphim => "Seraphim",
            Self::Valkyrie => "Valkyrie",
            Self::Archon => "Archon",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Highlander => "Hardy mountain folk with unmatched endurance.",
            Self::Lowlander => "Adaptable plains dwellers, jack of all trades.",
            Self::Islander => "Swift sailors with exceptional reflexes.",
            Self::Nomad => "Desert wanderers who thrive in harsh conditions.",
            Self::Imperial => "Noble-blooded with natural leadership.",
            Self::HighElf => "Masters of arcane magic with ancient knowledge.",
            Self::WoodElf => "Forest guardians with unparalleled stealth.",
            Self::DarkElf => "Shadow dwellers wielding dark magic.",
            Self::SeaElf => "Aquatic elves who breathe underwater.",
            Self::BloodElf => "Magic-addicted elves who drain mana from foes.",
            Self::MountainDwarf => "Traditional dwarves with stone-hard resolve.",
            Self::HillDwarf => "Surface dwarves with remarkable vitality.",
            Self::DeepDwarf => "Underdark dwellers with superior darkvision.",
            Self::ForgeDwarf => "Master smiths who craft legendary items.",
            Self::FrostDwarf => "Ice mountain dwarves immune to cold.",
            Self::GreyOrc => "Common orcs with balanced abilities.",
            Self::BlackOrc => "Elite orc warriors of terrifying strength.",
            Self::BloodOrc => "Berserkers who grow stronger when wounded.",
            Self::ShamanOrc => "Rare magic-wielding orcs.",
            Self::IronOrc => "Heavily armored orc defenders.",
            Self::Skeleton => "Animated bones immune to bleeding.",
            Self::Zombie => "Shambling corpses of great resilience.",
            Self::Ghoul => "Fast undead with paralyzing touch.",
            Self::Vampire => "Immortal blood drinkers with many powers.",
            Self::Lich => "Undead archmages of immense power.",
            Self::Imp => "Small trickster demons, quick and cunning.",
            Self::Succubus => "Charming demons who drain life force.",
            Self::Hellspawn => "Fire demons born of infernal flames.",
            Self::ShadowFiend => "Shadow demons masters of darkness.",
            Self::ArchDemon => "Powerful demon lords with all abilities.",
            Self::Wolfkin => "Wolf-human hybrids with pack instincts.",
            Self::Catfolk => "Feline humanoids with superior agility.",
            Self::Bearkin => "Bear-folk with tremendous strength.",
            Self::Ratling => "Rat-folk survivors of the underworld.",
            Self::Serpentine => "Snake-folk with venomous abilities.",
            Self::FireDrake => "Fire-breathing dragon descendants.",
            Self::IceDrake => "Frost-breathing dragon descendants.",
            Self::StormDrake => "Lightning-breathing dragon descendants.",
            Self::ShadowDrake => "Shadow-breathing dragon descendants.",
            Self::GoldenDrake => "Radiant dragon descendants of legend.",
            Self::Pixie => "Tiny flying fae with illusion magic.",
            Self::Sprite => "Nature spirits with healing powers.",
            Self::Dryad => "Tree-bound spirits of the forest.",
            Self::Wisp => "Ethereal beings who phase through matter.",
            Self::Changeling => "Shapeshifters who mimic any form.",
            Self::Golem => "Stone or clay constructs of great defense.",
            Self::Automaton => "Mechanical beings of balanced design.",
            Self::Warforged => "Battle constructs built for war.",
            Self::Homunculus => "Small familiar constructs.",
            Self::ClockworkKnight => "Precision mechanical warriors.",
            Self::Flameling => "Beings of pure fire.",
            Self::Tideling => "Beings of pure water.",
            Self::Stoneling => "Beings of pure earth.",
            Self::Windling => "Beings of pure air.",
            Self::Voidling => "Beings of the void between worlds.",
            Self::Aasimar => "Mortals touched by angelic blood.",
            Self::Nephilim => "Offspring of fallen angels.",
            Self::Seraphim => "Pure angelic beings of light.",
            Self::Valkyrie => "Warrior angels who choose the slain.",
            Self::Archon => "Embodiments of divine justice.",
        }
    }

    pub fn parent_species(&self) -> Species {
        match self {
            Self::Highlander | Self::Lowlander | Self::Islander |
            Self::Nomad | Self::Imperial => Species::Human,

            Self::HighElf | Self::WoodElf | Self::DarkElf |
            Self::SeaElf | Self::BloodElf => Species::Elf,

            Self::MountainDwarf | Self::HillDwarf | Self::DeepDwarf |
            Self::ForgeDwarf | Self::FrostDwarf => Species::Dwarf,

            Self::GreyOrc | Self::BlackOrc | Self::BloodOrc |
            Self::ShamanOrc | Self::IronOrc => Species::Orc,

            Self::Skeleton | Self::Zombie | Self::Ghoul |
            Self::Vampire | Self::Lich => Species::Undead,

            Self::Imp | Self::Succubus | Self::Hellspawn |
            Self::ShadowFiend | Self::ArchDemon => Species::Demon,

            Self::Wolfkin | Self::Catfolk | Self::Bearkin |
            Self::Ratling | Self::Serpentine => Species::Beastkin,

            Self::FireDrake | Self::IceDrake | Self::StormDrake |
            Self::ShadowDrake | Self::GoldenDrake => Species::Dragonborn,

            Self::Pixie | Self::Sprite | Self::Dryad |
            Self::Wisp | Self::Changeling => Species::Fae,

            Self::Golem | Self::Automaton | Self::Warforged |
            Self::Homunculus | Self::ClockworkKnight => Species::Construct,

            Self::Flameling | Self::Tideling | Self::Stoneling |
            Self::Windling | Self::Voidling => Species::Elemental,

            Self::Aasimar | Self::Nephilim | Self::Seraphim |
            Self::Valkyrie | Self::Archon => Species::Celestial,
        }
    }

    pub fn modifiers(&self) -> StatModifiers {
        // Subspecies modifiers stack with species modifiers
        match self {
            // Humans
            Self::Highlander => StatModifiers { hp: 1.05, attack: 1.0, defense: 1.10, mana: 0.95, speed: 0.95, xp_bonus: 1.0 },
            Self::Lowlander => StatModifiers { hp: 1.0, attack: 1.0, defense: 1.0, mana: 1.0, speed: 1.0, xp_bonus: 1.05 },
            Self::Islander => StatModifiers { hp: 0.95, attack: 1.0, defense: 0.95, mana: 1.0, speed: 1.15, xp_bonus: 1.0 },
            Self::Nomad => StatModifiers { hp: 1.10, attack: 1.0, defense: 0.95, mana: 0.95, speed: 1.05, xp_bonus: 1.0 },
            Self::Imperial => StatModifiers { hp: 1.0, attack: 0.95, defense: 1.0, mana: 1.05, speed: 1.0, xp_bonus: 1.10 },

            // Elves
            Self::HighElf => StatModifiers { hp: 0.95, attack: 0.90, defense: 0.90, mana: 1.25, speed: 1.0, xp_bonus: 1.0 },
            Self::WoodElf => StatModifiers { hp: 1.0, attack: 1.05, defense: 0.95, mana: 1.0, speed: 1.15, xp_bonus: 1.0 },
            Self::DarkElf => StatModifiers { hp: 0.95, attack: 1.10, defense: 0.90, mana: 1.15, speed: 1.05, xp_bonus: 1.0 },
            Self::SeaElf => StatModifiers { hp: 1.05, attack: 1.0, defense: 1.0, mana: 1.05, speed: 1.10, xp_bonus: 1.0 },
            Self::BloodElf => StatModifiers { hp: 0.90, attack: 1.0, defense: 0.85, mana: 1.30, speed: 1.0, xp_bonus: 1.0 },

            // Dwarves
            Self::MountainDwarf => StatModifiers { hp: 1.05, attack: 1.05, defense: 1.15, mana: 0.90, speed: 0.95, xp_bonus: 1.0 },
            Self::HillDwarf => StatModifiers { hp: 1.15, attack: 1.0, defense: 1.05, mana: 0.95, speed: 1.0, xp_bonus: 1.0 },
            Self::DeepDwarf => StatModifiers { hp: 1.0, attack: 1.10, defense: 1.10, mana: 0.95, speed: 0.90, xp_bonus: 1.0 },
            Self::ForgeDwarf => StatModifiers { hp: 1.05, attack: 1.0, defense: 1.05, mana: 0.90, speed: 0.95, xp_bonus: 1.05 },
            Self::FrostDwarf => StatModifiers { hp: 1.10, attack: 1.0, defense: 1.10, mana: 0.85, speed: 0.90, xp_bonus: 1.0 },

            // Orcs
            Self::GreyOrc => StatModifiers { hp: 1.0, attack: 1.05, defense: 1.0, mana: 0.95, speed: 1.0, xp_bonus: 1.0 },
            Self::BlackOrc => StatModifiers { hp: 1.10, attack: 1.20, defense: 1.05, mana: 0.80, speed: 0.95, xp_bonus: 0.95 },
            Self::BloodOrc => StatModifiers { hp: 0.95, attack: 1.15, defense: 0.90, mana: 0.85, speed: 1.10, xp_bonus: 1.0 },
            Self::ShamanOrc => StatModifiers { hp: 0.90, attack: 0.95, defense: 0.90, mana: 1.30, speed: 1.0, xp_bonus: 1.0 },
            Self::IronOrc => StatModifiers { hp: 1.15, attack: 1.0, defense: 1.25, mana: 0.75, speed: 0.85, xp_bonus: 1.0 },

            // Undead
            Self::Skeleton => StatModifiers { hp: 0.85, attack: 1.0, defense: 1.05, mana: 0.90, speed: 1.05, xp_bonus: 1.0 },
            Self::Zombie => StatModifiers { hp: 1.30, attack: 0.95, defense: 1.10, mana: 0.70, speed: 0.75, xp_bonus: 1.0 },
            Self::Ghoul => StatModifiers { hp: 0.95, attack: 1.10, defense: 0.90, mana: 0.85, speed: 1.20, xp_bonus: 1.0 },
            Self::Vampire => StatModifiers { hp: 1.0, attack: 1.15, defense: 1.0, mana: 1.15, speed: 1.15, xp_bonus: 1.0 },
            Self::Lich => StatModifiers { hp: 0.80, attack: 0.90, defense: 0.85, mana: 1.50, speed: 0.90, xp_bonus: 1.0 },

            // Demons
            Self::Imp => StatModifiers { hp: 0.75, attack: 0.90, defense: 0.80, mana: 1.10, speed: 1.30, xp_bonus: 1.0 },
            Self::Succubus => StatModifiers { hp: 0.90, attack: 1.0, defense: 0.85, mana: 1.20, speed: 1.10, xp_bonus: 1.0 },
            Self::Hellspawn => StatModifiers { hp: 1.10, attack: 1.20, defense: 1.0, mana: 1.05, speed: 1.0, xp_bonus: 1.0 },
            Self::ShadowFiend => StatModifiers { hp: 0.90, attack: 1.15, defense: 0.85, mana: 1.15, speed: 1.20, xp_bonus: 1.0 },
            Self::ArchDemon => StatModifiers { hp: 1.15, attack: 1.15, defense: 1.10, mana: 1.20, speed: 1.05, xp_bonus: 0.90 },

            // Beastkin
            Self::Wolfkin => StatModifiers { hp: 1.0, attack: 1.10, defense: 0.95, mana: 0.90, speed: 1.15, xp_bonus: 1.0 },
            Self::Catfolk => StatModifiers { hp: 0.90, attack: 1.05, defense: 0.85, mana: 0.95, speed: 1.25, xp_bonus: 1.0 },
            Self::Bearkin => StatModifiers { hp: 1.20, attack: 1.15, defense: 1.10, mana: 0.80, speed: 0.90, xp_bonus: 1.0 },
            Self::Ratling => StatModifiers { hp: 0.85, attack: 0.95, defense: 0.85, mana: 0.95, speed: 1.20, xp_bonus: 1.05 },
            Self::Serpentine => StatModifiers { hp: 0.95, attack: 1.10, defense: 0.90, mana: 1.05, speed: 1.10, xp_bonus: 1.0 },

            // Dragonborn
            Self::FireDrake => StatModifiers { hp: 1.05, attack: 1.15, defense: 1.05, mana: 1.0, speed: 1.0, xp_bonus: 1.0 },
            Self::IceDrake => StatModifiers { hp: 1.10, attack: 1.0, defense: 1.15, mana: 1.0, speed: 0.95, xp_bonus: 1.0 },
            Self::StormDrake => StatModifiers { hp: 1.0, attack: 1.10, defense: 1.0, mana: 1.10, speed: 1.10, xp_bonus: 1.0 },
            Self::ShadowDrake => StatModifiers { hp: 0.95, attack: 1.10, defense: 0.95, mana: 1.10, speed: 1.15, xp_bonus: 1.0 },
            Self::GoldenDrake => StatModifiers { hp: 1.05, attack: 1.05, defense: 1.10, mana: 1.15, speed: 1.0, xp_bonus: 1.10 },

            // Fae
            Self::Pixie => StatModifiers { hp: 0.60, attack: 0.75, defense: 0.60, mana: 1.40, speed: 1.50, xp_bonus: 1.0 },
            Self::Sprite => StatModifiers { hp: 0.75, attack: 0.80, defense: 0.70, mana: 1.30, speed: 1.30, xp_bonus: 1.0 },
            Self::Dryad => StatModifiers { hp: 0.90, attack: 0.85, defense: 0.90, mana: 1.25, speed: 1.10, xp_bonus: 1.0 },
            Self::Wisp => StatModifiers { hp: 0.65, attack: 0.70, defense: 0.50, mana: 1.50, speed: 1.40, xp_bonus: 1.0 },
            Self::Changeling => StatModifiers { hp: 0.85, attack: 0.95, defense: 0.85, mana: 1.15, speed: 1.15, xp_bonus: 1.05 },

            // Constructs
            Self::Golem => StatModifiers { hp: 1.40, attack: 1.0, defense: 1.35, mana: 0.40, speed: 0.70, xp_bonus: 1.0 },
            Self::Automaton => StatModifiers { hp: 1.20, attack: 1.05, defense: 1.20, mana: 0.60, speed: 0.90, xp_bonus: 1.0 },
            Self::Warforged => StatModifiers { hp: 1.25, attack: 1.15, defense: 1.15, mana: 0.50, speed: 0.95, xp_bonus: 1.0 },
            Self::Homunculus => StatModifiers { hp: 0.70, attack: 0.80, defense: 0.80, mana: 1.20, speed: 1.10, xp_bonus: 1.0 },
            Self::ClockworkKnight => StatModifiers { hp: 1.15, attack: 1.20, defense: 1.10, mana: 0.55, speed: 1.0, xp_bonus: 1.0 },

            // Elementals
            Self::Flameling => StatModifiers { hp: 0.85, attack: 1.25, defense: 0.80, mana: 1.20, speed: 1.15, xp_bonus: 1.0 },
            Self::Tideling => StatModifiers { hp: 1.0, attack: 1.0, defense: 1.0, mana: 1.30, speed: 1.05, xp_bonus: 1.0 },
            Self::Stoneling => StatModifiers { hp: 1.20, attack: 1.05, defense: 1.30, mana: 0.90, speed: 0.80, xp_bonus: 1.0 },
            Self::Windling => StatModifiers { hp: 0.75, attack: 0.95, defense: 0.70, mana: 1.20, speed: 1.40, xp_bonus: 1.0 },
            Self::Voidling => StatModifiers { hp: 0.90, attack: 1.20, defense: 0.85, mana: 1.35, speed: 1.10, xp_bonus: 1.0 },

            // Celestials
            Self::Aasimar => StatModifiers { hp: 1.05, attack: 1.0, defense: 1.05, mana: 1.15, speed: 1.05, xp_bonus: 1.05 },
            Self::Nephilim => StatModifiers { hp: 1.10, attack: 1.15, defense: 1.0, mana: 1.10, speed: 1.0, xp_bonus: 1.0 },
            Self::Seraphim => StatModifiers { hp: 0.95, attack: 0.95, defense: 0.95, mana: 1.40, speed: 1.15, xp_bonus: 1.0 },
            Self::Valkyrie => StatModifiers { hp: 1.10, attack: 1.15, defense: 1.10, mana: 1.05, speed: 1.10, xp_bonus: 1.0 },
            Self::Archon => StatModifiers { hp: 1.15, attack: 1.10, defense: 1.15, mana: 1.15, speed: 1.0, xp_bonus: 0.95 },
        }
    }

    pub fn racial_abilities(&self) -> Vec<RacialAbility> {
        match self {
            Self::Vampire => vec![RacialAbility::LifeDrain, RacialAbility::BatForm, RacialAbility::MistForm],
            Self::Lich => vec![RacialAbility::Phylactery, RacialAbility::SoulDrain, RacialAbility::UndeadCommand],
            Self::FireDrake => vec![RacialAbility::FireBreath, RacialAbility::HeatAura],
            Self::IceDrake => vec![RacialAbility::IceBreath, RacialAbility::FrostAura],
            Self::StormDrake => vec![RacialAbility::LightningBreath, RacialAbility::StormCall],
            Self::Pixie => vec![RacialAbility::Flight, RacialAbility::Invisibility, RacialAbility::GlamourMagic],
            Self::Wisp => vec![RacialAbility::PhaseShift, RacialAbility::Incorporeal],
            Self::Changeling => vec![RacialAbility::Shapeshift, RacialAbility::Mimic],
            Self::BloodOrc => vec![RacialAbility::BloodRage, RacialAbility::LastStand],
            Self::BloodElf => vec![RacialAbility::ManaDrain, RacialAbility::ArcaneHunger],
            Self::Ghoul => vec![RacialAbility::ParalyzingTouch, RacialAbility::CorpseEater],
            Self::Succubus => vec![RacialAbility::Charm, RacialAbility::LifeDrain, RacialAbility::Seduction],
            Self::ShadowFiend => vec![RacialAbility::ShadowMeld, RacialAbility::DarkVision],
            Self::Serpentine => vec![RacialAbility::PoisonBite, RacialAbility::Constrict],
            Self::Wolfkin => vec![RacialAbility::PackTactics, RacialAbility::Howl, RacialAbility::Tracking],
            Self::Catfolk => vec![RacialAbility::NightVision, RacialAbility::NineLives, RacialAbility::AlwaysLandOnFeet],
            Self::SeaElf => vec![RacialAbility::WaterBreathing, RacialAbility::AquaticSpeed],
            Self::DeepDwarf => vec![RacialAbility::DarkVision, RacialAbility::StoneCunning],
            Self::FrostDwarf => vec![RacialAbility::ColdImmunity, RacialAbility::IceWalk],
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
