//! Entity system: player, enemies, and their properties

use std::collections::HashMap;
use rand::prelude::*;
use serde::{Serialize, Deserialize};

use crate::classes::CharacterClass;
use crate::combat::StatusEffect;
use crate::items::{EquipSlot, Item, ItemKind};
use crate::magic::Skill;

/// All enemy types in the game
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
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
    BossGoblinKing,      // Level 5
    BossOrcWarlord,      // Level 10
    BossVampireLord,     // Level 15
    BossForestGuardian,  // Level 20
    BossIceDragon,       // Level 25
    BossDemonKing,       // Level 30

    // Mini-Bosses
    GoblinChampion,
    OrcBerserker,
    VampireElite,
    AncientWyrm,
    FrostLord,
    InfernalLord,
}

impl EnemyKind {
    /// Returns the glyph character for this enemy
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

    /// Returns a color index for this enemy (for UI rendering)
    pub fn color_index(&self) -> u8 {
        match self {
            Self::Rat | Self::Bat | Self::GiantRat | Self::CaveCrawler => 0,
            Self::Spider | Self::GiantSpider | Self::IceSpider => 12,
            Self::Goblin | Self::BossGoblinKing | Self::GoblinChampion | Self::Kobold => 5,
            Self::Skeleton | Self::Mummy | Self::BoneGolem => 2,
            Self::Orc | Self::BossOrcWarlord | Self::OrcBerserker | Self::Hobgoblin => 6,
            Self::Troll | Self::ForestTroll => 10,
            Self::CaveOgre | Self::CaveBear => 12,
            Self::Slime | Self::Mushroom => 5,
            Self::RockElemental => 1,
            Self::Zombie | Self::Ghoul => 6,
            Self::Ghost | Self::Wraith | Self::IceWraith | Self::Banshee => 1,
            Self::Vampire | Self::BossVampireLord | Self::VampireElite => 4,
            Self::DeathKnight => 14,
            Self::Wolf | Self::DireWolf | Self::FrostWolf => 1,
            Self::TreeEnt | Self::BossForestGuardian | Self::VenomousVine => 5,
            Self::Druid | Self::ForestSpirit => 6,
            Self::WildBoar => 12,
            Self::GiantWasp => 11,
            Self::IceElemental | Self::FrostGiant | Self::YetiWarrior | Self::BossIceDragon
            | Self::FrozenKnight | Self::Wendigo | Self::FrostLord => 9,
            Self::FireElemental | Self::LavaGolem | Self::Hellhound | Self::FireDrake
            | Self::MagmaSlime | Self::Salamander | Self::CinderWraith | Self::InfernalImp
            | Self::InfernalLord => 3,
            Self::Golem | Self::AncientGuardian | Self::CursedStatue => 11,
            Self::Sphinx | Self::Gargoyle => 11,
            Self::Lich | Self::MummyLord => 13,
            Self::ShadowAssassin => 0,
            Self::Demon | Self::DemonLord | Self::Balrog | Self::BossDemonKing
            | Self::PitFiend | Self::DoomGuard => 3,
            Self::Succubus | Self::ShadowDemon => 13,
            Self::AbyssalHorror => 4,
            Self::AncientWyrm => 5,
        }
    }

    /// Returns the display name of this enemy
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

    /// Returns base stats: (hp, attack, defense, xp_value)
    pub fn base_stats(&self) -> (i32, i32, i32, u32) {
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

    /// Returns whether this enemy is a boss
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

    /// Returns whether this enemy can poison
    pub fn can_poison(&self) -> bool {
        matches!(self, Self::Spider | Self::GiantSpider | Self::Slime
            | Self::Mushroom | Self::VenomousVine | Self::GiantWasp | Self::IceSpider)
    }

    /// Returns whether this enemy can burn
    pub fn can_burn(&self) -> bool {
        matches!(
            self,
            Self::FireElemental | Self::LavaGolem | Self::Hellhound | Self::FireDrake
            | Self::Demon | Self::DemonLord | Self::Balrog | Self::BossDemonKing
            | Self::MagmaSlime | Self::Salamander | Self::CinderWraith | Self::InfernalImp
            | Self::PitFiend | Self::InfernalLord
        )
    }

    /// Returns whether this enemy can freeze
    pub fn can_freeze(&self) -> bool {
        matches!(
            self,
            Self::IceElemental | Self::FrostGiant | Self::YetiWarrior
            | Self::IceWraith | Self::BossIceDragon | Self::FrostWolf
            | Self::IceSpider | Self::FrozenKnight | Self::Wendigo | Self::FrostLord
        )
    }

    /// Returns whether this enemy can cause bleeding
    pub fn can_bleed(&self) -> bool {
        matches!(
            self,
            Self::Wolf | Self::DireWolf | Self::Skeleton | Self::DeathKnight
            | Self::ShadowAssassin | Self::Vampire | Self::Ghoul
            | Self::BossVampireLord | Self::BossOrcWarlord
        )
    }

    /// Returns a random enemy for the given dungeon level
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

    /// Returns the boss for a given level, if any
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

/// An enemy instance
#[derive(Clone, Serialize, Deserialize)]
pub struct Enemy {
    pub x: usize,
    pub y: usize,
    pub kind: EnemyKind,
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub xp_value: u32,
    pub status_effects: HashMap<StatusEffect, u32>,
    pub last_seen_player: Option<(usize, usize)>,
}

impl Enemy {
    /// Create a new enemy
    pub fn new(x: usize, y: usize, kind: EnemyKind, level: u32) -> Self {
        let (base_hp, base_atk, base_def, base_xp) = kind.base_stats();
        let scale = 1.0 + (level as f32 * 0.1);
        let hp = (base_hp as f32 * scale) as i32;
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
        }
    }

    /// Returns whether the enemy is alive
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    /// Take damage and return actual damage dealt
    pub fn take_damage(&mut self, amount: i32) -> i32 {
        let actual = (amount - self.defense).max(1);
        self.hp -= actual;
        actual
    }

    /// Add a status effect
    pub fn add_status(&mut self, effect: StatusEffect, duration: u32) {
        self.status_effects.insert(effect, duration);
    }

    /// Check if enemy has a status effect
    pub fn has_status(&self, effect: StatusEffect) -> bool {
        self.status_effects.contains_key(&effect)
    }

    /// Tick all status effects and return damage events
    pub fn tick_status_effects(&mut self) -> Vec<(StatusEffect, i32)> {
        let mut damage_events = Vec::new();
        let mut to_remove = Vec::new();

        for (effect, duration) in self.status_effects.iter_mut() {
            match effect {
                StatusEffect::Poison => damage_events.push((*effect, 3)),
                StatusEffect::Burn => damage_events.push((*effect, 5)),
                StatusEffect::Bleed => damage_events.push((*effect, 2)),
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
}

/// The player character
#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    pub x: usize,
    pub y: usize,
    pub class: CharacterClass,
    pub hp: i32,
    pub max_hp: i32,
    pub mana: i32,
    pub max_mana: i32,
    pub base_attack: i32,
    pub base_defense: i32,
    pub speed: i32,
    pub gold: u32,
    pub level: u32,
    pub xp: u32,
    pub xp_to_level: u32,
    pub hunger: i32,
    pub max_hunger: i32,
    pub keys: u32,
    pub kills: u32,
    pub status_effects: HashMap<StatusEffect, u32>,
    pub equipment: HashMap<EquipSlot, Item>,
    pub inventory: Vec<Item>,
    pub skills: Vec<Skill>,
    pub active_skill: usize,
    pub minions: Vec<Enemy>,
}

impl Player {
    /// Create a new player
    pub fn new(x: usize, y: usize, class: CharacterClass) -> Self {
        let (hp, attack, defense, mana, speed) = class.base_stats();
        Self {
            x,
            y,
            class,
            hp,
            max_hp: hp,
            mana,
            max_mana: mana,
            base_attack: attack,
            base_defense: defense,
            speed,
            gold: 0,
            level: 1,
            xp: 0,
            xp_to_level: 100,
            hunger: 100,
            max_hunger: 100,
            keys: 0,
            kills: 0,
            status_effects: HashMap::new(),
            equipment: HashMap::new(),
            inventory: Vec::new(),
            skills: Skill::for_class(class),
            active_skill: 0,
            minions: Vec::new(),
        }
    }

    /// Calculate total attack
    pub fn total_attack(&self) -> i32 {
        let mut total = self.base_attack;
        for item in self.equipment.values() {
            let (atk, _, _, _) = item.stats();
            total += atk;
        }
        if self.has_status(StatusEffect::Strength) {
            total = (total as f32 * 1.5) as i32;
        }
        if self.has_status(StatusEffect::Weakness) {
            total = (total as f32 * 0.5) as i32;
        }
        total
    }

    /// Calculate total defense
    pub fn total_defense(&self) -> i32 {
        let mut total = self.base_defense;
        for item in self.equipment.values() {
            let (_, def, _, _) = item.stats();
            total += def;
        }
        if self.has_status(StatusEffect::Shield) {
            total += 10;
        }
        total
    }

    /// Calculate total max HP
    pub fn total_max_hp(&self) -> i32 {
        let mut total = self.max_hp;
        for item in self.equipment.values() {
            let (_, _, hp, _) = item.stats();
            total += hp;
        }
        total
    }

    /// Calculate total max mana
    pub fn total_max_mana(&self) -> i32 {
        let mut total = self.max_mana;
        for item in self.equipment.values() {
            let (_, _, _, mana) = item.stats();
            total += mana;
        }
        total
    }

    /// Gain XP and return true if leveled up
    pub fn gain_xp(&mut self, amount: u32) -> bool {
        self.xp += amount;
        if self.xp >= self.xp_to_level {
            self.xp -= self.xp_to_level;
            self.level += 1;
            self.xp_to_level = (self.xp_to_level as f32 * 1.4) as u32;
            self.max_hp += 8 + (self.level as i32 / 3);
            self.hp = self.total_max_hp();
            self.max_mana += 5;
            self.mana = self.total_max_mana();
            self.base_attack += 2;
            self.base_defense += 1;
            return true;
        }
        false
    }

    /// Heal the player
    pub fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.total_max_hp());
    }

    /// Restore mana
    pub fn restore_mana(&mut self, amount: i32) {
        self.mana = (self.mana + amount).min(self.total_max_mana());
    }

    /// Eat food
    pub fn eat(&mut self, food_value: i32) {
        self.hunger = (self.hunger + food_value).min(self.max_hunger);
    }

    /// Add a status effect
    pub fn add_status(&mut self, effect: StatusEffect, duration: u32) {
        self.status_effects.insert(effect, duration);
    }

    /// Check if player has a status effect
    pub fn has_status(&self, effect: StatusEffect) -> bool {
        self.status_effects.contains_key(&effect)
    }

    /// Remove a status effect
    pub fn remove_status(&mut self, effect: StatusEffect) {
        self.status_effects.remove(&effect);
    }

    /// Tick all status effects and return messages
    pub fn tick_status_effects(&mut self) -> Vec<String> {
        let mut messages = Vec::new();
        let mut to_remove = Vec::new();
        let mut damage = 0;
        let mut heal = 0;

        for (effect, duration) in self.status_effects.iter_mut() {
            match effect {
                StatusEffect::Poison => {
                    damage += 2;
                    messages.push("You take poison damage!".to_string());
                }
                StatusEffect::Burn => {
                    damage += 3;
                    messages.push("You are burning!".to_string());
                }
                StatusEffect::Bleed => {
                    damage += 1;
                    messages.push("You are bleeding!".to_string());
                }
                StatusEffect::Regeneration => {
                    heal += 3;
                }
                _ => {}
            }
            *duration = duration.saturating_sub(1);
            if *duration == 0 {
                to_remove.push(*effect);
                messages.push(format!("{} wore off.", effect.name()));
            }
        }

        self.hp -= damage;
        self.heal(heal);

        for effect in to_remove {
            self.status_effects.remove(&effect);
        }

        messages
    }

    /// Tick hunger and return optional message
    pub fn tick_hunger(&mut self) -> Option<String> {
        self.hunger -= 1;
        if self.hunger <= 0 {
            self.hp -= 1;
            Some("You are starving!".to_string())
        } else if self.hunger < 20 {
            Some("You are very hungry!".to_string())
        } else {
            None
        }
    }

    /// Equip an item and return the previously equipped item
    pub fn equip(&mut self, item: Item) -> Option<Item> {
        if let Some(slot) = item.kind.equip_slot() {
            // Handle rings specially - can wear two
            let actual_slot = if slot == EquipSlot::Ring1 {
                if self.equipment.contains_key(&EquipSlot::Ring1) && !self.equipment.contains_key(&EquipSlot::Ring2) {
                    EquipSlot::Ring2
                } else {
                    EquipSlot::Ring1
                }
            } else {
                slot
            };
            let old = self.equipment.remove(&actual_slot);
            self.equipment.insert(actual_slot, item);
            old
        } else {
            None
        }
    }

    /// Check if player can use their current skill
    pub fn can_use_skill(&self) -> bool {
        if self.skills.is_empty() {
            return false;
        }
        let skill = self.skills[self.active_skill];
        self.mana >= skill.mana_cost()
    }

    /// Get the current active skill
    pub fn current_skill(&self) -> Option<Skill> {
        if self.skills.is_empty() {
            None
        } else {
            Some(self.skills[self.active_skill])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = Player::new(5, 5, CharacterClass::Warrior);
        assert_eq!(player.x, 5);
        assert_eq!(player.y, 5);
        assert!(player.hp > 0);
    }

    #[test]
    fn test_enemy_damage() {
        let mut enemy = Enemy::new(0, 0, EnemyKind::Rat, 1);
        let initial_hp = enemy.hp;
        let damage = enemy.take_damage(5);
        assert!(damage > 0);
        assert!(enemy.hp < initial_hp);
    }

    #[test]
    fn test_player_xp_gain() {
        let mut player = Player::new(0, 0, CharacterClass::Warrior);
        let initial_level = player.level;
        player.xp = player.xp_to_level - 1;
        let leveled = player.gain_xp(10);
        assert!(leveled);
        assert_eq!(player.level, initial_level + 1);
    }
}
